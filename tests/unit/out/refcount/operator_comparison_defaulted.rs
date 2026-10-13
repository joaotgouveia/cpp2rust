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
pub struct is_void_5 {}
impl Clone for is_void_5 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_5 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_6 {}
impl Clone for remove_cv_6 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_6 {
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
pub struct is_const_13 {}
impl Clone for is_const_13 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_13 {
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
pub struct __is_integral_helper_20 {}
impl Clone for __is_integral_helper_20 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_20 {
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
pub struct is_integral_27 {}
impl Clone for is_integral_27 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_27 {
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
pub struct is_volatile_34 {}
impl Clone for is_volatile_34 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_34 {
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
pub struct __cv_selector_41 {}
impl Clone for __cv_selector_41 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_41 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __cv_selector_42 {}
impl Clone for __cv_selector_42 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_42 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __cv_selector_43 {}
impl Clone for __cv_selector_43 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_43 {
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
pub struct __match_cv_qualifiers_47 {}
impl Clone for __match_cv_qualifiers_47 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_47 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_48 {}
impl Clone for __match_cv_qualifiers_48 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_48 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_49 {}
impl Clone for __match_cv_qualifiers_49 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_49 {
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
pub struct __make_unsigned_54 {}
impl Clone for __make_unsigned_54 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_54 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_55 {}
impl Clone for __make_unsigned_55 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_55 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_56 {}
impl Clone for __make_unsigned_56 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_56 {
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
pub struct __make_unsigned_selector_58 {}
impl Clone for __make_unsigned_selector_58 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_58 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_59 {}
impl Clone for __make_unsigned_selector_59 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_59 {
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
pub struct __make_signed_85 {}
impl Clone for __make_signed_85 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_85 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_86 {}
impl Clone for __make_signed_86 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_86 {
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
pub struct common_type_94 {}
impl Clone for common_type_94 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for common_type_94 {
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
pub struct common_reference_95 {}
impl Clone for common_reference_95 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for common_reference_95 {
    fn default() -> Self {
        unimplemented!()
    }
}
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
    pub static swap_96: Value<std_ranges___swap__Swap> = unimplemented!();
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
    pub static std_partial_ordering_less_97: Value<std_partial_ordering> = unimplemented!();
);
thread_local!(
    pub static std_partial_ordering_equivalent_98: Value<std_partial_ordering> = unimplemented!();
);
thread_local!(
    pub static std_partial_ordering_greater_99: Value<std_partial_ordering> = unimplemented!();
);
thread_local!(
    pub static std_partial_ordering_unordered_100: Value<std_partial_ordering> = unimplemented!();
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
    pub static std_weak_ordering_less_101: Value<std_weak_ordering> = unimplemented!();
);
thread_local!(
    pub static std_weak_ordering_equivalent_102: Value<std_weak_ordering> = unimplemented!();
);
thread_local!(
    pub static std_weak_ordering_greater_103: Value<std_weak_ordering> = unimplemented!();
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
    pub static std_strong_ordering_less_104: Value<std_strong_ordering> = unimplemented!();
);
thread_local!(
    pub static std_strong_ordering_equal_105: Value<std_strong_ordering> = unimplemented!();
);
thread_local!(
    pub static std_strong_ordering_equivalent_106: Value<std_strong_ordering> = unimplemented!();
);
thread_local!(
    pub static std_strong_ordering_greater_107: Value<std_strong_ordering> = unimplemented!();
);
impl std_strong_ordering {}
impl std_strong_ordering {}
thread_local!(
    pub static __cmp_cat_id_108: Value<u32> = unimplemented!();
);
thread_local!(
    pub static __cmp_cat_id_109: Value<u32> = unimplemented!();
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
pub struct _Int_110 {
    #[offset(0)]
    pub _M_lo: u64,
    #[offset(8)]
    pub _M_hi: i32,
}
impl Clone for _Int_110 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Int_110 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct _Int_111 {
    #[offset(0)]
    pub _M_lo: u64,
    #[offset(8)]
    pub _M_hi: i16,
}
impl Clone for _Int_111 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Int_111 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Int_111 {}
impl _Int_111 {}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct _Int_112 {
    #[offset(0)]
    pub _M_lo: u64,
    #[offset(8)]
    pub _M_hi: i64,
}
impl Clone for _Int_112 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Int_112 {
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
    pub static __synth3way_113: Value<std___detail__Synth3way> = unimplemented!();
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Eq {
    #[offset(0)]
    pub a: i32,
    #[offset(4)]
    pub b: i32,
}
impl std::cmp::PartialEq for Eq {
    fn eq(&self, other: &Self) -> bool {
        {
            EqImpl::operator_eq(
                &Rc::new(RefCell::new(Eq {
                    a: self.a.clone(),
                    b: self.b.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(Eq {
                    a: other.a.clone(),
                    b: other.b.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for Eq {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Cmp {
    #[offset(0)]
    pub a: i32,
    #[offset(4)]
    pub b: i32,
}
impl std::cmp::Ord for Cmp {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        { unimplemented!() }
    }
}
impl std::cmp::PartialOrd for Cmp {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for Cmp {
    fn eq(&self, other: &Self) -> bool {
        { unimplemented!() }
    }
}
impl std::cmp::Eq for Cmp {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Both {
    #[offset(0)]
    pub a: i32,
}
impl std::cmp::Ord for Both {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        { unimplemented!() }
    }
}
impl std::cmp::PartialOrd for Both {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for Both {
    fn eq(&self, other: &Self) -> bool {
        {
            BothImpl::operator_eq(
                &Rc::new(RefCell::new(Both { a: self.a.clone() })).as_pointer(),
                Rc::new(RefCell::new(Both { a: other.a.clone() })).as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for Both {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct OrdOnly {
    #[offset(0)]
    pub a: i32,
}
impl std::cmp::Ord for OrdOnly {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        { unimplemented!() }
    }
}
impl std::cmp::PartialOrd for OrdOnly {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for OrdOnly {
    fn eq(&self, other: &Self) -> bool {
        { unimplemented!() }
    }
}
impl std::cmp::Eq for OrdOnly {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Inner {
    #[offset(0)]
    pub x: i32,
}
impl std::cmp::Ord for Inner {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        { unimplemented!() }
    }
}
impl std::cmp::PartialOrd for Inner {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for Inner {
    fn eq(&self, other: &Self) -> bool {
        { unimplemented!() }
    }
}
impl std::cmp::Eq for Inner {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Outer {
    #[offset(0)]
    #[byte_size(4)]
    pub i: Inner,
    #[offset(4)]
    pub y: i32,
}
impl std::cmp::Ord for Outer {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        { unimplemented!() }
    }
}
impl std::cmp::PartialOrd for Outer {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for Outer {
    fn eq(&self, other: &Self) -> bool {
        { unimplemented!() }
    }
}
impl std::cmp::Eq for Outer {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Secondary {
    #[offset(0)]
    pub a: i32,
}
impl std::cmp::Ord for Secondary {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        { unimplemented!() }
    }
}
impl std::cmp::PartialOrd for Secondary {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for Secondary {
    fn eq(&self, other: &Self) -> bool {
        {
            SecondaryImpl::operator_eq(
                &Rc::new(RefCell::new(Secondary { a: self.a.clone() })).as_pointer(),
                Rc::new(RefCell::new(Secondary { a: other.a.clone() })).as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for Secondary {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct PtrMember {
    #[offset(0)]
    #[byte_size(8)]
    pub p: Ptr<i32>,
}
impl std::cmp::Ord for PtrMember {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        { unimplemented!() }
    }
}
impl std::cmp::PartialOrd for PtrMember {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for PtrMember {
    fn eq(&self, other: &Self) -> bool {
        { unimplemented!() }
    }
}
impl std::cmp::Eq for PtrMember {}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let e1: Value<Eq> = Rc::new(RefCell::new(Eq { a: 1, b: 2 }));
    let e2: Value<Eq> = Rc::new(RefCell::new(Eq { a: 1, b: 2 }));
    let e3: Value<Eq> = Rc::new(RefCell::new(Eq { a: 1, b: 3 }));
    (if ({ EqImpl::operator_eq(&e1.as_pointer(), e2.as_pointer()) }) {
        &(0);
    } else {
        ({
            __assert_fail_117(
                Ptr::<i8>::from_string_literal(b"e1 == e2"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if !({ EqImpl::operator_eq(&e1.as_pointer(), e3.as_pointer()) }) {
        &(0);
    } else {
        ({
            __assert_fail_117(
                Ptr::<i8>::from_string_literal(b"e1 != e3"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let c1: Value<Cmp> = Rc::new(RefCell::new(Cmp { a: 1, b: 2 }));
    let c2: Value<Cmp> = Rc::new(RefCell::new(Cmp { a: 1, b: 3 }));
    let c3: Value<Cmp> = Rc::new(RefCell::new(Cmp { a: 2, b: 0 }));
    let c4: Value<Cmp> = Rc::new(RefCell::new(Cmp { a: 1, b: 9 }));
    (if ({
        let ___v: std_strong_ordering =
            ({ CmpImpl::operator_cmp(&c1.as_pointer(), c2.as_pointer()) });
        operator_lt_115(
            ___v,
            std___cmp_cat___unspec::new({ Ptr::<std___cmp_cat___unspec>::null() }),
        )
    }) {
        &(0);
    } else {
        ({
            __assert_fail_117(
                Ptr::<i8>::from_string_literal(b"c1 < c2"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ({
        let ___v: std_strong_ordering =
            ({ CmpImpl::operator_cmp(&c3.as_pointer(), c4.as_pointer()) });
        operator_gt_118(
            ___v,
            std___cmp_cat___unspec::new({ Ptr::<std___cmp_cat___unspec>::null() }),
        )
    }) {
        &(0);
    } else {
        ({
            __assert_fail_117(
                Ptr::<i8>::from_string_literal(b"c3 > c4"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ({
        let _arg0: Ptr<Cmp> = c1.as_pointer();
        CmpImpl::operator_eq(&c1.as_pointer(), _arg0)
    }) {
        &(0);
    } else {
        ({
            __assert_fail_117(
                Ptr::<i8>::from_string_literal(b"c1 == c1"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ({
        let _arg0: std_strong_ordering =
            ({ CmpImpl::operator_cmp(&c1.as_pointer(), c2.as_pointer()) });
        operator_eq_119(
            _arg0,
            (*std_strong_ordering_less_104.with(Value::clone).borrow()).clone(),
        )
    }) {
        &(0);
    } else {
        ({
            __assert_fail_117(
                Ptr::<i8>::from_string_literal(b"(c1 <=> c2) == std::strong_ordering::less"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let b1: Value<Both> = Rc::new(RefCell::new(Both { a: 1 }));
    let b2: Value<Both> = Rc::new(RefCell::new(Both { a: 2 }));
    (if ({
        let ___v: std_strong_ordering =
            ({ BothImpl::operator_cmp(&b1.as_pointer(), b2.as_pointer()) });
        operator_lt_115(
            ___v,
            std___cmp_cat___unspec::new({ Ptr::<std___cmp_cat___unspec>::null() }),
        )
    }) {
        &(0);
    } else {
        ({
            __assert_fail_117(
                Ptr::<i8>::from_string_literal(b"b1 < b2"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ({
        let _arg0: Ptr<Both> = b2.as_pointer();
        BothImpl::operator_eq(&b2.as_pointer(), _arg0)
    }) {
        &(0);
    } else {
        ({
            __assert_fail_117(
                Ptr::<i8>::from_string_literal(b"b2 == b2"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let o1: Value<OrdOnly> = Rc::new(RefCell::new(OrdOnly { a: 1 }));
    let o2: Value<OrdOnly> = Rc::new(RefCell::new(OrdOnly { a: 2 }));
    (if ({
        let ___v: std_strong_ordering =
            ({ OrdOnlyImpl::operator_cmp(&o1.as_pointer(), o2.as_pointer()) });
        operator_lt_115(
            ___v,
            std___cmp_cat___unspec::new({ Ptr::<std___cmp_cat___unspec>::null() }),
        )
    }) {
        &(0);
    } else {
        ({
            __assert_fail_117(
                Ptr::<i8>::from_string_literal(b"o1 < o2"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ({
        let _arg0: std_strong_ordering =
            ({ OrdOnlyImpl::operator_cmp(&o2.as_pointer(), o1.as_pointer()) });
        operator_eq_119(
            _arg0,
            (*std_strong_ordering_greater_107.with(Value::clone).borrow()).clone(),
        )
    }) {
        &(0);
    } else {
        ({
            __assert_fail_117(
                Ptr::<i8>::from_string_literal(b"(o2 <=> o1) == std::strong_ordering::greater"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let x1: Value<Outer> = Rc::new(RefCell::new(Outer {
        i: Inner { x: 1 },
        y: 9,
    }));
    let x2: Value<Outer> = Rc::new(RefCell::new(Outer {
        i: Inner { x: 2 },
        y: 0,
    }));
    let x3: Value<Outer> = Rc::new(RefCell::new(Outer {
        i: Inner { x: 1 },
        y: 9,
    }));
    (if ({
        let ___v: std_strong_ordering =
            ({ OuterImpl::operator_cmp(&x1.as_pointer(), x2.as_pointer()) });
        operator_lt_115(
            ___v,
            std___cmp_cat___unspec::new({ Ptr::<std___cmp_cat___unspec>::null() }),
        )
    }) {
        &(0);
    } else {
        ({
            __assert_fail_117(
                Ptr::<i8>::from_string_literal(b"x1 < x2"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ({ OuterImpl::operator_eq(&x1.as_pointer(), x3.as_pointer()) }) {
        &(0);
    } else {
        ({
            __assert_fail_117(
                Ptr::<i8>::from_string_literal(b"x1 == x3"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ({
        let _arg0: std_strong_ordering =
            ({ OuterImpl::operator_cmp(&x2.as_pointer(), x1.as_pointer()) });
        operator_eq_119(
            _arg0,
            (*std_strong_ordering_greater_107.with(Value::clone).borrow()).clone(),
        )
    }) {
        &(0);
    } else {
        ({
            __assert_fail_117(
                Ptr::<i8>::from_string_literal(b"(x2 <=> x1) == std::strong_ordering::greater"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let s1: Value<Secondary> = Rc::new(RefCell::new(Secondary { a: 1 }));
    let s2: Value<Secondary> = Rc::new(RefCell::new(Secondary { a: 2 }));
    (if ({ SecondaryImpl::operator_ne(&s1.as_pointer(), s2.as_pointer()) }) {
        &(0);
    } else {
        ({
            __assert_fail_117(
                Ptr::<i8>::from_string_literal(b"s1 != s2"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ({ SecondaryImpl::operator_lt(&s1.as_pointer(), s2.as_pointer()) }) {
        &(0);
    } else {
        ({
            __assert_fail_117(
                Ptr::<i8>::from_string_literal(b"s1 < s2"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ({ SecondaryImpl::operator_ge(&s2.as_pointer(), s1.as_pointer()) }) {
        &(0);
    } else {
        ({
            __assert_fail_117(
                Ptr::<i8>::from_string_literal(b"s2 >= s1"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if !({ SecondaryImpl::operator_lt(&s2.as_pointer(), s1.as_pointer()) }) {
        &(0);
    } else {
        ({
            __assert_fail_117(
                Ptr::<i8>::from_string_literal(b"!(s2 < s1)"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let arr: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 0])));
    let p1: Value<PtrMember> = Rc::new(RefCell::new(PtrMember {
        p: (arr.as_pointer() as Ptr<i32>),
    }));
    let p2: Value<PtrMember> = Rc::new(RefCell::new(PtrMember {
        p: (arr.as_pointer() as Ptr<i32>).offset((1) as isize),
    }));
    let p3: Value<PtrMember> = Rc::new(RefCell::new(PtrMember {
        p: (arr.as_pointer() as Ptr<i32>),
    }));
    (if ({
        let ___v: std_strong_ordering =
            ({ PtrMemberImpl::operator_cmp(&p1.as_pointer(), p2.as_pointer()) });
        operator_lt_115(
            ___v,
            std___cmp_cat___unspec::new({ Ptr::<std___cmp_cat___unspec>::null() }),
        )
    }) {
        &(0);
    } else {
        ({
            __assert_fail_117(
                Ptr::<i8>::from_string_literal(b"p1 < p2"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ({ PtrMemberImpl::operator_eq(&p1.as_pointer(), p3.as_pointer()) }) {
        &(0);
    } else {
        ({
            __assert_fail_117(
                Ptr::<i8>::from_string_literal(b"p1 == p3"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ({
        let _arg0: std_strong_ordering =
            ({ PtrMemberImpl::operator_cmp(&p2.as_pointer(), p1.as_pointer()) });
        operator_eq_119(
            _arg0,
            (*std_strong_ordering_greater_107.with(Value::clone).borrow()).clone(),
        )
    }) {
        &(0);
    } else {
        ({
            __assert_fail_117(
                Ptr::<i8>::from_string_literal(b"(p2 <=> p1) == std::strong_ordering::greater"),
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
#[track_caller]
pub fn __cpp2rust_unimplemented<T>() -> T {
    unimplemented!()
}
pub fn operator_eq_114(__v: std_strong_ordering, _a1: std___cmp_cat___unspec) -> bool {
    unimplemented!()
}
pub fn operator_lt_115(__v: std_strong_ordering, _a1: std___cmp_cat___unspec) -> bool {
    unimplemented!()
}
pub fn operator_ge_116(__v: std_strong_ordering, _a1: std___cmp_cat___unspec) -> bool {
    unimplemented!()
}
pub fn __assert_fail_117(
    mut __assertion: Ptr<i8>,
    mut __file: Ptr<i8>,
    mut __line: u32,
    mut __function: Ptr<i8>,
) {
    unimplemented!()
}
pub fn operator_gt_118(__v: std_strong_ordering, _a1: std___cmp_cat___unspec) -> bool {
    unimplemented!()
}
pub fn operator_eq_119(_a0: std_strong_ordering, _a1: std_strong_ordering) -> bool {
    unimplemented!()
}
pub trait BothImpl {
    fn operator_eq(&self, _a0: Ptr<Both>) -> bool;
    fn operator_cmp(&self, _a0: Ptr<Both>) -> std_strong_ordering;
}
impl BothImpl for Ptr<Both> {
    fn operator_eq(&self, _a0: Ptr<Both>) -> bool {
        return ({ (*self).with(|__s| __s.a) } == { _a0.with(|__s| __s.a) });
    }
    fn operator_cmp(&self, _a0: Ptr<Both>) -> std_strong_ordering {
        {
            let cmp: Value<std_strong_ordering> = Rc::new(RefCell::new(
                __cpp2rust_unimplemented::<std_strong_ordering>(),
            ));
            if !({
                let ___v: std_strong_ordering = (*cmp.borrow()).clone();
                operator_eq_114(
                    ___v,
                    std___cmp_cat___unspec::new({ Ptr::<std___cmp_cat___unspec>::null() }),
                )
            }) {
                return (*cmp.borrow_mut()).clone();
            }
        }
        return (*std_strong_ordering_equal_105.with(Value::clone).borrow()).clone();
    }
}
pub trait CmpImpl {
    fn operator_cmp(&self, _a0: Ptr<Cmp>) -> std_strong_ordering;
    fn operator_eq(&self, _a0: Ptr<Cmp>) -> bool;
}
impl CmpImpl for Ptr<Cmp> {
    fn operator_cmp(&self, _a0: Ptr<Cmp>) -> std_strong_ordering {
        {
            let cmp: Value<std_strong_ordering> = Rc::new(RefCell::new(
                __cpp2rust_unimplemented::<std_strong_ordering>(),
            ));
            if !({
                let ___v: std_strong_ordering = (*cmp.borrow()).clone();
                operator_eq_114(
                    ___v,
                    std___cmp_cat___unspec::new({ Ptr::<std___cmp_cat___unspec>::null() }),
                )
            }) {
                return (*cmp.borrow_mut()).clone();
            }
        }
        {
            let cmp: Value<std_strong_ordering> = Rc::new(RefCell::new(
                __cpp2rust_unimplemented::<std_strong_ordering>(),
            ));
            if !({
                let ___v: std_strong_ordering = (*cmp.borrow()).clone();
                operator_eq_114(
                    ___v,
                    std___cmp_cat___unspec::new({ Ptr::<std___cmp_cat___unspec>::null() }),
                )
            }) {
                return (*cmp.borrow_mut()).clone();
            }
        }
        return (*std_strong_ordering_equal_105.with(Value::clone).borrow()).clone();
    }
    fn operator_eq(&self, _a0: Ptr<Cmp>) -> bool {
        return ({ (*self).with(|__s| __s.a) } == { _a0.with(|__s| __s.a) })
            && ({ (*self).with(|__s| __s.b) } == { _a0.with(|__s| __s.b) });
    }
}
pub trait EqImpl {
    fn operator_eq(&self, _a0: Ptr<Eq>) -> bool;
}
impl EqImpl for Ptr<Eq> {
    fn operator_eq(&self, _a0: Ptr<Eq>) -> bool {
        return ({ (*self).with(|__s| __s.a) } == { _a0.with(|__s| __s.a) })
            && ({ (*self).with(|__s| __s.b) } == { _a0.with(|__s| __s.b) });
    }
}
pub trait InnerImpl {
    fn operator_cmp(&self, _a0: Ptr<Inner>) -> std_strong_ordering;
    fn operator_eq(&self, _a0: Ptr<Inner>) -> bool;
}
impl InnerImpl for Ptr<Inner> {
    fn operator_cmp(&self, _a0: Ptr<Inner>) -> std_strong_ordering {
        {
            let cmp: Value<std_strong_ordering> = Rc::new(RefCell::new(
                __cpp2rust_unimplemented::<std_strong_ordering>(),
            ));
            if !({
                let ___v: std_strong_ordering = (*cmp.borrow()).clone();
                operator_eq_114(
                    ___v,
                    std___cmp_cat___unspec::new({ Ptr::<std___cmp_cat___unspec>::null() }),
                )
            }) {
                return (*cmp.borrow_mut()).clone();
            }
        }
        return (*std_strong_ordering_equal_105.with(Value::clone).borrow()).clone();
    }
    fn operator_eq(&self, _a0: Ptr<Inner>) -> bool {
        return ({ (*self).with(|__s| __s.x) } == { _a0.with(|__s| __s.x) });
    }
}
pub trait OrdOnlyImpl {
    fn operator_cmp(&self, _a0: Ptr<OrdOnly>) -> std_strong_ordering;
}
impl OrdOnlyImpl for Ptr<OrdOnly> {
    fn operator_cmp(&self, _a0: Ptr<OrdOnly>) -> std_strong_ordering {
        {
            let cmp: Value<std_strong_ordering> = Rc::new(RefCell::new(
                __cpp2rust_unimplemented::<std_strong_ordering>(),
            ));
            if !({
                let ___v: std_strong_ordering = (*cmp.borrow()).clone();
                operator_eq_114(
                    ___v,
                    std___cmp_cat___unspec::new({ Ptr::<std___cmp_cat___unspec>::null() }),
                )
            }) {
                return (*cmp.borrow_mut()).clone();
            }
        }
        return (*std_strong_ordering_equal_105.with(Value::clone).borrow()).clone();
    }
}
pub trait OuterImpl {
    fn operator_cmp(&self, _a0: Ptr<Outer>) -> std_strong_ordering;
    fn operator_eq(&self, _a0: Ptr<Outer>) -> bool;
}
impl OuterImpl for Ptr<Outer> {
    fn operator_cmp(&self, _a0: Ptr<Outer>) -> std_strong_ordering {
        {
            let cmp: Value<std_strong_ordering> = Rc::new(RefCell::new(
                ({
                    let _arg0: Ptr<Inner> = field_ptr!(_a0, i);
                    InnerImpl::operator_cmp(&field_ptr!((*self), i), _arg0)
                }),
            ));
            if !({
                let ___v: std_strong_ordering = (*cmp.borrow()).clone();
                operator_eq_114(
                    ___v,
                    std___cmp_cat___unspec::new({ Ptr::<std___cmp_cat___unspec>::null() }),
                )
            }) {
                return (*cmp.borrow_mut()).clone();
            }
        }
        {
            let cmp: Value<std_strong_ordering> = Rc::new(RefCell::new(
                __cpp2rust_unimplemented::<std_strong_ordering>(),
            ));
            if !({
                let ___v: std_strong_ordering = (*cmp.borrow()).clone();
                operator_eq_114(
                    ___v,
                    std___cmp_cat___unspec::new({ Ptr::<std___cmp_cat___unspec>::null() }),
                )
            }) {
                return (*cmp.borrow_mut()).clone();
            }
        }
        return (*std_strong_ordering_equal_105.with(Value::clone).borrow()).clone();
    }
    fn operator_eq(&self, _a0: Ptr<Outer>) -> bool {
        return ({
            let _arg0: Ptr<Inner> = field_ptr!(_a0, i);
            InnerImpl::operator_eq(&field_ptr!((*self), i), _arg0)
        }) && ({ (*self).with(|__s| __s.y) } == { _a0.with(|__s| __s.y) });
    }
}
pub trait PtrMemberImpl {
    fn operator_cmp(&self, _a0: Ptr<PtrMember>) -> std_strong_ordering;
    fn operator_eq(&self, _a0: Ptr<PtrMember>) -> bool;
}
impl PtrMemberImpl for Ptr<PtrMember> {
    fn operator_cmp(&self, _a0: Ptr<PtrMember>) -> std_strong_ordering {
        {
            let cmp: Value<std_strong_ordering> = Rc::new(RefCell::new(
                __cpp2rust_unimplemented::<std_strong_ordering>(),
            ));
            if !({
                let ___v: std_strong_ordering = (*cmp.borrow()).clone();
                operator_eq_114(
                    ___v,
                    std___cmp_cat___unspec::new({ Ptr::<std___cmp_cat___unspec>::null() }),
                )
            }) {
                return (*cmp.borrow_mut()).clone();
            }
        }
        return (*std_strong_ordering_equal_105.with(Value::clone).borrow()).clone();
    }
    fn operator_eq(&self, _a0: Ptr<PtrMember>) -> bool {
        return ({ (*self).with(|__s| __s.p.clone()) } == { _a0.with(|__s| __s.p.clone()) });
    }
}
pub trait SecondaryImpl {
    fn operator_eq(&self, _a0: Ptr<Secondary>) -> bool;
    fn operator_ne(&self, _a0: Ptr<Secondary>) -> bool;
    fn operator_cmp(&self, _a0: Ptr<Secondary>) -> std_strong_ordering;
    fn operator_lt(&self, _a0: Ptr<Secondary>) -> bool;
    fn operator_ge(&self, _a0: Ptr<Secondary>) -> bool;
}
impl SecondaryImpl for Ptr<Secondary> {
    fn operator_eq(&self, _a0: Ptr<Secondary>) -> bool {
        return ({ (*self).with(|__s| __s.a) } == { _a0.with(|__s| __s.a) });
    }
    fn operator_ne(&self, _a0: Ptr<Secondary>) -> bool {
        return !({
            let _arg0: Ptr<Secondary> = (_a0).clone();
            SecondaryImpl::operator_eq(&(*self), _arg0)
        });
    }
    fn operator_cmp(&self, _a0: Ptr<Secondary>) -> std_strong_ordering {
        {
            let cmp: Value<std_strong_ordering> = Rc::new(RefCell::new(
                __cpp2rust_unimplemented::<std_strong_ordering>(),
            ));
            if !({
                let ___v: std_strong_ordering = (*cmp.borrow()).clone();
                operator_eq_114(
                    ___v,
                    std___cmp_cat___unspec::new({ Ptr::<std___cmp_cat___unspec>::null() }),
                )
            }) {
                return (*cmp.borrow_mut()).clone();
            }
        }
        return (*std_strong_ordering_equal_105.with(Value::clone).borrow()).clone();
    }
    fn operator_lt(&self, _a0: Ptr<Secondary>) -> bool {
        return ({
            let ___v: std_strong_ordering = ({
                let _arg0: Ptr<Secondary> = (_a0).clone();
                SecondaryImpl::operator_cmp(&(*self), _arg0)
            });
            operator_lt_115(
                ___v,
                std___cmp_cat___unspec::new({ Ptr::<std___cmp_cat___unspec>::null() }),
            )
        });
    }
    fn operator_ge(&self, _a0: Ptr<Secondary>) -> bool {
        return ({
            let ___v: std_strong_ordering = ({
                let _arg0: Ptr<Secondary> = (_a0).clone();
                SecondaryImpl::operator_cmp(&(*self), _arg0)
            });
            operator_ge_116(
                ___v,
                std___cmp_cat___unspec::new({ Ptr::<std___cmp_cat___unspec>::null() }),
            )
        });
    }
}
pub fn __cpp2rust_init_globals() {}
