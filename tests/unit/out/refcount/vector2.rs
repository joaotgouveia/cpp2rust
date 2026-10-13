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
pub struct __type_identity_9 {}
impl Clone for __type_identity_9 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_9 {
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
pub struct __or__13 {}
impl Clone for __or__13 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__13 {
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
pub struct __not__24 {}
impl Clone for __not__24 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__24 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_25 {}
impl Clone for is_reference_25 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_25 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_26 {}
impl Clone for is_reference_26 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_26 {
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
pub struct is_function_29 {}
impl Clone for is_function_29 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_29 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_30 {}
impl Clone for is_function_30 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_30 {
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
pub struct is_void_33 {}
impl Clone for is_void_33 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_33 {
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
pub struct __is_array_unknown_bounds_52 {}
impl Clone for __is_array_unknown_bounds_52 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_52 {
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
pub struct is_pointer_65 {}
impl Clone for is_pointer_65 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_pointer_65 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_arithmetic_66 {}
impl Clone for is_arithmetic_66 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_arithmetic_66 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_scalar_67 {}
impl Clone for is_scalar_67 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_scalar_67 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_68 {}
impl Clone for is_same_68 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_68 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_69 {}
impl Clone for is_same_69 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_69 {
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
pub struct __is_destructible_safe_75 {}
impl Clone for __is_destructible_safe_75 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_destructible_safe_75 {
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
pub struct is_constructible_76 {}
impl Clone for is_constructible_76 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_constructible_76 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_move_constructible_77 {}
impl Clone for is_move_constructible_77 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_move_constructible_77 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_default_constructible_78 {}
impl Clone for is_nothrow_default_constructible_78 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_default_constructible_78 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_copy_constructible_79 {}
impl Clone for is_nothrow_copy_constructible_79 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_copy_constructible_79 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_copy_constructible_80 {}
impl Clone for is_nothrow_copy_constructible_80 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_copy_constructible_80 {
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
pub struct is_trivially_destructible_81 {}
impl Clone for is_trivially_destructible_81 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_trivially_destructible_81 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_convertible_82 {}
impl Clone for is_convertible_82 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_82 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_convertible_83 {}
impl Clone for is_convertible_83 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_83 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_nothrow_new_constructible_impl_84 {}
impl Clone for __is_nothrow_new_constructible_impl_84 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_nothrow_new_constructible_impl_84 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_85 {}
impl Clone for remove_reference_85 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_85 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_86 {}
impl Clone for remove_reference_86 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_86 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __cv_selector_87 {}
impl Clone for __cv_selector_87 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_87 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __cv_selector_88 {}
impl Clone for __cv_selector_88 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_88 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __cv_selector_89 {}
impl Clone for __cv_selector_89 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_89 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __cv_selector_90 {}
impl Clone for __cv_selector_90 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_90 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_91 {}
impl Clone for __match_cv_qualifiers_91 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_91 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_92 {}
impl Clone for __match_cv_qualifiers_92 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_92 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_93 {}
impl Clone for __match_cv_qualifiers_93 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_93 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_94 {}
impl Clone for __match_cv_qualifiers_94 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_94 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_95 {}
impl Clone for __match_cv_qualifiers_95 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_95 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_96 {}
impl Clone for __make_unsigned_96 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_96 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_97 {}
impl Clone for __make_unsigned_97 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_97 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_98 {}
impl Clone for __make_unsigned_98 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_98 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_99 {}
impl Clone for __make_unsigned_selector_99 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_99 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_100 {}
impl Clone for __make_unsigned_selector_100 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_100 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_101 {}
impl Clone for __make_unsigned_selector_101 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_101 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_102 {}
impl Clone for __make_unsigned_selector_102 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_102 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_103 {}
impl Clone for __make_unsigned_selector_103 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_103 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_104 {}
impl Clone for __make_unsigned_selector_104 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_104 {
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
pub struct _List_108 {}
impl Clone for _List_108 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_108 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static std___make_unsigned_selector_base__List_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long____size_109 : Value<usize > = unimplemented!() ;
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_110 {}
impl Clone for _List_110 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_110 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static std___make_unsigned_selector_base__List_unsigned_short__unsigned_int__unsigned_long__unsigned_long_long____size_111 : Value<usize > = unimplemented!() ;
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_112 {}
impl Clone for _List_112 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_112 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static std___make_unsigned_selector_base__List_unsigned_int__unsigned_long__unsigned_long_long____size_113 : Value<usize > = unimplemented!() ;
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_114 {}
impl Clone for _List_114 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_114 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_115 {}
impl Clone for _List_115 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_115 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_116 {}
impl Clone for _List_116 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_116 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_117 {}
impl Clone for __select_117 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_117 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_118 {}
impl Clone for __select_118 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_118 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_119 {}
impl Clone for __select_119 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_119 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_120 {}
impl Clone for __select_120 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_120 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_121 {}
impl Clone for __select_121 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_121 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_122 {}
impl Clone for __make_signed_122 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_122 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_123 {}
impl Clone for __make_signed_123 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_123 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_124 {}
impl Clone for __make_signed_selector_124 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_124 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_125 {}
impl Clone for __make_signed_selector_125 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_125 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_126 {}
impl Clone for __make_signed_selector_126 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_126 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_127 {}
impl Clone for __make_signed_selector_127 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_127 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_128 {}
impl Clone for __make_signed_selector_128 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_128 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_pointer_129 {}
impl Clone for remove_pointer_129 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_pointer_129 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_pointer_130 {}
impl Clone for remove_pointer_130 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_pointer_130 {
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
pub struct common_type_131 {}
impl Clone for common_type_131 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for common_type_131 {
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
    pub static __is_nothrow_new_constructible_132: Value<bool> = unimplemented!();
);
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
pub struct __are_same_133 {}
impl Clone for __are_same_133 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_133 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_134 = u32;
pub const anon_134___value: anon_134 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_135 {}
impl Clone for __are_same_135 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_135 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_136 = u32;
pub const anon_136___value: anon_136 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_137 {}
impl Clone for __are_same_137 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_137 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_138 = u32;
pub const anon_138___value: anon_138 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_139 {}
impl Clone for __are_same_139 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_139 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_140 = u32;
pub const anon_140___value: anon_140 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_141 {}
impl Clone for __are_same_141 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_141 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_142 = u32;
pub const anon_142___value: anon_142 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_143 {}
impl Clone for __are_same_143 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_143 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_144 = u32;
pub const anon_144___value: anon_144 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_145 {}
impl Clone for __is_integer_145 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_145 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_146 = u32;
pub const anon_146___value: anon_146 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_147 {}
impl Clone for __is_integer_147 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_147 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_148 = u32;
pub const anon_148___value: anon_148 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_149 {}
impl Clone for __is_integer_149 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_149 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_150 = u32;
pub const anon_150___value: anon_150 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_151 {}
impl Clone for __is_integer_151 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_151 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_152 = u32;
pub const anon_152___value: anon_152 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_153 {}
impl Clone for __is_integer_153 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_153 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_154 = u32;
pub const anon_154___value: anon_154 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_nonvolatile_trivially_copyable_155 {}
impl Clone for __is_nonvolatile_trivially_copyable_155 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_nonvolatile_trivially_copyable_155 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_156 = u32;
pub const anon_156___value: anon_156 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __memcpyable_157 {}
impl Clone for __memcpyable_157 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __memcpyable_157 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __memcpyable_158 {}
impl Clone for __memcpyable_158 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __memcpyable_158 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __memcpyable_integer_159 {}
impl Clone for __memcpyable_integer_159 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __memcpyable_integer_159 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_move_iterator_160 {}
impl Clone for __is_move_iterator_160 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_move_iterator_160 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_161 = u32;
pub const anon_161___value: anon_161 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_move_iterator_162 {}
impl Clone for __is_move_iterator_162 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_move_iterator_162 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_163 = u32;
pub const anon_163___value: anon_163 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_nonstrict_164 {}
impl Clone for __is_integer_nonstrict_164 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_164 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_165 = u32;
pub const anon_165___width: anon_165 = 64;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_nonstrict_166 {}
impl Clone for __is_integer_nonstrict_166 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_166 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_167 = u32;
pub const anon_167___width: anon_167 = 64;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_integer_168 {}
impl Clone for __numeric_traits_integer_168 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_168 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_long____is_signed_169: Value<bool> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_long____digits_170: Value<i32> = unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_long____max_171: Value<i64> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_integer_172 {}
impl Clone for __numeric_traits_integer_172 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_172 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_long____is_signed_173: Value<bool> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_long____digits_174: Value<i32> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_long____max_175: Value<u64> =
        unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_floating_176 {}
impl Clone for __numeric_traits_floating_176 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_176 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_floating_177 {}
impl Clone for __numeric_traits_floating_177 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_177 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_floating_178 {}
impl Clone for __numeric_traits_floating_178 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_178 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_179 {}
impl Clone for __numeric_traits_179 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_179 {
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
    pub static piecewise_construct_180: Value<std_piecewise_construct_t> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct tuple_181 {}
impl Clone for tuple_181 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for tuple_181 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl tuple_181 {}
impl tuple_181 {}
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
impl std_output_iterator_tag {}
impl std_output_iterator_tag {}
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
pub struct iterator_182 {}
impl Clone for iterator_182 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_182 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_183 {}
impl Clone for iterator_183 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_183 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl iterator_183 {}
impl iterator_183 {}
impl iterator_183 {}
impl iterator_183 {
    pub fn copy_assign(&self, _a0: Ptr<iterator_183>) -> Ptr<iterator_183> {
        unimplemented!()
    }
}
impl iterator_183 {
    pub fn move_assign(&self, _a0: Ptr<iterator_183>) -> Ptr<iterator_183> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_184 {}
impl Clone for iterator_184 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_184 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl iterator_184 {
    pub fn copy_assign(&self, _a0: Ptr<iterator_184>) -> Ptr<iterator_184> {
        unimplemented!()
    }
}
impl iterator_184 {
    pub fn move_assign(&self, _a0: Ptr<iterator_184>) -> Ptr<iterator_184> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_185 {}
impl Clone for iterator_185 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_185 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl iterator_185 {
    pub fn copy_assign(&self, _a0: Ptr<iterator_185>) -> Ptr<iterator_185> {
        unimplemented!()
    }
}
impl iterator_185 {
    pub fn move_assign(&self, _a0: Ptr<iterator_185>) -> Ptr<iterator_185> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_186 {}
impl Clone for iterator_traits_186 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_186 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_187 {}
impl Clone for iterator_traits_187 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_187 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_188 {}
impl Clone for iterator_traits_188 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_188 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_189 {}
impl Clone for iterator_traits_189 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_189 {
    fn default() -> Self {
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
pub struct __iterator_traits_191 {}
impl Clone for __iterator_traits_191 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __iterator_traits_191 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __iterator_traits_192 {}
impl Clone for __iterator_traits_192 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __iterator_traits_192 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __iterator_traits_193 {}
impl Clone for __iterator_traits_193 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __iterator_traits_193 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __ptr_traits_ptr_to_194 {}
impl Clone for __ptr_traits_ptr_to_194 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_194 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct pointer_traits_195 {}
impl Clone for pointer_traits_195 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_195 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct reverse_iterator_196 {}
impl Clone for reverse_iterator_196 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for reverse_iterator_196 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_196 {
    pub fn copy_from(__x: Ptr<reverse_iterator_196>) -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_196 {
    pub fn copy_assign(&self, _a0: Ptr<reverse_iterator_196>) -> Ptr<reverse_iterator_196> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct reverse_iterator_197 {}
impl Clone for reverse_iterator_197 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for reverse_iterator_197 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_197 {
    pub fn copy_from(__x: Ptr<reverse_iterator_197>) -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_197 {
    pub fn copy_assign(&self, _a0: Ptr<reverse_iterator_197>) -> Ptr<reverse_iterator_197> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
pub struct __normal_iterator_198 {}
impl Clone for __normal_iterator_198 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __normal_iterator_198 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_198 {
    pub fn new_1(__i: Ptr<Ptr<i32>>) -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_198 {}
impl __normal_iterator_198 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
pub struct __normal_iterator_199 {}
impl Clone for __normal_iterator_199 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __normal_iterator_199 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_199 {
    pub fn new_1(__i: Ptr<Ptr<i32>>) -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_199 {}
impl __normal_iterator_199 {}
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
pub struct __new_allocator_200 {}
impl Clone for __new_allocator_200 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_200 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_200 {
    pub fn copy_from(_a0: Ptr<__new_allocator_200>) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_200 {
    pub fn copy_assign(&self, _a0: Ptr<__new_allocator_200>) -> Ptr<__new_allocator_200> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __new_allocator_201 {}
impl Clone for __new_allocator_201 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_201 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_201 {
    pub fn copy_from(_a0: Ptr<__new_allocator_201>) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_201 {
    pub fn copy_assign(&self, _a0: Ptr<__new_allocator_201>) -> Ptr<__new_allocator_201> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __new_allocator_202 {}
impl Clone for __new_allocator_202 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_202 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_202 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_202 {
    pub fn copy_from(_a0: Ptr<__new_allocator_202>) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_202 {
    pub fn copy_assign(&self, _a0: Ptr<__new_allocator_202>) -> Ptr<__new_allocator_202> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_204 {}
impl Clone for allocator_204 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_204 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_204 {}
impl allocator_204 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_205 {}
impl Clone for allocator_205 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_205 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_205 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl allocator_205 {
    pub fn copy_from(__a: Ptr<allocator_205>) -> Self {
        unimplemented!()
    }
}
impl allocator_205 {
    pub fn copy_assign(&self, _a0: Ptr<allocator_205>) -> Ptr<allocator_205> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_traits_206 {}
impl Clone for allocator_traits_206 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_traits_206 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_traits_206 {
    pub fn allocate_1(__a: Ptr<allocator_205>, mut __n: u64) -> Ptr<i32> {
        unimplemented!()
    }
}
impl allocator_traits_206 {
    pub fn deallocate(__a: Ptr<allocator_205>, mut __p: Ptr<i32>, mut __n: u64) {
        unimplemented!()
    }
}
impl allocator_traits_206 {
    pub fn max_size(__a: Ptr<allocator_205>) -> u64 {
        unimplemented!()
    }
}
impl allocator_traits_206 {
    pub fn select_on_container_copy_construction(__rhs: Ptr<allocator_205>) -> allocator_205 {
        unimplemented!()
    }
}
impl allocator_traits_206 {
    pub fn construct_207(__a: Ptr<allocator_205>, mut __p: Ptr<i32>, __args: Ptr<i32>) {
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
pub struct __is_move_insertable_209 {}
impl Clone for __is_move_insertable_209 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_move_insertable_209 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __alloc_traits_210 {}
impl Clone for __alloc_traits_210 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_210 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct rebind_211 {}
impl Clone for rebind_211 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_211 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __alloc_traits_210 {
    pub fn _S_select_on_copy(__a: Ptr<allocator_205>) -> allocator_205 {
        unimplemented!()
    }
}
impl __alloc_traits_210 {
    pub fn _S_propagate_on_copy_assign() -> bool {
        unimplemented!()
    }
}
impl __alloc_traits_210 {
    pub fn _S_always_equal() -> bool {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_bitwise_relocatable_212 {}
impl Clone for __is_bitwise_relocatable_212 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_bitwise_relocatable_212 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct initializer_list_213 {}
impl Clone for initializer_list_213 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for initializer_list_213 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct initializer_list_214 {}
impl Clone for initializer_list_214 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for initializer_list_214 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl initializer_list_214 {}
impl initializer_list_214 {}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(24)]
pub struct _Vector_base_215 {
    #[offset(0)]
    #[byte_size(24)]
    pub _M_impl: std__Vector_base_int__std_allocator_int____Vector_impl,
}
impl Clone for _Vector_base_215 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Vector_base_215 {
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
    pub fn new_1() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_int__std_allocator_int____Vector_impl {
    pub fn new_2(__a: Ptr<allocator_205>) -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_int__std_allocator_int____Vector_impl {
    pub fn move_from(__x: Ptr<std__Vector_base_int__std_allocator_int____Vector_impl>) -> Self {
        unimplemented!()
    }
}
impl _Vector_base_215 {
    pub fn new_1() -> Self {
        unimplemented!()
    }
}
impl _Vector_base_215 {
    pub fn new_2(mut __n: usize, __a: Ptr<allocator_205>) -> Self {
        unimplemented!()
    }
}
impl _Vector_base_215 {
    pub fn move_from(_a0: Ptr<_Vector_base_215>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(24)]
pub struct vector_216 {}
impl Clone for vector_216 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for vector_216 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(24)]
pub struct std_vector_int___Guard_alloc {
    #[offset(0)]
    #[byte_size(8)]
    pub _M_storage: Ptr<i32>,
    #[offset(8)]
    pub _M_len: u64,
    #[offset(16)]
    #[byte_size(8)]
    pub _M_vect: Ptr<_Vector_base_215>,
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
    pub fn new(mut __s: Ptr<i32>, mut __l: u64, __vect: Ptr<_Vector_base_215>) -> Self {
        unimplemented!()
    }
}
impl vector_216 {
    pub fn new_1() -> Self {
        unimplemented!()
    }
}
impl vector_216 {
    pub fn copy_from(__x: Ptr<vector_216>) -> Self {
        unimplemented!()
    }
}
impl vector_216 {
    pub fn move_from(_a0: Ptr<vector_216>) -> Self {
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
pub struct hash_239 {}
impl Clone for hash_239 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for hash_239 {
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
    pub fn hash_240(__val: Ptr<f32>) -> usize {
        unimplemented!()
    }
}
impl std__Hash_impl {
    pub fn hash_241(__val: Ptr<f64>) -> usize {
        unimplemented!()
    }
}
pub type anon_242 = u32;
pub const anon_242__S_word_bit: anon_242 = 64;
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
pub struct binary_function_243 {}
impl Clone for binary_function_243 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for binary_function_243 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct greater_244 {}
impl Clone for greater_244 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for greater_244 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct less_245 {}
impl Clone for less_245 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for less_245 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct greater_equal_246 {}
impl Clone for greater_equal_246 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for greater_equal_246 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct less_equal_247 {}
impl Clone for less_equal_247 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for less_equal_247 {
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
    pub static allocator_arg_248: Value<std_allocator_arg_t> = unimplemented!();
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
pub fn fn_249(v: Ptr<vector_216>, v3: vector_216) {
    let v3: Value<vector_216> = Rc::new(RefCell::new(v3));
    ({
        let ___x: Value<i32> = Rc::new(RefCell::new(20));
        vector_216Impl::push_back_58(&v, ___x.as_pointer())
    });
    let mut x: i32 = 0_i32;
    let v2: Value<vector_216> = Rc::new(RefCell::new(<vector_216>::default()));
    let mut v4: Ptr<vector_216> = (v3.as_pointer());
    ({
        let ___x: Value<i32> = Rc::new(RefCell::new(0));
        vector_216Impl::push_back_58(&v2.as_pointer(), ___x.as_pointer())
    });
    ({
        let ___x: Value<i32> = Rc::new(RefCell::new(1));
        vector_216Impl::push_back_58(&v2.as_pointer(), ___x.as_pointer())
    });
    ({
        let ___x: Value<i32> = Rc::new(RefCell::new(3));
        vector_216Impl::push_back_58(&v2.as_pointer(), ___x.as_pointer())
    });
    x = (({ vector_216Impl::operator_index_46(&v, 2_u64) }).read());
    ({ vector_216Impl::operator_index_46(&v2.as_pointer(), 0_u64) }).write(1);
    ({
        vector_216Impl::operator_index_46(
            &(if true { (v3.as_pointer()) } else { (v).clone() }),
            0_u64,
        )
    })
    .write(7);
    ({
        let ___x: Ptr<vector_216> = (v).clone();
        vector_216Impl::copy_assign(&v2.as_pointer(), ___x)
    });
    ({ vector_216Impl::operator_index_46(&(v4), 1_u64) }).write(13);
    (if (x == 6) {
        &(0);
    } else {
        ({
            __assert_fail_250(
                Ptr::<i8>::from_string_literal(b"x == 6"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"void fn(std::vector<int> &, std::vector<int>)"),
            )
        });
    });
    (if ((({ vector_216Impl::front_51(&v) }).read()) == 4) {
        &(0);
    } else {
        ({
            __assert_fail_250(
                Ptr::<i8>::from_string_literal(b"v.front() == 4"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"void fn(std::vector<int> &, std::vector<int>)"),
            )
        });
    });
    (if ((({ vector_216Impl::operator_index_46(&v, 1_u64) }).read()) == 5) {
        &(0);
    } else {
        ({
            __assert_fail_250(
                Ptr::<i8>::from_string_literal(b"v[1] == 5"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"void fn(std::vector<int> &, std::vector<int>)"),
            )
        });
    });
    (if ((({ vector_216Impl::operator_index_46(&v, 2_u64) }).read()) == 6) {
        &(0);
    } else {
        ({
            __assert_fail_250(
                Ptr::<i8>::from_string_literal(b"v[2] == 6"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"void fn(std::vector<int> &, std::vector<int>)"),
            )
        });
    });
    (if ((({ vector_216Impl::back_53(&v) }).read()) == 20) {
        &(0);
    } else {
        ({
            __assert_fail_250(
                Ptr::<i8>::from_string_literal(b"v.back() == 20"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"void fn(std::vector<int> &, std::vector<int>)"),
            )
        });
    });
    (if ((({ vector_216Impl::operator_index_46(&v2.as_pointer(), 0_u64) }).read()) == 4) {
        &(0);
    } else {
        ({
            __assert_fail_250(
                Ptr::<i8>::from_string_literal(b"v2[0] == 4"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"void fn(std::vector<int> &, std::vector<int>)"),
            )
        });
    });
    (if ((({ vector_216Impl::operator_index_46(&v2.as_pointer(), 1_u64) }).read()) == 5) {
        &(0);
    } else {
        ({
            __assert_fail_250(
                Ptr::<i8>::from_string_literal(b"v2[1] == 5"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"void fn(std::vector<int> &, std::vector<int>)"),
            )
        });
    });
    (if ((({ vector_216Impl::operator_index_46(&v2.as_pointer(), 2_u64) }).read()) == 6) {
        &(0);
    } else {
        ({
            __assert_fail_250(
                Ptr::<i8>::from_string_literal(b"v2[2] == 6"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"void fn(std::vector<int> &, std::vector<int>)"),
            )
        });
    });
    (if ((({ vector_216Impl::operator_index_46(&v3.as_pointer(), 0_u64) }).read()) == 7) {
        &(0);
    } else {
        ({
            __assert_fail_250(
                Ptr::<i8>::from_string_literal(b"v3[0] == 7"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"void fn(std::vector<int> &, std::vector<int>)"),
            )
        });
    });
    (if ((({ vector_216Impl::operator_index_46(&v3.as_pointer(), 1_u64) }).read()) == 13) {
        &(0);
    } else {
        ({
            __assert_fail_250(
                Ptr::<i8>::from_string_literal(b"v3[1] == 13"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"void fn(std::vector<int> &, std::vector<int>)"),
            )
        });
    });
    ({
        let ___x: Value<i32> = Rc::new(RefCell::new(20));
        vector_216Impl::push_back_58(&v, ___x.as_pointer())
    });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let v: Value<vector_216> = Rc::new(RefCell::new(<vector_216>::default()));
    let v2: Value<vector_216> = Rc::new(RefCell::new(<vector_216>::default()));
    ({
        let ___x: Value<i32> = Rc::new(RefCell::new(4));
        vector_216Impl::push_back_58(&v.as_pointer(), ___x.as_pointer())
    });
    ({
        let ___x: Value<i32> = Rc::new(RefCell::new(5));
        vector_216Impl::push_back_58(&v.as_pointer(), ___x.as_pointer())
    });
    ({
        let ___x: Value<i32> = Rc::new(RefCell::new(6));
        vector_216Impl::push_back_58(&v.as_pointer(), ___x.as_pointer())
    });
    ({
        let ___x: Value<i32> = Rc::new(RefCell::new(8));
        vector_216Impl::push_back_58(&v2.as_pointer(), ___x.as_pointer())
    });
    ({
        let ___x: Value<i32> = Rc::new(RefCell::new(9));
        vector_216Impl::push_back_58(&v2.as_pointer(), ___x.as_pointer())
    });
    ({ fn_249(v.as_pointer(), vector_216::copy_from({ v2.as_pointer() })) });
    return 0;
}
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct std___uses_alloc0__Sink;
pub fn __assert_fail_250(
    mut __assertion: Ptr<i8>,
    mut __file: Ptr<i8>,
    mut __line: u32,
    mut __function: Ptr<i8>,
) {
    unimplemented!()
}
pub trait _Vector_base_215Impl {
    fn _M_get_Tp_allocator_1(&self) -> Ptr<allocator_205> {
        unimplemented!()
    }
    fn _M_get_Tp_allocator_2(&self) -> Ptr<allocator_205> {
        unimplemented!()
    }
    fn _M_allocate(&self, mut __n: usize) -> Ptr<i32> {
        unimplemented!()
    }
    fn _M_deallocate(&self, mut __p: Ptr<i32>, mut __n: usize) {
        unimplemented!()
    }
}
impl _Vector_base_215Impl for Ptr<_Vector_base_215> {}
pub trait __new_allocator_202Impl {
    fn allocate(&self, mut __n: u64, _a1: Option<AnyPtr>) -> Ptr<i32> {
        unimplemented!()
    }
    fn deallocate(&self, mut __p: Ptr<i32>, mut __n: u64) {
        unimplemented!()
    }
    fn max_size(&self) -> u64 {
        unimplemented!()
    }
    fn construct_203(&self, mut __p: Ptr<i32>, __args: Ptr<i32>) {
        unimplemented!()
    }
}
impl __new_allocator_202Impl for Ptr<__new_allocator_202> {}
pub trait __normal_iterator_198Impl {
    fn operator_deref(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn operator_add(&self, mut __n: i64) -> __normal_iterator_198 {
        unimplemented!()
    }
    fn operator_sub(&self, mut __n: i64) -> __normal_iterator_198 {
        unimplemented!()
    }
    fn base(&self) -> Ptr<Ptr<i32>> {
        unimplemented!()
    }
}
impl __normal_iterator_198Impl for Ptr<__normal_iterator_198> {}
pub trait __normal_iterator_199Impl {
    fn base(&self) -> Ptr<Ptr<i32>> {
        unimplemented!()
    }
}
impl __normal_iterator_199Impl for Ptr<__normal_iterator_199> {}
pub trait initializer_list_213Impl {
    fn size(&self) -> u64 {
        unimplemented!()
    }
    fn begin(&self) -> Ptr<bool> {
        unimplemented!()
    }
}
impl initializer_list_213Impl for Ptr<initializer_list_213> {}
pub trait integral_constant_0Impl {
    fn to_bool(&self) -> bool {
        unimplemented!()
    }
}
impl integral_constant_0Impl for Ptr<integral_constant_0> {}
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
pub trait std_vector_int___Guard_allocImpl {
    fn _M_release(&self) -> Ptr<i32> {
        unimplemented!()
    }
}
impl std_vector_int___Guard_allocImpl for Ptr<std_vector_int___Guard_alloc> {}
pub trait vector_216Impl {
    fn copy_assign(&self, __x: Ptr<vector_216>) -> Ptr<vector_216> {
        unimplemented!()
    }
    fn move_assign(&self, __x: Ptr<vector_216>) -> Ptr<vector_216> {
        unimplemented!()
    }
    fn begin_26(&self) -> __normal_iterator_198 {
        unimplemented!()
    }
    fn begin_27(&self) -> __normal_iterator_199 {
        unimplemented!()
    }
    fn end_28(&self) -> __normal_iterator_198 {
        unimplemented!()
    }
    fn end_29(&self) -> __normal_iterator_199 {
        unimplemented!()
    }
    fn size(&self) -> u64 {
        unimplemented!()
    }
    fn max_size(&self) -> u64 {
        unimplemented!()
    }
    fn capacity(&self) -> u64 {
        unimplemented!()
    }
    fn empty(&self) -> bool {
        unimplemented!()
    }
    fn operator_index_46(&self, mut __n: u64) -> Ptr<i32> {
        unimplemented!()
    }
    fn front_51(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn back_53(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn push_back_58(&self, __x: Ptr<i32>) {
        unimplemented!()
    }
    fn clear(&self) {
        unimplemented!()
    }
    fn emplace_back_217(&self, __args: Ptr<i32>) -> Ptr<i32> {
        unimplemented!()
    }
}
impl vector_216Impl for Ptr<vector_216> {}
pub fn __cpp2rust_init_globals() {}
