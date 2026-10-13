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
pub fn operator_eq_114(_a0: Ptr<Defaulted>, _a1: Ptr<Defaulted>) -> bool {
    return ({ _a0.with(|__s| __s.a) } == { _a1.with(|__s| __s.a) })
        && ({ _a0.with(|__s| __s.b) } == { _a1.with(|__s| __s.b) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Defaulted {
    #[offset(0)]
    pub a: i32,
    #[offset(4)]
    pub b: i32,
}
impl std::cmp::PartialEq for Defaulted {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_114(
                Rc::new(RefCell::new(Defaulted {
                    a: self.a.clone(),
                    b: self.b.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(Defaulted {
                    a: other.a.clone(),
                    b: other.b.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for Defaulted {}
pub fn operator_cmp_115(_a0: Ptr<DefaultedOrd>, _a1: Ptr<DefaultedOrd>) -> std_strong_ordering {
    {
        let cmp: Value<std_strong_ordering> = Rc::new(RefCell::new(__cpp2rust_unimplemented::<
            std_strong_ordering,
        >()));
        if !({
            let ___v: std_strong_ordering = (*cmp.borrow()).clone();
            operator_eq_116(
                ___v,
                std___cmp_cat___unspec::new({ Ptr::<std___cmp_cat___unspec>::null() }),
            )
        }) {
            return (*cmp.borrow_mut()).clone();
        }
    }
    return (*std_strong_ordering_equal_105.with(Value::clone).borrow()).clone();
}
pub fn operator_eq_117(_a0: Ptr<DefaultedOrd>, _a1: Ptr<DefaultedOrd>) -> bool {
    return ({ _a0.with(|__s| __s.a) } == { _a1.with(|__s| __s.a) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct DefaultedOrd {
    #[offset(0)]
    pub a: i32,
}
impl std::cmp::Ord for DefaultedOrd {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        { unimplemented!() }
    }
}
impl std::cmp::PartialOrd for DefaultedOrd {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for DefaultedOrd {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_117(
                Rc::new(RefCell::new(DefaultedOrd { a: self.a.clone() })).as_pointer(),
                Rc::new(RefCell::new(DefaultedOrd { a: other.a.clone() })).as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for DefaultedOrd {}
pub fn operator_eq_118(x: Ptr<Inline>, y: Ptr<Inline>) -> bool {
    return ({ x.with(|__s| __s.a) } == { y.with(|__s| __s.a) });
}
pub fn operator_lt_119(x: Ptr<Inline>, y: Ptr<Inline>) -> bool {
    return ({ x.with(|__s| __s.a) } < { y.with(|__s| __s.a) });
}
pub fn operator_add_120(x: Ptr<Inline>, y: Ptr<Inline>) -> Inline {
    return Inline {
        a: ({ x.with(|__s| __s.a) } + { y.with(|__s| __s.a) }),
    };
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Inline {
    #[offset(0)]
    pub a: i32,
}
impl std::cmp::Ord for Inline {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            if operator_lt_119(
                Rc::new(RefCell::new(Inline { a: self.a.clone() })).as_pointer(),
                Rc::new(RefCell::new(Inline { a: other.a.clone() })).as_pointer(),
            ) {
                std::cmp::Ordering::Less
            } else if operator_lt_119(
                Rc::new(RefCell::new(Inline { a: other.a.clone() })).as_pointer(),
                Rc::new(RefCell::new(Inline { a: self.a.clone() })).as_pointer(),
            ) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for Inline {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for Inline {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_118(
                Rc::new(RefCell::new(Inline { a: self.a.clone() })).as_pointer(),
                Rc::new(RefCell::new(Inline { a: other.a.clone() })).as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for Inline {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct OutOfLine {
    #[offset(0)]
    pub a: i32,
}
impl std::cmp::PartialEq for OutOfLine {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_121(
                Rc::new(RefCell::new(OutOfLine { a: self.a.clone() })).as_pointer(),
                Rc::new(RefCell::new(OutOfLine { a: other.a.clone() })).as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for OutOfLine {}
pub fn operator_eq_121(x: Ptr<OutOfLine>, y: Ptr<OutOfLine>) -> bool {
    return ({ x.with(|__s| __s.a) } == { y.with(|__s| __s.a) });
}
pub fn operator_ne_122(x: Ptr<OutOfLine>, y: Ptr<OutOfLine>) -> bool {
    return !({
        let _x: Ptr<OutOfLine> = (x).clone();
        let _y: Ptr<OutOfLine> = (y).clone();
        operator_eq_121(_x, _y)
    });
}
pub fn operator_eq_124(x: Ptr<Tmpl_123>, y: Ptr<Tmpl_123>) -> bool {
    return ({ x.with(|__s| __s.v) } == { y.with(|__s| __s.v) });
}
pub fn operator_lt_125(x: Ptr<Tmpl_123>, y: Ptr<Tmpl_123>) -> bool {
    return ({ x.with(|__s| __s.v) } < { y.with(|__s| __s.v) });
}
pub fn operator_eq_126(x: Ptr<Tmpl_123>, y: Ptr<Tmpl_127>) -> bool {
    return ({ (x.with(|__s| __s.v) as i64) } == { y.with(|__s| __s.v) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Tmpl_123 {
    #[offset(0)]
    pub v: i32,
}
impl std::cmp::Ord for Tmpl_123 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            if operator_lt_125(
                Rc::new(RefCell::new(Tmpl_123 { v: self.v.clone() })).as_pointer(),
                Rc::new(RefCell::new(Tmpl_123 { v: other.v.clone() })).as_pointer(),
            ) {
                std::cmp::Ordering::Less
            } else if operator_lt_125(
                Rc::new(RefCell::new(Tmpl_123 { v: other.v.clone() })).as_pointer(),
                Rc::new(RefCell::new(Tmpl_123 { v: self.v.clone() })).as_pointer(),
            ) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for Tmpl_123 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for Tmpl_123 {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_124(
                Rc::new(RefCell::new(Tmpl_123 { v: self.v.clone() })).as_pointer(),
                Rc::new(RefCell::new(Tmpl_123 { v: other.v.clone() })).as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for Tmpl_123 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Tmpl_127 {
    #[offset(0)]
    pub v: i64,
}
pub fn operator_eq_129(_a0: Ptr<TmplDefaulted_128>, _a1: Ptr<TmplDefaulted_128>) -> bool {
    return ({ _a0.with(|__s| __s.v) } == { _a1.with(|__s| __s.v) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct TmplDefaulted_128 {
    #[offset(0)]
    pub v: i32,
}
impl std::cmp::PartialEq for TmplDefaulted_128 {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_129(
                Rc::new(RefCell::new(TmplDefaulted_128 { v: self.v.clone() })).as_pointer(),
                Rc::new(RefCell::new(TmplDefaulted_128 { v: other.v.clone() })).as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for TmplDefaulted_128 {}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let d1: Value<Defaulted> = Rc::new(RefCell::new(Defaulted { a: 1, b: 2 }));
    let d2: Value<Defaulted> = Rc::new(RefCell::new(Defaulted { a: 1, b: 2 }));
    let d3: Value<Defaulted> = Rc::new(RefCell::new(Defaulted { a: 1, b: 3 }));
    (if ({
        let _arg0: Ptr<Defaulted> = d1.as_pointer();
        operator_eq_114(_arg0, d2.as_pointer())
    }) {
        &(0);
    } else {
        ({
            __assert_fail_130(
                Ptr::<i8>::from_string_literal(b"d1 == d2"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if !({
        let _arg0: Ptr<Defaulted> = d1.as_pointer();
        operator_eq_114(_arg0, d3.as_pointer())
    }) {
        &(0);
    } else {
        ({
            __assert_fail_130(
                Ptr::<i8>::from_string_literal(b"d1 != d3"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if !({
        let _arg0: Ptr<Defaulted> = d1.as_pointer();
        operator_eq_114(_arg0, d3.as_pointer())
    }) {
        &(0);
    } else {
        ({
            __assert_fail_130(
                Ptr::<i8>::from_string_literal(b"!(d1 == d3)"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let o1: Value<DefaultedOrd> = Rc::new(RefCell::new(DefaultedOrd { a: 1 }));
    let o2: Value<DefaultedOrd> = Rc::new(RefCell::new(DefaultedOrd { a: 2 }));
    (if ({
        let ___v: std_strong_ordering = ({
            let _arg0: Ptr<DefaultedOrd> = o1.as_pointer();
            operator_cmp_115(_arg0, o2.as_pointer())
        });
        operator_lt_131(
            ___v,
            std___cmp_cat___unspec::new({ Ptr::<std___cmp_cat___unspec>::null() }),
        )
    }) {
        &(0);
    } else {
        ({
            __assert_fail_130(
                Ptr::<i8>::from_string_literal(b"o1 < o2"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ({
        let ___v: std_strong_ordering = ({
            let _arg0: Ptr<DefaultedOrd> = o2.as_pointer();
            operator_cmp_115(_arg0, o1.as_pointer())
        });
        operator_gt_132(
            ___v,
            std___cmp_cat___unspec::new({ Ptr::<std___cmp_cat___unspec>::null() }),
        )
    }) {
        &(0);
    } else {
        ({
            __assert_fail_130(
                Ptr::<i8>::from_string_literal(b"o2 > o1"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ({
        let _arg0: Ptr<DefaultedOrd> = o1.as_pointer();
        let _arg1: Ptr<DefaultedOrd> = o1.as_pointer();
        operator_eq_117(_arg0, _arg1)
    }) {
        &(0);
    } else {
        ({
            __assert_fail_130(
                Ptr::<i8>::from_string_literal(b"o1 == o1"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ({
        let _arg0: std_strong_ordering = ({
            let _arg0: Ptr<DefaultedOrd> = o1.as_pointer();
            operator_cmp_115(_arg0, o2.as_pointer())
        });
        operator_eq_133(
            _arg0,
            (*std_strong_ordering_less_104.with(Value::clone).borrow()).clone(),
        )
    }) {
        &(0);
    } else {
        ({
            __assert_fail_130(
                Ptr::<i8>::from_string_literal(b"(o1 <=> o2) == std::strong_ordering::less"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let i1: Value<Inline> = Rc::new(RefCell::new(Inline { a: 1 }));
    let i2: Value<Inline> = Rc::new(RefCell::new(Inline { a: 2 }));
    let i3: Value<Inline> = Rc::new(RefCell::new(Inline { a: 1 }));
    (if ({
        let _x: Ptr<Inline> = i1.as_pointer();
        operator_eq_118(_x, i3.as_pointer())
    }) {
        &(0);
    } else {
        ({
            __assert_fail_130(
                Ptr::<i8>::from_string_literal(b"i1 == i3"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if !({
        let _x: Ptr<Inline> = i1.as_pointer();
        operator_eq_118(_x, i2.as_pointer())
    }) {
        &(0);
    } else {
        ({
            __assert_fail_130(
                Ptr::<i8>::from_string_literal(b"!(i1 == i2)"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ({
        let _x: Ptr<Inline> = i1.as_pointer();
        operator_lt_119(_x, i2.as_pointer())
    }) {
        &(0);
    } else {
        ({
            __assert_fail_130(
                Ptr::<i8>::from_string_literal(b"i1 < i2"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if !({
        let _x: Ptr<Inline> = i2.as_pointer();
        operator_lt_119(_x, i1.as_pointer())
    }) {
        &(0);
    } else {
        ({
            __assert_fail_130(
                Ptr::<i8>::from_string_literal(b"!(i2 < i1)"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ({
        let _x: Value<Inline> = Rc::new(RefCell::new(
            ({
                let _x: Ptr<Inline> = i1.as_pointer();
                operator_add_120(_x, i2.as_pointer())
            }),
        ));
        let _y: Value<Inline> = Rc::new(RefCell::new(Inline { a: 3 }));
        operator_eq_118(_x.as_pointer(), _y.as_pointer())
    }) {
        &(0);
    } else {
        ({
            __assert_fail_130(
                Ptr::<i8>::from_string_literal(b"i1 + i2 == Inline{3}"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let f1: Value<OutOfLine> = Rc::new(RefCell::new(OutOfLine { a: 4 }));
    let f2: Value<OutOfLine> = Rc::new(RefCell::new(OutOfLine { a: 4 }));
    let f3: Value<OutOfLine> = Rc::new(RefCell::new(OutOfLine { a: 5 }));
    (if ({
        let _x: Ptr<OutOfLine> = f1.as_pointer();
        operator_eq_121(_x, f2.as_pointer())
    }) {
        &(0);
    } else {
        ({
            __assert_fail_130(
                Ptr::<i8>::from_string_literal(b"f1 == f2"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ({
        let _x: Ptr<OutOfLine> = f1.as_pointer();
        operator_ne_122(_x, f3.as_pointer())
    }) {
        &(0);
    } else {
        ({
            __assert_fail_130(
                Ptr::<i8>::from_string_literal(b"f1 != f3"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if !({
        let _x: Ptr<OutOfLine> = f1.as_pointer();
        operator_eq_121(_x, f3.as_pointer())
    }) {
        &(0);
    } else {
        ({
            __assert_fail_130(
                Ptr::<i8>::from_string_literal(b"!(f1 == f3)"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let t1: Value<Tmpl_123> = Rc::new(RefCell::new(Tmpl_123 { v: 1 }));
    let t2: Value<Tmpl_123> = Rc::new(RefCell::new(Tmpl_123 { v: 2 }));
    let t3: Value<Tmpl_123> = Rc::new(RefCell::new(Tmpl_123 { v: 1 }));
    (if ({
        let _x: Ptr<Tmpl_123> = t1.as_pointer();
        operator_eq_124(_x, t3.as_pointer())
    }) {
        &(0);
    } else {
        ({
            __assert_fail_130(
                Ptr::<i8>::from_string_literal(b"t1 == t3"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if !({
        let _x: Ptr<Tmpl_123> = t1.as_pointer();
        operator_eq_124(_x, t2.as_pointer())
    }) {
        &(0);
    } else {
        ({
            __assert_fail_130(
                Ptr::<i8>::from_string_literal(b"!(t1 == t2)"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ({
        let _x: Ptr<Tmpl_123> = t1.as_pointer();
        operator_lt_125(_x, t2.as_pointer())
    }) {
        &(0);
    } else {
        ({
            __assert_fail_130(
                Ptr::<i8>::from_string_literal(b"t1 < t2"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let u1: Value<Tmpl_127> = Rc::new(RefCell::new(Tmpl_127 { v: 1_i64 }));
    let u2: Value<Tmpl_127> = Rc::new(RefCell::new(Tmpl_127 { v: 2_i64 }));
    (if ({
        let _x: Ptr<Tmpl_123> = t1.as_pointer();
        operator_eq_126(_x, u1.as_pointer())
    }) {
        &(0);
    } else {
        ({
            __assert_fail_130(
                Ptr::<i8>::from_string_literal(b"t1 == u1"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if !({
        let _x: Ptr<Tmpl_123> = t1.as_pointer();
        operator_eq_126(_x, u2.as_pointer())
    }) {
        &(0);
    } else {
        ({
            __assert_fail_130(
                Ptr::<i8>::from_string_literal(b"!(t1 == u2)"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let v1: Value<TmplDefaulted_128> = Rc::new(RefCell::new(TmplDefaulted_128 { v: 7 }));
    let v2: Value<TmplDefaulted_128> = Rc::new(RefCell::new(TmplDefaulted_128 { v: 7 }));
    let v3: Value<TmplDefaulted_128> = Rc::new(RefCell::new(TmplDefaulted_128 { v: 8 }));
    (if ({
        let _arg0: Ptr<TmplDefaulted_128> = v1.as_pointer();
        operator_eq_129(_arg0, v2.as_pointer())
    }) {
        &(0);
    } else {
        ({
            __assert_fail_130(
                Ptr::<i8>::from_string_literal(b"v1 == v2"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if !({
        let _arg0: Ptr<TmplDefaulted_128> = v1.as_pointer();
        operator_eq_129(_arg0, v3.as_pointer())
    }) {
        &(0);
    } else {
        ({
            __assert_fail_130(
                Ptr::<i8>::from_string_literal(b"v1 != v3"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if !({
        let _arg0: Ptr<TmplDefaulted_128> = v1.as_pointer();
        operator_eq_129(_arg0, v3.as_pointer())
    }) {
        &(0);
    } else {
        ({
            __assert_fail_130(
                Ptr::<i8>::from_string_literal(b"!(v1 == v3)"),
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
pub fn operator_eq_116(__v: std_strong_ordering, _a1: std___cmp_cat___unspec) -> bool {
    unimplemented!()
}
pub fn __assert_fail_130(
    mut __assertion: Ptr<i8>,
    mut __file: Ptr<i8>,
    mut __line: u32,
    mut __function: Ptr<i8>,
) {
    unimplemented!()
}
pub fn operator_lt_131(__v: std_strong_ordering, _a1: std___cmp_cat___unspec) -> bool {
    unimplemented!()
}
pub fn operator_gt_132(__v: std_strong_ordering, _a1: std___cmp_cat___unspec) -> bool {
    unimplemented!()
}
pub fn operator_eq_133(_a0: std_strong_ordering, _a1: std_strong_ordering) -> bool {
    unimplemented!()
}
pub fn __cpp2rust_init_globals() {}
