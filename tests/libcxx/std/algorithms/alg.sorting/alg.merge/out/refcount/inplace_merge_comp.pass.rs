extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_is_valid_integer_type_for_random_signed_char_ {}
impl CppMove for support_detail_is_valid_integer_type_for_random_signed_char_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_is_valid_integer_type_for_random_short_ {}
impl CppMove for support_detail_is_valid_integer_type_for_random_short_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_is_valid_integer_type_for_random_int_ {}
impl CppMove for support_detail_is_valid_integer_type_for_random_int_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_is_valid_integer_type_for_random_long_ {}
impl CppMove for support_detail_is_valid_integer_type_for_random_long_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_is_valid_integer_type_for_random_long_long_ {}
impl CppMove for support_detail_is_valid_integer_type_for_random_long_long_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_is_valid_integer_type_for_random_unsigned_char_ {}
impl CppMove for support_detail_is_valid_integer_type_for_random_unsigned_char_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_is_valid_integer_type_for_random_unsigned_short_ {}
impl CppMove for support_detail_is_valid_integer_type_for_random_unsigned_short_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_is_valid_integer_type_for_random_unsigned_int_ {}
impl CppMove for support_detail_is_valid_integer_type_for_random_unsigned_int_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_is_valid_integer_type_for_random_unsigned_long_ {}
impl CppMove for support_detail_is_valid_integer_type_for_random_unsigned_long_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_is_valid_integer_type_for_random_unsigned_long_long_ {}
impl CppMove for support_detail_is_valid_integer_type_for_random_unsigned_long_long_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_is_valid_integer_type_for_random___int128_ {}
impl CppMove for support_detail_is_valid_integer_type_for_random___int128_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_is_valid_integer_type_for_random_unsigned___int128_ {}
impl CppMove for support_detail_is_valid_integer_type_for_random_unsigned___int128_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_is_valid_urng_support_simple_random_generator__void_ {}
impl CppMove for support_detail_is_valid_urng_support_simple_random_generator__void_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static value_0: Value<usize> = Rc::new(RefCell::new(
        (if ((65536_u64 & (((1) as u64) << 63_u64)) != 0) {
            63_u64
        } else {
            (value_1.with(|rc| *rc.borrow()) as u64)
        } as usize),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__63_ {}
impl CppMove for support_detail_meta_log2_imp_unsigned_long_long__65536__63_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static value_1: Value<usize> = Rc::new(RefCell::new(
        (if ((65536_u64 & (((1) as u64) << 62_u64)) != 0) {
            62_u64
        } else {
            (value_2.with(|rc| *rc.borrow()) as u64)
        } as usize),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__62_ {}
impl CppMove for support_detail_meta_log2_imp_unsigned_long_long__65536__62_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static value_2: Value<usize> = Rc::new(RefCell::new(
        (if ((65536_u64 & (((1) as u64) << 61_u64)) != 0) {
            61_u64
        } else {
            (value_3.with(|rc| *rc.borrow()) as u64)
        } as usize),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__61_ {}
impl CppMove for support_detail_meta_log2_imp_unsigned_long_long__65536__61_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static value_3: Value<usize> = Rc::new(RefCell::new(
        (if ((65536_u64 & (((1) as u64) << 60_u64)) != 0) {
            60_u64
        } else {
            (value_4.with(|rc| *rc.borrow()) as u64)
        } as usize),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__60_ {}
impl CppMove for support_detail_meta_log2_imp_unsigned_long_long__65536__60_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static value_4: Value<usize> = Rc::new(RefCell::new(
        (if ((65536_u64 & (((1) as u64) << 59_u64)) != 0) {
            59_u64
        } else {
            (value_5.with(|rc| *rc.borrow()) as u64)
        } as usize),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__59_ {}
impl CppMove for support_detail_meta_log2_imp_unsigned_long_long__65536__59_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static value_5: Value<usize> = Rc::new(RefCell::new(
        (if ((65536_u64 & (((1) as u64) << 58_u64)) != 0) {
            58_u64
        } else {
            (value_6.with(|rc| *rc.borrow()) as u64)
        } as usize),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__58_ {}
impl CppMove for support_detail_meta_log2_imp_unsigned_long_long__65536__58_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static value_6: Value<usize> = Rc::new(RefCell::new(
        (if ((65536_u64 & (((1) as u64) << 57_u64)) != 0) {
            57_u64
        } else {
            (value_7.with(|rc| *rc.borrow()) as u64)
        } as usize),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__57_ {}
impl CppMove for support_detail_meta_log2_imp_unsigned_long_long__65536__57_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static value_7: Value<usize> = Rc::new(RefCell::new(
        (if ((65536_u64 & (((1) as u64) << 56_u64)) != 0) {
            56_u64
        } else {
            (value_8.with(|rc| *rc.borrow()) as u64)
        } as usize),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__56_ {}
impl CppMove for support_detail_meta_log2_imp_unsigned_long_long__65536__56_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static value_8: Value<usize> = Rc::new(RefCell::new(
        (if ((65536_u64 & (((1) as u64) << 55_u64)) != 0) {
            55_u64
        } else {
            (value_9.with(|rc| *rc.borrow()) as u64)
        } as usize),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__55_ {}
impl CppMove for support_detail_meta_log2_imp_unsigned_long_long__65536__55_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static value_9: Value<usize> = Rc::new(RefCell::new(
        (if ((65536_u64 & (((1) as u64) << 54_u64)) != 0) {
            54_u64
        } else {
            (value_10.with(|rc| *rc.borrow()) as u64)
        } as usize),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__54_ {}
impl CppMove for support_detail_meta_log2_imp_unsigned_long_long__65536__54_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static value_10: Value<usize> = Rc::new(RefCell::new(
        (if ((65536_u64 & (((1) as u64) << 53_u64)) != 0) {
            53_u64
        } else {
            (value_11.with(|rc| *rc.borrow()) as u64)
        } as usize),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__53_ {}
impl CppMove for support_detail_meta_log2_imp_unsigned_long_long__65536__53_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static value_11: Value<usize> = Rc::new(RefCell::new(
        (if ((65536_u64 & (((1) as u64) << 52_u64)) != 0) {
            52_u64
        } else {
            (value_12.with(|rc| *rc.borrow()) as u64)
        } as usize),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__52_ {}
impl CppMove for support_detail_meta_log2_imp_unsigned_long_long__65536__52_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static value_12: Value<usize> = Rc::new(RefCell::new(
        (if ((65536_u64 & (((1) as u64) << 51_u64)) != 0) {
            51_u64
        } else {
            (value_13.with(|rc| *rc.borrow()) as u64)
        } as usize),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__51_ {}
impl CppMove for support_detail_meta_log2_imp_unsigned_long_long__65536__51_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static value_13: Value<usize> = Rc::new(RefCell::new(
        (if ((65536_u64 & (((1) as u64) << 50_u64)) != 0) {
            50_u64
        } else {
            (value_14.with(|rc| *rc.borrow()) as u64)
        } as usize),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__50_ {}
impl CppMove for support_detail_meta_log2_imp_unsigned_long_long__65536__50_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static value_14: Value<usize> = Rc::new(RefCell::new(
        (if ((65536_u64 & (((1) as u64) << 49_u64)) != 0) {
            49_u64
        } else {
            (value_15.with(|rc| *rc.borrow()) as u64)
        } as usize),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__49_ {}
impl CppMove for support_detail_meta_log2_imp_unsigned_long_long__65536__49_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static value_15: Value<usize> = Rc::new(RefCell::new(
        (if ((65536_u64 & (((1) as u64) << 48_u64)) != 0) {
            48_u64
        } else {
            (value_16.with(|rc| *rc.borrow()) as u64)
        } as usize),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__48_ {}
impl CppMove for support_detail_meta_log2_imp_unsigned_long_long__65536__48_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static value_16: Value<usize> = Rc::new(RefCell::new(
        (if ((65536_u64 & (((1) as u64) << 47_u64)) != 0) {
            47_u64
        } else {
            (value_17.with(|rc| *rc.borrow()) as u64)
        } as usize),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__47_ {}
impl CppMove for support_detail_meta_log2_imp_unsigned_long_long__65536__47_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static value_17: Value<usize> = Rc::new(RefCell::new(
        (if ((65536_u64 & (((1) as u64) << 46_u64)) != 0) {
            46_u64
        } else {
            (value_18.with(|rc| *rc.borrow()) as u64)
        } as usize),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__46_ {}
impl CppMove for support_detail_meta_log2_imp_unsigned_long_long__65536__46_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static value_18: Value<usize> = Rc::new(RefCell::new(
        (if ((65536_u64 & (((1) as u64) << 45_u64)) != 0) {
            45_u64
        } else {
            (value_19.with(|rc| *rc.borrow()) as u64)
        } as usize),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__45_ {}
impl CppMove for support_detail_meta_log2_imp_unsigned_long_long__65536__45_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static value_19: Value<usize> = Rc::new(RefCell::new(
        (if ((65536_u64 & (((1) as u64) << 44_u64)) != 0) {
            44_u64
        } else {
            (value_20.with(|rc| *rc.borrow()) as u64)
        } as usize),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__44_ {}
impl CppMove for support_detail_meta_log2_imp_unsigned_long_long__65536__44_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static value_20: Value<usize> = Rc::new(RefCell::new(
        (if ((65536_u64 & (((1) as u64) << 43_u64)) != 0) {
            43_u64
        } else {
            (value_21.with(|rc| *rc.borrow()) as u64)
        } as usize),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__43_ {}
impl CppMove for support_detail_meta_log2_imp_unsigned_long_long__65536__43_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static value_21: Value<usize> = Rc::new(RefCell::new(
        (if ((65536_u64 & (((1) as u64) << 42_u64)) != 0) {
            42_u64
        } else {
            (value_22.with(|rc| *rc.borrow()) as u64)
        } as usize),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__42_ {}
impl CppMove for support_detail_meta_log2_imp_unsigned_long_long__65536__42_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static value_22: Value<usize> = Rc::new(RefCell::new(
        (if ((65536_u64 & (((1) as u64) << 41_u64)) != 0) {
            41_u64
        } else {
            (value_23.with(|rc| *rc.borrow()) as u64)
        } as usize),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__41_ {}
impl CppMove for support_detail_meta_log2_imp_unsigned_long_long__65536__41_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static value_23: Value<usize> = Rc::new(RefCell::new(
        (if ((65536_u64 & (((1) as u64) << 40_u64)) != 0) {
            40_u64
        } else {
            (value_24.with(|rc| *rc.borrow()) as u64)
        } as usize),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__40_ {}
impl CppMove for support_detail_meta_log2_imp_unsigned_long_long__65536__40_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static value_24: Value<usize> = Rc::new(RefCell::new(
        (if ((65536_u64 & (((1) as u64) << 39_u64)) != 0) {
            39_u64
        } else {
            (value_25.with(|rc| *rc.borrow()) as u64)
        } as usize),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__39_ {}
impl CppMove for support_detail_meta_log2_imp_unsigned_long_long__65536__39_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static value_25: Value<usize> = Rc::new(RefCell::new(
        (if ((65536_u64 & (((1) as u64) << 38_u64)) != 0) {
            38_u64
        } else {
            (value_26.with(|rc| *rc.borrow()) as u64)
        } as usize),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__38_ {}
impl CppMove for support_detail_meta_log2_imp_unsigned_long_long__65536__38_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static value_26: Value<usize> = Rc::new(RefCell::new(
        (if ((65536_u64 & (((1) as u64) << 37_u64)) != 0) {
            37_u64
        } else {
            (value_27.with(|rc| *rc.borrow()) as u64)
        } as usize),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__37_ {}
impl CppMove for support_detail_meta_log2_imp_unsigned_long_long__65536__37_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static value_27: Value<usize> = Rc::new(RefCell::new(
        (if ((65536_u64 & (((1) as u64) << 36_u64)) != 0) {
            36_u64
        } else {
            (value_28.with(|rc| *rc.borrow()) as u64)
        } as usize),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__36_ {}
impl CppMove for support_detail_meta_log2_imp_unsigned_long_long__65536__36_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static value_28: Value<usize> = Rc::new(RefCell::new(
        (if ((65536_u64 & (((1) as u64) << 35_u64)) != 0) {
            35_u64
        } else {
            (value_29.with(|rc| *rc.borrow()) as u64)
        } as usize),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__35_ {}
impl CppMove for support_detail_meta_log2_imp_unsigned_long_long__65536__35_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static value_29: Value<usize> = Rc::new(RefCell::new(
        (if ((65536_u64 & (((1) as u64) << 34_u64)) != 0) {
            34_u64
        } else {
            (value_30.with(|rc| *rc.borrow()) as u64)
        } as usize),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__34_ {}
impl CppMove for support_detail_meta_log2_imp_unsigned_long_long__65536__34_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static value_30: Value<usize> = Rc::new(RefCell::new(
        (if ((65536_u64 & (((1) as u64) << 33_u64)) != 0) {
            33_u64
        } else {
            (value_31.with(|rc| *rc.borrow()) as u64)
        } as usize),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__33_ {}
impl CppMove for support_detail_meta_log2_imp_unsigned_long_long__65536__33_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static value_31: Value<usize> = Rc::new(RefCell::new(
        (if ((65536_u64 & (((1) as u64) << 32_u64)) != 0) {
            32_u64
        } else {
            (value_32.with(|rc| *rc.borrow()) as u64)
        } as usize),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__32_ {}
impl CppMove for support_detail_meta_log2_imp_unsigned_long_long__65536__32_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static value_32: Value<usize> = Rc::new(RefCell::new(
        (if ((65536_u64 & (((1) as u64) << 31_u64)) != 0) {
            31_u64
        } else {
            (value_33.with(|rc| *rc.borrow()) as u64)
        } as usize),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__31_ {}
impl CppMove for support_detail_meta_log2_imp_unsigned_long_long__65536__31_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static value_33: Value<usize> = Rc::new(RefCell::new(
        (if ((65536_u64 & (((1) as u64) << 30_u64)) != 0) {
            30_u64
        } else {
            (value_34.with(|rc| *rc.borrow()) as u64)
        } as usize),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__30_ {}
impl CppMove for support_detail_meta_log2_imp_unsigned_long_long__65536__30_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static value_34: Value<usize> = Rc::new(RefCell::new(
        (if ((65536_u64 & (((1) as u64) << 29_u64)) != 0) {
            29_u64
        } else {
            (value_35.with(|rc| *rc.borrow()) as u64)
        } as usize),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__29_ {}
impl CppMove for support_detail_meta_log2_imp_unsigned_long_long__65536__29_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static value_35: Value<usize> = Rc::new(RefCell::new(
        (if ((65536_u64 & (((1) as u64) << 28_u64)) != 0) {
            28_u64
        } else {
            (value_36.with(|rc| *rc.borrow()) as u64)
        } as usize),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__28_ {}
impl CppMove for support_detail_meta_log2_imp_unsigned_long_long__65536__28_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static value_36: Value<usize> = Rc::new(RefCell::new(
        (if ((65536_u64 & (((1) as u64) << 27_u64)) != 0) {
            27_u64
        } else {
            (value_37.with(|rc| *rc.borrow()) as u64)
        } as usize),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__27_ {}
impl CppMove for support_detail_meta_log2_imp_unsigned_long_long__65536__27_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static value_37: Value<usize> = Rc::new(RefCell::new(
        (if ((65536_u64 & (((1) as u64) << 26_u64)) != 0) {
            26_u64
        } else {
            (value_38.with(|rc| *rc.borrow()) as u64)
        } as usize),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__26_ {}
impl CppMove for support_detail_meta_log2_imp_unsigned_long_long__65536__26_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static value_38: Value<usize> = Rc::new(RefCell::new(
        (if ((65536_u64 & (((1) as u64) << 25_u64)) != 0) {
            25_u64
        } else {
            (value_39.with(|rc| *rc.borrow()) as u64)
        } as usize),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__25_ {}
impl CppMove for support_detail_meta_log2_imp_unsigned_long_long__65536__25_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static value_39: Value<usize> = Rc::new(RefCell::new(
        (if ((65536_u64 & (((1) as u64) << 24_u64)) != 0) {
            24_u64
        } else {
            (value_40.with(|rc| *rc.borrow()) as u64)
        } as usize),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__24_ {}
impl CppMove for support_detail_meta_log2_imp_unsigned_long_long__65536__24_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static value_40: Value<usize> = Rc::new(RefCell::new(
        (if ((65536_u64 & (((1) as u64) << 23_u64)) != 0) {
            23_u64
        } else {
            (value_41.with(|rc| *rc.borrow()) as u64)
        } as usize),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__23_ {}
impl CppMove for support_detail_meta_log2_imp_unsigned_long_long__65536__23_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static value_41: Value<usize> = Rc::new(RefCell::new(
        (if ((65536_u64 & (((1) as u64) << 22_u64)) != 0) {
            22_u64
        } else {
            (value_42.with(|rc| *rc.borrow()) as u64)
        } as usize),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__22_ {}
impl CppMove for support_detail_meta_log2_imp_unsigned_long_long__65536__22_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static value_42: Value<usize> = Rc::new(RefCell::new(
        (if ((65536_u64 & (((1) as u64) << 21_u64)) != 0) {
            21_u64
        } else {
            (value_43.with(|rc| *rc.borrow()) as u64)
        } as usize),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__21_ {}
impl CppMove for support_detail_meta_log2_imp_unsigned_long_long__65536__21_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static value_43: Value<usize> = Rc::new(RefCell::new(
        (if ((65536_u64 & (((1) as u64) << 20_u64)) != 0) {
            20_u64
        } else {
            (value_44.with(|rc| *rc.borrow()) as u64)
        } as usize),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__20_ {}
impl CppMove for support_detail_meta_log2_imp_unsigned_long_long__65536__20_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static value_44: Value<usize> = Rc::new(RefCell::new(
        (if ((65536_u64 & (((1) as u64) << 19_u64)) != 0) {
            19_u64
        } else {
            (value_45.with(|rc| *rc.borrow()) as u64)
        } as usize),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__19_ {}
impl CppMove for support_detail_meta_log2_imp_unsigned_long_long__65536__19_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static value_45: Value<usize> = Rc::new(RefCell::new(
        (if ((65536_u64 & (((1) as u64) << 18_u64)) != 0) {
            18_u64
        } else {
            (value_46.with(|rc| *rc.borrow()) as u64)
        } as usize),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__18_ {}
impl CppMove for support_detail_meta_log2_imp_unsigned_long_long__65536__18_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static value_46: Value<usize> = Rc::new(RefCell::new(
        (if ((65536_u64 & (((1) as u64) << 17_u64)) != 0) {
            17_u64
        } else {
            (value_47.with(|rc| *rc.borrow()) as u64)
        } as usize),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__17_ {}
impl CppMove for support_detail_meta_log2_imp_unsigned_long_long__65536__17_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static value_47: Value<usize> = Rc::new(RefCell::new(
        (if ((65536_u64 & (((1) as u64) << 16_u64)) != 0) {
            16_u64
        } else {
            (value_48.with(|rc| *rc.borrow()) as u64)
        } as usize),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__16_ {}
impl CppMove for support_detail_meta_log2_imp_unsigned_long_long__65536__16_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static value_48: Value<usize> = Rc::new(RefCell::new(
        (if ((65536_u64 & (((1) as u64) << 15_u64)) != 0) {
            15_u64
        } else {
            (value_49.with(|rc| *rc.borrow()) as u64)
        } as usize),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__15_ {}
impl CppMove for support_detail_meta_log2_imp_unsigned_long_long__65536__15_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static value_49: Value<usize> = Rc::new(RefCell::new(
        (if ((65536_u64 & (((1) as u64) << 14_u64)) != 0) {
            14_u64
        } else {
            (value_50.with(|rc| *rc.borrow()) as u64)
        } as usize),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__14_ {}
impl CppMove for support_detail_meta_log2_imp_unsigned_long_long__65536__14_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static value_50: Value<usize> = Rc::new(RefCell::new(
        (if ((65536_u64 & (((1) as u64) << 13_u64)) != 0) {
            13_u64
        } else {
            (value_51.with(|rc| *rc.borrow()) as u64)
        } as usize),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__13_ {}
impl CppMove for support_detail_meta_log2_imp_unsigned_long_long__65536__13_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static value_51: Value<usize> = Rc::new(RefCell::new(
        (if ((65536_u64 & (((1) as u64) << 12_u64)) != 0) {
            12_u64
        } else {
            (value_52.with(|rc| *rc.borrow()) as u64)
        } as usize),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__12_ {}
impl CppMove for support_detail_meta_log2_imp_unsigned_long_long__65536__12_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static value_52: Value<usize> = Rc::new(RefCell::new(
        (if ((65536_u64 & (((1) as u64) << 11_u64)) != 0) {
            11_u64
        } else {
            (value_53.with(|rc| *rc.borrow()) as u64)
        } as usize),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__11_ {}
impl CppMove for support_detail_meta_log2_imp_unsigned_long_long__65536__11_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static value_53: Value<usize> = Rc::new(RefCell::new(
        (if ((65536_u64 & (((1) as u64) << 10_u64)) != 0) {
            10_u64
        } else {
            (value_54.with(|rc| *rc.borrow()) as u64)
        } as usize),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__10_ {}
impl CppMove for support_detail_meta_log2_imp_unsigned_long_long__65536__10_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static value_54: Value<usize> = Rc::new(RefCell::new(
        (if ((65536_u64 & (((1) as u64) << 9_u64)) != 0) {
            9_u64
        } else {
            (value_55.with(|rc| *rc.borrow()) as u64)
        } as usize),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__9_ {}
impl CppMove for support_detail_meta_log2_imp_unsigned_long_long__65536__9_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static value_55: Value<usize> = Rc::new(RefCell::new(
        (if ((65536_u64 & (((1) as u64) << 8_u64)) != 0) {
            8_u64
        } else {
            (value_56.with(|rc| *rc.borrow()) as u64)
        } as usize),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__8_ {}
impl CppMove for support_detail_meta_log2_imp_unsigned_long_long__65536__8_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static value_56: Value<usize> = Rc::new(RefCell::new(
        (if ((65536_u64 & (((1) as u64) << 7_u64)) != 0) {
            7_u64
        } else {
            (value_57.with(|rc| *rc.borrow()) as u64)
        } as usize),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__7_ {}
impl CppMove for support_detail_meta_log2_imp_unsigned_long_long__65536__7_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static value_57: Value<usize> = Rc::new(RefCell::new(
        (if ((65536_u64 & (((1) as u64) << 6_u64)) != 0) {
            6_u64
        } else {
            (value_58.with(|rc| *rc.borrow()) as u64)
        } as usize),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__6_ {}
impl CppMove for support_detail_meta_log2_imp_unsigned_long_long__65536__6_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static value_58: Value<usize> = Rc::new(RefCell::new(
        (if ((65536_u64 & (((1) as u64) << 5_u64)) != 0) {
            5_u64
        } else {
            (value_59.with(|rc| *rc.borrow()) as u64)
        } as usize),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__5_ {}
impl CppMove for support_detail_meta_log2_imp_unsigned_long_long__65536__5_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static value_59: Value<usize> = Rc::new(RefCell::new(
        (if ((65536_u64 & (((1) as u64) << 4_u64)) != 0) {
            4_u64
        } else {
            (value_60.with(|rc| *rc.borrow()) as u64)
        } as usize),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__4_ {}
impl CppMove for support_detail_meta_log2_imp_unsigned_long_long__65536__4_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static value_60: Value<usize> = Rc::new(RefCell::new(
        (if ((65536_u64 & (((1) as u64) << 3_u64)) != 0) {
            3_u64
        } else {
            (value_61.with(|rc| *rc.borrow()) as u64)
        } as usize),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__3_ {}
impl CppMove for support_detail_meta_log2_imp_unsigned_long_long__65536__3_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static value_61: Value<usize> = Rc::new(RefCell::new(
        (if ((65536_u64 & (((1) as u64) << 2_u64)) != 0) {
            2_u64
        } else {
            (value_62.with(|rc| *rc.borrow()) as u64)
        } as usize),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__2_ {}
impl CppMove for support_detail_meta_log2_imp_unsigned_long_long__65536__2_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static value_62: Value<usize> = Rc::new(RefCell::new(
        (if ((65536_u64 & (((1) as u64) << 1_u64)) != 0) {
            1_u64
        } else {
            (value_63.with(|rc| *rc.borrow()) as u64)
        } as usize),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__1_ {}
impl CppMove for support_detail_meta_log2_imp_unsigned_long_long__65536__1_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static value_63: Value<usize> = Rc::new(RefCell::new(0_usize));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__0_ {}
impl CppMove for support_detail_meta_log2_imp_unsigned_long_long__65536__0_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static value_64: Value<usize> = Rc::new(RefCell::new(value_0.with(|rc| *rc.borrow())));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_unsigned_long__65536_ {}
impl CppMove for support_detail_meta_log2_unsigned_long__65536_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn countl_zero_65(mut n: u32) -> i32 {
    return n.leading_zeros() as i32;
}
pub fn countl_zero_66(mut n: u64) -> i32 {
    return n.leading_zeros() as i32;
}
thread_local!(
    static rp_67: Value<u64> = Rc::new(RefCell::new(65536_u64));
);
thread_local!(
    static rp_log2_68: Value<usize> = Rc::new(RefCell::new(16_usize));
);
thread_local!(
    static w_digits_69: Value<usize> = Rc::new(RefCell::new(64_usize));
);
thread_local!(
    static e_digits_70: Value<usize> = Rc::new(RefCell::new(16_usize));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(64)]
pub struct support_detail_independent_bits_engine_support_simple_random_generator__unsigned_long_ {
    #[offset(0)]
    #[byte_size(8)]
    eng_: Ptr<support_simple_random_generator>,
    #[offset(8)]
    width_: usize,
    #[offset(16)]
    wid0_: usize,
    #[offset(24)]
    round_count_all_: usize,
    #[offset(32)]
    round_count_regular_: usize,
    #[offset(40)]
    y0_: u64,
    #[offset(48)]
    y1_: u64,
    #[offset(56)]
    mask0_: u16,
    #[offset(58)]
    mask1_: u16,
}
impl support_detail_independent_bits_engine_support_simple_random_generator__unsigned_long_ {
    pub fn new(eng: Ptr<support_simple_random_generator>, mut width: usize) -> Self {
        let __this: Value<
            support_detail_independent_bits_engine_support_simple_random_generator__unsigned_long_,
        > = Rc::new(RefCell::new(Self {
            eng_: (eng).clone(),
            width_: width,
            wid0_: 0_usize,
            round_count_all_: 0_usize,
            round_count_regular_: 0_usize,
            y0_: 0_u64,
            y1_: 0_u64,
            mask0_: 0_u16,
            mask1_: 0_u16,
        }));
        let this: Ptr<
            support_detail_independent_bits_engine_support_simple_random_generator__unsigned_long_,
        > = __this.as_pointer();
        field!(this, round_count_all_).write(
            ((this.with(|__s| __s.width_)).wrapping_div(rp_log2_68.with(|rc| *rc.borrow())))
                .wrapping_add(
                    (((this.with(|__s| __s.width_))
                        .wrapping_rem(rp_log2_68.with(|rc| *rc.borrow()))
                        != 0_usize) as usize),
                ),
        );
        field!(this, wid0_).write(
            (this.with(|__s| __s.width_)).wrapping_div(this.with(|__s| __s.round_count_all_)),
        );
        {
            if (this.with(|__s| __s.wid0_) < w_digits_69.with(|rc| *rc.borrow())) {
                field!(this, y0_).write(
                    ((rp_67.with(|rc| *rc.borrow()) >> this.with(|__s| __s.wid0_))
                        << this.with(|__s| __s.wid0_)),
                );
            } else {
                field!(this, y0_).write(0_u64);
            }
        }
        if ((rp_67.with(|rc| *rc.borrow())).wrapping_sub(this.with(|__s| __s.y0_))
            > (this.with(|__s| __s.y0_))
                .wrapping_div((this.with(|__s| __s.round_count_all_) as u64)))
        {
            field!(this, round_count_all_).with_mut(|__v| {
                __v.prefix_inc();
            });
            field!(this, wid0_).write(
                (this.with(|__s| __s.width_)).wrapping_div(this.with(|__s| __s.round_count_all_)),
            );
            if (this.with(|__s| __s.wid0_) < w_digits_69.with(|rc| *rc.borrow())) {
                field!(this, y0_).write(
                    ((rp_67.with(|rc| *rc.borrow()) >> this.with(|__s| __s.wid0_))
                        << this.with(|__s| __s.wid0_)),
                );
            } else {
                field!(this, y0_).write(0_u64);
            }
        }
        field!(this, round_count_regular_).write(
            (this.with(|__s| __s.round_count_all_)).wrapping_sub(
                (this.with(|__s| __s.width_)).wrapping_rem(this.with(|__s| __s.round_count_all_)),
            ),
        );
        if (this.with(|__s| __s.wid0_)
            < (w_digits_69.with(|rc| *rc.borrow())).wrapping_sub(1_usize))
        {
            field!(this, y1_).write(
                ((rp_67.with(|rc| *rc.borrow())
                    >> ((this.with(|__s| __s.wid0_)).wrapping_add(1_usize)))
                    << ((this.with(|__s| __s.wid0_)).wrapping_add(1_usize))),
            );
        } else {
            field!(this, y1_).write(0_u64);
        }
        field!(this, mask0_).write(if (this.with(|__s| __s.wid0_) > 0_usize) {
            ((((!0 as u16) as i32)
                >> ((e_digits_70.with(|rc| *rc.borrow())).wrapping_sub(this.with(|__s| __s.wid0_))))
                as u16)
        } else {
            0_u16
        });
        field!(this, mask1_).write(
            if (this.with(|__s| __s.wid0_)
                < (e_digits_70.with(|rc| *rc.borrow())).wrapping_sub(1_usize))
            {
                ((((!0 as u16) as i32)
                    >> ((e_digits_70.with(|rc| *rc.borrow()))
                        .wrapping_sub(((this.with(|__s| __s.wid0_)).wrapping_add(1_usize)))))
                    as u16)
            } else {
                (!0 as u16)
            },
        );
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Callable0<u64>
    for support_detail_independent_bits_engine_support_simple_random_generator__unsigned_long_
{
    fn call(&self) -> u64 {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            support_detail_independent_bits_engine_support_simple_random_generator__unsigned_long_Impl::operator_call(__it)
        })
    }
}
impl CppMove
    for support_detail_independent_bits_engine_support_simple_random_generator__unsigned_long_
{
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn operator_eq_72(
    _a0: Ptr<support_uniform_int_distribution_long_>,
    _a1: Ptr<support_uniform_int_distribution_long_>,
) -> bool {
    return ({
        let _arg0: Ptr<support_uniform_int_distribution_long__param_type> = field_ptr!(_a0, param_);
        let _arg1: Ptr<support_uniform_int_distribution_long__param_type> = field_ptr!(_a1, param_);
        operator_eq_73(_arg0, _arg1)
    });
}
pub fn operator_eq_73(
    _a0: Ptr<support_uniform_int_distribution_long__param_type>,
    _a1: Ptr<support_uniform_int_distribution_long__param_type>,
) -> bool {
    return ({ _a0.with(|__s| __s.a_) } == { _a1.with(|__s| __s.a_) })
        && ({ _a0.with(|__s| __s.b_) } == { _a1.with(|__s| __s.b_) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(16)]
pub struct support_uniform_int_distribution_long__param_type {
    #[offset(0)]
    a_: i64,
    #[offset(8)]
    b_: i64,
}
impl support_uniform_int_distribution_long__param_type {
    pub fn new_1() -> Self {
        support_uniform_int_distribution_long__param_type::new_2({ 0_i64 }, None)
    }
    pub fn new_2(mut ax: i64, bx: Option<i64>) -> Self {
        let mut bx: i64 = bx.unwrap_or_else(|| i64::MAX);
        Self { a_: ax, b_: bx }
    }
}
impl CppEq for support_uniform_int_distribution_long__param_type {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_73(__lhs.clone(), __rhs.clone())
    }
}
impl CppMove for support_uniform_int_distribution_long__param_type {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for support_uniform_int_distribution_long__param_type {
    fn default() -> Self {
        { support_uniform_int_distribution_long__param_type::new_1() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct support_uniform_int_distribution_long_ {
    #[offset(0)]
    #[byte_size(16)]
    param_: support_uniform_int_distribution_long__param_type,
}
impl support_uniform_int_distribution_long_ {}
impl CppEq for support_uniform_int_distribution_long_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_72(__lhs.clone(), __rhs.clone())
    }
}
impl
    Callable2<
        Ptr<support_simple_random_generator>,
        Ptr<support_uniform_int_distribution_long__param_type>,
        i64,
    > for support_uniform_int_distribution_long_
{
    fn call(
        &self,
        a1: Ptr<support_simple_random_generator>,
        a2: Ptr<support_uniform_int_distribution_long__param_type>,
    ) -> i64 {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            support_uniform_int_distribution_long_Impl::operator_call_6_class_support_simple_random_generator(__it, a1, a2)
        })
    }
}
impl CppMove for support_uniform_int_distribution_long_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static default_seed_74: Value<u16> = Rc::new(RefCell::new(19937_u16));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct support_simple_random_generator {
    #[offset(0)]
    status_: u32,
}
impl support_simple_random_generator {
    pub fn min() -> u16 {
        return 0_u16;
    }
    pub fn max() -> u16 {
        return (-1_i32 as u16);
    }
    pub fn new_1() -> Self {
        support_simple_random_generator::new_2({ default_seed_74.with(|rc| *rc.borrow()) })
    }
    pub fn new_2(mut s: u16) -> Self {
        Self {
            status_: (s as u32),
        }
    }
}
impl Callable0<u16> for support_simple_random_generator {
    fn call(&self) -> u16 {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            support_simple_random_generatorImpl::operator_call(__it)
        })
    }
}
impl CppMove for support_simple_random_generator {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for support_simple_random_generator {
    fn default() -> Self {
        { support_simple_random_generator::new_1() }
    }
}
pub fn shuffle_75(
    mut first: Ptr<Option<Value<i32>>>,
    mut last: Ptr<Option<Value<i32>>>,
    gen_: Ptr<support_simple_random_generator>,
) {
    let mut last_iter: Ptr<Option<Value<i32>>> = (last).clone();
    let mut diff: isize = (((last_iter).clone() - (first).clone()) as i64 as isize);
    if (diff > 1_isize) {
        let uid: Value<support_uniform_int_distribution_long_> = Rc::new(RefCell::new(
            <support_uniform_int_distribution_long_>::default(),
        ));
        {
            last_iter.prefix_dec();
            &(diff.prefix_dec())
        };
        'loop_: while ({ (first).clone() } < { (last).clone() }) {
            let mut index: isize = (({
                let _gen_: Ptr<support_simple_random_generator> = (gen_).clone();
                let _param: Value<support_uniform_int_distribution_long__param_type> = Rc::new(
                    RefCell::new(support_uniform_int_distribution_long__param_type::new_2(
                        { 0_i64 },
                        { Some((diff as i64)) },
                    )),
                );
                support_uniform_int_distribution_long_Impl :: operator_call_6_class_support_simple_random_generator ( &uid .as_pointer()  , _gen_ , _param.as_pointer() , )
            }) as isize);
            if (index != 0_isize) {
                cpp_swap!((first).clone(), (first.offset((index) as isize)));
            }
            {
                first.prefix_inc();
                &(diff.prefix_dec())
            };
        }
    }
}
pub fn shuffle_76(
    mut first: Ptr<i32>,
    mut last: Ptr<i32>,
    gen_: Ptr<support_simple_random_generator>,
) {
    let mut last_iter: Ptr<i32> = (last).clone();
    let mut diff: isize = (((last_iter).clone() - (first).clone()) as i64 as isize);
    if (diff > 1_isize) {
        let uid: Value<support_uniform_int_distribution_long_> = Rc::new(RefCell::new(
            <support_uniform_int_distribution_long_>::default(),
        ));
        {
            last_iter.prefix_dec();
            &(diff.prefix_dec())
        };
        'loop_: while ({ (first).clone() } < { (last).clone() }) {
            let mut index: isize = (({
                let _gen_: Ptr<support_simple_random_generator> = (gen_).clone();
                let _param: Value<support_uniform_int_distribution_long__param_type> = Rc::new(
                    RefCell::new(support_uniform_int_distribution_long__param_type::new_2(
                        { 0_i64 },
                        { Some((diff as i64)) },
                    )),
                );
                support_uniform_int_distribution_long_Impl :: operator_call_6_class_support_simple_random_generator ( &uid .as_pointer()  , _gen_ , _param.as_pointer() , )
            }) as isize);
            if (index != 0_isize) {
                cpp_swap!((first).clone(), (first.offset((index) as isize)));
            }
            {
                first.prefix_inc();
                &(diff.prefix_dec())
            };
        }
    }
}
pub fn shuffle_77(mut first: Ptr<S>, mut last: Ptr<S>, gen_: Ptr<support_simple_random_generator>) {
    let mut last_iter: Ptr<S> = (last).clone();
    let mut diff: isize = (((last_iter).clone() - (first).clone()) as i64 as isize);
    if (diff > 1_isize) {
        let uid: Value<support_uniform_int_distribution_long_> = Rc::new(RefCell::new(
            <support_uniform_int_distribution_long_>::default(),
        ));
        {
            last_iter.prefix_dec();
            &(diff.prefix_dec())
        };
        'loop_: while ({ (first).clone() } < { (last).clone() }) {
            let mut index: isize = (({
                let _gen_: Ptr<support_simple_random_generator> = (gen_).clone();
                let _param: Value<support_uniform_int_distribution_long__param_type> = Rc::new(
                    RefCell::new(support_uniform_int_distribution_long__param_type::new_2(
                        { 0_i64 },
                        { Some((diff as i64)) },
                    )),
                );
                support_uniform_int_distribution_long_Impl :: operator_call_6_class_support_simple_random_generator ( &uid .as_pointer()  , _gen_ , _param.as_pointer() , )
            }) as isize);
            if (index != 0_isize) {
                cpp_swap!((first).clone(), (first.offset((index) as isize)));
            }
            {
                first.prefix_inc();
                &(diff.prefix_dec())
            };
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct indirect_less {}
impl Callable2<Ptr<Option<Value<i32>>>, Ptr<Option<Value<i32>>>, bool> for indirect_less {
    fn call(&self, a1: Ptr<Option<Value<i32>>>, a2: Ptr<Option<Value<i32>>>) -> bool {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            indirect_lessImpl::operator_call_1_class_std_unique_ptr_int_(__it, a1, a2)
        })
    }
}
impl CppMove for indirect_less {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, CppFrom, CopyAssign, MoveAssign)]
#[cpp_from(i32, new_1, value)]
#[byte_size(4)]
pub struct S {
    #[offset(0)]
    pub i_: i32,
}
impl S {
    pub fn new_2() -> Self {
        Self { i_: 0 }
    }
    pub fn new_1(mut i: i32) -> Self {
        Self { i_: i }
    }
    pub fn copy_from(rhs: Ptr<S>) -> Self {
        Self {
            i_: rhs.with(|__s| __s.i_),
        }
    }
    pub fn move_from(rhs: Ptr<S>) -> Self {
        let __this: S = Self {
            i_: rhs.with(|__s| __s.i_),
        };
        field!(rhs, i_).write(-1_i32);
        __this
    }
}
impl CppLess for S {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        SImpl::operator_lt(__lhs, __rhs.clone())
    }
}
impl CppEq for S {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        SImpl::operator_eq_10(__lhs, __rhs.clone())
    }
}
impl Clone for S {
    fn clone(&self) -> Self {
        let __src: Value<S> = Rc::new(RefCell::new(S {
            i_: self.i_.clone(),
        }));
        S::copy_from(__src.as_pointer())
    }
}
impl CppMove for S {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
impl Default for S {
    fn default() -> Self {
        { S::new_2() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign)]
#[byte_size(1)]
pub struct support_double_move_tracker {
    #[offset(0)]
    moved_from_: bool,
}
impl support_double_move_tracker {
    pub fn new() -> Self {
        Self { moved_from_: false }
    }
    pub fn move_from(other: Ptr<support_double_move_tracker>) -> Self {
        let __this: support_double_move_tracker = Self { moved_from_: false };
        assert!(!(other.with(|__s| __s.moved_from_)));
        field!(other, moved_from_).write(true);
        __this
    }
}
impl CppMove for support_double_move_tracker {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
impl Default for support_double_move_tracker {
    fn default() -> Self {
        { support_double_move_tracker::new() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t___types_type_list_signed_char__short__int__long__long_long____int128___types_type_list_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__
{}
impl CppMove for types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t___types_type_list_signed_char__short__int__long__long_long____int128___types_type_list_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__ { fn cpp_move(src: Ptr<Self>) -> Self { src.read() } }
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128___types_type_list_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__
{}
impl CppMove for types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128___types_type_list_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__ { fn cpp_move(src: Ptr<Self>) -> Self { src.read() } }
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128___types_type_list_bool__
{}
impl CppMove for types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128___types_type_list_bool__ { fn cpp_move(src: Ptr<Self>) -> Self { src.read() } }
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__bool___types_type_list_float__double__long_double__
{}
impl CppMove for types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__bool___types_type_list_float__double__long_double__ { fn cpp_move(src: Ptr<Self>) -> Self { src.read() } }
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct cpp17_output_iterator_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<i32>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl cpp17_output_iterator_int_ptr_ {
    pub fn move_from(_a0: Ptr<cpp17_output_iterator_int_ptr_>) -> Self {
        Self {
            it_: { (*_a0.upgrade().deref()).it_.clone() },
            tracker_: support_double_move_tracker::move_from({ field_ptr!(_a0, tracker_) }),
        }
    }
}
impl CppMove for cpp17_output_iterator_int_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
const _: () = assert!(
    true,
    "std::output_iterator<cpp17_output_iterator<int*>, int>"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct cpp17_input_iterator_int_ptr__int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<i32>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl cpp17_input_iterator_int_ptr__int_ptr_ {
    pub fn move_from(_a0: Ptr<cpp17_input_iterator_int_ptr__int_ptr_>) -> Self {
        Self {
            it_: { (*_a0.upgrade().deref()).it_.clone() },
            tracker_: support_double_move_tracker::move_from({ field_ptr!(_a0, tracker_) }),
        }
    }
}
impl CppMove for cpp17_input_iterator_int_ptr__int_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
const _: () = assert!(true, "std::input_iterator<cpp17_input_iterator<int*>>");
pub fn operator_eq_78(
    x: Ptr<bidirectional_iterator_int_ptr_>,
    y: Ptr<bidirectional_iterator_int_ptr_>,
) -> bool {
    return ({ x.with(|__s| __s.it_.clone()) } == { y.with(|__s| __s.it_.clone()) });
}
pub fn operator_ne_79(
    x: Ptr<bidirectional_iterator_int_ptr_>,
    y: Ptr<bidirectional_iterator_int_ptr_>,
) -> bool {
    return ({ x.with(|__s| __s.it_.clone()) } != { y.with(|__s| __s.it_.clone()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct bidirectional_iterator_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<i32>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl bidirectional_iterator_int_ptr_ {
    pub fn new_1(mut it: Ptr<i32>) -> Self {
        Self {
            it_: (it).clone(),
            tracker_: support_double_move_tracker::new(),
        }
    }
    pub fn move_from(_a0: Ptr<bidirectional_iterator_int_ptr_>) -> Self {
        Self {
            it_: { (*_a0.upgrade().deref()).it_.clone() },
            tracker_: support_double_move_tracker::move_from({ field_ptr!(_a0, tracker_) }),
        }
    }
}
impl std::cmp::PartialEq for bidirectional_iterator_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        {
            libcc2rs::with_record_ptrs(self, other, |__lhs, __rhs| {
                operator_eq_78(__lhs.clone(), __rhs.clone())
            })
        }
    }
}
impl std::cmp::Eq for bidirectional_iterator_int_ptr_ {}
impl CppAdvance for bidirectional_iterator_int_ptr_ {
    fn cpp_inc(__it: &Ptr<Self>) {
        __it.operator_inc_6();
    }
    fn cpp_dec(__it: &Ptr<Self>) {
        __it.operator_dec_7();
    }
}
impl CppIterator<i32> for bidirectional_iterator_int_ptr_ {
    fn iter_deref(&self) -> Ptr<i32> {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            bidirectional_iterator_int_ptr_Impl::operator_deref(__it)
        })
    }
    fn iter_inc(&mut self) -> &mut Self {
        libcc2rs::with_record_ptr(self, |__it| {
            bidirectional_iterator_int_ptr_Impl::operator_inc_6(__it);
        });
        self
    }
}
impl CppInputIterator<i32> for bidirectional_iterator_int_ptr_ {}
impl CppBidirectionalIterator<i32> for bidirectional_iterator_int_ptr_ {
    fn iter_dec(&mut self) -> &mut Self {
        libcc2rs::with_record_ptr(self, |__it| {
            bidirectional_iterator_int_ptr_Impl::operator_dec_7(__it);
        });
        self
    }
}
impl CppMove for bidirectional_iterator_int_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
pub fn operator_eq_80(
    x: Ptr<bidirectional_iterator_S_ptr_>,
    y: Ptr<bidirectional_iterator_S_ptr_>,
) -> bool {
    return ({ x.with(|__s| __s.it_.clone()) } == { y.with(|__s| __s.it_.clone()) });
}
pub fn operator_ne_81(
    x: Ptr<bidirectional_iterator_S_ptr_>,
    y: Ptr<bidirectional_iterator_S_ptr_>,
) -> bool {
    return ({ x.with(|__s| __s.it_.clone()) } != { y.with(|__s| __s.it_.clone()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct bidirectional_iterator_S_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<S>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl bidirectional_iterator_S_ptr_ {
    pub fn new_1(mut it: Ptr<S>) -> Self {
        Self {
            it_: (it).clone(),
            tracker_: support_double_move_tracker::new(),
        }
    }
    pub fn move_from(_a0: Ptr<bidirectional_iterator_S_ptr_>) -> Self {
        Self {
            it_: { (*_a0.upgrade().deref()).it_.clone() },
            tracker_: support_double_move_tracker::move_from({ field_ptr!(_a0, tracker_) }),
        }
    }
}
impl std::cmp::PartialEq for bidirectional_iterator_S_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        {
            libcc2rs::with_record_ptrs(self, other, |__lhs, __rhs| {
                operator_eq_80(__lhs.clone(), __rhs.clone())
            })
        }
    }
}
impl std::cmp::Eq for bidirectional_iterator_S_ptr_ {}
impl CppAdvance for bidirectional_iterator_S_ptr_ {
    fn cpp_inc(__it: &Ptr<Self>) {
        __it.operator_inc_6();
    }
    fn cpp_dec(__it: &Ptr<Self>) {
        __it.operator_dec_7();
    }
}
impl CppIterator<S> for bidirectional_iterator_S_ptr_ {
    fn iter_deref(&self) -> Ptr<S> {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            bidirectional_iterator_S_ptr_Impl::operator_deref(__it)
        })
    }
    fn iter_inc(&mut self) -> &mut Self {
        libcc2rs::with_record_ptr(self, |__it| {
            bidirectional_iterator_S_ptr_Impl::operator_inc_6(__it);
        });
        self
    }
}
impl CppInputIterator<S> for bidirectional_iterator_S_ptr_ {}
impl CppBidirectionalIterator<S> for bidirectional_iterator_S_ptr_ {
    fn iter_dec(&mut self) -> &mut Self {
        libcc2rs::with_record_ptr(self, |__it| {
            bidirectional_iterator_S_ptr_Impl::operator_dec_7(__it);
        });
        self
    }
}
impl CppMove for bidirectional_iterator_S_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
pub fn operator_add_82(
    x: random_access_iterator_int_ptr_,
    mut n: isize,
) -> random_access_iterator_int_ptr_ {
    let x: Value<random_access_iterator_int_ptr_> = Rc::new(RefCell::new(x));
    ({ random_access_iterator_int_ptr_Impl::operator_add_assign(&x.as_pointer(), n) });
    return random_access_iterator_int_ptr_::move_from({ x.as_pointer() });
}
pub fn operator_sub_83(
    x: random_access_iterator_int_ptr_,
    mut n: isize,
) -> random_access_iterator_int_ptr_ {
    let x: Value<random_access_iterator_int_ptr_> = Rc::new(RefCell::new(x));
    ({ random_access_iterator_int_ptr_Impl::operator_sub_assign(&x.as_pointer(), n) });
    return random_access_iterator_int_ptr_::move_from({ x.as_pointer() });
}
pub fn operator_sub_84(
    mut x: random_access_iterator_int_ptr_,
    mut y: random_access_iterator_int_ptr_,
) -> isize {
    return (((x.it_).clone() - (y.it_).clone()) as i64 as isize);
}
pub fn operator_eq_85(
    x: Ptr<random_access_iterator_int_ptr_>,
    y: Ptr<random_access_iterator_int_ptr_>,
) -> bool {
    return ({ x.with(|__s| __s.it_.clone()) } == { y.with(|__s| __s.it_.clone()) });
}
pub fn operator_ne_86(
    x: Ptr<random_access_iterator_int_ptr_>,
    y: Ptr<random_access_iterator_int_ptr_>,
) -> bool {
    return ({ x.with(|__s| __s.it_.clone()) } != { y.with(|__s| __s.it_.clone()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct random_access_iterator_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<i32>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl random_access_iterator_int_ptr_ {
    pub fn new_1(mut it: Ptr<i32>) -> Self {
        Self {
            it_: (it).clone(),
            tracker_: support_double_move_tracker::new(),
        }
    }
    pub fn move_from(_a0: Ptr<random_access_iterator_int_ptr_>) -> Self {
        Self {
            it_: { (*_a0.upgrade().deref()).it_.clone() },
            tracker_: support_double_move_tracker::move_from({ field_ptr!(_a0, tracker_) }),
        }
    }
}
impl std::cmp::PartialEq for random_access_iterator_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        {
            libcc2rs::with_record_ptrs(self, other, |__lhs, __rhs| {
                operator_eq_85(__lhs.clone(), __rhs.clone())
            })
        }
    }
}
impl std::cmp::Eq for random_access_iterator_int_ptr_ {}
impl CppAdvance for random_access_iterator_int_ptr_ {
    fn cpp_inc(__it: &Ptr<Self>) {
        __it.operator_inc_7();
    }
    fn cpp_dec(__it: &Ptr<Self>) {
        __it.operator_dec_8();
    }
}
impl CppIterator<i32> for random_access_iterator_int_ptr_ {
    fn iter_deref(&self) -> Ptr<i32> {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            random_access_iterator_int_ptr_Impl::operator_deref(__it)
        })
    }
    fn iter_inc(&mut self) -> &mut Self {
        libcc2rs::with_record_ptr(self, |__it| {
            random_access_iterator_int_ptr_Impl::operator_inc_7(__it);
        });
        self
    }
}
impl CppInputIterator<i32> for random_access_iterator_int_ptr_ {}
impl CppBidirectionalIterator<i32> for random_access_iterator_int_ptr_ {
    fn iter_dec(&mut self) -> &mut Self {
        libcc2rs::with_record_ptr(self, |__it| {
            random_access_iterator_int_ptr_Impl::operator_dec_8(__it);
        });
        self
    }
}
impl CppMove for random_access_iterator_int_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
pub fn operator_add_87(
    x: random_access_iterator_S_ptr_,
    mut n: isize,
) -> random_access_iterator_S_ptr_ {
    let x: Value<random_access_iterator_S_ptr_> = Rc::new(RefCell::new(x));
    ({ random_access_iterator_S_ptr_Impl::operator_add_assign(&x.as_pointer(), n) });
    return random_access_iterator_S_ptr_::move_from({ x.as_pointer() });
}
pub fn operator_sub_88(
    x: random_access_iterator_S_ptr_,
    mut n: isize,
) -> random_access_iterator_S_ptr_ {
    let x: Value<random_access_iterator_S_ptr_> = Rc::new(RefCell::new(x));
    ({ random_access_iterator_S_ptr_Impl::operator_sub_assign(&x.as_pointer(), n) });
    return random_access_iterator_S_ptr_::move_from({ x.as_pointer() });
}
pub fn operator_sub_89(
    mut x: random_access_iterator_S_ptr_,
    mut y: random_access_iterator_S_ptr_,
) -> isize {
    return (((x.it_).clone() - (y.it_).clone()) as i64 as isize);
}
pub fn operator_eq_90(
    x: Ptr<random_access_iterator_S_ptr_>,
    y: Ptr<random_access_iterator_S_ptr_>,
) -> bool {
    return ({ x.with(|__s| __s.it_.clone()) } == { y.with(|__s| __s.it_.clone()) });
}
pub fn operator_ne_91(
    x: Ptr<random_access_iterator_S_ptr_>,
    y: Ptr<random_access_iterator_S_ptr_>,
) -> bool {
    return ({ x.with(|__s| __s.it_.clone()) } != { y.with(|__s| __s.it_.clone()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct random_access_iterator_S_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<S>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl random_access_iterator_S_ptr_ {
    pub fn new_1(mut it: Ptr<S>) -> Self {
        Self {
            it_: (it).clone(),
            tracker_: support_double_move_tracker::new(),
        }
    }
    pub fn move_from(_a0: Ptr<random_access_iterator_S_ptr_>) -> Self {
        Self {
            it_: { (*_a0.upgrade().deref()).it_.clone() },
            tracker_: support_double_move_tracker::move_from({ field_ptr!(_a0, tracker_) }),
        }
    }
}
impl std::cmp::PartialEq for random_access_iterator_S_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        {
            libcc2rs::with_record_ptrs(self, other, |__lhs, __rhs| {
                operator_eq_90(__lhs.clone(), __rhs.clone())
            })
        }
    }
}
impl std::cmp::Eq for random_access_iterator_S_ptr_ {}
impl CppAdvance for random_access_iterator_S_ptr_ {
    fn cpp_inc(__it: &Ptr<Self>) {
        __it.operator_inc_7();
    }
    fn cpp_dec(__it: &Ptr<Self>) {
        __it.operator_dec_8();
    }
}
impl CppIterator<S> for random_access_iterator_S_ptr_ {
    fn iter_deref(&self) -> Ptr<S> {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            random_access_iterator_S_ptr_Impl::operator_deref(__it)
        })
    }
    fn iter_inc(&mut self) -> &mut Self {
        libcc2rs::with_record_ptr(self, |__it| {
            random_access_iterator_S_ptr_Impl::operator_inc_7(__it);
        });
        self
    }
}
impl CppInputIterator<S> for random_access_iterator_S_ptr_ {}
impl CppBidirectionalIterator<S> for random_access_iterator_S_ptr_ {
    fn iter_dec(&mut self) -> &mut Self {
        libcc2rs::with_record_ptr(self, |__it| {
            random_access_iterator_S_ptr_Impl::operator_dec_8(__it);
        });
        self
    }
}
impl CppMove for random_access_iterator_S_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct cpp20_random_access_iterator_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<i32>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl cpp20_random_access_iterator_int_ptr_ {
    pub fn move_from(_a0: Ptr<cpp20_random_access_iterator_int_ptr_>) -> Self {
        Self {
            it_: { (*_a0.upgrade().deref()).it_.clone() },
            tracker_: support_double_move_tracker::move_from({ field_ptr!(_a0, tracker_) }),
        }
    }
}
impl CppMove for cpp20_random_access_iterator_int_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
const _: () = assert!(
    true,
    "std::random_access_iterator<cpp20_random_access_iterator<int*>>"
);
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct cpp20_input_iterator_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<i32>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl cpp20_input_iterator_int_ptr_ {
    pub fn move_from(_a0: Ptr<cpp20_input_iterator_int_ptr_>) -> Self {
        Self {
            it_: { (*_a0.upgrade().deref()).it_.clone() },
            tracker_: support_double_move_tracker::move_from({ field_ptr!(_a0, tracker_) }),
        }
    }
}
impl CppMove for cpp20_input_iterator_int_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
const _: () = assert!(true, "std::input_iterator<cpp20_input_iterator<int*>>");
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct cpp20_output_iterator_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<i32>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl cpp20_output_iterator_int_ptr_ {
    pub fn move_from(_a0: Ptr<cpp20_output_iterator_int_ptr_>) -> Self {
        Self {
            it_: { (*_a0.upgrade().deref()).it_.clone() },
            tracker_: support_double_move_tracker::move_from({ field_ptr!(_a0, tracker_) }),
        }
    }
}
impl CppMove for cpp20_output_iterator_int_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
const _: () = assert!(
    true,
    "std::output_iterator<cpp20_output_iterator<int*>, int>"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(32)]
pub struct IteratorOpCounts {
    #[offset(0)]
    pub increments: usize,
    #[offset(8)]
    pub decrements: usize,
    #[offset(16)]
    pub zero_moves: usize,
    #[offset(24)]
    pub equal_cmps: usize,
}
impl CppMove for IteratorOpCounts {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for IteratorOpCounts {
    fn default() -> Self {
        {
            IteratorOpCounts {
                increments: 0_usize,
                decrements: 0_usize,
                zero_moves: 0_usize,
                equal_cmps: 0_usize,
            }
        }
    }
}
pub fn operator_eq_92(
    _a0: Ptr<forward_sized_iterator_int_ptr_>,
    _a1: Ptr<forward_sized_iterator_int_ptr_>,
) -> bool {
    return ({ _a0.with(|__s| __s.it_.clone()) } == { _a1.with(|__s| __s.it_.clone()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct forward_sized_iterator_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    pub it_: Ptr<i32>,
}
impl forward_sized_iterator_int_ptr_ {}
impl CppEq for forward_sized_iterator_int_ptr_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_92(__lhs.clone(), __rhs.clone())
    }
}
impl CppMove for forward_sized_iterator_int_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for forward_sized_iterator_int_ptr_ {
    fn default() -> Self {
        {
            forward_sized_iterator_int_ptr_ {
                it_: Ptr::<i32>::null(),
            }
        }
    }
}
const _: () = assert!(true, "std::forward_iterator<forward_sized_iterator<>>");
const _: () = assert!(
    true,
    "std::sized_sentinel_for<forward_sized_iterator<>, forward_sized_iterator<>>"
);
pub fn operator_add_93(mut i: adl_Iterator, mut n: isize) -> adl_Iterator {
    return adl_Iterator::new_1(
        { i.ptr_.offset((n) as isize) },
        { (i.iter_moves_).clone() },
        { (i.iter_swaps_).clone() },
    );
}
pub fn operator_add_94(mut n: isize, mut i: adl_Iterator) -> adl_Iterator {
    return ({
        let _i: adl_Iterator = i;
        operator_add_93(_i, n)
    });
}
pub fn iter_swap_95(mut a: adl_Iterator, mut b: adl_Iterator) {
    cpp_swap!((a.ptr_).clone(), (b.ptr_).clone());
    if !(a.iter_swaps_).is_null() {
        a.iter_swaps_.with_mut(|__v| {
            __v.prefix_inc();
        });
    }
}
pub fn iter_move_96(iter: adl_Iterator) -> Ptr<i32> {
    let iter: Value<adl_Iterator> = Rc::new(RefCell::new(iter));
    if !({ (*iter.borrow()).iter_moves_.clone() }).is_null() {
        { (*iter.borrow()).iter_moves_.clone() }.with_mut(|__v| {
            __v.prefix_inc();
        });
    }
    return ({ adl_IteratorImpl::operator_deref(&iter.as_pointer()) });
}
pub fn operator_eq_97(lhs: Ptr<adl_Iterator>, rhs: Ptr<adl_Iterator>) -> bool {
    return ({ lhs.with(|__s| __s.ptr_.clone()) } == { rhs.with(|__s| __s.ptr_.clone()) });
}
pub fn operator_cmp_98(lhs: Ptr<adl_Iterator>, rhs: Ptr<adl_Iterator>) -> std::cmp::Ordering {
    return std::cmp::Ord::cmp(
        &(lhs.with(|__s| __s.ptr_.clone())),
        &(rhs.with(|__s| __s.ptr_.clone())),
    );
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(24)]
pub struct adl_Iterator {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<i32>,
    #[offset(8)]
    #[byte_size(8)]
    iter_moves_: Ptr<i32>,
    #[offset(16)]
    #[byte_size(8)]
    iter_swaps_: Ptr<i32>,
}
impl adl_Iterator {
    fn new_1(mut p: Ptr<i32>, mut iter_moves: Ptr<i32>, mut iter_swaps: Ptr<i32>) -> Self {
        Self {
            ptr_: (p).clone(),
            iter_moves_: (iter_moves).clone(),
            iter_swaps_: (iter_swaps).clone(),
        }
    }
    pub fn TrackMoves(mut p: Ptr<i32>, iter_moves: Ptr<i32>) -> adl_Iterator {
        return adl_Iterator::new_1({ (p).clone() }, { (iter_moves).clone() }, {
            Ptr::<i32>::null()
        });
    }
    pub fn TrackSwaps_4(iter_swaps: Ptr<i32>) -> adl_Iterator {
        return adl_Iterator::new_1({ Ptr::<i32>::null() }, { Ptr::<i32>::null() }, {
            (iter_swaps).clone()
        });
    }
    pub fn TrackSwaps_5(mut p: Ptr<i32>, iter_swaps: Ptr<i32>) -> adl_Iterator {
        return adl_Iterator::new_1({ (p).clone() }, { Ptr::<i32>::null() }, {
            (iter_swaps).clone()
        });
    }
}
impl std::cmp::Ord for adl_Iterator {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            libcc2rs::with_record_ptrs(self, other, |__lhs, __rhs| {
                operator_cmp_98(__lhs.clone(), __rhs.clone())
            })
        }
    }
}
impl std::cmp::PartialOrd for adl_Iterator {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for adl_Iterator {
    fn eq(&self, other: &Self) -> bool {
        {
            libcc2rs::with_record_ptrs(self, other, |__lhs, __rhs| {
                operator_eq_97(__lhs.clone(), __rhs.clone())
            })
        }
    }
}
impl std::cmp::Eq for adl_Iterator {}
impl CppAdvance for adl_Iterator {
    fn cpp_inc(__it: &Ptr<Self>) {
        __it.operator_inc_14();
    }
    fn cpp_dec(__it: &Ptr<Self>) {
        __it.operator_dec_16();
    }
}
impl CppIterator<i32> for adl_Iterator {
    fn iter_deref(&self) -> Ptr<i32> {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            adl_IteratorImpl::operator_deref(__it)
        })
    }
    fn iter_inc(&mut self) -> &mut Self {
        libcc2rs::with_record_ptr(self, |__it| {
            adl_IteratorImpl::operator_inc_14(__it);
        });
        self
    }
}
impl CppInputIterator<i32> for adl_Iterator {}
impl CppBidirectionalIterator<i32> for adl_Iterator {
    fn iter_dec(&mut self) -> &mut Self {
        libcc2rs::with_record_ptr(self, |__it| {
            adl_IteratorImpl::operator_dec_16(__it);
        });
        self
    }
}
impl CppRandomAccessIterator<i32> for adl_Iterator {
    fn iter_add_assign(&mut self, n: isize) -> Self {
        libcc2rs::with_record_ptr(self, |__it| {
            adl_IteratorImpl::operator_add_assign(__it, n as _);
        });
        self.clone()
    }
    fn iter_distance(&self, from: &Self) -> isize {
        libcc2rs::with_record_ptrs(self, from, |__lhs, __rhs| {
            (adl_IteratorImpl::operator_sub_11(__lhs, __rhs.read())) as isize
        })
    }
}
impl CppMove for adl_Iterator {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for adl_Iterator {
    fn default() -> Self {
        {
            adl_Iterator {
                ptr_: Ptr::<i32>::null(),
                iter_moves_: Ptr::<i32>::null(),
                iter_swaps_: Ptr::<i32>::null(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct rvalue_iterator_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<Ptr<i32>>,
}
impl rvalue_iterator_int_ptr_ {}
impl CppLess for rvalue_iterator_int_ptr_ {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        rvalue_iterator_int_ptr_Impl::operator_cmp(__lhs, __rhs.clone()) == std::cmp::Ordering::Less
    }
}
impl CppEq for rvalue_iterator_int_ptr_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        rvalue_iterator_int_ptr_Impl::operator_cmp(__lhs, __rhs.clone())
            == std::cmp::Ordering::Equal
    }
}
impl CppMove for rvalue_iterator_int_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
const _: () = assert!(true, "std::random_access_iterator<rvalue_iterator<int*>>");
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(Ptr< i8  >  , new, value)]
#[byte_size(16)]
pub struct util_iterator_error {}
impl util_iterator_error {
    pub fn new(mut what: Ptr<i8>) -> Self {
        Self {}
    }
}
impl CppMove for util_iterator_error {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn test_one_99(mut N: u32, mut M: u32, randomness: Ptr<support_simple_random_generator>) {
    assert!((M <= N));
    let mut ia: Ptr<i32> =
        Ptr::alloc_array((0..(N as usize)).map(|_| 0_i32).collect::<Box<[i32]>>());
    let mut i: u32 = 0_u32;
    'loop_: while (i < N) {
        elem!(ia, i).write({ (i as i32) });
        i.prefix_inc();
    }
    ({
        let _first: Ptr<i32> = (ia).clone();
        let _last: Ptr<i32> = ia.offset((N) as isize);
        let _gen_: Ptr<support_simple_random_generator> = (randomness).clone();
        shuffle_76(_first, _last, _gen_)
    });
    ({
        let ___first: Ptr<i32> = (ia).clone();
        let ___last: Ptr<i32> = ia.offset((M) as isize);
        libcc2rs::sort_refcount(___first, ___last, <std_greater_int_>::default())
    });
    ({
        let ___first: Ptr<i32> = ia.offset((M) as isize);
        let ___last: Ptr<i32> = ia.offset((N) as isize);
        libcc2rs::sort_refcount(___first, ___last, <std_greater_int_>::default())
    });
    let pred: Value<binary_counting_predicate_std_greater_int___int__int_> = Rc::new(RefCell::new(
        binary_counting_predicate_std_greater_int___int__int_::new({
            (<std_greater_int_>::default())
        }),
    ));
    ({
        let ___first: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ (ia).clone() });
        let ___middle: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ ia.offset((M) as isize) });
        let ___last: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ ia.offset((N) as isize) });
        inplace_merge_100(
            ___first,
            ___middle,
            ___last,
            ({ ref_101(pred.as_pointer()) }),
        )
    });
    if (N > 0_u32) {
        assert!(({ (elem!(ia, 0).read()) } == { ((N as i32) - 1) }));
        assert!(((elem!(ia, (N).wrapping_sub(1_u32)).read()) == 0));
        assert!(
            ({
                let ___first: Ptr<i32> = (ia).clone();
                let ___last: Ptr<i32> = ia.offset((N) as isize);
                is_sorted_102(___first, ___last, <std_greater_int_>::default())
            })
        );
    }
    ia.delete();
}
pub fn test_one_103(mut N: u32, mut M: u32, randomness: Ptr<support_simple_random_generator>) {
    assert!((M <= N));
    let mut ia: Ptr<i32> =
        Ptr::alloc_array((0..(N as usize)).map(|_| 0_i32).collect::<Box<[i32]>>());
    let mut i: u32 = 0_u32;
    'loop_: while (i < N) {
        elem!(ia, i).write({ (i as i32) });
        i.prefix_inc();
    }
    ({
        let _first: Ptr<i32> = (ia).clone();
        let _last: Ptr<i32> = ia.offset((N) as isize);
        let _gen_: Ptr<support_simple_random_generator> = (randomness).clone();
        shuffle_76(_first, _last, _gen_)
    });
    ({
        let ___first: Ptr<i32> = (ia).clone();
        let ___last: Ptr<i32> = ia.offset((M) as isize);
        libcc2rs::sort_refcount(___first, ___last, <std_greater_int_>::default())
    });
    ({
        let ___first: Ptr<i32> = ia.offset((M) as isize);
        let ___last: Ptr<i32> = ia.offset((N) as isize);
        libcc2rs::sort_refcount(___first, ___last, <std_greater_int_>::default())
    });
    let pred: Value<binary_counting_predicate_std_greater_int___int__int_> = Rc::new(RefCell::new(
        binary_counting_predicate_std_greater_int___int__int_::new({
            (<std_greater_int_>::default())
        }),
    ));
    ({
        let ___first: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ (ia).clone() });
        let ___middle: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ ia.offset((M) as isize) });
        let ___last: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ ia.offset((N) as isize) });
        inplace_merge_104(
            ___first,
            ___middle,
            ___last,
            ({ ref_101(pred.as_pointer()) }),
        )
    });
    if (N > 0_u32) {
        assert!(({ (elem!(ia, 0).read()) } == { ((N as i32) - 1) }));
        assert!(((elem!(ia, (N).wrapping_sub(1_u32)).read()) == 0));
        assert!(
            ({
                let ___first: Ptr<i32> = (ia).clone();
                let ___last: Ptr<i32> = ia.offset((N) as isize);
                is_sorted_102(___first, ___last, <std_greater_int_>::default())
            })
        );
    }
    ia.delete();
}
pub fn test_one_105(mut N: u32, mut M: u32, randomness: Ptr<support_simple_random_generator>) {
    assert!((M <= N));
    let mut ia: Ptr<i32> =
        Ptr::alloc_array((0..(N as usize)).map(|_| 0_i32).collect::<Box<[i32]>>());
    let mut i: u32 = 0_u32;
    'loop_: while (i < N) {
        elem!(ia, i).write({ (i as i32) });
        i.prefix_inc();
    }
    ({
        let _first: Ptr<i32> = (ia).clone();
        let _last: Ptr<i32> = ia.offset((N) as isize);
        let _gen_: Ptr<support_simple_random_generator> = (randomness).clone();
        shuffle_76(_first, _last, _gen_)
    });
    ({
        let ___first: Ptr<i32> = (ia).clone();
        let ___last: Ptr<i32> = ia.offset((M) as isize);
        libcc2rs::sort_refcount(___first, ___last, <std_greater_int_>::default())
    });
    ({
        let ___first: Ptr<i32> = ia.offset((M) as isize);
        let ___last: Ptr<i32> = ia.offset((N) as isize);
        libcc2rs::sort_refcount(___first, ___last, <std_greater_int_>::default())
    });
    let pred: Value<binary_counting_predicate_std_greater_int___int__int_> = Rc::new(RefCell::new(
        binary_counting_predicate_std_greater_int___int__int_::new({
            (<std_greater_int_>::default())
        }),
    ));
    ({
        let ___first: Ptr<i32> = (ia).clone();
        let ___middle: Ptr<i32> = ia.offset((M) as isize);
        let ___last: Ptr<i32> = ia.offset((N) as isize);
        inplace_merge_106(
            ___first,
            ___middle,
            ___last,
            ({ ref_101(pred.as_pointer()) }),
        )
    });
    if (N > 0_u32) {
        assert!(({ (elem!(ia, 0).read()) } == { ((N as i32) - 1) }));
        assert!(((elem!(ia, (N).wrapping_sub(1_u32)).read()) == 0));
        assert!(
            ({
                let ___first: Ptr<i32> = (ia).clone();
                let ___last: Ptr<i32> = ia.offset((N) as isize);
                is_sorted_102(___first, ___last, <std_greater_int_>::default())
            })
        );
    }
    ia.delete();
}
pub fn test_one_107(mut N: u32, mut M: u32, randomness: Ptr<support_simple_random_generator>) {
    assert!((M <= N));
    let mut ia: Ptr<S> = Ptr::alloc_array(
        (0..(N as usize))
            .map(|_| <S>::default())
            .collect::<Box<[S]>>(),
    );
    let mut i: u32 = 0_u32;
    'loop_: while (i < N) {
        ({
            let _i: i32 = (i as i32);
            SImpl::operator_assign_7(&ia.offset((i) as isize), _i)
        });
        i.prefix_inc();
    }
    ({
        let _first: Ptr<S> = (ia).clone();
        let _last: Ptr<S> = ia.offset((N) as isize);
        let _gen_: Ptr<support_simple_random_generator> = (randomness).clone();
        shuffle_77(_first, _last, _gen_)
    });
    ({
        let ___first: Ptr<S> = (ia).clone();
        let ___last: Ptr<S> = ia.offset((M) as isize);
        libcc2rs::sort_refcount(___first, ___last, <std_greater_S_>::default())
    });
    ({
        let ___first: Ptr<S> = ia.offset((M) as isize);
        let ___last: Ptr<S> = ia.offset((N) as isize);
        libcc2rs::sort_refcount(___first, ___last, <std_greater_S_>::default())
    });
    let pred: Value<binary_counting_predicate_std_greater_S___S__S_> = Rc::new(RefCell::new(
        binary_counting_predicate_std_greater_S___S__S_::new({ (<std_greater_S_>::default()) }),
    ));
    ({
        let ___first: bidirectional_iterator_S_ptr_ =
            bidirectional_iterator_S_ptr_::new_1({ (ia).clone() });
        let ___middle: bidirectional_iterator_S_ptr_ =
            bidirectional_iterator_S_ptr_::new_1({ ia.offset((M) as isize) });
        let ___last: bidirectional_iterator_S_ptr_ =
            bidirectional_iterator_S_ptr_::new_1({ ia.offset((N) as isize) });
        inplace_merge_108(
            ___first,
            ___middle,
            ___last,
            ({ ref_109(pred.as_pointer()) }),
        )
    });
    if (N > 0_u32) {
        assert!(
            ({
                let _i: i32 = ((N as i32) - 1);
                SImpl::operator_eq_11(&ia.offset((0) as isize), _i)
            })
        );
        assert!(({ SImpl::operator_eq_11(&ia.offset(((N).wrapping_sub(1_u32)) as isize), 0,) }));
        assert!(
            ({
                let ___first: Ptr<S> = (ia).clone();
                let ___last: Ptr<S> = ia.offset((N) as isize);
                is_sorted_110(___first, ___last, <std_greater_S_>::default())
            })
        );
    }
    ia.delete();
}
pub fn test_one_111(mut N: u32, mut M: u32, randomness: Ptr<support_simple_random_generator>) {
    assert!((M <= N));
    let mut ia: Ptr<S> = Ptr::alloc_array(
        (0..(N as usize))
            .map(|_| <S>::default())
            .collect::<Box<[S]>>(),
    );
    let mut i: u32 = 0_u32;
    'loop_: while (i < N) {
        ({
            let _i: i32 = (i as i32);
            SImpl::operator_assign_7(&ia.offset((i) as isize), _i)
        });
        i.prefix_inc();
    }
    ({
        let _first: Ptr<S> = (ia).clone();
        let _last: Ptr<S> = ia.offset((N) as isize);
        let _gen_: Ptr<support_simple_random_generator> = (randomness).clone();
        shuffle_77(_first, _last, _gen_)
    });
    ({
        let ___first: Ptr<S> = (ia).clone();
        let ___last: Ptr<S> = ia.offset((M) as isize);
        libcc2rs::sort_refcount(___first, ___last, <std_greater_S_>::default())
    });
    ({
        let ___first: Ptr<S> = ia.offset((M) as isize);
        let ___last: Ptr<S> = ia.offset((N) as isize);
        libcc2rs::sort_refcount(___first, ___last, <std_greater_S_>::default())
    });
    let pred: Value<binary_counting_predicate_std_greater_S___S__S_> = Rc::new(RefCell::new(
        binary_counting_predicate_std_greater_S___S__S_::new({ (<std_greater_S_>::default()) }),
    ));
    ({
        let ___first: random_access_iterator_S_ptr_ =
            random_access_iterator_S_ptr_::new_1({ (ia).clone() });
        let ___middle: random_access_iterator_S_ptr_ =
            random_access_iterator_S_ptr_::new_1({ ia.offset((M) as isize) });
        let ___last: random_access_iterator_S_ptr_ =
            random_access_iterator_S_ptr_::new_1({ ia.offset((N) as isize) });
        inplace_merge_112(
            ___first,
            ___middle,
            ___last,
            ({ ref_109(pred.as_pointer()) }),
        )
    });
    if (N > 0_u32) {
        assert!(
            ({
                let _i: i32 = ((N as i32) - 1);
                SImpl::operator_eq_11(&ia.offset((0) as isize), _i)
            })
        );
        assert!(({ SImpl::operator_eq_11(&ia.offset(((N).wrapping_sub(1_u32)) as isize), 0,) }));
        assert!(
            ({
                let ___first: Ptr<S> = (ia).clone();
                let ___last: Ptr<S> = ia.offset((N) as isize);
                is_sorted_110(___first, ___last, <std_greater_S_>::default())
            })
        );
    }
    ia.delete();
}
pub fn test_one_113(mut N: u32, mut M: u32, randomness: Ptr<support_simple_random_generator>) {
    assert!((M <= N));
    let mut ia: Ptr<S> = Ptr::alloc_array(
        (0..(N as usize))
            .map(|_| <S>::default())
            .collect::<Box<[S]>>(),
    );
    let mut i: u32 = 0_u32;
    'loop_: while (i < N) {
        ({
            let _i: i32 = (i as i32);
            SImpl::operator_assign_7(&ia.offset((i) as isize), _i)
        });
        i.prefix_inc();
    }
    ({
        let _first: Ptr<S> = (ia).clone();
        let _last: Ptr<S> = ia.offset((N) as isize);
        let _gen_: Ptr<support_simple_random_generator> = (randomness).clone();
        shuffle_77(_first, _last, _gen_)
    });
    ({
        let ___first: Ptr<S> = (ia).clone();
        let ___last: Ptr<S> = ia.offset((M) as isize);
        libcc2rs::sort_refcount(___first, ___last, <std_greater_S_>::default())
    });
    ({
        let ___first: Ptr<S> = ia.offset((M) as isize);
        let ___last: Ptr<S> = ia.offset((N) as isize);
        libcc2rs::sort_refcount(___first, ___last, <std_greater_S_>::default())
    });
    let pred: Value<binary_counting_predicate_std_greater_S___S__S_> = Rc::new(RefCell::new(
        binary_counting_predicate_std_greater_S___S__S_::new({ (<std_greater_S_>::default()) }),
    ));
    ({
        let ___first: Ptr<S> = (ia).clone();
        let ___middle: Ptr<S> = ia.offset((M) as isize);
        let ___last: Ptr<S> = ia.offset((N) as isize);
        inplace_merge_114(
            ___first,
            ___middle,
            ___last,
            ({ ref_109(pred.as_pointer()) }),
        )
    });
    if (N > 0_u32) {
        assert!(
            ({
                let _i: i32 = ((N as i32) - 1);
                SImpl::operator_eq_11(&ia.offset((0) as isize), _i)
            })
        );
        assert!(({ SImpl::operator_eq_11(&ia.offset(((N).wrapping_sub(1_u32)) as isize), 0,) }));
        assert!(
            ({
                let ___first: Ptr<S> = (ia).clone();
                let ___last: Ptr<S> = ia.offset((N) as isize);
                is_sorted_110(___first, ___last, <std_greater_S_>::default())
            })
        );
    }
    ia.delete();
}
pub fn test_115(mut N: u32, randomness: Ptr<support_simple_random_generator>) {
    ({
        let _N: u32 = N;
        let _randomness: Ptr<support_simple_random_generator> = (randomness).clone();
        test_one_99(_N, 0_u32, _randomness)
    });
    ({
        let _N: u32 = N;
        let _M: u32 = (N).wrapping_div(4_u32);
        let _randomness: Ptr<support_simple_random_generator> = (randomness).clone();
        test_one_99(_N, _M, _randomness)
    });
    ({
        let _N: u32 = N;
        let _M: u32 = (N).wrapping_div(2_u32);
        let _randomness: Ptr<support_simple_random_generator> = (randomness).clone();
        test_one_99(_N, _M, _randomness)
    });
    ({
        let _N: u32 = N;
        let _M: u32 = ((3_u32).wrapping_mul(N)).wrapping_div(4_u32);
        let _randomness: Ptr<support_simple_random_generator> = (randomness).clone();
        test_one_99(_N, _M, _randomness)
    });
    ({
        let _N: u32 = N;
        let _M: u32 = N;
        let _randomness: Ptr<support_simple_random_generator> = (randomness).clone();
        test_one_99(_N, _M, _randomness)
    });
}
pub fn test_116(mut N: u32, randomness: Ptr<support_simple_random_generator>) {
    ({
        let _N: u32 = N;
        let _randomness: Ptr<support_simple_random_generator> = (randomness).clone();
        test_one_103(_N, 0_u32, _randomness)
    });
    ({
        let _N: u32 = N;
        let _M: u32 = (N).wrapping_div(4_u32);
        let _randomness: Ptr<support_simple_random_generator> = (randomness).clone();
        test_one_103(_N, _M, _randomness)
    });
    ({
        let _N: u32 = N;
        let _M: u32 = (N).wrapping_div(2_u32);
        let _randomness: Ptr<support_simple_random_generator> = (randomness).clone();
        test_one_103(_N, _M, _randomness)
    });
    ({
        let _N: u32 = N;
        let _M: u32 = ((3_u32).wrapping_mul(N)).wrapping_div(4_u32);
        let _randomness: Ptr<support_simple_random_generator> = (randomness).clone();
        test_one_103(_N, _M, _randomness)
    });
    ({
        let _N: u32 = N;
        let _M: u32 = N;
        let _randomness: Ptr<support_simple_random_generator> = (randomness).clone();
        test_one_103(_N, _M, _randomness)
    });
}
pub fn test_117(mut N: u32, randomness: Ptr<support_simple_random_generator>) {
    ({
        let _N: u32 = N;
        let _randomness: Ptr<support_simple_random_generator> = (randomness).clone();
        test_one_105(_N, 0_u32, _randomness)
    });
    ({
        let _N: u32 = N;
        let _M: u32 = (N).wrapping_div(4_u32);
        let _randomness: Ptr<support_simple_random_generator> = (randomness).clone();
        test_one_105(_N, _M, _randomness)
    });
    ({
        let _N: u32 = N;
        let _M: u32 = (N).wrapping_div(2_u32);
        let _randomness: Ptr<support_simple_random_generator> = (randomness).clone();
        test_one_105(_N, _M, _randomness)
    });
    ({
        let _N: u32 = N;
        let _M: u32 = ((3_u32).wrapping_mul(N)).wrapping_div(4_u32);
        let _randomness: Ptr<support_simple_random_generator> = (randomness).clone();
        test_one_105(_N, _M, _randomness)
    });
    ({
        let _N: u32 = N;
        let _M: u32 = N;
        let _randomness: Ptr<support_simple_random_generator> = (randomness).clone();
        test_one_105(_N, _M, _randomness)
    });
}
pub fn test_118(mut N: u32, randomness: Ptr<support_simple_random_generator>) {
    ({
        let _N: u32 = N;
        let _randomness: Ptr<support_simple_random_generator> = (randomness).clone();
        test_one_107(_N, 0_u32, _randomness)
    });
    ({
        let _N: u32 = N;
        let _M: u32 = (N).wrapping_div(4_u32);
        let _randomness: Ptr<support_simple_random_generator> = (randomness).clone();
        test_one_107(_N, _M, _randomness)
    });
    ({
        let _N: u32 = N;
        let _M: u32 = (N).wrapping_div(2_u32);
        let _randomness: Ptr<support_simple_random_generator> = (randomness).clone();
        test_one_107(_N, _M, _randomness)
    });
    ({
        let _N: u32 = N;
        let _M: u32 = ((3_u32).wrapping_mul(N)).wrapping_div(4_u32);
        let _randomness: Ptr<support_simple_random_generator> = (randomness).clone();
        test_one_107(_N, _M, _randomness)
    });
    ({
        let _N: u32 = N;
        let _M: u32 = N;
        let _randomness: Ptr<support_simple_random_generator> = (randomness).clone();
        test_one_107(_N, _M, _randomness)
    });
}
pub fn test_119(mut N: u32, randomness: Ptr<support_simple_random_generator>) {
    ({
        let _N: u32 = N;
        let _randomness: Ptr<support_simple_random_generator> = (randomness).clone();
        test_one_111(_N, 0_u32, _randomness)
    });
    ({
        let _N: u32 = N;
        let _M: u32 = (N).wrapping_div(4_u32);
        let _randomness: Ptr<support_simple_random_generator> = (randomness).clone();
        test_one_111(_N, _M, _randomness)
    });
    ({
        let _N: u32 = N;
        let _M: u32 = (N).wrapping_div(2_u32);
        let _randomness: Ptr<support_simple_random_generator> = (randomness).clone();
        test_one_111(_N, _M, _randomness)
    });
    ({
        let _N: u32 = N;
        let _M: u32 = ((3_u32).wrapping_mul(N)).wrapping_div(4_u32);
        let _randomness: Ptr<support_simple_random_generator> = (randomness).clone();
        test_one_111(_N, _M, _randomness)
    });
    ({
        let _N: u32 = N;
        let _M: u32 = N;
        let _randomness: Ptr<support_simple_random_generator> = (randomness).clone();
        test_one_111(_N, _M, _randomness)
    });
}
pub fn test_120(mut N: u32, randomness: Ptr<support_simple_random_generator>) {
    ({
        let _N: u32 = N;
        let _randomness: Ptr<support_simple_random_generator> = (randomness).clone();
        test_one_113(_N, 0_u32, _randomness)
    });
    ({
        let _N: u32 = N;
        let _M: u32 = (N).wrapping_div(4_u32);
        let _randomness: Ptr<support_simple_random_generator> = (randomness).clone();
        test_one_113(_N, _M, _randomness)
    });
    ({
        let _N: u32 = N;
        let _M: u32 = (N).wrapping_div(2_u32);
        let _randomness: Ptr<support_simple_random_generator> = (randomness).clone();
        test_one_113(_N, _M, _randomness)
    });
    ({
        let _N: u32 = N;
        let _M: u32 = ((3_u32).wrapping_mul(N)).wrapping_div(4_u32);
        let _randomness: Ptr<support_simple_random_generator> = (randomness).clone();
        test_one_113(_N, _M, _randomness)
    });
    ({
        let _N: u32 = N;
        let _M: u32 = N;
        let _randomness: Ptr<support_simple_random_generator> = (randomness).clone();
        test_one_113(_N, _M, _randomness)
    });
}
pub fn test_121(randomness: Ptr<support_simple_random_generator>) {
    ({ test_one_99(0_u32, 0_u32, (randomness).clone()) });
    ({ test_one_99(1_u32, 0_u32, (randomness).clone()) });
    ({ test_one_99(1_u32, 1_u32, (randomness).clone()) });
    ({ test_one_99(2_u32, 0_u32, (randomness).clone()) });
    ({ test_one_99(2_u32, 1_u32, (randomness).clone()) });
    ({ test_one_99(2_u32, 2_u32, (randomness).clone()) });
    ({ test_one_99(3_u32, 0_u32, (randomness).clone()) });
    ({ test_one_99(3_u32, 1_u32, (randomness).clone()) });
    ({ test_one_99(3_u32, 2_u32, (randomness).clone()) });
    ({ test_one_99(3_u32, 3_u32, (randomness).clone()) });
    ({ test_115(4_u32, (randomness).clone()) });
    ({ test_115(20_u32, (randomness).clone()) });
    ({ test_115(50_u32, (randomness).clone()) });
    {
        ({ test_115(100_u32, (randomness).clone()) });
        ({ test_115(1000_u32, (randomness).clone()) });
    }
}
pub fn test_122(randomness: Ptr<support_simple_random_generator>) {
    ({ test_one_103(0_u32, 0_u32, (randomness).clone()) });
    ({ test_one_103(1_u32, 0_u32, (randomness).clone()) });
    ({ test_one_103(1_u32, 1_u32, (randomness).clone()) });
    ({ test_one_103(2_u32, 0_u32, (randomness).clone()) });
    ({ test_one_103(2_u32, 1_u32, (randomness).clone()) });
    ({ test_one_103(2_u32, 2_u32, (randomness).clone()) });
    ({ test_one_103(3_u32, 0_u32, (randomness).clone()) });
    ({ test_one_103(3_u32, 1_u32, (randomness).clone()) });
    ({ test_one_103(3_u32, 2_u32, (randomness).clone()) });
    ({ test_one_103(3_u32, 3_u32, (randomness).clone()) });
    ({ test_116(4_u32, (randomness).clone()) });
    ({ test_116(20_u32, (randomness).clone()) });
    ({ test_116(50_u32, (randomness).clone()) });
    {
        ({ test_116(100_u32, (randomness).clone()) });
        ({ test_116(1000_u32, (randomness).clone()) });
    }
}
pub fn test_123(randomness: Ptr<support_simple_random_generator>) {
    ({ test_one_105(0_u32, 0_u32, (randomness).clone()) });
    ({ test_one_105(1_u32, 0_u32, (randomness).clone()) });
    ({ test_one_105(1_u32, 1_u32, (randomness).clone()) });
    ({ test_one_105(2_u32, 0_u32, (randomness).clone()) });
    ({ test_one_105(2_u32, 1_u32, (randomness).clone()) });
    ({ test_one_105(2_u32, 2_u32, (randomness).clone()) });
    ({ test_one_105(3_u32, 0_u32, (randomness).clone()) });
    ({ test_one_105(3_u32, 1_u32, (randomness).clone()) });
    ({ test_one_105(3_u32, 2_u32, (randomness).clone()) });
    ({ test_one_105(3_u32, 3_u32, (randomness).clone()) });
    ({ test_117(4_u32, (randomness).clone()) });
    ({ test_117(20_u32, (randomness).clone()) });
    ({ test_117(50_u32, (randomness).clone()) });
    {
        ({ test_117(100_u32, (randomness).clone()) });
        ({ test_117(1000_u32, (randomness).clone()) });
    }
}
pub fn test_124(randomness: Ptr<support_simple_random_generator>) {
    ({ test_one_107(0_u32, 0_u32, (randomness).clone()) });
    ({ test_one_107(1_u32, 0_u32, (randomness).clone()) });
    ({ test_one_107(1_u32, 1_u32, (randomness).clone()) });
    ({ test_one_107(2_u32, 0_u32, (randomness).clone()) });
    ({ test_one_107(2_u32, 1_u32, (randomness).clone()) });
    ({ test_one_107(2_u32, 2_u32, (randomness).clone()) });
    ({ test_one_107(3_u32, 0_u32, (randomness).clone()) });
    ({ test_one_107(3_u32, 1_u32, (randomness).clone()) });
    ({ test_one_107(3_u32, 2_u32, (randomness).clone()) });
    ({ test_one_107(3_u32, 3_u32, (randomness).clone()) });
    ({ test_118(4_u32, (randomness).clone()) });
    ({ test_118(20_u32, (randomness).clone()) });
    ({ test_118(50_u32, (randomness).clone()) });
    {
        ({ test_118(100_u32, (randomness).clone()) });
        ({ test_118(1000_u32, (randomness).clone()) });
    }
}
pub fn test_125(randomness: Ptr<support_simple_random_generator>) {
    ({ test_one_111(0_u32, 0_u32, (randomness).clone()) });
    ({ test_one_111(1_u32, 0_u32, (randomness).clone()) });
    ({ test_one_111(1_u32, 1_u32, (randomness).clone()) });
    ({ test_one_111(2_u32, 0_u32, (randomness).clone()) });
    ({ test_one_111(2_u32, 1_u32, (randomness).clone()) });
    ({ test_one_111(2_u32, 2_u32, (randomness).clone()) });
    ({ test_one_111(3_u32, 0_u32, (randomness).clone()) });
    ({ test_one_111(3_u32, 1_u32, (randomness).clone()) });
    ({ test_one_111(3_u32, 2_u32, (randomness).clone()) });
    ({ test_one_111(3_u32, 3_u32, (randomness).clone()) });
    ({ test_119(4_u32, (randomness).clone()) });
    ({ test_119(20_u32, (randomness).clone()) });
    ({ test_119(50_u32, (randomness).clone()) });
    {
        ({ test_119(100_u32, (randomness).clone()) });
        ({ test_119(1000_u32, (randomness).clone()) });
    }
}
pub fn test_126(randomness: Ptr<support_simple_random_generator>) {
    ({ test_one_113(0_u32, 0_u32, (randomness).clone()) });
    ({ test_one_113(1_u32, 0_u32, (randomness).clone()) });
    ({ test_one_113(1_u32, 1_u32, (randomness).clone()) });
    ({ test_one_113(2_u32, 0_u32, (randomness).clone()) });
    ({ test_one_113(2_u32, 1_u32, (randomness).clone()) });
    ({ test_one_113(2_u32, 2_u32, (randomness).clone()) });
    ({ test_one_113(3_u32, 0_u32, (randomness).clone()) });
    ({ test_one_113(3_u32, 1_u32, (randomness).clone()) });
    ({ test_one_113(3_u32, 2_u32, (randomness).clone()) });
    ({ test_one_113(3_u32, 3_u32, (randomness).clone()) });
    ({ test_120(4_u32, (randomness).clone()) });
    ({ test_120(20_u32, (randomness).clone()) });
    ({ test_120(50_u32, (randomness).clone()) });
    {
        ({ test_120(100_u32, (randomness).clone()) });
        ({ test_120(1000_u32, (randomness).clone()) });
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct less_by_first {}
impl Callable2<Ptr<(Value<i32>, Value<i32>)>, Ptr<(Value<i32>, Value<i32>)>, bool>
    for less_by_first
{
    fn call(&self, a1: Ptr<(Value<i32>, Value<i32>)>, a2: Ptr<(Value<i32>, Value<i32>)>) -> bool {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            less_by_firstImpl::operator_call_1_struct_std_pair_int__int_(__it, a1, a2)
        })
    }
}
impl CppMove for less_by_first {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn test_PR31166_127() {
    let vec_: Value<Box<[(Value<i32>, Value<i32>)]>> = Rc::new(RefCell::new(Box::new([
        (Rc::new(RefCell::new(1)), Rc::new(RefCell::new(0))),
        (Rc::new(RefCell::new(2)), Rc::new(RefCell::new(0))),
        (Rc::new(RefCell::new(2)), Rc::new(RefCell::new(1))),
        (Rc::new(RefCell::new(2)), Rc::new(RefCell::new(2))),
        (Rc::new(RefCell::new(2)), Rc::new(RefCell::new(3))),
    ])));
    let mut i: i32 = 0;
    'loop_: while (i < 5) {
        let res: Value<Vec<(Value<i32>, Value<i32>)>> = Rc::new(RefCell::new(
            match ({ (vec_.as_pointer() as Ptr<(Value<i32>, Value<i32>)>).offset((5) as isize) },) {
                (mut __tmp_0,) => {
                    let mut __v = Vec::new();
                    let mut __it = (vec_.as_pointer() as Ptr<(Value<i32>, Value<i32>)>);
                    while __it != __tmp_0 {
                        __v.push(<(Value<i32>, Value<i32>) as CppFrom<(
                            Value<i32>,
                            Value<i32>,
                        )>>::cpp_from(__it.iter_deref()));
                        __it.iter_inc();
                    }
                    __v
                }
            },
        ));
        ({
            let ___first: RandomAccessIterator<(Value<i32>, Value<i32>)> =
                CppRange::begin(&res.decay());
            let ___middle: RandomAccessIterator<(Value<i32>, Value<i32>)> =
                CppRange::begin(&res.decay()).iter_add((i as isize));
            let ___last: RandomAccessIterator<(Value<i32>, Value<i32>)> =
                CppRange::end(&res.decay());
            inplace_merge_128(___first, ___middle, ___last, <less_by_first>::default())
        });
        assert!(({ ((*res.borrow()).len() == 5_usize) }));
        assert!(
            ({
                let ___first1: RandomAccessIterator<(Value<i32>, Value<i32>)> =
                    CppRange::begin(&res.decay());
                let ___last1: RandomAccessIterator<(Value<i32>, Value<i32>)> =
                    CppRange::end(&res.decay());
                equal_129(
                    ___first1,
                    ___last1,
                    (vec_.as_pointer() as Ptr<(Value<i32>, Value<i32>)>),
                )
            })
        );
        i.prefix_inc();
    }
}
pub fn test_130() -> bool {
    let randomness: Value<support_simple_random_generator> =
        Rc::new(RefCell::new(support_simple_random_generator::new_1()));
    ({ test_121(randomness.as_pointer()) });
    ({ test_122(randomness.as_pointer()) });
    ({ test_123(randomness.as_pointer()) });
    ({ test_124(randomness.as_pointer()) });
    ({ test_125(randomness.as_pointer()) });
    ({ test_126(randomness.as_pointer()) });
    {
        let mut N: i32 = 100;
        let mut M: i32 = 50;
        let mut ia: Ptr<Option<Value<i32>>> = Ptr::alloc_array(
            (0..(N as usize))
                .map(|_| None)
                .collect::<Box<[Option<Value<i32>>]>>(),
        );
        let mut i: i32 = 0;
        'loop_: while (i < N) {
            {
                let _p: Ptr<_> = Ptr::alloc(i);
                ia.offset((i) as isize).write(_p.to_owned_opt())
            };
            i.prefix_inc();
        }
        ({
            let _first: Ptr<Option<Value<i32>>> = (ia).clone();
            let _last: Ptr<Option<Value<i32>>> = ia.offset((N) as isize);
            shuffle_75(_first, _last, randomness.as_pointer())
        });
        ({
            let ___first: Ptr<Option<Value<i32>>> = (ia).clone();
            let ___last: Ptr<Option<Value<i32>>> = ia.offset((M) as isize);
            libcc2rs::sort_refcount(___first, ___last, <indirect_less>::default())
        });
        ({
            let ___first: Ptr<Option<Value<i32>>> = ia.offset((M) as isize);
            let ___last: Ptr<Option<Value<i32>>> = ia.offset((N) as isize);
            libcc2rs::sort_refcount(___first, ___last, <indirect_less>::default())
        });
        ({
            let ___first: Ptr<Option<Value<i32>>> = (ia).clone();
            let ___middle: Ptr<Option<Value<i32>>> = ia.offset((M) as isize);
            let ___last: Ptr<Option<Value<i32>>> = ia.offset((N) as isize);
            inplace_merge_131(___first, ___middle, ___last, <indirect_less>::default())
        });
        assert!(({ (((*elem!(ia, 0).upgrade().deref()).as_pointer().read()) == 0) }));
        assert!(
            ({
                ({ ((*elem!(ia, (N - 1)).upgrade().deref()).as_pointer().read()) } == { (N - 1) })
            })
        );
        assert!(
            ({
                let ___first: Ptr<Option<Value<i32>>> = (ia).clone();
                let ___last: Ptr<Option<Value<i32>>> = ia.offset((N) as isize);
                is_sorted_132(___first, ___last, <indirect_less>::default())
            })
        );
        ia.delete();
    }
    ({ test_PR31166_127() });
    return true;
}
pub fn main() {
    let size: usize = ::std::env::args().map(|arg| arg.len() + 1).sum();
    let args: Value<Box<[i8]>> = Rc::new(RefCell::new(vec![0; size].into_boxed_slice()));
    let args_ptr: Ptr<i8> = args.as_pointer();
    let mut off = 0;
    let argv: Value<Box<[Ptr<i8>]>> = Rc::new(RefCell::new(
        ::std::env::args()
            .map(|arg| {
                let ptr = args_ptr.offset(off);
                ptr.write_c_bytes(arg.as_bytes());
                off += arg.len() + 1;
                ptr
            })
            .chain(::std::iter::once(Ptr::null()))
            .collect(),
    ));
    __cpp2rust_init_globals();
    ::std::process::exit(main_0(::std::env::args().len() as i32, argv.as_pointer()));
}
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> i32 {
    ({ test_130() });
    return 0;
}
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct binary_counting_predicate_std_greater_int___int__int_;
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct binary_counting_predicate_std_greater_S___S__S_;
pub trait SImpl {
    fn copy_assign(&self, rhs: Ptr<S>) -> Ptr<S>;
    fn move_assign(&self, rhs: Ptr<S>) -> Ptr<S>;
    fn operator_assign_7(&self, i: i32) -> Ptr<S>;
    fn operator_lt(&self, rhs: Ptr<S>) -> bool;
    fn operator_gt(&self, rhs: Ptr<S>) -> bool;
    fn operator_eq_10(&self, rhs: Ptr<S>) -> bool;
    fn operator_eq_11(&self, i: i32) -> bool;
    fn set(&self, i: i32);
}
impl SImpl for Ptr<S> {
    fn copy_assign(&self, rhs: Ptr<S>) -> Ptr<S> {
        field!((*self), i_).write({ rhs.with(|__s| __s.i_) });
        return (*self).clone();
    }
    fn move_assign(&self, rhs: Ptr<S>) -> Ptr<S> {
        field!((*self), i_).write({ rhs.with(|__s| __s.i_) });
        field!(rhs, i_).write(-2_i32);
        assert!(((*self) != (rhs)));
        return (*self).clone();
    }
    fn operator_assign_7(&self, mut i: i32) -> Ptr<S> {
        field!((*self), i_).write(i);
        return (*self).clone();
    }
    fn operator_lt(&self, rhs: Ptr<S>) -> bool {
        return ({ (*self).with(|__s| __s.i_) } < { rhs.with(|__s| __s.i_) });
    }
    fn operator_gt(&self, rhs: Ptr<S>) -> bool {
        return ({ (*self).with(|__s| __s.i_) } > { rhs.with(|__s| __s.i_) });
    }
    fn operator_eq_10(&self, rhs: Ptr<S>) -> bool {
        return ({ (*self).with(|__s| __s.i_) } == { rhs.with(|__s| __s.i_) });
    }
    fn operator_eq_11(&self, mut i: i32) -> bool {
        return ((*self).with(|__s| __s.i_) == i);
    }
    fn set(&self, mut i: i32) {
        field!((*self), i_).write(i);
    }
}
pub trait adl_IteratorImpl {
    fn iter_moves(&self) -> i32;
    fn iter_swaps(&self) -> i32;
    fn operator_deref(&self) -> Ptr<i32>;
    fn operator_index(&self, n: isize) -> Ptr<i32>;
    fn operator_sub_10(&self, n: isize) -> adl_Iterator;
    fn operator_sub_11(&self, rhs: adl_Iterator) -> isize;
    fn operator_add_assign(&self, n: isize) -> Ptr<adl_Iterator>;
    fn operator_sub_assign(&self, n: isize) -> Ptr<adl_Iterator>;
    fn operator_inc_14(&self) -> Ptr<adl_Iterator>;
    fn operator_post_inc_15(&self, _a0: i32) -> adl_Iterator;
    fn operator_dec_16(&self) -> Ptr<adl_Iterator>;
    fn operator_post_dec_17(&self, _a0: i32) -> adl_Iterator;
}
impl adl_IteratorImpl for Ptr<adl_Iterator> {
    fn iter_moves(&self) -> i32 {
        assert!(!((*self).with(|__s| __s.iter_moves_.clone())).is_null());
        return ((*self).with(|__s| __s.iter_moves_.clone()).read());
    }
    fn iter_swaps(&self) -> i32 {
        assert!(!((*self).with(|__s| __s.iter_swaps_.clone())).is_null());
        return ((*self).with(|__s| __s.iter_swaps_.clone()).read());
    }
    fn operator_deref(&self) -> Ptr<i32> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
    fn operator_index(&self, mut n: isize) -> Ptr<i32> {
        return (*self).with(|__s| __s.ptr_.clone()).offset((n) as isize);
    }
    fn operator_sub_10(&self, mut n: isize) -> adl_Iterator {
        return adl_Iterator::new_1(
            { (*self).with(|__s| __s.ptr_.clone()).offset(-((n) as isize)) },
            { (*self).with(|__s| __s.iter_moves_.clone()) },
            { (*self).with(|__s| __s.iter_swaps_.clone()) },
        );
    }
    fn operator_sub_11(&self, mut rhs: adl_Iterator) -> isize {
        return (((*self).with(|__s| __s.ptr_.clone()) - (rhs.ptr_).clone()) as i64 as isize);
    }
    fn operator_add_assign(&self, mut n: isize) -> Ptr<adl_Iterator> {
        {
            let _ptr = field!((*self), ptr_);
            _ptr.write(_ptr.read() + n)
        };
        return (*self).clone();
    }
    fn operator_sub_assign(&self, mut n: isize) -> Ptr<adl_Iterator> {
        {
            let _ptr = field!((*self), ptr_);
            _ptr.write(_ptr.read() - n)
        };
        return (*self).clone();
    }
    fn operator_inc_14(&self) -> Ptr<adl_Iterator> {
        field!((*self), ptr_).with_mut(|__v| {
            __v.prefix_inc();
        });
        return (*self).clone();
    }
    fn operator_post_inc_15(&self, mut _a0: i32) -> adl_Iterator {
        let mut prev: adl_Iterator = (*(*self).upgrade().deref()).clone();
        field!((*self), ptr_).with_mut(|__v| {
            __v.prefix_inc();
        });
        return prev;
    }
    fn operator_dec_16(&self) -> Ptr<adl_Iterator> {
        field!((*self), ptr_).with_mut(|__v| {
            __v.prefix_dec();
        });
        return (*self).clone();
    }
    fn operator_post_dec_17(&self, mut _a0: i32) -> adl_Iterator {
        let mut prev: adl_Iterator = (*(*self).upgrade().deref()).clone();
        field!((*self), ptr_).with_mut(|__v| {
            __v.prefix_dec();
        });
        return prev;
    }
}
pub trait bidirectional_iterator_S_ptr_Impl {
    fn operator_deref(&self) -> Ptr<S>;
    fn operator_inc_6(&self) -> Ptr<bidirectional_iterator_S_ptr_>;
    fn operator_dec_7(&self) -> Ptr<bidirectional_iterator_S_ptr_>;
    fn operator_post_inc_8(&self, mut _a0: i32) -> bidirectional_iterator_S_ptr_ {
        unimplemented!()
    }
    fn operator_post_dec_9(&self, mut _a0: i32) -> bidirectional_iterator_S_ptr_ {
        unimplemented!()
    }
    fn move_assign(
        &self,
        _a0: Ptr<bidirectional_iterator_S_ptr_>,
    ) -> Ptr<bidirectional_iterator_S_ptr_>;
}
impl bidirectional_iterator_S_ptr_Impl for Ptr<bidirectional_iterator_S_ptr_> {
    fn operator_deref(&self) -> Ptr<S> {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_inc_6(&self) -> Ptr<bidirectional_iterator_S_ptr_> {
        field!((*self), it_).with_mut(|__v| {
            __v.prefix_inc();
        });
        return (*self).clone();
    }
    fn operator_dec_7(&self) -> Ptr<bidirectional_iterator_S_ptr_> {
        field!((*self), it_).with_mut(|__v| {
            __v.prefix_dec();
        });
        return (*self).clone();
    }
    fn move_assign(
        &self,
        _a0: Ptr<bidirectional_iterator_S_ptr_>,
    ) -> Ptr<bidirectional_iterator_S_ptr_> {
        field!((*self), it_).write({ { (*_a0.upgrade().deref()).it_.clone() } });
        ({
            let _other: Ptr<support_double_move_tracker> = field_ptr!(_a0, tracker_);
            support_double_move_trackerImpl::move_assign(&field_ptr!((*self), tracker_), _other)
        });
        return (*self).clone();
    }
}
pub trait bidirectional_iterator_int_ptr_Impl {
    fn operator_deref(&self) -> Ptr<i32>;
    fn operator_inc_6(&self) -> Ptr<bidirectional_iterator_int_ptr_>;
    fn operator_dec_7(&self) -> Ptr<bidirectional_iterator_int_ptr_>;
    fn operator_post_inc_8(&self, mut _a0: i32) -> bidirectional_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_post_dec_9(&self, mut _a0: i32) -> bidirectional_iterator_int_ptr_ {
        unimplemented!()
    }
    fn move_assign(
        &self,
        _a0: Ptr<bidirectional_iterator_int_ptr_>,
    ) -> Ptr<bidirectional_iterator_int_ptr_>;
}
impl bidirectional_iterator_int_ptr_Impl for Ptr<bidirectional_iterator_int_ptr_> {
    fn operator_deref(&self) -> Ptr<i32> {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_inc_6(&self) -> Ptr<bidirectional_iterator_int_ptr_> {
        field!((*self), it_).with_mut(|__v| {
            __v.prefix_inc();
        });
        return (*self).clone();
    }
    fn operator_dec_7(&self) -> Ptr<bidirectional_iterator_int_ptr_> {
        field!((*self), it_).with_mut(|__v| {
            __v.prefix_dec();
        });
        return (*self).clone();
    }
    fn move_assign(
        &self,
        _a0: Ptr<bidirectional_iterator_int_ptr_>,
    ) -> Ptr<bidirectional_iterator_int_ptr_> {
        field!((*self), it_).write({ { (*_a0.upgrade().deref()).it_.clone() } });
        ({
            let _other: Ptr<support_double_move_tracker> = field_ptr!(_a0, tracker_);
            support_double_move_trackerImpl::move_assign(&field_ptr!((*self), tracker_), _other)
        });
        return (*self).clone();
    }
}
pub trait cpp17_input_iterator_int_ptr__int_ptr_Impl {
    fn operator_deref(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn operator_inc_5(&self) -> Ptr<cpp17_input_iterator_int_ptr__int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_6(&self, mut _a0: i32) -> cpp17_input_iterator_int_ptr__int_ptr_ {
        unimplemented!()
    }
    fn move_assign(
        &self,
        _a0: Ptr<cpp17_input_iterator_int_ptr__int_ptr_>,
    ) -> Ptr<cpp17_input_iterator_int_ptr__int_ptr_>;
}
impl cpp17_input_iterator_int_ptr__int_ptr_Impl for Ptr<cpp17_input_iterator_int_ptr__int_ptr_> {
    fn move_assign(
        &self,
        _a0: Ptr<cpp17_input_iterator_int_ptr__int_ptr_>,
    ) -> Ptr<cpp17_input_iterator_int_ptr__int_ptr_> {
        field!((*self), it_).write({ { (*_a0.upgrade().deref()).it_.clone() } });
        ({
            let _other: Ptr<support_double_move_tracker> = field_ptr!(_a0, tracker_);
            support_double_move_trackerImpl::move_assign(&field_ptr!((*self), tracker_), _other)
        });
        return (*self).clone();
    }
}
pub trait cpp17_output_iterator_int_ptr_Impl {
    fn operator_deref(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn operator_inc_5(&self) -> Ptr<cpp17_output_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_6(&self, mut _a0: i32) -> cpp17_output_iterator_int_ptr_ {
        unimplemented!()
    }
    fn move_assign(
        &self,
        _a0: Ptr<cpp17_output_iterator_int_ptr_>,
    ) -> Ptr<cpp17_output_iterator_int_ptr_>;
}
impl cpp17_output_iterator_int_ptr_Impl for Ptr<cpp17_output_iterator_int_ptr_> {
    fn move_assign(
        &self,
        _a0: Ptr<cpp17_output_iterator_int_ptr_>,
    ) -> Ptr<cpp17_output_iterator_int_ptr_> {
        field!((*self), it_).write({ { (*_a0.upgrade().deref()).it_.clone() } });
        ({
            let _other: Ptr<support_double_move_tracker> = field_ptr!(_a0, tracker_);
            support_double_move_trackerImpl::move_assign(&field_ptr!((*self), tracker_), _other)
        });
        return (*self).clone();
    }
}
pub trait cpp20_input_iterator_int_ptr_Impl {
    fn move_assign(
        &self,
        _a0: Ptr<cpp20_input_iterator_int_ptr_>,
    ) -> Ptr<cpp20_input_iterator_int_ptr_>;
    fn operator_deref(&self) -> Ptr<i32>;
    fn operator_inc_5(&self) -> Ptr<cpp20_input_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_6(&self, mut _a0: i32) {
        unimplemented!()
    }
}
impl cpp20_input_iterator_int_ptr_Impl for Ptr<cpp20_input_iterator_int_ptr_> {
    fn move_assign(
        &self,
        _a0: Ptr<cpp20_input_iterator_int_ptr_>,
    ) -> Ptr<cpp20_input_iterator_int_ptr_> {
        field!((*self), it_).write({ { (*_a0.upgrade().deref()).it_.clone() } });
        ({
            let _other: Ptr<support_double_move_tracker> = field_ptr!(_a0, tracker_);
            support_double_move_trackerImpl::move_assign(&field_ptr!((*self), tracker_), _other)
        });
        return (*self).clone();
    }
    fn operator_deref(&self) -> Ptr<i32> {
        return (*self).with(|__s| __s.it_.clone());
    }
}
pub trait cpp20_output_iterator_int_ptr_Impl {
    fn move_assign(
        &self,
        _a0: Ptr<cpp20_output_iterator_int_ptr_>,
    ) -> Ptr<cpp20_output_iterator_int_ptr_>;
    fn operator_deref(&self) -> Ptr<i32>;
    fn operator_inc_5(&self) -> Ptr<cpp20_output_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_6(&self, mut _a0: i32) -> cpp20_output_iterator_int_ptr_ {
        unimplemented!()
    }
}
impl cpp20_output_iterator_int_ptr_Impl for Ptr<cpp20_output_iterator_int_ptr_> {
    fn move_assign(
        &self,
        _a0: Ptr<cpp20_output_iterator_int_ptr_>,
    ) -> Ptr<cpp20_output_iterator_int_ptr_> {
        field!((*self), it_).write({ { (*_a0.upgrade().deref()).it_.clone() } });
        ({
            let _other: Ptr<support_double_move_tracker> = field_ptr!(_a0, tracker_);
            support_double_move_trackerImpl::move_assign(&field_ptr!((*self), tracker_), _other)
        });
        return (*self).clone();
    }
    fn operator_deref(&self) -> Ptr<i32> {
        return (*self).with(|__s| __s.it_.clone());
    }
}
pub trait cpp20_random_access_iterator_int_ptr_Impl {
    fn operator_deref(&self) -> Ptr<i32>;
    fn operator_index(&self, n: isize) -> Ptr<i32>;
    fn operator_inc_7(&self) -> Ptr<cpp20_random_access_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_dec_8(&self) -> Ptr<cpp20_random_access_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_9(&self, mut _a0: i32) -> cpp20_random_access_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_post_dec_10(&self, mut _a0: i32) -> cpp20_random_access_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_add_assign(&self, mut n: isize) -> Ptr<cpp20_random_access_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_sub_assign(&self, mut n: isize) -> Ptr<cpp20_random_access_iterator_int_ptr_> {
        unimplemented!()
    }
    fn move_assign(
        &self,
        _a0: Ptr<cpp20_random_access_iterator_int_ptr_>,
    ) -> Ptr<cpp20_random_access_iterator_int_ptr_>;
}
impl cpp20_random_access_iterator_int_ptr_Impl for Ptr<cpp20_random_access_iterator_int_ptr_> {
    fn operator_deref(&self) -> Ptr<i32> {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_index(&self, mut n: isize) -> Ptr<i32> {
        return (*self).with(|__s| __s.it_.clone()).offset((n) as isize);
    }
    fn move_assign(
        &self,
        _a0: Ptr<cpp20_random_access_iterator_int_ptr_>,
    ) -> Ptr<cpp20_random_access_iterator_int_ptr_> {
        field!((*self), it_).write({ { (*_a0.upgrade().deref()).it_.clone() } });
        ({
            let _other: Ptr<support_double_move_tracker> = field_ptr!(_a0, tracker_);
            support_double_move_trackerImpl::move_assign(&field_ptr!((*self), tracker_), _other)
        });
        return (*self).clone();
    }
}
pub trait forward_sized_iterator_int_ptr_Impl {
    fn operator_deref(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn operator_inc_4(&self) -> Ptr<forward_sized_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_5(&self, mut _a0: i32) -> forward_sized_iterator_int_ptr_ {
        unimplemented!()
    }
}
impl forward_sized_iterator_int_ptr_Impl for Ptr<forward_sized_iterator_int_ptr_> {}
pub trait indirect_lessImpl {
    fn operator_call_1_class_std_unique_ptr_int_(
        &self,
        x: Ptr<Option<Value<i32>>>,
        y: Ptr<Option<Value<i32>>>,
    ) -> bool;
}
impl indirect_lessImpl for Ptr<indirect_less> {
    fn operator_call_1_class_std_unique_ptr_int_(
        &self,
        x: Ptr<Option<Value<i32>>>,
        y: Ptr<Option<Value<i32>>>,
    ) -> bool {
        return ({ ((*x.upgrade().deref()).as_pointer().read()) } < {
            ((*y.upgrade().deref()).as_pointer().read())
        });
    }
}
pub trait less_by_firstImpl {
    fn operator_call_1_struct_std_pair_int__int_(
        &self,
        lhs: Ptr<(Value<i32>, Value<i32>)>,
        rhs: Ptr<(Value<i32>, Value<i32>)>,
    ) -> bool;
}
impl less_by_firstImpl for Ptr<less_by_first> {
    fn operator_call_1_struct_std_pair_int__int_(
        &self,
        lhs: Ptr<(Value<i32>, Value<i32>)>,
        rhs: Ptr<(Value<i32>, Value<i32>)>,
    ) -> bool {
        return ({
            let ___x: Ptr<i32> = (*lhs.upgrade().deref()).0.as_pointer();
            let ___y: Ptr<i32> = (*rhs.upgrade().deref()).0.as_pointer();
            <std_less_int_>::default()(___x, ___y)
        });
    }
}
pub trait random_access_iterator_S_ptr_Impl {
    fn operator_deref(&self) -> Ptr<S>;
    fn operator_index(&self, mut n: isize) -> Ptr<S> {
        unimplemented!()
    }
    fn operator_inc_7(&self) -> Ptr<random_access_iterator_S_ptr_>;
    fn operator_dec_8(&self) -> Ptr<random_access_iterator_S_ptr_>;
    fn operator_post_inc_9(&self, mut _a0: i32) -> random_access_iterator_S_ptr_ {
        unimplemented!()
    }
    fn operator_post_dec_10(&self, mut _a0: i32) -> random_access_iterator_S_ptr_ {
        unimplemented!()
    }
    fn operator_add_assign(&self, n: isize) -> Ptr<random_access_iterator_S_ptr_>;
    fn operator_sub_assign(&self, n: isize) -> Ptr<random_access_iterator_S_ptr_>;
    fn move_assign(
        &self,
        _a0: Ptr<random_access_iterator_S_ptr_>,
    ) -> Ptr<random_access_iterator_S_ptr_>;
}
impl random_access_iterator_S_ptr_Impl for Ptr<random_access_iterator_S_ptr_> {
    fn operator_deref(&self) -> Ptr<S> {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_inc_7(&self) -> Ptr<random_access_iterator_S_ptr_> {
        field!((*self), it_).with_mut(|__v| {
            __v.prefix_inc();
        });
        return (*self).clone();
    }
    fn operator_dec_8(&self) -> Ptr<random_access_iterator_S_ptr_> {
        field!((*self), it_).with_mut(|__v| {
            __v.prefix_dec();
        });
        return (*self).clone();
    }
    fn operator_add_assign(&self, mut n: isize) -> Ptr<random_access_iterator_S_ptr_> {
        {
            let _ptr = field!((*self), it_);
            _ptr.write(_ptr.read() + n)
        };
        return (*self).clone();
    }
    fn operator_sub_assign(&self, mut n: isize) -> Ptr<random_access_iterator_S_ptr_> {
        {
            let _ptr = field!((*self), it_);
            _ptr.write(_ptr.read() - n)
        };
        return (*self).clone();
    }
    fn move_assign(
        &self,
        _a0: Ptr<random_access_iterator_S_ptr_>,
    ) -> Ptr<random_access_iterator_S_ptr_> {
        field!((*self), it_).write({ { (*_a0.upgrade().deref()).it_.clone() } });
        ({
            let _other: Ptr<support_double_move_tracker> = field_ptr!(_a0, tracker_);
            support_double_move_trackerImpl::move_assign(&field_ptr!((*self), tracker_), _other)
        });
        return (*self).clone();
    }
}
pub trait random_access_iterator_int_ptr_Impl {
    fn operator_deref(&self) -> Ptr<i32>;
    fn operator_index(&self, mut n: isize) -> Ptr<i32> {
        unimplemented!()
    }
    fn operator_inc_7(&self) -> Ptr<random_access_iterator_int_ptr_>;
    fn operator_dec_8(&self) -> Ptr<random_access_iterator_int_ptr_>;
    fn operator_post_inc_9(&self, mut _a0: i32) -> random_access_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_post_dec_10(&self, mut _a0: i32) -> random_access_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_add_assign(&self, n: isize) -> Ptr<random_access_iterator_int_ptr_>;
    fn operator_sub_assign(&self, n: isize) -> Ptr<random_access_iterator_int_ptr_>;
    fn move_assign(
        &self,
        _a0: Ptr<random_access_iterator_int_ptr_>,
    ) -> Ptr<random_access_iterator_int_ptr_>;
}
impl random_access_iterator_int_ptr_Impl for Ptr<random_access_iterator_int_ptr_> {
    fn operator_deref(&self) -> Ptr<i32> {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_inc_7(&self) -> Ptr<random_access_iterator_int_ptr_> {
        field!((*self), it_).with_mut(|__v| {
            __v.prefix_inc();
        });
        return (*self).clone();
    }
    fn operator_dec_8(&self) -> Ptr<random_access_iterator_int_ptr_> {
        field!((*self), it_).with_mut(|__v| {
            __v.prefix_dec();
        });
        return (*self).clone();
    }
    fn operator_add_assign(&self, mut n: isize) -> Ptr<random_access_iterator_int_ptr_> {
        {
            let _ptr = field!((*self), it_);
            _ptr.write(_ptr.read() + n)
        };
        return (*self).clone();
    }
    fn operator_sub_assign(&self, mut n: isize) -> Ptr<random_access_iterator_int_ptr_> {
        {
            let _ptr = field!((*self), it_);
            _ptr.write(_ptr.read() - n)
        };
        return (*self).clone();
    }
    fn move_assign(
        &self,
        _a0: Ptr<random_access_iterator_int_ptr_>,
    ) -> Ptr<random_access_iterator_int_ptr_> {
        field!((*self), it_).write({ { (*_a0.upgrade().deref()).it_.clone() } });
        ({
            let _other: Ptr<support_double_move_tracker> = field_ptr!(_a0, tracker_);
            support_double_move_trackerImpl::move_assign(&field_ptr!((*self), tracker_), _other)
        });
        return (*self).clone();
    }
}
pub trait rvalue_iterator_int_ptr_Impl {
    fn operator_deref(&self) -> Ptr<Ptr<i32>> {
        unimplemented!()
    }
    fn operator_inc_4(&self) -> Ptr<rvalue_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_5(&self, mut _a0: i32) -> rvalue_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_dec_6(&self) -> Ptr<rvalue_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_dec_7(&self, mut _a0: i32) -> rvalue_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_add(&self, mut n: isize) -> rvalue_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_sub_9(&self, mut n: isize) -> rvalue_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_sub_10(&self, other: Ptr<rvalue_iterator_int_ptr_>) -> isize {
        unimplemented!()
    }
    fn operator_add_assign(&self, mut n: isize) -> Ptr<rvalue_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_sub_assign(&self, mut n: isize) -> Ptr<rvalue_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_index(&self, mut n: isize) -> Ptr<Ptr<i32>> {
        unimplemented!()
    }
    fn operator_cmp(&self, _a0: Ptr<rvalue_iterator_int_ptr_>) -> std::cmp::Ordering;
    fn operator_eq(&self, _a0: Ptr<rvalue_iterator_int_ptr_>) -> bool;
}
impl rvalue_iterator_int_ptr_Impl for Ptr<rvalue_iterator_int_ptr_> {
    fn operator_cmp(&self, _a0: Ptr<rvalue_iterator_int_ptr_>) -> std::cmp::Ordering {
        {
            let cmp: Value<std::cmp::Ordering> = Rc::new(RefCell::new(std::cmp::Ord::cmp(
                &((*self).with(|__s| __s.it_.clone())),
                &(_a0.with(|__s| __s.it_.clone())),
            )));
            if !(({ (*cmp.borrow()) }) == std::cmp::Ordering::Equal) {
                return ({ (*cmp.borrow()) });
            }
        }
        return ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) });
    }
    fn operator_eq(&self, _a0: Ptr<rvalue_iterator_int_ptr_>) -> bool {
        return ({ (*self).with(|__s| __s.it_.clone()) } == { _a0.with(|__s| __s.it_.clone()) });
    }
}
pub trait support_detail_independent_bits_engine_support_simple_random_generator__unsigned_long_Impl
{
    fn operator_call(&self) -> u64;
    fn generate_3(&self, _a0: std_integral_constant_bool__false_) -> u64 {
        unimplemented!()
    }
    fn generate_4(&self, _a0: std_integral_constant_bool__true_) -> u64;
}
impl support_detail_independent_bits_engine_support_simple_random_generator__unsigned_long_Impl
    for Ptr<support_detail_independent_bits_engine_support_simple_random_generator__unsigned_long_>
{
    fn operator_call(&self) -> u64 {
        return ({
            support_detail_independent_bits_engine_support_simple_random_generator__unsigned_long_Impl :: generate_4 ( self , <std_integral_constant_bool__true_  >::default()  , )
        });
    }
    fn generate_4(&self, _a0: std_integral_constant_bool__true_) -> u64 {
        let _a0: Value<std_integral_constant_bool__true_> = Rc::new(RefCell::new(_a0));
        let mut r_digits: usize = (digits_71.with(|rc| *rc.borrow()) as usize);
        let result: Value<u64> = Rc::new(RefCell::new(0_u64));
        let mut k: usize = 0_usize;
        'loop_: while (k < (*self).with(|__s| __s.round_count_regular_)) {
            let mut eng_result: u16 = 0_u16;
            let mut __do_while = true;
            'loop_: while __do_while || ((eng_result as u64) >= (*self).with(|__s| __s.y0_)) {
                __do_while = false;
                eng_result = (({
                    (({
                        support_simple_random_generatorImpl::operator_call(
                            &(*self).with(|__s| __s.eng_.clone()),
                        )
                    }) as i32)
                } - { (({ support_simple_random_generator::min() }) as i32) })
                    as u16);
            }
            if ((*self).with(|__s| __s.wid0_) < r_digits) {
                (*result.borrow_mut()) <<= ((*self).with(|__s| __s.wid0_) as u64);
            } else {
                (*result.borrow_mut()) = 0_u64;
            }
            (*result.borrow_mut()) = {
                (*result.borrow()).wrapping_add(
                    (((eng_result as i32) & ((*self).with(|__s| __s.mask0_) as i32)) as u64),
                )
            };
            k.prefix_inc();
        }
        let mut k: usize = (*self).with(|__s| __s.round_count_regular_);
        'loop_: while (k < (*self).with(|__s| __s.round_count_all_)) {
            let mut eng_result: u16 = 0_u16;
            let mut __do_while = true;
            'loop_: while __do_while || ((eng_result as u64) >= (*self).with(|__s| __s.y1_)) {
                __do_while = false;
                eng_result = (({
                    (({
                        support_simple_random_generatorImpl::operator_call(
                            &(*self).with(|__s| __s.eng_.clone()),
                        )
                    }) as i32)
                } - { (({ support_simple_random_generator::min() }) as i32) })
                    as u16);
            }
            if ((*self).with(|__s| __s.wid0_) < (r_digits).wrapping_sub(1_usize)) {
                (*result.borrow_mut()) <<=
                    (((*self).with(|__s| __s.wid0_)).wrapping_add(1_usize) as u64);
            } else {
                (*result.borrow_mut()) = 0_u64;
            }
            (*result.borrow_mut()) = {
                (*result.borrow()).wrapping_add(
                    (((eng_result as i32) & ((*self).with(|__s| __s.mask1_) as i32)) as u64),
                )
            };
            k.prefix_inc();
        }
        return (*result.borrow());
    }
}
pub trait support_double_move_trackerImpl {
    fn move_assign(
        &self,
        other: Ptr<support_double_move_tracker>,
    ) -> Ptr<support_double_move_tracker>;
}
impl support_double_move_trackerImpl for Ptr<support_double_move_tracker> {
    fn move_assign(
        &self,
        other: Ptr<support_double_move_tracker>,
    ) -> Ptr<support_double_move_tracker> {
        assert!(!(other.with(|__s| __s.moved_from_)));
        field!(other, moved_from_).write(true);
        field!((*self), moved_from_).write(false);
        return (*self).clone();
    }
}
pub trait support_simple_random_generatorImpl {
    fn operator_call(&self) -> u16;
}
impl support_simple_random_generatorImpl for Ptr<support_simple_random_generator> {
    fn operator_call(&self) -> u16 {
        field!((*self), status_).write({
            (((*self).with(|__s| __s.status_)).wrapping_mul((214013_u32 as u32)))
                .wrapping_add((2531011_u32 as u32))
        });
        return (((*self).with(|__s| __s.status_) >> 16) as u16);
    }
}
pub trait support_uniform_int_distribution_long_Impl {
    fn reset(&self) {
        unimplemented!()
    }
    fn a(&self) -> i64 {
        unimplemented!()
    }
    fn b(&self) -> i64 {
        unimplemented!()
    }
    fn param_9(&self) -> support_uniform_int_distribution_long__param_type {
        unimplemented!()
    }
    fn param_10(&self, param: Ptr<support_uniform_int_distribution_long__param_type>) {
        unimplemented!()
    }
    fn min(&self) -> i64 {
        unimplemented!()
    }
    fn max(&self) -> i64 {
        unimplemented!()
    }
    fn operator_call_6_class_support_simple_random_generator(
        &self,
        gen_: Ptr<support_simple_random_generator>,
        param: Ptr<support_uniform_int_distribution_long__param_type>,
    ) -> i64;
}
impl support_uniform_int_distribution_long_Impl for Ptr<support_uniform_int_distribution_long_> {
    fn operator_call_6_class_support_simple_random_generator(
        &self,
        gen_: Ptr<support_simple_random_generator>,
        param: Ptr<support_uniform_int_distribution_long__param_type>,
    ) -> i64 {
        const _: () = assert!(true, "detail::is_valid_urng<URNG>::value");;
        let mut rp: u64 = ((({ support_uniform_int_distribution_long__param_typeImpl::b(&param) })
            as u64)
            .wrapping_sub(
                (({ support_uniform_int_distribution_long__param_typeImpl::a(&param) }) as u64),
            ))
        .wrapping_add((1_u64 as u64));
        if (rp == 1_u64) {
            return ({ support_uniform_int_distribution_long__param_typeImpl::a(&param) });
        }
        let mut ur_digits: usize = (digits_71.with(|rc| *rc.borrow()) as usize);
        if (rp == 0_u64) {
            return (({
                support_detail_independent_bits_engine_support_simple_random_generator__unsigned_long_Impl :: operator_call ( &Rc::new(RefCell::new(support_detail_independent_bits_engine_support_simple_random_generator__unsigned_long_ :: new ( { (gen_ ).clone() } , { ur_digits  } , )  )).as_pointer() , )
            }) as i64);
        }
        let mut width: usize =
            ((ur_digits).wrapping_sub((({ countl_zero_66(rp) }) as usize))).wrapping_sub(1_usize);
        if ((rp & (u64::MAX >> ((ur_digits).wrapping_sub(width)))) != 0_u64) {
            width.prefix_inc();
        }
        let eng : Value<support_detail_independent_bits_engine_support_simple_random_generator__unsigned_long_ > = Rc::new(RefCell::new(support_detail_independent_bits_engine_support_simple_random_generator__unsigned_long_ :: new ( { (gen_ ).clone() } , { width  } , )  )) ;
        let mut eng_result: u64 = 0_u64;
        let mut __do_while = true;
        'loop_: while __do_while || (eng_result >= rp) {
            __do_while = false;
            eng_result = ({
                support_detail_independent_bits_engine_support_simple_random_generator__unsigned_long_Impl :: operator_call ( &eng .as_pointer()  , )
            });
        }
        return (((eng_result).wrapping_add(
            (({ support_uniform_int_distribution_long__param_typeImpl::a(&param) }) as u64),
        )) as i64);
    }
}
pub trait support_uniform_int_distribution_long__param_typeImpl {
    fn a(&self) -> i64;
    fn b(&self) -> i64;
}
impl support_uniform_int_distribution_long__param_typeImpl
    for Ptr<support_uniform_int_distribution_long__param_type>
{
    fn a(&self) -> i64 {
        return (*self).with(|__s| __s.a_);
    }
    fn b(&self) -> i64 {
        return (*self).with(|__s| __s.b_);
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = value_0.with(|_| ());
    let _ = value_1.with(|_| ());
    let _ = value_2.with(|_| ());
    let _ = value_3.with(|_| ());
    let _ = value_4.with(|_| ());
    let _ = value_5.with(|_| ());
    let _ = value_6.with(|_| ());
    let _ = value_7.with(|_| ());
    let _ = value_8.with(|_| ());
    let _ = value_9.with(|_| ());
    let _ = value_10.with(|_| ());
    let _ = value_11.with(|_| ());
    let _ = value_12.with(|_| ());
    let _ = value_13.with(|_| ());
    let _ = value_14.with(|_| ());
    let _ = value_15.with(|_| ());
    let _ = value_16.with(|_| ());
    let _ = value_17.with(|_| ());
    let _ = value_18.with(|_| ());
    let _ = value_19.with(|_| ());
    let _ = value_20.with(|_| ());
    let _ = value_21.with(|_| ());
    let _ = value_22.with(|_| ());
    let _ = value_23.with(|_| ());
    let _ = value_24.with(|_| ());
    let _ = value_25.with(|_| ());
    let _ = value_26.with(|_| ());
    let _ = value_27.with(|_| ());
    let _ = value_28.with(|_| ());
    let _ = value_29.with(|_| ());
    let _ = value_30.with(|_| ());
    let _ = value_31.with(|_| ());
    let _ = value_32.with(|_| ());
    let _ = value_33.with(|_| ());
    let _ = value_34.with(|_| ());
    let _ = value_35.with(|_| ());
    let _ = value_36.with(|_| ());
    let _ = value_37.with(|_| ());
    let _ = value_38.with(|_| ());
    let _ = value_39.with(|_| ());
    let _ = value_40.with(|_| ());
    let _ = value_41.with(|_| ());
    let _ = value_42.with(|_| ());
    let _ = value_43.with(|_| ());
    let _ = value_44.with(|_| ());
    let _ = value_45.with(|_| ());
    let _ = value_46.with(|_| ());
    let _ = value_47.with(|_| ());
    let _ = value_48.with(|_| ());
    let _ = value_49.with(|_| ());
    let _ = value_50.with(|_| ());
    let _ = value_51.with(|_| ());
    let _ = value_52.with(|_| ());
    let _ = value_53.with(|_| ());
    let _ = value_54.with(|_| ());
    let _ = value_55.with(|_| ());
    let _ = value_56.with(|_| ());
    let _ = value_57.with(|_| ());
    let _ = value_58.with(|_| ());
    let _ = value_59.with(|_| ());
    let _ = value_60.with(|_| ());
    let _ = value_61.with(|_| ());
    let _ = value_62.with(|_| ());
    let _ = value_63.with(|_| ());
    let _ = value_64.with(|_| ());
    let _ = rp_67.with(|_| ());
    let _ = rp_log2_68.with(|_| ());
    let _ = w_digits_69.with(|_| ());
    let _ = e_digits_70.with(|_| ());
    let _ = default_seed_74.with(|_| ());
}
