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
pub struct __not__36 {}
impl Clone for __not__36 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__36 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __not__37 {}
impl Clone for __not__37 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__37 {
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
pub struct is_reference_42 {}
impl Clone for is_reference_42 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_42 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_43 {}
impl Clone for is_reference_43 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_43 {
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
pub struct is_function_48 {}
impl Clone for is_function_48 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_48 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_49 {}
impl Clone for is_function_49 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_49 {
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
pub struct is_void_55 {}
impl Clone for is_void_55 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_55 {
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
pub struct is_integral_88 {}
impl Clone for is_integral_88 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_88 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_floating_point_helper_89 {}
impl Clone for __is_floating_point_helper_89 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_floating_point_helper_89 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_floating_point_90 {}
impl Clone for is_floating_point_90 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_floating_point_90 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_pointer_91 {}
impl Clone for is_pointer_91 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_pointer_91 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_pointer_92 {}
impl Clone for is_pointer_92 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_pointer_92 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_pointer_93 {}
impl Clone for is_pointer_93 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_pointer_93 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_enum_94 {}
impl Clone for is_enum_94 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_enum_94 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_null_pointer_95 {}
impl Clone for is_null_pointer_95 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_null_pointer_95 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_arithmetic_96 {}
impl Clone for is_arithmetic_96 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_arithmetic_96 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_arithmetic_97 {}
impl Clone for is_arithmetic_97 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_arithmetic_97 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_member_pointer_98 {}
impl Clone for is_member_pointer_98 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_member_pointer_98 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_scalar_99 {}
impl Clone for is_scalar_99 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_scalar_99 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_scalar_100 {}
impl Clone for is_scalar_100 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_scalar_100 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_101 {}
impl Clone for is_same_101 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_101 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_102 {}
impl Clone for is_same_102 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_102 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_103 {}
impl Clone for is_same_103 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_103 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_104 {}
impl Clone for is_same_104 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_104 {
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
pub struct is_volatile_106 {}
impl Clone for is_volatile_106 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_106 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_volatile_107 {}
impl Clone for is_volatile_107 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_107 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_volatile_108 {}
impl Clone for is_volatile_108 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_108 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_volatile_109 {}
impl Clone for is_volatile_109 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_109 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_all_extents_110 {}
impl Clone for remove_all_extents_110 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_all_extents_110 {
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
pub struct __is_destructible_impl_111 {}
impl Clone for __is_destructible_impl_111 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_destructible_impl_111 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_destructible_safe_112 {}
impl Clone for __is_destructible_safe_112 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_destructible_safe_112 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_destructible_safe_113 {}
impl Clone for __is_destructible_safe_113 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_destructible_safe_113 {
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
pub struct __is_nt_destructible_impl_114 {}
impl Clone for __is_nt_destructible_impl_114 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_nt_destructible_impl_114 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_nt_destructible_safe_115 {}
impl Clone for __is_nt_destructible_safe_115 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_nt_destructible_safe_115 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_destructible_116 {}
impl Clone for is_nothrow_destructible_116 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_destructible_116 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_constructible_117 {}
impl Clone for is_constructible_117 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_constructible_117 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_constructible_118 {}
impl Clone for is_constructible_118 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_constructible_118 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_move_constructible_119 {}
impl Clone for is_move_constructible_119 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_move_constructible_119 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_move_constructible_120 {}
impl Clone for is_move_constructible_120 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_move_constructible_120 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_default_constructible_121 {}
impl Clone for is_nothrow_default_constructible_121 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_default_constructible_121 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_default_constructible_122 {}
impl Clone for is_nothrow_default_constructible_122 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_default_constructible_122 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_copy_constructible_123 {}
impl Clone for is_nothrow_copy_constructible_123 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_copy_constructible_123 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_copy_constructible_124 {}
impl Clone for is_nothrow_copy_constructible_124 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_copy_constructible_124 {
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
pub struct is_trivially_destructible_125 {}
impl Clone for is_trivially_destructible_125 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_trivially_destructible_125 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_trivially_destructible_126 {}
impl Clone for is_trivially_destructible_126 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_trivially_destructible_126 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_nothrow_new_constructible_impl_127 {}
impl Clone for __is_nothrow_new_constructible_impl_127 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_nothrow_new_constructible_impl_127 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_nothrow_new_constructible_impl_128 {}
impl Clone for __is_nothrow_new_constructible_impl_128 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_nothrow_new_constructible_impl_128 {
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
pub struct __cv_selector_134 {}
impl Clone for __cv_selector_134 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_134 {
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
pub struct __make_unsigned_143 {}
impl Clone for __make_unsigned_143 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_143 {
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
    pub static std___make_unsigned_selector_base__List_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long____size_156 : Value<usize > = unimplemented!() ;
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
thread_local!(
    pub static std___make_unsigned_selector_base__List_unsigned_short__unsigned_int__unsigned_long__unsigned_long_long____size_158 : Value<usize > = unimplemented!() ;
);
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
thread_local!(
    pub static std___make_unsigned_selector_base__List_unsigned_int__unsigned_long__unsigned_long_long____size_160 : Value<usize > = unimplemented!() ;
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_161 {}
impl Clone for _List_161 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_161 {
    fn default() -> Self {
        unimplemented!()
    }
}
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
pub struct __make_signed_169 {}
impl Clone for __make_signed_169 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_169 {
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
pub struct common_type_176 {}
impl Clone for common_type_176 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for common_type_176 {
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
    pub static __is_nothrow_new_constructible_177: Value<bool> = unimplemented!();
);
thread_local!(
    pub static __is_nothrow_new_constructible_178: Value<bool> = unimplemented!();
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
pub struct __are_same_179 {}
impl Clone for __are_same_179 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_179 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_180 = u32;
pub const anon_180___value: anon_180 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_181 {}
impl Clone for __are_same_181 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_181 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_182 = u32;
pub const anon_182___value: anon_182 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_183 {}
impl Clone for __are_same_183 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_183 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_184 = u32;
pub const anon_184___value: anon_184 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_185 {}
impl Clone for __are_same_185 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_185 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_186 = u32;
pub const anon_186___value: anon_186 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_187 {}
impl Clone for __are_same_187 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_187 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_188 = u32;
pub const anon_188___value: anon_188 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_189 {}
impl Clone for __are_same_189 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_189 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_190 = u32;
pub const anon_190___value: anon_190 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_191 {}
impl Clone for __is_integer_191 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_191 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_192 = u32;
pub const anon_192___value: anon_192 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_193 {}
impl Clone for __is_integer_193 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_193 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_194 = u32;
pub const anon_194___value: anon_194 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_195 {}
impl Clone for __is_integer_195 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_195 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_196 = u32;
pub const anon_196___value: anon_196 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_197 {}
impl Clone for __is_integer_197 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_197 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_198 = u32;
pub const anon_198___value: anon_198 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_199 {}
impl Clone for __is_integer_199 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_199 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_200 = u32;
pub const anon_200___value: anon_200 = 0;
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
pub struct __ptr_traits_ptr_to_233 {}
impl Clone for __ptr_traits_ptr_to_233 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_233 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct pointer_traits_234 {}
impl Clone for pointer_traits_234 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_234 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct pointer_traits_235 {}
impl Clone for pointer_traits_235 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_235 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct reverse_iterator_236 {}
impl Clone for reverse_iterator_236 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for reverse_iterator_236 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_236 {
    pub fn copy_from(__x: Ptr<reverse_iterator_236>) -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_236 {
    pub fn copy_assign(&self, _a0: Ptr<reverse_iterator_236>) -> Ptr<reverse_iterator_236> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct reverse_iterator_237 {}
impl Clone for reverse_iterator_237 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for reverse_iterator_237 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_237 {
    pub fn copy_from(__x: Ptr<reverse_iterator_237>) -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_237 {
    pub fn copy_assign(&self, _a0: Ptr<reverse_iterator_237>) -> Ptr<reverse_iterator_237> {
        unimplemented!()
    }
}
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
    pub fn new_1(__i: Ptr<Ptr<i32>>) -> Self {
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
#[byte_size(8)]
pub struct __normal_iterator_240 {}
impl Clone for __normal_iterator_240 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __normal_iterator_240 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_240 {
    pub fn new_1(__i: Ptr<Ptr<vector_241>>) -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_240 {}
impl __normal_iterator_240 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
pub struct __normal_iterator_242 {}
impl Clone for __normal_iterator_242 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __normal_iterator_242 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_242 {
    pub fn new_1(__i: Ptr<Ptr<vector_241>>) -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_242 {}
impl __normal_iterator_242 {}
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
pub struct __new_allocator_243 {}
impl Clone for __new_allocator_243 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_243 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_243 {
    pub fn copy_from(_a0: Ptr<__new_allocator_243>) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_243 {
    pub fn copy_assign(&self, _a0: Ptr<__new_allocator_243>) -> Ptr<__new_allocator_243> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __new_allocator_244 {}
impl Clone for __new_allocator_244 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_244 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_244 {
    pub fn copy_from(_a0: Ptr<__new_allocator_244>) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_244 {
    pub fn copy_assign(&self, _a0: Ptr<__new_allocator_244>) -> Ptr<__new_allocator_244> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __new_allocator_245 {}
impl Clone for __new_allocator_245 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_245 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_245 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_245 {
    pub fn copy_from(_a0: Ptr<__new_allocator_245>) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_245 {
    pub fn copy_assign(&self, _a0: Ptr<__new_allocator_245>) -> Ptr<__new_allocator_245> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __new_allocator_247 {}
impl Clone for __new_allocator_247 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_247 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_247 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_247 {
    pub fn copy_from(_a0: Ptr<__new_allocator_247>) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_247 {
    pub fn copy_assign(&self, _a0: Ptr<__new_allocator_247>) -> Ptr<__new_allocator_247> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_250 {}
impl Clone for allocator_250 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_250 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_250 {}
impl allocator_250 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_251 {}
impl Clone for allocator_251 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_251 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_251 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl allocator_251 {
    pub fn copy_from(__a: Ptr<allocator_251>) -> Self {
        unimplemented!()
    }
}
impl allocator_251 {
    pub fn copy_assign(&self, _a0: Ptr<allocator_251>) -> Ptr<allocator_251> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_252 {}
impl Clone for allocator_252 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_252 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_252 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl allocator_252 {
    pub fn copy_from(__a: Ptr<allocator_252>) -> Self {
        unimplemented!()
    }
}
impl allocator_252 {
    pub fn copy_assign(&self, _a0: Ptr<allocator_252>) -> Ptr<allocator_252> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_traits_253 {}
impl Clone for allocator_traits_253 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_traits_253 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_traits_253 {
    pub fn allocate_1(__a: Ptr<allocator_251>, mut __n: u64) -> Ptr<i32> {
        unimplemented!()
    }
}
impl allocator_traits_253 {
    pub fn deallocate(__a: Ptr<allocator_251>, mut __p: Ptr<i32>, mut __n: u64) {
        unimplemented!()
    }
}
impl allocator_traits_253 {
    pub fn max_size(__a: Ptr<allocator_251>) -> u64 {
        unimplemented!()
    }
}
impl allocator_traits_253 {
    pub fn construct_254(__a: Ptr<allocator_251>, mut __p: Ptr<i32>, __args: Ptr<i32>) {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_traits_255 {}
impl Clone for allocator_traits_255 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_traits_255 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_traits_255 {
    pub fn allocate_1(__a: Ptr<allocator_252>, mut __n: u64) -> Ptr<vector_241> {
        unimplemented!()
    }
}
impl allocator_traits_255 {
    pub fn deallocate(__a: Ptr<allocator_252>, mut __p: Ptr<vector_241>, mut __n: u64) {
        unimplemented!()
    }
}
impl allocator_traits_255 {
    pub fn max_size(__a: Ptr<allocator_252>) -> u64 {
        unimplemented!()
    }
}
impl allocator_traits_255 {
    pub fn construct_256(
        __a: Ptr<allocator_252>,
        mut __p: Ptr<vector_241>,
        __args: Ptr<vector_241>,
    ) {
        unimplemented!()
    }
}
impl allocator_traits_255 {
    pub fn destroy_257(__a: Ptr<allocator_252>, mut __p: Ptr<vector_241>) {
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
pub struct __is_move_insertable_259 {}
impl Clone for __is_move_insertable_259 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_move_insertable_259 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_move_insertable_260 {}
impl Clone for __is_move_insertable_260 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_move_insertable_260 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __alloc_traits_261 {}
impl Clone for __alloc_traits_261 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_261 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct rebind_262 {}
impl Clone for rebind_262 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_262 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __alloc_traits_263 {}
impl Clone for __alloc_traits_263 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_263 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct rebind_264 {}
impl Clone for rebind_264 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_264 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_bitwise_relocatable_265 {}
impl Clone for __is_bitwise_relocatable_265 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_bitwise_relocatable_265 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_bitwise_relocatable_266 {}
impl Clone for __is_bitwise_relocatable_266 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_bitwise_relocatable_266 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct initializer_list_267 {}
impl Clone for initializer_list_267 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for initializer_list_267 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct initializer_list_268 {}
impl Clone for initializer_list_268 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for initializer_list_268 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl initializer_list_268 {}
impl initializer_list_268 {}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(24)]
pub struct _Vector_base_269 {
    #[offset(0)]
    #[byte_size(24)]
    pub _M_impl: std__Vector_base_int__std_allocator_int____Vector_impl,
}
impl Clone for _Vector_base_269 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Vector_base_269 {
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
    pub fn move_from(__x: Ptr<std__Vector_base_int__std_allocator_int____Vector_impl>) -> Self {
        unimplemented!()
    }
}
impl _Vector_base_269 {
    pub fn new_1() -> Self {
        unimplemented!()
    }
}
impl _Vector_base_269 {
    pub fn move_from(_a0: Ptr<_Vector_base_269>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(24)]
pub struct _Vector_base_270 {
    #[offset(0)]
    #[byte_size(24)]
    pub _M_impl: std__Vector_base_std_vector_int___std_allocator_std_vector_int_____Vector_impl,
}
impl Clone for _Vector_base_270 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Vector_base_270 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(24)]
pub struct std__Vector_base_std_vector_int___std_allocator_std_vector_int_____Vector_impl_data {
    #[offset(0)]
    #[byte_size(8)]
    pub _M_start: Ptr<vector_241>,
    #[offset(8)]
    #[byte_size(8)]
    pub _M_finish: Ptr<vector_241>,
    #[offset(16)]
    #[byte_size(8)]
    pub _M_end_of_storage: Ptr<vector_241>,
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
impl _Vector_base_270 {
    pub fn new_1() -> Self {
        unimplemented!()
    }
}
impl _Vector_base_270 {
    pub fn move_from(_a0: Ptr<_Vector_base_270>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(24)]
pub struct vector_241 {}
impl Clone for vector_241 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for vector_241 {
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
    pub _M_vect: Ptr<_Vector_base_269>,
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
    pub fn new(mut __s: Ptr<i32>, mut __l: u64, __vect: Ptr<_Vector_base_269>) -> Self {
        unimplemented!()
    }
}
impl vector_241 {
    pub fn new_1() -> Self {
        unimplemented!()
    }
}
impl vector_241 {
    pub fn copy_from(__x: Ptr<vector_241>) -> Self {
        unimplemented!()
    }
}
impl vector_241 {
    pub fn move_from(_a0: Ptr<vector_241>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(24)]
pub struct vector_272 {}
impl Clone for vector_272 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for vector_272 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(24)]
pub struct std_vector_std_vector_int____Guard_alloc {
    #[offset(0)]
    #[byte_size(8)]
    pub _M_storage: Ptr<vector_241>,
    #[offset(8)]
    pub _M_len: u64,
    #[offset(16)]
    #[byte_size(8)]
    pub _M_vect: Ptr<_Vector_base_270>,
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
    pub fn new(mut __s: Ptr<vector_241>, mut __l: u64, __vect: Ptr<_Vector_base_270>) -> Self {
        unimplemented!()
    }
}
impl vector_272 {
    pub fn new_1() -> Self {
        unimplemented!()
    }
}
impl vector_272 {
    pub fn copy_from(__x: Ptr<vector_272>) -> Self {
        unimplemented!()
    }
}
impl vector_272 {
    pub fn move_from(_a0: Ptr<vector_272>) -> Self {
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
pub struct __hash_base_288 {}
impl Clone for __hash_base_288 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_288 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_289 {}
impl Clone for __hash_base_289 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_289 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_290 {}
impl Clone for __hash_base_290 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_290 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_291 {}
impl Clone for __hash_base_291 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_291 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_292 {}
impl Clone for __hash_base_292 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_292 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_293 {}
impl Clone for __hash_base_293 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_293 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_294 {}
impl Clone for __hash_base_294 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_294 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct hash_295 {}
impl Clone for hash_295 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for hash_295 {
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
    pub fn hash_296(__val: Ptr<f32>) -> usize {
        unimplemented!()
    }
}
impl std__Hash_impl {
    pub fn hash_297(__val: Ptr<f64>) -> usize {
        unimplemented!()
    }
}
pub type anon_298 = u32;
pub const anon_298__S_word_bit: anon_298 = 64;
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
pub struct binary_function_299 {}
impl Clone for binary_function_299 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for binary_function_299 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct greater_300 {}
impl Clone for greater_300 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for greater_300 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct less_301 {}
impl Clone for less_301 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for less_301 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct greater_equal_302 {}
impl Clone for greater_equal_302 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for greater_equal_302 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct less_equal_303 {}
impl Clone for less_equal_303 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for less_equal_303 {
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
    pub static allocator_arg_304: Value<std_allocator_arg_t> = unimplemented!();
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
    let v: Value<vector_241> = Rc::new(RefCell::new(<vector_241>::default()));
    ({
        let ___x: Value<i32> = Rc::new(RefCell::new(1));
        vector_241Impl::push_back_58(&v.as_pointer(), ___x.as_pointer())
    });
    ({
        let ___x: Value<i32> = Rc::new(RefCell::new(2));
        vector_241Impl::push_back_58(&v.as_pointer(), ___x.as_pointer())
    });
    ({
        let ___x: Value<i32> = Rc::new(RefCell::new(3));
        vector_241Impl::push_back_58(&v.as_pointer(), ___x.as_pointer())
    });
    let mut square: i32 = 0;
    {
        let __range1: Ptr<vector_241> = v.as_pointer();
        let __begin1: Value<__normal_iterator_238> =
            Rc::new(RefCell::new(({ vector_241Impl::begin_26(&__range1) })));
        let __end1: Value<__normal_iterator_238> =
            Rc::new(RefCell::new(({ vector_241Impl::end_28(&__range1) })));
        'loop_: while ({
            let ___lhs: Ptr<__normal_iterator_238> = __begin1.as_pointer();
            operator_ne_305(___lhs, __end1.as_pointer())
        }) {
            let mut e1: i32 =
                (({ __normal_iterator_238Impl::operator_deref(&__begin1.as_pointer()) }).read());
            {
                let __range2: Ptr<vector_241> = v.as_pointer();
                let __begin2: Value<__normal_iterator_238> =
                    Rc::new(RefCell::new(({ vector_241Impl::begin_26(&__range2) })));
                let __end2: Value<__normal_iterator_238> =
                    Rc::new(RefCell::new(({ vector_241Impl::end_28(&__range2) })));
                'loop_: while ({
                    let ___lhs: Ptr<__normal_iterator_238> = __begin2.as_pointer();
                    operator_ne_305(___lhs, __end2.as_pointer())
                }) {
                    let mut e2: i32 =
                        (({ __normal_iterator_238Impl::operator_deref(&__begin2.as_pointer()) })
                            .read());
                    square += (e1 * e2);
                    ({ __normal_iterator_238Impl::operator_inc_6(&__begin2.as_pointer()) });
                }
            }
            ({ __normal_iterator_238Impl::operator_inc_6(&__begin1.as_pointer()) });
        }
    }
    {
        let __range1: Ptr<vector_241> = v.as_pointer();
        let __begin1: Value<__normal_iterator_238> =
            Rc::new(RefCell::new(({ vector_241Impl::begin_26(&__range1) })));
        let __end1: Value<__normal_iterator_238> =
            Rc::new(RefCell::new(({ vector_241Impl::end_28(&__range1) })));
        'loop_: while ({
            let ___lhs: Ptr<__normal_iterator_238> = __begin1.as_pointer();
            operator_ne_305(___lhs, __end1.as_pointer())
        }) {
            let e1: Ptr<i32> =
                ({ __normal_iterator_238Impl::operator_deref(&__begin1.as_pointer()) });
            {
                let __range2: Ptr<vector_241> = v.as_pointer();
                let __begin2: Value<__normal_iterator_238> =
                    Rc::new(RefCell::new(({ vector_241Impl::begin_26(&__range2) })));
                let __end2: Value<__normal_iterator_238> =
                    Rc::new(RefCell::new(({ vector_241Impl::end_28(&__range2) })));
                'loop_: while ({
                    let ___lhs: Ptr<__normal_iterator_238> = __begin2.as_pointer();
                    operator_ne_305(___lhs, __end2.as_pointer())
                }) {
                    let e2: Ptr<i32> =
                        ({ __normal_iterator_238Impl::operator_deref(&__begin2.as_pointer()) });
                    square += { ({ (e1.read()) } * { (e2.read()) }) };
                    ({ __normal_iterator_238Impl::operator_inc_6(&__begin2.as_pointer()) });
                }
            }
            ({ __normal_iterator_238Impl::operator_inc_6(&__begin1.as_pointer()) });
        }
    }
    {
        let __range1: Ptr<vector_241> = v.as_pointer();
        let __begin1: Value<__normal_iterator_238> =
            Rc::new(RefCell::new(({ vector_241Impl::begin_26(&__range1) })));
        let __end1: Value<__normal_iterator_238> =
            Rc::new(RefCell::new(({ vector_241Impl::end_28(&__range1) })));
        'loop_: while ({
            let ___lhs: Ptr<__normal_iterator_238> = __begin1.as_pointer();
            operator_ne_305(___lhs, __end1.as_pointer())
        }) {
            let e1: Ptr<i32> =
                ({ __normal_iterator_238Impl::operator_deref(&__begin1.as_pointer()) });
            {
                let __range2: Ptr<vector_241> = v.as_pointer();
                let __begin2: Value<__normal_iterator_238> =
                    Rc::new(RefCell::new(({ vector_241Impl::begin_26(&__range2) })));
                let __end2: Value<__normal_iterator_238> =
                    Rc::new(RefCell::new(({ vector_241Impl::end_28(&__range2) })));
                'loop_: while ({
                    let ___lhs: Ptr<__normal_iterator_238> = __begin2.as_pointer();
                    operator_ne_305(___lhs, __end2.as_pointer())
                }) {
                    let e2: Ptr<i32> =
                        ({ __normal_iterator_238Impl::operator_deref(&__begin2.as_pointer()) });
                    square += { ({ (e1.read()) } * { (e2.read()) }) };
                    ({ __normal_iterator_238Impl::operator_inc_6(&__begin2.as_pointer()) });
                }
            }
            ({ __normal_iterator_238Impl::operator_inc_6(&__begin1.as_pointer()) });
        }
    }
    {
        let __range1: Ptr<vector_241> = v.as_pointer();
        let __begin1: Value<__normal_iterator_238> =
            Rc::new(RefCell::new(({ vector_241Impl::begin_26(&__range1) })));
        let __end1: Value<__normal_iterator_238> =
            Rc::new(RefCell::new(({ vector_241Impl::end_28(&__range1) })));
        'loop_: while ({
            let ___lhs: Ptr<__normal_iterator_238> = __begin1.as_pointer();
            operator_ne_305(___lhs, __end1.as_pointer())
        }) {
            let e1: Ptr<i32> =
                ({ __normal_iterator_238Impl::operator_deref(&__begin1.as_pointer()) });
            {
                let __range2: Ptr<vector_241> = v.as_pointer();
                let __begin2: Value<__normal_iterator_238> =
                    Rc::new(RefCell::new(({ vector_241Impl::begin_26(&__range2) })));
                let __end2: Value<__normal_iterator_238> =
                    Rc::new(RefCell::new(({ vector_241Impl::end_28(&__range2) })));
                'loop_: while ({
                    let ___lhs: Ptr<__normal_iterator_238> = __begin2.as_pointer();
                    operator_ne_305(___lhs, __end2.as_pointer())
                }) {
                    let e2: Ptr<i32> =
                        ({ __normal_iterator_238Impl::operator_deref(&__begin2.as_pointer()) });
                    square += { ({ (e1.read()) } * { (e2.read()) }) };
                    ({ __normal_iterator_238Impl::operator_inc_6(&__begin2.as_pointer()) });
                }
            }
            ({ __normal_iterator_238Impl::operator_inc_6(&__begin1.as_pointer()) });
        }
    }
    let m: Value<vector_272> = Rc::new(RefCell::new(<vector_272>::default()));
    let v1: Value<vector_241> = Rc::new(RefCell::new(<vector_241>::default()));
    ({ vector_272Impl::push_back_58(&m.as_pointer(), v1.as_pointer()) });
    let v2: Value<vector_241> = Rc::new(RefCell::new(<vector_241>::default()));
    ({ vector_272Impl::push_back_58(&m.as_pointer(), v2.as_pointer()) });
    let v3: Value<vector_241> = Rc::new(RefCell::new(<vector_241>::default()));
    ({ vector_272Impl::push_back_58(&m.as_pointer(), v3.as_pointer()) });
    {
        let __range1: Ptr<vector_272> = m.as_pointer();
        let __begin1: Value<__normal_iterator_240> =
            Rc::new(RefCell::new(({ vector_272Impl::begin_26(&__range1) })));
        let __end1: Value<__normal_iterator_240> =
            Rc::new(RefCell::new(({ vector_272Impl::end_28(&__range1) })));
        'loop_: while ({
            let ___lhs: Ptr<__normal_iterator_240> = __begin1.as_pointer();
            operator_ne_306(___lhs, __end1.as_pointer())
        }) {
            let row: Ptr<vector_241> =
                ({ __normal_iterator_240Impl::operator_deref(&__begin1.as_pointer()) });
            {
                let __range2: Ptr<vector_241> = (row).clone();
                let __begin2: Value<__normal_iterator_238> =
                    Rc::new(RefCell::new(({ vector_241Impl::begin_26(&__range2) })));
                let __end2: Value<__normal_iterator_238> =
                    Rc::new(RefCell::new(({ vector_241Impl::end_28(&__range2) })));
                'loop_: while ({
                    let ___lhs: Ptr<__normal_iterator_238> = __begin2.as_pointer();
                    operator_ne_305(___lhs, __end2.as_pointer())
                }) {
                    let col: Ptr<i32> =
                        ({ __normal_iterator_238Impl::operator_deref(&__begin2.as_pointer()) });
                    square += { (col.read()) };
                    ({ __normal_iterator_238Impl::operator_inc_6(&__begin2.as_pointer()) });
                }
            }
            ({ __normal_iterator_240Impl::operator_inc_6(&__begin1.as_pointer()) });
        }
    }
    (if (square == 144) {
        &(0);
    } else {
        ({
            __assert_fail_307(
                Ptr::<i8>::from_string_literal(b"square == 144"),
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
pub fn operator_ne_305(
    __lhs: Ptr<__normal_iterator_238>,
    __rhs: Ptr<__normal_iterator_238>,
) -> bool {
    unimplemented!()
}
pub fn operator_ne_306(
    __lhs: Ptr<__normal_iterator_240>,
    __rhs: Ptr<__normal_iterator_240>,
) -> bool {
    unimplemented!()
}
pub fn __assert_fail_307(
    mut __assertion: Ptr<i8>,
    mut __file: Ptr<i8>,
    mut __line: u32,
    mut __function: Ptr<i8>,
) {
    unimplemented!()
}
pub trait _Vector_base_269Impl {
    fn _M_get_Tp_allocator_1(&self) -> Ptr<allocator_251> {
        unimplemented!()
    }
    fn _M_get_Tp_allocator_2(&self) -> Ptr<allocator_251> {
        unimplemented!()
    }
    fn _M_allocate(&self, mut __n: usize) -> Ptr<i32> {
        unimplemented!()
    }
    fn _M_deallocate(&self, mut __p: Ptr<i32>, mut __n: usize) {
        unimplemented!()
    }
}
impl _Vector_base_269Impl for Ptr<_Vector_base_269> {}
pub trait _Vector_base_270Impl {
    fn _M_get_Tp_allocator_1(&self) -> Ptr<allocator_252> {
        unimplemented!()
    }
    fn _M_get_Tp_allocator_2(&self) -> Ptr<allocator_252> {
        unimplemented!()
    }
    fn _M_allocate(&self, mut __n: usize) -> Ptr<vector_241> {
        unimplemented!()
    }
    fn _M_deallocate(&self, mut __p: Ptr<vector_241>, mut __n: usize) {
        unimplemented!()
    }
}
impl _Vector_base_270Impl for Ptr<_Vector_base_270> {}
pub trait __new_allocator_245Impl {
    fn allocate(&self, mut __n: u64, _a1: Option<AnyPtr>) -> Ptr<i32> {
        unimplemented!()
    }
    fn deallocate(&self, mut __p: Ptr<i32>, mut __n: u64) {
        unimplemented!()
    }
    fn max_size(&self) -> u64 {
        unimplemented!()
    }
    fn construct_246(&self, mut __p: Ptr<i32>, __args: Ptr<i32>) {
        unimplemented!()
    }
}
impl __new_allocator_245Impl for Ptr<__new_allocator_245> {}
pub trait __new_allocator_247Impl {
    fn allocate(&self, mut __n: u64, _a1: Option<AnyPtr>) -> Ptr<vector_241> {
        unimplemented!()
    }
    fn deallocate(&self, mut __p: Ptr<vector_241>, mut __n: u64) {
        unimplemented!()
    }
    fn max_size(&self) -> u64 {
        unimplemented!()
    }
    fn construct_248(&self, mut __p: Ptr<vector_241>, __args: Ptr<vector_241>) {
        unimplemented!()
    }
    fn destroy_249(&self, mut __p: Ptr<vector_241>) {
        unimplemented!()
    }
}
impl __new_allocator_247Impl for Ptr<__new_allocator_247> {}
pub trait __normal_iterator_238Impl {
    fn operator_deref(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn operator_inc_6(&self) -> Ptr<__normal_iterator_238> {
        unimplemented!()
    }
    fn operator_sub(&self, mut __n: i64) -> __normal_iterator_238 {
        unimplemented!()
    }
    fn base(&self) -> Ptr<Ptr<i32>> {
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
pub trait __normal_iterator_240Impl {
    fn operator_deref(&self) -> Ptr<vector_241> {
        unimplemented!()
    }
    fn operator_inc_6(&self) -> Ptr<__normal_iterator_240> {
        unimplemented!()
    }
    fn operator_sub(&self, mut __n: i64) -> __normal_iterator_240 {
        unimplemented!()
    }
    fn base(&self) -> Ptr<Ptr<vector_241>> {
        unimplemented!()
    }
}
impl __normal_iterator_240Impl for Ptr<__normal_iterator_240> {}
pub trait __normal_iterator_242Impl {
    fn base(&self) -> Ptr<Ptr<vector_241>> {
        unimplemented!()
    }
}
impl __normal_iterator_242Impl for Ptr<__normal_iterator_242> {}
pub trait initializer_list_267Impl {
    fn size(&self) -> u64 {
        unimplemented!()
    }
    fn begin(&self) -> Ptr<bool> {
        unimplemented!()
    }
}
impl initializer_list_267Impl for Ptr<initializer_list_267> {}
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
pub trait vector_241Impl {
    fn copy_assign(&self, __x: Ptr<vector_241>) -> Ptr<vector_241> {
        unimplemented!()
    }
    fn move_assign(&self, __x: Ptr<vector_241>) -> Ptr<vector_241> {
        unimplemented!()
    }
    fn begin_26(&self) -> __normal_iterator_238 {
        unimplemented!()
    }
    fn begin_27(&self) -> __normal_iterator_239 {
        unimplemented!()
    }
    fn end_28(&self) -> __normal_iterator_238 {
        unimplemented!()
    }
    fn end_29(&self) -> __normal_iterator_239 {
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
    fn back_53(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn push_back_58(&self, __x: Ptr<i32>) {
        unimplemented!()
    }
    fn emplace_back_271(&self, __args: Ptr<i32>) -> Ptr<i32> {
        unimplemented!()
    }
}
impl vector_241Impl for Ptr<vector_241> {}
pub trait vector_272Impl {
    fn copy_assign(&self, __x: Ptr<vector_272>) -> Ptr<vector_272> {
        unimplemented!()
    }
    fn move_assign(&self, __x: Ptr<vector_272>) -> Ptr<vector_272> {
        unimplemented!()
    }
    fn begin_26(&self) -> __normal_iterator_240 {
        unimplemented!()
    }
    fn begin_27(&self) -> __normal_iterator_242 {
        unimplemented!()
    }
    fn end_28(&self) -> __normal_iterator_240 {
        unimplemented!()
    }
    fn end_29(&self) -> __normal_iterator_242 {
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
    fn back_53(&self) -> Ptr<vector_241> {
        unimplemented!()
    }
    fn push_back_58(&self, __x: Ptr<vector_241>) {
        unimplemented!()
    }
    fn emplace_back_273(&self, __args: Ptr<vector_241>) -> Ptr<vector_241> {
        unimplemented!()
    }
}
impl vector_272Impl for Ptr<vector_272> {}
pub fn __cpp2rust_init_globals() {}
