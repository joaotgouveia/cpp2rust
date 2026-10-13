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
pub struct __type_identity_7 {}
impl Clone for __type_identity_7 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_7 {
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_12 {}
impl Clone for is_reference_12 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_12 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_13 {}
impl Clone for is_reference_13 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_13 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_14 {}
impl Clone for is_function_14 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_14 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_15 {}
impl Clone for is_function_15 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_15 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_16 {}
impl Clone for is_void_16 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_16 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_17 {}
impl Clone for is_void_17 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_17 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_19 {}
impl Clone for remove_cv_19 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_19 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_const_24 {}
impl Clone for is_const_24 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_24 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_const_25 {}
impl Clone for is_const_25 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_25 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_const_26 {}
impl Clone for is_const_26 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_26 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_29 {}
impl Clone for __is_array_unknown_bounds_29 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_29 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_30 {}
impl Clone for __is_array_unknown_bounds_30 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_30 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_31 {}
impl Clone for __is_integral_helper_31 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_31 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_32 {}
impl Clone for __is_integral_helper_32 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_32 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_36 {}
impl Clone for is_integral_36 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_36 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_37 {}
impl Clone for is_integral_37 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_37 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_38 {}
impl Clone for is_integral_38 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_38 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_39 {}
impl Clone for is_integral_39 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_39 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_volatile_41 {}
impl Clone for is_volatile_41 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_41 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_volatile_42 {}
impl Clone for is_volatile_42 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_42 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_volatile_43 {}
impl Clone for is_volatile_43 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_43 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_volatile_44 {}
impl Clone for is_volatile_44 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_44 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_volatile_45 {}
impl Clone for is_volatile_45 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_45 {
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
pub struct is_nothrow_copy_constructible_46 {}
impl Clone for is_nothrow_copy_constructible_46 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_copy_constructible_46 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_copy_constructible_47 {}
impl Clone for is_nothrow_copy_constructible_47 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_copy_constructible_47 {
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
pub struct __cv_selector_48 {}
impl Clone for __cv_selector_48 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_48 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __cv_selector_49 {}
impl Clone for __cv_selector_49 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_49 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __cv_selector_50 {}
impl Clone for __cv_selector_50 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_50 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __cv_selector_51 {}
impl Clone for __cv_selector_51 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_51 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_52 {}
impl Clone for __match_cv_qualifiers_52 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_52 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_53 {}
impl Clone for __match_cv_qualifiers_53 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_53 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_54 {}
impl Clone for __match_cv_qualifiers_54 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_54 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_55 {}
impl Clone for __match_cv_qualifiers_55 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_55 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_56 {}
impl Clone for __match_cv_qualifiers_56 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_56 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_57 {}
impl Clone for __make_unsigned_57 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_57 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_58 {}
impl Clone for __make_unsigned_58 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_58 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_59 {}
impl Clone for __make_unsigned_59 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_59 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_60 {}
impl Clone for __make_unsigned_selector_60 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_60 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_61 {}
impl Clone for __make_unsigned_selector_61 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_61 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_62 {}
impl Clone for __make_unsigned_selector_62 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_62 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_63 {}
impl Clone for __make_unsigned_selector_63 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_63 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_64 {}
impl Clone for __make_unsigned_selector_64 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_64 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_65 {}
impl Clone for __make_unsigned_selector_65 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_65 {
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
pub struct _List_69 {}
impl Clone for _List_69 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_69 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static std___make_unsigned_selector_base__List_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long____size_70 : Value<usize > = unimplemented!() ;
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_71 {}
impl Clone for _List_71 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_71 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static std___make_unsigned_selector_base__List_unsigned_short__unsigned_int__unsigned_long__unsigned_long_long____size_72 : Value<usize > = unimplemented!() ;
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_73 {}
impl Clone for _List_73 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_73 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static std___make_unsigned_selector_base__List_unsigned_int__unsigned_long__unsigned_long_long____size_74 : Value<usize > = unimplemented!() ;
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_75 {}
impl Clone for _List_75 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_75 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_76 {}
impl Clone for _List_76 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_76 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_77 {}
impl Clone for _List_77 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_77 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_78 {}
impl Clone for __select_78 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_78 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_79 {}
impl Clone for __select_79 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_79 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_80 {}
impl Clone for __select_80 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_80 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_81 {}
impl Clone for __select_81 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_81 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_82 {}
impl Clone for __select_82 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_82 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_83 {}
impl Clone for __make_signed_83 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_83 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_84 {}
impl Clone for __make_signed_84 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_84 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_85 {}
impl Clone for __make_signed_selector_85 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_85 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_86 {}
impl Clone for __make_signed_selector_86 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_86 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_87 {}
impl Clone for __make_signed_selector_87 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_87 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_88 {}
impl Clone for __make_signed_selector_88 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_88 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_89 {}
impl Clone for __make_signed_selector_89 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_89 {
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
pub struct common_type_90 {}
impl Clone for common_type_90 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for common_type_90 {
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
pub struct __are_same_91 {}
impl Clone for __are_same_91 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_91 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_92 = u32;
pub const anon_92___value: anon_92 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_93 {}
impl Clone for __are_same_93 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_93 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_94 = u32;
pub const anon_94___value: anon_94 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_95 {}
impl Clone for __are_same_95 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_95 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_96 = u32;
pub const anon_96___value: anon_96 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_97 {}
impl Clone for __are_same_97 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_97 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_98 = u32;
pub const anon_98___value: anon_98 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_99 {}
impl Clone for __are_same_99 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_99 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_100 = u32;
pub const anon_100___value: anon_100 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
pub const anon_102___value: anon_102 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_103 {}
impl Clone for __is_integer_103 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_103 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_104 = u32;
pub const anon_104___value: anon_104 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_105 {}
impl Clone for __is_integer_105 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_105 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_106 = u32;
pub const anon_106___value: anon_106 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_107 {}
impl Clone for __is_integer_107 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_107 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_108 = u32;
pub const anon_108___value: anon_108 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_109 {}
impl Clone for __is_integer_109 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_109 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_110 = u32;
pub const anon_110___value: anon_110 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_111 {}
impl Clone for __is_integer_111 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_111 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_112 = u32;
pub const anon_112___value: anon_112 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __memcpyable_integer_113 {}
impl Clone for __memcpyable_integer_113 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __memcpyable_integer_113 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_move_iterator_114 {}
impl Clone for __is_move_iterator_114 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_move_iterator_114 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_115 = u32;
pub const anon_115___value: anon_115 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_nonstrict_116 {}
impl Clone for __is_integer_nonstrict_116 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_116 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_117 = u32;
pub const anon_117___width: anon_117 = 64;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_nonstrict_118 {}
impl Clone for __is_integer_nonstrict_118 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_118 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_119 = u32;
pub const anon_119___width: anon_119 = 64;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_integer_120 {}
impl Clone for __numeric_traits_integer_120 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_120 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_long____is_signed_121: Value<bool> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_long____digits_122: Value<i32> = unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_long____max_123: Value<i64> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_integer_124 {}
impl Clone for __numeric_traits_integer_124 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_124 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_long____is_signed_125: Value<bool> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_long____digits_126: Value<i32> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_long____max_127: Value<u64> =
        unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_floating_128 {}
impl Clone for __numeric_traits_floating_128 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_128 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_floating_129 {}
impl Clone for __numeric_traits_floating_129 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_129 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_floating_130 {}
impl Clone for __numeric_traits_floating_130 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_130 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_131 {}
impl Clone for __numeric_traits_131 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_131 {
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
    pub static piecewise_construct_132: Value<std_piecewise_construct_t> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct tuple_133 {}
impl Clone for tuple_133 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for tuple_133 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl tuple_133 {}
impl tuple_133 {}
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
pub struct iterator_134 {}
impl Clone for iterator_134 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_134 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_135 {}
impl Clone for iterator_135 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_135 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl iterator_135 {}
impl iterator_135 {}
impl iterator_135 {}
impl iterator_135 {
    pub fn copy_assign(&self, _a0: Ptr<iterator_135>) -> Ptr<iterator_135> {
        unimplemented!()
    }
}
impl iterator_135 {
    pub fn move_assign(&self, _a0: Ptr<iterator_135>) -> Ptr<iterator_135> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_136 {}
impl Clone for iterator_136 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_136 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl iterator_136 {
    pub fn copy_assign(&self, _a0: Ptr<iterator_136>) -> Ptr<iterator_136> {
        unimplemented!()
    }
}
impl iterator_136 {
    pub fn move_assign(&self, _a0: Ptr<iterator_136>) -> Ptr<iterator_136> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_137 {}
impl Clone for iterator_137 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_137 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl iterator_137 {
    pub fn copy_assign(&self, _a0: Ptr<iterator_137>) -> Ptr<iterator_137> {
        unimplemented!()
    }
}
impl iterator_137 {
    pub fn move_assign(&self, _a0: Ptr<iterator_137>) -> Ptr<iterator_137> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_138 {}
impl Clone for iterator_traits_138 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_138 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_139 {}
impl Clone for iterator_traits_139 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_139 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_140 {}
impl Clone for iterator_traits_140 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_140 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __iterator_traits_141 {}
impl Clone for __iterator_traits_141 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __iterator_traits_141 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __iterator_traits_142 {}
impl Clone for __iterator_traits_142 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __iterator_traits_142 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct reverse_iterator_143 {}
impl Clone for reverse_iterator_143 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for reverse_iterator_143 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_143 {
    pub fn copy_from(__x: Ptr<reverse_iterator_143>) -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_143 {
    pub fn copy_assign(&self, _a0: Ptr<reverse_iterator_143>) -> Ptr<reverse_iterator_143> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct reverse_iterator_144 {}
impl Clone for reverse_iterator_144 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for reverse_iterator_144 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_144 {
    pub fn copy_from(__x: Ptr<reverse_iterator_144>) -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_144 {
    pub fn copy_assign(&self, _a0: Ptr<reverse_iterator_144>) -> Ptr<reverse_iterator_144> {
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
pub struct __new_allocator_145 {}
impl Clone for __new_allocator_145 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_145 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_145 {
    pub fn copy_from(_a0: Ptr<__new_allocator_145>) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_145 {
    pub fn copy_assign(&self, _a0: Ptr<__new_allocator_145>) -> Ptr<__new_allocator_145> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __new_allocator_146 {}
impl Clone for __new_allocator_146 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_146 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_146 {
    pub fn copy_from(_a0: Ptr<__new_allocator_146>) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_146 {
    pub fn copy_assign(&self, _a0: Ptr<__new_allocator_146>) -> Ptr<__new_allocator_146> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __new_allocator_147 {}
impl Clone for __new_allocator_147 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_147 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_147 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_147 {
    pub fn copy_from(_a0: Ptr<__new_allocator_147>) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_147 {
    pub fn copy_assign(&self, _a0: Ptr<__new_allocator_147>) -> Ptr<__new_allocator_147> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __new_allocator_148 {}
impl Clone for __new_allocator_148 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_148 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_148 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_148 {
    pub fn copy_from(_a0: Ptr<__new_allocator_148>) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_148 {
    pub fn copy_assign(&self, _a0: Ptr<__new_allocator_148>) -> Ptr<__new_allocator_148> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_149 {}
impl Clone for allocator_149 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_149 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_149 {}
impl allocator_149 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_150 {}
impl Clone for allocator_150 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_150 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_150 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl allocator_150 {
    pub fn copy_from(__a: Ptr<allocator_150>) -> Self {
        unimplemented!()
    }
}
impl allocator_150 {
    pub fn copy_assign(&self, _a0: Ptr<allocator_150>) -> Ptr<allocator_150> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_151 {}
impl Clone for allocator_151 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_151 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_151 {
    pub fn copy_from(__a: Ptr<allocator_151>) -> Self {
        unimplemented!()
    }
}
impl allocator_151 {
    pub fn copy_assign(&self, _a0: Ptr<allocator_151>) -> Ptr<allocator_151> {
        unimplemented!()
    }
}
impl allocator_151 {
    pub fn new_1(_a0: Ptr<allocator_150>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_traits_152 {}
impl Clone for allocator_traits_152 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_traits_152 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_traits_153 {}
impl Clone for allocator_traits_153 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_traits_153 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_traits_153 {
    pub fn allocate_1(__a: Ptr<allocator_151>, mut __n: u64) -> Ptr<u64> {
        unimplemented!()
    }
}
impl allocator_traits_153 {
    pub fn deallocate(__a: Ptr<allocator_151>, mut __p: Ptr<u64>, mut __n: u64) {
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
pub struct __alloc_traits_155 {}
impl Clone for __alloc_traits_155 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_155 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct rebind_156 {}
impl Clone for rebind_156 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_156 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __alloc_traits_157 {}
impl Clone for __alloc_traits_157 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_157 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct initializer_list_158 {}
impl Clone for initializer_list_158 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for initializer_list_158 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(40)]
pub struct vector_159 {}
impl Clone for vector_159 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for vector_159 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl vector_159 {
    pub fn copy_from(__x: Ptr<vector_159>) -> Self {
        unimplemented!()
    }
}
impl vector_159 {
    pub fn move_from(_a0: Ptr<vector_159>) -> Self {
        unimplemented!()
    }
}
impl vector_159 {
    pub fn new_1(__l: initializer_list_158, __a: Option<Ptr<allocator_150>>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_160 {}
impl Clone for __hash_base_160 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_160 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_161 {}
impl Clone for __hash_base_161 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_161 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_162 {}
impl Clone for __hash_base_162 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_162 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_163 {}
impl Clone for __hash_base_163 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_163 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_164 {}
impl Clone for __hash_base_164 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_164 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_165 {}
impl Clone for __hash_base_165 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_165 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_166 {}
impl Clone for __hash_base_166 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_166 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_167 {}
impl Clone for __hash_base_167 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_167 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_168 {}
impl Clone for __hash_base_168 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_168 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_169 {}
impl Clone for __hash_base_169 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_169 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_170 {}
impl Clone for __hash_base_170 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_170 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct hash_181 {}
impl Clone for hash_181 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for hash_181 {
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
    pub fn hash_182(__val: Ptr<f32>) -> usize {
        unimplemented!()
    }
}
impl std__Hash_impl {
    pub fn hash_183(__val: Ptr<f64>) -> usize {
        unimplemented!()
    }
}
pub type anon_184 = u32;
pub const anon_184__S_word_bit: anon_184 = 64;
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
    pub fn new_1() -> Self {
        unimplemented!()
    }
}
impl std__Bit_iterator {
    pub fn new_2(mut __x: Ptr<u64>, mut __y: u32) -> Self {
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
impl std__Bit_const_iterator {
    pub fn new_1(mut __x: Ptr<u64>, mut __y: u32) -> Self {
        unimplemented!()
    }
}
impl std__Bit_const_iterator {
    pub fn new_2(__x: Ptr<std__Bit_iterator>) -> Self {
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
#[byte_size(40)]
pub struct _Bvector_base_185 {}
impl Clone for _Bvector_base_185 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Bvector_base_185 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(40)]
pub struct std__Bvector_base_std_allocator_bool____Bvector_impl_data {
    #[offset(0)]
    #[byte_size(16)]
    pub _M_start: std__Bit_iterator,
    #[offset(16)]
    #[byte_size(16)]
    pub _M_finish: std__Bit_iterator,
    #[offset(32)]
    #[byte_size(8)]
    pub _M_end_of_storage: Ptr<u64>,
}
impl Clone for std__Bvector_base_std_allocator_bool____Bvector_impl_data {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std__Bvector_base_std_allocator_bool____Bvector_impl_data {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std__Bvector_base_std_allocator_bool____Bvector_impl_data {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl std__Bvector_base_std_allocator_bool____Bvector_impl_data {
    pub fn copy_from(_a0: Ptr<std__Bvector_base_std_allocator_bool____Bvector_impl_data>) -> Self {
        unimplemented!()
    }
}
impl std__Bvector_base_std_allocator_bool____Bvector_impl_data {
    pub fn copy_assign(
        &self,
        _a0: Ptr<std__Bvector_base_std_allocator_bool____Bvector_impl_data>,
    ) -> Ptr<std__Bvector_base_std_allocator_bool____Bvector_impl_data> {
        unimplemented!()
    }
}
impl std__Bvector_base_std_allocator_bool____Bvector_impl_data {
    pub fn move_from(__x: Ptr<std__Bvector_base_std_allocator_bool____Bvector_impl_data>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(40)]
pub struct std__Bvector_base_std_allocator_bool____Bvector_impl {}
impl Clone for std__Bvector_base_std_allocator_bool____Bvector_impl {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std__Bvector_base_std_allocator_bool____Bvector_impl {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std__Bvector_base_std_allocator_bool____Bvector_impl {
    pub fn new_1(__a: Ptr<allocator_151>) -> Self {
        unimplemented!()
    }
}
impl std__Bvector_base_std_allocator_bool____Bvector_impl {
    pub fn move_from(__x: Ptr<std__Bvector_base_std_allocator_bool____Bvector_impl>) -> Self {
        unimplemented!()
    }
}
impl _Bvector_base_185 {
    pub fn new_1(__a: Ptr<allocator_150>) -> Self {
        unimplemented!()
    }
}
impl _Bvector_base_185 {
    pub fn move_from(_a0: Ptr<_Bvector_base_185>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct binary_function_186 {}
impl Clone for binary_function_186 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for binary_function_186 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct greater_187 {}
impl Clone for greater_187 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for greater_187 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct less_188 {}
impl Clone for less_188 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for less_188 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct greater_equal_189 {}
impl Clone for greater_equal_189 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for greater_equal_189 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct less_equal_190 {}
impl Clone for less_equal_190 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for less_equal_190 {
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
    pub static allocator_arg_191: Value<std_allocator_arg_t> = unimplemented!();
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
    let v: Value<vector_159> = Rc::new(RefCell::new(vector_159::new_1(
        { __cpp2rust_unimplemented::<initializer_list_158>() },
        None,
    )));
    (if ({
        std__Bit_referenceImpl::to_bool(
            &Rc::new(RefCell::new(
                ({ vector_159Impl::operator_index_37(&v.as_pointer(), 0_u64) }),
            ))
            .as_pointer(),
        )
    }) {
        &(0);
    } else {
        ({
            __assert_fail_192(
                Ptr::<i8>::from_string_literal(b"v[0]"),
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
#[track_caller]
pub fn __cpp2rust_unimplemented<T>() -> T {
    unimplemented!()
}
pub fn __assert_fail_192(
    mut __assertion: Ptr<i8>,
    mut __file: Ptr<i8>,
    mut __line: u32,
    mut __function: Ptr<i8>,
) {
    unimplemented!()
}
pub trait __new_allocator_148Impl {
    fn allocate(&self, mut __n: u64, _a1: Option<AnyPtr>) -> Ptr<u64> {
        unimplemented!()
    }
    fn deallocate(&self, mut __p: Ptr<u64>, mut __n: u64) {
        unimplemented!()
    }
}
impl __new_allocator_148Impl for Ptr<__new_allocator_148> {}
pub trait initializer_list_158Impl {
    fn size(&self) -> u64 {
        unimplemented!()
    }
    fn begin(&self) -> Ptr<bool> {
        unimplemented!()
    }
    fn end(&self) -> Ptr<bool> {
        unimplemented!()
    }
}
impl initializer_list_158Impl for Ptr<initializer_list_158> {}
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
    fn operator_inc_5(&self) -> Ptr<std__Bit_iterator> {
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
pub trait std__Bvector_base_std_allocator_bool____Bvector_implImpl {
    fn _M_end_addr(&self) -> Ptr<u64> {
        unimplemented!()
    }
}
impl std__Bvector_base_std_allocator_bool____Bvector_implImpl
    for Ptr<std__Bvector_base_std_allocator_bool____Bvector_impl>
{
}
pub trait std__Bvector_base_std_allocator_bool____Bvector_impl_dataImpl {
    fn _M_reset(&self) {
        unimplemented!()
    }
}
impl std__Bvector_base_std_allocator_bool____Bvector_impl_dataImpl
    for Ptr<std__Bvector_base_std_allocator_bool____Bvector_impl_data>
{
}
pub trait vector_159Impl {
    fn copy_assign(&self, __x: Ptr<vector_159>) -> Ptr<vector_159> {
        unimplemented!()
    }
    fn move_assign(&self, __x: Ptr<vector_159>) -> Ptr<vector_159> {
        unimplemented!()
    }
    fn begin_21(&self) -> std__Bit_iterator {
        unimplemented!()
    }
    fn begin_22(&self) -> std__Bit_const_iterator {
        unimplemented!()
    }
    fn end_24(&self) -> std__Bit_const_iterator {
        unimplemented!()
    }
    fn size(&self) -> u64 {
        unimplemented!()
    }
    fn operator_index_37(&self, mut __n: u64) -> std__Bit_reference {
        unimplemented!()
    }
}
impl vector_159Impl for Ptr<vector_159> {}
pub fn __cpp2rust_init_globals() {}
