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
pub struct __conditional_11 {}
impl Clone for __conditional_11 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __conditional_11 {
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
pub struct __and__44 {}
impl Clone for __and__44 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__44 {
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
pub struct __not__46 {}
impl Clone for __not__46 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__46 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __not__47 {}
impl Clone for __not__47 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__47 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __not__48 {}
impl Clone for __not__48 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__48 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __not__49 {}
impl Clone for __not__49 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__49 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __not__50 {}
impl Clone for __not__50 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__50 {
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
pub struct is_reference_52 {}
impl Clone for is_reference_52 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_52 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_53 {}
impl Clone for is_reference_53 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_53 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_54 {}
impl Clone for is_reference_54 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_54 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_55 {}
impl Clone for is_reference_55 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_55 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_56 {}
impl Clone for is_reference_56 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_56 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_57 {}
impl Clone for is_function_57 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_57 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_58 {}
impl Clone for is_function_58 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_58 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_59 {}
impl Clone for is_function_59 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_59 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_60 {}
impl Clone for is_function_60 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_60 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_61 {}
impl Clone for is_void_61 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_61 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_62 {}
impl Clone for is_void_62 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_62 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_63 {}
impl Clone for is_void_63 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_63 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_64 {}
impl Clone for is_void_64 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_64 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_65 {}
impl Clone for is_void_65 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_65 {
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
pub struct remove_cv_67 {}
impl Clone for remove_cv_67 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_67 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_68 {}
impl Clone for remove_cv_68 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_68 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_69 {}
impl Clone for remove_cv_69 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_69 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_70 {}
impl Clone for remove_cv_70 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_70 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_71 {}
impl Clone for remove_cv_71 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_71 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_72 {}
impl Clone for remove_cv_72 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_72 {
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
pub struct is_const_74 {}
impl Clone for is_const_74 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_74 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_const_75 {}
impl Clone for is_const_75 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_75 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_const_76 {}
impl Clone for is_const_76 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_76 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_const_77 {}
impl Clone for is_const_77 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_77 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_const_78 {}
impl Clone for is_const_78 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_78 {
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
pub struct __is_array_unknown_bounds_80 {}
impl Clone for __is_array_unknown_bounds_80 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_80 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_81 {}
impl Clone for __is_array_unknown_bounds_81 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_81 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_82 {}
impl Clone for __is_array_unknown_bounds_82 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_82 {
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
pub struct is_integral_90 {}
impl Clone for is_integral_90 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_90 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_91 {}
impl Clone for is_integral_91 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_91 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_92 {}
impl Clone for is_integral_92 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_92 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_93 {}
impl Clone for is_integral_93 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_93 {
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
pub struct __is_floating_point_helper_97 {}
impl Clone for __is_floating_point_helper_97 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_floating_point_helper_97 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_floating_point_98 {}
impl Clone for is_floating_point_98 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_floating_point_98 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_pointer_99 {}
impl Clone for is_pointer_99 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_pointer_99 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_pointer_100 {}
impl Clone for is_pointer_100 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_pointer_100 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_enum_101 {}
impl Clone for is_enum_101 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_enum_101 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_null_pointer_102 {}
impl Clone for is_null_pointer_102 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_null_pointer_102 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_arithmetic_103 {}
impl Clone for is_arithmetic_103 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_arithmetic_103 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_member_pointer_104 {}
impl Clone for is_member_pointer_104 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_member_pointer_104 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_scalar_105 {}
impl Clone for is_scalar_105 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_scalar_105 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_106 {}
impl Clone for is_same_106 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_106 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_107 {}
impl Clone for is_same_107 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_107 {
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
pub struct remove_all_extents_114 {}
impl Clone for remove_all_extents_114 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_all_extents_114 {
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
pub struct __is_destructible_impl_115 {}
impl Clone for __is_destructible_impl_115 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_destructible_impl_115 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_destructible_safe_116 {}
impl Clone for __is_destructible_safe_116 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_destructible_safe_116 {
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
pub struct is_constructible_119 {}
impl Clone for is_constructible_119 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_constructible_119 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_constructible_120 {}
impl Clone for is_constructible_120 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_constructible_120 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_constructible_121 {}
impl Clone for is_constructible_121 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_constructible_121 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_default_constructible_122 {}
impl Clone for is_default_constructible_122 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_default_constructible_122 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_default_constructible_123 {}
impl Clone for is_default_constructible_123 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_default_constructible_123 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_move_constructible_124 {}
impl Clone for is_move_constructible_124 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_move_constructible_124 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_move_constructible_125 {}
impl Clone for is_move_constructible_125 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_move_constructible_125 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_default_constructible_126 {}
impl Clone for is_nothrow_default_constructible_126 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_default_constructible_126 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_copy_constructible_127 {}
impl Clone for is_nothrow_copy_constructible_127 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_copy_constructible_127 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_move_constructible_128 {}
impl Clone for is_nothrow_move_constructible_128 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_move_constructible_128 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_move_constructible_129 {}
impl Clone for is_nothrow_move_constructible_129 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_move_constructible_129 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_copy_assignable_130 {}
impl Clone for is_copy_assignable_130 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_copy_assignable_130 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_copy_assignable_131 {}
impl Clone for is_copy_assignable_131 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_copy_assignable_131 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_move_assignable_132 {}
impl Clone for is_move_assignable_132 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_move_assignable_132 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_move_assignable_133 {}
impl Clone for is_move_assignable_133 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_move_assignable_133 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_move_assignable_134 {}
impl Clone for is_move_assignable_134 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_move_assignable_134 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_move_assignable_135 {}
impl Clone for is_nothrow_move_assignable_135 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_move_assignable_135 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_move_assignable_136 {}
impl Clone for is_nothrow_move_assignable_136 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_move_assignable_136 {
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
pub struct __is_implicitly_default_constructible_impl_137 {}
impl Clone for __is_implicitly_default_constructible_impl_137 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_implicitly_default_constructible_impl_137 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_implicitly_default_constructible_impl_138 {}
impl Clone for __is_implicitly_default_constructible_impl_138 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_implicitly_default_constructible_impl_138 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_implicitly_default_constructible_safe_139 {}
impl Clone for __is_implicitly_default_constructible_safe_139 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_implicitly_default_constructible_safe_139 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_implicitly_default_constructible_safe_140 {}
impl Clone for __is_implicitly_default_constructible_safe_140 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_implicitly_default_constructible_safe_140 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_implicitly_default_constructible_141 {}
impl Clone for __is_implicitly_default_constructible_141 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_implicitly_default_constructible_141 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_implicitly_default_constructible_142 {}
impl Clone for __is_implicitly_default_constructible_142 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_implicitly_default_constructible_142 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_trivially_destructible_143 {}
impl Clone for is_trivially_destructible_143 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_trivially_destructible_143 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_convertible_144 {}
impl Clone for is_convertible_144 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_144 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_convertible_145 {}
impl Clone for is_convertible_145 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_145 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_convertible_146 {}
impl Clone for is_convertible_146 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_146 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_convertible_147 {}
impl Clone for is_convertible_147 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_147 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_convertible_148 {}
impl Clone for is_convertible_148 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_148 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_nothrow_new_constructible_impl_149 {}
impl Clone for __is_nothrow_new_constructible_impl_149 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_nothrow_new_constructible_impl_149 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_150 {}
impl Clone for remove_reference_150 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_150 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_151 {}
impl Clone for remove_reference_151 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_151 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_152 {}
impl Clone for remove_reference_152 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_152 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_153 {}
impl Clone for remove_reference_153 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_153 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_154 {}
impl Clone for remove_reference_154 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_154 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_155 {}
impl Clone for remove_reference_155 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_155 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_156 {}
impl Clone for remove_reference_156 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_156 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __cv_selector_157 {}
impl Clone for __cv_selector_157 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_157 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __cv_selector_158 {}
impl Clone for __cv_selector_158 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_158 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __cv_selector_159 {}
impl Clone for __cv_selector_159 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_159 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __cv_selector_160 {}
impl Clone for __cv_selector_160 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_160 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __cv_selector_161 {}
impl Clone for __cv_selector_161 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_161 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_162 {}
impl Clone for __match_cv_qualifiers_162 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_162 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_163 {}
impl Clone for __match_cv_qualifiers_163 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_163 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_164 {}
impl Clone for __match_cv_qualifiers_164 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_164 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_165 {}
impl Clone for __match_cv_qualifiers_165 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_165 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_166 {}
impl Clone for __match_cv_qualifiers_166 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_166 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_167 {}
impl Clone for __match_cv_qualifiers_167 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_167 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_168 {}
impl Clone for __make_unsigned_168 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_168 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_169 {}
impl Clone for __make_unsigned_169 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_169 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_170 {}
impl Clone for __make_unsigned_170 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_170 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_171 {}
impl Clone for __make_unsigned_171 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_171 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_172 {}
impl Clone for __make_unsigned_selector_172 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_172 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_173 {}
impl Clone for __make_unsigned_selector_173 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_173 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_174 {}
impl Clone for __make_unsigned_selector_174 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_174 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_175 {}
impl Clone for __make_unsigned_selector_175 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_175 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_176 {}
impl Clone for __make_unsigned_selector_176 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_176 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_177 {}
impl Clone for __make_unsigned_selector_177 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_177 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_178 {}
impl Clone for __make_unsigned_selector_178 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_178 {
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
pub struct _List_182 {}
impl Clone for _List_182 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_182 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static std___make_unsigned_selector_base__List_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long____size_183 : Value<usize > = unimplemented!() ;
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_184 {}
impl Clone for _List_184 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_184 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static std___make_unsigned_selector_base__List_unsigned_short__unsigned_int__unsigned_long__unsigned_long_long____size_185 : Value<usize > = unimplemented!() ;
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_186 {}
impl Clone for _List_186 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_186 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static std___make_unsigned_selector_base__List_unsigned_int__unsigned_long__unsigned_long_long____size_187 : Value<usize > = unimplemented!() ;
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_188 {}
impl Clone for _List_188 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_188 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_189 {}
impl Clone for _List_189 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_189 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_190 {}
impl Clone for _List_190 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_190 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_191 {}
impl Clone for __select_191 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_191 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_192 {}
impl Clone for __select_192 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_192 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_193 {}
impl Clone for __select_193 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_193 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_194 {}
impl Clone for __select_194 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_194 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_195 {}
impl Clone for __select_195 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_195 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct make_unsigned_196 {}
impl Clone for make_unsigned_196 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for make_unsigned_196 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_197 {}
impl Clone for __make_signed_197 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_197 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_198 {}
impl Clone for __make_signed_198 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_198 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_199 {}
impl Clone for __make_signed_selector_199 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_199 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_200 {}
impl Clone for __make_signed_selector_200 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_200 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_201 {}
impl Clone for __make_signed_selector_201 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_201 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_202 {}
impl Clone for __make_signed_selector_202 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_202 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_203 {}
impl Clone for __make_signed_selector_203 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_203 {
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
pub struct common_type_204 {}
impl Clone for common_type_204 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for common_type_204 {
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
pub struct __is_tuple_like_impl_205 {}
impl Clone for __is_tuple_like_impl_205 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_tuple_like_impl_205 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_tuple_like_impl_206 {}
impl Clone for __is_tuple_like_impl_206 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_tuple_like_impl_206 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_tuple_like_207 {}
impl Clone for __is_tuple_like_207 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_tuple_like_207 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_tuple_like_208 {}
impl Clone for __is_tuple_like_208 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_tuple_like_208 {
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
    pub static __is_nothrow_new_constructible_209: Value<bool> = unimplemented!();
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
pub struct __are_same_210 {}
impl Clone for __are_same_210 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_210 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_211 = u32;
pub const anon_211___value: anon_211 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_212 {}
impl Clone for __are_same_212 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_212 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_213 = u32;
pub const anon_213___value: anon_213 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_214 {}
impl Clone for __are_same_214 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_214 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_215 = u32;
pub const anon_215___value: anon_215 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_216 {}
impl Clone for __are_same_216 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_216 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_217 = u32;
pub const anon_217___value: anon_217 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_218 {}
impl Clone for __are_same_218 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_218 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_219 = u32;
pub const anon_219___value: anon_219 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_220 {}
impl Clone for __are_same_220 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_220 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_221 = u32;
pub const anon_221___value: anon_221 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_222 {}
impl Clone for __is_integer_222 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_222 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_223 = u32;
pub const anon_223___value: anon_223 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_224 {}
impl Clone for __is_integer_224 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_224 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_225 = u32;
pub const anon_225___value: anon_225 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_226 {}
impl Clone for __is_integer_226 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_226 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_227 = u32;
pub const anon_227___value: anon_227 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_228 {}
impl Clone for __is_integer_228 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_228 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_229 = u32;
pub const anon_229___value: anon_229 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_230 {}
impl Clone for __is_integer_230 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_230 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_231 = u32;
pub const anon_231___value: anon_231 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_nonvolatile_trivially_copyable_232 {}
impl Clone for __is_nonvolatile_trivially_copyable_232 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_nonvolatile_trivially_copyable_232 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_233 = u32;
pub const anon_233___value: anon_233 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __memcpyable_234 {}
impl Clone for __memcpyable_234 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __memcpyable_234 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __memcpyable_integer_235 {}
impl Clone for __memcpyable_integer_235 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __memcpyable_integer_235 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_nonstrict_236 {}
impl Clone for __is_integer_nonstrict_236 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_236 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_237 = u32;
pub const anon_237___width: anon_237 = 64;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_nonstrict_238 {}
impl Clone for __is_integer_nonstrict_238 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_238 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_239 = u32;
pub const anon_239___width: anon_239 = 64;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_integer_240 {}
impl Clone for __numeric_traits_integer_240 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_240 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_long____is_signed_241: Value<bool> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_long____digits_242: Value<i32> = unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_long____max_243: Value<i64> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_integer_244 {}
impl Clone for __numeric_traits_integer_244 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_244 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_long____is_signed_245: Value<bool> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_long____digits_246: Value<i32> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_long____max_247: Value<u64> =
        unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_floating_248 {}
impl Clone for __numeric_traits_floating_248 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_248 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_floating_249 {}
impl Clone for __numeric_traits_floating_249 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_249 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_floating_250 {}
impl Clone for __numeric_traits_floating_250 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_250 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_251 {}
impl Clone for __numeric_traits_251 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_251 {
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
    pub static piecewise_construct_252: Value<std_piecewise_construct_t> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct pair_253 {
    #[offset(0)]
    #[byte_size(8)]
    pub first: Ptr<Item>,
    #[offset(8)]
    pub second: i64,
}
impl Clone for pair_253 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pair_253 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl pair_253 {
    pub fn copy_from(_a0: Ptr<pair_253>) -> Self {
        unimplemented!()
    }
}
impl pair_253 {
    pub fn move_from(_a0: Ptr<pair_253>) -> Self {
        unimplemented!()
    }
}
impl pair_253 {
    pub fn new_1() -> Self {
        unimplemented!()
    }
}
impl pair_253 {
    pub fn new_2(__x: Ptr<Ptr<Item>>, __y: Ptr<i64>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct tuple_254 {}
impl Clone for tuple_254 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for tuple_254 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl tuple_254 {}
impl tuple_254 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _PCC_255 {}
impl Clone for _PCC_255 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _PCC_255 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __pair_base_256 {}
impl Clone for __pair_base_256 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __pair_base_256 {
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
pub struct iterator_257 {}
impl Clone for iterator_257 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_257 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_258 {}
impl Clone for iterator_258 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_258 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl iterator_258 {}
impl iterator_258 {}
impl iterator_258 {}
impl iterator_258 {
    pub fn copy_assign(&self, _a0: Ptr<iterator_258>) -> Ptr<iterator_258> {
        unimplemented!()
    }
}
impl iterator_258 {
    pub fn move_assign(&self, _a0: Ptr<iterator_258>) -> Ptr<iterator_258> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_259 {}
impl Clone for iterator_259 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_259 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl iterator_259 {
    pub fn copy_assign(&self, _a0: Ptr<iterator_259>) -> Ptr<iterator_259> {
        unimplemented!()
    }
}
impl iterator_259 {
    pub fn move_assign(&self, _a0: Ptr<iterator_259>) -> Ptr<iterator_259> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_260 {}
impl Clone for iterator_260 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_260 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl iterator_260 {
    pub fn copy_assign(&self, _a0: Ptr<iterator_260>) -> Ptr<iterator_260> {
        unimplemented!()
    }
}
impl iterator_260 {
    pub fn move_assign(&self, _a0: Ptr<iterator_260>) -> Ptr<iterator_260> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_261 {}
impl Clone for iterator_traits_261 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_261 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_262 {}
impl Clone for iterator_traits_262 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_262 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_263 {}
impl Clone for iterator_traits_263 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_263 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_264 {}
impl Clone for iterator_traits_264 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_264 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_265 {}
impl Clone for iterator_traits_265 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_265 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __iterator_traits_266 {}
impl Clone for __iterator_traits_266 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __iterator_traits_266 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __iterator_traits_267 {}
impl Clone for __iterator_traits_267 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __iterator_traits_267 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __iterator_traits_268 {}
impl Clone for __iterator_traits_268 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __iterator_traits_268 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __ptr_traits_ptr_to_269 {}
impl Clone for __ptr_traits_ptr_to_269 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_269 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct pointer_traits_270 {}
impl Clone for pointer_traits_270 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_270 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct reverse_iterator_271 {}
impl Clone for reverse_iterator_271 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for reverse_iterator_271 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_271 {
    pub fn copy_from(__x: Ptr<reverse_iterator_271>) -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_271 {
    pub fn copy_assign(&self, _a0: Ptr<reverse_iterator_271>) -> Ptr<reverse_iterator_271> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct reverse_iterator_272 {}
impl Clone for reverse_iterator_272 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for reverse_iterator_272 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_272 {
    pub fn copy_from(__x: Ptr<reverse_iterator_272>) -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_272 {
    pub fn copy_assign(&self, _a0: Ptr<reverse_iterator_272>) -> Ptr<reverse_iterator_272> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
pub struct __normal_iterator_273 {}
impl Clone for __normal_iterator_273 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __normal_iterator_273 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_273 {
    pub fn new_1(__i: Ptr<Ptr<Item>>) -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_273 {}
impl __normal_iterator_273 {}
impl __normal_iterator_273 {
    pub fn copy_assign(&self, _a0: Ptr<__normal_iterator_273>) -> Ptr<__normal_iterator_273> {
        unimplemented!()
    }
}
impl __normal_iterator_273 {
    pub fn move_assign(&self, _a0: Ptr<__normal_iterator_273>) -> Ptr<__normal_iterator_273> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
pub struct __normal_iterator_274 {}
impl Clone for __normal_iterator_274 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __normal_iterator_274 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_274 {
    pub fn new_1(__i: Ptr<Ptr<Item>>) -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_274 {}
impl __normal_iterator_274 {}
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
pub struct _Iter_comp_iter_275 {
    #[offset(0)]
    #[byte_size(8)]
    pub _M_comp: FnPtr<fn(Ptr<Item>, Ptr<Item>) -> bool>,
}
impl Clone for _Iter_comp_iter_275 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Iter_comp_iter_275 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Iter_comp_iter_275 {
    pub fn new(__comp: FnPtr<fn(Ptr<Item>, Ptr<Item>) -> bool>) -> Self {
        unimplemented!()
    }
}
impl _Iter_comp_iter_275 {}
impl _Iter_comp_iter_275 {}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(8)]
pub struct _Iter_comp_val_280 {
    #[offset(0)]
    #[byte_size(8)]
    pub _M_comp: FnPtr<fn(Ptr<Item>, Ptr<Item>) -> bool>,
}
impl Clone for _Iter_comp_val_280 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Iter_comp_val_280 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Iter_comp_val_280 {
    pub fn new_1(__comp: Ptr<_Iter_comp_iter_275>) -> Self {
        unimplemented!()
    }
}
impl _Iter_comp_val_280 {}
impl _Iter_comp_val_280 {}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(8)]
pub struct _Val_comp_iter_282 {
    #[offset(0)]
    #[byte_size(8)]
    pub _M_comp: FnPtr<fn(Ptr<Item>, Ptr<Item>) -> bool>,
}
impl Clone for _Val_comp_iter_282 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Val_comp_iter_282 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Val_comp_iter_282 {
    pub fn new_1(__comp: Ptr<_Iter_comp_iter_275>) -> Self {
        unimplemented!()
    }
}
impl _Val_comp_iter_282 {}
impl _Val_comp_iter_282 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct initializer_list_285 {}
impl Clone for initializer_list_285 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for initializer_list_285 {
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
#[byte_size(24)]
pub struct _Temporary_buffer_286 {}
impl Clone for _Temporary_buffer_286 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Temporary_buffer_286 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct std__Temporary_buffer___gnu_cxx___normal_iterator_Item_ptr__std_vector_Item____Item___Impl
{
    #[offset(0)]
    pub _M_len: i64,
    #[offset(8)]
    #[byte_size(8)]
    pub _M_buffer: Ptr<Item>,
}
impl Clone
    for std__Temporary_buffer___gnu_cxx___normal_iterator_Item_ptr__std_vector_Item____Item___Impl
{
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default
    for std__Temporary_buffer___gnu_cxx___normal_iterator_Item_ptr__std_vector_Item____Item___Impl
{
    fn default() -> Self {
        unimplemented!()
    }
}
impl std__Temporary_buffer___gnu_cxx___normal_iterator_Item_ptr__std_vector_Item____Item___Impl {
    pub fn new(mut __original_len: i64) -> Self {
        unimplemented!()
    }
}
impl std__Temporary_buffer___gnu_cxx___normal_iterator_Item_ptr__std_vector_Item____Item___Impl {}
impl _Temporary_buffer_286 {
    pub fn new(__seed: __normal_iterator_273, mut __original_len: i64) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __uninitialized_construct_buf_dispatch_287 {}
impl Clone for __uninitialized_construct_buf_dispatch_287 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __uninitialized_construct_buf_dispatch_287 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __uninitialized_construct_buf_dispatch_287 {
    pub fn __ucr_288(mut _a0: Ptr<Item>, mut _a1: Ptr<Item>, _a2: __normal_iterator_273) {
        unimplemented!()
    }
}
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
    pub __value32: anon_289,
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
pub struct anon_289 {
    #[offset(0)]
    pub __low: u32,
    #[offset(4)]
    pub __high: u32,
}
impl Clone for anon_289 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for anon_289 {
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
pub type anon_290 = u32;
pub const anon_290__S_threshold: anon_290 = 16;
pub type anon_291 = u32;
pub const anon_291__S_chunk_size: anon_291 = 7;
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
#[byte_size(1)]
pub struct __new_allocator_292 {}
impl Clone for __new_allocator_292 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_292 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_292 {
    pub fn copy_from(_a0: Ptr<__new_allocator_292>) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_292 {
    pub fn copy_assign(&self, _a0: Ptr<__new_allocator_292>) -> Ptr<__new_allocator_292> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __new_allocator_293 {}
impl Clone for __new_allocator_293 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_293 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_293 {
    pub fn copy_from(_a0: Ptr<__new_allocator_293>) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_293 {
    pub fn copy_assign(&self, _a0: Ptr<__new_allocator_293>) -> Ptr<__new_allocator_293> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __new_allocator_294 {}
impl Clone for __new_allocator_294 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_294 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_294 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_294 {
    pub fn copy_from(_a0: Ptr<__new_allocator_294>) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_294 {
    pub fn copy_assign(&self, _a0: Ptr<__new_allocator_294>) -> Ptr<__new_allocator_294> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_296 {}
impl Clone for allocator_296 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_296 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_296 {}
impl allocator_296 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_297 {}
impl Clone for allocator_297 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_297 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_297 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl allocator_297 {
    pub fn copy_from(__a: Ptr<allocator_297>) -> Self {
        unimplemented!()
    }
}
impl allocator_297 {
    pub fn copy_assign(&self, _a0: Ptr<allocator_297>) -> Ptr<allocator_297> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_traits_298 {}
impl Clone for allocator_traits_298 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_traits_298 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_traits_298 {
    pub fn allocate_1(__a: Ptr<allocator_297>, mut __n: u64) -> Ptr<Item> {
        unimplemented!()
    }
}
impl allocator_traits_298 {
    pub fn deallocate(__a: Ptr<allocator_297>, mut __p: Ptr<Item>, mut __n: u64) {
        unimplemented!()
    }
}
impl allocator_traits_298 {
    pub fn max_size(__a: Ptr<allocator_297>) -> u64 {
        unimplemented!()
    }
}
impl allocator_traits_298 {
    pub fn construct_299(__a: Ptr<allocator_297>, mut __p: Ptr<Item>, __args: Ptr<Item>) {
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
pub struct __is_move_insertable_301 {}
impl Clone for __is_move_insertable_301 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_move_insertable_301 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __alloc_traits_302 {}
impl Clone for __alloc_traits_302 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_302 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct rebind_303 {}
impl Clone for rebind_303 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_303 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_bitwise_relocatable_304 {}
impl Clone for __is_bitwise_relocatable_304 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_bitwise_relocatable_304 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(24)]
pub struct _Vector_base_305 {
    #[offset(0)]
    #[byte_size(24)]
    pub _M_impl: std__Vector_base_Item__std_allocator_Item____Vector_impl,
}
impl Clone for _Vector_base_305 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Vector_base_305 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(24)]
pub struct std__Vector_base_Item__std_allocator_Item____Vector_impl_data {
    #[offset(0)]
    #[byte_size(8)]
    pub _M_start: Ptr<Item>,
    #[offset(8)]
    #[byte_size(8)]
    pub _M_finish: Ptr<Item>,
    #[offset(16)]
    #[byte_size(8)]
    pub _M_end_of_storage: Ptr<Item>,
}
impl Clone for std__Vector_base_Item__std_allocator_Item____Vector_impl_data {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std__Vector_base_Item__std_allocator_Item____Vector_impl_data {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_Item__std_allocator_Item____Vector_impl_data {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_Item__std_allocator_Item____Vector_impl_data {
    pub fn move_from(
        __x: Ptr<std__Vector_base_Item__std_allocator_Item____Vector_impl_data>,
    ) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(24)]
pub struct std__Vector_base_Item__std_allocator_Item____Vector_impl {}
impl Clone for std__Vector_base_Item__std_allocator_Item____Vector_impl {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std__Vector_base_Item__std_allocator_Item____Vector_impl {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_Item__std_allocator_Item____Vector_impl {
    pub fn new_1() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_Item__std_allocator_Item____Vector_impl {
    pub fn move_from(__x: Ptr<std__Vector_base_Item__std_allocator_Item____Vector_impl>) -> Self {
        unimplemented!()
    }
}
impl _Vector_base_305 {
    pub fn new_1() -> Self {
        unimplemented!()
    }
}
impl _Vector_base_305 {
    pub fn move_from(_a0: Ptr<_Vector_base_305>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(24)]
pub struct vector_306 {}
impl Clone for vector_306 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for vector_306 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(24)]
pub struct std_vector_Item___Guard_alloc {
    #[offset(0)]
    #[byte_size(8)]
    pub _M_storage: Ptr<Item>,
    #[offset(8)]
    pub _M_len: u64,
    #[offset(16)]
    #[byte_size(8)]
    pub _M_vect: Ptr<_Vector_base_305>,
}
impl Clone for std_vector_Item___Guard_alloc {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_vector_Item___Guard_alloc {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_vector_Item___Guard_alloc {
    pub fn new(mut __s: Ptr<Item>, mut __l: u64, __vect: Ptr<_Vector_base_305>) -> Self {
        unimplemented!()
    }
}
impl vector_306 {
    pub fn new_1() -> Self {
        unimplemented!()
    }
}
impl vector_306 {
    pub fn copy_from(__x: Ptr<vector_306>) -> Self {
        unimplemented!()
    }
}
impl vector_306 {
    pub fn move_from(_a0: Ptr<vector_306>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_308 {}
impl Clone for __hash_base_308 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_308 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_309 {}
impl Clone for __hash_base_309 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_309 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_310 {}
impl Clone for __hash_base_310 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_310 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_311 {}
impl Clone for __hash_base_311 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_311 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_312 {}
impl Clone for __hash_base_312 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_312 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_313 {}
impl Clone for __hash_base_313 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_313 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_314 {}
impl Clone for __hash_base_314 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_314 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_315 {}
impl Clone for __hash_base_315 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_315 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_316 {}
impl Clone for __hash_base_316 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_316 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_317 {}
impl Clone for __hash_base_317 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_317 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_318 {}
impl Clone for __hash_base_318 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_318 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_319 {}
impl Clone for __hash_base_319 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_319 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_320 {}
impl Clone for __hash_base_320 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_320 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_321 {}
impl Clone for __hash_base_321 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_321 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_322 {}
impl Clone for __hash_base_322 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_322 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_323 {}
impl Clone for __hash_base_323 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_323 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_324 {}
impl Clone for __hash_base_324 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_324 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_325 {}
impl Clone for __hash_base_325 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_325 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_326 {}
impl Clone for __hash_base_326 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_326 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_327 {}
impl Clone for __hash_base_327 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_327 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_328 {}
impl Clone for __hash_base_328 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_328 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct hash_329 {}
impl Clone for hash_329 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for hash_329 {
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
    pub fn hash_330(__val: Ptr<f32>) -> usize {
        unimplemented!()
    }
}
impl std__Hash_impl {
    pub fn hash_331(__val: Ptr<f64>) -> usize {
        unimplemented!()
    }
}
pub type anon_332 = u32;
pub const anon_332__S_word_bit: anon_332 = 64;
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
pub struct binary_function_333 {}
impl Clone for binary_function_333 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for binary_function_333 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct greater_334 {}
impl Clone for greater_334 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for greater_334 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct less_335 {}
impl Clone for less_335 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for less_335 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct greater_equal_336 {}
impl Clone for greater_equal_336 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for greater_equal_336 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct less_equal_337 {}
impl Clone for less_equal_337 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for less_equal_337 {
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
    pub static allocator_arg_338: Value<std_allocator_arg_t> = unimplemented!();
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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Item {
    #[offset(0)]
    pub key: i32,
    #[offset(4)]
    pub value: i32,
}
pub fn Compare_339(a: Ptr<Item>, b: Ptr<Item>) -> bool {
    return ({ a.with(|__s| __s.key) } < { b.with(|__s| __s.key) });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let v: Value<vector_306> = Rc::new(RefCell::new(<vector_306>::default()));
    ({
        let ___x: Value<Item> = Rc::new(RefCell::new(Item { key: 3, value: 30 }));
        vector_306Impl::push_back_58(&v.as_pointer(), ___x.as_pointer())
    });
    ({
        let ___x: Value<Item> = Rc::new(RefCell::new(Item { key: 1, value: 10 }));
        vector_306Impl::push_back_58(&v.as_pointer(), ___x.as_pointer())
    });
    ({
        let ___x: Value<Item> = Rc::new(RefCell::new(Item { key: 2, value: 20 }));
        vector_306Impl::push_back_58(&v.as_pointer(), ___x.as_pointer())
    });
    ({
        let ___first: __normal_iterator_273 = ({ vector_306Impl::begin_26(&v.as_pointer()) });
        let ___last: __normal_iterator_273 = ({ vector_306Impl::end_28(&v.as_pointer()) });
        stable_sort_340(
            ___first,
            ___last,
            FnPtr::<fn(Ptr<Item>, Ptr<Item>) -> bool>::new(Compare_339),
        )
    });
    (if (({ vector_306Impl::operator_index_46(&v.as_pointer(), 0_u64) }).with(|__s| __s.key) == 1) {
        &(0);
    } else {
        ({
            __assert_fail_341(
                Ptr::<i8>::from_string_literal(b"v[0].key == 1"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ vector_306Impl::operator_index_46(&v.as_pointer(), 1_u64) }).with(|__s| __s.key) == 2) {
        &(0);
    } else {
        ({
            __assert_fail_341(
                Ptr::<i8>::from_string_literal(b"v[1].key == 2"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ vector_306Impl::operator_index_46(&v.as_pointer(), 2_u64) }).with(|__s| __s.key) == 3) {
        &(0);
    } else {
        ({
            __assert_fail_341(
                Ptr::<i8>::from_string_literal(b"v[2].key == 3"),
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
pub fn stable_sort_340(
    __first: __normal_iterator_273,
    __last: __normal_iterator_273,
    mut __comp: FnPtr<fn(Ptr<Item>, Ptr<Item>) -> bool>,
) {
    unimplemented!()
}
pub fn __assert_fail_341(
    mut __assertion: Ptr<i8>,
    mut __file: Ptr<i8>,
    mut __line: u32,
    mut __function: Ptr<i8>,
) {
    unimplemented!()
}
pub trait _Iter_comp_iter_275Impl {
    fn operator_call_276(
        &self,
        __it1: __normal_iterator_273,
        __it2: __normal_iterator_273,
    ) -> bool {
        unimplemented!()
    }
    fn operator_call_277(&self, mut __it1: Ptr<Item>, mut __it2: Ptr<Item>) -> bool {
        unimplemented!()
    }
    fn operator_call_278(&self, __it1: __normal_iterator_273, mut __it2: Ptr<Item>) -> bool {
        unimplemented!()
    }
    fn operator_call_279(&self, mut __it1: Ptr<Item>, __it2: __normal_iterator_273) -> bool {
        unimplemented!()
    }
}
impl _Iter_comp_iter_275Impl for Ptr<_Iter_comp_iter_275> {}
pub trait _Iter_comp_val_280Impl {
    fn operator_call_281(&self, __it: __normal_iterator_273, __val: Ptr<Item>) -> bool {
        unimplemented!()
    }
}
impl _Iter_comp_val_280Impl for Ptr<_Iter_comp_val_280> {}
pub trait _Temporary_buffer_286Impl {
    fn size(&self) -> i64 {
        unimplemented!()
    }
    fn requested_size(&self) -> i64 {
        unimplemented!()
    }
    fn begin(&self) -> Ptr<Item> {
        unimplemented!()
    }
    fn end(&self) -> Ptr<Item> {
        unimplemented!()
    }
}
impl _Temporary_buffer_286Impl for Ptr<_Temporary_buffer_286> {}
pub trait _Val_comp_iter_282Impl {
    fn operator_call_283(&self, __val: Ptr<Item>, __it: __normal_iterator_273) -> bool {
        unimplemented!()
    }
    fn operator_call_284(&self, __val: Ptr<Item>, __it: __normal_iterator_273) -> bool {
        unimplemented!()
    }
}
impl _Val_comp_iter_282Impl for Ptr<_Val_comp_iter_282> {}
pub trait _Vector_base_305Impl {
    fn _M_get_Tp_allocator_1(&self) -> Ptr<allocator_297> {
        unimplemented!()
    }
    fn _M_get_Tp_allocator_2(&self) -> Ptr<allocator_297> {
        unimplemented!()
    }
    fn _M_allocate(&self, mut __n: usize) -> Ptr<Item> {
        unimplemented!()
    }
    fn _M_deallocate(&self, mut __p: Ptr<Item>, mut __n: usize) {
        unimplemented!()
    }
}
impl _Vector_base_305Impl for Ptr<_Vector_base_305> {}
pub trait __new_allocator_294Impl {
    fn allocate(&self, mut __n: u64, _a1: Option<AnyPtr>) -> Ptr<Item> {
        unimplemented!()
    }
    fn deallocate(&self, mut __p: Ptr<Item>, mut __n: u64) {
        unimplemented!()
    }
    fn max_size(&self) -> u64 {
        unimplemented!()
    }
    fn construct_295(&self, mut __p: Ptr<Item>, __args: Ptr<Item>) {
        unimplemented!()
    }
}
impl __new_allocator_294Impl for Ptr<__new_allocator_294> {}
pub trait __normal_iterator_273Impl {
    fn operator_deref(&self) -> Ptr<Item> {
        unimplemented!()
    }
    fn operator_inc_6(&self) -> Ptr<__normal_iterator_273> {
        unimplemented!()
    }
    fn operator_dec_8(&self) -> Ptr<__normal_iterator_273> {
        unimplemented!()
    }
    fn operator_add_assign(&self, mut __n: i64) -> Ptr<__normal_iterator_273> {
        unimplemented!()
    }
    fn operator_add(&self, mut __n: i64) -> __normal_iterator_273 {
        unimplemented!()
    }
    fn operator_sub(&self, mut __n: i64) -> __normal_iterator_273 {
        unimplemented!()
    }
    fn base(&self) -> Ptr<Ptr<Item>> {
        unimplemented!()
    }
}
impl __normal_iterator_273Impl for Ptr<__normal_iterator_273> {}
pub trait __normal_iterator_274Impl {
    fn base(&self) -> Ptr<Ptr<Item>> {
        unimplemented!()
    }
}
impl __normal_iterator_274Impl for Ptr<__normal_iterator_274> {}
pub trait initializer_list_285Impl {
    fn size(&self) -> u64 {
        unimplemented!()
    }
    fn begin(&self) -> Ptr<bool> {
        unimplemented!()
    }
}
impl initializer_list_285Impl for Ptr<initializer_list_285> {}
pub trait integral_constant_0Impl {
    fn to_bool(&self) -> bool {
        unimplemented!()
    }
}
impl integral_constant_0Impl for Ptr<integral_constant_0> {}
pub trait pair_253Impl {
    fn copy_assign(&self, __p: Ptr<pair_253>) -> Ptr<pair_253> {
        unimplemented!()
    }
    fn move_assign(&self, __p: Ptr<pair_253>) -> Ptr<pair_253> {
        unimplemented!()
    }
}
impl pair_253Impl for Ptr<pair_253> {}
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
pub trait vector_306Impl {
    fn copy_assign(&self, __x: Ptr<vector_306>) -> Ptr<vector_306> {
        unimplemented!()
    }
    fn move_assign(&self, __x: Ptr<vector_306>) -> Ptr<vector_306> {
        unimplemented!()
    }
    fn begin_26(&self) -> __normal_iterator_273 {
        unimplemented!()
    }
    fn begin_27(&self) -> __normal_iterator_274 {
        unimplemented!()
    }
    fn end_28(&self) -> __normal_iterator_273 {
        unimplemented!()
    }
    fn end_29(&self) -> __normal_iterator_274 {
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
    fn operator_index_46(&self, mut __n: u64) -> Ptr<Item> {
        unimplemented!()
    }
    fn back_53(&self) -> Ptr<Item> {
        unimplemented!()
    }
    fn push_back_58(&self, __x: Ptr<Item>) {
        unimplemented!()
    }
    fn emplace_back_307(&self, __args: Ptr<Item>) -> Ptr<Item> {
        unimplemented!()
    }
}
impl vector_306Impl for Ptr<vector_306> {}
pub fn __cpp2rust_init_globals() {}
