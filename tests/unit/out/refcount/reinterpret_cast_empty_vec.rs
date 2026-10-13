extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
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
pub struct is_reference_17 {}
impl Clone for is_reference_17 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_17 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_18 {}
impl Clone for is_reference_18 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_18 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_19 {}
impl Clone for is_function_19 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_19 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_20 {}
impl Clone for is_function_20 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_20 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_21 {}
impl Clone for is_void_21 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_21 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_22 {}
impl Clone for is_void_22 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_22 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_23 {}
impl Clone for is_void_23 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_23 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_27 {}
impl Clone for remove_cv_27 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_27 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_28 {}
impl Clone for remove_cv_28 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_28 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_const_32 {}
impl Clone for is_const_32 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_32 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_const_33 {}
impl Clone for is_const_33 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_33 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_34 {}
impl Clone for __is_array_unknown_bounds_34 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_34 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_35 {}
impl Clone for __is_array_unknown_bounds_35 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_35 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_40 {}
impl Clone for __is_integral_helper_40 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_40 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_arithmetic_46 {}
impl Clone for is_arithmetic_46 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_arithmetic_46 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_scalar_47 {}
impl Clone for is_scalar_47 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_scalar_47 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_48 {}
impl Clone for is_same_48 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_48 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_volatile_49 {}
impl Clone for is_volatile_49 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_49 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
pub struct __is_destructible_safe_54 {}
impl Clone for __is_destructible_safe_54 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_destructible_safe_54 {
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
pub struct is_nothrow_default_constructible_55 {}
impl Clone for is_nothrow_default_constructible_55 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_default_constructible_55 {
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __cv_selector_57 {}
impl Clone for __cv_selector_57 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_57 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_61 {}
impl Clone for __match_cv_qualifiers_61 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_61 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_66 {}
impl Clone for __make_unsigned_66 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_66 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_69 {}
impl Clone for __make_unsigned_selector_69 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_69 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
pub struct _List_78 {}
impl Clone for _List_78 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_78 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static std___make_unsigned_selector_base__List_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long____size_79 : Value<usize > = unimplemented!() ;
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_80 {}
impl Clone for _List_80 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_80 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static std___make_unsigned_selector_base__List_unsigned_short__unsigned_int__unsigned_long__unsigned_long_long____size_81 : Value<usize > = unimplemented!() ;
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_82 {}
impl Clone for _List_82 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_82 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static std___make_unsigned_selector_base__List_unsigned_int__unsigned_long__unsigned_long_long____size_83 : Value<usize > = unimplemented!() ;
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_84 {}
impl Clone for _List_84 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_84 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_87 {}
impl Clone for __select_87 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_87 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_92 {}
impl Clone for __make_signed_92 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_92 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_94 {}
impl Clone for __make_signed_selector_94 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_94 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
pub struct common_type_99 {}
impl Clone for common_type_99 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for common_type_99 {
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
pub struct __are_same_100 {}
impl Clone for __are_same_100 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_100 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_101 = u32;
pub const anon_101___value: anon_101 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_102 {}
impl Clone for __are_same_102 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_102 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_103 = u32;
pub const anon_103___value: anon_103 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_104 {}
impl Clone for __are_same_104 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_104 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_105 = u32;
pub const anon_105___value: anon_105 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_106 {}
impl Clone for __are_same_106 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_106 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_107 = u32;
pub const anon_107___value: anon_107 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_108 {}
impl Clone for __are_same_108 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_108 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_109 = u32;
pub const anon_109___value: anon_109 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_110 {}
impl Clone for __are_same_110 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_110 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_111 = u32;
pub const anon_111___value: anon_111 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_112 {}
impl Clone for __is_integer_112 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_112 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_113 = u32;
pub const anon_113___value: anon_113 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_114 {}
impl Clone for __is_integer_114 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_114 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_115 = u32;
pub const anon_115___value: anon_115 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_116 {}
impl Clone for __is_integer_116 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_116 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_117 = u32;
pub const anon_117___value: anon_117 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_118 {}
impl Clone for __is_integer_118 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_118 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_119 = u32;
pub const anon_119___value: anon_119 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_120 {}
impl Clone for __is_integer_120 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_120 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_121 = u32;
pub const anon_121___value: anon_121 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __memcpyable_integer_122 {}
impl Clone for __memcpyable_integer_122 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __memcpyable_integer_122 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_nonstrict_123 {}
impl Clone for __is_integer_nonstrict_123 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_123 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_124 = u32;
pub const anon_124___width: anon_124 = 64;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_nonstrict_125 {}
impl Clone for __is_integer_nonstrict_125 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_125 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_126 = u32;
pub const anon_126___width: anon_126 = 64;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_integer_127 {}
impl Clone for __numeric_traits_integer_127 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_127 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_long____is_signed_128: Value<bool> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_long____digits_129: Value<i32> = unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_long____max_130: Value<i64> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_integer_131 {}
impl Clone for __numeric_traits_integer_131 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_131 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_long____is_signed_132: Value<bool> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_long____digits_133: Value<i32> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_long____max_134: Value<u64> =
        unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_floating_135 {}
impl Clone for __numeric_traits_floating_135 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_135 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_floating_136 {}
impl Clone for __numeric_traits_floating_136 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_136 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_floating_137 {}
impl Clone for __numeric_traits_floating_137 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_137 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_138 {}
impl Clone for __numeric_traits_138 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_138 {
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
    pub static piecewise_construct_139: Value<std_piecewise_construct_t> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct tuple_140 {}
impl Clone for tuple_140 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for tuple_140 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl tuple_140 {}
impl tuple_140 {}
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
pub struct iterator_141 {}
impl Clone for iterator_141 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_141 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_142 {}
impl Clone for iterator_142 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_142 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl iterator_142 {}
impl iterator_142 {}
impl iterator_142 {}
impl iterator_142 {
    pub fn copy_assign(&self, _a0: Ptr<iterator_142>) -> Ptr<iterator_142> {
        unimplemented!()
    }
}
impl iterator_142 {
    pub fn move_assign(&self, _a0: Ptr<iterator_142>) -> Ptr<iterator_142> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_143 {}
impl Clone for iterator_143 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_143 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl iterator_143 {
    pub fn copy_assign(&self, _a0: Ptr<iterator_143>) -> Ptr<iterator_143> {
        unimplemented!()
    }
}
impl iterator_143 {
    pub fn move_assign(&self, _a0: Ptr<iterator_143>) -> Ptr<iterator_143> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
impl iterator_144 {
    pub fn copy_assign(&self, _a0: Ptr<iterator_144>) -> Ptr<iterator_144> {
        unimplemented!()
    }
}
impl iterator_144 {
    pub fn move_assign(&self, _a0: Ptr<iterator_144>) -> Ptr<iterator_144> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_145 {}
impl Clone for iterator_traits_145 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_145 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_146 {}
impl Clone for iterator_traits_146 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_146 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_147 {}
impl Clone for iterator_traits_147 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_147 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __iterator_traits_148 {}
impl Clone for __iterator_traits_148 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __iterator_traits_148 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __iterator_traits_149 {}
impl Clone for __iterator_traits_149 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __iterator_traits_149 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __ptr_traits_ptr_to_150 {}
impl Clone for __ptr_traits_ptr_to_150 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_150 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct pointer_traits_151 {}
impl Clone for pointer_traits_151 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_151 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct reverse_iterator_152 {}
impl Clone for reverse_iterator_152 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for reverse_iterator_152 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_152 {
    pub fn copy_from(__x: Ptr<reverse_iterator_152>) -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_152 {
    pub fn copy_assign(&self, _a0: Ptr<reverse_iterator_152>) -> Ptr<reverse_iterator_152> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct reverse_iterator_153 {}
impl Clone for reverse_iterator_153 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for reverse_iterator_153 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_153 {
    pub fn copy_from(__x: Ptr<reverse_iterator_153>) -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_153 {
    pub fn copy_assign(&self, _a0: Ptr<reverse_iterator_153>) -> Ptr<reverse_iterator_153> {
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
pub struct __new_allocator_154 {}
impl Clone for __new_allocator_154 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_154 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_154 {
    pub fn copy_from(_a0: Ptr<__new_allocator_154>) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_154 {
    pub fn copy_assign(&self, _a0: Ptr<__new_allocator_154>) -> Ptr<__new_allocator_154> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __new_allocator_155 {}
impl Clone for __new_allocator_155 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_155 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_155 {
    pub fn copy_from(_a0: Ptr<__new_allocator_155>) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_155 {
    pub fn copy_assign(&self, _a0: Ptr<__new_allocator_155>) -> Ptr<__new_allocator_155> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __new_allocator_156 {}
impl Clone for __new_allocator_156 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_156 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_156 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_156 {
    pub fn copy_from(_a0: Ptr<__new_allocator_156>) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_156 {
    pub fn copy_assign(&self, _a0: Ptr<__new_allocator_156>) -> Ptr<__new_allocator_156> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_157 {}
impl Clone for allocator_157 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_157 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_157 {}
impl allocator_157 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_158 {}
impl Clone for allocator_158 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_158 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_158 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl allocator_158 {
    pub fn copy_from(__a: Ptr<allocator_158>) -> Self {
        unimplemented!()
    }
}
impl allocator_158 {
    pub fn copy_assign(&self, _a0: Ptr<allocator_158>) -> Ptr<allocator_158> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_traits_159 {}
impl Clone for allocator_traits_159 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_traits_159 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_traits_159 {
    pub fn deallocate(__a: Ptr<allocator_158>, mut __p: Ptr<u32>, mut __n: u64) {
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
pub struct __alloc_traits_161 {}
impl Clone for __alloc_traits_161 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_161 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct rebind_162 {}
impl Clone for rebind_162 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_162 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct initializer_list_163 {}
impl Clone for initializer_list_163 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for initializer_list_163 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(24)]
pub struct _Vector_base_164 {
    #[offset(0)]
    #[byte_size(24)]
    pub _M_impl: std__Vector_base_unsigned_int__std_allocator_unsigned_int____Vector_impl,
}
impl Clone for _Vector_base_164 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Vector_base_164 {
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
    pub fn new_1() -> Self {
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
impl _Vector_base_164 {
    pub fn new_1() -> Self {
        unimplemented!()
    }
}
impl _Vector_base_164 {
    pub fn move_from(_a0: Ptr<_Vector_base_164>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(24)]
pub struct vector_165 {}
impl Clone for vector_165 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for vector_165 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl vector_165 {
    pub fn new_1() -> Self {
        unimplemented!()
    }
}
impl vector_165 {
    pub fn copy_from(__x: Ptr<vector_165>) -> Self {
        unimplemented!()
    }
}
impl vector_165 {
    pub fn move_from(_a0: Ptr<vector_165>) -> Self {
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct hash_187 {}
impl Clone for hash_187 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for hash_187 {
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
    pub fn hash_188(__val: Ptr<f32>) -> usize {
        unimplemented!()
    }
}
impl std__Hash_impl {
    pub fn hash_189(__val: Ptr<f64>) -> usize {
        unimplemented!()
    }
}
pub type anon_190 = u32;
pub const anon_190__S_word_bit: anon_190 = 64;
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
pub struct binary_function_191 {}
impl Clone for binary_function_191 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for binary_function_191 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct greater_192 {}
impl Clone for greater_192 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for greater_192 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct less_193 {}
impl Clone for less_193 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for less_193 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct greater_equal_194 {}
impl Clone for greater_equal_194 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for greater_equal_194 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct less_equal_195 {}
impl Clone for less_equal_195 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for less_equal_195 {
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
    pub static allocator_arg_196: Value<std_allocator_arg_t> = unimplemented!();
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
    let v: Value<vector_165> = Rc::new(RefCell::new(<vector_165>::default()));
    let mut bytes: Ptr<u16> =
        ({ vector_165Impl::data_55(&v.as_pointer()) }).reinterpret_cast::<u16>();
    return 0;
}
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct std___uses_alloc0__Sink;
pub trait _Vector_base_164Impl {
    fn _M_get_Tp_allocator_1(&self) -> Ptr<allocator_158> {
        unimplemented!()
    }
    fn _M_deallocate(&self, mut __p: Ptr<u32>, mut __n: usize) {
        unimplemented!()
    }
}
impl _Vector_base_164Impl for Ptr<_Vector_base_164> {}
pub trait __new_allocator_156Impl {
    fn deallocate(&self, mut __p: Ptr<u32>, mut __n: u64) {
        unimplemented!()
    }
}
impl __new_allocator_156Impl for Ptr<__new_allocator_156> {}
pub trait initializer_list_163Impl {
    fn size(&self) -> u64 {
        unimplemented!()
    }
    fn begin(&self) -> Ptr<bool> {
        unimplemented!()
    }
}
impl initializer_list_163Impl for Ptr<initializer_list_163> {}
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
pub trait vector_165Impl {
    fn copy_assign(&self, __x: Ptr<vector_165>) -> Ptr<vector_165> {
        unimplemented!()
    }
    fn move_assign(&self, __x: Ptr<vector_165>) -> Ptr<vector_165> {
        unimplemented!()
    }
    fn data_55(&self) -> Ptr<u32> {
        unimplemented!()
    }
}
impl vector_165Impl for Ptr<vector_165> {}
pub fn __cpp2rust_init_globals() {}
