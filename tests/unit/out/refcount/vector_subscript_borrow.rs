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
pub struct __or__12 {}
impl Clone for __or__12 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__12 {
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
pub struct is_function_28 {}
impl Clone for is_function_28 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_28 {
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
pub struct is_const_43 {}
impl Clone for is_const_43 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_43 {
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
pub struct is_arithmetic_60 {}
impl Clone for is_arithmetic_60 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_arithmetic_60 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_arithmetic_61 {}
impl Clone for is_arithmetic_61 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_arithmetic_61 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_scalar_62 {}
impl Clone for is_scalar_62 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_scalar_62 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_scalar_63 {}
impl Clone for is_scalar_63 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_scalar_63 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_64 {}
impl Clone for is_same_64 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_64 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_65 {}
impl Clone for is_same_65 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_65 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_volatile_66 {}
impl Clone for is_volatile_66 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_66 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_volatile_67 {}
impl Clone for is_volatile_67 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_67 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_volatile_68 {}
impl Clone for is_volatile_68 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_68 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_volatile_69 {}
impl Clone for is_volatile_69 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_69 {
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
pub struct __is_destructible_safe_71 {}
impl Clone for __is_destructible_safe_71 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_destructible_safe_71 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_destructible_safe_72 {}
impl Clone for __is_destructible_safe_72 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_destructible_safe_72 {
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
pub struct is_copy_assignable_73 {}
impl Clone for is_copy_assignable_73 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_copy_assignable_73 {
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
pub struct is_trivially_destructible_74 {}
impl Clone for is_trivially_destructible_74 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_trivially_destructible_74 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_trivially_destructible_75 {}
impl Clone for is_trivially_destructible_75 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_trivially_destructible_75 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_76 {}
impl Clone for remove_reference_76 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_76 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __cv_selector_77 {}
impl Clone for __cv_selector_77 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_77 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __cv_selector_78 {}
impl Clone for __cv_selector_78 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_78 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __cv_selector_79 {}
impl Clone for __cv_selector_79 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_79 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __cv_selector_80 {}
impl Clone for __cv_selector_80 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_80 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_81 {}
impl Clone for __match_cv_qualifiers_81 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_81 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_82 {}
impl Clone for __match_cv_qualifiers_82 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_82 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_83 {}
impl Clone for __match_cv_qualifiers_83 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_83 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_84 {}
impl Clone for __match_cv_qualifiers_84 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_84 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_85 {}
impl Clone for __match_cv_qualifiers_85 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_85 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_86 {}
impl Clone for __make_unsigned_86 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_86 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_87 {}
impl Clone for __make_unsigned_87 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_87 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_88 {}
impl Clone for __make_unsigned_88 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_88 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_89 {}
impl Clone for __make_unsigned_selector_89 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_89 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_90 {}
impl Clone for __make_unsigned_selector_90 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_90 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_91 {}
impl Clone for __make_unsigned_selector_91 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_91 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_92 {}
impl Clone for __make_unsigned_selector_92 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_92 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_93 {}
impl Clone for __make_unsigned_selector_93 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_93 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_94 {}
impl Clone for __make_unsigned_selector_94 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_94 {
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
thread_local!(
    pub static std___make_unsigned_selector_base__List_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long____size_99 : Value<usize > = unimplemented!() ;
);
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
thread_local!(
    pub static std___make_unsigned_selector_base__List_unsigned_short__unsigned_int__unsigned_long__unsigned_long_long____size_101 : Value<usize > = unimplemented!() ;
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_102 {}
impl Clone for _List_102 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_102 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static std___make_unsigned_selector_base__List_unsigned_int__unsigned_long__unsigned_long_long____size_103 : Value<usize > = unimplemented!() ;
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_104 {}
impl Clone for _List_104 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_104 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_105 {}
impl Clone for _List_105 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_105 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_106 {}
impl Clone for _List_106 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_106 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_107 {}
impl Clone for __select_107 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_107 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_108 {}
impl Clone for __select_108 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_108 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_109 {}
impl Clone for __select_109 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_109 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_110 {}
impl Clone for __select_110 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_110 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_111 {}
impl Clone for __select_111 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_111 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_112 {}
impl Clone for __make_signed_112 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_112 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_113 {}
impl Clone for __make_signed_113 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_113 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_114 {}
impl Clone for __make_signed_selector_114 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_114 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_115 {}
impl Clone for __make_signed_selector_115 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_115 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_116 {}
impl Clone for __make_signed_selector_116 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_116 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_117 {}
impl Clone for __make_signed_selector_117 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_117 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_118 {}
impl Clone for __make_signed_selector_118 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_118 {
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
pub struct common_type_119 {}
impl Clone for common_type_119 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for common_type_119 {
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
pub struct __are_same_120 {}
impl Clone for __are_same_120 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_120 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_121 = u32;
pub const anon_121___value: anon_121 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_122 {}
impl Clone for __are_same_122 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_122 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_123 = u32;
pub const anon_123___value: anon_123 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_124 {}
impl Clone for __are_same_124 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_124 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_125 = u32;
pub const anon_125___value: anon_125 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_126 {}
impl Clone for __are_same_126 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_126 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_127 = u32;
pub const anon_127___value: anon_127 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_128 {}
impl Clone for __are_same_128 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_128 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_129 = u32;
pub const anon_129___value: anon_129 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_130 {}
impl Clone for __are_same_130 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_130 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_131 = u32;
pub const anon_131___value: anon_131 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_132 {}
impl Clone for __are_same_132 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_132 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_133 = u32;
pub const anon_133___value: anon_133 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_134 {}
impl Clone for __is_integer_134 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_134 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_135 = u32;
pub const anon_135___value: anon_135 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_136 {}
impl Clone for __is_integer_136 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_136 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_137 = u32;
pub const anon_137___value: anon_137 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_138 {}
impl Clone for __is_integer_138 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_138 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_139 = u32;
pub const anon_139___value: anon_139 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_140 {}
impl Clone for __is_integer_140 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_140 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_141 = u32;
pub const anon_141___value: anon_141 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_142 {}
impl Clone for __is_integer_142 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_142 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_143 = u32;
pub const anon_143___value: anon_143 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_144 {}
impl Clone for __is_integer_144 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_144 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_145 = u32;
pub const anon_145___value: anon_145 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_byte_146 {}
impl Clone for __is_byte_146 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_byte_146 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_147 = u32;
pub const anon_147___value: anon_147 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_byte_148 {}
impl Clone for __is_byte_148 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_byte_148 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_149 = u32;
pub const anon_149___value: anon_149 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __memcpyable_integer_150 {}
impl Clone for __memcpyable_integer_150 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __memcpyable_integer_150 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __memcpyable_integer_151 {}
impl Clone for __memcpyable_integer_151 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __memcpyable_integer_151 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_152 = u32;
pub const anon_152___width: anon_152 = 32;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __enable_if_153 {}
impl Clone for __enable_if_153 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __enable_if_153 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __conditional_type_154 {}
impl Clone for __conditional_type_154 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __conditional_type_154 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_nonstrict_155 {}
impl Clone for __is_integer_nonstrict_155 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_155 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_156 = u32;
pub const anon_156___width: anon_156 = 64;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_nonstrict_157 {}
impl Clone for __is_integer_nonstrict_157 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_157 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_158 = u32;
pub const anon_158___width: anon_158 = 64;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_integer_159 {}
impl Clone for __numeric_traits_integer_159 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_159 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_long____is_signed_160: Value<bool> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_long____digits_161: Value<i32> = unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_long____max_162: Value<i64> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_integer_163 {}
impl Clone for __numeric_traits_integer_163 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_163 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_long____is_signed_164: Value<bool> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_long____digits_165: Value<i32> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_long____max_166: Value<u64> =
        unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_floating_167 {}
impl Clone for __numeric_traits_floating_167 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_167 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_floating_168 {}
impl Clone for __numeric_traits_floating_168 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_168 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_floating_169 {}
impl Clone for __numeric_traits_floating_169 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_169 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_170 {}
impl Clone for __numeric_traits_170 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_170 {
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
    pub static piecewise_construct_171: Value<std_piecewise_construct_t> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct tuple_172 {}
impl Clone for tuple_172 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for tuple_172 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl tuple_172 {}
impl tuple_172 {}
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
pub struct iterator_173 {}
impl Clone for iterator_173 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_173 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_174 {}
impl Clone for iterator_174 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_174 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl iterator_174 {}
impl iterator_174 {}
impl iterator_174 {}
impl iterator_174 {
    pub fn copy_assign(&self, _a0: Ptr<iterator_174>) -> Ptr<iterator_174> {
        unimplemented!()
    }
}
impl iterator_174 {
    pub fn move_assign(&self, _a0: Ptr<iterator_174>) -> Ptr<iterator_174> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_175 {}
impl Clone for iterator_175 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_175 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl iterator_175 {
    pub fn copy_assign(&self, _a0: Ptr<iterator_175>) -> Ptr<iterator_175> {
        unimplemented!()
    }
}
impl iterator_175 {
    pub fn move_assign(&self, _a0: Ptr<iterator_175>) -> Ptr<iterator_175> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_176 {}
impl Clone for iterator_176 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_176 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl iterator_176 {
    pub fn copy_assign(&self, _a0: Ptr<iterator_176>) -> Ptr<iterator_176> {
        unimplemented!()
    }
}
impl iterator_176 {
    pub fn move_assign(&self, _a0: Ptr<iterator_176>) -> Ptr<iterator_176> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_177 {}
impl Clone for iterator_traits_177 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_177 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_178 {}
impl Clone for iterator_traits_178 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_178 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_179 {}
impl Clone for iterator_traits_179 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_179 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_180 {}
impl Clone for iterator_traits_180 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_180 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_181 {}
impl Clone for iterator_traits_181 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_181 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __iterator_traits_182 {}
impl Clone for __iterator_traits_182 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __iterator_traits_182 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __iterator_traits_183 {}
impl Clone for __iterator_traits_183 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __iterator_traits_183 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __iterator_traits_184 {}
impl Clone for __iterator_traits_184 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __iterator_traits_184 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct reverse_iterator_185 {}
impl Clone for reverse_iterator_185 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for reverse_iterator_185 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_185 {
    pub fn copy_from(__x: Ptr<reverse_iterator_185>) -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_185 {
    pub fn copy_assign(&self, _a0: Ptr<reverse_iterator_185>) -> Ptr<reverse_iterator_185> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct reverse_iterator_186 {}
impl Clone for reverse_iterator_186 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for reverse_iterator_186 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_186 {
    pub fn copy_from(__x: Ptr<reverse_iterator_186>) -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_186 {
    pub fn copy_assign(&self, _a0: Ptr<reverse_iterator_186>) -> Ptr<reverse_iterator_186> {
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
pub struct __new_allocator_187 {}
impl Clone for __new_allocator_187 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_187 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_187 {
    pub fn copy_from(_a0: Ptr<__new_allocator_187>) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_187 {
    pub fn copy_assign(&self, _a0: Ptr<__new_allocator_187>) -> Ptr<__new_allocator_187> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __new_allocator_188 {}
impl Clone for __new_allocator_188 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_188 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_188 {
    pub fn copy_from(_a0: Ptr<__new_allocator_188>) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_188 {
    pub fn copy_assign(&self, _a0: Ptr<__new_allocator_188>) -> Ptr<__new_allocator_188> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __new_allocator_189 {}
impl Clone for __new_allocator_189 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_189 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_189 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_189 {
    pub fn copy_from(_a0: Ptr<__new_allocator_189>) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_189 {
    pub fn copy_assign(&self, _a0: Ptr<__new_allocator_189>) -> Ptr<__new_allocator_189> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __new_allocator_190 {}
impl Clone for __new_allocator_190 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_190 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_190 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_190 {
    pub fn copy_from(_a0: Ptr<__new_allocator_190>) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_190 {
    pub fn copy_assign(&self, _a0: Ptr<__new_allocator_190>) -> Ptr<__new_allocator_190> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_191 {}
impl Clone for allocator_191 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_191 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_191 {}
impl allocator_191 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_192 {}
impl Clone for allocator_192 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_192 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_192 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl allocator_192 {
    pub fn copy_from(__a: Ptr<allocator_192>) -> Self {
        unimplemented!()
    }
}
impl allocator_192 {
    pub fn copy_assign(&self, _a0: Ptr<allocator_192>) -> Ptr<allocator_192> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_193 {}
impl Clone for allocator_193 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_193 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_193 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl allocator_193 {
    pub fn copy_from(__a: Ptr<allocator_193>) -> Self {
        unimplemented!()
    }
}
impl allocator_193 {
    pub fn copy_assign(&self, _a0: Ptr<allocator_193>) -> Ptr<allocator_193> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_traits_194 {}
impl Clone for allocator_traits_194 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_traits_194 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_traits_194 {
    pub fn allocate_1(__a: Ptr<allocator_192>, mut __n: u64) -> Ptr<i32> {
        unimplemented!()
    }
}
impl allocator_traits_194 {
    pub fn deallocate(__a: Ptr<allocator_192>, mut __p: Ptr<i32>, mut __n: u64) {
        unimplemented!()
    }
}
impl allocator_traits_194 {
    pub fn max_size(__a: Ptr<allocator_192>) -> u64 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_traits_195 {}
impl Clone for allocator_traits_195 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_traits_195 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_traits_195 {
    pub fn allocate_1(__a: Ptr<allocator_193>, mut __n: u64) -> Ptr<u32> {
        unimplemented!()
    }
}
impl allocator_traits_195 {
    pub fn deallocate(__a: Ptr<allocator_193>, mut __p: Ptr<u32>, mut __n: u64) {
        unimplemented!()
    }
}
impl allocator_traits_195 {
    pub fn max_size(__a: Ptr<allocator_193>) -> u64 {
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
pub struct __alloc_traits_197 {}
impl Clone for __alloc_traits_197 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_197 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct rebind_198 {}
impl Clone for rebind_198 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_198 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __alloc_traits_199 {}
impl Clone for __alloc_traits_199 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_199 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct rebind_200 {}
impl Clone for rebind_200 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_200 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct _UninitDestroyGuard_201 {
    #[offset(0)]
    #[byte_size(8)]
    pub _M_first: Ptr<i32>,
    #[offset(8)]
    #[byte_size(8)]
    pub _M_cur: Ptr<Ptr<i32>>,
}
impl Clone for _UninitDestroyGuard_201 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _UninitDestroyGuard_201 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _UninitDestroyGuard_201 {
    pub fn new(__first: Ptr<Ptr<i32>>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __uninitialized_default_n_1_202 {}
impl Clone for __uninitialized_default_n_1_202 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __uninitialized_default_n_1_202 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __uninitialized_default_n_1_202 {
    pub fn __uninit_default_n_203(mut __first: Ptr<u32>, mut __n: u64) -> Ptr<u32> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct initializer_list_204 {}
impl Clone for initializer_list_204 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for initializer_list_204 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct initializer_list_205 {}
impl Clone for initializer_list_205 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for initializer_list_205 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl initializer_list_205 {}
impl initializer_list_205 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct initializer_list_206 {}
impl Clone for initializer_list_206 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for initializer_list_206 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl initializer_list_206 {}
impl initializer_list_206 {}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(24)]
pub struct _Vector_base_207 {
    #[offset(0)]
    #[byte_size(24)]
    pub _M_impl: std__Vector_base_int__std_allocator_int____Vector_impl,
}
impl Clone for _Vector_base_207 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Vector_base_207 {
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
    pub fn new_1(__a: Ptr<allocator_192>) -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_int__std_allocator_int____Vector_impl {
    pub fn move_from(__x: Ptr<std__Vector_base_int__std_allocator_int____Vector_impl>) -> Self {
        unimplemented!()
    }
}
impl _Vector_base_207 {
    pub fn new_1(mut __n: usize, __a: Ptr<allocator_192>) -> Self {
        unimplemented!()
    }
}
impl _Vector_base_207 {
    pub fn move_from(_a0: Ptr<_Vector_base_207>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(24)]
pub struct _Vector_base_208 {
    #[offset(0)]
    #[byte_size(24)]
    pub _M_impl: std__Vector_base_unsigned_int__std_allocator_unsigned_int____Vector_impl,
}
impl Clone for _Vector_base_208 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Vector_base_208 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(24)]
pub struct std__Vector_base_unsigned_int__std_allocator_unsigned_int____Vector_impl_data {
    #[offset(0)]
    #[byte_size(8)]
    pub _M_start: Ptr<u32>,
    #[offset(8)]
    #[byte_size(8)]
    pub _M_finish: Ptr<u32>,
    #[offset(16)]
    #[byte_size(8)]
    pub _M_end_of_storage: Ptr<u32>,
}
impl Clone for std__Vector_base_unsigned_int__std_allocator_unsigned_int____Vector_impl_data {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std__Vector_base_unsigned_int__std_allocator_unsigned_int____Vector_impl_data {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_unsigned_int__std_allocator_unsigned_int____Vector_impl_data {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_unsigned_int__std_allocator_unsigned_int____Vector_impl_data {
    pub fn move_from(
        __x: Ptr<std__Vector_base_unsigned_int__std_allocator_unsigned_int____Vector_impl_data>,
    ) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(24)]
pub struct std__Vector_base_unsigned_int__std_allocator_unsigned_int____Vector_impl {}
impl Clone for std__Vector_base_unsigned_int__std_allocator_unsigned_int____Vector_impl {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std__Vector_base_unsigned_int__std_allocator_unsigned_int____Vector_impl {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_unsigned_int__std_allocator_unsigned_int____Vector_impl {
    pub fn new_1(__a: Ptr<allocator_193>) -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_unsigned_int__std_allocator_unsigned_int____Vector_impl {
    pub fn move_from(
        __x: Ptr<std__Vector_base_unsigned_int__std_allocator_unsigned_int____Vector_impl>,
    ) -> Self {
        unimplemented!()
    }
}
impl _Vector_base_208 {
    pub fn new_1(mut __n: usize, __a: Ptr<allocator_193>) -> Self {
        unimplemented!()
    }
}
impl _Vector_base_208 {
    pub fn move_from(_a0: Ptr<_Vector_base_208>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(24)]
pub struct vector_209 {}
impl Clone for vector_209 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for vector_209 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl vector_209 {
    pub fn new_1(mut __n: u64, __value: Ptr<i32>, __a: Option<Ptr<allocator_192>>) -> Self {
        unimplemented!()
    }
}
impl vector_209 {
    pub fn copy_from(__x: Ptr<vector_209>) -> Self {
        unimplemented!()
    }
}
impl vector_209 {
    pub fn move_from(_a0: Ptr<vector_209>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(24)]
pub struct vector_210 {}
impl Clone for vector_210 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for vector_210 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl vector_210 {
    pub fn new_1(mut __n: u64, __a: Option<Ptr<allocator_193>>) -> Self {
        unimplemented!()
    }
}
impl vector_210 {
    pub fn copy_from(__x: Ptr<vector_210>) -> Self {
        unimplemented!()
    }
}
impl vector_210 {
    pub fn move_from(_a0: Ptr<vector_210>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_211 {}
impl Clone for __hash_base_211 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_211 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_212 {}
impl Clone for __hash_base_212 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_212 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_213 {}
impl Clone for __hash_base_213 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_213 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_214 {}
impl Clone for __hash_base_214 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_214 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_215 {}
impl Clone for __hash_base_215 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_215 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_216 {}
impl Clone for __hash_base_216 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_216 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_217 {}
impl Clone for __hash_base_217 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_217 {
    fn default() -> Self {
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
pub struct hash_232 {}
impl Clone for hash_232 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for hash_232 {
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
    pub fn hash_233(__val: Ptr<f32>) -> usize {
        unimplemented!()
    }
}
impl std__Hash_impl {
    pub fn hash_234(__val: Ptr<f64>) -> usize {
        unimplemented!()
    }
}
pub type anon_235 = u32;
pub const anon_235__S_word_bit: anon_235 = 64;
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
pub struct binary_function_236 {}
impl Clone for binary_function_236 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for binary_function_236 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct greater_237 {}
impl Clone for greater_237 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for greater_237 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct less_238 {}
impl Clone for less_238 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for less_238 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct greater_equal_239 {}
impl Clone for greater_equal_239 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for greater_equal_239 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct less_equal_240 {}
impl Clone for less_equal_240 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for less_equal_240 {
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
    pub static allocator_arg_241: Value<std_allocator_arg_t> = unimplemented!();
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
thread_local!(
    pub static next_242: Value<i32> = Rc::new(RefCell::new(0));
);
pub fn bump_243(mut v: Ptr<vector_209>, mut x: i32) -> i32 {
    {
        let __rhs = x;
        ({ vector_209Impl::operator_index_46(&(v), 0_u64) }).with_mut(|__v| *__v = *__v + __rhs)
    };
    return (*next_242.with(Value::clone).borrow_mut()).postfix_inc();
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let v: Value<vector_209> = Rc::new(RefCell::new({
        let __tmp_0: Value<i32> = Rc::new(RefCell::new(1));
        vector_209::new_1({ 4_u64 }, { __tmp_0.as_pointer() }, None)
    }));
    let items: Value<vector_210> = Rc::new(RefCell::new(vector_210::new_1({ 3_u64 }, None)));
    let mut i: u32 = 0_u32;
    'loop_: while ((i as u64) < ({ vector_210Impl::size(&items.as_pointer()) })) {
        let __rhs = (i).wrapping_mul(2_u32);
        ({ vector_210Impl::operator_index_46(&items.as_pointer(), (i as u64)) }).write(__rhs);
        i.prefix_inc();
    }
    {
        let rhs_0 = (({ vector_210Impl::operator_index_46(&items.as_pointer(), 1_u64) }).read())
            .wrapping_add(5_u32);
        ({ vector_210Impl::operator_index_46(&items.as_pointer(), 1_u64) }).write(rhs_0)
    };
    ({ vector_210Impl::operator_index_46(&items.as_pointer(), 2_u64) })
        .with_mut(|__v| __v.postfix_inc());
    (if (((({ vector_210Impl::operator_index_46(&items.as_pointer(), 0_u64) }).read()) == 0_u32)
        && ((({ vector_210Impl::operator_index_46(&items.as_pointer(), 1_u64) }).read()) == 7_u32))
        && ((({ vector_210Impl::operator_index_46(&items.as_pointer(), 2_u64) }).read()) == 5_u32)
    {
        &(0);
    } else {
        ({
            __assert_fail_244(
                Ptr::<i8>::from_string_literal(b"items[0] == 0 && items[1] == 7 && items[2] == 5"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    ({
        let ___n: u64 =
            ((({ vector_209Impl::operator_index_46(&v.as_pointer(), 0_u64) }).read()) as u64);
        vector_209Impl::operator_index_46(&v.as_pointer(), ___n)
    })
    .write(3);
    let __rhs = ((({ vector_209Impl::operator_index_46(&v.as_pointer(), 1_u64) }).read()) + 4);
    ({
        let ___n: u64 =
            ((({ vector_209Impl::operator_index_46(&v.as_pointer(), 1_u64) }).read()) as u64);
        vector_209Impl::operator_index_46(&v.as_pointer(), ___n)
    })
    .write(__rhs);
    (if ((({ vector_209Impl::operator_index_46(&v.as_pointer(), 1_u64) }).read()) == 3)
        && ((({ vector_209Impl::operator_index_46(&v.as_pointer(), 3_u64) }).read()) == 7)
    {
        &(0);
    } else {
        ({
            __assert_fail_244(
                Ptr::<i8>::from_string_literal(b"v[1] == 3 && v[3] == 7"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut p: Ptr<i32> = ({ vector_209Impl::operator_index_46(&v.as_pointer(), 0_u64) });
    ({ vector_209Impl::operator_index_46(&v.as_pointer(), ((p.read()) as u64)) }).write(9);
    (if ((({ vector_209Impl::operator_index_46(&v.as_pointer(), 1_u64) }).read()) == 9) {
        &(0);
    } else {
        ({
            __assert_fail_244(
                Ptr::<i8>::from_string_literal(b"v[1] == 9"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    ({
        let ___n: u64 = (({ bump_243((v.as_pointer()), 10) }) as u64);
        vector_209Impl::operator_index_46(&v.as_pointer(), ___n)
    })
    .write(2);
    (if ((({ vector_209Impl::operator_index_46(&v.as_pointer(), 0_u64) }).read()) == 2) {
        &(0);
    } else {
        ({
            __assert_fail_244(
                Ptr::<i8>::from_string_literal(b"v[0] == 2"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({
        let _v: Ptr<vector_209> = (v.as_pointer());
        let _x: i32 = (({ vector_209Impl::operator_index_46(&v.as_pointer(), 2_u64) }).read());
        bump_243(_v, _x)
    }) == 1)
        && ((({ vector_209Impl::operator_index_46(&v.as_pointer(), 0_u64) }).read()) == 3)
    {
        &(0);
    } else {
        ({
            __assert_fail_244(
                Ptr::<i8>::from_string_literal(b"bump(&v, v[2]) == 1 && v[0] == 3"),
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
pub fn __assert_fail_244(
    mut __assertion: Ptr<i8>,
    mut __file: Ptr<i8>,
    mut __line: u32,
    mut __function: Ptr<i8>,
) {
    unimplemented!()
}
pub trait _UninitDestroyGuard_201Impl {
    fn release(&self) {
        unimplemented!()
    }
}
impl _UninitDestroyGuard_201Impl for Ptr<_UninitDestroyGuard_201> {}
pub trait _Vector_base_207Impl {
    fn _M_get_Tp_allocator_1(&self) -> Ptr<allocator_192> {
        unimplemented!()
    }
    fn _M_allocate(&self, mut __n: usize) -> Ptr<i32> {
        unimplemented!()
    }
    fn _M_deallocate(&self, mut __p: Ptr<i32>, mut __n: usize) {
        unimplemented!()
    }
}
impl _Vector_base_207Impl for Ptr<_Vector_base_207> {}
pub trait _Vector_base_208Impl {
    fn _M_get_Tp_allocator_1(&self) -> Ptr<allocator_193> {
        unimplemented!()
    }
    fn _M_allocate(&self, mut __n: usize) -> Ptr<u32> {
        unimplemented!()
    }
    fn _M_deallocate(&self, mut __p: Ptr<u32>, mut __n: usize) {
        unimplemented!()
    }
}
impl _Vector_base_208Impl for Ptr<_Vector_base_208> {}
pub trait __new_allocator_189Impl {
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
impl __new_allocator_189Impl for Ptr<__new_allocator_189> {}
pub trait __new_allocator_190Impl {
    fn allocate(&self, mut __n: u64, _a1: Option<AnyPtr>) -> Ptr<u32> {
        unimplemented!()
    }
    fn deallocate(&self, mut __p: Ptr<u32>, mut __n: u64) {
        unimplemented!()
    }
    fn max_size(&self) -> u64 {
        unimplemented!()
    }
}
impl __new_allocator_190Impl for Ptr<__new_allocator_190> {}
pub trait initializer_list_204Impl {
    fn size(&self) -> u64 {
        unimplemented!()
    }
    fn begin(&self) -> Ptr<bool> {
        unimplemented!()
    }
}
impl initializer_list_204Impl for Ptr<initializer_list_204> {}
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
pub trait vector_209Impl {
    fn copy_assign(&self, __x: Ptr<vector_209>) -> Ptr<vector_209> {
        unimplemented!()
    }
    fn move_assign(&self, __x: Ptr<vector_209>) -> Ptr<vector_209> {
        unimplemented!()
    }
    fn size(&self) -> u64 {
        unimplemented!()
    }
    fn operator_index_46(&self, mut __n: u64) -> Ptr<i32> {
        unimplemented!()
    }
}
impl vector_209Impl for Ptr<vector_209> {}
pub trait vector_210Impl {
    fn copy_assign(&self, __x: Ptr<vector_210>) -> Ptr<vector_210> {
        unimplemented!()
    }
    fn move_assign(&self, __x: Ptr<vector_210>) -> Ptr<vector_210> {
        unimplemented!()
    }
    fn size(&self) -> u64 {
        unimplemented!()
    }
    fn operator_index_46(&self, mut __n: u64) -> Ptr<u32> {
        unimplemented!()
    }
}
impl vector_210Impl for Ptr<vector_210> {}
pub fn __cpp2rust_init_globals() {
    let _ = next_242.with(|_| ());
}
