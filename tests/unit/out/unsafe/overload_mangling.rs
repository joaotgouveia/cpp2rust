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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
pub static mut std___make_unsigned_selector_base__List_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long____size_127 : std::cell::LazyCell< usize > = std::cell::LazyCell::new(|| unsafe { unimplemented!() } ) ;
#[derive(VaArg, FnPtrArg, Copy)]
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
pub static mut std___make_unsigned_selector_base__List_unsigned_short__unsigned_int__unsigned_long__unsigned_long_long____size_129 : std::cell::LazyCell< usize > = std::cell::LazyCell::new(|| unsafe { unimplemented!() } ) ;
#[derive(VaArg, FnPtrArg, Copy)]
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
pub static mut std___make_unsigned_selector_base__List_unsigned_int__unsigned_long__unsigned_long_long____size_131 : std::cell::LazyCell< usize > = std::cell::LazyCell::new(|| unsafe { unimplemented!() } ) ;
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg)]
pub struct pair_148 {
    pub first: i32,
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
    pub unsafe fn copy_from(_a0: *const pair_148) -> Self {
        unimplemented!()
    }
}
impl pair_148 {
    pub unsafe fn move_from(_a0: *mut pair_148) -> Self {
        unimplemented!()
    }
}
impl pair_148 {
    pub unsafe fn copy_assign(&mut self, __p: *const pair_148) -> *mut pair_148 {
        unimplemented!()
    }
}
impl pair_148 {
    pub unsafe fn move_assign(&mut self, __p: *mut pair_148) -> *mut pair_148 {
        unimplemented!()
    }
}
impl pair_148 {
    pub unsafe fn new(__x: *mut i32, __y: *mut i32) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct pair_149 {
    pub first: i32,
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
    pub unsafe fn copy_from(_a0: *const pair_149) -> Self {
        unimplemented!()
    }
}
impl pair_149 {
    pub unsafe fn move_from(_a0: *mut pair_149) -> Self {
        unimplemented!()
    }
}
impl pair_149 {
    pub unsafe fn copy_assign(&mut self, __p: *const pair_149) -> *mut pair_149 {
        unimplemented!()
    }
}
impl pair_149 {
    pub unsafe fn move_assign(&mut self, __p: *mut pair_149) -> *mut pair_149 {
        unimplemented!()
    }
}
impl pair_149 {
    pub unsafe fn new(__x: *mut i32, __y: *mut i64) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
pub unsafe fn inc_154(mut p: *mut i32) {
    (*p) += 1;
}
pub unsafe fn add_155(mut p: *mut i32, mut n: i32) {
    (*p) += n;
}
pub unsafe fn twice_156(mut n: i32) -> i32 {
    return ((n) * (2));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Access_157 {}
impl Access_157 {
    pub unsafe fn get_1(&mut self, mut p: *mut S) -> i32 {
        return (*p).base;
    }
    pub unsafe fn get_2(&mut self, mut p: *const S) -> i32 {
        return (((*p).base) + (1));
    }
    pub unsafe fn ref_3(&mut self, r: *mut S) -> i32 {
        return (((*r).base) + (2));
    }
    pub unsafe fn ref_4(&mut self, r: *const S) -> i32 {
        return (((*r).base) + (3));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct S {
    pub base: i32,
}
impl S {
    pub unsafe fn plain_4(&self, mut x: i32) -> i32 {
        return ((self.base) + (x));
    }
    pub unsafe fn plain_5(&self, mut x: i64) -> i32 {
        return (((self.base) + (x as i32)) + (1));
    }
    pub unsafe fn take_6(&self, x: *mut i32) -> i32 {
        return (((self.base) + (*x)) + (1));
    }
    pub unsafe fn take_7(&self, x: *mut i32) -> i32 {
        return (((self.base) + (*x)) + (2));
    }
    pub unsafe fn pick_8(&self, mut p: pair_148) -> i32 {
        return ((self.base) + (p.first));
    }
    pub unsafe fn pick_9(&self, mut p: pair_149) -> i32 {
        return ((self.base) + (p.second as i32));
    }
    pub unsafe fn apply_10(&self, mut f: Option<unsafe fn(*mut i32)>, mut x: i32) -> i32 {
        (unsafe { (f).unwrap()((&mut x as *mut i32)) });
        return ((self.base) + (x));
    }
    pub unsafe fn apply_11(&self, mut f: Option<unsafe fn(*mut i32, i32)>, mut x: i32) -> i32 {
        (unsafe { (f).unwrap()((&mut x as *mut i32), 10) });
        return ((self.base) + (x));
    }
    pub unsafe fn apply_12(&self, mut f: Option<unsafe fn(i32) -> i32>, mut x: i32) -> i32 {
        return ((self.base) + (unsafe { (f).unwrap()(x) }));
    }
    pub unsafe fn combine_13(
        &self,
        p: *const pair_149,
        mut f: Option<unsafe fn(i32) -> i32>,
        mut q: *const i32,
        mut n: u64,
    ) -> i32 {
        return ((((self.base) + ((*p).second as i32)) + (unsafe { (f).unwrap()((*q)) }))
            + (n as i32));
    }
    pub unsafe fn combine_14(
        &self,
        p: *const pair_148,
        mut f: Option<unsafe fn(*mut i32, i32)>,
        mut q: *mut i32,
        mut n: u64,
    ) -> i32 {
        (unsafe {
            let _arg0: *mut i32 = q;
            let _arg1: i32 = (n as i32);
            (f).unwrap()(_arg0, _arg1)
        });
        return (((self.base) + ((*p).first)) + (*q));
    }
    pub unsafe fn width_158(&self, mut x: i32) -> i32 {
        return ((self.base) + ((x) * (::std::mem::size_of::<libc::c_char>() as i32)));
    }
    pub unsafe fn width_159(&self, mut x: i32) -> i32 {
        return ((self.base) + ((x) * (::std::mem::size_of::<i32>() as i32)));
    }
    pub unsafe fn scale_160(&self, mut x: i32) -> i32 {
        return ((self.base) + ((x) * (2)));
    }
    pub unsafe fn scale_161(&self, mut x: i32) -> i32 {
        return ((self.base) + ((x) * (3)));
    }
    pub unsafe fn count_162(&self, mut x: i32) -> i32 {
        return (((self.base) + (x)) + (0 as i32));
    }
    pub unsafe fn count_163(&self, mut x: i32) -> i32 {
        return (((self.base) + (x)) + (2 as i32));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Box_ {
    pub v: i32,
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut s: S = S { base: 100 };
    (if ((unsafe { S::width_158(&s, 3) }) == (103)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_164(
                c"s.width<char>(3) == 103".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { S::width_159(&s, 3) }) == (112)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_164(
                c"s.width<int>(3) == 112".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { S::scale_160(&s, 5) }) == (110)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_164(
                c"s.scale<2>(5) == 110".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { S::scale_161(&s, 5) }) == (115)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_164(
                c"s.scale<3>(5) == 115".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { S::count_162(&s, 1) }) == (101)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_164(
                c"s.count<>(1) == 101".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { S::count_163(&s, 1) }) == (103)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_164(
                c"(s.count<int, long>(1) == 103)".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { S::plain_4(&s, 1) }) == (101)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_164(
                c"s.plain(1) == 101".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { S::plain_5(&s, 1_i64) }) == (102)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_164(
                c"s.plain(1L) == 102".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut y: i32 = 1;
    (if ((unsafe { S::take_6(&s, &mut y) }) == (102)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_164(
                c"s.take(y) == 102".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe {
        let mut _x: i32 = 5;
        S::take_7(&s, &mut _x)
    }) == (107))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_164(
                c"s.take(5) == 107".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe {
        S::pick_8(&s, {
            let mut __tmp_0: i32 = 1;
            let mut __tmp_1: i32 = 2;
            pair_148::new({ &mut __tmp_0 }, { &mut __tmp_1 })
        })
    }) == (101))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_164(
                c"s.pick(std::pair<int, int>(1, 2)) == 101".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe {
        S::pick_9(&s, {
            let mut __tmp_2: i32 = 1;
            let mut __tmp_3: i64 = 2_i64;
            pair_149::new({ &mut __tmp_2 }, { &mut __tmp_3 })
        })
    }) == (102))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_164(
                c"s.pick(std::pair<int, long>(1, 2L)) == 102".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { S::apply_10(&s, Some(inc_154), 1) }) == (102)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_164(
                c"s.apply(inc, 1) == 102".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { S::apply_11(&s, Some(add_155), 1) }) == (111)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_164(
                c"s.apply(add, 1) == 111".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { S::apply_12(&s, Some(twice_156), 3) }) == (106)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_164(
                c"s.apply(twice, 3) == 106".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let c: i32 = 3;
    (if ((unsafe {
        let mut _p: pair_149 = {
            let mut __tmp_4: i32 = 1;
            let mut __tmp_5: i64 = 2_i64;
            pair_149::new({ &mut __tmp_4 }, { &mut __tmp_5 })
        };
        S::combine_13(&s, &mut _p, Some(twice_156), (&c as *const i32), 4_u64)
    }) == (112))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_164(
                c"s.combine(std::pair<int, long>(1, 2L), twice, &c, 4UL) == 112".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut z: i32 = 1;
    (if ((unsafe {
        let mut _p: pair_148 = {
            let mut __tmp_6: i32 = 1;
            let mut __tmp_7: i32 = 2;
            pair_148::new({ &mut __tmp_6 }, { &mut __tmp_7 })
        };
        S::combine_14(&s, &mut _p, Some(add_155), (&mut z as *mut i32), 5_u64)
    }) == (107))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_164(
                c"s.combine(std::pair<int, int>(1, 2), add, &z, 5UL) == 107".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((z) == (6)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_164(
                c"z == 6".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut a: Access_157 = <Access_157>::default();
    let mut cs: *const S = (&mut s as *mut S).cast_const();
    let cr: *const S = &s;
    (if ((unsafe { Access_157::get_1(&mut a, (&mut s as *mut S)) }) == (100)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_164(
                c"a.get(&s) == 100".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { Access_157::get_2(&mut a, cs) }) == (101)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_164(
                c"a.get(cs) == 101".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { Access_157::ref_3(&mut a, &mut s) }) == (102)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_164(
                c"a.ref(s) == 102".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe {
        let _r: *const S = cr;
        Access_157::ref_4(&mut a, _r)
    }) == (103))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_164(
                c"a.ref(cr) == 103".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut b: Box_ = Box_ { v: 4 };
    (if ((b.v) == (4)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_164(
                c"b.v == 4".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    return 0;
}
pub unsafe fn __assert_fail_164(
    mut __assertion: *const libc::c_char,
    mut __file: *const libc::c_char,
    mut __line: u32,
    mut __function: *const libc::c_char,
) {
    unimplemented!()
}
pub unsafe fn __cpp2rust_init_globals() {}
