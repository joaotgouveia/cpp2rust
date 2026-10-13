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
pub struct __is_floating_point_helper_83 {}
impl Clone for __is_floating_point_helper_83 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_floating_point_helper_83 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_floating_point_84 {}
impl Clone for is_floating_point_84 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_floating_point_84 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_pointer_85 {}
impl Clone for is_pointer_85 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_pointer_85 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_pointer_86 {}
impl Clone for is_pointer_86 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_pointer_86 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_enum_87 {}
impl Clone for is_enum_87 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_enum_87 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_arithmetic_88 {}
impl Clone for is_arithmetic_88 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_arithmetic_88 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_arithmetic_89 {}
impl Clone for is_arithmetic_89 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_arithmetic_89 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_scalar_90 {}
impl Clone for is_scalar_90 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_scalar_90 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_scalar_91 {}
impl Clone for is_scalar_91 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_scalar_91 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_92 {}
impl Clone for is_same_92 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_92 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_93 {}
impl Clone for is_same_93 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_93 {
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
pub struct __is_destructible_safe_100 {}
impl Clone for __is_destructible_safe_100 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_destructible_safe_100 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_destructible_safe_101 {}
impl Clone for __is_destructible_safe_101 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_destructible_safe_101 {
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
pub struct is_constructible_103 {}
impl Clone for is_constructible_103 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_constructible_103 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_move_constructible_104 {}
impl Clone for is_move_constructible_104 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_move_constructible_104 {
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
pub struct is_trivially_destructible_110 {}
impl Clone for is_trivially_destructible_110 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_trivially_destructible_110 {
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
pub struct __is_nothrow_new_constructible_impl_113 {}
impl Clone for __is_nothrow_new_constructible_impl_113 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_nothrow_new_constructible_impl_113 {
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
thread_local!(
    pub static __is_nothrow_new_constructible_162: Value<bool> = unimplemented!();
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
pub struct __are_same_169 {}
impl Clone for __are_same_169 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_169 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_170 = u32;
pub const anon_170___value: anon_170 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_171 {}
impl Clone for __are_same_171 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_171 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_172 = u32;
pub const anon_172___value: anon_172 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_173 {}
impl Clone for __are_same_173 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_173 {
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
pub const anon_176___value: anon_176 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_177 {}
impl Clone for __is_integer_177 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_177 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_178 = u32;
pub const anon_178___value: anon_178 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_179 {}
impl Clone for __is_integer_179 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_179 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_180 = u32;
pub const anon_180___value: anon_180 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_181 {}
impl Clone for __is_integer_181 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_181 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_182 = u32;
pub const anon_182___value: anon_182 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_183 {}
impl Clone for __is_integer_183 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_183 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_184 = u32;
pub const anon_184___value: anon_184 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __memcpyable_integer_185 {}
impl Clone for __memcpyable_integer_185 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __memcpyable_integer_185 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_nonstrict_186 {}
impl Clone for __is_integer_nonstrict_186 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_186 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_187 = u32;
pub const anon_187___width: anon_187 = 64;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_nonstrict_188 {}
impl Clone for __is_integer_nonstrict_188 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_188 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_189 = u32;
pub const anon_189___width: anon_189 = 64;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_integer_190 {}
impl Clone for __numeric_traits_integer_190 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_190 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_long____is_signed_191: Value<bool> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_long____digits_192: Value<i32> = unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_long____max_193: Value<i64> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_integer_194 {}
impl Clone for __numeric_traits_integer_194 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_194 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_long____is_signed_195: Value<bool> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_long____digits_196: Value<i32> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_long____max_197: Value<u64> =
        unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_floating_198 {}
impl Clone for __numeric_traits_floating_198 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_198 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_floating_199 {}
impl Clone for __numeric_traits_floating_199 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_199 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_floating_200 {}
impl Clone for __numeric_traits_floating_200 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_200 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_201 {}
impl Clone for __numeric_traits_201 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_201 {
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
pub struct iterator_204 {}
impl Clone for iterator_204 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_204 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_205 {}
impl Clone for iterator_205 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_205 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl iterator_205 {}
impl iterator_205 {}
impl iterator_205 {}
impl iterator_205 {
    pub fn copy_assign(&self, _a0: Ptr<iterator_205>) -> Ptr<iterator_205> {
        unimplemented!()
    }
}
impl iterator_205 {
    pub fn move_assign(&self, _a0: Ptr<iterator_205>) -> Ptr<iterator_205> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_206 {}
impl Clone for iterator_206 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_206 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl iterator_206 {
    pub fn copy_assign(&self, _a0: Ptr<iterator_206>) -> Ptr<iterator_206> {
        unimplemented!()
    }
}
impl iterator_206 {
    pub fn move_assign(&self, _a0: Ptr<iterator_206>) -> Ptr<iterator_206> {
        unimplemented!()
    }
}
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
impl iterator_207 {
    pub fn copy_assign(&self, _a0: Ptr<iterator_207>) -> Ptr<iterator_207> {
        unimplemented!()
    }
}
impl iterator_207 {
    pub fn move_assign(&self, _a0: Ptr<iterator_207>) -> Ptr<iterator_207> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_208 {}
impl Clone for iterator_traits_208 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_208 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_209 {}
impl Clone for iterator_traits_209 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_209 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_210 {}
impl Clone for iterator_traits_210 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_210 {
    fn default() -> Self {
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
pub struct __iterator_traits_214 {}
impl Clone for __iterator_traits_214 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __iterator_traits_214 {
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
pub struct __ptr_traits_ptr_to_216 {}
impl Clone for __ptr_traits_ptr_to_216 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_216 {
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
#[byte_size(1)]
pub struct pointer_traits_219 {}
impl Clone for pointer_traits_219 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_219 {
    fn default() -> Self {
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
#[byte_size(16)]
pub struct reverse_iterator_221 {}
impl Clone for reverse_iterator_221 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for reverse_iterator_221 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_221 {
    pub fn copy_from(__x: Ptr<reverse_iterator_221>) -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_221 {
    pub fn copy_assign(&self, _a0: Ptr<reverse_iterator_221>) -> Ptr<reverse_iterator_221> {
        unimplemented!()
    }
}
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
    pub fn new_1(__i: Ptr<Ptr<i32>>) -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_222 {}
impl __normal_iterator_222 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
pub struct __normal_iterator_223 {}
impl Clone for __normal_iterator_223 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __normal_iterator_223 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_223 {
    pub fn new_1(__i: Ptr<Ptr<i32>>) -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_223 {}
impl __normal_iterator_223 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
pub struct __normal_iterator_224 {}
impl Clone for __normal_iterator_224 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __normal_iterator_224 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_224 {
    pub fn new_1(__i: Ptr<Ptr<Ptr<i32>>>) -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_224 {}
impl __normal_iterator_224 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
pub struct __normal_iterator_225 {}
impl Clone for __normal_iterator_225 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __normal_iterator_225 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_225 {
    pub fn new_1(__i: Ptr<Ptr<Ptr<i32>>>) -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_225 {}
impl __normal_iterator_225 {}
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
pub struct __new_allocator_227 {}
impl Clone for __new_allocator_227 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_227 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_227 {
    pub fn copy_from(_a0: Ptr<__new_allocator_227>) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_227 {
    pub fn copy_assign(&self, _a0: Ptr<__new_allocator_227>) -> Ptr<__new_allocator_227> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __new_allocator_228 {}
impl Clone for __new_allocator_228 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_228 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_228 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_228 {
    pub fn copy_from(_a0: Ptr<__new_allocator_228>) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_228 {
    pub fn copy_assign(&self, _a0: Ptr<__new_allocator_228>) -> Ptr<__new_allocator_228> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __new_allocator_230 {}
impl Clone for __new_allocator_230 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_230 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_230 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_230 {
    pub fn copy_from(_a0: Ptr<__new_allocator_230>) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_230 {
    pub fn copy_assign(&self, _a0: Ptr<__new_allocator_230>) -> Ptr<__new_allocator_230> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_232 {}
impl Clone for allocator_232 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_232 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_232 {}
impl allocator_232 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_233 {}
impl Clone for allocator_233 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_233 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_233 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl allocator_233 {
    pub fn copy_from(__a: Ptr<allocator_233>) -> Self {
        unimplemented!()
    }
}
impl allocator_233 {
    pub fn copy_assign(&self, _a0: Ptr<allocator_233>) -> Ptr<allocator_233> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_234 {}
impl Clone for allocator_234 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_234 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_234 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl allocator_234 {
    pub fn copy_from(__a: Ptr<allocator_234>) -> Self {
        unimplemented!()
    }
}
impl allocator_234 {
    pub fn copy_assign(&self, _a0: Ptr<allocator_234>) -> Ptr<allocator_234> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_traits_235 {}
impl Clone for allocator_traits_235 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_traits_235 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_traits_235 {
    pub fn allocate_1(__a: Ptr<allocator_233>, mut __n: u64) -> Ptr<i32> {
        unimplemented!()
    }
}
impl allocator_traits_235 {
    pub fn deallocate(__a: Ptr<allocator_233>, mut __p: Ptr<i32>, mut __n: u64) {
        unimplemented!()
    }
}
impl allocator_traits_235 {
    pub fn max_size(__a: Ptr<allocator_233>) -> u64 {
        unimplemented!()
    }
}
impl allocator_traits_235 {
    pub fn construct_236(__a: Ptr<allocator_233>, mut __p: Ptr<i32>, __args: Ptr<i32>) {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_traits_237 {}
impl Clone for allocator_traits_237 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_traits_237 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_traits_237 {
    pub fn allocate_1(__a: Ptr<allocator_234>, mut __n: u64) -> Ptr<Ptr<i32>> {
        unimplemented!()
    }
}
impl allocator_traits_237 {
    pub fn deallocate(__a: Ptr<allocator_234>, mut __p: Ptr<Ptr<i32>>, mut __n: u64) {
        unimplemented!()
    }
}
impl allocator_traits_237 {
    pub fn max_size(__a: Ptr<allocator_234>) -> u64 {
        unimplemented!()
    }
}
impl allocator_traits_237 {
    pub fn construct_238(__a: Ptr<allocator_234>, mut __p: Ptr<Ptr<i32>>, __args: Ptr<Ptr<i32>>) {
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
pub struct __is_move_insertable_240 {}
impl Clone for __is_move_insertable_240 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_move_insertable_240 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_move_insertable_241 {}
impl Clone for __is_move_insertable_241 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_move_insertable_241 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __alloc_traits_242 {}
impl Clone for __alloc_traits_242 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_242 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct rebind_243 {}
impl Clone for rebind_243 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_243 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __alloc_traits_244 {}
impl Clone for __alloc_traits_244 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_244 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct rebind_245 {}
impl Clone for rebind_245 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_245 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_bitwise_relocatable_246 {}
impl Clone for __is_bitwise_relocatable_246 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_bitwise_relocatable_246 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_bitwise_relocatable_247 {}
impl Clone for __is_bitwise_relocatable_247 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_bitwise_relocatable_247 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct initializer_list_248 {}
impl Clone for initializer_list_248 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for initializer_list_248 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(24)]
pub struct _Vector_base_249 {
    #[offset(0)]
    #[byte_size(24)]
    pub _M_impl: std__Vector_base_int__std_allocator_int____Vector_impl,
}
impl Clone for _Vector_base_249 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Vector_base_249 {
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
impl _Vector_base_249 {
    pub fn new_1() -> Self {
        unimplemented!()
    }
}
impl _Vector_base_249 {
    pub fn move_from(_a0: Ptr<_Vector_base_249>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(24)]
pub struct _Vector_base_250 {
    #[offset(0)]
    #[byte_size(24)]
    pub _M_impl: std__Vector_base_int_ptr__std_allocator_int_ptr____Vector_impl,
}
impl Clone for _Vector_base_250 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Vector_base_250 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(24)]
pub struct std__Vector_base_int_ptr__std_allocator_int_ptr____Vector_impl_data {
    #[offset(0)]
    #[byte_size(8)]
    pub _M_start: Ptr<Ptr<i32>>,
    #[offset(8)]
    #[byte_size(8)]
    pub _M_finish: Ptr<Ptr<i32>>,
    #[offset(16)]
    #[byte_size(8)]
    pub _M_end_of_storage: Ptr<Ptr<i32>>,
}
impl Clone for std__Vector_base_int_ptr__std_allocator_int_ptr____Vector_impl_data {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std__Vector_base_int_ptr__std_allocator_int_ptr____Vector_impl_data {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_int_ptr__std_allocator_int_ptr____Vector_impl_data {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_int_ptr__std_allocator_int_ptr____Vector_impl_data {
    pub fn move_from(
        __x: Ptr<std__Vector_base_int_ptr__std_allocator_int_ptr____Vector_impl_data>,
    ) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(24)]
pub struct std__Vector_base_int_ptr__std_allocator_int_ptr____Vector_impl {}
impl Clone for std__Vector_base_int_ptr__std_allocator_int_ptr____Vector_impl {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std__Vector_base_int_ptr__std_allocator_int_ptr____Vector_impl {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_int_ptr__std_allocator_int_ptr____Vector_impl {
    pub fn new_1() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_int_ptr__std_allocator_int_ptr____Vector_impl {
    pub fn move_from(
        __x: Ptr<std__Vector_base_int_ptr__std_allocator_int_ptr____Vector_impl>,
    ) -> Self {
        unimplemented!()
    }
}
impl _Vector_base_250 {
    pub fn new_1() -> Self {
        unimplemented!()
    }
}
impl _Vector_base_250 {
    pub fn move_from(_a0: Ptr<_Vector_base_250>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(24)]
pub struct vector_251 {}
impl Clone for vector_251 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for vector_251 {
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
    pub _M_vect: Ptr<_Vector_base_249>,
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
    pub fn new(mut __s: Ptr<i32>, mut __l: u64, __vect: Ptr<_Vector_base_249>) -> Self {
        unimplemented!()
    }
}
impl vector_251 {
    pub fn new_1() -> Self {
        unimplemented!()
    }
}
impl vector_251 {
    pub fn copy_from(__x: Ptr<vector_251>) -> Self {
        unimplemented!()
    }
}
impl vector_251 {
    pub fn move_from(_a0: Ptr<vector_251>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(24)]
pub struct vector_253 {}
impl Clone for vector_253 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for vector_253 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(24)]
pub struct std_vector_int_ptr___Guard_alloc {
    #[offset(0)]
    #[byte_size(8)]
    pub _M_storage: Ptr<Ptr<i32>>,
    #[offset(8)]
    pub _M_len: u64,
    #[offset(16)]
    #[byte_size(8)]
    pub _M_vect: Ptr<_Vector_base_250>,
}
impl Clone for std_vector_int_ptr___Guard_alloc {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_vector_int_ptr___Guard_alloc {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_vector_int_ptr___Guard_alloc {
    pub fn new(mut __s: Ptr<Ptr<i32>>, mut __l: u64, __vect: Ptr<_Vector_base_250>) -> Self {
        unimplemented!()
    }
}
impl vector_253 {
    pub fn new_1() -> Self {
        unimplemented!()
    }
}
impl vector_253 {
    pub fn copy_from(__x: Ptr<vector_253>) -> Self {
        unimplemented!()
    }
}
impl vector_253 {
    pub fn move_from(_a0: Ptr<vector_253>) -> Self {
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
pub struct __hash_base_262 {}
impl Clone for __hash_base_262 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_262 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_263 {}
impl Clone for __hash_base_263 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_263 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_264 {}
impl Clone for __hash_base_264 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_264 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_265 {}
impl Clone for __hash_base_265 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_265 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_266 {}
impl Clone for __hash_base_266 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_266 {
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
pub struct hash_276 {}
impl Clone for hash_276 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for hash_276 {
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
    pub fn hash_277(__val: Ptr<f32>) -> usize {
        unimplemented!()
    }
}
impl std__Hash_impl {
    pub fn hash_278(__val: Ptr<f64>) -> usize {
        unimplemented!()
    }
}
pub type anon_279 = u32;
pub const anon_279__S_word_bit: anon_279 = 64;
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
pub struct binary_function_280 {}
impl Clone for binary_function_280 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for binary_function_280 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct greater_281 {}
impl Clone for greater_281 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for greater_281 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct less_282 {}
impl Clone for less_282 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for less_282 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct greater_equal_283 {}
impl Clone for greater_equal_283 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for greater_equal_283 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct less_equal_284 {}
impl Clone for less_equal_284 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for less_equal_284 {
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
    pub static allocator_arg_285: Value<std_allocator_arg_t> = unimplemented!();
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
    let v1: Value<vector_251> = Rc::new(RefCell::new(<vector_251>::default()));
    ({
        let ___x: Value<i32> = Rc::new(RefCell::new(1));
        vector_251Impl::push_back_58(&v1.as_pointer(), ___x.as_pointer())
    });
    ({
        let ___x: Value<i32> = Rc::new(RefCell::new(2));
        vector_251Impl::push_back_58(&v1.as_pointer(), ___x.as_pointer())
    });
    ({
        let ___x: Value<i32> = Rc::new(RefCell::new(3));
        vector_251Impl::push_back_58(&v1.as_pointer(), ___x.as_pointer())
    });
    let mut sum: i32 = 0;
    {
        let __range1: Ptr<vector_251> = v1.as_pointer();
        let __begin1: Value<__normal_iterator_222> =
            Rc::new(RefCell::new(({ vector_251Impl::begin_26(&__range1) })));
        let __end1: Value<__normal_iterator_222> =
            Rc::new(RefCell::new(({ vector_251Impl::end_28(&__range1) })));
        'loop_: while ({
            let ___lhs: Ptr<__normal_iterator_222> = __begin1.as_pointer();
            operator_ne_286(___lhs, __end1.as_pointer())
        }) {
            let mut x: i32 =
                (({ __normal_iterator_222Impl::operator_deref(&__begin1.as_pointer()) }).read());
            sum += x.prefix_inc();
            ({ __normal_iterator_222Impl::operator_inc_6(&__begin1.as_pointer()) });
        }
    }
    {
        let __range1: Ptr<vector_251> = v1.as_pointer();
        let __begin1: Value<__normal_iterator_222> =
            Rc::new(RefCell::new(({ vector_251Impl::begin_26(&__range1) })));
        let __end1: Value<__normal_iterator_222> =
            Rc::new(RefCell::new(({ vector_251Impl::end_28(&__range1) })));
        'loop_: while ({
            let ___lhs: Ptr<__normal_iterator_222> = __begin1.as_pointer();
            operator_ne_286(___lhs, __end1.as_pointer())
        }) {
            let mut x: i32 =
                (({ __normal_iterator_222Impl::operator_deref(&__begin1.as_pointer()) }).read());
            sum += x;
            ({ __normal_iterator_222Impl::operator_inc_6(&__begin1.as_pointer()) });
        }
    }
    {
        let __range1: Ptr<vector_251> = v1.as_pointer();
        let __begin1: Value<__normal_iterator_222> =
            Rc::new(RefCell::new(({ vector_251Impl::begin_26(&__range1) })));
        let __end1: Value<__normal_iterator_222> =
            Rc::new(RefCell::new(({ vector_251Impl::end_28(&__range1) })));
        'loop_: while ({
            let ___lhs: Ptr<__normal_iterator_222> = __begin1.as_pointer();
            operator_ne_286(___lhs, __end1.as_pointer())
        }) {
            let x: Ptr<i32> =
                ({ __normal_iterator_222Impl::operator_deref(&__begin1.as_pointer()) });
            {
                x.with_mut(|__v| *__v = *__v + 10)
            };
            ({ __normal_iterator_222Impl::operator_inc_6(&__begin1.as_pointer()) });
        }
    }
    {
        let __range1: Ptr<vector_251> = v1.as_pointer();
        let __begin1: Value<__normal_iterator_222> =
            Rc::new(RefCell::new(({ vector_251Impl::begin_26(&__range1) })));
        let __end1: Value<__normal_iterator_222> =
            Rc::new(RefCell::new(({ vector_251Impl::end_28(&__range1) })));
        'loop_: while ({
            let ___lhs: Ptr<__normal_iterator_222> = __begin1.as_pointer();
            operator_ne_286(___lhs, __end1.as_pointer())
        }) {
            let x: Ptr<i32> =
                ({ __normal_iterator_222Impl::operator_deref(&__begin1.as_pointer()) });
            sum += { (x.read()) };
            ({ __normal_iterator_222Impl::operator_inc_6(&__begin1.as_pointer()) });
        }
    }
    let v2: Value<vector_253> = Rc::new(RefCell::new(<vector_253>::default()));
    ({
        let ___x: Value<Ptr<i32>> = Rc::new(RefCell::new(
            ({ vector_251Impl::operator_index_46(&v1.as_pointer(), 0_u64) }),
        ));
        vector_253Impl::push_back_58(&v2.as_pointer(), ___x.as_pointer())
    });
    ({
        let ___x: Value<Ptr<i32>> = Rc::new(RefCell::new(
            ({ vector_251Impl::operator_index_46(&v1.as_pointer(), 1_u64) }),
        ));
        vector_253Impl::push_back_58(&v2.as_pointer(), ___x.as_pointer())
    });
    ({
        let ___x: Value<Ptr<i32>> = Rc::new(RefCell::new(
            ({ vector_251Impl::operator_index_46(&v1.as_pointer(), 2_u64) }),
        ));
        vector_253Impl::push_back_58(&v2.as_pointer(), ___x.as_pointer())
    });
    {
        let __range1: Ptr<vector_253> = v2.as_pointer();
        let __begin1: Value<__normal_iterator_224> =
            Rc::new(RefCell::new(({ vector_253Impl::begin_26(&__range1) })));
        let __end1: Value<__normal_iterator_224> =
            Rc::new(RefCell::new(({ vector_253Impl::end_28(&__range1) })));
        'loop_: while ({
            let ___lhs: Ptr<__normal_iterator_224> = __begin1.as_pointer();
            operator_ne_287(___lhs, __end1.as_pointer())
        }) {
            let mut p: Ptr<i32> =
                (({ __normal_iterator_224Impl::operator_deref(&__begin1.as_pointer()) }).read());
            {
                p.with_mut(|__v| *__v = *__v + 5)
            };
            ({ __normal_iterator_224Impl::operator_inc_6(&__begin1.as_pointer()) });
        }
    }
    {
        let __range1: Ptr<vector_253> = v2.as_pointer();
        let __begin1: Value<__normal_iterator_224> =
            Rc::new(RefCell::new(({ vector_253Impl::begin_26(&__range1) })));
        let __end1: Value<__normal_iterator_224> =
            Rc::new(RefCell::new(({ vector_253Impl::end_28(&__range1) })));
        'loop_: while ({
            let ___lhs: Ptr<__normal_iterator_224> = __begin1.as_pointer();
            operator_ne_287(___lhs, __end1.as_pointer())
        }) {
            let mut p: Ptr<i32> =
                (({ __normal_iterator_224Impl::operator_deref(&__begin1.as_pointer()) }).read());
            sum += { (p.read()) };
            ({ __normal_iterator_224Impl::operator_inc_6(&__begin1.as_pointer()) });
        }
    }
    {
        let __range1: Ptr<vector_253> = v2.as_pointer();
        let __begin1: Value<__normal_iterator_224> =
            Rc::new(RefCell::new(({ vector_253Impl::begin_26(&__range1) })));
        let __end1: Value<__normal_iterator_224> =
            Rc::new(RefCell::new(({ vector_253Impl::end_28(&__range1) })));
        'loop_: while ({
            let ___lhs: Ptr<__normal_iterator_224> = __begin1.as_pointer();
            operator_ne_287(___lhs, __end1.as_pointer())
        }) {
            let mut p: Ptr<i32> =
                (({ __normal_iterator_224Impl::operator_deref(&__begin1.as_pointer()) }).read());
            {
                p.with_mut(|__v| *__v = *__v + 5)
            };
            ({ __normal_iterator_224Impl::operator_inc_6(&__begin1.as_pointer()) });
        }
    }
    {
        let __range1: Ptr<vector_253> = v2.as_pointer();
        let __begin1: Value<__normal_iterator_224> =
            Rc::new(RefCell::new(({ vector_253Impl::begin_26(&__range1) })));
        let __end1: Value<__normal_iterator_224> =
            Rc::new(RefCell::new(({ vector_253Impl::end_28(&__range1) })));
        'loop_: while ({
            let ___lhs: Ptr<__normal_iterator_224> = __begin1.as_pointer();
            operator_ne_287(___lhs, __end1.as_pointer())
        }) {
            let mut p: Ptr<i32> =
                (({ __normal_iterator_224Impl::operator_deref(&__begin1.as_pointer()) }).read());
            sum += { (p.read()) };
            ({ __normal_iterator_224Impl::operator_inc_6(&__begin1.as_pointer()) });
        }
    }
    (if (sum == 168) {
        &(0);
    } else {
        ({
            __assert_fail_288(
                Ptr::<i8>::from_string_literal(b"sum == 168"),
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
pub fn operator_ne_286(
    __lhs: Ptr<__normal_iterator_222>,
    __rhs: Ptr<__normal_iterator_222>,
) -> bool {
    unimplemented!()
}
pub fn operator_ne_287(
    __lhs: Ptr<__normal_iterator_224>,
    __rhs: Ptr<__normal_iterator_224>,
) -> bool {
    unimplemented!()
}
pub fn __assert_fail_288(
    mut __assertion: Ptr<i8>,
    mut __file: Ptr<i8>,
    mut __line: u32,
    mut __function: Ptr<i8>,
) {
    unimplemented!()
}
pub trait _Vector_base_249Impl {
    fn _M_get_Tp_allocator_1(&self) -> Ptr<allocator_233> {
        unimplemented!()
    }
    fn _M_get_Tp_allocator_2(&self) -> Ptr<allocator_233> {
        unimplemented!()
    }
    fn _M_allocate(&self, mut __n: usize) -> Ptr<i32> {
        unimplemented!()
    }
    fn _M_deallocate(&self, mut __p: Ptr<i32>, mut __n: usize) {
        unimplemented!()
    }
}
impl _Vector_base_249Impl for Ptr<_Vector_base_249> {}
pub trait _Vector_base_250Impl {
    fn _M_get_Tp_allocator_1(&self) -> Ptr<allocator_234> {
        unimplemented!()
    }
    fn _M_get_Tp_allocator_2(&self) -> Ptr<allocator_234> {
        unimplemented!()
    }
    fn _M_allocate(&self, mut __n: usize) -> Ptr<Ptr<i32>> {
        unimplemented!()
    }
    fn _M_deallocate(&self, mut __p: Ptr<Ptr<i32>>, mut __n: usize) {
        unimplemented!()
    }
}
impl _Vector_base_250Impl for Ptr<_Vector_base_250> {}
pub trait __new_allocator_228Impl {
    fn allocate(&self, mut __n: u64, _a1: Option<AnyPtr>) -> Ptr<i32> {
        unimplemented!()
    }
    fn deallocate(&self, mut __p: Ptr<i32>, mut __n: u64) {
        unimplemented!()
    }
    fn max_size(&self) -> u64 {
        unimplemented!()
    }
    fn construct_229(&self, mut __p: Ptr<i32>, __args: Ptr<i32>) {
        unimplemented!()
    }
}
impl __new_allocator_228Impl for Ptr<__new_allocator_228> {}
pub trait __new_allocator_230Impl {
    fn allocate(&self, mut __n: u64, _a1: Option<AnyPtr>) -> Ptr<Ptr<i32>> {
        unimplemented!()
    }
    fn deallocate(&self, mut __p: Ptr<Ptr<i32>>, mut __n: u64) {
        unimplemented!()
    }
    fn max_size(&self) -> u64 {
        unimplemented!()
    }
    fn construct_231(&self, mut __p: Ptr<Ptr<i32>>, __args: Ptr<Ptr<i32>>) {
        unimplemented!()
    }
}
impl __new_allocator_230Impl for Ptr<__new_allocator_230> {}
pub trait __normal_iterator_222Impl {
    fn operator_deref(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn operator_inc_6(&self) -> Ptr<__normal_iterator_222> {
        unimplemented!()
    }
    fn operator_sub(&self, mut __n: i64) -> __normal_iterator_222 {
        unimplemented!()
    }
    fn base(&self) -> Ptr<Ptr<i32>> {
        unimplemented!()
    }
}
impl __normal_iterator_222Impl for Ptr<__normal_iterator_222> {}
pub trait __normal_iterator_223Impl {
    fn base(&self) -> Ptr<Ptr<i32>> {
        unimplemented!()
    }
}
impl __normal_iterator_223Impl for Ptr<__normal_iterator_223> {}
pub trait __normal_iterator_224Impl {
    fn operator_deref(&self) -> Ptr<Ptr<i32>> {
        unimplemented!()
    }
    fn operator_inc_6(&self) -> Ptr<__normal_iterator_224> {
        unimplemented!()
    }
    fn operator_sub(&self, mut __n: i64) -> __normal_iterator_224 {
        unimplemented!()
    }
    fn base(&self) -> Ptr<Ptr<Ptr<i32>>> {
        unimplemented!()
    }
}
impl __normal_iterator_224Impl for Ptr<__normal_iterator_224> {}
pub trait __normal_iterator_225Impl {
    fn base(&self) -> Ptr<Ptr<Ptr<i32>>> {
        unimplemented!()
    }
}
impl __normal_iterator_225Impl for Ptr<__normal_iterator_225> {}
pub trait initializer_list_248Impl {
    fn size(&self) -> u64 {
        unimplemented!()
    }
    fn begin(&self) -> Ptr<bool> {
        unimplemented!()
    }
}
impl initializer_list_248Impl for Ptr<initializer_list_248> {}
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
pub trait vector_251Impl {
    fn copy_assign(&self, __x: Ptr<vector_251>) -> Ptr<vector_251> {
        unimplemented!()
    }
    fn move_assign(&self, __x: Ptr<vector_251>) -> Ptr<vector_251> {
        unimplemented!()
    }
    fn begin_26(&self) -> __normal_iterator_222 {
        unimplemented!()
    }
    fn begin_27(&self) -> __normal_iterator_223 {
        unimplemented!()
    }
    fn end_28(&self) -> __normal_iterator_222 {
        unimplemented!()
    }
    fn end_29(&self) -> __normal_iterator_223 {
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
    fn operator_index_46(&self, mut __n: u64) -> Ptr<i32> {
        unimplemented!()
    }
    fn back_53(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn push_back_58(&self, __x: Ptr<i32>) {
        unimplemented!()
    }
    fn emplace_back_252(&self, __args: Ptr<i32>) -> Ptr<i32> {
        unimplemented!()
    }
}
impl vector_251Impl for Ptr<vector_251> {}
pub trait vector_253Impl {
    fn copy_assign(&self, __x: Ptr<vector_253>) -> Ptr<vector_253> {
        unimplemented!()
    }
    fn move_assign(&self, __x: Ptr<vector_253>) -> Ptr<vector_253> {
        unimplemented!()
    }
    fn begin_26(&self) -> __normal_iterator_224 {
        unimplemented!()
    }
    fn begin_27(&self) -> __normal_iterator_225 {
        unimplemented!()
    }
    fn end_28(&self) -> __normal_iterator_224 {
        unimplemented!()
    }
    fn end_29(&self) -> __normal_iterator_225 {
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
    fn back_53(&self) -> Ptr<Ptr<i32>> {
        unimplemented!()
    }
    fn push_back_58(&self, __x: Ptr<Ptr<i32>>) {
        unimplemented!()
    }
    fn emplace_back_254(&self, __args: Ptr<Ptr<i32>>) -> Ptr<Ptr<i32>> {
        unimplemented!()
    }
}
impl vector_253Impl for Ptr<vector_253> {}
pub fn __cpp2rust_init_globals() {}
