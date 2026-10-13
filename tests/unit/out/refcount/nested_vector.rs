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
pub struct __and__32 {}
impl Clone for __and__32 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__32 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __and__33 {}
impl Clone for __and__33 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__33 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __and__34 {}
impl Clone for __and__34 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__34 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __not__35 {}
impl Clone for __not__35 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__35 {
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
pub struct is_reference_41 {}
impl Clone for is_reference_41 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_41 {
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
pub struct is_function_46 {}
impl Clone for is_function_46 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_46 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_47 {}
impl Clone for is_function_47 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_47 {
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
pub struct is_void_52 {}
impl Clone for is_void_52 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_52 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_53 {}
impl Clone for is_void_53 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_53 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_54 {}
impl Clone for is_void_54 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_54 {
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
pub struct remove_cv_58 {}
impl Clone for remove_cv_58 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_58 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_59 {}
impl Clone for remove_cv_59 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_59 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_60 {}
impl Clone for remove_cv_60 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_60 {
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
pub struct is_const_63 {}
impl Clone for is_const_63 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_63 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_const_64 {}
impl Clone for is_const_64 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_64 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_const_65 {}
impl Clone for is_const_65 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_65 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_const_66 {}
impl Clone for is_const_66 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_66 {
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
pub struct __is_array_unknown_bounds_68 {}
impl Clone for __is_array_unknown_bounds_68 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_68 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_69 {}
impl Clone for __is_array_unknown_bounds_69 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_69 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_70 {}
impl Clone for __is_array_unknown_bounds_70 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_70 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_71 {}
impl Clone for __is_array_unknown_bounds_71 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_71 {
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
pub struct __is_integral_helper_74 {}
impl Clone for __is_integral_helper_74 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_74 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_75 {}
impl Clone for __is_integral_helper_75 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_75 {
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
pub struct is_integral_80 {}
impl Clone for is_integral_80 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_80 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_81 {}
impl Clone for is_integral_81 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_81 {
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
pub struct __is_floating_point_helper_87 {}
impl Clone for __is_floating_point_helper_87 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_floating_point_helper_87 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_floating_point_88 {}
impl Clone for is_floating_point_88 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_floating_point_88 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_pointer_89 {}
impl Clone for is_pointer_89 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_pointer_89 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_pointer_90 {}
impl Clone for is_pointer_90 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_pointer_90 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_enum_91 {}
impl Clone for is_enum_91 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_enum_91 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_null_pointer_92 {}
impl Clone for is_null_pointer_92 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_null_pointer_92 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_arithmetic_93 {}
impl Clone for is_arithmetic_93 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_arithmetic_93 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_arithmetic_94 {}
impl Clone for is_arithmetic_94 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_arithmetic_94 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_member_pointer_95 {}
impl Clone for is_member_pointer_95 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_member_pointer_95 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_scalar_96 {}
impl Clone for is_scalar_96 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_scalar_96 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_scalar_97 {}
impl Clone for is_scalar_97 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_scalar_97 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_98 {}
impl Clone for is_same_98 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_98 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_99 {}
impl Clone for is_same_99 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_99 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_100 {}
impl Clone for is_same_100 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_100 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_volatile_101 {}
impl Clone for is_volatile_101 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_101 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_volatile_102 {}
impl Clone for is_volatile_102 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_102 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_volatile_103 {}
impl Clone for is_volatile_103 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_103 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_volatile_104 {}
impl Clone for is_volatile_104 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_104 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_volatile_105 {}
impl Clone for is_volatile_105 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_105 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_all_extents_106 {}
impl Clone for remove_all_extents_106 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_all_extents_106 {
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
pub struct __is_destructible_impl_107 {}
impl Clone for __is_destructible_impl_107 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_destructible_impl_107 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_destructible_safe_108 {}
impl Clone for __is_destructible_safe_108 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_destructible_safe_108 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_destructible_safe_109 {}
impl Clone for __is_destructible_safe_109 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_destructible_safe_109 {
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
pub struct __is_nt_destructible_impl_110 {}
impl Clone for __is_nt_destructible_impl_110 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_nt_destructible_impl_110 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_nt_destructible_safe_111 {}
impl Clone for __is_nt_destructible_safe_111 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_nt_destructible_safe_111 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_destructible_112 {}
impl Clone for is_nothrow_destructible_112 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_destructible_112 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_constructible_113 {}
impl Clone for is_constructible_113 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_constructible_113 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_constructible_114 {}
impl Clone for is_constructible_114 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_constructible_114 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_move_constructible_115 {}
impl Clone for is_move_constructible_115 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_move_constructible_115 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_default_constructible_116 {}
impl Clone for is_nothrow_default_constructible_116 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_default_constructible_116 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_copy_constructible_117 {}
impl Clone for is_nothrow_copy_constructible_117 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_copy_constructible_117 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_copy_constructible_118 {}
impl Clone for is_nothrow_copy_constructible_118 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_copy_constructible_118 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_copy_constructible_119 {}
impl Clone for is_nothrow_copy_constructible_119 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_copy_constructible_119 {
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
pub struct is_trivially_destructible_120 {}
impl Clone for is_trivially_destructible_120 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_trivially_destructible_120 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_trivially_destructible_121 {}
impl Clone for is_trivially_destructible_121 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_trivially_destructible_121 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_convertible_122 {}
impl Clone for is_convertible_122 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_122 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_nothrow_new_constructible_impl_123 {}
impl Clone for __is_nothrow_new_constructible_impl_123 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_nothrow_new_constructible_impl_123 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_nothrow_new_constructible_impl_124 {}
impl Clone for __is_nothrow_new_constructible_impl_124 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_nothrow_new_constructible_impl_124 {
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
pub struct __cv_selector_130 {}
impl Clone for __cv_selector_130 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_130 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __cv_selector_131 {}
impl Clone for __cv_selector_131 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_131 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __cv_selector_132 {}
impl Clone for __cv_selector_132 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_132 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __cv_selector_133 {}
impl Clone for __cv_selector_133 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_133 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_134 {}
impl Clone for __match_cv_qualifiers_134 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_134 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_135 {}
impl Clone for __match_cv_qualifiers_135 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_135 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_136 {}
impl Clone for __match_cv_qualifiers_136 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_136 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_137 {}
impl Clone for __match_cv_qualifiers_137 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_137 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_138 {}
impl Clone for __match_cv_qualifiers_138 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_138 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_139 {}
impl Clone for __make_unsigned_139 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_139 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_140 {}
impl Clone for __make_unsigned_140 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_140 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_141 {}
impl Clone for __make_unsigned_141 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_141 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_142 {}
impl Clone for __make_unsigned_selector_142 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_142 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_143 {}
impl Clone for __make_unsigned_selector_143 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_143 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_144 {}
impl Clone for __make_unsigned_selector_144 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_144 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_145 {}
impl Clone for __make_unsigned_selector_145 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_145 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_146 {}
impl Clone for __make_unsigned_selector_146 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_146 {
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
pub struct _List_151 {}
impl Clone for _List_151 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_151 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static std___make_unsigned_selector_base__List_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long____size_152 : Value<usize > = unimplemented!() ;
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_153 {}
impl Clone for _List_153 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_153 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static std___make_unsigned_selector_base__List_unsigned_short__unsigned_int__unsigned_long__unsigned_long_long____size_154 : Value<usize > = unimplemented!() ;
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_155 {}
impl Clone for _List_155 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_155 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static std___make_unsigned_selector_base__List_unsigned_int__unsigned_long__unsigned_long_long____size_156 : Value<usize > = unimplemented!() ;
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_157 {}
impl Clone for _List_157 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_157 {
    fn default() -> Self {
        unimplemented!()
    }
}
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_159 {}
impl Clone for _List_159 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_159 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_160 {}
impl Clone for __select_160 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_160 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_161 {}
impl Clone for __select_161 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_161 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_162 {}
impl Clone for __select_162 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_162 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_163 {}
impl Clone for __select_163 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_163 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_164 {}
impl Clone for __select_164 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_164 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_165 {}
impl Clone for __make_signed_165 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_165 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_166 {}
impl Clone for __make_signed_166 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_166 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_167 {}
impl Clone for __make_signed_selector_167 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_167 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_168 {}
impl Clone for __make_signed_selector_168 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_168 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_169 {}
impl Clone for __make_signed_selector_169 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_169 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_170 {}
impl Clone for __make_signed_selector_170 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_170 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_171 {}
impl Clone for __make_signed_selector_171 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_171 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_pointer_172 {}
impl Clone for remove_pointer_172 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_pointer_172 {
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
pub struct common_type_173 {}
impl Clone for common_type_173 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for common_type_173 {
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
    pub static __is_nothrow_new_constructible_174: Value<bool> = unimplemented!();
);
thread_local!(
    pub static __is_nothrow_new_constructible_175: Value<bool> = unimplemented!();
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
pub const anon_177___value: anon_177 = 1;
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
pub struct __are_same_180 {}
impl Clone for __are_same_180 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_180 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_181 = u32;
pub const anon_181___value: anon_181 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_182 {}
impl Clone for __are_same_182 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_182 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_183 = u32;
pub const anon_183___value: anon_183 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_184 {}
impl Clone for __are_same_184 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_184 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_185 = u32;
pub const anon_185___value: anon_185 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_186 {}
impl Clone for __are_same_186 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_186 {
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
pub const anon_189___value: anon_189 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_190 {}
impl Clone for __is_integer_190 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_190 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_191 = u32;
pub const anon_191___value: anon_191 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_192 {}
impl Clone for __is_integer_192 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_192 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_193 = u32;
pub const anon_193___value: anon_193 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_194 {}
impl Clone for __is_integer_194 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_194 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_195 = u32;
pub const anon_195___value: anon_195 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_196 {}
impl Clone for __is_integer_196 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_196 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_197 = u32;
pub const anon_197___value: anon_197 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_nonvolatile_trivially_copyable_198 {}
impl Clone for __is_nonvolatile_trivially_copyable_198 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_nonvolatile_trivially_copyable_198 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_199 = u32;
pub const anon_199___value: anon_199 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __memcpyable_200 {}
impl Clone for __memcpyable_200 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __memcpyable_200 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __memcpyable_integer_201 {}
impl Clone for __memcpyable_integer_201 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __memcpyable_integer_201 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_nonstrict_202 {}
impl Clone for __is_integer_nonstrict_202 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_202 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_203 = u32;
pub const anon_203___width: anon_203 = 64;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_nonstrict_204 {}
impl Clone for __is_integer_nonstrict_204 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_204 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_205 = u32;
pub const anon_205___width: anon_205 = 64;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_integer_206 {}
impl Clone for __numeric_traits_integer_206 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_206 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_long____is_signed_207: Value<bool> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_long____digits_208: Value<i32> = unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_long____max_209: Value<i64> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_integer_210 {}
impl Clone for __numeric_traits_integer_210 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_210 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_long____is_signed_211: Value<bool> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_long____digits_212: Value<i32> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_long____max_213: Value<u64> =
        unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_floating_214 {}
impl Clone for __numeric_traits_floating_214 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_214 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_floating_215 {}
impl Clone for __numeric_traits_floating_215 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_215 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_floating_216 {}
impl Clone for __numeric_traits_floating_216 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_216 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_217 {}
impl Clone for __numeric_traits_217 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_217 {
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
    pub static piecewise_construct_218: Value<std_piecewise_construct_t> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct tuple_219 {}
impl Clone for tuple_219 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for tuple_219 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl tuple_219 {}
impl tuple_219 {}
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
pub struct iterator_220 {}
impl Clone for iterator_220 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_220 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_221 {}
impl Clone for iterator_221 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_221 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl iterator_221 {}
impl iterator_221 {}
impl iterator_221 {}
impl iterator_221 {
    pub fn copy_assign(&self, _a0: Ptr<iterator_221>) -> Ptr<iterator_221> {
        unimplemented!()
    }
}
impl iterator_221 {
    pub fn move_assign(&self, _a0: Ptr<iterator_221>) -> Ptr<iterator_221> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_222 {}
impl Clone for iterator_222 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_222 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl iterator_222 {
    pub fn copy_assign(&self, _a0: Ptr<iterator_222>) -> Ptr<iterator_222> {
        unimplemented!()
    }
}
impl iterator_222 {
    pub fn move_assign(&self, _a0: Ptr<iterator_222>) -> Ptr<iterator_222> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_223 {}
impl Clone for iterator_223 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_223 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl iterator_223 {
    pub fn copy_assign(&self, _a0: Ptr<iterator_223>) -> Ptr<iterator_223> {
        unimplemented!()
    }
}
impl iterator_223 {
    pub fn move_assign(&self, _a0: Ptr<iterator_223>) -> Ptr<iterator_223> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_224 {}
impl Clone for iterator_traits_224 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_224 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_225 {}
impl Clone for iterator_traits_225 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_225 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_226 {}
impl Clone for iterator_traits_226 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_226 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_227 {}
impl Clone for iterator_traits_227 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_227 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_228 {}
impl Clone for iterator_traits_228 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_228 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_229 {}
impl Clone for iterator_traits_229 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_229 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __iterator_traits_230 {}
impl Clone for __iterator_traits_230 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __iterator_traits_230 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __iterator_traits_231 {}
impl Clone for __iterator_traits_231 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __iterator_traits_231 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __ptr_traits_ptr_to_232 {}
impl Clone for __ptr_traits_ptr_to_232 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_232 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct pointer_traits_233 {}
impl Clone for pointer_traits_233 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_233 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct reverse_iterator_234 {}
impl Clone for reverse_iterator_234 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for reverse_iterator_234 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_234 {
    pub fn copy_from(__x: Ptr<reverse_iterator_234>) -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_234 {
    pub fn copy_assign(&self, _a0: Ptr<reverse_iterator_234>) -> Ptr<reverse_iterator_234> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct reverse_iterator_235 {}
impl Clone for reverse_iterator_235 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for reverse_iterator_235 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_235 {
    pub fn copy_from(__x: Ptr<reverse_iterator_235>) -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_235 {
    pub fn copy_assign(&self, _a0: Ptr<reverse_iterator_235>) -> Ptr<reverse_iterator_235> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
pub struct __normal_iterator_236 {}
impl Clone for __normal_iterator_236 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __normal_iterator_236 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_236 {
    pub fn new_1(__i: Ptr<Ptr<vector_237>>) -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_236 {}
impl __normal_iterator_236 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
pub struct __normal_iterator_238 {}
impl Clone for __normal_iterator_238 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __normal_iterator_238 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_238 {
    pub fn new_1(__i: Ptr<Ptr<vector_237>>) -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_238 {}
impl __normal_iterator_238 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
pub struct __normal_iterator_239 {}
impl Clone for __normal_iterator_239 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __normal_iterator_239 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_239 {
    pub fn new_1(__i: Ptr<Ptr<i32>>) -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_239 {}
impl __normal_iterator_239 {}
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
pub struct __new_allocator_240 {}
impl Clone for __new_allocator_240 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_240 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_240 {
    pub fn copy_from(_a0: Ptr<__new_allocator_240>) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_240 {
    pub fn copy_assign(&self, _a0: Ptr<__new_allocator_240>) -> Ptr<__new_allocator_240> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __new_allocator_241 {}
impl Clone for __new_allocator_241 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_241 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_241 {
    pub fn copy_from(_a0: Ptr<__new_allocator_241>) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_241 {
    pub fn copy_assign(&self, _a0: Ptr<__new_allocator_241>) -> Ptr<__new_allocator_241> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __new_allocator_242 {}
impl Clone for __new_allocator_242 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_242 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_242 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_242 {
    pub fn copy_from(_a0: Ptr<__new_allocator_242>) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_242 {
    pub fn copy_assign(&self, _a0: Ptr<__new_allocator_242>) -> Ptr<__new_allocator_242> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __new_allocator_246 {}
impl Clone for __new_allocator_246 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_246 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_246 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_246 {
    pub fn copy_from(_a0: Ptr<__new_allocator_246>) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_246 {
    pub fn copy_assign(&self, _a0: Ptr<__new_allocator_246>) -> Ptr<__new_allocator_246> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_247 {}
impl Clone for allocator_247 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_247 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_247 {}
impl allocator_247 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_248 {}
impl Clone for allocator_248 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_248 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_248 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl allocator_248 {
    pub fn copy_from(__a: Ptr<allocator_248>) -> Self {
        unimplemented!()
    }
}
impl allocator_248 {
    pub fn copy_assign(&self, _a0: Ptr<allocator_248>) -> Ptr<allocator_248> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_249 {}
impl Clone for allocator_249 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_249 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_249 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl allocator_249 {
    pub fn copy_from(__a: Ptr<allocator_249>) -> Self {
        unimplemented!()
    }
}
impl allocator_249 {
    pub fn copy_assign(&self, _a0: Ptr<allocator_249>) -> Ptr<allocator_249> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_traits_250 {}
impl Clone for allocator_traits_250 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_traits_250 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_traits_250 {
    pub fn allocate_1(__a: Ptr<allocator_249>, mut __n: u64) -> Ptr<vector_237> {
        unimplemented!()
    }
}
impl allocator_traits_250 {
    pub fn deallocate(__a: Ptr<allocator_249>, mut __p: Ptr<vector_237>, mut __n: u64) {
        unimplemented!()
    }
}
impl allocator_traits_250 {
    pub fn max_size(__a: Ptr<allocator_249>) -> u64 {
        unimplemented!()
    }
}
impl allocator_traits_250 {
    pub fn construct_251(
        __a: Ptr<allocator_249>,
        mut __p: Ptr<vector_237>,
        __args: Ptr<vector_237>,
    ) {
        unimplemented!()
    }
}
impl allocator_traits_250 {
    pub fn construct_252(
        __a: Ptr<allocator_249>,
        mut __p: Ptr<vector_237>,
        __args: Ptr<vector_237>,
    ) {
        unimplemented!()
    }
}
impl allocator_traits_250 {
    pub fn destroy_253(__a: Ptr<allocator_249>, mut __p: Ptr<vector_237>) {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_traits_254 {}
impl Clone for allocator_traits_254 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_traits_254 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_traits_254 {
    pub fn allocate_1(__a: Ptr<allocator_248>, mut __n: u64) -> Ptr<i32> {
        unimplemented!()
    }
}
impl allocator_traits_254 {
    pub fn deallocate(__a: Ptr<allocator_248>, mut __p: Ptr<i32>, mut __n: u64) {
        unimplemented!()
    }
}
impl allocator_traits_254 {
    pub fn max_size(__a: Ptr<allocator_248>) -> u64 {
        unimplemented!()
    }
}
impl allocator_traits_254 {
    pub fn select_on_container_copy_construction(__rhs: Ptr<allocator_248>) -> allocator_248 {
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
pub struct __is_move_insertable_256 {}
impl Clone for __is_move_insertable_256 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_move_insertable_256 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __alloc_traits_257 {}
impl Clone for __alloc_traits_257 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_257 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct rebind_258 {}
impl Clone for rebind_258 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_258 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __alloc_traits_259 {}
impl Clone for __alloc_traits_259 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_259 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct rebind_260 {}
impl Clone for rebind_260 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_260 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __alloc_traits_259 {
    pub fn _S_select_on_copy(__a: Ptr<allocator_248>) -> allocator_248 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_bitwise_relocatable_261 {}
impl Clone for __is_bitwise_relocatable_261 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_bitwise_relocatable_261 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct initializer_list_262 {}
impl Clone for initializer_list_262 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for initializer_list_262 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct initializer_list_263 {}
impl Clone for initializer_list_263 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for initializer_list_263 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl initializer_list_263 {}
impl initializer_list_263 {}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(24)]
pub struct _Vector_base_264 {
    #[offset(0)]
    #[byte_size(24)]
    pub _M_impl: std__Vector_base_std_vector_int___std_allocator_std_vector_int_____Vector_impl,
}
impl Clone for _Vector_base_264 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Vector_base_264 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(24)]
pub struct std__Vector_base_std_vector_int___std_allocator_std_vector_int_____Vector_impl_data {
    #[offset(0)]
    #[byte_size(8)]
    pub _M_start: Ptr<vector_237>,
    #[offset(8)]
    #[byte_size(8)]
    pub _M_finish: Ptr<vector_237>,
    #[offset(16)]
    #[byte_size(8)]
    pub _M_end_of_storage: Ptr<vector_237>,
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
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_std_vector_int___std_allocator_std_vector_int_____Vector_impl_data {
    pub fn move_from(
        __x: Ptr<
            std__Vector_base_std_vector_int___std_allocator_std_vector_int_____Vector_impl_data,
        >,
    ) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(24)]
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
    pub fn new_1() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_std_vector_int___std_allocator_std_vector_int_____Vector_impl {
    pub fn move_from(
        __x: Ptr<std__Vector_base_std_vector_int___std_allocator_std_vector_int_____Vector_impl>,
    ) -> Self {
        unimplemented!()
    }
}
impl _Vector_base_264 {
    pub fn new_1() -> Self {
        unimplemented!()
    }
}
impl _Vector_base_264 {
    pub fn move_from(_a0: Ptr<_Vector_base_264>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(24)]
pub struct _Vector_base_265 {
    #[offset(0)]
    #[byte_size(24)]
    pub _M_impl: std__Vector_base_int__std_allocator_int____Vector_impl,
}
impl Clone for _Vector_base_265 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Vector_base_265 {
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
    pub fn new_1(__a: Ptr<allocator_248>) -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_int__std_allocator_int____Vector_impl {
    pub fn move_from(__x: Ptr<std__Vector_base_int__std_allocator_int____Vector_impl>) -> Self {
        unimplemented!()
    }
}
impl _Vector_base_265 {
    pub fn new_1(__a: Ptr<allocator_248>) -> Self {
        unimplemented!()
    }
}
impl _Vector_base_265 {
    pub fn new_2(mut __n: usize, __a: Ptr<allocator_248>) -> Self {
        unimplemented!()
    }
}
impl _Vector_base_265 {
    pub fn move_from(_a0: Ptr<_Vector_base_265>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(24)]
pub struct vector_237 {}
impl Clone for vector_237 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for vector_237 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl vector_237 {
    pub fn copy_from(__x: Ptr<vector_237>) -> Self {
        unimplemented!()
    }
}
impl vector_237 {
    pub fn move_from(_a0: Ptr<vector_237>) -> Self {
        unimplemented!()
    }
}
impl vector_237 {
    pub fn new_1(__l: initializer_list_263, __a: Option<Ptr<allocator_248>>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(24)]
pub struct vector_266 {}
impl Clone for vector_266 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for vector_266 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(24)]
pub struct std_vector_std_vector_int____Guard_alloc {
    #[offset(0)]
    #[byte_size(8)]
    pub _M_storage: Ptr<vector_237>,
    #[offset(8)]
    pub _M_len: u64,
    #[offset(16)]
    #[byte_size(8)]
    pub _M_vect: Ptr<_Vector_base_264>,
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
    pub fn new(mut __s: Ptr<vector_237>, mut __l: u64, __vect: Ptr<_Vector_base_264>) -> Self {
        unimplemented!()
    }
}
impl vector_266 {
    pub fn new_1() -> Self {
        unimplemented!()
    }
}
impl vector_266 {
    pub fn copy_from(__x: Ptr<vector_266>) -> Self {
        unimplemented!()
    }
}
impl vector_266 {
    pub fn move_from(_a0: Ptr<vector_266>) -> Self {
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
pub type anon_291 = u32;
pub const anon_291__S_word_bit: anon_291 = 64;
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
pub struct binary_function_292 {}
impl Clone for binary_function_292 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for binary_function_292 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct greater_293 {}
impl Clone for greater_293 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for greater_293 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct less_294 {}
impl Clone for less_294 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for less_294 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct greater_equal_295 {}
impl Clone for greater_equal_295 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for greater_equal_295 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct less_equal_296 {}
impl Clone for less_equal_296 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for less_equal_296 {
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
    pub static allocator_arg_297: Value<std_allocator_arg_t> = unimplemented!();
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
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let v1: Value<vector_266> = Rc::new(RefCell::new(<vector_266>::default()));
    let v2: Value<vector_237> = Rc::new(RefCell::new(vector_237::new_1(
        { __cpp2rust_unimplemented::<initializer_list_263>() },
        None,
    )));
    ({ vector_266Impl::push_back_57(&v1.as_pointer(), v2.as_pointer()) });
    (if ((({ vector_237Impl::at_49(&({ vector_266Impl::back_53(&v1.as_pointer()) }), 0_u64) })
        .read())
        == 1)
    {
        &(0);
    } else {
        ({
            __assert_fail_298(
                Ptr::<i8>::from_string_literal(b"v1.back().at(0) == 1"),
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
pub fn __assert_fail_298(
    mut __assertion: Ptr<i8>,
    mut __file: Ptr<i8>,
    mut __line: u32,
    mut __function: Ptr<i8>,
) {
    unimplemented!()
}
pub trait _Vector_base_264Impl {
    fn _M_get_Tp_allocator_1(&self) -> Ptr<allocator_249> {
        unimplemented!()
    }
    fn _M_get_Tp_allocator_2(&self) -> Ptr<allocator_249> {
        unimplemented!()
    }
    fn _M_allocate(&self, mut __n: usize) -> Ptr<vector_237> {
        unimplemented!()
    }
    fn _M_deallocate(&self, mut __p: Ptr<vector_237>, mut __n: usize) {
        unimplemented!()
    }
}
impl _Vector_base_264Impl for Ptr<_Vector_base_264> {}
pub trait _Vector_base_265Impl {
    fn _M_get_Tp_allocator_1(&self) -> Ptr<allocator_248> {
        unimplemented!()
    }
    fn _M_get_Tp_allocator_2(&self) -> Ptr<allocator_248> {
        unimplemented!()
    }
    fn _M_allocate(&self, mut __n: usize) -> Ptr<i32> {
        unimplemented!()
    }
    fn _M_deallocate(&self, mut __p: Ptr<i32>, mut __n: usize) {
        unimplemented!()
    }
}
impl _Vector_base_265Impl for Ptr<_Vector_base_265> {}
pub trait __new_allocator_242Impl {
    fn allocate(&self, mut __n: u64, _a1: Option<AnyPtr>) -> Ptr<vector_237> {
        unimplemented!()
    }
    fn deallocate(&self, mut __p: Ptr<vector_237>, mut __n: u64) {
        unimplemented!()
    }
    fn max_size(&self) -> u64 {
        unimplemented!()
    }
    fn construct_243(&self, mut __p: Ptr<vector_237>, __args: Ptr<vector_237>) {
        unimplemented!()
    }
    fn construct_244(&self, mut __p: Ptr<vector_237>, __args: Ptr<vector_237>) {
        unimplemented!()
    }
    fn destroy_245(&self, mut __p: Ptr<vector_237>) {
        unimplemented!()
    }
}
impl __new_allocator_242Impl for Ptr<__new_allocator_242> {}
pub trait __new_allocator_246Impl {
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
impl __new_allocator_246Impl for Ptr<__new_allocator_246> {}
pub trait __normal_iterator_236Impl {
    fn operator_deref(&self) -> Ptr<vector_237> {
        unimplemented!()
    }
    fn operator_sub(&self, mut __n: i64) -> __normal_iterator_236 {
        unimplemented!()
    }
    fn base(&self) -> Ptr<Ptr<vector_237>> {
        unimplemented!()
    }
}
impl __normal_iterator_236Impl for Ptr<__normal_iterator_236> {}
pub trait __normal_iterator_238Impl {
    fn base(&self) -> Ptr<Ptr<vector_237>> {
        unimplemented!()
    }
}
impl __normal_iterator_238Impl for Ptr<__normal_iterator_238> {}
pub trait __normal_iterator_239Impl {
    fn base(&self) -> Ptr<Ptr<i32>> {
        unimplemented!()
    }
}
impl __normal_iterator_239Impl for Ptr<__normal_iterator_239> {}
pub trait initializer_list_262Impl {
    fn size(&self) -> u64 {
        unimplemented!()
    }
    fn begin(&self) -> Ptr<bool> {
        unimplemented!()
    }
}
impl initializer_list_262Impl for Ptr<initializer_list_262> {}
pub trait initializer_list_263Impl {
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
impl initializer_list_263Impl for Ptr<initializer_list_263> {}
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
pub trait vector_237Impl {
    fn copy_assign(&self, __x: Ptr<vector_237>) -> Ptr<vector_237> {
        unimplemented!()
    }
    fn move_assign(&self, __x: Ptr<vector_237>) -> Ptr<vector_237> {
        unimplemented!()
    }
    fn begin_27(&self) -> __normal_iterator_239 {
        unimplemented!()
    }
    fn end_29(&self) -> __normal_iterator_239 {
        unimplemented!()
    }
    fn size(&self) -> u64 {
        unimplemented!()
    }
    fn operator_index_46(&self, mut __n: u64) -> Ptr<i32> {
        unimplemented!()
    }
    fn at_49(&self, mut __n: u64) -> Ptr<i32> {
        unimplemented!()
    }
}
impl vector_237Impl for Ptr<vector_237> {}
pub trait vector_266Impl {
    fn copy_assign(&self, __x: Ptr<vector_266>) -> Ptr<vector_266> {
        unimplemented!()
    }
    fn move_assign(&self, __x: Ptr<vector_266>) -> Ptr<vector_266> {
        unimplemented!()
    }
    fn begin_26(&self) -> __normal_iterator_236 {
        unimplemented!()
    }
    fn begin_27(&self) -> __normal_iterator_238 {
        unimplemented!()
    }
    fn end_28(&self) -> __normal_iterator_236 {
        unimplemented!()
    }
    fn end_29(&self) -> __normal_iterator_238 {
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
    fn back_53(&self) -> Ptr<vector_237> {
        unimplemented!()
    }
    fn push_back_57(&self, __x: Ptr<vector_237>) {
        unimplemented!()
    }
}
impl vector_266Impl for Ptr<vector_266> {}
pub fn __cpp2rust_init_globals() {}
