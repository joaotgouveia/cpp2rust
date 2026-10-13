extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[derive(VaArg, FnPtrArg)]
pub struct max_align_t {
    pub __clang_max_align_nonce1: i64,
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
pub struct __not__30 {}
impl Clone for __not__30 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__30 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __not__31 {}
impl Clone for __not__31 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__31 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __not__32 {}
impl Clone for __not__32 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__32 {
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
pub struct is_reference_35 {}
impl Clone for is_reference_35 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_35 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
pub struct is_void_39 {}
impl Clone for is_void_39 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_39 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_void_40 {}
impl Clone for is_void_40 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_40 {
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_cv_44 {}
impl Clone for remove_cv_44 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_44 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_cv_45 {}
impl Clone for remove_cv_45 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_45 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_cv_46 {}
impl Clone for remove_cv_46 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_46 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_const_49 {}
impl Clone for is_const_49 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_49 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_const_50 {}
impl Clone for is_const_50 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_50 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_const_51 {}
impl Clone for is_const_51 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_51 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_array_unknown_bounds_53 {}
impl Clone for __is_array_unknown_bounds_53 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_53 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_pointer_64 {}
impl Clone for is_pointer_64 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_pointer_64 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_volatile_65 {}
impl Clone for is_volatile_65 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_65 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
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
pub struct is_constructible_70 {}
impl Clone for is_constructible_70 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_constructible_70 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_constructible_71 {}
impl Clone for is_constructible_71 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_constructible_71 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_constructible_72 {}
impl Clone for is_constructible_72 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_constructible_72 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_constructible_73 {}
impl Clone for is_constructible_73 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_constructible_73 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_default_constructible_74 {}
impl Clone for is_default_constructible_74 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_default_constructible_74 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_default_constructible_75 {}
impl Clone for is_default_constructible_75 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_default_constructible_75 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_copy_assignable_76 {}
impl Clone for is_copy_assignable_76 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_copy_assignable_76 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_copy_assignable_77 {}
impl Clone for is_copy_assignable_77 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_copy_assignable_77 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_move_assignable_78 {}
impl Clone for is_move_assignable_78 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_move_assignable_78 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_move_assignable_79 {}
impl Clone for is_move_assignable_79 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_move_assignable_79 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_nothrow_move_assignable_80 {}
impl Clone for is_nothrow_move_assignable_80 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_move_assignable_80 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_nothrow_move_assignable_81 {}
impl Clone for is_nothrow_move_assignable_81 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_move_assignable_81 {
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
pub struct __is_implicitly_default_constructible_impl_82 {}
impl Clone for __is_implicitly_default_constructible_impl_82 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_implicitly_default_constructible_impl_82 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_implicitly_default_constructible_impl_83 {}
impl Clone for __is_implicitly_default_constructible_impl_83 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_implicitly_default_constructible_impl_83 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_implicitly_default_constructible_safe_84 {}
impl Clone for __is_implicitly_default_constructible_safe_84 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_implicitly_default_constructible_safe_84 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_implicitly_default_constructible_safe_85 {}
impl Clone for __is_implicitly_default_constructible_safe_85 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_implicitly_default_constructible_safe_85 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_implicitly_default_constructible_86 {}
impl Clone for __is_implicitly_default_constructible_86 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_implicitly_default_constructible_86 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_implicitly_default_constructible_87 {}
impl Clone for __is_implicitly_default_constructible_87 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_implicitly_default_constructible_87 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_convertible_88 {}
impl Clone for is_convertible_88 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_88 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_convertible_89 {}
impl Clone for is_convertible_89 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_89 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_convertible_90 {}
impl Clone for is_convertible_90 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_90 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_convertible_91 {}
impl Clone for is_convertible_91 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_91 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_reference_92 {}
impl Clone for remove_reference_92 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_92 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_reference_93 {}
impl Clone for remove_reference_93 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_93 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_reference_94 {}
impl Clone for remove_reference_94 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_94 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_reference_95 {}
impl Clone for remove_reference_95 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_95 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __cv_selector_96 {}
impl Clone for __cv_selector_96 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_96 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __cv_selector_97 {}
impl Clone for __cv_selector_97 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_97 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __cv_selector_98 {}
impl Clone for __cv_selector_98 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_98 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __cv_selector_99 {}
impl Clone for __cv_selector_99 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_99 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __match_cv_qualifiers_100 {}
impl Clone for __match_cv_qualifiers_100 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_100 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __match_cv_qualifiers_101 {}
impl Clone for __match_cv_qualifiers_101 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_101 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __match_cv_qualifiers_102 {}
impl Clone for __match_cv_qualifiers_102 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_102 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __match_cv_qualifiers_103 {}
impl Clone for __match_cv_qualifiers_103 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_103 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __match_cv_qualifiers_104 {}
impl Clone for __match_cv_qualifiers_104 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_104 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_unsigned_105 {}
impl Clone for __make_unsigned_105 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_105 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_unsigned_106 {}
impl Clone for __make_unsigned_106 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_106 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_unsigned_107 {}
impl Clone for __make_unsigned_107 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_107 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_unsigned_selector_108 {}
impl Clone for __make_unsigned_selector_108 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_108 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_unsigned_selector_109 {}
impl Clone for __make_unsigned_selector_109 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_109 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_unsigned_selector_110 {}
impl Clone for __make_unsigned_selector_110 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_110 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_unsigned_selector_111 {}
impl Clone for __make_unsigned_selector_111 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_111 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_unsigned_selector_112 {}
impl Clone for __make_unsigned_selector_112 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_112 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_unsigned_selector_113 {}
impl Clone for __make_unsigned_selector_113 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_113 {
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
pub struct _List_117 {}
impl Clone for _List_117 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_117 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut std___make_unsigned_selector_base__List_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long____size_118 : std::cell::LazyCell< usize > = std::cell::LazyCell::new(|| unsafe { unimplemented!() } ) ;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _List_119 {}
impl Clone for _List_119 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_119 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut std___make_unsigned_selector_base__List_unsigned_short__unsigned_int__unsigned_long__unsigned_long_long____size_120 : std::cell::LazyCell< usize > = std::cell::LazyCell::new(|| unsafe { unimplemented!() } ) ;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _List_121 {}
impl Clone for _List_121 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_121 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut std___make_unsigned_selector_base__List_unsigned_int__unsigned_long__unsigned_long_long____size_122 : std::cell::LazyCell< usize > = std::cell::LazyCell::new(|| unsafe { unimplemented!() } ) ;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _List_123 {}
impl Clone for _List_123 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_123 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _List_124 {}
impl Clone for _List_124 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_124 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _List_125 {}
impl Clone for _List_125 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_125 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __select_126 {}
impl Clone for __select_126 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_126 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __select_127 {}
impl Clone for __select_127 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_127 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __select_128 {}
impl Clone for __select_128 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_128 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __select_129 {}
impl Clone for __select_129 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_129 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __select_130 {}
impl Clone for __select_130 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_130 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_signed_131 {}
impl Clone for __make_signed_131 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_131 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_signed_132 {}
impl Clone for __make_signed_132 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_132 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_signed_selector_133 {}
impl Clone for __make_signed_selector_133 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_133 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_signed_selector_134 {}
impl Clone for __make_signed_selector_134 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_134 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_signed_selector_135 {}
impl Clone for __make_signed_selector_135 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_135 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_signed_selector_136 {}
impl Clone for __make_signed_selector_136 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_136 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_signed_selector_137 {}
impl Clone for __make_signed_selector_137 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_137 {
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
pub struct common_type_138 {}
impl Clone for common_type_138 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for common_type_138 {
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
pub struct pair_139 {
    pub first: *mut i32,
    pub second: u64,
}
impl Clone for pair_139 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pair_139 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl pair_139 {
    pub unsafe fn copy_from(_a0: *const pair_139) -> Self {
        unimplemented!()
    }
}
impl pair_139 {
    pub unsafe fn move_from(_a0: *mut pair_139) -> Self {
        unimplemented!()
    }
}
impl pair_139 {
    pub unsafe fn copy_assign(&mut self, __p: *const pair_139) -> *mut pair_139 {
        unimplemented!()
    }
}
impl pair_139 {
    pub unsafe fn move_assign(&mut self, __p: *mut pair_139) -> *mut pair_139 {
        unimplemented!()
    }
}
impl pair_139 {
    pub unsafe fn new_1() -> Self {
        unimplemented!()
    }
}
impl pair_139 {
    pub unsafe fn new_2(__x: *mut *mut i32, __y: *mut u64) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _PCC_140 {}
impl Clone for _PCC_140 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _PCC_140 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __pair_base_141 {}
impl Clone for __pair_base_141 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __pair_base_141 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut S_int__called_143: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { true });
pub static mut S_int__last_144: std::cell::LazyCell<pair_139> =
    std::cell::LazyCell::new(|| unsafe { pair_139::new_1() });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct S_142 {}
impl S_142 {
    pub unsafe fn record(&mut self, mut p: *mut i32, mut n: usize) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut S_int__called_143)) = false;
        (unsafe {
            let mut ___p: pair_139 =
                pair_139::new_2({ &mut p }, { &mut *(&raw mut n).cast::<u64>() });
            pair_139::move_assign(
                &mut (*std::cell::LazyCell::force_mut(&mut *&raw mut S_int__last_144)),
                &mut ___p,
            )
        });
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut x: i32 = 0;
    let mut t: S_142 = <S_142>::default();
    (if (*std::cell::LazyCell::force_mut(&mut *&raw mut S_int__called_143)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_145(
                c"S<int>::called".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (unsafe { S_142::record(&mut t, (&mut x as *mut i32), 3_usize) });
    (if !(*std::cell::LazyCell::force_mut(&mut *&raw mut S_int__called_143)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_145(
                c"!S<int>::called".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*std::cell::LazyCell::force_mut(&mut *&raw mut S_int__last_144)).first)
        == (&mut x as *mut i32))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_145(
                c"S<int>::last.first == &x".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*std::cell::LazyCell::force_mut(&mut *&raw mut S_int__last_144)).second) == (3_u64)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_145(
                c"S<int>::last.second == 3".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    return 0;
}
pub unsafe fn __assert_fail_145(
    mut __assertion: *const libc::c_char,
    mut __file: *const libc::c_char,
    mut __line: u32,
    mut __function: *const libc::c_char,
) {
    unimplemented!()
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const S_int__called_143);
    std::cell::LazyCell::force(&*&raw const S_int__last_144);
}
