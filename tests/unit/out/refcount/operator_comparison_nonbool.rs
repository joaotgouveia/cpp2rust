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
pub struct __type_identity_8 {}
impl Clone for __type_identity_8 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_8 {
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
pub struct __and__20 {}
impl Clone for __and__20 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__20 {
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
pub struct __not__27 {}
impl Clone for __not__27 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__27 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __not__28 {}
impl Clone for __not__28 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__28 {
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
pub struct is_reference_31 {}
impl Clone for is_reference_31 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_31 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_32 {}
impl Clone for is_reference_32 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_32 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_33 {}
impl Clone for is_reference_33 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_33 {
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
pub struct is_function_35 {}
impl Clone for is_function_35 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_35 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_36 {}
impl Clone for is_function_36 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_36 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_37 {}
impl Clone for is_function_37 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_37 {
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
pub struct is_function_40 {}
impl Clone for is_function_40 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_40 {
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
pub struct is_void_44 {}
impl Clone for is_void_44 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_44 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_45 {}
impl Clone for is_void_45 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_45 {
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
pub struct remove_cv_49 {}
impl Clone for remove_cv_49 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_49 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_50 {}
impl Clone for remove_cv_50 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_50 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_51 {}
impl Clone for remove_cv_51 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_51 {
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
pub struct is_const_72 {}
impl Clone for is_const_72 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_72 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_const_73 {}
impl Clone for is_const_73 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_73 {
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
pub struct __is_array_unknown_bounds_76 {}
impl Clone for __is_array_unknown_bounds_76 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_76 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_77 {}
impl Clone for __is_array_unknown_bounds_77 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_77 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_78 {}
impl Clone for __is_array_unknown_bounds_78 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_78 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_79 {}
impl Clone for __is_array_unknown_bounds_79 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_79 {
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
pub struct __is_integral_helper_82 {}
impl Clone for __is_integral_helper_82 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_82 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_83 {}
impl Clone for __is_integral_helper_83 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_83 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_84 {}
impl Clone for __is_integral_helper_84 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_84 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_85 {}
impl Clone for __is_integral_helper_85 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_85 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_86 {}
impl Clone for __is_integral_helper_86 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_86 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_87 {}
impl Clone for __is_integral_helper_87 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_87 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_88 {}
impl Clone for __is_integral_helper_88 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_88 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_89 {}
impl Clone for __is_integral_helper_89 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_89 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_90 {}
impl Clone for __is_integral_helper_90 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_90 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_91 {}
impl Clone for __is_integral_helper_91 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_91 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_92 {}
impl Clone for __is_integral_helper_92 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_92 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_93 {}
impl Clone for __is_integral_helper_93 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_93 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_94 {}
impl Clone for is_integral_94 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_94 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_95 {}
impl Clone for is_integral_95 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_95 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_96 {}
impl Clone for is_integral_96 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_96 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_97 {}
impl Clone for is_integral_97 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_97 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_98 {}
impl Clone for is_integral_98 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_98 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_99 {}
impl Clone for is_integral_99 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_99 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_100 {}
impl Clone for is_integral_100 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_100 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_101 {}
impl Clone for is_integral_101 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_101 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_102 {}
impl Clone for is_integral_102 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_102 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_103 {}
impl Clone for is_integral_103 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_103 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_104 {}
impl Clone for is_integral_104 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_104 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_105 {}
impl Clone for is_integral_105 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_105 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_106 {}
impl Clone for is_integral_106 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_106 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_107 {}
impl Clone for is_integral_107 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_107 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_floating_point_helper_108 {}
impl Clone for __is_floating_point_helper_108 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_floating_point_helper_108 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_floating_point_109 {}
impl Clone for is_floating_point_109 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_floating_point_109 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_volatile_110 {}
impl Clone for is_volatile_110 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_110 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_volatile_111 {}
impl Clone for is_volatile_111 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_111 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_volatile_112 {}
impl Clone for is_volatile_112 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_112 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_volatile_113 {}
impl Clone for is_volatile_113 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_113 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_volatile_114 {}
impl Clone for is_volatile_114 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_114 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_volatile_115 {}
impl Clone for is_volatile_115 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_115 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_volatile_116 {}
impl Clone for is_volatile_116 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_116 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_volatile_117 {}
impl Clone for is_volatile_117 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_117 {
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
pub struct is_move_constructible_118 {}
impl Clone for is_move_constructible_118 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_move_constructible_118 {
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
pub struct is_nothrow_copy_constructible_121 {}
impl Clone for is_nothrow_copy_constructible_121 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_copy_constructible_121 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_copy_constructible_122 {}
impl Clone for is_nothrow_copy_constructible_122 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_copy_constructible_122 {
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
pub struct is_nothrow_move_constructible_124 {}
impl Clone for is_nothrow_move_constructible_124 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_move_constructible_124 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_move_constructible_125 {}
impl Clone for is_nothrow_move_constructible_125 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_move_constructible_125 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_move_constructible_126 {}
impl Clone for is_nothrow_move_constructible_126 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_move_constructible_126 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_move_assignable_127 {}
impl Clone for is_move_assignable_127 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_move_assignable_127 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_move_assignable_128 {}
impl Clone for is_move_assignable_128 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_move_assignable_128 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_move_assignable_129 {}
impl Clone for is_move_assignable_129 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_move_assignable_129 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_move_assignable_130 {}
impl Clone for is_nothrow_move_assignable_130 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_move_assignable_130 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_move_assignable_131 {}
impl Clone for is_nothrow_move_assignable_131 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_move_assignable_131 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_move_assignable_132 {}
impl Clone for is_nothrow_move_assignable_132 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_move_assignable_132 {
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
pub struct remove_reference_134 {}
impl Clone for remove_reference_134 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_134 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_135 {}
impl Clone for remove_reference_135 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_135 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_136 {}
impl Clone for remove_reference_136 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_136 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_137 {}
impl Clone for remove_reference_137 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_137 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_138 {}
impl Clone for remove_reference_138 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_138 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_139 {}
impl Clone for remove_reference_139 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_139 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __cv_selector_140 {}
impl Clone for __cv_selector_140 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_140 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __cv_selector_141 {}
impl Clone for __cv_selector_141 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_141 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __cv_selector_142 {}
impl Clone for __cv_selector_142 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_142 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __cv_selector_143 {}
impl Clone for __cv_selector_143 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_143 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __cv_selector_144 {}
impl Clone for __cv_selector_144 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_144 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __cv_selector_145 {}
impl Clone for __cv_selector_145 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_145 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __cv_selector_146 {}
impl Clone for __cv_selector_146 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_146 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_147 {}
impl Clone for __match_cv_qualifiers_147 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_147 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_148 {}
impl Clone for __match_cv_qualifiers_148 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_148 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_149 {}
impl Clone for __match_cv_qualifiers_149 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_149 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_150 {}
impl Clone for __match_cv_qualifiers_150 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_150 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_151 {}
impl Clone for __match_cv_qualifiers_151 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_151 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_152 {}
impl Clone for __match_cv_qualifiers_152 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_152 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_153 {}
impl Clone for __match_cv_qualifiers_153 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_153 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_154 {}
impl Clone for __match_cv_qualifiers_154 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_154 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_155 {}
impl Clone for __make_unsigned_155 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_155 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_156 {}
impl Clone for __make_unsigned_156 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_156 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_157 {}
impl Clone for __make_unsigned_157 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_157 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_158 {}
impl Clone for __make_unsigned_158 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_158 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_159 {}
impl Clone for __make_unsigned_159 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_159 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_160 {}
impl Clone for __make_unsigned_selector_160 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_160 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_161 {}
impl Clone for __make_unsigned_selector_161 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_161 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_162 {}
impl Clone for __make_unsigned_selector_162 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_162 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_163 {}
impl Clone for __make_unsigned_selector_163 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_163 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_164 {}
impl Clone for __make_unsigned_selector_164 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_164 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_165 {}
impl Clone for __make_unsigned_selector_165 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_165 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_166 {}
impl Clone for __make_unsigned_selector_166 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_166 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_167 {}
impl Clone for __make_unsigned_selector_167 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_167 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_168 {}
impl Clone for __make_unsigned_selector_168 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_168 {
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
pub struct _List_172 {}
impl Clone for _List_172 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_172 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static std___make_unsigned_selector_base__List_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long____size_173 : Value<usize > = unimplemented!() ;
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_174 {}
impl Clone for _List_174 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_174 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static std___make_unsigned_selector_base__List_unsigned_short__unsigned_int__unsigned_long__unsigned_long_long____size_175 : Value<usize > = unimplemented!() ;
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_176 {}
impl Clone for _List_176 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_176 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static std___make_unsigned_selector_base__List_unsigned_int__unsigned_long__unsigned_long_long____size_177 : Value<usize > = unimplemented!() ;
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_178 {}
impl Clone for _List_178 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_178 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_179 {}
impl Clone for _List_179 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_179 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_180 {}
impl Clone for _List_180 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_180 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_181 {}
impl Clone for __select_181 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_181 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_182 {}
impl Clone for __select_182 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_182 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_183 {}
impl Clone for __select_183 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_183 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_184 {}
impl Clone for __select_184 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_184 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_185 {}
impl Clone for __select_185 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_185 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_186 {}
impl Clone for __select_186 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_186 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct make_unsigned_187 {}
impl Clone for make_unsigned_187 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for make_unsigned_187 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_188 {}
impl Clone for __make_signed_188 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_188 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_189 {}
impl Clone for __make_signed_189 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_189 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_190 {}
impl Clone for __make_signed_190 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_190 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_191 {}
impl Clone for __make_signed_selector_191 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_191 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_192 {}
impl Clone for __make_signed_selector_192 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_192 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_193 {}
impl Clone for __make_signed_selector_193 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_193 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_194 {}
impl Clone for __make_signed_selector_194 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_194 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_195 {}
impl Clone for __make_signed_selector_195 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_195 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_196 {}
impl Clone for __make_signed_selector_196 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_196 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_197 {}
impl Clone for __make_signed_selector_197 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_197 {
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
pub struct common_type_198 {}
impl Clone for common_type_198 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for common_type_198 {
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
pub struct __is_tuple_like_impl_199 {}
impl Clone for __is_tuple_like_impl_199 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_tuple_like_impl_199 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_tuple_like_impl_200 {}
impl Clone for __is_tuple_like_impl_200 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_tuple_like_impl_200 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_tuple_like_impl_201 {}
impl Clone for __is_tuple_like_impl_201 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_tuple_like_impl_201 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_tuple_like_202 {}
impl Clone for __is_tuple_like_202 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_tuple_like_202 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_tuple_like_203 {}
impl Clone for __is_tuple_like_203 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_tuple_like_203 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_tuple_like_204 {}
impl Clone for __is_tuple_like_204 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_tuple_like_204 {
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
pub struct common_reference_205 {}
impl Clone for common_reference_205 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for common_reference_205 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static is_floating_point_v_206: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_integral_v_207: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_integral_v_208: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_same_v_209: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_same_v_210: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_integral_v_211: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_same_v_212: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_same_v_213: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_integral_v_214: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_integral_v_215: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_same_v_216: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_same_v_217: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_object_v_218: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_object_v_219: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_object_v_220: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_object_v_221: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_same_v_222: Value<bool> = unimplemented!();
);
thread_local!(
    pub static is_integral_v_223: Value<bool> = unimplemented!();
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
pub struct __are_same_224 {}
impl Clone for __are_same_224 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_224 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_225 = u32;
pub const anon_225___value: anon_225 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_226 {}
impl Clone for __are_same_226 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_226 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_227 = u32;
pub const anon_227___value: anon_227 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_228 {}
impl Clone for __are_same_228 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_228 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_229 = u32;
pub const anon_229___value: anon_229 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_230 {}
impl Clone for __are_same_230 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_230 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_231 = u32;
pub const anon_231___value: anon_231 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_232 {}
impl Clone for __are_same_232 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_232 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_233 = u32;
pub const anon_233___value: anon_233 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_234 {}
impl Clone for __are_same_234 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_234 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_235 = u32;
pub const anon_235___value: anon_235 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_236 {}
impl Clone for __is_integer_236 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_236 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_237 = u32;
pub const anon_237___value: anon_237 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_238 {}
impl Clone for __is_integer_238 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_238 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_239 = u32;
pub const anon_239___value: anon_239 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_240 {}
impl Clone for __is_integer_240 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_240 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_241 = u32;
pub const anon_241___value: anon_241 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_242 {}
impl Clone for __is_integer_242 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_242 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_243 = u32;
pub const anon_243___value: anon_243 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_244 {}
impl Clone for __is_integer_244 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_244 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_245 = u32;
pub const anon_245___value: anon_245 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_byte_246 {}
impl Clone for __is_byte_246 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_byte_246 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_247 = u32;
pub const anon_247___value: anon_247 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_nonvolatile_trivially_copyable_248 {}
impl Clone for __is_nonvolatile_trivially_copyable_248 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_nonvolatile_trivially_copyable_248 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_249 = u32;
pub const anon_249___value: anon_249 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_nonvolatile_trivially_copyable_250 {}
impl Clone for __is_nonvolatile_trivially_copyable_250 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_nonvolatile_trivially_copyable_250 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_251 = u32;
pub const anon_251___value: anon_251 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_nonvolatile_trivially_copyable_252 {}
impl Clone for __is_nonvolatile_trivially_copyable_252 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_nonvolatile_trivially_copyable_252 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_253 = u32;
pub const anon_253___value: anon_253 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __memcpyable_254 {}
impl Clone for __memcpyable_254 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __memcpyable_254 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __memcpyable_255 {}
impl Clone for __memcpyable_255 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __memcpyable_255 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __memcpyable_256 {}
impl Clone for __memcpyable_256 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __memcpyable_256 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __memcpyable_integer_257 {}
impl Clone for __memcpyable_integer_257 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __memcpyable_integer_257 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __memcpyable_integer_258 {}
impl Clone for __memcpyable_integer_258 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __memcpyable_integer_258 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __memcpyable_integer_259 {}
impl Clone for __memcpyable_integer_259 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __memcpyable_integer_259 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static __can_use_memchr_for_find_260: Value<bool> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_nonstrict_261 {}
impl Clone for __is_integer_nonstrict_261 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_261 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_262 = u32;
pub const anon_262___value: anon_262 = 1;
pub type anon_263 = u32;
pub const anon_263___width: anon_263 = 128;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_nonstrict_264 {}
impl Clone for __is_integer_nonstrict_264 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_264 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_265 = u32;
pub const anon_265___width: anon_265 = 64;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_nonstrict_266 {}
impl Clone for __is_integer_nonstrict_266 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_266 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_267 = u32;
pub const anon_267___width: anon_267 = 64;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_integer_268 {}
impl Clone for __numeric_traits_integer_268 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_268 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_long____is_signed_269: Value<bool> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_long____digits_270: Value<i32> = unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_long____max_271: Value<i64> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_integer_272 {}
impl Clone for __numeric_traits_integer_272 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_272 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned___int128____is_signed_273: Value<bool> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned___int128____digits_274: Value<i32> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned___int128____max_275: Value<u128> =
        unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_integer_276 {}
impl Clone for __numeric_traits_integer_276 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_276 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_long____is_signed_277: Value<bool> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_long____digits_278: Value<i32> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_long____max_279: Value<u64> =
        unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_floating_280 {}
impl Clone for __numeric_traits_floating_280 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_280 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_floating_281 {}
impl Clone for __numeric_traits_floating_281 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_281 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_floating_282 {}
impl Clone for __numeric_traits_floating_282 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_282 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_283 {}
impl Clone for __numeric_traits_283 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_283 {
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
pub struct std_ranges___swap__Swap {}
impl Clone for std_ranges___swap__Swap {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___swap__Swap {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static swap_284: Value<std_ranges___swap__Swap> = unimplemented!();
);
pub type std___cmp_cat__Ord = i8;
pub const std___cmp_cat__Ord_equivalent: std___cmp_cat__Ord = 0;
pub const std___cmp_cat__Ord_less: std___cmp_cat__Ord = -1;
pub const std___cmp_cat__Ord_greater: std___cmp_cat__Ord = 1;
pub type std___cmp_cat__Ncmp = i8;
pub const std___cmp_cat__Ncmp__Unordered: std___cmp_cat__Ncmp = 2;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std___cmp_cat___unspec {}
impl Clone for std___cmp_cat___unspec {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___cmp_cat___unspec {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std___cmp_cat___unspec {
    pub fn new(mut _a0: Ptr<std___cmp_cat___unspec>) -> Self {
        unimplemented!()
    }
}
impl std___cmp_cat___unspec {}
impl std___cmp_cat___unspec {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_partial_ordering {}
impl Clone for std_partial_ordering {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_partial_ordering {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static std_partial_ordering_less_285: Value<std_partial_ordering> = unimplemented!();
);
thread_local!(
    pub static std_partial_ordering_equivalent_286: Value<std_partial_ordering> = unimplemented!();
);
thread_local!(
    pub static std_partial_ordering_greater_287: Value<std_partial_ordering> = unimplemented!();
);
thread_local!(
    pub static std_partial_ordering_unordered_288: Value<std_partial_ordering> = unimplemented!();
);
impl std_partial_ordering {}
impl std_partial_ordering {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_weak_ordering {}
impl Clone for std_weak_ordering {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_weak_ordering {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static std_weak_ordering_less_289: Value<std_weak_ordering> = unimplemented!();
);
thread_local!(
    pub static std_weak_ordering_equivalent_290: Value<std_weak_ordering> = unimplemented!();
);
thread_local!(
    pub static std_weak_ordering_greater_291: Value<std_weak_ordering> = unimplemented!();
);
impl std_weak_ordering {}
impl std_weak_ordering {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_strong_ordering {}
impl Clone for std_strong_ordering {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_strong_ordering {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static std_strong_ordering_less_292: Value<std_strong_ordering> = unimplemented!();
);
thread_local!(
    pub static std_strong_ordering_equal_293: Value<std_strong_ordering> = unimplemented!();
);
thread_local!(
    pub static std_strong_ordering_equivalent_294: Value<std_strong_ordering> = unimplemented!();
);
thread_local!(
    pub static std_strong_ordering_greater_295: Value<std_strong_ordering> = unimplemented!();
);
impl std_strong_ordering {}
impl std_strong_ordering {}
thread_local!(
    pub static __cmp_cat_id_296: Value<u32> = unimplemented!();
);
thread_local!(
    pub static __cmp_cat_id_297: Value<u32> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_compare_three_way {}
impl Clone for std_compare_three_way {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_compare_three_way {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_compare_three_way {}
impl std_compare_three_way {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std___compare__Strong_order {}
impl Clone for std___compare__Strong_order {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___compare__Strong_order {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type std___compare__Strong_order__Fp_fmt = i32;
pub const std___compare__Strong_order__Fp_fmt__Binary16: std___compare__Strong_order__Fp_fmt = 0;
pub const std___compare__Strong_order__Fp_fmt__Binary32: std___compare__Strong_order__Fp_fmt = 1;
pub const std___compare__Strong_order__Fp_fmt__Binary64: std___compare__Strong_order__Fp_fmt = 2;
pub const std___compare__Strong_order__Fp_fmt__Binary128: std___compare__Strong_order__Fp_fmt = 3;
pub const std___compare__Strong_order__Fp_fmt__X86_80bit: std___compare__Strong_order__Fp_fmt = 4;
pub const std___compare__Strong_order__Fp_fmt__M68k_80bit: std___compare__Strong_order__Fp_fmt = 5;
pub const std___compare__Strong_order__Fp_fmt__Dbldbl: std___compare__Strong_order__Fp_fmt = 6;
pub const std___compare__Strong_order__Fp_fmt__Bfloat16: std___compare__Strong_order__Fp_fmt = 7;
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct _Int_298 {
    #[offset(0)]
    pub _M_lo: u64,
    #[offset(8)]
    pub _M_hi: i32,
}
impl Clone for _Int_298 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Int_298 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct _Int_299 {
    #[offset(0)]
    pub _M_lo: u64,
    #[offset(8)]
    pub _M_hi: i16,
}
impl Clone for _Int_299 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Int_299 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Int_299 {}
impl _Int_299 {}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct _Int_300 {
    #[offset(0)]
    pub _M_lo: u64,
    #[offset(8)]
    pub _M_hi: i64,
}
impl Clone for _Int_300 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Int_300 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std___compare__Weak_order {}
impl Clone for std___compare__Weak_order {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___compare__Weak_order {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std___compare__Partial_order {}
impl Clone for std___compare__Partial_order {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___compare__Partial_order {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std___compare__Strong_fallback {}
impl Clone for std___compare__Strong_fallback {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___compare__Strong_fallback {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std___compare__Weak_fallback {}
impl Clone for std___compare__Weak_fallback {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___compare__Weak_fallback {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std___compare__Partial_fallback {}
impl Clone for std___compare__Partial_fallback {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___compare__Partial_fallback {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std___detail__Synth3way {}
impl Clone for std___detail__Synth3way {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___detail__Synth3way {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static __synth3way_301: Value<std___detail__Synth3way> = unimplemented!();
);
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
pub struct std_identity {}
impl Clone for std_identity {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_identity {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges_equal_to {}
impl Clone for std_ranges_equal_to {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges_equal_to {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges_less {}
impl Clone for std_ranges_less {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges_less {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges_greater {}
impl Clone for std_ranges_greater {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges_greater {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_default_sentinel_t {}
impl Clone for std_default_sentinel_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_default_sentinel_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_302 {}
impl Clone for iterator_traits_302 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_302 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_303 {}
impl Clone for iterator_traits_303 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_303 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_304 {}
impl Clone for iterator_traits_304 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_304 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_305 {}
impl Clone for iterator_traits_305 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_305 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges___imove__IterMove {}
impl Clone for std_ranges___imove__IterMove {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___imove__IterMove {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static iter_move_307: Value<std_ranges___imove__IterMove> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges___iswap__IterSwap {}
impl Clone for std_ranges___iswap__IterSwap {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___iswap__IterSwap {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static iter_swap_308: Value<std_ranges___iswap__IterSwap> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_unreachable_sentinel_t {}
impl Clone for std_unreachable_sentinel_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_unreachable_sentinel_t {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static __decay_copy_309: Value<std_ranges___access__Decay_copy> = unimplemented!();
);
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
pub struct std_contiguous_iterator_tag {}
impl Clone for std_contiguous_iterator_tag {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_contiguous_iterator_tag {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_310 {}
impl Clone for iterator_310 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_310 {
    fn default() -> Self {
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_destroying_delete_t {}
impl Clone for std_destroying_delete_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_destroying_delete_t {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_destroying_delete_t {}
impl std_destroying_delete_t {}
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
impl __gnu_cxx___ops__Iter_less_val {
    pub fn new_1() -> Self {
        unimplemented!()
    }
}
impl __gnu_cxx___ops__Iter_less_val {
    pub fn new_2(_a0: __gnu_cxx___ops__Iter_less_iter) -> Self {
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
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(8)]
pub struct _Iter_equals_val_320 {
    #[offset(0)]
    #[byte_size(8)]
    pub _M_value: Ptr<RefInt>,
}
impl Clone for _Iter_equals_val_320 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Iter_equals_val_320 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Iter_equals_val_320 {
    pub fn new(__value: Ptr<RefInt>) -> Self {
        unimplemented!()
    }
}
impl _Iter_equals_val_320 {}
impl _Iter_equals_val_320 {}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(8)]
pub struct div_t {
    #[offset(0)]
    pub quot: i32,
    #[offset(4)]
    pub rem: i32,
}
impl Clone for div_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for div_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct ldiv_t {
    #[offset(0)]
    pub quot: i64,
    #[offset(8)]
    pub rem: i64,
}
impl Clone for ldiv_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for ldiv_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct lldiv_t {
    #[offset(0)]
    pub quot: i64,
    #[offset(8)]
    pub rem: i64,
}
impl Clone for lldiv_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for lldiv_t {
    fn default() -> Self {
        unimplemented!()
    }
}
impl lldiv_t {}
impl lldiv_t {}
impl lldiv_t {}
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
#[byte_size(128)]
pub struct fd_set {
    #[offset(0)]
    #[byte_size(128)]
    pub fds_bits: Value<Box<[i64]>>,
}
impl Clone for fd_set {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for fd_set {
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
    pub __value32: anon_322,
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
pub struct anon_322 {
    #[offset(0)]
    pub __low: u32,
    #[offset(4)]
    pub __high: u32,
}
impl Clone for anon_322 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for anon_322 {
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
#[byte_size(48)]
pub struct random_data {
    #[offset(0)]
    #[byte_size(8)]
    pub fptr: Ptr<i32>,
    #[offset(8)]
    #[byte_size(8)]
    pub rptr: Ptr<i32>,
    #[offset(16)]
    #[byte_size(8)]
    pub state: Ptr<i32>,
    #[offset(24)]
    pub rand_type: i32,
    #[offset(28)]
    pub rand_deg: i32,
    #[offset(32)]
    pub rand_sep: i32,
    #[offset(40)]
    #[byte_size(8)]
    pub end_ptr: Ptr<i32>,
}
impl Clone for random_data {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for random_data {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(24)]
pub struct drand48_data {
    #[offset(0)]
    #[byte_size(6)]
    pub __x: Value<Box<[u16]>>,
    #[offset(6)]
    #[byte_size(6)]
    pub __old_x: Value<Box<[u16]>>,
    #[offset(12)]
    pub __c: u16,
    #[offset(14)]
    pub __init: u16,
    #[offset(16)]
    pub __a: u64,
}
impl Clone for drand48_data {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for drand48_data {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_323 = u32;
pub const anon_323__S_threshold: anon_323 = 16;
pub type anon_324 = u32;
pub const anon_324__S_chunk_size: anon_324 = 7;
thread_local!(
    pub static e_v_325: Value<f64> = unimplemented!();
);
thread_local!(
    pub static log2e_v_326: Value<f64> = unimplemented!();
);
thread_local!(
    pub static log10e_v_327: Value<f64> = unimplemented!();
);
thread_local!(
    pub static pi_v_328: Value<f64> = unimplemented!();
);
thread_local!(
    pub static inv_pi_v_329: Value<f64> = unimplemented!();
);
thread_local!(
    pub static inv_sqrtpi_v_330: Value<f64> = unimplemented!();
);
thread_local!(
    pub static ln2_331: Value<f64> = unimplemented!();
);
thread_local!(
    pub static ln2_v_332: Value<f64> = unimplemented!();
);
thread_local!(
    pub static ln10_333: Value<f64> = unimplemented!();
);
thread_local!(
    pub static ln10_v_334: Value<f64> = unimplemented!();
);
thread_local!(
    pub static sqrt2_v_335: Value<f64> = unimplemented!();
);
thread_local!(
    pub static sqrt3_v_336: Value<f64> = unimplemented!();
);
thread_local!(
    pub static inv_sqrt3_v_337: Value<f64> = unimplemented!();
);
thread_local!(
    pub static egamma_v_338: Value<f64> = unimplemented!();
);
thread_local!(
    pub static phi_v_339: Value<f64> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct numeric_limits_340 {}
impl Clone for numeric_limits_340 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for numeric_limits_340 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static std_numeric_limits_std_ranges___detail___max_size_type__digits_341: Value<i32> =
        unimplemented!();
);
impl numeric_limits_340 {
    pub fn min() -> std_ranges___detail___max_size_type {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct numeric_limits_342 {}
impl Clone for numeric_limits_342 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for numeric_limits_342 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static std_numeric_limits_std_ranges___detail___max_diff_type__digits_343: Value<i32> =
        unimplemented!();
);
impl numeric_limits_342 {
    pub fn min() -> std_ranges___detail___max_diff_type {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(32)]
pub struct std_ranges___detail___max_size_type {}
impl Clone for std_ranges___detail___max_size_type {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___detail___max_size_type {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static std_ranges___detail___max_size_type__S_rep_bits_344: Value<usize> = unimplemented!();
);
impl std_ranges___detail___max_size_type {
    pub fn new_1(__d: Ptr<std_ranges___detail___max_diff_type>) -> Self {
        unimplemented!()
    }
}
impl std_ranges___detail___max_size_type {}
impl std_ranges___detail___max_size_type {}
impl std_ranges___detail___max_size_type {
    pub fn copy_assign(
        &self,
        _a0: Ptr<std_ranges___detail___max_size_type>,
    ) -> Ptr<std_ranges___detail___max_size_type> {
        unimplemented!()
    }
}
impl std_ranges___detail___max_size_type {
    pub fn move_assign(
        &self,
        _a0: Ptr<std_ranges___detail___max_size_type>,
    ) -> Ptr<std_ranges___detail___max_size_type> {
        unimplemented!()
    }
}
impl std_ranges___detail___max_size_type {
    pub fn new_2(mut __i: i32) -> Self {
        unimplemented!()
    }
}
impl std_ranges___detail___max_size_type {
    pub fn new_3(mut __i: u128) -> Self {
        unimplemented!()
    }
}
impl std_ranges___detail___max_size_type {
    pub fn new_4(mut __i: u64) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(32)]
pub struct std_ranges___detail___max_diff_type {}
impl Clone for std_ranges___detail___max_diff_type {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___detail___max_diff_type {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_ranges___detail___max_diff_type {
    pub fn new_1(__d: Ptr<std_ranges___detail___max_size_type>) -> Self {
        unimplemented!()
    }
}
impl std_ranges___detail___max_diff_type {}
impl std_ranges___detail___max_diff_type {}
impl std_ranges___detail___max_diff_type {
    pub fn new_2(mut __i: i32) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges___access__Begin {}
impl Clone for std_ranges___access__Begin {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___access__Begin {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges___access__End {}
impl Clone for std_ranges___access__End {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___access__End {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges___access__RBegin {}
impl Clone for std_ranges___access__RBegin {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___access__RBegin {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges___access__REnd {}
impl Clone for std_ranges___access__REnd {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___access__REnd {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges___access__Size {}
impl Clone for std_ranges___access__Size {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___access__Size {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges___access__SSize {}
impl Clone for std_ranges___access__SSize {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___access__SSize {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges___access__Empty {}
impl Clone for std_ranges___access__Empty {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___access__Empty {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges___access__Data {}
impl Clone for std_ranges___access__Data {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___access__Data {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static begin_345: Value<std_ranges___access__Begin> = unimplemented!();
);
thread_local!(
    pub static end_346: Value<std_ranges___access__End> = unimplemented!();
);
thread_local!(
    pub static size_347: Value<std_ranges___access__Size> = unimplemented!();
);
thread_local!(
    pub static empty_348: Value<std_ranges___access__Empty> = unimplemented!();
);
thread_local!(
    pub static data_349: Value<std_ranges___access__Data> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges_view_base {}
impl Clone for std_ranges_view_base {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges_view_base {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges___access__CBegin {}
impl Clone for std_ranges___access__CBegin {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___access__CBegin {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges___access__CEnd {}
impl Clone for std_ranges___access__CEnd {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___access__CEnd {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges___access__CRBegin {}
impl Clone for std_ranges___access__CRBegin {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___access__CRBegin {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges___access__CREnd {}
impl Clone for std_ranges___access__CREnd {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___access__CREnd {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges___access__CData {}
impl Clone for std_ranges___access__CData {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___access__CData {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges___advance_fn {}
impl Clone for std_ranges___advance_fn {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___advance_fn {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static advance_350: Value<std_ranges___advance_fn> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges___distance_fn {}
impl Clone for std_ranges___distance_fn {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___distance_fn {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static distance_351: Value<std_ranges___distance_fn> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges___next_fn {}
impl Clone for std_ranges___next_fn {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___next_fn {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static next_352: Value<std_ranges___next_fn> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges___prev_fn {}
impl Clone for std_ranges___prev_fn {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___prev_fn {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static prev_353: Value<std_ranges___prev_fn> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges_dangling {}
impl Clone for std_ranges_dangling {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges_dangling {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges___equal_fn {}
impl Clone for std_ranges___equal_fn {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___equal_fn {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges___copy_fn {}
impl Clone for std_ranges___copy_fn {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___copy_fn {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static copy_354: Value<std_ranges___copy_fn> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges___move_fn {}
impl Clone for std_ranges___move_fn {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___move_fn {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static move_355: Value<std_ranges___move_fn> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges___copy_backward_fn {}
impl Clone for std_ranges___copy_backward_fn {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___copy_backward_fn {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges___move_backward_fn {}
impl Clone for std_ranges___move_backward_fn {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___move_backward_fn {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static move_backward_356: Value<std_ranges___move_backward_fn> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges___copy_n_fn {}
impl Clone for std_ranges___copy_n_fn {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___copy_n_fn {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges___fill_n_fn {}
impl Clone for std_ranges___fill_n_fn {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___fill_n_fn {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static fill_n_357: Value<std_ranges___fill_n_fn> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges___fill_fn {}
impl Clone for std_ranges___fill_fn {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___fill_fn {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type std_ranges_subrange_kind = bool;
pub const std_ranges_subrange_kind_unsized: std_ranges_subrange_kind = false;
pub const std_ranges_subrange_kind_sized: std_ranges_subrange_kind = true;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges___find_fn {}
impl Clone for std_ranges___find_fn {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___find_fn {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges___find_if_fn {}
impl Clone for std_ranges___find_if_fn {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___find_if_fn {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static find_if_358: Value<std_ranges___find_if_fn> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges___find_if_not_fn {}
impl Clone for std_ranges___find_if_not_fn {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___find_if_not_fn {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static find_if_not_359: Value<std_ranges___find_if_not_fn> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges___mismatch_fn {}
impl Clone for std_ranges___mismatch_fn {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___mismatch_fn {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges___search_fn {}
impl Clone for std_ranges___search_fn {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___search_fn {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static search_360: Value<std_ranges___search_fn> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges___min_fn {}
impl Clone for std_ranges___min_fn {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___min_fn {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges___adjacent_find_fn {}
impl Clone for std_ranges___adjacent_find_fn {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___adjacent_find_fn {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static adjacent_find_361: Value<std_ranges___adjacent_find_fn> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges___all_of_fn {}
impl Clone for std_ranges___all_of_fn {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___all_of_fn {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges___any_of_fn {}
impl Clone for std_ranges___any_of_fn {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___any_of_fn {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges___none_of_fn {}
impl Clone for std_ranges___none_of_fn {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___none_of_fn {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static none_of_362: Value<std_ranges___none_of_fn> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges___for_each_fn {}
impl Clone for std_ranges___for_each_fn {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___for_each_fn {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static for_each_363: Value<std_ranges___for_each_fn> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges___for_each_n_fn {}
impl Clone for std_ranges___for_each_n_fn {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___for_each_n_fn {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges___find_first_of_fn {}
impl Clone for std_ranges___find_first_of_fn {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___find_first_of_fn {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges___count_fn {}
impl Clone for std_ranges___count_fn {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___count_fn {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges___count_if_fn {}
impl Clone for std_ranges___count_if_fn {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___count_if_fn {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static count_if_364: Value<std_ranges___count_if_fn> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges___search_n_fn {}
impl Clone for std_ranges___search_n_fn {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___search_n_fn {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges___find_end_fn {}
impl Clone for std_ranges___find_end_fn {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___find_end_fn {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges___is_permutation_fn {}
impl Clone for std_ranges___is_permutation_fn {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___is_permutation_fn {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges___copy_if_fn {}
impl Clone for std_ranges___copy_if_fn {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___copy_if_fn {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges___swap_ranges_fn {}
impl Clone for std_ranges___swap_ranges_fn {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___swap_ranges_fn {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static swap_ranges_365: Value<std_ranges___swap_ranges_fn> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges___transform_fn {}
impl Clone for std_ranges___transform_fn {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___transform_fn {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges___replace_fn {}
impl Clone for std_ranges___replace_fn {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___replace_fn {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges___replace_if_fn {}
impl Clone for std_ranges___replace_if_fn {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___replace_if_fn {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges___replace_copy_fn {}
impl Clone for std_ranges___replace_copy_fn {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___replace_copy_fn {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges___replace_copy_if_fn {}
impl Clone for std_ranges___replace_copy_if_fn {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___replace_copy_if_fn {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges___generate_n_fn {}
impl Clone for std_ranges___generate_n_fn {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___generate_n_fn {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges___generate_fn {}
impl Clone for std_ranges___generate_fn {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___generate_fn {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges___remove_if_fn {}
impl Clone for std_ranges___remove_if_fn {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___remove_if_fn {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static remove_if_366: Value<std_ranges___remove_if_fn> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges___remove_fn {}
impl Clone for std_ranges___remove_fn {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___remove_fn {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges___remove_copy_if_fn {}
impl Clone for std_ranges___remove_copy_if_fn {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___remove_copy_if_fn {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges___remove_copy_fn {}
impl Clone for std_ranges___remove_copy_fn {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___remove_copy_fn {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges___unique_fn {}
impl Clone for std_ranges___unique_fn {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___unique_fn {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges___unique_copy_fn {}
impl Clone for std_ranges___unique_copy_fn {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___unique_copy_fn {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges___reverse_fn {}
impl Clone for std_ranges___reverse_fn {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___reverse_fn {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static reverse_367: Value<std_ranges___reverse_fn> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges___reverse_copy_fn {}
impl Clone for std_ranges___reverse_copy_fn {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___reverse_copy_fn {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges___rotate_fn {}
impl Clone for std_ranges___rotate_fn {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___rotate_fn {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges___rotate_copy_fn {}
impl Clone for std_ranges___rotate_copy_fn {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___rotate_copy_fn {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges___sample_fn {}
impl Clone for std_ranges___sample_fn {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___sample_fn {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges___shuffle_fn {}
impl Clone for std_ranges___shuffle_fn {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___shuffle_fn {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges___push_heap_fn {}
impl Clone for std_ranges___push_heap_fn {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___push_heap_fn {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static push_heap_368: Value<std_ranges___push_heap_fn> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges___pop_heap_fn {}
impl Clone for std_ranges___pop_heap_fn {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___pop_heap_fn {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static pop_heap_369: Value<std_ranges___pop_heap_fn> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges___make_heap_fn {}
impl Clone for std_ranges___make_heap_fn {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___make_heap_fn {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static make_heap_370: Value<std_ranges___make_heap_fn> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges___sort_heap_fn {}
impl Clone for std_ranges___sort_heap_fn {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___sort_heap_fn {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static sort_heap_371: Value<std_ranges___sort_heap_fn> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges___is_heap_until_fn {}
impl Clone for std_ranges___is_heap_until_fn {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___is_heap_until_fn {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static is_heap_until_372: Value<std_ranges___is_heap_until_fn> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges___is_heap_fn {}
impl Clone for std_ranges___is_heap_fn {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___is_heap_fn {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges___sort_fn {}
impl Clone for std_ranges___sort_fn {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___sort_fn {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges___stable_sort_fn {}
impl Clone for std_ranges___stable_sort_fn {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___stable_sort_fn {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges___partial_sort_fn {}
impl Clone for std_ranges___partial_sort_fn {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___partial_sort_fn {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges___partial_sort_copy_fn {}
impl Clone for std_ranges___partial_sort_copy_fn {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___partial_sort_copy_fn {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges___is_sorted_until_fn {}
impl Clone for std_ranges___is_sorted_until_fn {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___is_sorted_until_fn {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges___is_sorted_fn {}
impl Clone for std_ranges___is_sorted_fn {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___is_sorted_fn {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges___nth_element_fn {}
impl Clone for std_ranges___nth_element_fn {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___nth_element_fn {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges___lower_bound_fn {}
impl Clone for std_ranges___lower_bound_fn {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___lower_bound_fn {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static lower_bound_373: Value<std_ranges___lower_bound_fn> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges___upper_bound_fn {}
impl Clone for std_ranges___upper_bound_fn {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___upper_bound_fn {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static upper_bound_374: Value<std_ranges___upper_bound_fn> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges___equal_range_fn {}
impl Clone for std_ranges___equal_range_fn {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___equal_range_fn {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges___binary_search_fn {}
impl Clone for std_ranges___binary_search_fn {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___binary_search_fn {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges___is_partitioned_fn {}
impl Clone for std_ranges___is_partitioned_fn {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___is_partitioned_fn {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges___partition_fn {}
impl Clone for std_ranges___partition_fn {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___partition_fn {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges___stable_partition_fn {}
impl Clone for std_ranges___stable_partition_fn {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___stable_partition_fn {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges___partition_copy_fn {}
impl Clone for std_ranges___partition_copy_fn {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___partition_copy_fn {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges___partition_point_fn {}
impl Clone for std_ranges___partition_point_fn {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___partition_point_fn {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges___merge_fn {}
impl Clone for std_ranges___merge_fn {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___merge_fn {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges___inplace_merge_fn {}
impl Clone for std_ranges___inplace_merge_fn {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___inplace_merge_fn {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges___includes_fn {}
impl Clone for std_ranges___includes_fn {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___includes_fn {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges___set_union_fn {}
impl Clone for std_ranges___set_union_fn {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___set_union_fn {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges___set_intersection_fn {}
impl Clone for std_ranges___set_intersection_fn {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___set_intersection_fn {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges___set_difference_fn {}
impl Clone for std_ranges___set_difference_fn {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___set_difference_fn {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges___set_symmetric_difference_fn {}
impl Clone for std_ranges___set_symmetric_difference_fn {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___set_symmetric_difference_fn {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges___max_fn {}
impl Clone for std_ranges___max_fn {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___max_fn {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges___clamp_fn {}
impl Clone for std_ranges___clamp_fn {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___clamp_fn {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges___minmax_fn {}
impl Clone for std_ranges___minmax_fn {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___minmax_fn {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges___min_element_fn {}
impl Clone for std_ranges___min_element_fn {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___min_element_fn {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges___max_element_fn {}
impl Clone for std_ranges___max_element_fn {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___max_element_fn {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges___minmax_element_fn {}
impl Clone for std_ranges___minmax_element_fn {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___minmax_element_fn {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges___lexicographical_compare_fn {}
impl Clone for std_ranges___lexicographical_compare_fn {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___lexicographical_compare_fn {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_ranges___lexicographical_compare_fn {}
impl std_ranges___lexicographical_compare_fn {}
impl std_ranges___lexicographical_compare_fn {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges___next_permutation_fn {}
impl Clone for std_ranges___next_permutation_fn {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___next_permutation_fn {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_ranges___prev_permutation_fn {}
impl Clone for std_ranges___prev_permutation_fn {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_ranges___prev_permutation_fn {
    fn default() -> Self {
        unimplemented!()
    }
}
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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct X {
    #[offset(0)]
    pub v: i32,
}
impl std::cmp::Ord for X {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            if ((operator_lt_375(self.clone(), other.clone())) != 0) {
                std::cmp::Ordering::Less
            } else if ((operator_lt_375(other.clone(), self.clone())) != 0) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for X {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for X {
    fn eq(&self, other: &Self) -> bool {
        { ((operator_eq_376(self.clone(), other.clone())) != 0) }
    }
}
impl std::cmp::Eq for X {}
pub fn operator_eq_376(mut a: X, mut b: X) -> i32 {
    return if (a.v == b.v) { 2 } else { 0 };
}
pub fn operator_ne_377(mut a: X, mut b: X) -> i32 {
    return if (a.v != b.v) { 3 } else { 0 };
}
pub fn operator_lt_375(mut a: X, mut b: X) -> i32 {
    return if (a.v < b.v) { 4 } else { 0 };
}
pub fn operator_gt_378(mut a: X, mut b: X) -> i32 {
    return if (a.v > b.v) { 5 } else { 0 };
}
pub fn operator_le_379(mut a: X, mut b: X) -> i32 {
    return if (a.v <= b.v) { 6 } else { 0 };
}
pub fn operator_ge_380(mut a: X, mut b: X) -> i32 {
    return if (a.v >= b.v) { 7 } else { 0 };
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Result {
    #[offset(0)]
    pub r: i32,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Custom {
    #[offset(0)]
    pub v: i32,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Mixed {
    #[offset(0)]
    pub v: i32,
}
impl std::cmp::Ord for Mixed {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            if ((MixedImpl::operator_lt(
                &Rc::new(RefCell::new(Mixed { v: self.v.clone() })).as_pointer(),
                Rc::new(RefCell::new(Mixed { v: other.v.clone() })).as_pointer(),
            )) != 0)
            {
                std::cmp::Ordering::Less
            } else if ((MixedImpl::operator_lt(
                &Rc::new(RefCell::new(Mixed { v: other.v.clone() })).as_pointer(),
                Rc::new(RefCell::new(Mixed { v: self.v.clone() })).as_pointer(),
            )) != 0)
            {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for Mixed {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for Mixed {
    fn eq(&self, other: &Self) -> bool {
        {
            ((MixedImpl::operator_eq(
                &Rc::new(RefCell::new(Mixed { v: self.v.clone() })).as_pointer(),
                Rc::new(RefCell::new(Mixed { v: other.v.clone() })).as_pointer(),
            )) != 0)
        }
    }
}
impl std::cmp::Eq for Mixed {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct MyBool {
    #[offset(0)]
    pub value: bool,
}
impl MyBool {
    pub fn new(mut v: bool) -> Self {
        Self { value: v }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Boolish {
    #[offset(0)]
    pub v: i32,
}
impl std::cmp::Ord for Boolish {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            if {
                let __cmp = &operator_lt_381(self.clone(), other.clone());
                MyBoolImpl::to_bool(
                    &Rc::new(RefCell::new(MyBool {
                        value: __cmp.value.clone(),
                    }))
                    .as_pointer(),
                )
            } {
                std::cmp::Ordering::Less
            } else if {
                let __cmp = &operator_lt_381(other.clone(), self.clone());
                MyBoolImpl::to_bool(
                    &Rc::new(RefCell::new(MyBool {
                        value: __cmp.value.clone(),
                    }))
                    .as_pointer(),
                )
            } {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for Boolish {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for Boolish {
    fn eq(&self, other: &Self) -> bool {
        {
            {
                let __cmp = &operator_eq_382(self.clone(), other.clone());
                MyBoolImpl::to_bool(
                    &Rc::new(RefCell::new(MyBool {
                        value: __cmp.value.clone(),
                    }))
                    .as_pointer(),
                )
            }
        }
    }
}
impl std::cmp::Eq for Boolish {}
pub fn operator_eq_382(mut a: Boolish, mut b: Boolish) -> MyBool {
    return MyBool::new({ (a.v == b.v) });
}
pub fn operator_lt_381(mut a: Boolish, mut b: Boolish) -> MyBool {
    return MyBool::new({ (a.v < b.v) });
}
#[derive(Record, ByteRepr, FnPtrArg, Default)]
#[byte_size(1)]
pub struct RefBool {
    #[offset(0)]
    pub value: bool,
}
impl RefBool {
    pub fn new(mut v: bool) -> Self {
        Self { value: v }
    }
}
thread_local!(
    pub static ref_true_383: Value<RefBool> = Rc::new(RefCell::new(RefBool::new({ true })));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct RefInt {
    #[offset(0)]
    pub v: i32,
}
impl std::cmp::PartialEq for RefInt {
    fn eq(&self, other: &Self) -> bool {
        {
            {
                let __cmp = RefIntImpl::operator_eq(
                    &Rc::new(RefCell::new(RefInt { v: self.v.clone() })).as_pointer(),
                    Rc::new(RefCell::new(RefInt { v: other.v.clone() })).as_pointer(),
                );
                RefBoolImpl::to_bool(&__cmp)
            }
        }
    }
}
impl std::cmp::Eq for RefInt {}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut a: X = X { v: 1 };
    let mut b: X = X { v: 2 };
    let mut c: X = X { v: 1 };
    (if (({
        let _a: X = (a).clone();
        operator_eq_376(_a, (c).clone())
    }) == 2)
    {
        &(0);
    } else {
        ({
            __assert_fail_384(
                Ptr::<i8>::from_string_literal(b"(a == c) == 2"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({
        let _a: X = (a).clone();
        operator_eq_376(_a, (b).clone())
    }) == 0)
    {
        &(0);
    } else {
        ({
            __assert_fail_384(
                Ptr::<i8>::from_string_literal(b"(a == b) == 0"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({
        let _a: X = (a).clone();
        operator_ne_377(_a, (b).clone())
    }) == 3)
    {
        &(0);
    } else {
        ({
            __assert_fail_384(
                Ptr::<i8>::from_string_literal(b"(a != b) == 3"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({
        let _a: X = (a).clone();
        operator_lt_375(_a, (b).clone())
    }) == 4)
    {
        &(0);
    } else {
        ({
            __assert_fail_384(
                Ptr::<i8>::from_string_literal(b"(a < b) == 4"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({
        let _a: X = (b).clone();
        operator_gt_378(_a, (a).clone())
    }) == 5)
    {
        &(0);
    } else {
        ({
            __assert_fail_384(
                Ptr::<i8>::from_string_literal(b"(b > a) == 5"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({
        let _a: X = (a).clone();
        operator_le_379(_a, (c).clone())
    }) == 6)
    {
        &(0);
    } else {
        ({
            __assert_fail_384(
                Ptr::<i8>::from_string_literal(b"(a <= c) == 6"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({
        let _a: X = (a).clone();
        operator_ge_380(_a, (c).clone())
    }) == 7)
    {
        &(0);
    } else {
        ({
            __assert_fail_384(
                Ptr::<i8>::from_string_literal(b"(a >= c) == 7"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let xs: Value<Box<[X]>> = Rc::new(RefCell::new(Box::new([X { v: 3 }, X { v: 1 }, X { v: 2 }])));
    ({
        let ___first: Ptr<X> = (xs.as_pointer() as Ptr<X>);
        let ___last: Ptr<X> = (xs.as_pointer() as Ptr<X>).offset((3) as isize);
        sort_385(___first, ___last)
    });
    (if (({ (*xs.borrow())[(0) as usize].v } == 1) && ({ (*xs.borrow())[(1) as usize].v } == 2))
        && ({ (*xs.borrow())[(2) as usize].v } == 3)
    {
        &(0);
    } else {
        ({
            __assert_fail_384(
                Ptr::<i8>::from_string_literal(b"xs[0].v == 1 && xs[1].v == 2 && xs[2].v == 3"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let p: Value<Custom> = Rc::new(RefCell::new(Custom { v: 5 }));
    let q: Value<Custom> = Rc::new(RefCell::new(Custom { v: 2 }));
    let mut r: Result = ({ CustomImpl::operator_cmp(&p.as_pointer(), q.as_pointer()) });
    (if (r.r == 3) {
        &(0);
    } else {
        ({
            __assert_fail_384(
                Ptr::<i8>::from_string_literal(b"r.r == 3"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let m: Value<Mixed> = Rc::new(RefCell::new(Mixed { v: 4 }));
    let n: Value<Mixed> = Rc::new(RefCell::new(Mixed { v: 4 }));
    (if (({ MixedImpl::operator_eq(&m.as_pointer(), n.as_pointer()) }) == 9) {
        &(0);
    } else {
        ({
            __assert_fail_384(
                Ptr::<i8>::from_string_literal(b"(m == n) == 9"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ({ ({ MixedImpl::operator_cmp(&m.as_pointer(), n.as_pointer()) }).r } == 0) {
        &(0);
    } else {
        ({
            __assert_fail_384(
                Ptr::<i8>::from_string_literal(b"(m <=> n).r == 0"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let ms: Value<Box<[Mixed]>> = Rc::new(RefCell::new(Box::new([
        Mixed { v: 6 },
        Mixed { v: 4 },
        Mixed { v: 5 },
    ])));
    ({
        let ___first: Ptr<Mixed> = (ms.as_pointer() as Ptr<Mixed>);
        let ___last: Ptr<Mixed> = (ms.as_pointer() as Ptr<Mixed>).offset((3) as isize);
        sort_386(___first, ___last)
    });
    (if (({ (*ms.borrow())[(0) as usize].v } == 4) && ({ (*ms.borrow())[(1) as usize].v } == 5))
        && ({ (*ms.borrow())[(2) as usize].v } == 6)
    {
        &(0);
    } else {
        ({
            __assert_fail_384(
                Ptr::<i8>::from_string_literal(b"ms[0].v == 4 && ms[1].v == 5 && ms[2].v == 6"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({
        let _o: Ptr<Mixed> = (ms.as_pointer() as Ptr<Mixed>).offset(1);
        MixedImpl::operator_lt(&(ms.as_pointer() as Ptr<Mixed>).offset(0), _o)
    }) == 8)
    {
        &(0);
    } else {
        ({
            __assert_fail_384(
                Ptr::<i8>::from_string_literal(b"(ms[0] < ms[1]) == 8"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut b1: Boolish = Boolish { v: 1 };
    let mut b2: Boolish = Boolish { v: 2 };
    (if ({
        MyBoolImpl::to_bool(
            &Rc::new(RefCell::new(
                ({
                    let _a: Boolish = (b1).clone();
                    operator_eq_382(_a, Boolish { v: 1 })
                }),
            ))
            .as_pointer(),
        )
    }) {
        &(0);
    } else {
        ({
            __assert_fail_384(
                Ptr::<i8>::from_string_literal(b"b1 == Boolish{1}"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if !({
        MyBoolImpl::to_bool(
            &Rc::new(RefCell::new(
                ({
                    let _a: Boolish = (b2).clone();
                    operator_lt_381(_a, (b1).clone())
                }),
            ))
            .as_pointer(),
        )
    }) {
        &(0);
    } else {
        ({
            __assert_fail_384(
                Ptr::<i8>::from_string_literal(b"!(b2 < b1)"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut lt: MyBool = ({
        let _a: Boolish = (b1).clone();
        operator_lt_381(_a, (b2).clone())
    });
    (if lt.value {
        &(0);
    } else {
        ({
            __assert_fail_384(
                Ptr::<i8>::from_string_literal(b"lt.value"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let bs: Value<Box<[Boolish]>> = Rc::new(RefCell::new(Box::new([
        Boolish { v: 3 },
        Boolish { v: 1 },
        Boolish { v: 2 },
    ])));
    ({
        let ___first: Ptr<Boolish> = (bs.as_pointer() as Ptr<Boolish>);
        let ___last: Ptr<Boolish> = (bs.as_pointer() as Ptr<Boolish>).offset((3) as isize);
        sort_387(___first, ___last)
    });
    (if (({ (*bs.borrow())[(0) as usize].v } == 1) && ({ (*bs.borrow())[(1) as usize].v } == 2))
        && ({ (*bs.borrow())[(2) as usize].v } == 3)
    {
        &(0);
    } else {
        ({
            __assert_fail_384(
                Ptr::<i8>::from_string_literal(b"bs[0].v == 1 && bs[1].v == 2 && bs[2].v == 3"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let ri1: Value<RefInt> = Rc::new(RefCell::new(RefInt { v: 1 }));
    let ri2: Value<RefInt> = Rc::new(RefCell::new(RefInt { v: 2 }));
    (if ({
        RefBoolImpl::to_bool(&({ RefIntImpl::operator_eq(&ri1.as_pointer(), ri2.as_pointer()) }))
    }) {
        &(0);
    } else {
        ({
            __assert_fail_384(
                Ptr::<i8>::from_string_literal(b"ri1 == ri2"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let ris: Value<Box<[RefInt]>> =
        Rc::new(RefCell::new(Box::new([RefInt { v: 1 }, RefInt { v: 2 }])));
    (if (({
        let ___first: Ptr<RefInt> = (ris.as_pointer() as Ptr<RefInt>);
        let ___last: Ptr<RefInt> = (ris.as_pointer() as Ptr<RefInt>).offset((2) as isize);
        find_388(___first, ___last, ri2.as_pointer())
    }) == (ris.as_pointer() as Ptr<RefInt>))
    {
        &(0);
    } else {
        ({
            __assert_fail_384(
                Ptr::<i8>::from_string_literal(b"std::find(ris, ris + 2, ri2) == ris"),
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
pub struct std_ranges___access__Decay_copy;
pub fn __assert_fail_384(
    mut __assertion: Ptr<i8>,
    mut __file: Ptr<i8>,
    mut __line: u32,
    mut __function: Ptr<i8>,
) {
    unimplemented!()
}
pub fn sort_385(mut __first: Ptr<X>, mut __last: Ptr<X>) {
    unimplemented!()
}
pub fn sort_386(mut __first: Ptr<Mixed>, mut __last: Ptr<Mixed>) {
    unimplemented!()
}
pub fn sort_387(mut __first: Ptr<Boolish>, mut __last: Ptr<Boolish>) {
    unimplemented!()
}
pub fn find_388(
    mut __first: Ptr<RefInt>,
    mut __last: Ptr<RefInt>,
    __val: Ptr<RefInt>,
) -> Ptr<RefInt> {
    unimplemented!()
}
pub trait CustomImpl {
    fn operator_cmp(&self, o: Ptr<Custom>) -> Result;
}
impl CustomImpl for Ptr<Custom> {
    fn operator_cmp(&self, o: Ptr<Custom>) -> Result {
        return Result {
            r: ({ (*self).with(|__s| __s.v) } - { o.with(|__s| __s.v) }),
        };
    }
}
pub trait MixedImpl {
    fn operator_cmp(&self, o: Ptr<Mixed>) -> Result;
    fn operator_lt(&self, o: Ptr<Mixed>) -> i32;
    fn operator_eq(&self, o: Ptr<Mixed>) -> i32;
}
impl MixedImpl for Ptr<Mixed> {
    fn operator_cmp(&self, o: Ptr<Mixed>) -> Result {
        return Result {
            r: ({ (*self).with(|__s| __s.v) } - { o.with(|__s| __s.v) }),
        };
    }
    fn operator_lt(&self, o: Ptr<Mixed>) -> i32 {
        return if ({ (*self).with(|__s| __s.v) } < { o.with(|__s| __s.v) }) {
            8
        } else {
            0
        };
    }
    fn operator_eq(&self, o: Ptr<Mixed>) -> i32 {
        return if ({ (*self).with(|__s| __s.v) } == { o.with(|__s| __s.v) }) {
            9
        } else {
            0
        };
    }
}
pub trait MyBoolImpl {
    fn to_bool(&self) -> bool;
}
impl MyBoolImpl for Ptr<MyBool> {
    fn to_bool(&self) -> bool {
        return (*self).with(|__s| __s.value);
    }
}
pub trait RefBoolImpl {
    fn to_bool(&self) -> bool;
}
impl RefBoolImpl for Ptr<RefBool> {
    fn to_bool(&self) -> bool {
        return (*self).with(|__s| __s.value);
    }
}
pub trait RefIntImpl {
    fn operator_eq(&self, _a0: Ptr<RefInt>) -> Ptr<RefBool>;
}
impl RefIntImpl for Ptr<RefInt> {
    fn operator_eq(&self, _a0: Ptr<RefInt>) -> Ptr<RefBool> {
        return ref_true_383.with(|v| v.as_pointer());
    }
}
pub trait _Iter_equals_val_320Impl {
    fn operator_call_321(&self, mut __it: Ptr<RefInt>) -> bool {
        unimplemented!()
    }
}
impl _Iter_equals_val_320Impl for Ptr<_Iter_equals_val_320> {}
pub trait __gnu_cxx___ops__Iter_less_iterImpl {
    fn operator_call_311(&self, mut __it1: Ptr<X>, mut __it2: Ptr<X>) -> bool {
        unimplemented!()
    }
    fn operator_call_312(&self, mut __it1: Ptr<Mixed>, mut __it2: Ptr<Mixed>) -> bool {
        unimplemented!()
    }
    fn operator_call_313(&self, mut __it1: Ptr<Boolish>, mut __it2: Ptr<Boolish>) -> bool {
        unimplemented!()
    }
}
impl __gnu_cxx___ops__Iter_less_iterImpl for Ptr<__gnu_cxx___ops__Iter_less_iter> {}
pub trait __gnu_cxx___ops__Iter_less_valImpl {
    fn operator_call_314(&self, mut __it: Ptr<X>, __val: Ptr<X>) -> bool {
        unimplemented!()
    }
    fn operator_call_315(&self, mut __it: Ptr<Mixed>, __val: Ptr<Mixed>) -> bool {
        unimplemented!()
    }
    fn operator_call_316(&self, mut __it: Ptr<Boolish>, __val: Ptr<Boolish>) -> bool {
        unimplemented!()
    }
}
impl __gnu_cxx___ops__Iter_less_valImpl for Ptr<__gnu_cxx___ops__Iter_less_val> {}
pub trait __gnu_cxx___ops__Val_less_iterImpl {
    fn operator_call_317(&self, __val: Ptr<X>, mut __it: Ptr<X>) -> bool {
        unimplemented!()
    }
    fn operator_call_318(&self, __val: Ptr<Mixed>, mut __it: Ptr<Mixed>) -> bool {
        unimplemented!()
    }
    fn operator_call_319(&self, __val: Ptr<Boolish>, mut __it: Ptr<Boolish>) -> bool {
        unimplemented!()
    }
}
impl __gnu_cxx___ops__Val_less_iterImpl for Ptr<__gnu_cxx___ops__Val_less_iter> {}
pub trait integral_constant_0Impl {
    fn to_bool(&self) -> bool {
        unimplemented!()
    }
}
impl integral_constant_0Impl for Ptr<integral_constant_0> {}
pub trait std_ranges___detail___max_diff_typeImpl {
    fn operator_inc_9(&self) -> Ptr<std_ranges___detail___max_diff_type> {
        unimplemented!()
    }
    fn operator_dec_11(&self) -> Ptr<std_ranges___detail___max_diff_type> {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        __r: Ptr<std_ranges___detail___max_diff_type>,
    ) -> Ptr<std_ranges___detail___max_diff_type> {
        unimplemented!()
    }
    fn operator_sub_assign(
        &self,
        __r: Ptr<std_ranges___detail___max_diff_type>,
    ) -> Ptr<std_ranges___detail___max_diff_type> {
        unimplemented!()
    }
    fn operator_mul_assign(
        &self,
        __r: Ptr<std_ranges___detail___max_diff_type>,
    ) -> Ptr<std_ranges___detail___max_diff_type> {
        unimplemented!()
    }
    fn operator_div_assign(
        &self,
        __r: Ptr<std_ranges___detail___max_diff_type>,
    ) -> Ptr<std_ranges___detail___max_diff_type> {
        unimplemented!()
    }
    fn operator_rem_assign(
        &self,
        __r: Ptr<std_ranges___detail___max_diff_type>,
    ) -> Ptr<std_ranges___detail___max_diff_type> {
        unimplemented!()
    }
    fn operator_shl_assign(
        &self,
        __r: Ptr<std_ranges___detail___max_diff_type>,
    ) -> Ptr<std_ranges___detail___max_diff_type> {
        unimplemented!()
    }
    fn operator_shr_assign(
        &self,
        __r: Ptr<std_ranges___detail___max_diff_type>,
    ) -> Ptr<std_ranges___detail___max_diff_type> {
        unimplemented!()
    }
    fn operator_bitand_assign(
        &self,
        __r: Ptr<std_ranges___detail___max_diff_type>,
    ) -> Ptr<std_ranges___detail___max_diff_type> {
        unimplemented!()
    }
    fn operator_bitor_assign(
        &self,
        __r: Ptr<std_ranges___detail___max_diff_type>,
    ) -> Ptr<std_ranges___detail___max_diff_type> {
        unimplemented!()
    }
    fn operator_bitxor_assign(
        &self,
        __r: Ptr<std_ranges___detail___max_diff_type>,
    ) -> Ptr<std_ranges___detail___max_diff_type> {
        unimplemented!()
    }
    fn operator_cmp(&self, __r: Ptr<std_ranges___detail___max_diff_type>) -> std_strong_ordering {
        unimplemented!()
    }
}
impl std_ranges___detail___max_diff_typeImpl for Ptr<std_ranges___detail___max_diff_type> {}
pub trait std_ranges___detail___max_size_typeImpl {
    fn operator_bitnot(&self) -> std_ranges___detail___max_size_type {
        unimplemented!()
    }
    fn operator_neg(&self) -> std_ranges___detail___max_size_type {
        unimplemented!()
    }
    fn operator_inc_9(&self) -> Ptr<std_ranges___detail___max_size_type> {
        unimplemented!()
    }
    fn operator_dec_11(&self) -> Ptr<std_ranges___detail___max_size_type> {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        __r: Ptr<std_ranges___detail___max_size_type>,
    ) -> Ptr<std_ranges___detail___max_size_type> {
        unimplemented!()
    }
    fn operator_sub_assign(
        &self,
        __r: Ptr<std_ranges___detail___max_size_type>,
    ) -> Ptr<std_ranges___detail___max_size_type> {
        unimplemented!()
    }
    fn operator_mul_assign(
        &self,
        __r: std_ranges___detail___max_size_type,
    ) -> Ptr<std_ranges___detail___max_size_type> {
        unimplemented!()
    }
    fn operator_div_assign(
        &self,
        __r: Ptr<std_ranges___detail___max_size_type>,
    ) -> Ptr<std_ranges___detail___max_size_type> {
        unimplemented!()
    }
    fn operator_rem_assign(
        &self,
        __r: Ptr<std_ranges___detail___max_size_type>,
    ) -> Ptr<std_ranges___detail___max_size_type> {
        unimplemented!()
    }
    fn operator_shl_assign(
        &self,
        __r: Ptr<std_ranges___detail___max_size_type>,
    ) -> Ptr<std_ranges___detail___max_size_type> {
        unimplemented!()
    }
    fn operator_shr_assign(
        &self,
        __r: Ptr<std_ranges___detail___max_size_type>,
    ) -> Ptr<std_ranges___detail___max_size_type> {
        unimplemented!()
    }
    fn operator_bitand_assign(
        &self,
        __r: Ptr<std_ranges___detail___max_size_type>,
    ) -> Ptr<std_ranges___detail___max_size_type> {
        unimplemented!()
    }
    fn operator_bitor_assign(
        &self,
        __r: Ptr<std_ranges___detail___max_size_type>,
    ) -> Ptr<std_ranges___detail___max_size_type> {
        unimplemented!()
    }
    fn operator_bitxor_assign(
        &self,
        __r: Ptr<std_ranges___detail___max_size_type>,
    ) -> Ptr<std_ranges___detail___max_size_type> {
        unimplemented!()
    }
}
impl std_ranges___detail___max_size_typeImpl for Ptr<std_ranges___detail___max_size_type> {}
pub fn __cpp2rust_init_globals() {
    let _ = ref_true_383.with(|_| ());
}
