extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[derive(VaArg, FnPtrArg, Copy)]
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
pub static mut std_integral_constant_bool__true__value_1: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
impl integral_constant_0 {
    pub unsafe fn to_bool(&self) -> bool {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
pub static mut std_integral_constant_bool__false__value_3: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
pub static mut std___make_unsigned_selector_base__List_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long____size_173 : std::cell::LazyCell< usize > = std::cell::LazyCell::new(|| unsafe { unimplemented!() } ) ;
#[derive(VaArg, FnPtrArg, Copy)]
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
pub static mut std___make_unsigned_selector_base__List_unsigned_short__unsigned_int__unsigned_long__unsigned_long_long____size_175 : std::cell::LazyCell< usize > = std::cell::LazyCell::new(|| unsafe { unimplemented!() } ) ;
#[derive(VaArg, FnPtrArg, Copy)]
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
pub static mut std___make_unsigned_selector_base__List_unsigned_int__unsigned_long__unsigned_long_long____size_177 : std::cell::LazyCell< usize > = std::cell::LazyCell::new(|| unsafe { unimplemented!() } ) ;
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const std___nonesuchbase,
    ) -> *mut std___nonesuchbase {
        unimplemented!()
    }
}
impl std___nonesuchbase {
    pub unsafe fn move_assign(&mut self, _a0: *mut std___nonesuchbase) -> *mut std___nonesuchbase {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
pub static mut is_floating_point_v_206: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_integral_v_207: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_integral_v_208: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_same_v_209: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_same_v_210: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_integral_v_211: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_same_v_212: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_same_v_213: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_integral_v_214: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_integral_v_215: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_same_v_216: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_same_v_217: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_object_v_218: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_object_v_219: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_object_v_220: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_object_v_221: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_same_v_222: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut is_integral_v_223: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
pub static mut __can_use_memchr_for_find_260: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
pub static mut __gnu_cxx___numeric_traits_integer_long____is_signed_269: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_long____digits_270: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_long____max_271: std::cell::LazyCell<i64> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
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
pub static mut __gnu_cxx___numeric_traits_integer_unsigned___int128____is_signed_273:
    std::cell::LazyCell<bool> = std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_unsigned___int128____digits_274:
    std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_unsigned___int128____max_275:
    std::cell::LazyCell<u128> = std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
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
pub static mut __gnu_cxx___numeric_traits_integer_unsigned_long____is_signed_277:
    std::cell::LazyCell<bool> = std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_unsigned_long____digits_278: std::cell::LazyCell<
    i32,
> = std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_unsigned_long____max_279: std::cell::LazyCell<
    u64,
> = std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
pub static mut swap_284: std::cell::LazyCell<std_ranges___swap__Swap> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub type std___cmp_cat__Ord = i8;
pub const std___cmp_cat__Ord_equivalent: std___cmp_cat__Ord = 0;
pub const std___cmp_cat__Ord_less: std___cmp_cat__Ord = -1;
pub const std___cmp_cat__Ord_greater: std___cmp_cat__Ord = 1;
pub type std___cmp_cat__Ncmp = i8;
pub const std___cmp_cat__Ncmp__Unordered: std___cmp_cat__Ncmp = 2;
#[derive(VaArg, FnPtrArg, Copy)]
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
    pub unsafe fn new(mut _a0: *mut std___cmp_cat___unspec) -> Self {
        unimplemented!()
    }
}
impl std___cmp_cat___unspec {}
impl std___cmp_cat___unspec {}
#[derive(VaArg, FnPtrArg, Copy)]
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
pub static mut std_partial_ordering_less_285: std::cell::LazyCell<std_partial_ordering> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_partial_ordering_equivalent_286: std::cell::LazyCell<std_partial_ordering> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_partial_ordering_greater_287: std::cell::LazyCell<std_partial_ordering> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_partial_ordering_unordered_288: std::cell::LazyCell<std_partial_ordering> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
impl std_partial_ordering {}
impl std_partial_ordering {}
#[derive(VaArg, FnPtrArg, Copy)]
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
pub static mut std_weak_ordering_less_289: std::cell::LazyCell<std_weak_ordering> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_weak_ordering_equivalent_290: std::cell::LazyCell<std_weak_ordering> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_weak_ordering_greater_291: std::cell::LazyCell<std_weak_ordering> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
impl std_weak_ordering {}
impl std_weak_ordering {}
#[derive(VaArg, FnPtrArg, Copy)]
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
pub static mut std_strong_ordering_less_292: std::cell::LazyCell<std_strong_ordering> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_strong_ordering_equal_293: std::cell::LazyCell<std_strong_ordering> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_strong_ordering_equivalent_294: std::cell::LazyCell<std_strong_ordering> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut std_strong_ordering_greater_295: std::cell::LazyCell<std_strong_ordering> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
impl std_strong_ordering {}
impl std_strong_ordering {}
pub static mut __cmp_cat_id_296: std::cell::LazyCell<u32> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __cmp_cat_id_297: std::cell::LazyCell<u32> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg)]
pub struct _Int_298 {
    pub _M_lo: u64,
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
#[derive(VaArg, FnPtrArg)]
pub struct _Int_299 {
    pub _M_lo: u64,
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
#[derive(VaArg, FnPtrArg)]
pub struct _Int_300 {
    pub _M_lo: u64,
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
pub static mut __synth3way_301: std::cell::LazyCell<std___detail__Synth3way> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
pub static mut iter_move_307: std::cell::LazyCell<std_ranges___imove__IterMove> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
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
pub static mut iter_swap_308: std::cell::LazyCell<std_ranges___iswap__IterSwap> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
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
pub static mut __decay_copy_309: std::cell::LazyCell<std_ranges___access__Decay_copy> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
    pub unsafe fn new() -> Self {
        unimplemented!()
    }
}
impl std_exception {
    pub unsafe fn copy_from(_a0: *const std_exception) -> Self {
        unimplemented!()
    }
}
impl std_exception {
    pub unsafe fn copy_assign(&mut self, _a0: *const std_exception) -> *mut std_exception {
        unimplemented!()
    }
}
impl std_exception {
    pub unsafe fn move_from(_a0: *mut std_exception) -> Self {
        unimplemented!()
    }
}
impl std_exception {
    pub unsafe fn move_assign(&mut self, _a0: *mut std_exception) -> *mut std_exception {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
    pub unsafe fn new() -> Self {
        unimplemented!()
    }
}
impl std_bad_alloc {
    pub unsafe fn copy_from(_a0: *const std_bad_alloc) -> Self {
        unimplemented!()
    }
}
impl std_bad_alloc {
    pub unsafe fn copy_assign(&mut self, _a0: *const std_bad_alloc) -> *mut std_bad_alloc {
        unimplemented!()
    }
}
pub type std_align_val_t = usize;
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
impl __gnu_cxx___ops__Iter_less_iter {
    pub unsafe fn operator_call_311(&self, mut __it1: *mut X, mut __it2: *mut X) -> bool {
        unimplemented!()
    }
}
impl __gnu_cxx___ops__Iter_less_iter {
    pub unsafe fn operator_call_312(&self, mut __it1: *mut Mixed, mut __it2: *mut Mixed) -> bool {
        unimplemented!()
    }
}
impl __gnu_cxx___ops__Iter_less_iter {
    pub unsafe fn operator_call_313(
        &self,
        mut __it1: *mut Boolish,
        mut __it2: *mut Boolish,
    ) -> bool {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
    pub unsafe fn new_1() -> Self {
        unimplemented!()
    }
}
impl __gnu_cxx___ops__Iter_less_val {
    pub unsafe fn new_2(mut _a0: __gnu_cxx___ops__Iter_less_iter) -> Self {
        unimplemented!()
    }
}
impl __gnu_cxx___ops__Iter_less_val {}
impl __gnu_cxx___ops__Iter_less_val {}
impl __gnu_cxx___ops__Iter_less_val {
    pub unsafe fn operator_call_314(&self, mut __it: *mut X, __val: *mut X) -> bool {
        unimplemented!()
    }
}
impl __gnu_cxx___ops__Iter_less_val {
    pub unsafe fn operator_call_315(&self, mut __it: *mut Mixed, __val: *mut Mixed) -> bool {
        unimplemented!()
    }
}
impl __gnu_cxx___ops__Iter_less_val {
    pub unsafe fn operator_call_316(&self, mut __it: *mut Boolish, __val: *mut Boolish) -> bool {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
impl __gnu_cxx___ops__Val_less_iter {
    pub unsafe fn operator_call_317(&self, __val: *mut X, mut __it: *mut X) -> bool {
        unimplemented!()
    }
}
impl __gnu_cxx___ops__Val_less_iter {
    pub unsafe fn operator_call_318(&self, __val: *mut Mixed, mut __it: *mut Mixed) -> bool {
        unimplemented!()
    }
}
impl __gnu_cxx___ops__Val_less_iter {
    pub unsafe fn operator_call_319(&self, __val: *mut Boolish, mut __it: *mut Boolish) -> bool {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg)]
pub struct _Iter_equals_val_320 {
    pub _M_value: *const RefInt,
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
    pub unsafe fn new(__value: *const RefInt) -> Self {
        unimplemented!()
    }
}
impl _Iter_equals_val_320 {}
impl _Iter_equals_val_320 {}
impl _Iter_equals_val_320 {
    pub unsafe fn operator_call_321(&mut self, mut __it: *mut RefInt) -> bool {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct div_t {
    pub quot: i32,
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
#[derive(VaArg, FnPtrArg)]
pub struct ldiv_t {
    pub quot: i64,
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
#[derive(VaArg, FnPtrArg)]
pub struct lldiv_t {
    pub quot: i64,
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
#[derive(VaArg, FnPtrArg)]
pub struct __locale_struct {
    pub __locales: [*mut __locale_data; 13],
    pub __ctype_b: *const u16,
    pub __ctype_tolower: *const i32,
    pub __ctype_toupper: *const i32,
    pub __names: [*const libc::c_char; 13],
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
#[derive(VaArg, FnPtrArg)]
pub struct __fsid_t {
    pub __val: [i32; 2],
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
#[derive(VaArg, FnPtrArg)]
pub struct __sigset_t {
    pub __val: [u64; 16],
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
#[derive(VaArg, FnPtrArg)]
pub struct timeval {
    pub tv_sec: i64,
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
#[derive(VaArg, FnPtrArg)]
pub struct timespec {
    pub tv_sec: i64,
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
#[derive(VaArg, FnPtrArg)]
pub struct fd_set {
    pub fds_bits: [i64; 16],
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
#[derive(VaArg, FnPtrArg)]
pub struct __atomic_wide_counter {
    pub __value64: u64,
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
#[derive(VaArg, FnPtrArg)]
pub struct anon_322 {
    pub __low: u32,
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
#[derive(VaArg, FnPtrArg)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
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
#[derive(VaArg, FnPtrArg)]
pub struct __pthread_internal_slist {
    pub __next: *mut __pthread_internal_slist,
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
#[derive(VaArg, FnPtrArg)]
pub struct __pthread_mutex_s {
    pub __lock: i32,
    pub __count: u32,
    pub __owner: i32,
    pub __nusers: u32,
    pub __kind: i32,
    pub __spins: i16,
    pub __elision: i16,
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
#[derive(VaArg, FnPtrArg)]
pub struct __pthread_rwlock_arch_t {
    pub __readers: u32,
    pub __writers: u32,
    pub __wrphase_futex: u32,
    pub __writers_futex: u32,
    pub __pad3: u32,
    pub __pad4: u32,
    pub __cur_writer: i32,
    pub __shared: i32,
    pub __rwelision: i8,
    pub __pad1: [u8; 7],
    pub __pad2: u64,
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
#[derive(VaArg, FnPtrArg)]
pub struct __pthread_cond_s {
    pub __wseq: __atomic_wide_counter,
    pub __g1_start: __atomic_wide_counter,
    pub __g_refs: [u32; 2],
    pub __g_size: [u32; 2],
    pub __g1_orig_size: u32,
    pub __wrefs: u32,
    pub __g_signals: [u32; 2],
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
#[derive(VaArg, FnPtrArg)]
pub struct __once_flag {
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
#[derive(VaArg, FnPtrArg)]
pub struct pthread_mutexattr_t {
    pub __size: [libc::c_char; 4],
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
#[derive(VaArg, FnPtrArg)]
pub struct pthread_condattr_t {
    pub __size: [libc::c_char; 4],
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
#[derive(VaArg, FnPtrArg)]
pub struct pthread_attr_t {
    pub __size: [libc::c_char; 56],
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
#[derive(VaArg, FnPtrArg)]
pub struct pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
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
#[derive(VaArg, FnPtrArg)]
pub struct pthread_cond_t {
    pub __data: __pthread_cond_s,
    pub __size: [libc::c_char; 48],
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
#[derive(VaArg, FnPtrArg)]
pub struct pthread_rwlock_t {
    pub __data: __pthread_rwlock_arch_t,
    pub __size: [libc::c_char; 56],
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
#[derive(VaArg, FnPtrArg)]
pub struct pthread_rwlockattr_t {
    pub __size: [libc::c_char; 8],
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
#[derive(VaArg, FnPtrArg)]
pub struct pthread_barrier_t {
    pub __size: [libc::c_char; 32],
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
#[derive(VaArg, FnPtrArg)]
pub struct pthread_barrierattr_t {
    pub __size: [libc::c_char; 4],
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
#[derive(VaArg, FnPtrArg)]
pub struct random_data {
    pub fptr: *mut i32,
    pub rptr: *mut i32,
    pub state: *mut i32,
    pub rand_type: i32,
    pub rand_deg: i32,
    pub rand_sep: i32,
    pub end_ptr: *mut i32,
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
#[derive(VaArg, FnPtrArg)]
pub struct drand48_data {
    pub __x: [u16; 3],
    pub __old_x: [u16; 3],
    pub __c: u16,
    pub __init: u16,
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
pub static mut e_v_325: std::cell::LazyCell<f64> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut log2e_v_326: std::cell::LazyCell<f64> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut log10e_v_327: std::cell::LazyCell<f64> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut pi_v_328: std::cell::LazyCell<f64> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut inv_pi_v_329: std::cell::LazyCell<f64> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut inv_sqrtpi_v_330: std::cell::LazyCell<f64> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut ln2_331: std::cell::LazyCell<f64> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut ln2_v_332: std::cell::LazyCell<f64> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut ln10_333: std::cell::LazyCell<f64> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut ln10_v_334: std::cell::LazyCell<f64> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut sqrt2_v_335: std::cell::LazyCell<f64> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut sqrt3_v_336: std::cell::LazyCell<f64> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut inv_sqrt3_v_337: std::cell::LazyCell<f64> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut egamma_v_338: std::cell::LazyCell<f64> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut phi_v_339: std::cell::LazyCell<f64> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
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
pub static mut std_numeric_limits_std_ranges___detail___max_size_type__digits_341:
    std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { unimplemented!() });
impl numeric_limits_340 {
    pub unsafe fn min() -> std_ranges___detail___max_size_type {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
pub static mut std_numeric_limits_std_ranges___detail___max_diff_type__digits_343:
    std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { unimplemented!() });
impl numeric_limits_342 {
    pub unsafe fn min() -> std_ranges___detail___max_diff_type {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
pub static mut std_ranges___detail___max_size_type__S_rep_bits_344: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
impl std_ranges___detail___max_size_type {
    pub unsafe fn new_1(__d: *const std_ranges___detail___max_diff_type) -> Self {
        unimplemented!()
    }
}
impl std_ranges___detail___max_size_type {
    pub unsafe fn operator_bitnot(&self) -> std_ranges___detail___max_size_type {
        unimplemented!()
    }
}
impl std_ranges___detail___max_size_type {
    pub unsafe fn operator_neg(&self) -> std_ranges___detail___max_size_type {
        unimplemented!()
    }
}
impl std_ranges___detail___max_size_type {
    pub unsafe fn operator_inc_9(&mut self) -> *mut std_ranges___detail___max_size_type {
        unimplemented!()
    }
}
impl std_ranges___detail___max_size_type {
    pub unsafe fn operator_dec_11(&mut self) -> *mut std_ranges___detail___max_size_type {
        unimplemented!()
    }
}
impl std_ranges___detail___max_size_type {
    pub unsafe fn operator_add_assign(
        &mut self,
        __r: *const std_ranges___detail___max_size_type,
    ) -> *mut std_ranges___detail___max_size_type {
        unimplemented!()
    }
}
impl std_ranges___detail___max_size_type {
    pub unsafe fn operator_sub_assign(
        &mut self,
        __r: *const std_ranges___detail___max_size_type,
    ) -> *mut std_ranges___detail___max_size_type {
        unimplemented!()
    }
}
impl std_ranges___detail___max_size_type {
    pub unsafe fn operator_mul_assign(
        &mut self,
        mut __r: std_ranges___detail___max_size_type,
    ) -> *mut std_ranges___detail___max_size_type {
        unimplemented!()
    }
}
impl std_ranges___detail___max_size_type {
    pub unsafe fn operator_div_assign(
        &mut self,
        __r: *const std_ranges___detail___max_size_type,
    ) -> *mut std_ranges___detail___max_size_type {
        unimplemented!()
    }
}
impl std_ranges___detail___max_size_type {
    pub unsafe fn operator_rem_assign(
        &mut self,
        __r: *const std_ranges___detail___max_size_type,
    ) -> *mut std_ranges___detail___max_size_type {
        unimplemented!()
    }
}
impl std_ranges___detail___max_size_type {
    pub unsafe fn operator_shl_assign(
        &mut self,
        __r: *const std_ranges___detail___max_size_type,
    ) -> *mut std_ranges___detail___max_size_type {
        unimplemented!()
    }
}
impl std_ranges___detail___max_size_type {
    pub unsafe fn operator_shr_assign(
        &mut self,
        __r: *const std_ranges___detail___max_size_type,
    ) -> *mut std_ranges___detail___max_size_type {
        unimplemented!()
    }
}
impl std_ranges___detail___max_size_type {
    pub unsafe fn operator_bitand_assign(
        &mut self,
        __r: *const std_ranges___detail___max_size_type,
    ) -> *mut std_ranges___detail___max_size_type {
        unimplemented!()
    }
}
impl std_ranges___detail___max_size_type {
    pub unsafe fn operator_bitor_assign(
        &mut self,
        __r: *const std_ranges___detail___max_size_type,
    ) -> *mut std_ranges___detail___max_size_type {
        unimplemented!()
    }
}
impl std_ranges___detail___max_size_type {
    pub unsafe fn operator_bitxor_assign(
        &mut self,
        __r: *const std_ranges___detail___max_size_type,
    ) -> *mut std_ranges___detail___max_size_type {
        unimplemented!()
    }
}
impl std_ranges___detail___max_size_type {}
impl std_ranges___detail___max_size_type {}
impl std_ranges___detail___max_size_type {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const std_ranges___detail___max_size_type,
    ) -> *mut std_ranges___detail___max_size_type {
        unimplemented!()
    }
}
impl std_ranges___detail___max_size_type {
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut std_ranges___detail___max_size_type,
    ) -> *mut std_ranges___detail___max_size_type {
        unimplemented!()
    }
}
impl std_ranges___detail___max_size_type {
    pub unsafe fn new_2(mut __i: i32) -> Self {
        unimplemented!()
    }
}
impl std_ranges___detail___max_size_type {
    pub unsafe fn new_3(mut __i: u128) -> Self {
        unimplemented!()
    }
}
impl std_ranges___detail___max_size_type {
    pub unsafe fn new_4(mut __i: u64) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
    pub unsafe fn new_1(__d: *const std_ranges___detail___max_size_type) -> Self {
        unimplemented!()
    }
}
impl std_ranges___detail___max_diff_type {
    pub unsafe fn operator_inc_9(&mut self) -> *mut std_ranges___detail___max_diff_type {
        unimplemented!()
    }
}
impl std_ranges___detail___max_diff_type {
    pub unsafe fn operator_dec_11(&mut self) -> *mut std_ranges___detail___max_diff_type {
        unimplemented!()
    }
}
impl std_ranges___detail___max_diff_type {
    pub unsafe fn operator_add_assign(
        &mut self,
        __r: *const std_ranges___detail___max_diff_type,
    ) -> *mut std_ranges___detail___max_diff_type {
        unimplemented!()
    }
}
impl std_ranges___detail___max_diff_type {
    pub unsafe fn operator_sub_assign(
        &mut self,
        __r: *const std_ranges___detail___max_diff_type,
    ) -> *mut std_ranges___detail___max_diff_type {
        unimplemented!()
    }
}
impl std_ranges___detail___max_diff_type {
    pub unsafe fn operator_mul_assign(
        &mut self,
        __r: *const std_ranges___detail___max_diff_type,
    ) -> *mut std_ranges___detail___max_diff_type {
        unimplemented!()
    }
}
impl std_ranges___detail___max_diff_type {
    pub unsafe fn operator_div_assign(
        &mut self,
        __r: *const std_ranges___detail___max_diff_type,
    ) -> *mut std_ranges___detail___max_diff_type {
        unimplemented!()
    }
}
impl std_ranges___detail___max_diff_type {
    pub unsafe fn operator_rem_assign(
        &mut self,
        __r: *const std_ranges___detail___max_diff_type,
    ) -> *mut std_ranges___detail___max_diff_type {
        unimplemented!()
    }
}
impl std_ranges___detail___max_diff_type {
    pub unsafe fn operator_shl_assign(
        &mut self,
        __r: *const std_ranges___detail___max_diff_type,
    ) -> *mut std_ranges___detail___max_diff_type {
        unimplemented!()
    }
}
impl std_ranges___detail___max_diff_type {
    pub unsafe fn operator_shr_assign(
        &mut self,
        __r: *const std_ranges___detail___max_diff_type,
    ) -> *mut std_ranges___detail___max_diff_type {
        unimplemented!()
    }
}
impl std_ranges___detail___max_diff_type {
    pub unsafe fn operator_bitand_assign(
        &mut self,
        __r: *const std_ranges___detail___max_diff_type,
    ) -> *mut std_ranges___detail___max_diff_type {
        unimplemented!()
    }
}
impl std_ranges___detail___max_diff_type {
    pub unsafe fn operator_bitor_assign(
        &mut self,
        __r: *const std_ranges___detail___max_diff_type,
    ) -> *mut std_ranges___detail___max_diff_type {
        unimplemented!()
    }
}
impl std_ranges___detail___max_diff_type {
    pub unsafe fn operator_bitxor_assign(
        &mut self,
        __r: *const std_ranges___detail___max_diff_type,
    ) -> *mut std_ranges___detail___max_diff_type {
        unimplemented!()
    }
}
impl std_ranges___detail___max_diff_type {
    pub unsafe fn operator_cmp(
        &self,
        __r: *const std_ranges___detail___max_diff_type,
    ) -> std_strong_ordering {
        unimplemented!()
    }
}
impl std_ranges___detail___max_diff_type {}
impl std_ranges___detail___max_diff_type {}
impl std_ranges___detail___max_diff_type {
    pub unsafe fn new_2(mut __i: i32) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
pub static mut begin_345: std::cell::LazyCell<std_ranges___access__Begin> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut end_346: std::cell::LazyCell<std_ranges___access__End> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut size_347: std::cell::LazyCell<std_ranges___access__Size> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut empty_348: std::cell::LazyCell<std_ranges___access__Empty> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut data_349: std::cell::LazyCell<std_ranges___access__Data> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
pub static mut advance_350: std::cell::LazyCell<std_ranges___advance_fn> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
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
pub static mut distance_351: std::cell::LazyCell<std_ranges___distance_fn> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
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
pub static mut next_352: std::cell::LazyCell<std_ranges___next_fn> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
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
pub static mut prev_353: std::cell::LazyCell<std_ranges___prev_fn> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
pub static mut copy_354: std::cell::LazyCell<std_ranges___copy_fn> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
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
pub static mut move_355: std::cell::LazyCell<std_ranges___move_fn> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
pub static mut move_backward_356: std::cell::LazyCell<std_ranges___move_backward_fn> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
pub static mut fill_n_357: std::cell::LazyCell<std_ranges___fill_n_fn> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
pub static mut find_if_358: std::cell::LazyCell<std_ranges___find_if_fn> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
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
pub static mut find_if_not_359: std::cell::LazyCell<std_ranges___find_if_not_fn> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
pub static mut search_360: std::cell::LazyCell<std_ranges___search_fn> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
pub static mut adjacent_find_361: std::cell::LazyCell<std_ranges___adjacent_find_fn> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
pub static mut none_of_362: std::cell::LazyCell<std_ranges___none_of_fn> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
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
pub static mut for_each_363: std::cell::LazyCell<std_ranges___for_each_fn> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
pub static mut count_if_364: std::cell::LazyCell<std_ranges___count_if_fn> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
pub static mut swap_ranges_365: std::cell::LazyCell<std_ranges___swap_ranges_fn> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
pub static mut remove_if_366: std::cell::LazyCell<std_ranges___remove_if_fn> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
pub static mut reverse_367: std::cell::LazyCell<std_ranges___reverse_fn> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
pub static mut push_heap_368: std::cell::LazyCell<std_ranges___push_heap_fn> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
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
pub static mut pop_heap_369: std::cell::LazyCell<std_ranges___pop_heap_fn> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
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
pub static mut make_heap_370: std::cell::LazyCell<std_ranges___make_heap_fn> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
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
pub static mut sort_heap_371: std::cell::LazyCell<std_ranges___sort_heap_fn> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
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
pub static mut is_heap_until_372: std::cell::LazyCell<std_ranges___is_heap_until_fn> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
pub static mut lower_bound_373: std::cell::LazyCell<std_ranges___lower_bound_fn> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
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
pub static mut upper_bound_374: std::cell::LazyCell<std_ranges___upper_bound_fn> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct X {
    pub v: i32,
}
impl std::cmp::Ord for X {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
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
        unsafe { ((operator_eq_376(self.clone(), other.clone())) != 0) }
    }
}
impl std::cmp::Eq for X {}
pub unsafe fn operator_eq_376(mut a: X, mut b: X) -> i32 {
    return if ((a.v) == (b.v)) { 2 } else { 0 };
}
pub unsafe fn operator_ne_377(mut a: X, mut b: X) -> i32 {
    return if ((a.v) != (b.v)) { 3 } else { 0 };
}
pub unsafe fn operator_lt_375(mut a: X, mut b: X) -> i32 {
    return if ((a.v) < (b.v)) { 4 } else { 0 };
}
pub unsafe fn operator_gt_378(mut a: X, mut b: X) -> i32 {
    return if ((a.v) > (b.v)) { 5 } else { 0 };
}
pub unsafe fn operator_le_379(mut a: X, mut b: X) -> i32 {
    return if ((a.v) <= (b.v)) { 6 } else { 0 };
}
pub unsafe fn operator_ge_380(mut a: X, mut b: X) -> i32 {
    return if ((a.v) >= (b.v)) { 7 } else { 0 };
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Result {
    pub r: i32,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Custom {
    pub v: i32,
}
impl Custom {
    pub unsafe fn operator_cmp(&self, o: *const Custom) -> Result {
        return Result {
            r: ((self.v) - ((*o).v)),
        };
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Mixed {
    pub v: i32,
}
impl Mixed {
    pub unsafe fn operator_cmp(&self, o: *const Mixed) -> Result {
        return Result {
            r: ((self.v) - ((*o).v)),
        };
    }
    pub unsafe fn operator_lt(&self, o: *const Mixed) -> i32 {
        return if ((self.v) < ((*o).v)) { 8 } else { 0 };
    }
    pub unsafe fn operator_eq(&self, o: *const Mixed) -> i32 {
        return if ((self.v) == ((*o).v)) { 9 } else { 0 };
    }
}
impl std::cmp::Ord for Mixed {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if ((Mixed::operator_lt(self, other as *const Mixed)) != 0) {
                std::cmp::Ordering::Less
            } else if ((Mixed::operator_lt(other, self as *const Mixed)) != 0) {
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
        unsafe { ((Mixed::operator_eq(self, other as *const Mixed)) != 0) }
    }
}
impl std::cmp::Eq for Mixed {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct MyBool {
    pub value: bool,
}
impl MyBool {
    pub unsafe fn new(mut v: bool) -> Self {
        let mut this = Self { value: v };
        this
    }
    pub unsafe fn to_bool(&self) -> bool {
        return self.value;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Boolish {
    pub v: i32,
}
impl std::cmp::Ord for Boolish {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if {
                let __cmp = &operator_lt_381(self.clone(), other.clone());
                MyBool::to_bool(__cmp)
            } {
                std::cmp::Ordering::Less
            } else if {
                let __cmp = &operator_lt_381(other.clone(), self.clone());
                MyBool::to_bool(__cmp)
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
        unsafe {
            {
                let __cmp = &operator_eq_382(self.clone(), other.clone());
                MyBool::to_bool(__cmp)
            }
        }
    }
}
impl std::cmp::Eq for Boolish {}
pub unsafe fn operator_eq_382(mut a: Boolish, mut b: Boolish) -> MyBool {
    return MyBool::new({ ((a.v) == (b.v)) });
}
pub unsafe fn operator_lt_381(mut a: Boolish, mut b: Boolish) -> MyBool {
    return MyBool::new({ ((a.v) < (b.v)) });
}
#[repr(C)]
#[derive(FnPtrArg, Default)]
pub struct RefBool {
    pub value: bool,
}
impl RefBool {
    pub unsafe fn new(mut v: bool) -> Self {
        let mut this = Self { value: v };
        this
    }
    pub unsafe fn to_bool(&self) -> bool {
        return self.value;
    }
}
pub static mut ref_true_383: std::cell::LazyCell<RefBool> =
    std::cell::LazyCell::new(|| unsafe { RefBool::new({ true }) });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct RefInt {
    pub v: i32,
}
impl RefInt {
    pub unsafe fn operator_eq(&self, _a0: *const RefInt) -> *mut RefBool {
        return &mut (*std::cell::LazyCell::force_mut(&mut *&raw mut ref_true_383));
    }
}
impl std::cmp::PartialEq for RefInt {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            {
                let __cmp = RefInt::operator_eq(self, other as *const RefInt);
                RefBool::to_bool(&*__cmp)
            }
        }
    }
}
impl std::cmp::Eq for RefInt {}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut a: X = X { v: 1 };
    let mut b: X = X { v: 2 };
    let mut c: X = X { v: 1 };
    (if ((unsafe {
        let _a: X = a;
        operator_eq_376(_a, c)
    }) == (2))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_384(
                c"(a == c) == 2".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe {
        let _a: X = a;
        operator_eq_376(_a, b)
    }) == (0))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_384(
                c"(a == b) == 0".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe {
        let _a: X = a;
        operator_ne_377(_a, b)
    }) == (3))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_384(
                c"(a != b) == 3".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe {
        let _a: X = a;
        operator_lt_375(_a, b)
    }) == (4))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_384(
                c"(a < b) == 4".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe {
        let _a: X = b;
        operator_gt_378(_a, a)
    }) == (5))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_384(
                c"(b > a) == 5".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe {
        let _a: X = a;
        operator_le_379(_a, c)
    }) == (6))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_384(
                c"(a <= c) == 6".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe {
        let _a: X = a;
        operator_ge_380(_a, c)
    }) == (7))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_384(
                c"(a >= c) == 7".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut xs: [X; 3] = [X { v: 3 }, X { v: 1 }, X { v: 2 }];
    (unsafe {
        let ___first: *mut X = xs.as_mut_ptr();
        let ___last: *mut X = xs.as_mut_ptr().offset((3) as isize);
        sort_385(___first, ___last)
    });
    (if (((xs[(0) as usize].v) == (1)) && ((xs[(1) as usize].v) == (2)))
        && ((xs[(2) as usize].v) == (3))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_384(
                c"xs[0].v == 1 && xs[1].v == 2 && xs[2].v == 3".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut p: Custom = Custom { v: 5 };
    let mut q: Custom = Custom { v: 2 };
    let mut r: Result = (unsafe { Custom::operator_cmp(&p, &q) });
    (if ((r.r) == (3)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_384(
                c"r.r == 3".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut m: Mixed = Mixed { v: 4 };
    let mut n: Mixed = Mixed { v: 4 };
    (if ((unsafe { Mixed::operator_eq(&m, &n) }) == (9)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_384(
                c"(m == n) == 9".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((unsafe { Mixed::operator_cmp(&m, &n) }).r) == (0)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_384(
                c"(m <=> n).r == 0".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut ms: [Mixed; 3] = [Mixed { v: 6 }, Mixed { v: 4 }, Mixed { v: 5 }];
    (unsafe {
        let ___first: *mut Mixed = ms.as_mut_ptr();
        let ___last: *mut Mixed = ms.as_mut_ptr().offset((3) as isize);
        sort_386(___first, ___last)
    });
    (if (((ms[(0) as usize].v) == (4)) && ((ms[(1) as usize].v) == (5)))
        && ((ms[(2) as usize].v) == (6))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_384(
                c"ms[0].v == 4 && ms[1].v == 5 && ms[2].v == 6".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe {
        let _o: *const Mixed = &ms[(1) as usize];
        Mixed::operator_lt(&ms[(0) as usize], _o)
    }) == (8))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_384(
                c"(ms[0] < ms[1]) == 8".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut b1: Boolish = Boolish { v: 1 };
    let mut b2: Boolish = Boolish { v: 2 };
    (if (unsafe {
        MyBool::to_bool(
            &(unsafe {
                let _a: Boolish = b1;
                operator_eq_382(_a, Boolish { v: 1 })
            }),
        )
    }) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_384(
                c"b1 == Boolish{1}".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if !(unsafe {
        MyBool::to_bool(
            &(unsafe {
                let _a: Boolish = b2;
                operator_lt_381(_a, b1)
            }),
        )
    }) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_384(
                c"!(b2 < b1)".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut lt: MyBool = (unsafe {
        let _a: Boolish = b1;
        operator_lt_381(_a, b2)
    });
    (if lt.value {
        &(0);
    } else {
        (unsafe {
            __assert_fail_384(
                c"lt.value".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut bs: [Boolish; 3] = [Boolish { v: 3 }, Boolish { v: 1 }, Boolish { v: 2 }];
    (unsafe {
        let ___first: *mut Boolish = bs.as_mut_ptr();
        let ___last: *mut Boolish = bs.as_mut_ptr().offset((3) as isize);
        sort_387(___first, ___last)
    });
    (if (((bs[(0) as usize].v) == (1)) && ((bs[(1) as usize].v) == (2)))
        && ((bs[(2) as usize].v) == (3))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_384(
                c"bs[0].v == 1 && bs[1].v == 2 && bs[2].v == 3".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut ri1: RefInt = RefInt { v: 1 };
    let mut ri2: RefInt = RefInt { v: 2 };
    (if (unsafe { RefBool::to_bool(&(*(unsafe { RefInt::operator_eq(&ri1, &ri2) }))) }) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_384(
                c"ri1 == ri2".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut ris: [RefInt; 2] = [RefInt { v: 1 }, RefInt { v: 2 }];
    (if ((unsafe {
        let ___first: *mut RefInt = ris.as_mut_ptr();
        let ___last: *mut RefInt = ris.as_mut_ptr().offset((2) as isize);
        find_388(___first, ___last, &ri2)
    }) == (ris.as_mut_ptr()))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_384(
                c"std::find(ris, ris + 2, ri2) == ris".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    return 0;
}
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct __locale_data;
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct std_ranges___access__Decay_copy;
pub unsafe fn __assert_fail_384(
    mut __assertion: *const libc::c_char,
    mut __file: *const libc::c_char,
    mut __line: u32,
    mut __function: *const libc::c_char,
) {
    unimplemented!()
}
pub unsafe fn sort_385(mut __first: *mut X, mut __last: *mut X) {
    unimplemented!()
}
pub unsafe fn sort_386(mut __first: *mut Mixed, mut __last: *mut Mixed) {
    unimplemented!()
}
pub unsafe fn sort_387(mut __first: *mut Boolish, mut __last: *mut Boolish) {
    unimplemented!()
}
pub unsafe fn find_388(
    mut __first: *mut RefInt,
    mut __last: *mut RefInt,
    __val: *const RefInt,
) -> *mut RefInt {
    unimplemented!()
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const ref_true_383);
}
