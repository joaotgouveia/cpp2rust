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
pub struct is_void_6 {}
impl Clone for is_void_6 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_6 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_7 {}
impl Clone for remove_cv_7 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_7 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_8 {}
impl Clone for remove_cv_8 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_8 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_9 {}
impl Clone for remove_cv_9 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_9 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_10 {}
impl Clone for remove_cv_10 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_10 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_11 {}
impl Clone for remove_cv_11 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_11 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_12 {}
impl Clone for remove_cv_12 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_12 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_13 {}
impl Clone for remove_cv_13 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_13 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_const_14 {}
impl Clone for is_const_14 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_14 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_const_15 {}
impl Clone for is_const_15 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_15 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_const_16 {}
impl Clone for is_const_16 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_16 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_const_17 {}
impl Clone for is_const_17 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_17 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_const_18 {}
impl Clone for is_const_18 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_18 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_const_19 {}
impl Clone for is_const_19 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_19 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_const_20 {}
impl Clone for is_const_20 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_20 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_21 {}
impl Clone for __is_integral_helper_21 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_21 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_22 {}
impl Clone for __is_integral_helper_22 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_22 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_23 {}
impl Clone for __is_integral_helper_23 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_23 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_24 {}
impl Clone for __is_integral_helper_24 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_24 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_25 {}
impl Clone for __is_integral_helper_25 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_25 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_26 {}
impl Clone for __is_integral_helper_26 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_26 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_27 {}
impl Clone for __is_integral_helper_27 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_27 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_28 {}
impl Clone for is_integral_28 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_28 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_29 {}
impl Clone for is_integral_29 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_29 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_30 {}
impl Clone for is_integral_30 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_30 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_31 {}
impl Clone for is_integral_31 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_31 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_32 {}
impl Clone for is_integral_32 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_32 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_33 {}
impl Clone for is_integral_33 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_33 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_34 {}
impl Clone for is_integral_34 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_34 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_volatile_35 {}
impl Clone for is_volatile_35 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_35 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_volatile_36 {}
impl Clone for is_volatile_36 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_36 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_volatile_37 {}
impl Clone for is_volatile_37 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_37 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_volatile_38 {}
impl Clone for is_volatile_38 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_38 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_volatile_39 {}
impl Clone for is_volatile_39 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_39 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_volatile_40 {}
impl Clone for is_volatile_40 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_40 {
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
pub struct remove_reference_42 {}
impl Clone for remove_reference_42 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_42 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_43 {}
impl Clone for remove_reference_43 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_43 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __cv_selector_44 {}
impl Clone for __cv_selector_44 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_44 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __cv_selector_45 {}
impl Clone for __cv_selector_45 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_45 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __cv_selector_46 {}
impl Clone for __cv_selector_46 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_46 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __cv_selector_47 {}
impl Clone for __cv_selector_47 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_47 {
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
pub struct __match_cv_qualifiers_50 {}
impl Clone for __match_cv_qualifiers_50 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_50 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_51 {}
impl Clone for __match_cv_qualifiers_51 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_51 {
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
pub struct __make_unsigned_60 {}
impl Clone for __make_unsigned_60 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_60 {
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
pub struct __make_unsigned_selector_66 {}
impl Clone for __make_unsigned_selector_66 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_66 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_67 {}
impl Clone for __make_unsigned_selector_67 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_67 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_68 {}
impl Clone for __make_unsigned_selector_68 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_68 {
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
pub struct _List_72 {}
impl Clone for _List_72 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_72 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static std___make_unsigned_selector_base__List_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long____size_73 : Value<usize > = unimplemented!() ;
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_74 {}
impl Clone for _List_74 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_74 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static std___make_unsigned_selector_base__List_unsigned_short__unsigned_int__unsigned_long__unsigned_long_long____size_75 : Value<usize > = unimplemented!() ;
);
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
thread_local!(
    pub static std___make_unsigned_selector_base__List_unsigned_int__unsigned_long__unsigned_long_long____size_77 : Value<usize > = unimplemented!() ;
);
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
pub struct __select_83 {}
impl Clone for __select_83 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_83 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_84 {}
impl Clone for __select_84 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_84 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_85 {}
impl Clone for __select_85 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_85 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_86 {}
impl Clone for __select_86 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_86 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_87 {}
impl Clone for __make_signed_87 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_87 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_88 {}
impl Clone for __make_signed_88 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_88 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_89 {}
impl Clone for __make_signed_89 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_89 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_90 {}
impl Clone for __make_signed_selector_90 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_90 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_91 {}
impl Clone for __make_signed_selector_91 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_91 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_92 {}
impl Clone for __make_signed_selector_92 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_92 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_93 {}
impl Clone for __make_signed_selector_93 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_93 {
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
pub struct common_type_97 {}
impl Clone for common_type_97 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for common_type_97 {
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
pub struct common_reference_98 {}
impl Clone for common_reference_98 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for common_reference_98 {
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
pub struct std_sorted_unique_t {}
impl Clone for std_sorted_unique_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_sorted_unique_t {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_sorted_unique_t {}
impl std_sorted_unique_t {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct std_sorted_equivalent_t {}
impl Clone for std_sorted_equivalent_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_sorted_equivalent_t {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_sorted_equivalent_t {}
impl std_sorted_equivalent_t {}
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
    pub static swap_99: Value<std_ranges___swap__Swap> = unimplemented!();
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
    pub static std_partial_ordering_less_100: Value<std_partial_ordering> = unimplemented!();
);
thread_local!(
    pub static std_partial_ordering_equivalent_101: Value<std_partial_ordering> = unimplemented!();
);
thread_local!(
    pub static std_partial_ordering_greater_102: Value<std_partial_ordering> = unimplemented!();
);
thread_local!(
    pub static std_partial_ordering_unordered_103: Value<std_partial_ordering> = unimplemented!();
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
    pub static std_weak_ordering_less_104: Value<std_weak_ordering> = unimplemented!();
);
thread_local!(
    pub static std_weak_ordering_equivalent_105: Value<std_weak_ordering> = unimplemented!();
);
thread_local!(
    pub static std_weak_ordering_greater_106: Value<std_weak_ordering> = unimplemented!();
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
    pub static std_strong_ordering_less_107: Value<std_strong_ordering> = unimplemented!();
);
thread_local!(
    pub static std_strong_ordering_equal_108: Value<std_strong_ordering> = unimplemented!();
);
thread_local!(
    pub static std_strong_ordering_equivalent_109: Value<std_strong_ordering> = unimplemented!();
);
thread_local!(
    pub static std_strong_ordering_greater_110: Value<std_strong_ordering> = unimplemented!();
);
impl std_strong_ordering {}
impl std_strong_ordering {}
thread_local!(
    pub static __cmp_cat_id_111: Value<u32> = unimplemented!();
);
thread_local!(
    pub static __cmp_cat_id_112: Value<u32> = unimplemented!();
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
pub struct _Int_113 {
    #[offset(0)]
    pub _M_lo: u64,
    #[offset(8)]
    pub _M_hi: i32,
}
impl Clone for _Int_113 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Int_113 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct _Int_114 {
    #[offset(0)]
    pub _M_lo: u64,
    #[offset(8)]
    pub _M_hi: i16,
}
impl Clone for _Int_114 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Int_114 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Int_114 {}
impl _Int_114 {}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct _Int_115 {
    #[offset(0)]
    pub _M_lo: u64,
    #[offset(8)]
    pub _M_hi: i64,
}
impl Clone for _Int_115 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Int_115 {
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
thread_local!(
    pub static __synth3way_116: Value<std___detail__Synth3way> = unimplemented!();
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
pub struct __are_same_117 {}
impl Clone for __are_same_117 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_117 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_118 = u32;
pub const anon_118___value: anon_118 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_119 {}
impl Clone for __are_same_119 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_119 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_120 = u32;
pub const anon_120___value: anon_120 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_121 {}
impl Clone for __are_same_121 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_121 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_122 = u32;
pub const anon_122___value: anon_122 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_123 {}
impl Clone for __are_same_123 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_123 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_124 = u32;
pub const anon_124___value: anon_124 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_125 {}
impl Clone for __are_same_125 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_125 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_126 = u32;
pub const anon_126___value: anon_126 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_127 {}
impl Clone for __are_same_127 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_127 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_128 = u32;
pub const anon_128___value: anon_128 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_129 {}
impl Clone for __is_integer_129 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_129 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_130 = u32;
pub const anon_130___value: anon_130 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_131 {}
impl Clone for __is_integer_131 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_131 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_132 = u32;
pub const anon_132___value: anon_132 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_133 {}
impl Clone for __is_integer_133 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_133 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_134 = u32;
pub const anon_134___value: anon_134 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __memcpyable_integer_135 {}
impl Clone for __memcpyable_integer_135 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __memcpyable_integer_135 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __memcpyable_integer_136 {}
impl Clone for __memcpyable_integer_136 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __memcpyable_integer_136 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __memcpyable_integer_137 {}
impl Clone for __memcpyable_integer_137 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __memcpyable_integer_137 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(Clone, Record, ByteRepr, VaArg, Destructor, FnPtrArg)]
#[byte_size(1)]
pub struct Guard_141 {
    #[offset(0)]
    #[byte_size(1)]
    pub f: FnPtr<fn()>,
}
impl Default for Guard_141 {
    fn default() -> Self {
        {
            Guard_141 {
                f: FnPtr::<fn()>::new(|| {
                    (*count_143.with(Value::clone).borrow_mut()).postfix_inc();
                }),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, Destructor, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Guard_144 {
    #[offset(0)]
    #[byte_size(8)]
    pub f: FnPtr<fn()>,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct Holder_146 {
    #[offset(0)]
    #[byte_size(1)]
    pub f: FnPtr<fn(i32) -> i32>,
    #[offset(4)]
    pub calls: i32,
}
impl Default for Holder_146 {
    fn default() -> Self {
        {
            Holder_146 {
                f: FnPtr::<fn(i32) -> i32>::new(|mut x: i32| -> i32 {
                    {
                        return (x + 1);
                    }
                }),
                calls: 0_i32,
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Holder_148 {
    #[offset(0)]
    #[byte_size(4)]
    pub f: FnPtr<fn(i32) -> i32>,
    #[offset(4)]
    pub calls: i32,
}
#[derive(Record, ByteRepr, MoveCtor, FnPtrArg)]
#[byte_size(8)]
pub struct Moving_150 {
    #[offset(0)]
    #[byte_size(1)]
    pub f: FnPtr<fn(i32) -> i32>,
    #[offset(4)]
    pub moves: i32,
}
impl Moving_150 {
    pub fn new(f: FnPtr<fn(i32) -> i32>) -> Self {
        let f: Value<FnPtr<fn(i32) -> i32>> = Rc::new(RefCell::new(f));
        Self {
            f: (*f.borrow()).copy_from(),
            moves: 0,
        }
    }
    pub fn move_from(o: Ptr<Moving_150>) -> Self {
        Self {
            f: (*o.upgrade().deref()).f.move_from(),
            moves: (o.with(|__s| __s.moves) + 1),
        }
    }
}
impl Default for Moving_150 {
    fn default() -> Self {
        {
            Moving_150 {
                f: FnPtr::<fn(i32) -> i32>::new(|mut x: i32| -> i32 {
                    {
                        return (x * 2);
                    }
                }),
                moves: 0,
            }
        }
    }
}
pub fn wrap_153(fn_: FnPtr<fn(i32) -> i32>) -> FnPtr<fn(i32) -> i32> {
    let fn_: Value<FnPtr<fn(i32) -> i32>> = Rc::new(RefCell::new(fn_));
    return lambda!(
        {
            let fn_: Value<FnPtr<fn(i32) -> i32>> =
                Rc::new(RefCell::new((*fn_.borrow()).copy_from()));
        },
        |mut x: i32| -> i32 {
            return (({ (*fn_.borrow()).call(x) }) + 1);
        }
    );
}
pub fn wrap_156(fn_: FnPtr<fn(i32) -> i32>) -> FnPtr<fn(i32) -> i32> {
    let fn_: Value<FnPtr<fn(i32) -> i32>> = Rc::new(RefCell::new(fn_));
    return lambda!(
        {
            let fn_: Value<FnPtr<fn(i32) -> i32>> =
                Rc::new(RefCell::new((*fn_.borrow()).copy_from()));
        },
        |mut x: i32| -> i32 {
            return (({ (*fn_.borrow()).call(x) }) + 1);
        }
    );
}
thread_local!(
    pub static count_143: Value<i32> = Rc::new(RefCell::new(0));
);
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    {
        let g: Value<Guard_141> = Rc::new(RefCell::new(Guard_141 {
            f: FnPtr::<fn()>::new(|| {
                (*count_143.with(Value::clone).borrow_mut()).postfix_inc();
            }),
        }));
        let _dtor_g = ScopedDestructor::new(&g, |__p| __p.destructor());
        (if (count_143.with(|rc| *rc.borrow()) == 0) {
            &(0);
        } else {
            ({
                __assert_fail_158(
                    Ptr::<i8>::from_string_literal(b"count == 0"),
                    Ptr::<i8>::from_string_literal(file!().as_bytes()),
                    line!() as u32,
                    Ptr::<i8>::from_string_literal(b"int main()"),
                )
            });
        });
    }
    (if (count_143.with(|rc| *rc.borrow()) == 1) {
        &(0);
    } else {
        ({
            __assert_fail_158(
                Ptr::<i8>::from_string_literal(b"count == 1"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let n: Value<Holder_146> = Rc::new(RefCell::new(Holder_146 {
        f: FnPtr::<fn(i32) -> i32>::new(|mut x: i32| -> i32 {
            {
                return (x + 1);
            }
        }),
        calls: 0,
    }));
    (if (({ Holder_146Impl::call(&n.as_pointer(), 2) }) == 3) {
        &(0);
    } else {
        ({
            __assert_fail_158(
                Ptr::<i8>::from_string_literal(b"n.call(2) == 3"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ({ (*n.borrow()).calls } == 1) {
        &(0);
    } else {
        ({
            __assert_fail_158(
                Ptr::<i8>::from_string_literal(b"n.calls == 1"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let cleaned: Value<i32> = Rc::new(RefCell::new(0));
    {
        let g: Value<Guard_144> = Rc::new(RefCell::new(Guard_144 {
            f: lambda!(
                {
                    let cleaned: Ptr<i32> = cleaned.as_pointer();
                },
                || {
                    cleaned.with_mut(|__v| __v.postfix_inc());
                }
            ),
        }));
        let _dtor_g = ScopedDestructor::new(&g, |__p| __p.destructor());
        (if ((*cleaned.borrow()) == 0) {
            &(0);
        } else {
            ({
                __assert_fail_158(
                    Ptr::<i8>::from_string_literal(b"cleaned == 0"),
                    Ptr::<i8>::from_string_literal(file!().as_bytes()),
                    line!() as u32,
                    Ptr::<i8>::from_string_literal(b"int main()"),
                )
            });
        });
    }
    (if ((*cleaned.borrow()) == 1) {
        &(0);
    } else {
        ({
            __assert_fail_158(
                Ptr::<i8>::from_string_literal(b"cleaned == 1"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let factor: Value<i32> = Rc::new(RefCell::new(3));
    let h: Value<Holder_148> = Rc::new(RefCell::new(Holder_148 {
        f: lambda!(
            {
                let factor: Value<i32> = Rc::new(RefCell::new((*factor.borrow())));
            },
            |mut x: i32| -> i32 {
                return (x * (*factor.borrow()));
            }
        ),
        calls: 0,
    }));
    (*factor.borrow_mut()) = 100;
    (if (({ Holder_148Impl::call(&h.as_pointer(), 2) }) == 6) {
        &(0);
    } else {
        ({
            __assert_fail_158(
                Ptr::<i8>::from_string_literal(b"h.call(2) == 6"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ Holder_148Impl::call(&h.as_pointer(), 5) }) == 15) {
        &(0);
    } else {
        ({
            __assert_fail_158(
                Ptr::<i8>::from_string_literal(b"h.call(5) == 15"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ({ (*h.borrow()).calls } == 2) {
        &(0);
    } else {
        ({
            __assert_fail_158(
                Ptr::<i8>::from_string_literal(b"h.calls == 2"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let w: Value<FnPtr<fn(i32) -> i32>> = Rc::new(RefCell::new(
        ({
            wrap_153(lambda!(
                {
                    let factor: Value<i32> = Rc::new(RefCell::new((*factor.borrow())));
                },
                |mut x: i32| -> i32 {
                    return (x * (*factor.borrow()));
                }
            ))
        }),
    ));
    (*factor.borrow_mut()) = 7;
    (if (({ (*w.borrow()).call(2) }) == 201) {
        &(0);
    } else {
        ({
            __assert_fail_158(
                Ptr::<i8>::from_string_literal(b"w(2) == 201"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let ww: Value<FnPtr<fn(i32) -> i32>> =
        Rc::new(RefCell::new(({ wrap_156((*w.borrow()).copy_from()) })));
    (if (({ (*ww.borrow()).call(2) }) == 202) {
        &(0);
    } else {
        ({
            __assert_fail_158(
                Ptr::<i8>::from_string_literal(b"ww(2) == 202"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let m: Value<Moving_150> = Rc::new(RefCell::new(Moving_150::new({
        FnPtr::<fn(i32) -> i32>::new(|mut x: i32| -> i32 {
            {
                return (x * 2);
            }
        })
    })));
    let m2: Value<Moving_150> = Rc::new(RefCell::new(Moving_150::move_from({ m.as_pointer() })));
    (if (({ (*m2.borrow()).f.call(3) }) == 6) {
        &(0);
    } else {
        ({
            __assert_fail_158(
                Ptr::<i8>::from_string_literal(b"m2.f(3) == 6"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ({ (*m2.borrow()).moves } == 1) {
        &(0);
    } else {
        ({
            __assert_fail_158(
                Ptr::<i8>::from_string_literal(b"m2.moves == 1"),
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
pub struct std___detail__Synth3way;
pub fn __assert_fail_158(
    mut __assertion: Ptr<i8>,
    mut __file: Ptr<i8>,
    mut __line: u32,
    mut __function: Ptr<i8>,
) {
    unimplemented!()
}
pub trait Guard_141Impl {
    fn destructor(&self);
}
impl Guard_141Impl for Ptr<Guard_141> {
    fn destructor(&self) {
        ({ (*(*self).upgrade().deref()).f.call() });
    }
}
pub trait Guard_144Impl {
    fn destructor(&self);
}
impl Guard_144Impl for Ptr<Guard_144> {
    fn destructor(&self) {
        ({ (*(*self).upgrade().deref()).f.call() });
    }
}
pub trait Holder_146Impl {
    fn call(&self, x: i32) -> i32;
}
impl Holder_146Impl for Ptr<Holder_146> {
    fn call(&self, mut x: i32) -> i32 {
        field!((*self), calls).with_mut(|__v| __v.postfix_inc());
        return ({ (*(*self).upgrade().deref()).f.call(x) });
    }
}
pub trait Holder_148Impl {
    fn call(&self, x: i32) -> i32;
}
impl Holder_148Impl for Ptr<Holder_148> {
    fn call(&self, mut x: i32) -> i32 {
        field!((*self), calls).with_mut(|__v| __v.postfix_inc());
        return ({ (*(*self).upgrade().deref()).f.call(x) });
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = count_143.with(|_| ());
}
