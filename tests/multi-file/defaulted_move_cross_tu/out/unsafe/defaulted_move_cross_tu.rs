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
impl integral_constant_0 {}
impl integral_constant_0 {}
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
pub struct __or__10 {}
impl Clone for __or__10 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__10 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __or__11 {}
impl Clone for __or__11 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__11 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
pub struct is_reference_16 {}
impl Clone for is_reference_16 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_16 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_function_17 {}
impl Clone for is_function_17 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_17 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_void_18 {}
impl Clone for is_void_18 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_18 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_void_19 {}
impl Clone for is_void_19 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_19 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_cv_20 {}
impl Clone for remove_cv_20 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_20 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_cv_21 {}
impl Clone for remove_cv_21 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_21 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_cv_22 {}
impl Clone for remove_cv_22 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_22 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_cv_23 {}
impl Clone for remove_cv_23 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_23 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_cv_24 {}
impl Clone for remove_cv_24 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_24 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_cv_25 {}
impl Clone for remove_cv_25 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_25 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_cv_26 {}
impl Clone for remove_cv_26 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_26 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_const_27 {}
impl Clone for is_const_27 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_27 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_const_28 {}
impl Clone for is_const_28 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_28 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_const_29 {}
impl Clone for is_const_29 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_29 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_const_30 {}
impl Clone for is_const_30 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_30 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_const_31 {}
impl Clone for is_const_31 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_31 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_array_unknown_bounds_32 {}
impl Clone for __is_array_unknown_bounds_32 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_32 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integral_helper_33 {}
impl Clone for __is_integral_helper_33 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_33 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integral_helper_34 {}
impl Clone for __is_integral_helper_34 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_34 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integral_helper_35 {}
impl Clone for __is_integral_helper_35 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_35 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integral_helper_36 {}
impl Clone for __is_integral_helper_36 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_36 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integral_helper_37 {}
impl Clone for __is_integral_helper_37 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_37 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integral_helper_38 {}
impl Clone for __is_integral_helper_38 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_38 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integral_helper_39 {}
impl Clone for __is_integral_helper_39 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_39 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_integral_40 {}
impl Clone for is_integral_40 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_40 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_integral_41 {}
impl Clone for is_integral_41 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_41 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_integral_42 {}
impl Clone for is_integral_42 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_42 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_integral_43 {}
impl Clone for is_integral_43 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_43 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_integral_44 {}
impl Clone for is_integral_44 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_44 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_integral_45 {}
impl Clone for is_integral_45 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_45 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_integral_46 {}
impl Clone for is_integral_46 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_46 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_arithmetic_47 {}
impl Clone for is_arithmetic_47 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_arithmetic_47 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_scalar_48 {}
impl Clone for is_scalar_48 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_scalar_48 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_same_49 {}
impl Clone for is_same_49 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_49 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_volatile_50 {}
impl Clone for is_volatile_50 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_50 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_volatile_51 {}
impl Clone for is_volatile_51 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_51 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_volatile_52 {}
impl Clone for is_volatile_52 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_52 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_volatile_53 {}
impl Clone for is_volatile_53 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_53 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_volatile_54 {}
impl Clone for is_volatile_54 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_54 {
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
pub struct __is_destructible_safe_55 {}
impl Clone for __is_destructible_safe_55 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_destructible_safe_55 {
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
pub struct is_trivially_destructible_56 {}
impl Clone for is_trivially_destructible_56 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_trivially_destructible_56 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_reference_57 {}
impl Clone for remove_reference_57 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_57 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __cv_selector_58 {}
impl Clone for __cv_selector_58 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_58 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __cv_selector_59 {}
impl Clone for __cv_selector_59 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_59 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __cv_selector_60 {}
impl Clone for __cv_selector_60 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_60 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __cv_selector_61 {}
impl Clone for __cv_selector_61 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_61 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __match_cv_qualifiers_62 {}
impl Clone for __match_cv_qualifiers_62 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_62 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __match_cv_qualifiers_63 {}
impl Clone for __match_cv_qualifiers_63 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_63 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __match_cv_qualifiers_64 {}
impl Clone for __match_cv_qualifiers_64 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_64 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __match_cv_qualifiers_65 {}
impl Clone for __match_cv_qualifiers_65 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_65 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __match_cv_qualifiers_66 {}
impl Clone for __match_cv_qualifiers_66 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_66 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_unsigned_67 {}
impl Clone for __make_unsigned_67 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_67 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_unsigned_68 {}
impl Clone for __make_unsigned_68 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_68 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_unsigned_69 {}
impl Clone for __make_unsigned_69 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_69 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_unsigned_selector_70 {}
impl Clone for __make_unsigned_selector_70 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_70 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_unsigned_selector_71 {}
impl Clone for __make_unsigned_selector_71 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_71 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_unsigned_selector_72 {}
impl Clone for __make_unsigned_selector_72 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_72 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_unsigned_selector_73 {}
impl Clone for __make_unsigned_selector_73 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_73 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_unsigned_selector_74 {}
impl Clone for __make_unsigned_selector_74 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_74 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_unsigned_selector_75 {}
impl Clone for __make_unsigned_selector_75 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_75 {
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
pub struct _List_79 {}
impl Clone for _List_79 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_79 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut std___make_unsigned_selector_base__List_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long____size_80 : std::cell::LazyCell< usize > = std::cell::LazyCell::new(|| unsafe { unimplemented!() } ) ;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _List_81 {}
impl Clone for _List_81 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_81 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut std___make_unsigned_selector_base__List_unsigned_short__unsigned_int__unsigned_long__unsigned_long_long____size_82 : std::cell::LazyCell< usize > = std::cell::LazyCell::new(|| unsafe { unimplemented!() } ) ;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _List_83 {}
impl Clone for _List_83 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_83 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut std___make_unsigned_selector_base__List_unsigned_int__unsigned_long__unsigned_long_long____size_84 : std::cell::LazyCell< usize > = std::cell::LazyCell::new(|| unsafe { unimplemented!() } ) ;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _List_85 {}
impl Clone for _List_85 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_85 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _List_86 {}
impl Clone for _List_86 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_86 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _List_87 {}
impl Clone for _List_87 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_87 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __select_88 {}
impl Clone for __select_88 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_88 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __select_89 {}
impl Clone for __select_89 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_89 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __select_90 {}
impl Clone for __select_90 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_90 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __select_91 {}
impl Clone for __select_91 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_91 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __select_92 {}
impl Clone for __select_92 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_92 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_signed_93 {}
impl Clone for __make_signed_93 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_93 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_signed_94 {}
impl Clone for __make_signed_94 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_94 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_signed_selector_95 {}
impl Clone for __make_signed_selector_95 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_95 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_signed_selector_96 {}
impl Clone for __make_signed_selector_96 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_96 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_signed_selector_97 {}
impl Clone for __make_signed_selector_97 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_97 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_signed_selector_98 {}
impl Clone for __make_signed_selector_98 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_98 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_signed_selector_99 {}
impl Clone for __make_signed_selector_99 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_99 {
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
pub struct common_type_100 {}
impl Clone for common_type_100 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for common_type_100 {
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
pub struct __are_same_101 {}
impl Clone for __are_same_101 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_101 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_102 = u32;
pub const anon_102___value: anon_102 = 1;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __are_same_103 {}
impl Clone for __are_same_103 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_103 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_104 = u32;
pub const anon_104___value: anon_104 = 0;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __are_same_105 {}
impl Clone for __are_same_105 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_105 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_106 = u32;
pub const anon_106___value: anon_106 = 0;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __are_same_107 {}
impl Clone for __are_same_107 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_107 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_108 = u32;
pub const anon_108___value: anon_108 = 1;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __are_same_109 {}
impl Clone for __are_same_109 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_109 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_110 = u32;
pub const anon_110___value: anon_110 = 0;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __are_same_111 {}
impl Clone for __are_same_111 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_111 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_112 = u32;
pub const anon_112___value: anon_112 = 0;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_113 {}
impl Clone for __is_integer_113 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_113 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_114 = u32;
pub const anon_114___value: anon_114 = 1;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_115 {}
impl Clone for __is_integer_115 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_115 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_116 = u32;
pub const anon_116___value: anon_116 = 1;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_117 {}
impl Clone for __is_integer_117 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_117 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_118 = u32;
pub const anon_118___value: anon_118 = 0;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_119 {}
impl Clone for __is_integer_119 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_119 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_120 = u32;
pub const anon_120___value: anon_120 = 0;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_121 {}
impl Clone for __is_integer_121 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_121 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_122 = u32;
pub const anon_122___value: anon_122 = 0;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_byte_123 {}
impl Clone for __is_byte_123 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_byte_123 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_124 = u32;
pub const anon_124___value: anon_124 = 0;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __memcpyable_integer_125 {}
impl Clone for __memcpyable_integer_125 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __memcpyable_integer_125 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_nonstrict_126 {}
impl Clone for __is_integer_nonstrict_126 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_126 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_127 = u32;
pub const anon_127___width: anon_127 = 64;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_nonstrict_128 {}
impl Clone for __is_integer_nonstrict_128 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_128 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_129 = u32;
pub const anon_129___width: anon_129 = 64;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_integer_130 {}
impl Clone for __numeric_traits_integer_130 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_130 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut __gnu_cxx___numeric_traits_integer_long____is_signed_131: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_long____digits_132: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_long____max_133: std::cell::LazyCell<i64> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_integer_134 {}
impl Clone for __numeric_traits_integer_134 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_134 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut __gnu_cxx___numeric_traits_integer_unsigned_long____is_signed_135:
    std::cell::LazyCell<bool> = std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_unsigned_long____digits_136: std::cell::LazyCell<
    i32,
> = std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_unsigned_long____max_137: std::cell::LazyCell<
    u64,
> = std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_floating_138 {}
impl Clone for __numeric_traits_floating_138 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_138 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_floating_139 {}
impl Clone for __numeric_traits_floating_139 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_139 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_floating_140 {}
impl Clone for __numeric_traits_floating_140 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_140 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_141 {}
impl Clone for __numeric_traits_141 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_141 {
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
pub static mut piecewise_construct_142: std::cell::LazyCell<std_piecewise_construct_t> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
pub struct tuple_143 {}
impl Clone for tuple_143 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for tuple_143 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl tuple_143 {}
impl tuple_143 {}
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
pub struct iterator_144 {}
impl Clone for iterator_144 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_144 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct iterator_145 {}
impl Clone for iterator_145 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_145 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl iterator_145 {}
impl iterator_145 {}
impl iterator_145 {}
impl iterator_145 {
    pub unsafe fn copy_assign(&mut self, _a0: *const iterator_145) -> *mut iterator_145 {
        unimplemented!()
    }
}
impl iterator_145 {
    pub unsafe fn move_assign(&mut self, _a0: *mut iterator_145) -> *mut iterator_145 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct iterator_146 {}
impl Clone for iterator_146 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_146 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl iterator_146 {
    pub unsafe fn copy_assign(&mut self, _a0: *const iterator_146) -> *mut iterator_146 {
        unimplemented!()
    }
}
impl iterator_146 {
    pub unsafe fn move_assign(&mut self, _a0: *mut iterator_146) -> *mut iterator_146 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct iterator_147 {}
impl Clone for iterator_147 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_147 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl iterator_147 {
    pub unsafe fn copy_assign(&mut self, _a0: *const iterator_147) -> *mut iterator_147 {
        unimplemented!()
    }
}
impl iterator_147 {
    pub unsafe fn move_assign(&mut self, _a0: *mut iterator_147) -> *mut iterator_147 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct iterator_traits_148 {}
impl Clone for iterator_traits_148 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_148 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct iterator_traits_149 {}
impl Clone for iterator_traits_149 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_149 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct iterator_traits_150 {}
impl Clone for iterator_traits_150 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_150 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct iterator_traits_151 {}
impl Clone for iterator_traits_151 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_151 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __iterator_traits_152 {}
impl Clone for __iterator_traits_152 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __iterator_traits_152 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __iterator_traits_153 {}
impl Clone for __iterator_traits_153 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __iterator_traits_153 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __iterator_traits_154 {}
impl Clone for __iterator_traits_154 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __iterator_traits_154 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct reverse_iterator_155 {}
impl Clone for reverse_iterator_155 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for reverse_iterator_155 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_155 {
    pub unsafe fn copy_from(__x: *const reverse_iterator_155) -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_155 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const reverse_iterator_155,
    ) -> *mut reverse_iterator_155 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct reverse_iterator_156 {}
impl Clone for reverse_iterator_156 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for reverse_iterator_156 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_156 {
    pub unsafe fn copy_from(__x: *const reverse_iterator_156) -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_156 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const reverse_iterator_156,
    ) -> *mut reverse_iterator_156 {
        unimplemented!()
    }
}
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
impl __gnu_cxx___ops__Iter_less_val {}
impl __gnu_cxx___ops__Iter_less_val {}
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
pub struct __new_allocator_157 {}
impl Clone for __new_allocator_157 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_157 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_157 {
    pub unsafe fn copy_from(_a0: *const __new_allocator_157) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_157 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const __new_allocator_157,
    ) -> *mut __new_allocator_157 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __new_allocator_158 {}
impl Clone for __new_allocator_158 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_158 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_158 {
    pub unsafe fn copy_from(_a0: *const __new_allocator_158) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_158 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const __new_allocator_158,
    ) -> *mut __new_allocator_158 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __new_allocator_159 {}
impl Clone for __new_allocator_159 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_159 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_159 {
    pub unsafe fn new() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_159 {
    pub unsafe fn copy_from(_a0: *const __new_allocator_159) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_159 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const __new_allocator_159,
    ) -> *mut __new_allocator_159 {
        unimplemented!()
    }
}
impl __new_allocator_159 {
    pub unsafe fn allocate(
        &mut self,
        mut __n: u64,
        mut _a1: Option<*const ::libc::c_void>,
    ) -> *mut i32 {
        unimplemented!()
    }
}
impl __new_allocator_159 {
    pub unsafe fn deallocate(&mut self, mut __p: *mut i32, mut __n: u64) {
        unimplemented!()
    }
}
impl __new_allocator_159 {
    pub unsafe fn max_size(&self) -> u64 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct allocator_160 {}
impl Clone for allocator_160 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_160 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_160 {}
impl allocator_160 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct allocator_161 {}
impl Clone for allocator_161 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_161 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_161 {
    pub unsafe fn new() -> Self {
        unimplemented!()
    }
}
impl allocator_161 {
    pub unsafe fn copy_from(__a: *const allocator_161) -> Self {
        unimplemented!()
    }
}
impl allocator_161 {
    pub unsafe fn copy_assign(&mut self, _a0: *const allocator_161) -> *mut allocator_161 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct allocator_traits_162 {}
impl Clone for allocator_traits_162 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_traits_162 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_traits_162 {
    pub unsafe fn allocate_1(__a: *mut allocator_161, mut __n: u64) -> *mut i32 {
        unimplemented!()
    }
}
impl allocator_traits_162 {
    pub unsafe fn deallocate(__a: *mut allocator_161, mut __p: *mut i32, mut __n: u64) {
        unimplemented!()
    }
}
impl allocator_traits_162 {
    pub unsafe fn max_size(__a: *const allocator_161) -> u64 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __alloc_traits_164 {}
impl Clone for __alloc_traits_164 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_164 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct rebind_165 {}
impl Clone for rebind_165 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_165 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct _UninitDestroyGuard_166 {
    pub _M_first: *mut i32,
    pub _M_cur: *mut *mut i32,
}
impl Clone for _UninitDestroyGuard_166 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _UninitDestroyGuard_166 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _UninitDestroyGuard_166 {
    pub unsafe fn new(__first: *mut *mut i32) -> Self {
        unimplemented!()
    }
}
impl _UninitDestroyGuard_166 {
    pub unsafe fn release(&mut self) {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct initializer_list_167 {}
impl Clone for initializer_list_167 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for initializer_list_167 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl initializer_list_167 {
    pub unsafe fn size(&self) -> u64 {
        unimplemented!()
    }
}
impl initializer_list_167 {
    pub unsafe fn begin(&self) -> *const bool {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct initializer_list_168 {}
impl Clone for initializer_list_168 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for initializer_list_168 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl initializer_list_168 {}
impl initializer_list_168 {}
#[derive(VaArg, FnPtrArg)]
pub struct _Vector_base_169 {
    pub _M_impl: std__Vector_base_int__std_allocator_int____Vector_impl,
}
impl Clone for _Vector_base_169 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Vector_base_169 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct std__Vector_base_int__std_allocator_int____Vector_impl_data {
    pub _M_start: *mut i32,
    pub _M_finish: *mut i32,
    pub _M_end_of_storage: *mut i32,
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
    pub unsafe fn new() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_int__std_allocator_int____Vector_impl_data {
    pub unsafe fn move_from(
        __x: *mut std__Vector_base_int__std_allocator_int____Vector_impl_data,
    ) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
    pub unsafe fn new_1(__a: *const allocator_161) -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_int__std_allocator_int____Vector_impl {
    pub unsafe fn move_from(
        __x: *mut std__Vector_base_int__std_allocator_int____Vector_impl,
    ) -> Self {
        unimplemented!()
    }
}
impl _Vector_base_169 {
    pub unsafe fn _M_get_Tp_allocator_1(&mut self) -> *mut allocator_161 {
        unimplemented!()
    }
}
impl _Vector_base_169 {
    pub unsafe fn new_1(mut __n: usize, __a: *const allocator_161) -> Self {
        unimplemented!()
    }
}
impl _Vector_base_169 {
    pub unsafe fn move_from(_a0: *mut _Vector_base_169) -> Self {
        unimplemented!()
    }
}
impl _Vector_base_169 {
    pub unsafe fn _M_allocate(&mut self, mut __n: usize) -> *mut i32 {
        unimplemented!()
    }
}
impl _Vector_base_169 {
    pub unsafe fn _M_deallocate(&mut self, mut __p: *mut i32, mut __n: usize) {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct vector_170 {}
impl Clone for vector_170 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for vector_170 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl vector_170 {
    pub unsafe fn new_1(
        mut __n: u64,
        __value: *const i32,
        __a: Option<*const allocator_161>,
    ) -> Self {
        unimplemented!()
    }
}
impl vector_170 {
    pub unsafe fn copy_from(__x: *const vector_170) -> Self {
        unimplemented!()
    }
}
impl vector_170 {
    pub unsafe fn move_from(_a0: *mut vector_170) -> Self {
        unimplemented!()
    }
}
impl vector_170 {
    pub unsafe fn copy_assign(&mut self, __x: *const vector_170) -> *mut vector_170 {
        unimplemented!()
    }
}
impl vector_170 {
    pub unsafe fn move_assign(&mut self, __x: *mut vector_170) -> *mut vector_170 {
        unimplemented!()
    }
}
impl vector_170 {
    pub unsafe fn size(&self) -> u64 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_171 {}
impl Clone for __hash_base_171 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_171 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_172 {}
impl Clone for __hash_base_172 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_172 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_173 {}
impl Clone for __hash_base_173 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_173 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_174 {}
impl Clone for __hash_base_174 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_174 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_175 {}
impl Clone for __hash_base_175 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_175 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_176 {}
impl Clone for __hash_base_176 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_176 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_177 {}
impl Clone for __hash_base_177 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_177 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_178 {}
impl Clone for __hash_base_178 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_178 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_179 {}
impl Clone for __hash_base_179 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_179 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_180 {}
impl Clone for __hash_base_180 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_180 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_181 {}
impl Clone for __hash_base_181 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_181 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_182 {}
impl Clone for __hash_base_182 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_182 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_183 {}
impl Clone for __hash_base_183 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_183 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_184 {}
impl Clone for __hash_base_184 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_184 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_185 {}
impl Clone for __hash_base_185 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_185 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_186 {}
impl Clone for __hash_base_186 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_186 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_187 {}
impl Clone for __hash_base_187 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_187 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_188 {}
impl Clone for __hash_base_188 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_188 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_189 {}
impl Clone for __hash_base_189 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_189 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_190 {}
impl Clone for __hash_base_190 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_190 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_191 {}
impl Clone for __hash_base_191 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_191 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct hash_192 {}
impl Clone for hash_192 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for hash_192 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
    pub unsafe fn hash(
        mut __ptr: *const ::libc::c_void,
        mut __clength: usize,
        mut __seed: Option<usize>,
    ) -> usize {
        unimplemented!()
    }
}
impl std__Hash_impl {
    pub unsafe fn hash_193(__val: *const f32) -> usize {
        unimplemented!()
    }
}
impl std__Hash_impl {
    pub unsafe fn hash_194(__val: *const f64) -> usize {
        unimplemented!()
    }
}
pub type anon_195 = u32;
pub const anon_195__S_word_bit: anon_195 = 64;
#[derive(VaArg, FnPtrArg, Copy)]
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
    pub unsafe fn copy_from(_a0: *const std__Bit_reference) -> Self {
        unimplemented!()
    }
}
impl std__Bit_reference {
    pub unsafe fn to_bool(&self) -> bool {
        unimplemented!()
    }
}
impl std__Bit_reference {
    pub unsafe fn operator_assign_5(&mut self, mut __x: bool) -> *mut std__Bit_reference {
        unimplemented!()
    }
}
impl std__Bit_reference {
    pub unsafe fn copy_assign(
        &mut self,
        __x: *const std__Bit_reference,
    ) -> *mut std__Bit_reference {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct std__Bit_iterator_base {
    pub _M_p: *mut u64,
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
    pub unsafe fn _M_assume_normalized(&self) {
        unimplemented!()
    }
}
impl std__Bit_iterator_base {
    pub unsafe fn new(mut __x: *mut u64, mut __y: u32) -> Self {
        unimplemented!()
    }
}
impl std__Bit_iterator_base {
    pub unsafe fn _M_bump_up(&mut self) {
        unimplemented!()
    }
}
impl std__Bit_iterator_base {
    pub unsafe fn _M_bump_down(&mut self) {
        unimplemented!()
    }
}
impl std__Bit_iterator_base {
    pub unsafe fn _M_incr(&mut self, mut __i: i64) {
        unimplemented!()
    }
}
impl std__Bit_iterator_base {}
impl std__Bit_iterator_base {}
impl std__Bit_iterator_base {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const std__Bit_iterator_base,
    ) -> *mut std__Bit_iterator_base {
        unimplemented!()
    }
}
impl std__Bit_iterator_base {
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut std__Bit_iterator_base,
    ) -> *mut std__Bit_iterator_base {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
    pub unsafe fn new_1(mut __x: *mut u64, mut __y: u32) -> Self {
        unimplemented!()
    }
}
impl std__Bit_iterator {
    pub unsafe fn operator_deref(&self) -> std__Bit_reference {
        unimplemented!()
    }
}
impl std__Bit_iterator {
    pub unsafe fn operator_add_assign(&mut self, mut __i: i64) -> *mut std__Bit_iterator {
        unimplemented!()
    }
}
impl std__Bit_iterator {
    pub unsafe fn operator_sub_assign(&mut self, mut __i: i64) -> *mut std__Bit_iterator {
        unimplemented!()
    }
}
impl std__Bit_iterator {}
impl std__Bit_iterator {}
impl std__Bit_iterator {
    pub unsafe fn copy_assign(&mut self, _a0: *const std__Bit_iterator) -> *mut std__Bit_iterator {
        unimplemented!()
    }
}
impl std__Bit_iterator {
    pub unsafe fn move_assign(&mut self, _a0: *mut std__Bit_iterator) -> *mut std__Bit_iterator {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
impl std__Bit_const_iterator {
    pub unsafe fn operator_deref(&self) -> bool {
        unimplemented!()
    }
}
impl std__Bit_const_iterator {
    pub unsafe fn operator_add_assign(&mut self, mut __i: i64) -> *mut std__Bit_const_iterator {
        unimplemented!()
    }
}
impl std__Bit_const_iterator {
    pub unsafe fn operator_sub_assign(&mut self, mut __i: i64) -> *mut std__Bit_const_iterator {
        unimplemented!()
    }
}
impl std__Bit_const_iterator {}
impl std__Bit_const_iterator {}
impl std__Bit_const_iterator {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const std__Bit_const_iterator,
    ) -> *mut std__Bit_const_iterator {
        unimplemented!()
    }
}
impl std__Bit_const_iterator {
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut std__Bit_const_iterator,
    ) -> *mut std__Bit_const_iterator {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct binary_function_196 {}
impl Clone for binary_function_196 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for binary_function_196 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct greater_197 {}
impl Clone for greater_197 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for greater_197 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct less_198 {}
impl Clone for less_198 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for less_198 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct greater_equal_199 {}
impl Clone for greater_equal_199 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for greater_equal_199 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct less_equal_200 {}
impl Clone for less_equal_200 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for less_equal_200 {
    fn default() -> Self {
        unimplemented!()
    }
}
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
#[derive(VaArg, FnPtrArg, Copy)]
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
pub static mut allocator_arg_201: std::cell::LazyCell<std_allocator_arg_t> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg)]
pub struct std___uses_alloc0 {
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
#[repr(C)]
#[derive(MoveCtorUnsafe, FnPtrArg, Default)]
pub struct S {
    pub v: vector_170,
    pub n: [i32; 2],
}
impl S {
    pub unsafe fn new(mut x: i32) -> Self {
        let mut this = Self {
            v: vector_170::new_1({ (x as u64) }, { &x }, None),
            n: [x, ((x) + (1))],
        };
        this
    }
    pub unsafe fn move_from(_a0: *mut S) -> Self {
        let mut this = Self {
            v: vector_170::move_from({ &mut (*_a0).v }),
            n: std::array::from_fn::<_, 2, _>(|__i: usize| (*_a0).n[(__i)]),
        };
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut S) -> *mut S {
        self.v = ((*_a0).v).clone();
        (unsafe {
            let _arg0: *mut ::libc::c_void =
                ((&mut self.n as *mut [i32; 2]) as *mut ::libc::c_void);
            let _arg1: *const ::libc::c_void =
                ((&mut (*_a0).n as *mut [i32; 2]) as *const ::libc::c_void);
            __builtin_memcpy_202(_arg0, _arg1, 8_usize)
        });
        return &mut (*(self as *mut S));
    }
}
pub unsafe fn sum_203(s: *const S) -> i32 {
    return ((((unsafe { vector_170::size(&(*s).v) }) as i32) + ((*s).n[(0) as usize]))
        + ((*s).n[(1) as usize]));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut s: S = S::new({ 2 });
    (if ((unsafe { sum_203(&s) }) == (7)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_204(
                c"sum(s) == 7".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { shuffle_205(3) }) == (10)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_204(
                c"shuffle(3) == 10".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    return 0;
}
impl integral_constant_2 {}
impl integral_constant_2 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_reference_206 {}
impl Clone for remove_reference_206 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_206 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_reference_207 {}
impl Clone for remove_reference_207 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_207 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_reference_208 {}
impl Clone for remove_reference_208 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_208 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_reference_209 {}
impl Clone for remove_reference_209 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_209 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
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
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __normal_iterator_211 {}
impl Clone for __normal_iterator_211 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __normal_iterator_211 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_211 {
    pub unsafe fn new_1(__i: *const *const i32) -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_211 {
    pub unsafe fn base(&self) -> *const *const i32 {
        unimplemented!()
    }
}
impl __normal_iterator_211 {}
impl __normal_iterator_211 {}
impl __alloc_traits_164 {
    pub unsafe fn _S_propagate_on_move_assign() -> bool {
        unimplemented!()
    }
}
impl __alloc_traits_164 {
    pub unsafe fn _S_always_equal() -> bool {
        unimplemented!()
    }
}
impl __alloc_traits_164 {
    pub unsafe fn _S_nothrow_move() -> bool {
        unimplemented!()
    }
}
impl std__Vector_base_int__std_allocator_int____Vector_impl_data {
    pub unsafe fn _M_copy_data(
        &mut self,
        __x: *const std__Vector_base_int__std_allocator_int____Vector_impl_data,
    ) {
        unimplemented!()
    }
}
impl std__Vector_base_int__std_allocator_int____Vector_impl_data {
    pub unsafe fn _M_swap_data(
        &mut self,
        __x: *mut std__Vector_base_int__std_allocator_int____Vector_impl_data,
    ) {
        unimplemented!()
    }
}
impl _Vector_base_169 {
    pub unsafe fn _M_get_Tp_allocator_2(&self) -> *const allocator_161 {
        unimplemented!()
    }
}
impl _Vector_base_169 {
    pub unsafe fn get_allocator(&self) -> allocator_161 {
        unimplemented!()
    }
}
impl _Vector_base_169 {
    pub unsafe fn new_2(__a: *const allocator_161) -> Self {
        unimplemented!()
    }
}
impl vector_170 {
    pub unsafe fn new_2(__a: *const allocator_161) -> Self {
        unimplemented!()
    }
}
impl vector_170 {
    pub unsafe fn begin_27(&self) -> __normal_iterator_211 {
        unimplemented!()
    }
}
impl vector_170 {
    pub unsafe fn end_29(&self) -> __normal_iterator_211 {
        unimplemented!()
    }
}
impl vector_170 {
    pub unsafe fn empty(&self) -> bool {
        unimplemented!()
    }
}
pub unsafe fn shuffle_205(mut x: i32) -> i32 {
    let mut a: S = S::new({ x });
    let mut b: S = S::move_from({ &mut a });
    (if (unsafe { vector_170::empty(&a.v) }) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_204(
                c"a.v.empty()".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int shuffle(int)".as_ptr(),
            )
        });
    });
    let mut c: S = S::new({ 1 });
    (unsafe { S::move_assign(&mut c, &mut b) });
    (if (unsafe { vector_170::empty(&b.v) }) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_204(
                c"b.v.empty()".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int shuffle(int)".as_ptr(),
            )
        });
    });
    return (unsafe { sum_203(&c) });
}
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct std___uses_alloc0__Sink;
pub unsafe fn __builtin_memcpy_202(
    mut _a0: *mut ::libc::c_void,
    mut _a1: *const ::libc::c_void,
    mut _a2: usize,
) -> *mut ::libc::c_void {
    unimplemented!()
}
pub unsafe fn __assert_fail_204(
    mut __assertion: *const libc::c_char,
    mut __file: *const libc::c_char,
    mut __line: u32,
    mut __function: *const libc::c_char,
) {
    unimplemented!()
}
pub unsafe fn __cpp2rust_init_globals() {}
