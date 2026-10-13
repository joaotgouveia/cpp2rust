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
pub struct __and__15 {}
impl Clone for __and__15 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__15 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __and__16 {}
impl Clone for __and__16 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__16 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __and__17 {}
impl Clone for __and__17 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__17 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __and__18 {}
impl Clone for __and__18 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__18 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __and__19 {}
impl Clone for __and__19 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__19 {
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
pub struct __not__43 {}
impl Clone for __not__43 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__43 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_44 {}
impl Clone for is_reference_44 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_44 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_45 {}
impl Clone for is_reference_45 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_45 {
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
pub struct is_const_56 {}
impl Clone for is_const_56 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_56 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_const_57 {}
impl Clone for is_const_57 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_57 {
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
pub struct __is_array_unknown_bounds_61 {}
impl Clone for __is_array_unknown_bounds_61 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_61 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_62 {}
impl Clone for __is_array_unknown_bounds_62 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_62 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_63 {}
impl Clone for __is_integral_helper_63 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_63 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_64 {}
impl Clone for __is_integral_helper_64 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_64 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_65 {}
impl Clone for __is_integral_helper_65 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_65 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_66 {}
impl Clone for __is_integral_helper_66 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_66 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_67 {}
impl Clone for __is_integral_helper_67 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_67 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_68 {}
impl Clone for is_integral_68 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_68 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_69 {}
impl Clone for is_integral_69 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_69 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_70 {}
impl Clone for is_integral_70 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_70 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_71 {}
impl Clone for is_integral_71 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_71 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_72 {}
impl Clone for is_integral_72 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_72 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_pointer_73 {}
impl Clone for is_pointer_73 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_pointer_73 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_pointer_74 {}
impl Clone for is_pointer_74 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_pointer_74 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_75 {}
impl Clone for is_same_75 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_75 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_76 {}
impl Clone for is_same_76 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_76 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_77 {}
impl Clone for is_same_77 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_77 {
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
pub struct is_volatile_80 {}
impl Clone for is_volatile_80 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_80 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_volatile_81 {}
impl Clone for is_volatile_81 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_81 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_volatile_82 {}
impl Clone for is_volatile_82 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_82 {
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
pub struct is_constructible_83 {}
impl Clone for is_constructible_83 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_constructible_83 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_constructible_84 {}
impl Clone for is_constructible_84 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_constructible_84 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_constructible_85 {}
impl Clone for is_constructible_85 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_constructible_85 {
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
pub struct is_constructible_87 {}
impl Clone for is_constructible_87 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_constructible_87 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_constructible_88 {}
impl Clone for is_constructible_88 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_constructible_88 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_constructible_89 {}
impl Clone for is_constructible_89 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_constructible_89 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_constructible_90 {}
impl Clone for is_constructible_90 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_constructible_90 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_copy_assignable_91 {}
impl Clone for is_copy_assignable_91 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_copy_assignable_91 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_copy_assignable_92 {}
impl Clone for is_copy_assignable_92 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_copy_assignable_92 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_move_assignable_93 {}
impl Clone for is_move_assignable_93 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_move_assignable_93 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_move_assignable_94 {}
impl Clone for is_move_assignable_94 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_move_assignable_94 {
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
pub struct is_convertible_95 {}
impl Clone for is_convertible_95 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_95 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_convertible_96 {}
impl Clone for is_convertible_96 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_96 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_convertible_97 {}
impl Clone for is_convertible_97 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_97 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_convertible_98 {}
impl Clone for is_convertible_98 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_98 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_convertible_99 {}
impl Clone for is_convertible_99 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_99 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_convertible_100 {}
impl Clone for is_convertible_100 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_100 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_convertible_101 {}
impl Clone for is_convertible_101 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_101 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_convertible_102 {}
impl Clone for is_convertible_102 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_102 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_103 {}
impl Clone for remove_reference_103 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_103 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_104 {}
impl Clone for remove_reference_104 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_104 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __cv_selector_105 {}
impl Clone for __cv_selector_105 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_105 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __cv_selector_106 {}
impl Clone for __cv_selector_106 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_106 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __cv_selector_107 {}
impl Clone for __cv_selector_107 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_107 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __cv_selector_108 {}
impl Clone for __cv_selector_108 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_108 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_109 {}
impl Clone for __match_cv_qualifiers_109 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_109 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_110 {}
impl Clone for __match_cv_qualifiers_110 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_110 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_111 {}
impl Clone for __match_cv_qualifiers_111 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_111 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_112 {}
impl Clone for __match_cv_qualifiers_112 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_112 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_113 {}
impl Clone for __match_cv_qualifiers_113 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_113 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_114 {}
impl Clone for __make_unsigned_114 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_114 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_115 {}
impl Clone for __make_unsigned_115 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_115 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_116 {}
impl Clone for __make_unsigned_116 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_116 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_117 {}
impl Clone for __make_unsigned_selector_117 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_117 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_118 {}
impl Clone for __make_unsigned_selector_118 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_118 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_119 {}
impl Clone for __make_unsigned_selector_119 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_119 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_120 {}
impl Clone for __make_unsigned_selector_120 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_120 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_121 {}
impl Clone for __make_unsigned_selector_121 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_121 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_122 {}
impl Clone for __make_unsigned_selector_122 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_122 {
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
thread_local!(
    pub static std___make_unsigned_selector_base__List_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long____size_127 : Value<usize > = unimplemented!() ;
);
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
thread_local!(
    pub static std___make_unsigned_selector_base__List_unsigned_short__unsigned_int__unsigned_long__unsigned_long_long____size_129 : Value<usize > = unimplemented!() ;
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_130 {}
impl Clone for _List_130 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_130 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static std___make_unsigned_selector_base__List_unsigned_int__unsigned_long__unsigned_long_long____size_131 : Value<usize > = unimplemented!() ;
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_132 {}
impl Clone for _List_132 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_132 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_133 {}
impl Clone for _List_133 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_133 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_134 {}
impl Clone for _List_134 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_134 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_135 {}
impl Clone for __select_135 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_135 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_136 {}
impl Clone for __select_136 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_136 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_137 {}
impl Clone for __select_137 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_137 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_138 {}
impl Clone for __select_138 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_138 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_139 {}
impl Clone for __select_139 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_139 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_140 {}
impl Clone for __make_signed_140 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_140 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_141 {}
impl Clone for __make_signed_141 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_141 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_142 {}
impl Clone for __make_signed_selector_142 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_142 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_143 {}
impl Clone for __make_signed_selector_143 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_143 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_144 {}
impl Clone for __make_signed_selector_144 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_144 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_145 {}
impl Clone for __make_signed_selector_145 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_145 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_146 {}
impl Clone for __make_signed_selector_146 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_146 {
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
pub struct common_type_147 {}
impl Clone for common_type_147 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for common_type_147 {
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
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(8)]
pub struct pair_148 {
    #[offset(0)]
    pub first: i32,
    #[offset(4)]
    pub second: i32,
}
impl Clone for pair_148 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pair_148 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl pair_148 {
    pub fn copy_from(_a0: Ptr<pair_148>) -> Self {
        unimplemented!()
    }
}
impl pair_148 {
    pub fn move_from(_a0: Ptr<pair_148>) -> Self {
        unimplemented!()
    }
}
impl pair_148 {
    pub fn new(__x: Ptr<i32>, __y: Ptr<i32>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct pair_149 {
    #[offset(0)]
    pub first: i32,
    #[offset(8)]
    pub second: i64,
}
impl Clone for pair_149 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pair_149 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl pair_149 {
    pub fn copy_from(_a0: Ptr<pair_149>) -> Self {
        unimplemented!()
    }
}
impl pair_149 {
    pub fn move_from(_a0: Ptr<pair_149>) -> Self {
        unimplemented!()
    }
}
impl pair_149 {
    pub fn new(__x: Ptr<i32>, __y: Ptr<i64>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _PCC_150 {}
impl Clone for _PCC_150 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _PCC_150 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _PCC_151 {}
impl Clone for _PCC_151 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _PCC_151 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __pair_base_152 {}
impl Clone for __pair_base_152 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __pair_base_152 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __pair_base_153 {}
impl Clone for __pair_base_153 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __pair_base_153 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub fn inc_154(mut p: Ptr<i32>) {
    {
        p.with_mut(|__v| *__v = *__v + 1)
    };
}
pub fn add_155(mut p: Ptr<i32>, mut n: i32) {
    {
        let __rhs = { n };
        p.with_mut(|__v| *__v = *__v + __rhs)
    };
}
pub fn twice_156(mut n: i32) -> i32 {
    return (n * 2);
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Access_157 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct S {
    #[offset(0)]
    pub base: i32,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Box_ {
    #[offset(0)]
    pub v: i32,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let s: Value<S> = Rc::new(RefCell::new(S { base: 100 }));
    (if (({ SImpl::width_158(&s.as_pointer(), 3) }) == 103) {
        &(0);
    } else {
        ({
            __assert_fail_164(
                Ptr::<i8>::from_string_literal(b"s.width<char>(3) == 103"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ SImpl::width_159(&s.as_pointer(), 3) }) == 112) {
        &(0);
    } else {
        ({
            __assert_fail_164(
                Ptr::<i8>::from_string_literal(b"s.width<int>(3) == 112"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ SImpl::scale_160(&s.as_pointer(), 5) }) == 110) {
        &(0);
    } else {
        ({
            __assert_fail_164(
                Ptr::<i8>::from_string_literal(b"s.scale<2>(5) == 110"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ SImpl::scale_161(&s.as_pointer(), 5) }) == 115) {
        &(0);
    } else {
        ({
            __assert_fail_164(
                Ptr::<i8>::from_string_literal(b"s.scale<3>(5) == 115"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ SImpl::count_162(&s.as_pointer(), 1) }) == 101) {
        &(0);
    } else {
        ({
            __assert_fail_164(
                Ptr::<i8>::from_string_literal(b"s.count<>(1) == 101"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ SImpl::count_163(&s.as_pointer(), 1) }) == 103) {
        &(0);
    } else {
        ({
            __assert_fail_164(
                Ptr::<i8>::from_string_literal(b"(s.count<int, long>(1) == 103)"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ SImpl::plain_4(&s.as_pointer(), 1) }) == 101) {
        &(0);
    } else {
        ({
            __assert_fail_164(
                Ptr::<i8>::from_string_literal(b"s.plain(1) == 101"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ SImpl::plain_5(&s.as_pointer(), 1_i64) }) == 102) {
        &(0);
    } else {
        ({
            __assert_fail_164(
                Ptr::<i8>::from_string_literal(b"s.plain(1L) == 102"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let y: Value<i32> = Rc::new(RefCell::new(1));
    (if (({ SImpl::take_6(&s.as_pointer(), y.as_pointer()) }) == 102) {
        &(0);
    } else {
        ({
            __assert_fail_164(
                Ptr::<i8>::from_string_literal(b"s.take(y) == 102"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({
        let _x: Value<i32> = Rc::new(RefCell::new(5));
        SImpl::take_7(&s.as_pointer(), _x.as_pointer())
    }) == 107)
    {
        &(0);
    } else {
        ({
            __assert_fail_164(
                Ptr::<i8>::from_string_literal(b"s.take(5) == 107"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({
        SImpl::pick_8(&s.as_pointer(), {
            let __tmp_0: Value<i32> = Rc::new(RefCell::new(1));
            let __tmp_1: Value<i32> = Rc::new(RefCell::new(2));
            pair_148::new({ __tmp_0.as_pointer() }, { __tmp_1.as_pointer() })
        })
    }) == 101)
    {
        &(0);
    } else {
        ({
            __assert_fail_164(
                Ptr::<i8>::from_string_literal(b"s.pick(std::pair<int, int>(1, 2)) == 101"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({
        SImpl::pick_9(&s.as_pointer(), {
            let __tmp_2: Value<i32> = Rc::new(RefCell::new(1));
            let __tmp_3: Value<i64> = Rc::new(RefCell::new(2_i64));
            pair_149::new({ __tmp_2.as_pointer() }, { __tmp_3.as_pointer() })
        })
    }) == 102)
    {
        &(0);
    } else {
        ({
            __assert_fail_164(
                Ptr::<i8>::from_string_literal(b"s.pick(std::pair<int, long>(1, 2L)) == 102"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ SImpl::apply_10(&s.as_pointer(), FnPtr::<fn(Ptr<i32>)>::new(inc_154), 1) }) == 102) {
        &(0);
    } else {
        ({
            __assert_fail_164(
                Ptr::<i8>::from_string_literal(b"s.apply(inc, 1) == 102"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ SImpl::apply_11(&s.as_pointer(), FnPtr::<fn(Ptr<i32>, i32)>::new(add_155), 1) }) == 111)
    {
        &(0);
    } else {
        ({
            __assert_fail_164(
                Ptr::<i8>::from_string_literal(b"s.apply(add, 1) == 111"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ SImpl::apply_12(&s.as_pointer(), FnPtr::<fn(i32) -> i32>::new(twice_156), 3) }) == 106)
    {
        &(0);
    } else {
        ({
            __assert_fail_164(
                Ptr::<i8>::from_string_literal(b"s.apply(twice, 3) == 106"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let c: Value<i32> = Rc::new(RefCell::new(3));
    (if (({
        let _p: Value<pair_149> = Rc::new(RefCell::new({
            let __tmp_4: Value<i32> = Rc::new(RefCell::new(1));
            let __tmp_5: Value<i64> = Rc::new(RefCell::new(2_i64));
            pair_149::new({ __tmp_4.as_pointer() }, { __tmp_5.as_pointer() })
        }));
        SImpl::combine_13(
            &s.as_pointer(),
            _p.as_pointer(),
            FnPtr::<fn(i32) -> i32>::new(twice_156),
            (c.as_pointer()),
            4_u64,
        )
    }) == 112)
    {
        &(0);
    } else {
        ({
            __assert_fail_164(
                Ptr::<i8>::from_string_literal(
                    b"s.combine(std::pair<int, long>(1, 2L), twice, &c, 4UL) == 112",
                ),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let z: Value<i32> = Rc::new(RefCell::new(1));
    (if (({
        let _p: Value<pair_148> = Rc::new(RefCell::new({
            let __tmp_6: Value<i32> = Rc::new(RefCell::new(1));
            let __tmp_7: Value<i32> = Rc::new(RefCell::new(2));
            pair_148::new({ __tmp_6.as_pointer() }, { __tmp_7.as_pointer() })
        }));
        SImpl::combine_14(
            &s.as_pointer(),
            _p.as_pointer(),
            FnPtr::<fn(Ptr<i32>, i32)>::new(add_155),
            (z.as_pointer()),
            5_u64,
        )
    }) == 107)
    {
        &(0);
    } else {
        ({
            __assert_fail_164(
                Ptr::<i8>::from_string_literal(
                    b"s.combine(std::pair<int, int>(1, 2), add, &z, 5UL) == 107",
                ),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ((*z.borrow()) == 6) {
        &(0);
    } else {
        ({
            __assert_fail_164(
                Ptr::<i8>::from_string_literal(b"z == 6"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let a: Value<Access_157> = Rc::new(RefCell::new(<Access_157>::default()));
    let mut cs: Ptr<S> = (s.as_pointer());
    let cr: Ptr<S> = s.as_pointer();
    (if (({ Access_157Impl::get_1(&a.as_pointer(), (s.as_pointer())) }) == 100) {
        &(0);
    } else {
        ({
            __assert_fail_164(
                Ptr::<i8>::from_string_literal(b"a.get(&s) == 100"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ Access_157Impl::get_2(&a.as_pointer(), (cs).clone()) }) == 101) {
        &(0);
    } else {
        ({
            __assert_fail_164(
                Ptr::<i8>::from_string_literal(b"a.get(cs) == 101"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ Access_157Impl::ref_3(&a.as_pointer(), s.as_pointer()) }) == 102) {
        &(0);
    } else {
        ({
            __assert_fail_164(
                Ptr::<i8>::from_string_literal(b"a.ref(s) == 102"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({
        let _r: Ptr<S> = (cr).clone();
        Access_157Impl::ref_4(&a.as_pointer(), _r)
    }) == 103)
    {
        &(0);
    } else {
        ({
            __assert_fail_164(
                Ptr::<i8>::from_string_literal(b"a.ref(cr) == 103"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut b: Box_ = Box_ { v: 4 };
    (if (b.v == 4) {
        &(0);
    } else {
        ({
            __assert_fail_164(
                Ptr::<i8>::from_string_literal(b"b.v == 4"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    return 0;
}
pub fn __assert_fail_164(
    mut __assertion: Ptr<i8>,
    mut __file: Ptr<i8>,
    mut __line: u32,
    mut __function: Ptr<i8>,
) {
    unimplemented!()
}
pub trait Access_157Impl {
    fn get_1(&self, p: Ptr<S>) -> i32;
    fn get_2(&self, p: Ptr<S>) -> i32;
    fn ref_3(&self, r: Ptr<S>) -> i32;
    fn ref_4(&self, r: Ptr<S>) -> i32;
}
impl Access_157Impl for Ptr<Access_157> {
    fn get_1(&self, mut p: Ptr<S>) -> i32 {
        return p.with(|__s| __s.base);
    }
    fn get_2(&self, mut p: Ptr<S>) -> i32 {
        return (p.with(|__s| __s.base) + 1);
    }
    fn ref_3(&self, r: Ptr<S>) -> i32 {
        return (r.with(|__s| __s.base) + 2);
    }
    fn ref_4(&self, r: Ptr<S>) -> i32 {
        return (r.with(|__s| __s.base) + 3);
    }
}
pub trait SImpl {
    fn plain_4(&self, x: i32) -> i32;
    fn plain_5(&self, x: i64) -> i32;
    fn take_6(&self, x: Ptr<i32>) -> i32;
    fn take_7(&self, x: Ptr<i32>) -> i32;
    fn pick_8(&self, p: pair_148) -> i32;
    fn pick_9(&self, p: pair_149) -> i32;
    fn apply_10(&self, f: FnPtr<fn(Ptr<i32>)>, x: i32) -> i32;
    fn apply_11(&self, f: FnPtr<fn(Ptr<i32>, i32)>, x: i32) -> i32;
    fn apply_12(&self, f: FnPtr<fn(i32) -> i32>, x: i32) -> i32;
    fn combine_13(&self, p: Ptr<pair_149>, f: FnPtr<fn(i32) -> i32>, q: Ptr<i32>, n: u64) -> i32;
    fn combine_14(&self, p: Ptr<pair_148>, f: FnPtr<fn(Ptr<i32>, i32)>, q: Ptr<i32>, n: u64)
    -> i32;
    fn width_158(&self, x: i32) -> i32;
    fn width_159(&self, x: i32) -> i32;
    fn scale_160(&self, x: i32) -> i32;
    fn scale_161(&self, x: i32) -> i32;
    fn count_162(&self, x: i32) -> i32;
    fn count_163(&self, x: i32) -> i32;
}
impl SImpl for Ptr<S> {
    fn plain_4(&self, mut x: i32) -> i32 {
        return ((*self).with(|__s| __s.base) + x);
    }
    fn plain_5(&self, mut x: i64) -> i32 {
        return (((*self).with(|__s| __s.base) + (x as i32)) + 1);
    }
    fn take_6(&self, x: Ptr<i32>) -> i32 {
        return (({ (*self).with(|__s| __s.base) } + { (x.read()) }) + 1);
    }
    fn take_7(&self, x: Ptr<i32>) -> i32 {
        return (({ (*self).with(|__s| __s.base) } + { (x.read()) }) + 2);
    }
    fn pick_8(&self, p: pair_148) -> i32 {
        let p: Value<pair_148> = Rc::new(RefCell::new(p));
        return ((*self).with(|__s| __s.base) + { (*p.borrow()).first });
    }
    fn pick_9(&self, p: pair_149) -> i32 {
        let p: Value<pair_149> = Rc::new(RefCell::new(p));
        return ((*self).with(|__s| __s.base) + ({ (*p.borrow()).second } as i32));
    }
    fn apply_10(&self, mut f: FnPtr<fn(Ptr<i32>)>, x: i32) -> i32 {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        ({ f.call((x.as_pointer())) });
        return ((*self).with(|__s| __s.base) + (*x.borrow()));
    }
    fn apply_11(&self, mut f: FnPtr<fn(Ptr<i32>, i32)>, x: i32) -> i32 {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        ({ f.call((x.as_pointer()), 10) });
        return ((*self).with(|__s| __s.base) + (*x.borrow()));
    }
    fn apply_12(&self, mut f: FnPtr<fn(i32) -> i32>, mut x: i32) -> i32 {
        return ({ (*self).with(|__s| __s.base) } + { ({ f.call(x) }) });
    }
    fn combine_13(
        &self,
        p: Ptr<pair_149>,
        mut f: FnPtr<fn(i32) -> i32>,
        mut q: Ptr<i32>,
        mut n: u64,
    ) -> i32 {
        return ({
            ({ ({ (*self).with(|__s| __s.base) } + { (p.with(|__s| __s.second) as i32) }) } + {
                ({ f.call((q.read())) })
            })
        } + { (n as i32) });
    }
    fn combine_14(
        &self,
        p: Ptr<pair_148>,
        mut f: FnPtr<fn(Ptr<i32>, i32)>,
        mut q: Ptr<i32>,
        mut n: u64,
    ) -> i32 {
        ({
            let _arg0: Ptr<i32> = (q).clone();
            let _arg1: i32 = (n as i32);
            f.call(_arg0, _arg1)
        });
        return ({ ({ (*self).with(|__s| __s.base) } + { p.with(|__s| __s.first) }) } + {
            (q.read())
        });
    }
    fn width_158(&self, mut x: i32) -> i32 {
        return ((*self).with(|__s| __s.base) + (x * (1usize as i32)));
    }
    fn width_159(&self, mut x: i32) -> i32 {
        return ((*self).with(|__s| __s.base) + (x * (4usize as i32)));
    }
    fn scale_160(&self, mut x: i32) -> i32 {
        return ((*self).with(|__s| __s.base) + (x * 2));
    }
    fn scale_161(&self, mut x: i32) -> i32 {
        return ((*self).with(|__s| __s.base) + (x * 3));
    }
    fn count_162(&self, mut x: i32) -> i32 {
        return (((*self).with(|__s| __s.base) + x) + (0 as i32));
    }
    fn count_163(&self, mut x: i32) -> i32 {
        return (((*self).with(|__s| __s.base) + x) + (2 as i32));
    }
}
pub trait integral_constant_0Impl {
    fn to_bool(&self) -> bool {
        unimplemented!()
    }
}
impl integral_constant_0Impl for Ptr<integral_constant_0> {}
pub trait pair_148Impl {
    fn copy_assign(&self, __p: Ptr<pair_148>) -> Ptr<pair_148> {
        unimplemented!()
    }
    fn move_assign(&self, __p: Ptr<pair_148>) -> Ptr<pair_148> {
        unimplemented!()
    }
}
impl pair_148Impl for Ptr<pair_148> {}
pub trait pair_149Impl {
    fn copy_assign(&self, __p: Ptr<pair_149>) -> Ptr<pair_149> {
        unimplemented!()
    }
    fn move_assign(&self, __p: Ptr<pair_149>) -> Ptr<pair_149> {
        unimplemented!()
    }
}
impl pair_149Impl for Ptr<pair_149> {}
pub fn __cpp2rust_init_globals() {}
