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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_is_valid_integer_type_for_random_short_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_is_valid_integer_type_for_random_int_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_is_valid_integer_type_for_random_long_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_is_valid_integer_type_for_random_long_long_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_is_valid_integer_type_for_random_unsigned_char_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_is_valid_integer_type_for_random_unsigned_short_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_is_valid_integer_type_for_random_unsigned_int_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_is_valid_integer_type_for_random_unsigned_long_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_is_valid_integer_type_for_random_unsigned_long_long_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_is_valid_integer_type_for_random___int128_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_is_valid_integer_type_for_random_unsigned___int128_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_is_valid_urng_support_simple_random_generator__void_ {}
thread_local!(
    pub static value_0: Value<()> =
        Rc::new(RefCell::new(if ((65536_() & ((1_()) << 63_())) != 0) {
            63_()
        } else {
            value_1.with(|rc| *rc.borrow())
        }));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__63_ {}
thread_local!(
    pub static value_1: Value<()> =
        Rc::new(RefCell::new(if ((65536_() & ((1_()) << 62_())) != 0) {
            62_()
        } else {
            value_2.with(|rc| *rc.borrow())
        }));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__62_ {}
thread_local!(
    pub static value_2: Value<()> =
        Rc::new(RefCell::new(if ((65536_() & ((1_()) << 61_())) != 0) {
            61_()
        } else {
            value_3.with(|rc| *rc.borrow())
        }));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__61_ {}
thread_local!(
    pub static value_3: Value<()> =
        Rc::new(RefCell::new(if ((65536_() & ((1_()) << 60_())) != 0) {
            60_()
        } else {
            value_4.with(|rc| *rc.borrow())
        }));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__60_ {}
thread_local!(
    pub static value_4: Value<()> =
        Rc::new(RefCell::new(if ((65536_() & ((1_()) << 59_())) != 0) {
            59_()
        } else {
            value_5.with(|rc| *rc.borrow())
        }));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__59_ {}
thread_local!(
    pub static value_5: Value<()> =
        Rc::new(RefCell::new(if ((65536_() & ((1_()) << 58_())) != 0) {
            58_()
        } else {
            value_6.with(|rc| *rc.borrow())
        }));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__58_ {}
thread_local!(
    pub static value_6: Value<()> =
        Rc::new(RefCell::new(if ((65536_() & ((1_()) << 57_())) != 0) {
            57_()
        } else {
            value_7.with(|rc| *rc.borrow())
        }));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__57_ {}
thread_local!(
    pub static value_7: Value<()> =
        Rc::new(RefCell::new(if ((65536_() & ((1_()) << 56_())) != 0) {
            56_()
        } else {
            value_8.with(|rc| *rc.borrow())
        }));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__56_ {}
thread_local!(
    pub static value_8: Value<()> =
        Rc::new(RefCell::new(if ((65536_() & ((1_()) << 55_())) != 0) {
            55_()
        } else {
            value_9.with(|rc| *rc.borrow())
        }));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__55_ {}
thread_local!(
    pub static value_9: Value<()> =
        Rc::new(RefCell::new(if ((65536_() & ((1_()) << 54_())) != 0) {
            54_()
        } else {
            value_10.with(|rc| *rc.borrow())
        }));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__54_ {}
thread_local!(
    pub static value_10: Value<()> =
        Rc::new(RefCell::new(if ((65536_() & ((1_()) << 53_())) != 0) {
            53_()
        } else {
            value_11.with(|rc| *rc.borrow())
        }));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__53_ {}
thread_local!(
    pub static value_11: Value<()> =
        Rc::new(RefCell::new(if ((65536_() & ((1_()) << 52_())) != 0) {
            52_()
        } else {
            value_12.with(|rc| *rc.borrow())
        }));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__52_ {}
thread_local!(
    pub static value_12: Value<()> =
        Rc::new(RefCell::new(if ((65536_() & ((1_()) << 51_())) != 0) {
            51_()
        } else {
            value_13.with(|rc| *rc.borrow())
        }));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__51_ {}
thread_local!(
    pub static value_13: Value<()> =
        Rc::new(RefCell::new(if ((65536_() & ((1_()) << 50_())) != 0) {
            50_()
        } else {
            value_14.with(|rc| *rc.borrow())
        }));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__50_ {}
thread_local!(
    pub static value_14: Value<()> =
        Rc::new(RefCell::new(if ((65536_() & ((1_()) << 49_())) != 0) {
            49_()
        } else {
            value_15.with(|rc| *rc.borrow())
        }));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__49_ {}
thread_local!(
    pub static value_15: Value<()> =
        Rc::new(RefCell::new(if ((65536_() & ((1_()) << 48_())) != 0) {
            48_()
        } else {
            value_16.with(|rc| *rc.borrow())
        }));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__48_ {}
thread_local!(
    pub static value_16: Value<()> =
        Rc::new(RefCell::new(if ((65536_() & ((1_()) << 47_())) != 0) {
            47_()
        } else {
            value_17.with(|rc| *rc.borrow())
        }));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__47_ {}
thread_local!(
    pub static value_17: Value<()> =
        Rc::new(RefCell::new(if ((65536_() & ((1_()) << 46_())) != 0) {
            46_()
        } else {
            value_18.with(|rc| *rc.borrow())
        }));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__46_ {}
thread_local!(
    pub static value_18: Value<()> =
        Rc::new(RefCell::new(if ((65536_() & ((1_()) << 45_())) != 0) {
            45_()
        } else {
            value_19.with(|rc| *rc.borrow())
        }));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__45_ {}
thread_local!(
    pub static value_19: Value<()> =
        Rc::new(RefCell::new(if ((65536_() & ((1_()) << 44_())) != 0) {
            44_()
        } else {
            value_20.with(|rc| *rc.borrow())
        }));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__44_ {}
thread_local!(
    pub static value_20: Value<()> =
        Rc::new(RefCell::new(if ((65536_() & ((1_()) << 43_())) != 0) {
            43_()
        } else {
            value_21.with(|rc| *rc.borrow())
        }));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__43_ {}
thread_local!(
    pub static value_21: Value<()> =
        Rc::new(RefCell::new(if ((65536_() & ((1_()) << 42_())) != 0) {
            42_()
        } else {
            value_22.with(|rc| *rc.borrow())
        }));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__42_ {}
thread_local!(
    pub static value_22: Value<()> =
        Rc::new(RefCell::new(if ((65536_() & ((1_()) << 41_())) != 0) {
            41_()
        } else {
            value_23.with(|rc| *rc.borrow())
        }));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__41_ {}
thread_local!(
    pub static value_23: Value<()> =
        Rc::new(RefCell::new(if ((65536_() & ((1_()) << 40_())) != 0) {
            40_()
        } else {
            value_24.with(|rc| *rc.borrow())
        }));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__40_ {}
thread_local!(
    pub static value_24: Value<()> =
        Rc::new(RefCell::new(if ((65536_() & ((1_()) << 39_())) != 0) {
            39_()
        } else {
            value_25.with(|rc| *rc.borrow())
        }));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__39_ {}
thread_local!(
    pub static value_25: Value<()> =
        Rc::new(RefCell::new(if ((65536_() & ((1_()) << 38_())) != 0) {
            38_()
        } else {
            value_26.with(|rc| *rc.borrow())
        }));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__38_ {}
thread_local!(
    pub static value_26: Value<()> =
        Rc::new(RefCell::new(if ((65536_() & ((1_()) << 37_())) != 0) {
            37_()
        } else {
            value_27.with(|rc| *rc.borrow())
        }));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__37_ {}
thread_local!(
    pub static value_27: Value<()> =
        Rc::new(RefCell::new(if ((65536_() & ((1_()) << 36_())) != 0) {
            36_()
        } else {
            value_28.with(|rc| *rc.borrow())
        }));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__36_ {}
thread_local!(
    pub static value_28: Value<()> =
        Rc::new(RefCell::new(if ((65536_() & ((1_()) << 35_())) != 0) {
            35_()
        } else {
            value_29.with(|rc| *rc.borrow())
        }));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__35_ {}
thread_local!(
    pub static value_29: Value<()> =
        Rc::new(RefCell::new(if ((65536_() & ((1_()) << 34_())) != 0) {
            34_()
        } else {
            value_30.with(|rc| *rc.borrow())
        }));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__34_ {}
thread_local!(
    pub static value_30: Value<()> =
        Rc::new(RefCell::new(if ((65536_() & ((1_()) << 33_())) != 0) {
            33_()
        } else {
            value_31.with(|rc| *rc.borrow())
        }));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__33_ {}
thread_local!(
    pub static value_31: Value<()> =
        Rc::new(RefCell::new(if ((65536_() & ((1_()) << 32_())) != 0) {
            32_()
        } else {
            value_32.with(|rc| *rc.borrow())
        }));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__32_ {}
thread_local!(
    pub static value_32: Value<()> =
        Rc::new(RefCell::new(if ((65536_() & ((1_()) << 31_())) != 0) {
            31_()
        } else {
            value_33.with(|rc| *rc.borrow())
        }));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__31_ {}
thread_local!(
    pub static value_33: Value<()> =
        Rc::new(RefCell::new(if ((65536_() & ((1_()) << 30_())) != 0) {
            30_()
        } else {
            value_34.with(|rc| *rc.borrow())
        }));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__30_ {}
thread_local!(
    pub static value_34: Value<()> =
        Rc::new(RefCell::new(if ((65536_() & ((1_()) << 29_())) != 0) {
            29_()
        } else {
            value_35.with(|rc| *rc.borrow())
        }));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__29_ {}
thread_local!(
    pub static value_35: Value<()> =
        Rc::new(RefCell::new(if ((65536_() & ((1_()) << 28_())) != 0) {
            28_()
        } else {
            value_36.with(|rc| *rc.borrow())
        }));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__28_ {}
thread_local!(
    pub static value_36: Value<()> =
        Rc::new(RefCell::new(if ((65536_() & ((1_()) << 27_())) != 0) {
            27_()
        } else {
            value_37.with(|rc| *rc.borrow())
        }));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__27_ {}
thread_local!(
    pub static value_37: Value<()> =
        Rc::new(RefCell::new(if ((65536_() & ((1_()) << 26_())) != 0) {
            26_()
        } else {
            value_38.with(|rc| *rc.borrow())
        }));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__26_ {}
thread_local!(
    pub static value_38: Value<()> =
        Rc::new(RefCell::new(if ((65536_() & ((1_()) << 25_())) != 0) {
            25_()
        } else {
            value_39.with(|rc| *rc.borrow())
        }));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__25_ {}
thread_local!(
    pub static value_39: Value<()> =
        Rc::new(RefCell::new(if ((65536_() & ((1_()) << 24_())) != 0) {
            24_()
        } else {
            value_40.with(|rc| *rc.borrow())
        }));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__24_ {}
thread_local!(
    pub static value_40: Value<()> =
        Rc::new(RefCell::new(if ((65536_() & ((1_()) << 23_())) != 0) {
            23_()
        } else {
            value_41.with(|rc| *rc.borrow())
        }));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__23_ {}
thread_local!(
    pub static value_41: Value<()> =
        Rc::new(RefCell::new(if ((65536_() & ((1_()) << 22_())) != 0) {
            22_()
        } else {
            value_42.with(|rc| *rc.borrow())
        }));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__22_ {}
thread_local!(
    pub static value_42: Value<()> =
        Rc::new(RefCell::new(if ((65536_() & ((1_()) << 21_())) != 0) {
            21_()
        } else {
            value_43.with(|rc| *rc.borrow())
        }));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__21_ {}
thread_local!(
    pub static value_43: Value<()> =
        Rc::new(RefCell::new(if ((65536_() & ((1_()) << 20_())) != 0) {
            20_()
        } else {
            value_44.with(|rc| *rc.borrow())
        }));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__20_ {}
thread_local!(
    pub static value_44: Value<()> =
        Rc::new(RefCell::new(if ((65536_() & ((1_()) << 19_())) != 0) {
            19_()
        } else {
            value_45.with(|rc| *rc.borrow())
        }));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__19_ {}
thread_local!(
    pub static value_45: Value<()> =
        Rc::new(RefCell::new(if ((65536_() & ((1_()) << 18_())) != 0) {
            18_()
        } else {
            value_46.with(|rc| *rc.borrow())
        }));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__18_ {}
thread_local!(
    pub static value_46: Value<()> =
        Rc::new(RefCell::new(if ((65536_() & ((1_()) << 17_())) != 0) {
            17_()
        } else {
            value_47.with(|rc| *rc.borrow())
        }));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__17_ {}
thread_local!(
    pub static value_47: Value<()> =
        Rc::new(RefCell::new(if ((65536_() & ((1_()) << 16_())) != 0) {
            16_()
        } else {
            value_48.with(|rc| *rc.borrow())
        }));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__16_ {}
thread_local!(
    pub static value_48: Value<()> =
        Rc::new(RefCell::new(if ((65536_() & ((1_()) << 15_())) != 0) {
            15_()
        } else {
            value_49.with(|rc| *rc.borrow())
        }));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__15_ {}
thread_local!(
    pub static value_49: Value<()> =
        Rc::new(RefCell::new(if ((65536_() & ((1_()) << 14_())) != 0) {
            14_()
        } else {
            value_50.with(|rc| *rc.borrow())
        }));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__14_ {}
thread_local!(
    pub static value_50: Value<()> =
        Rc::new(RefCell::new(if ((65536_() & ((1_()) << 13_())) != 0) {
            13_()
        } else {
            value_51.with(|rc| *rc.borrow())
        }));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__13_ {}
thread_local!(
    pub static value_51: Value<()> =
        Rc::new(RefCell::new(if ((65536_() & ((1_()) << 12_())) != 0) {
            12_()
        } else {
            value_52.with(|rc| *rc.borrow())
        }));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__12_ {}
thread_local!(
    pub static value_52: Value<()> =
        Rc::new(RefCell::new(if ((65536_() & ((1_()) << 11_())) != 0) {
            11_()
        } else {
            value_53.with(|rc| *rc.borrow())
        }));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__11_ {}
thread_local!(
    pub static value_53: Value<()> =
        Rc::new(RefCell::new(if ((65536_() & ((1_()) << 10_())) != 0) {
            10_()
        } else {
            value_54.with(|rc| *rc.borrow())
        }));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__10_ {}
thread_local!(
    pub static value_54: Value<()> =
        Rc::new(RefCell::new(if ((65536_() & ((1_()) << 9_())) != 0) {
            9_()
        } else {
            value_55.with(|rc| *rc.borrow())
        }));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__9_ {}
thread_local!(
    pub static value_55: Value<()> =
        Rc::new(RefCell::new(if ((65536_() & ((1_()) << 8_())) != 0) {
            8_()
        } else {
            value_56.with(|rc| *rc.borrow())
        }));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__8_ {}
thread_local!(
    pub static value_56: Value<()> =
        Rc::new(RefCell::new(if ((65536_() & ((1_()) << 7_())) != 0) {
            7_()
        } else {
            value_57.with(|rc| *rc.borrow())
        }));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__7_ {}
thread_local!(
    pub static value_57: Value<()> =
        Rc::new(RefCell::new(if ((65536_() & ((1_()) << 6_())) != 0) {
            6_()
        } else {
            value_58.with(|rc| *rc.borrow())
        }));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__6_ {}
thread_local!(
    pub static value_58: Value<()> =
        Rc::new(RefCell::new(if ((65536_() & ((1_()) << 5_())) != 0) {
            5_()
        } else {
            value_59.with(|rc| *rc.borrow())
        }));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__5_ {}
thread_local!(
    pub static value_59: Value<()> =
        Rc::new(RefCell::new(if ((65536_() & ((1_()) << 4_())) != 0) {
            4_()
        } else {
            value_60.with(|rc| *rc.borrow())
        }));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__4_ {}
thread_local!(
    pub static value_60: Value<()> =
        Rc::new(RefCell::new(if ((65536_() & ((1_()) << 3_())) != 0) {
            3_()
        } else {
            value_61.with(|rc| *rc.borrow())
        }));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__3_ {}
thread_local!(
    pub static value_61: Value<()> =
        Rc::new(RefCell::new(if ((65536_() & ((1_()) << 2_())) != 0) {
            2_()
        } else {
            value_62.with(|rc| *rc.borrow())
        }));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__2_ {}
thread_local!(
    pub static value_62: Value<()> =
        Rc::new(RefCell::new(if ((65536_() & ((1_()) << 1_())) != 0) {
            1_()
        } else {
            value_63.with(|rc| *rc.borrow())
        }));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__1_ {}
thread_local!(
    pub static value_63: Value<()> = Rc::new(RefCell::new(0_()));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__0_ {}
thread_local!(
    pub static value_64: Value<()> = Rc::new(RefCell::new(value_0.with(|rc| *rc.borrow())));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct support_detail_meta_log2_unsigned_long__65536_ {}
pub fn countl_zero_65(mut n: ()) -> () {
    return unimplemented!();
}
pub fn countl_zero_66(mut n: ()) -> () {
    return unimplemented!();
}
thread_local!(
    static rp_67: Value<()> = Rc::new(RefCell::new(
        ((({ support_simple_random_generator::max() }) as ())
            - (({ support_simple_random_generator::min() }) as ()))
            .wrapping_add((1_() as ())),
    ));
);
thread_local!(
    static rp_log2_68: Value<()> = Rc::new(RefCell::new(value_64.with(|rc| *rc.borrow())));
);
thread_local!(
    static w_digits_69: Value<()> = Rc::new(RefCell::new(digits_70.with(|rc| *rc.borrow())));
);
thread_local!(
    static e_digits_71: Value<()> = Rc::new(RefCell::new(digits_72.with(|rc| *rc.borrow())));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(64)]
pub struct support_detail_independent_bits_engine_support_simple_random_generator__unsigned_long_ {
    #[offset(0)]
    #[byte_size(8)]
    eng_: Ptr<support_simple_random_generator>,
    #[offset(8)]
    width_: (),
    #[offset(16)]
    wid0_: (),
    #[offset(24)]
    round_count_all_: (),
    #[offset(32)]
    round_count_regular_: (),
    #[offset(40)]
    y0_: (),
    #[offset(48)]
    y1_: (),
    #[offset(56)]
    mask0_: u16,
    #[offset(58)]
    mask1_: u16,
}
impl support_detail_independent_bits_engine_support_simple_random_generator__unsigned_long_ {
    pub fn new(eng: Ptr<support_simple_random_generator>, mut width: ()) -> Self {
        let __this: Value<
            support_detail_independent_bits_engine_support_simple_random_generator__unsigned_long_,
        > = Rc::new(RefCell::new(Self {
            eng_: (eng).clone(),
            width_: width,
            wid0_: unimplemented!(),
            round_count_all_: unimplemented!(),
            round_count_regular_: unimplemented!(),
            y0_: unimplemented!(),
            y1_: unimplemented!(),
            mask0_: 0_u16,
            mask1_: 0_u16,
        }));
        let this: Ptr<
            support_detail_independent_bits_engine_support_simple_random_generator__unsigned_long_,
        > = __this.as_pointer();
        field!(this, round_count_all_).write(
            ((this.with(|__s| __s.width_)).wrapping_div(rp_log2_68.with(|rc| *rc.borrow())))
                .wrapping_add(
                    ((this.with(|__s| __s.width_))
                        .wrapping_rem(rp_log2_68.with(|rc| *rc.borrow()))
                        != 0_()),
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
                field!(this, y0_).write(0_());
            }
        }
        if ((rp_67.with(|rc| *rc.borrow())).wrapping_sub(this.with(|__s| __s.y0_))
            > (this.with(|__s| __s.y0_)).wrapping_div(this.with(|__s| __s.round_count_all_)))
        {
            field!(this, round_count_all_).with_mut(|__v| __v.prefix_inc());
            field!(this, wid0_).write(
                (this.with(|__s| __s.width_)).wrapping_div(this.with(|__s| __s.round_count_all_)),
            );
            if (this.with(|__s| __s.wid0_) < w_digits_69.with(|rc| *rc.borrow())) {
                field!(this, y0_).write(
                    ((rp_67.with(|rc| *rc.borrow()) >> this.with(|__s| __s.wid0_))
                        << this.with(|__s| __s.wid0_)),
                );
            } else {
                field!(this, y0_).write(0_());
            }
        }
        field!(this, round_count_regular_).write(
            (this.with(|__s| __s.round_count_all_)).wrapping_sub(
                (this.with(|__s| __s.width_)).wrapping_rem(this.with(|__s| __s.round_count_all_)),
            ),
        );
        if (this.with(|__s| __s.wid0_) < (w_digits_69.with(|rc| *rc.borrow())).wrapping_sub(1_())) {
            field!(this, y1_).write(
                ((rp_67.with(|rc| *rc.borrow())
                    >> ((this.with(|__s| __s.wid0_)).wrapping_add(1_())))
                    << ((this.with(|__s| __s.wid0_)).wrapping_add(1_()))),
            );
        } else {
            field!(this, y1_).write(0_());
        }
        field!(this, mask0_).write(if (this.with(|__s| __s.wid0_) > 0_()) {
            ((((!0_() as u16) as ())
                >> ((e_digits_71.with(|rc| *rc.borrow())).wrapping_sub(this.with(|__s| __s.wid0_))))
                as u16)
        } else {
            0_u16
        });
        field!(this, mask1_).write(
            if (this.with(|__s| __s.wid0_)
                < (e_digits_71.with(|rc| *rc.borrow())).wrapping_sub(1_()))
            {
                ((((!0_() as u16) as ())
                    >> ((e_digits_71.with(|rc| *rc.borrow()))
                        .wrapping_sub(((this.with(|__s| __s.wid0_)).wrapping_add(1_())))))
                    as u16)
            } else {
                (!0_() as u16)
            },
        );
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
pub fn operator_eq_73(
    _a0: Ptr<support_uniform_int_distribution_long_>,
    _a1: Ptr<support_uniform_int_distribution_long_>,
) -> () {
    return ({
        let _arg0: Ptr<support_uniform_int_distribution_long__param_type> = field_ptr!(_a0, param_);
        let _arg1: Ptr<support_uniform_int_distribution_long__param_type> = field_ptr!(_a1, param_);
        operator_eq_74(_arg0, _arg1)
    });
}
pub fn operator_eq_74(
    _a0: Ptr<support_uniform_int_distribution_long__param_type>,
    _a1: Ptr<support_uniform_int_distribution_long__param_type>,
) -> () {
    return ({ _a0.with(|__s| __s.a_) } == { _a1.with(|__s| __s.a_) })
        && ({ _a0.with(|__s| __s.b_) } == { _a1.with(|__s| __s.b_) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(16)]
pub struct support_uniform_int_distribution_long__param_type {
    #[offset(0)]
    a_: (),
    #[offset(8)]
    b_: (),
}
impl support_uniform_int_distribution_long__param_type {
    pub fn new_1() -> Self {
        support_uniform_int_distribution_long__param_type::new_2({ 0_() }, None)
    }
    pub fn new_2(mut ax: (), bx: Option<()>) -> Self {
        let mut bx: () = bx.unwrap_or_else(|| ({ std_numeric_limits_long_::max() }));
        Self { a_: ax, b_: bx }
    }
}
impl std::cmp::PartialEq for support_uniform_int_distribution_long__param_type {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_74(
                Rc::new(RefCell::new(
                    support_uniform_int_distribution_long__param_type {
                        a_: self.a_.clone(),
                        b_: self.b_.clone(),
                    },
                ))
                .as_pointer(),
                Rc::new(RefCell::new(
                    support_uniform_int_distribution_long__param_type {
                        a_: other.a_.clone(),
                        b_: other.b_.clone(),
                    },
                ))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for support_uniform_int_distribution_long__param_type {}
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
impl std::cmp::PartialEq for support_uniform_int_distribution_long_ {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_73(
                Rc::new(RefCell::new(support_uniform_int_distribution_long_ {
                    param_: support_uniform_int_distribution_long__param_type {
                        a_: self.param_.a_.clone(),
                        b_: self.param_.b_.clone(),
                    },
                }))
                .as_pointer(),
                Rc::new(RefCell::new(support_uniform_int_distribution_long_ {
                    param_: support_uniform_int_distribution_long__param_type {
                        a_: other.param_.a_.clone(),
                        b_: other.param_.b_.clone(),
                    },
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for support_uniform_int_distribution_long_ {}
thread_local!(
    pub static default_seed_75: Value<u16> = Rc::new(RefCell::new(19937_u16));
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
        return (-1_() as u16);
    }
    pub fn new_1() -> Self {
        support_simple_random_generator::new_2({ default_seed_75.with(|rc| *rc.borrow()) })
    }
    pub fn new_2(mut s: u16) -> Self {
        Self {
            status_: (s as u32),
        }
    }
}
impl Default for support_simple_random_generator {
    fn default() -> Self {
        { support_simple_random_generator::new_1() }
    }
}
pub fn shuffle_76(
    mut first: Ptr<()>,
    mut last: Ptr<()>,
    gen_: Ptr<support_simple_random_generator>,
) {
    let mut last_iter: Ptr<()> = (last).clone();
    let mut diff: () = (((last_iter).clone() - (first).clone()) as () as ());
    if (diff > 1_()) {
        let uid: Value<support_uniform_int_distribution_long_> = Rc::new(RefCell::new(
            <support_uniform_int_distribution_long_>::default(),
        ));
        {
            last_iter.prefix_dec();
            &(diff.prefix_dec())
        };
        'loop_: while ({ (first).clone() } < { (last).clone() }) {
            let mut index: () = ({
                let _gen_: Ptr<support_simple_random_generator> = (gen_).clone();
                let _param: Value<support_uniform_int_distribution_long__param_type> = Rc::new(
                    RefCell::new(support_uniform_int_distribution_long__param_type::new_2(
                        { 0_() },
                        { Some(diff) },
                    )),
                );
                support_uniform_int_distribution_long_Impl :: operator_call_6_class_support_simple_random_generator ( &uid .as_pointer()  , _gen_ , _param.as_pointer() , )
            });
            if (index != 0_()) {
                unimplemented!();
            }
            {
                first.prefix_inc();
                &(diff.prefix_dec())
            };
        }
    }
}
pub fn shuffle_77(
    mut first: Ptr<()>,
    mut last: Ptr<()>,
    gen_: Ptr<support_simple_random_generator>,
) {
    let mut last_iter: Ptr<()> = (last).clone();
    let mut diff: () = (((last_iter).clone() - (first).clone()) as () as ());
    if (diff > 1_()) {
        let uid: Value<support_uniform_int_distribution_long_> = Rc::new(RefCell::new(
            <support_uniform_int_distribution_long_>::default(),
        ));
        {
            last_iter.prefix_dec();
            &(diff.prefix_dec())
        };
        'loop_: while ({ (first).clone() } < { (last).clone() }) {
            let mut index: () = ({
                let _gen_: Ptr<support_simple_random_generator> = (gen_).clone();
                let _param: Value<support_uniform_int_distribution_long__param_type> = Rc::new(
                    RefCell::new(support_uniform_int_distribution_long__param_type::new_2(
                        { 0_() },
                        { Some(diff) },
                    )),
                );
                support_uniform_int_distribution_long_Impl :: operator_call_6_class_support_simple_random_generator ( &uid .as_pointer()  , _gen_ , _param.as_pointer() , )
            });
            if (index != 0_()) {
                unimplemented!();
            }
            {
                first.prefix_inc();
                &(diff.prefix_dec())
            };
        }
    }
}
pub fn shuffle_78(mut first: Ptr<S>, mut last: Ptr<S>, gen_: Ptr<support_simple_random_generator>) {
    let mut last_iter: Ptr<S> = (last).clone();
    let mut diff: () = (((last_iter).clone() - (first).clone()) as () as ());
    if (diff > 1_()) {
        let uid: Value<support_uniform_int_distribution_long_> = Rc::new(RefCell::new(
            <support_uniform_int_distribution_long_>::default(),
        ));
        {
            last_iter.prefix_dec();
            &(diff.prefix_dec())
        };
        'loop_: while ({ (first).clone() } < { (last).clone() }) {
            let mut index: () = ({
                let _gen_: Ptr<support_simple_random_generator> = (gen_).clone();
                let _param: Value<support_uniform_int_distribution_long__param_type> = Rc::new(
                    RefCell::new(support_uniform_int_distribution_long__param_type::new_2(
                        { 0_() },
                        { Some(diff) },
                    )),
                );
                support_uniform_int_distribution_long_Impl :: operator_call_6_class_support_simple_random_generator ( &uid .as_pointer()  , _gen_ , _param.as_pointer() , )
            });
            if (index != 0_()) {
                unimplemented!();
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
#[derive(Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, CopyAssign, MoveAssign)]
#[byte_size(4)]
pub struct S {
    #[offset(0)]
    pub i_: (),
}
impl S {
    pub fn new_1() -> Self {
        Self { i_: 0_() }
    }
    pub fn new_2(mut i: ()) -> Self {
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
        field!(rhs, i_).write(-1_());
        __this
    }
}
impl std::cmp::Ord for S {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            if SImpl::operator_lt(
                &Rc::new(RefCell::new(S {
                    i_: self.i_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(S {
                    i_: other.i_.clone(),
                }))
                .as_pointer(),
            ) {
                std::cmp::Ordering::Less
            } else if SImpl::operator_lt(
                &Rc::new(RefCell::new(S {
                    i_: other.i_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(S {
                    i_: self.i_.clone(),
                }))
                .as_pointer(),
            ) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for S {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for S {
    fn eq(&self, other: &Self) -> bool {
        {
            SImpl::operator_eq_10(
                &Rc::new(RefCell::new(S {
                    i_: self.i_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(S {
                    i_: other.i_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for S {}
impl Clone for S {
    fn clone(&self) -> Self {
        let __src: Value<S> = Rc::new(RefCell::new(S {
            i_: self.i_.clone(),
        }));
        S::copy_from(__src.as_pointer())
    }
}
impl Default for S {
    fn default() -> Self {
        { S::new_1() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign)]
#[byte_size(1)]
pub struct support_double_move_tracker {
    #[offset(0)]
    moved_from_: (),
}
impl support_double_move_tracker {
    pub fn new() -> Self {
        Self { moved_from_: false }
    }
    pub fn move_from(other: Ptr<support_double_move_tracker>) -> Self {
        let __this: support_double_move_tracker = Self { moved_from_: false };
        unimplemented!();
        field!(other, moved_from_).write(true);
        __this
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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128___types_type_list_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__
{}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128___types_type_list_bool__
{}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__bool___types_type_list_float__double__long_double__
{}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct cpp17_output_iterator_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<()>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl cpp17_output_iterator_int_ptr_ {}
const _: () = assert!(
    true,
    "std::output_iterator<cpp17_output_iterator<int*>, int>"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct cpp17_input_iterator_int_ptr__int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<()>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl cpp17_input_iterator_int_ptr__int_ptr_ {}
const _: () = assert!(true, "std::input_iterator<cpp17_input_iterator<int*>>");
pub fn operator_eq_79(
    x: Ptr<bidirectional_iterator_int_ptr_>,
    y: Ptr<bidirectional_iterator_int_ptr_>,
) -> () {
    return ({ x.with(|__s| __s.it_.clone()) } == { y.with(|__s| __s.it_.clone()) });
}
pub fn operator_ne_80(
    x: Ptr<bidirectional_iterator_int_ptr_>,
    y: Ptr<bidirectional_iterator_int_ptr_>,
) -> () {
    return ({ x.with(|__s| __s.it_.clone()) } != { y.with(|__s| __s.it_.clone()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign)]
#[byte_size(16)]
pub struct bidirectional_iterator_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<()>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl bidirectional_iterator_int_ptr_ {
    pub fn new_1() -> Self {
        Self {
            it_: Ptr::<()>::null(),
            tracker_: support_double_move_tracker::new(),
        }
    }
    pub fn new_2(mut it: Ptr<()>) -> Self {
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
            operator_eq_79(
                Rc::new(RefCell::new(bidirectional_iterator_int_ptr_ {
                    it_: self.it_.clone(),
                    tracker_: support_double_move_tracker {
                        moved_from_: self.tracker_.moved_from_.clone(),
                    },
                }))
                .as_pointer(),
                Rc::new(RefCell::new(bidirectional_iterator_int_ptr_ {
                    it_: other.it_.clone(),
                    tracker_: support_double_move_tracker {
                        moved_from_: other.tracker_.moved_from_.clone(),
                    },
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for bidirectional_iterator_int_ptr_ {}
impl Default for bidirectional_iterator_int_ptr_ {
    fn default() -> Self {
        { bidirectional_iterator_int_ptr_::new_1() }
    }
}
pub fn operator_eq_81(
    x: Ptr<bidirectional_iterator_S_ptr_>,
    y: Ptr<bidirectional_iterator_S_ptr_>,
) -> () {
    return ({ x.with(|__s| __s.it_.clone()) } == { y.with(|__s| __s.it_.clone()) });
}
pub fn operator_ne_82(
    x: Ptr<bidirectional_iterator_S_ptr_>,
    y: Ptr<bidirectional_iterator_S_ptr_>,
) -> () {
    return ({ x.with(|__s| __s.it_.clone()) } != { y.with(|__s| __s.it_.clone()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign)]
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
    pub fn new_1() -> Self {
        Self {
            it_: Ptr::<S>::null(),
            tracker_: support_double_move_tracker::new(),
        }
    }
    pub fn new_2(mut it: Ptr<S>) -> Self {
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
            operator_eq_81(
                Rc::new(RefCell::new(bidirectional_iterator_S_ptr_ {
                    it_: self.it_.clone(),
                    tracker_: support_double_move_tracker {
                        moved_from_: self.tracker_.moved_from_.clone(),
                    },
                }))
                .as_pointer(),
                Rc::new(RefCell::new(bidirectional_iterator_S_ptr_ {
                    it_: other.it_.clone(),
                    tracker_: support_double_move_tracker {
                        moved_from_: other.tracker_.moved_from_.clone(),
                    },
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for bidirectional_iterator_S_ptr_ {}
impl Default for bidirectional_iterator_S_ptr_ {
    fn default() -> Self {
        { bidirectional_iterator_S_ptr_::new_1() }
    }
}
pub fn operator_add_83(
    x: random_access_iterator_int_ptr_,
    mut n: (),
) -> random_access_iterator_int_ptr_ {
    let x: Value<random_access_iterator_int_ptr_> = Rc::new(RefCell::new(x));
    ({ random_access_iterator_int_ptr_Impl::operator_add_assign(&x.as_pointer(), n) });
    return random_access_iterator_int_ptr_::move_from({ x.as_pointer() });
}
pub fn operator_sub_84(
    mut x: random_access_iterator_int_ptr_,
    mut y: random_access_iterator_int_ptr_,
) -> () {
    return (((x.it_).clone() - (y.it_).clone()) as () as ());
}
pub fn operator_eq_85(
    x: Ptr<random_access_iterator_int_ptr_>,
    y: Ptr<random_access_iterator_int_ptr_>,
) -> () {
    return ({ x.with(|__s| __s.it_.clone()) } == { y.with(|__s| __s.it_.clone()) });
}
pub fn operator_ne_86(
    x: Ptr<random_access_iterator_int_ptr_>,
    y: Ptr<random_access_iterator_int_ptr_>,
) -> () {
    return ({ x.with(|__s| __s.it_.clone()) } != { y.with(|__s| __s.it_.clone()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign)]
#[byte_size(16)]
pub struct random_access_iterator_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<()>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl random_access_iterator_int_ptr_ {
    pub fn new_1() -> Self {
        Self {
            it_: Ptr::<()>::null(),
            tracker_: support_double_move_tracker::new(),
        }
    }
    pub fn new_2(mut it: Ptr<()>) -> Self {
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
            operator_eq_85(
                Rc::new(RefCell::new(random_access_iterator_int_ptr_ {
                    it_: self.it_.clone(),
                    tracker_: support_double_move_tracker {
                        moved_from_: self.tracker_.moved_from_.clone(),
                    },
                }))
                .as_pointer(),
                Rc::new(RefCell::new(random_access_iterator_int_ptr_ {
                    it_: other.it_.clone(),
                    tracker_: support_double_move_tracker {
                        moved_from_: other.tracker_.moved_from_.clone(),
                    },
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for random_access_iterator_int_ptr_ {}
impl Default for random_access_iterator_int_ptr_ {
    fn default() -> Self {
        { random_access_iterator_int_ptr_::new_1() }
    }
}
pub fn operator_add_87(
    x: random_access_iterator_S_ptr_,
    mut n: (),
) -> random_access_iterator_S_ptr_ {
    let x: Value<random_access_iterator_S_ptr_> = Rc::new(RefCell::new(x));
    ({ random_access_iterator_S_ptr_Impl::operator_add_assign(&x.as_pointer(), n) });
    return random_access_iterator_S_ptr_::move_from({ x.as_pointer() });
}
pub fn operator_sub_88(
    mut x: random_access_iterator_S_ptr_,
    mut y: random_access_iterator_S_ptr_,
) -> () {
    return (((x.it_).clone() - (y.it_).clone()) as () as ());
}
pub fn operator_eq_89(
    x: Ptr<random_access_iterator_S_ptr_>,
    y: Ptr<random_access_iterator_S_ptr_>,
) -> () {
    return ({ x.with(|__s| __s.it_.clone()) } == { y.with(|__s| __s.it_.clone()) });
}
pub fn operator_ne_90(
    x: Ptr<random_access_iterator_S_ptr_>,
    y: Ptr<random_access_iterator_S_ptr_>,
) -> () {
    return ({ x.with(|__s| __s.it_.clone()) } != { y.with(|__s| __s.it_.clone()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign)]
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
    pub fn new_1() -> Self {
        Self {
            it_: Ptr::<S>::null(),
            tracker_: support_double_move_tracker::new(),
        }
    }
    pub fn new_2(mut it: Ptr<S>) -> Self {
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
            operator_eq_89(
                Rc::new(RefCell::new(random_access_iterator_S_ptr_ {
                    it_: self.it_.clone(),
                    tracker_: support_double_move_tracker {
                        moved_from_: self.tracker_.moved_from_.clone(),
                    },
                }))
                .as_pointer(),
                Rc::new(RefCell::new(random_access_iterator_S_ptr_ {
                    it_: other.it_.clone(),
                    tracker_: support_double_move_tracker {
                        moved_from_: other.tracker_.moved_from_.clone(),
                    },
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for random_access_iterator_S_ptr_ {}
impl Default for random_access_iterator_S_ptr_ {
    fn default() -> Self {
        { random_access_iterator_S_ptr_::new_1() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct cpp20_random_access_iterator_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<()>,
    #[offset(8)]
    #[byte_size(1)]
    tracker_: support_double_move_tracker,
}
impl cpp20_random_access_iterator_int_ptr_ {}
const _: () = assert!(
    true,
    "std::random_access_iterator<cpp20_random_access_iterator<int*>>"
);
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct cpp20_input_iterator_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<()>,
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
const _: () = assert!(true, "std::input_iterator<cpp20_input_iterator<int*>>");
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct cpp20_output_iterator_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<()>,
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
const _: () = assert!(
    true,
    "std::output_iterator<cpp20_output_iterator<int*>, int>"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(32)]
pub struct IteratorOpCounts {
    #[offset(0)]
    pub increments: (),
    #[offset(8)]
    pub decrements: (),
    #[offset(16)]
    pub zero_moves: (),
    #[offset(24)]
    pub equal_cmps: (),
}
impl Default for IteratorOpCounts {
    fn default() -> Self {
        {
            IteratorOpCounts {
                increments: 0_(),
                decrements: 0_(),
                zero_moves: 0_(),
                equal_cmps: 0_(),
            }
        }
    }
}
pub fn operator_eq_91(
    _a0: Ptr<forward_sized_iterator_int_ptr_>,
    _a1: Ptr<forward_sized_iterator_int_ptr_>,
) -> () {
    return ({ _a0.with(|__s| __s.it_.clone()) } == { _a1.with(|__s| __s.it_.clone()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct forward_sized_iterator_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    pub it_: Ptr<()>,
}
impl forward_sized_iterator_int_ptr_ {}
impl std::cmp::PartialEq for forward_sized_iterator_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_91(
                Rc::new(RefCell::new(forward_sized_iterator_int_ptr_ {
                    it_: self.it_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(forward_sized_iterator_int_ptr_ {
                    it_: other.it_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for forward_sized_iterator_int_ptr_ {}
impl Default for forward_sized_iterator_int_ptr_ {
    fn default() -> Self {
        {
            forward_sized_iterator_int_ptr_ {
                it_: Ptr::<()>::null(),
            }
        }
    }
}
const _: () = assert!(true, "std::forward_iterator<forward_sized_iterator<>>");
const _: () = assert!(
    true,
    "std::sized_sentinel_for<forward_sized_iterator<>, forward_sized_iterator<>>"
);
pub fn operator_add_92(mut i: adl_Iterator, mut n: ()) -> adl_Iterator {
    return adl_Iterator::new_1(
        { i.ptr_.offset((n) as isize) },
        { (i.iter_moves_).clone() },
        { (i.iter_swaps_).clone() },
    );
}
pub fn operator_add_93(mut n: (), mut i: adl_Iterator) -> adl_Iterator {
    return ({
        let _i: adl_Iterator = i;
        operator_add_92(_i, n)
    });
}
pub fn iter_swap_94(mut a: adl_Iterator, mut b: adl_Iterator) {
    unimplemented!();
    if !(a.iter_swaps_).is_null() {
        a.iter_swaps_.with_mut(|__v| __v.prefix_inc());
    }
}
pub fn iter_move_95(iter: adl_Iterator) -> Ptr<()> {
    let iter: Value<adl_Iterator> = Rc::new(RefCell::new(iter));
    if !({ (*iter.borrow()).iter_moves_.clone() }).is_null() {
        { (*iter.borrow()).iter_moves_.clone() }.with_mut(|__v| __v.prefix_inc());
    }
    return ({ adl_IteratorImpl::operator_deref(&iter.as_pointer()) }).reinterpret_cast::<()>();
}
pub fn operator_eq_96(lhs: Ptr<adl_Iterator>, rhs: Ptr<adl_Iterator>) -> () {
    return ({ lhs.with(|__s| __s.ptr_.clone()) } == { rhs.with(|__s| __s.ptr_.clone()) });
}
pub fn operator_cmp_97(lhs: Ptr<adl_Iterator>, rhs: Ptr<adl_Iterator>) -> () {
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
    ptr_: Ptr<()>,
    #[offset(8)]
    #[byte_size(8)]
    iter_moves_: Ptr<()>,
    #[offset(16)]
    #[byte_size(8)]
    iter_swaps_: Ptr<()>,
}
impl adl_Iterator {
    fn new_1(mut p: Ptr<()>, mut iter_moves: Ptr<()>, mut iter_swaps: Ptr<()>) -> Self {
        Self {
            ptr_: (p).clone(),
            iter_moves_: (iter_moves).clone(),
            iter_swaps_: (iter_swaps).clone(),
        }
    }
    pub fn TrackMoves(mut p: Ptr<()>, iter_moves: Ptr<()>) -> adl_Iterator {
        return adl_Iterator::new_1({ (p).clone() }, { (iter_moves).clone() }, {
            Ptr::<()>::null()
        });
    }
    pub fn TrackSwaps_4(iter_swaps: Ptr<()>) -> adl_Iterator {
        return adl_Iterator::new_1({ Ptr::<()>::null() }, { Ptr::<()>::null() }, {
            (iter_swaps).clone()
        });
    }
    pub fn TrackSwaps_5(mut p: Ptr<()>, iter_swaps: Ptr<()>) -> adl_Iterator {
        return adl_Iterator::new_1({ (p).clone() }, { Ptr::<()>::null() }, {
            (iter_swaps).clone()
        });
    }
}
impl std::cmp::Ord for adl_Iterator {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            operator_cmp_97(
                Rc::new(RefCell::new(adl_Iterator {
                    ptr_: self.ptr_.clone(),
                    iter_moves_: self.iter_moves_.clone(),
                    iter_swaps_: self.iter_swaps_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(adl_Iterator {
                    ptr_: other.ptr_.clone(),
                    iter_moves_: other.iter_moves_.clone(),
                    iter_swaps_: other.iter_swaps_.clone(),
                }))
                .as_pointer(),
            )
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
            operator_eq_96(
                Rc::new(RefCell::new(adl_Iterator {
                    ptr_: self.ptr_.clone(),
                    iter_moves_: self.iter_moves_.clone(),
                    iter_swaps_: self.iter_swaps_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(adl_Iterator {
                    ptr_: other.ptr_.clone(),
                    iter_moves_: other.iter_moves_.clone(),
                    iter_swaps_: other.iter_swaps_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for adl_Iterator {}
impl Default for adl_Iterator {
    fn default() -> Self {
        {
            adl_Iterator {
                ptr_: Ptr::<()>::null(),
                iter_moves_: Ptr::<()>::null(),
                iter_swaps_: Ptr::<()>::null(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct rvalue_iterator_int_ptr_ {
    #[offset(0)]
    #[byte_size(8)]
    it_: Ptr<Ptr<()>>,
}
impl rvalue_iterator_int_ptr_ {}
impl std::cmp::Ord for rvalue_iterator_int_ptr_ {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            rvalue_iterator_int_ptr_Impl::operator_cmp(
                &Rc::new(RefCell::new(rvalue_iterator_int_ptr_ {
                    it_: self.it_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(rvalue_iterator_int_ptr_ {
                    it_: other.it_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::PartialOrd for rvalue_iterator_int_ptr_ {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for rvalue_iterator_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        {
            rvalue_iterator_int_ptr_Impl::operator_cmp(
                &Rc::new(RefCell::new(rvalue_iterator_int_ptr_ {
                    it_: self.it_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(rvalue_iterator_int_ptr_ {
                    it_: other.it_.clone(),
                }))
                .as_pointer(),
            ) == std::cmp::Ordering::Equal
        }
    }
}
impl std::cmp::Eq for rvalue_iterator_int_ptr_ {}
const _: () = assert!(true, "std::random_access_iterator<rvalue_iterator<int*>>");
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct util_iterator_error {}
impl util_iterator_error {
    pub fn new(mut what: Ptr<()>) -> Self {
        Self {}
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(16)]
pub struct binary_counting_predicate_std_greater_int___int__int_ {
    #[offset(0)]
    #[byte_size(1)]
    p_: (),
    #[offset(8)]
    count_: (),
}
impl binary_counting_predicate_std_greater_int___int__int_ {
    pub fn new(p: ()) -> Self {
        let p: Value<()> = Rc::new(RefCell::new(p));
        Self {
            p_: (*p.borrow()).clone(),
            count_: 0_(),
        }
    }
}
impl Default for binary_counting_predicate_std_greater_int___int__int_ {
    fn default() -> Self {
        {
            binary_counting_predicate_std_greater_int___int__int_ {
                p_: unimplemented!(),
                count_: unimplemented!(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(16)]
pub struct binary_counting_predicate_std_greater_S___S__S_ {
    #[offset(0)]
    #[byte_size(1)]
    p_: (),
    #[offset(8)]
    count_: (),
}
impl binary_counting_predicate_std_greater_S___S__S_ {
    pub fn new(p: ()) -> Self {
        let p: Value<()> = Rc::new(RefCell::new(p));
        Self {
            p_: (*p.borrow()).clone(),
            count_: 0_(),
        }
    }
}
impl Default for binary_counting_predicate_std_greater_S___S__S_ {
    fn default() -> Self {
        {
            binary_counting_predicate_std_greater_S___S__S_ {
                p_: unimplemented!(),
                count_: unimplemented!(),
            }
        }
    }
}
pub fn test_one_98(mut N: (), mut M: (), randomness: Ptr<support_simple_random_generator>) {
    unimplemented!();
    let mut ia: Ptr<()> = Ptr::alloc_array((0..N).map(|_| unimplemented!()).collect::<Box<[()]>>());
    let mut i: () = 0_();
    'loop_: while (i < N) {
        elem!(ia, i).write({ i });
        i.prefix_inc();
    }
    ({
        let _first: Ptr<()> = (ia).clone();
        let _last: Ptr<()> = ia.offset((N) as isize);
        let _gen_: Ptr<support_simple_random_generator> = (randomness).clone();
        shuffle_77(_first, _last, _gen_)
    });
    unimplemented!();
    unimplemented!();
    let pred: Value<binary_counting_predicate_std_greater_int___int__int_> = Rc::new(RefCell::new(
        binary_counting_predicate_std_greater_int___int__int_::new({ (unimplemented!()) }),
    ));
    unimplemented!();
    if (N > 0_()) {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    ia.delete();
}
pub fn test_one_99(mut N: (), mut M: (), randomness: Ptr<support_simple_random_generator>) {
    unimplemented!();
    let mut ia: Ptr<()> = Ptr::alloc_array((0..N).map(|_| unimplemented!()).collect::<Box<[()]>>());
    let mut i: () = 0_();
    'loop_: while (i < N) {
        elem!(ia, i).write({ i });
        i.prefix_inc();
    }
    ({
        let _first: Ptr<()> = (ia).clone();
        let _last: Ptr<()> = ia.offset((N) as isize);
        let _gen_: Ptr<support_simple_random_generator> = (randomness).clone();
        shuffle_77(_first, _last, _gen_)
    });
    unimplemented!();
    unimplemented!();
    let pred: Value<binary_counting_predicate_std_greater_int___int__int_> = Rc::new(RefCell::new(
        binary_counting_predicate_std_greater_int___int__int_::new({ (unimplemented!()) }),
    ));
    unimplemented!();
    if (N > 0_()) {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    ia.delete();
}
pub fn test_one_100(mut N: (), mut M: (), randomness: Ptr<support_simple_random_generator>) {
    unimplemented!();
    let mut ia: Ptr<()> = Ptr::alloc_array((0..N).map(|_| unimplemented!()).collect::<Box<[()]>>());
    let mut i: () = 0_();
    'loop_: while (i < N) {
        elem!(ia, i).write({ i });
        i.prefix_inc();
    }
    ({
        let _first: Ptr<()> = (ia).clone();
        let _last: Ptr<()> = ia.offset((N) as isize);
        let _gen_: Ptr<support_simple_random_generator> = (randomness).clone();
        shuffle_77(_first, _last, _gen_)
    });
    unimplemented!();
    unimplemented!();
    let pred: Value<binary_counting_predicate_std_greater_int___int__int_> = Rc::new(RefCell::new(
        binary_counting_predicate_std_greater_int___int__int_::new({ (unimplemented!()) }),
    ));
    unimplemented!();
    if (N > 0_()) {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    ia.delete();
}
pub fn test_one_101(mut N: (), mut M: (), randomness: Ptr<support_simple_random_generator>) {
    unimplemented!();
    let mut ia: Ptr<S> = Ptr::alloc_array((0..N).map(|_| <S>::default()).collect::<Box<[S]>>());
    let mut i: () = 0_();
    'loop_: while (i < N) {
        ({
            let _i: () = i;
            SImpl::operator_assign_7(&ia.offset((i) as isize), _i)
        });
        i.prefix_inc();
    }
    ({
        let _first: Ptr<S> = (ia).clone();
        let _last: Ptr<S> = ia.offset((N) as isize);
        let _gen_: Ptr<support_simple_random_generator> = (randomness).clone();
        shuffle_78(_first, _last, _gen_)
    });
    unimplemented!();
    unimplemented!();
    let pred: Value<binary_counting_predicate_std_greater_S___S__S_> = Rc::new(RefCell::new(
        binary_counting_predicate_std_greater_S___S__S_::new({ (unimplemented!()) }),
    ));
    unimplemented!();
    if (N > 0_()) {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    ia.delete();
}
pub fn test_one_102(mut N: (), mut M: (), randomness: Ptr<support_simple_random_generator>) {
    unimplemented!();
    let mut ia: Ptr<S> = Ptr::alloc_array((0..N).map(|_| <S>::default()).collect::<Box<[S]>>());
    let mut i: () = 0_();
    'loop_: while (i < N) {
        ({
            let _i: () = i;
            SImpl::operator_assign_7(&ia.offset((i) as isize), _i)
        });
        i.prefix_inc();
    }
    ({
        let _first: Ptr<S> = (ia).clone();
        let _last: Ptr<S> = ia.offset((N) as isize);
        let _gen_: Ptr<support_simple_random_generator> = (randomness).clone();
        shuffle_78(_first, _last, _gen_)
    });
    unimplemented!();
    unimplemented!();
    let pred: Value<binary_counting_predicate_std_greater_S___S__S_> = Rc::new(RefCell::new(
        binary_counting_predicate_std_greater_S___S__S_::new({ (unimplemented!()) }),
    ));
    unimplemented!();
    if (N > 0_()) {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    ia.delete();
}
pub fn test_one_103(mut N: (), mut M: (), randomness: Ptr<support_simple_random_generator>) {
    unimplemented!();
    let mut ia: Ptr<S> = Ptr::alloc_array((0..N).map(|_| <S>::default()).collect::<Box<[S]>>());
    let mut i: () = 0_();
    'loop_: while (i < N) {
        ({
            let _i: () = i;
            SImpl::operator_assign_7(&ia.offset((i) as isize), _i)
        });
        i.prefix_inc();
    }
    ({
        let _first: Ptr<S> = (ia).clone();
        let _last: Ptr<S> = ia.offset((N) as isize);
        let _gen_: Ptr<support_simple_random_generator> = (randomness).clone();
        shuffle_78(_first, _last, _gen_)
    });
    unimplemented!();
    unimplemented!();
    let pred: Value<binary_counting_predicate_std_greater_S___S__S_> = Rc::new(RefCell::new(
        binary_counting_predicate_std_greater_S___S__S_::new({ (unimplemented!()) }),
    ));
    unimplemented!();
    if (N > 0_()) {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    ia.delete();
}
pub fn test_104(mut N: (), randomness: Ptr<support_simple_random_generator>) {
    ({
        let _N: () = N;
        let _randomness: Ptr<support_simple_random_generator> = (randomness).clone();
        test_one_98(_N, 0_(), _randomness)
    });
    ({
        let _N: () = N;
        let _M: () = (N).wrapping_div(4_());
        let _randomness: Ptr<support_simple_random_generator> = (randomness).clone();
        test_one_98(_N, _M, _randomness)
    });
    ({
        let _N: () = N;
        let _M: () = (N).wrapping_div(2_());
        let _randomness: Ptr<support_simple_random_generator> = (randomness).clone();
        test_one_98(_N, _M, _randomness)
    });
    ({
        let _N: () = N;
        let _M: () = ((3_()).wrapping_mul(N)).wrapping_div(4_());
        let _randomness: Ptr<support_simple_random_generator> = (randomness).clone();
        test_one_98(_N, _M, _randomness)
    });
    ({
        let _N: () = N;
        let _M: () = N;
        let _randomness: Ptr<support_simple_random_generator> = (randomness).clone();
        test_one_98(_N, _M, _randomness)
    });
}
pub fn test_105(mut N: (), randomness: Ptr<support_simple_random_generator>) {
    ({
        let _N: () = N;
        let _randomness: Ptr<support_simple_random_generator> = (randomness).clone();
        test_one_99(_N, 0_(), _randomness)
    });
    ({
        let _N: () = N;
        let _M: () = (N).wrapping_div(4_());
        let _randomness: Ptr<support_simple_random_generator> = (randomness).clone();
        test_one_99(_N, _M, _randomness)
    });
    ({
        let _N: () = N;
        let _M: () = (N).wrapping_div(2_());
        let _randomness: Ptr<support_simple_random_generator> = (randomness).clone();
        test_one_99(_N, _M, _randomness)
    });
    ({
        let _N: () = N;
        let _M: () = ((3_()).wrapping_mul(N)).wrapping_div(4_());
        let _randomness: Ptr<support_simple_random_generator> = (randomness).clone();
        test_one_99(_N, _M, _randomness)
    });
    ({
        let _N: () = N;
        let _M: () = N;
        let _randomness: Ptr<support_simple_random_generator> = (randomness).clone();
        test_one_99(_N, _M, _randomness)
    });
}
pub fn test_106(mut N: (), randomness: Ptr<support_simple_random_generator>) {
    ({
        let _N: () = N;
        let _randomness: Ptr<support_simple_random_generator> = (randomness).clone();
        test_one_100(_N, 0_(), _randomness)
    });
    ({
        let _N: () = N;
        let _M: () = (N).wrapping_div(4_());
        let _randomness: Ptr<support_simple_random_generator> = (randomness).clone();
        test_one_100(_N, _M, _randomness)
    });
    ({
        let _N: () = N;
        let _M: () = (N).wrapping_div(2_());
        let _randomness: Ptr<support_simple_random_generator> = (randomness).clone();
        test_one_100(_N, _M, _randomness)
    });
    ({
        let _N: () = N;
        let _M: () = ((3_()).wrapping_mul(N)).wrapping_div(4_());
        let _randomness: Ptr<support_simple_random_generator> = (randomness).clone();
        test_one_100(_N, _M, _randomness)
    });
    ({
        let _N: () = N;
        let _M: () = N;
        let _randomness: Ptr<support_simple_random_generator> = (randomness).clone();
        test_one_100(_N, _M, _randomness)
    });
}
pub fn test_107(mut N: (), randomness: Ptr<support_simple_random_generator>) {
    ({
        let _N: () = N;
        let _randomness: Ptr<support_simple_random_generator> = (randomness).clone();
        test_one_101(_N, 0_(), _randomness)
    });
    ({
        let _N: () = N;
        let _M: () = (N).wrapping_div(4_());
        let _randomness: Ptr<support_simple_random_generator> = (randomness).clone();
        test_one_101(_N, _M, _randomness)
    });
    ({
        let _N: () = N;
        let _M: () = (N).wrapping_div(2_());
        let _randomness: Ptr<support_simple_random_generator> = (randomness).clone();
        test_one_101(_N, _M, _randomness)
    });
    ({
        let _N: () = N;
        let _M: () = ((3_()).wrapping_mul(N)).wrapping_div(4_());
        let _randomness: Ptr<support_simple_random_generator> = (randomness).clone();
        test_one_101(_N, _M, _randomness)
    });
    ({
        let _N: () = N;
        let _M: () = N;
        let _randomness: Ptr<support_simple_random_generator> = (randomness).clone();
        test_one_101(_N, _M, _randomness)
    });
}
pub fn test_108(mut N: (), randomness: Ptr<support_simple_random_generator>) {
    ({
        let _N: () = N;
        let _randomness: Ptr<support_simple_random_generator> = (randomness).clone();
        test_one_102(_N, 0_(), _randomness)
    });
    ({
        let _N: () = N;
        let _M: () = (N).wrapping_div(4_());
        let _randomness: Ptr<support_simple_random_generator> = (randomness).clone();
        test_one_102(_N, _M, _randomness)
    });
    ({
        let _N: () = N;
        let _M: () = (N).wrapping_div(2_());
        let _randomness: Ptr<support_simple_random_generator> = (randomness).clone();
        test_one_102(_N, _M, _randomness)
    });
    ({
        let _N: () = N;
        let _M: () = ((3_()).wrapping_mul(N)).wrapping_div(4_());
        let _randomness: Ptr<support_simple_random_generator> = (randomness).clone();
        test_one_102(_N, _M, _randomness)
    });
    ({
        let _N: () = N;
        let _M: () = N;
        let _randomness: Ptr<support_simple_random_generator> = (randomness).clone();
        test_one_102(_N, _M, _randomness)
    });
}
pub fn test_109(mut N: (), randomness: Ptr<support_simple_random_generator>) {
    ({
        let _N: () = N;
        let _randomness: Ptr<support_simple_random_generator> = (randomness).clone();
        test_one_103(_N, 0_(), _randomness)
    });
    ({
        let _N: () = N;
        let _M: () = (N).wrapping_div(4_());
        let _randomness: Ptr<support_simple_random_generator> = (randomness).clone();
        test_one_103(_N, _M, _randomness)
    });
    ({
        let _N: () = N;
        let _M: () = (N).wrapping_div(2_());
        let _randomness: Ptr<support_simple_random_generator> = (randomness).clone();
        test_one_103(_N, _M, _randomness)
    });
    ({
        let _N: () = N;
        let _M: () = ((3_()).wrapping_mul(N)).wrapping_div(4_());
        let _randomness: Ptr<support_simple_random_generator> = (randomness).clone();
        test_one_103(_N, _M, _randomness)
    });
    ({
        let _N: () = N;
        let _M: () = N;
        let _randomness: Ptr<support_simple_random_generator> = (randomness).clone();
        test_one_103(_N, _M, _randomness)
    });
}
pub fn test_110(randomness: Ptr<support_simple_random_generator>) {
    ({ test_one_98(0_(), 0_(), (randomness).clone()) });
    ({ test_one_98(1_(), 0_(), (randomness).clone()) });
    ({ test_one_98(1_(), 1_(), (randomness).clone()) });
    ({ test_one_98(2_(), 0_(), (randomness).clone()) });
    ({ test_one_98(2_(), 1_(), (randomness).clone()) });
    ({ test_one_98(2_(), 2_(), (randomness).clone()) });
    ({ test_one_98(3_(), 0_(), (randomness).clone()) });
    ({ test_one_98(3_(), 1_(), (randomness).clone()) });
    ({ test_one_98(3_(), 2_(), (randomness).clone()) });
    ({ test_one_98(3_(), 3_(), (randomness).clone()) });
    ({ test_104(4_(), (randomness).clone()) });
    ({ test_104(20_(), (randomness).clone()) });
    ({ test_104(50_(), (randomness).clone()) });
    {
        ({ test_104(100_(), (randomness).clone()) });
        ({ test_104(1000_(), (randomness).clone()) });
    }
}
pub fn test_111(randomness: Ptr<support_simple_random_generator>) {
    ({ test_one_99(0_(), 0_(), (randomness).clone()) });
    ({ test_one_99(1_(), 0_(), (randomness).clone()) });
    ({ test_one_99(1_(), 1_(), (randomness).clone()) });
    ({ test_one_99(2_(), 0_(), (randomness).clone()) });
    ({ test_one_99(2_(), 1_(), (randomness).clone()) });
    ({ test_one_99(2_(), 2_(), (randomness).clone()) });
    ({ test_one_99(3_(), 0_(), (randomness).clone()) });
    ({ test_one_99(3_(), 1_(), (randomness).clone()) });
    ({ test_one_99(3_(), 2_(), (randomness).clone()) });
    ({ test_one_99(3_(), 3_(), (randomness).clone()) });
    ({ test_105(4_(), (randomness).clone()) });
    ({ test_105(20_(), (randomness).clone()) });
    ({ test_105(50_(), (randomness).clone()) });
    {
        ({ test_105(100_(), (randomness).clone()) });
        ({ test_105(1000_(), (randomness).clone()) });
    }
}
pub fn test_112(randomness: Ptr<support_simple_random_generator>) {
    ({ test_one_100(0_(), 0_(), (randomness).clone()) });
    ({ test_one_100(1_(), 0_(), (randomness).clone()) });
    ({ test_one_100(1_(), 1_(), (randomness).clone()) });
    ({ test_one_100(2_(), 0_(), (randomness).clone()) });
    ({ test_one_100(2_(), 1_(), (randomness).clone()) });
    ({ test_one_100(2_(), 2_(), (randomness).clone()) });
    ({ test_one_100(3_(), 0_(), (randomness).clone()) });
    ({ test_one_100(3_(), 1_(), (randomness).clone()) });
    ({ test_one_100(3_(), 2_(), (randomness).clone()) });
    ({ test_one_100(3_(), 3_(), (randomness).clone()) });
    ({ test_106(4_(), (randomness).clone()) });
    ({ test_106(20_(), (randomness).clone()) });
    ({ test_106(50_(), (randomness).clone()) });
    {
        ({ test_106(100_(), (randomness).clone()) });
        ({ test_106(1000_(), (randomness).clone()) });
    }
}
pub fn test_113(randomness: Ptr<support_simple_random_generator>) {
    ({ test_one_101(0_(), 0_(), (randomness).clone()) });
    ({ test_one_101(1_(), 0_(), (randomness).clone()) });
    ({ test_one_101(1_(), 1_(), (randomness).clone()) });
    ({ test_one_101(2_(), 0_(), (randomness).clone()) });
    ({ test_one_101(2_(), 1_(), (randomness).clone()) });
    ({ test_one_101(2_(), 2_(), (randomness).clone()) });
    ({ test_one_101(3_(), 0_(), (randomness).clone()) });
    ({ test_one_101(3_(), 1_(), (randomness).clone()) });
    ({ test_one_101(3_(), 2_(), (randomness).clone()) });
    ({ test_one_101(3_(), 3_(), (randomness).clone()) });
    ({ test_107(4_(), (randomness).clone()) });
    ({ test_107(20_(), (randomness).clone()) });
    ({ test_107(50_(), (randomness).clone()) });
    {
        ({ test_107(100_(), (randomness).clone()) });
        ({ test_107(1000_(), (randomness).clone()) });
    }
}
pub fn test_114(randomness: Ptr<support_simple_random_generator>) {
    ({ test_one_102(0_(), 0_(), (randomness).clone()) });
    ({ test_one_102(1_(), 0_(), (randomness).clone()) });
    ({ test_one_102(1_(), 1_(), (randomness).clone()) });
    ({ test_one_102(2_(), 0_(), (randomness).clone()) });
    ({ test_one_102(2_(), 1_(), (randomness).clone()) });
    ({ test_one_102(2_(), 2_(), (randomness).clone()) });
    ({ test_one_102(3_(), 0_(), (randomness).clone()) });
    ({ test_one_102(3_(), 1_(), (randomness).clone()) });
    ({ test_one_102(3_(), 2_(), (randomness).clone()) });
    ({ test_one_102(3_(), 3_(), (randomness).clone()) });
    ({ test_108(4_(), (randomness).clone()) });
    ({ test_108(20_(), (randomness).clone()) });
    ({ test_108(50_(), (randomness).clone()) });
    {
        ({ test_108(100_(), (randomness).clone()) });
        ({ test_108(1000_(), (randomness).clone()) });
    }
}
pub fn test_115(randomness: Ptr<support_simple_random_generator>) {
    ({ test_one_103(0_(), 0_(), (randomness).clone()) });
    ({ test_one_103(1_(), 0_(), (randomness).clone()) });
    ({ test_one_103(1_(), 1_(), (randomness).clone()) });
    ({ test_one_103(2_(), 0_(), (randomness).clone()) });
    ({ test_one_103(2_(), 1_(), (randomness).clone()) });
    ({ test_one_103(2_(), 2_(), (randomness).clone()) });
    ({ test_one_103(3_(), 0_(), (randomness).clone()) });
    ({ test_one_103(3_(), 1_(), (randomness).clone()) });
    ({ test_one_103(3_(), 2_(), (randomness).clone()) });
    ({ test_one_103(3_(), 3_(), (randomness).clone()) });
    ({ test_109(4_(), (randomness).clone()) });
    ({ test_109(20_(), (randomness).clone()) });
    ({ test_109(50_(), (randomness).clone()) });
    {
        ({ test_109(100_(), (randomness).clone()) });
        ({ test_109(1000_(), (randomness).clone()) });
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct less_by_first {}
pub fn test_PR31166_116() {
    let vec_: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        {
            let __tmp_0: Value<()> = Rc::new(RefCell::new(1_()));
            let __tmp_1: Value<()> = Rc::new(RefCell::new(0_()));
            std_pair_int__int_::new_1({ __tmp_0.as_pointer() }, { __tmp_1.as_pointer() })
        },
        {
            let __tmp_2: Value<()> = Rc::new(RefCell::new(2_()));
            let __tmp_3: Value<()> = Rc::new(RefCell::new(0_()));
            std_pair_int__int_::new_1({ __tmp_2.as_pointer() }, { __tmp_3.as_pointer() })
        },
        {
            let __tmp_4: Value<()> = Rc::new(RefCell::new(2_()));
            let __tmp_5: Value<()> = Rc::new(RefCell::new(1_()));
            std_pair_int__int_::new_1({ __tmp_4.as_pointer() }, { __tmp_5.as_pointer() })
        },
        {
            let __tmp_6: Value<()> = Rc::new(RefCell::new(2_()));
            let __tmp_7: Value<()> = Rc::new(RefCell::new(2_()));
            std_pair_int__int_::new_1({ __tmp_6.as_pointer() }, { __tmp_7.as_pointer() })
        },
        {
            let __tmp_8: Value<()> = Rc::new(RefCell::new(2_()));
            let __tmp_9: Value<()> = Rc::new(RefCell::new(3_()));
            std_pair_int__int_::new_1({ __tmp_8.as_pointer() }, { __tmp_9.as_pointer() })
        },
    ])));
    let mut i: () = 0_();
    'loop_: while (i < 5_()) {
        let res: Value<()> = Rc::new(RefCell::new(
            std_vector_std_pair_int__int___std_allocator_std_pair_int__int___::new_1(
                { (vec_.as_pointer() as Ptr<()>) },
                { (vec_.as_pointer() as Ptr<()>).offset((5_()) as isize) },
            ),
        ));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        i.prefix_inc();
    }
}
pub fn test_117() -> () {
    let randomness: Value<support_simple_random_generator> =
        Rc::new(RefCell::new(support_simple_random_generator::new_1()));
    ({ test_110(randomness.as_pointer()) });
    ({ test_111(randomness.as_pointer()) });
    ({ test_112(randomness.as_pointer()) });
    ({ test_113(randomness.as_pointer()) });
    ({ test_114(randomness.as_pointer()) });
    ({ test_115(randomness.as_pointer()) });
    {
        let mut N: () = 100_();
        let mut M: () = 50_();
        let mut ia: Ptr<()> =
            Ptr::alloc_array((0..N).map(|_| unimplemented!()).collect::<Box<[()]>>());
        let mut i: () = 0_();
        'loop_: while (i < N) {
            unimplemented!();
            i.prefix_inc();
        }
        ({
            let _first: Ptr<()> = (ia).clone();
            let _last: Ptr<()> = ia.offset((N) as isize);
            shuffle_76(_first, _last, randomness.as_pointer())
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        ia.delete();
    }
    ({ test_PR31166_116() });
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    ({ test_117() });
    return 0_();
}
pub trait SImpl {
    fn copy_assign(&self, rhs: Ptr<S>) -> Ptr<S>;
    fn move_assign(&self, rhs: Ptr<S>) -> Ptr<S>;
    fn operator_assign_7(&self, i: ()) -> Ptr<S>;
    fn operator_lt(&self, rhs: Ptr<S>) -> ();
    fn operator_gt(&self, rhs: Ptr<S>) -> ();
    fn operator_eq_10(&self, rhs: Ptr<S>) -> ();
    fn operator_eq_11(&self, i: ()) -> ();
    fn set(&self, i: ());
}
impl SImpl for Ptr<S> {
    fn copy_assign(&self, rhs: Ptr<S>) -> Ptr<S> {
        field!((*self), i_).write({ rhs.with(|__s| __s.i_) });
        return (*self).clone();
    }
    fn move_assign(&self, rhs: Ptr<S>) -> Ptr<S> {
        field!((*self), i_).write({ rhs.with(|__s| __s.i_) });
        field!(rhs, i_).write(-2_());
        unimplemented!();
        return (*self).clone();
    }
    fn operator_assign_7(&self, mut i: ()) -> Ptr<S> {
        field!((*self), i_).write(i);
        return (*self).clone();
    }
    fn operator_lt(&self, rhs: Ptr<S>) -> () {
        return ({ (*self).with(|__s| __s.i_) } < { rhs.with(|__s| __s.i_) });
    }
    fn operator_gt(&self, rhs: Ptr<S>) -> () {
        return ({ (*self).with(|__s| __s.i_) } > { rhs.with(|__s| __s.i_) });
    }
    fn operator_eq_10(&self, rhs: Ptr<S>) -> () {
        return ({ (*self).with(|__s| __s.i_) } == { rhs.with(|__s| __s.i_) });
    }
    fn operator_eq_11(&self, mut i: ()) -> () {
        return ((*self).with(|__s| __s.i_) == i);
    }
    fn set(&self, mut i: ()) {
        field!((*self), i_).write(i);
    }
}
pub trait adl_IteratorImpl {
    fn iter_moves(&self) -> ();
    fn iter_swaps(&self) -> ();
    fn operator_deref(&self) -> Ptr<()>;
    fn operator_index(&self, n: ()) -> Ptr<()>;
    fn operator_sub_10(&self, n: ()) -> adl_Iterator;
    fn operator_sub_11(&self, rhs: adl_Iterator) -> ();
    fn operator_add_assign(&self, n: ()) -> Ptr<adl_Iterator>;
    fn operator_sub_assign(&self, n: ()) -> Ptr<adl_Iterator>;
    fn operator_inc_14(&self) -> Ptr<adl_Iterator>;
    fn operator_post_inc_15(&self, _a0: ()) -> adl_Iterator;
    fn operator_dec_16(&self) -> Ptr<adl_Iterator>;
    fn operator_post_dec_17(&self, _a0: ()) -> adl_Iterator;
}
impl adl_IteratorImpl for Ptr<adl_Iterator> {
    fn iter_moves(&self) -> () {
        unimplemented!();
        return ((*self).with(|__s| __s.iter_moves_.clone()).read());
    }
    fn iter_swaps(&self) -> () {
        unimplemented!();
        return ((*self).with(|__s| __s.iter_swaps_.clone()).read());
    }
    fn operator_deref(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
    fn operator_index(&self, mut n: ()) -> Ptr<()> {
        return (*self).with(|__s| __s.ptr_.clone()).offset((n) as isize);
    }
    fn operator_sub_10(&self, mut n: ()) -> adl_Iterator {
        return adl_Iterator::new_1(
            { (*self).with(|__s| __s.ptr_.clone()).offset(-((n) as isize)) },
            { (*self).with(|__s| __s.iter_moves_.clone()) },
            { (*self).with(|__s| __s.iter_swaps_.clone()) },
        );
    }
    fn operator_sub_11(&self, mut rhs: adl_Iterator) -> () {
        return (((*self).with(|__s| __s.ptr_.clone()) - (rhs.ptr_).clone()) as () as ());
    }
    fn operator_add_assign(&self, mut n: ()) -> Ptr<adl_Iterator> {
        {
            let _ptr = field!((*self), ptr_);
            _ptr.write(_ptr.read() + n)
        };
        return (*self).clone();
    }
    fn operator_sub_assign(&self, mut n: ()) -> Ptr<adl_Iterator> {
        {
            let _ptr = field!((*self), ptr_);
            _ptr.write(_ptr.read() - n)
        };
        return (*self).clone();
    }
    fn operator_inc_14(&self) -> Ptr<adl_Iterator> {
        field!((*self), ptr_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
    fn operator_post_inc_15(&self, mut _a0: ()) -> adl_Iterator {
        let mut prev: adl_Iterator = (*(*self).upgrade().deref()).clone();
        field!((*self), ptr_).with_mut(|__v| __v.prefix_inc());
        return prev;
    }
    fn operator_dec_16(&self) -> Ptr<adl_Iterator> {
        field!((*self), ptr_).with_mut(|__v| __v.prefix_dec());
        return (*self).clone();
    }
    fn operator_post_dec_17(&self, mut _a0: ()) -> adl_Iterator {
        let mut prev: adl_Iterator = (*(*self).upgrade().deref()).clone();
        field!((*self), ptr_).with_mut(|__v| __v.prefix_dec());
        return prev;
    }
}
pub trait bidirectional_iterator_S_ptr_Impl {
    fn operator_deref(&self) -> ();
    fn operator_inc_6(&self) -> Ptr<bidirectional_iterator_S_ptr_>;
    fn operator_dec_7(&self) -> Ptr<bidirectional_iterator_S_ptr_>;
    fn operator_post_inc_8(&self, mut _a0: ()) -> bidirectional_iterator_S_ptr_ {
        unimplemented!()
    }
    fn operator_post_dec_9(&self, mut _a0: ()) -> bidirectional_iterator_S_ptr_ {
        unimplemented!()
    }
    fn move_assign(
        &self,
        _a0: Ptr<bidirectional_iterator_S_ptr_>,
    ) -> Ptr<bidirectional_iterator_S_ptr_>;
}
impl bidirectional_iterator_S_ptr_Impl for Ptr<bidirectional_iterator_S_ptr_> {
    fn operator_deref(&self) -> () {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_inc_6(&self) -> Ptr<bidirectional_iterator_S_ptr_> {
        field!((*self), it_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
    fn operator_dec_7(&self) -> Ptr<bidirectional_iterator_S_ptr_> {
        field!((*self), it_).with_mut(|__v| __v.prefix_dec());
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
    fn operator_deref(&self) -> ();
    fn operator_inc_6(&self) -> Ptr<bidirectional_iterator_int_ptr_>;
    fn operator_dec_7(&self) -> Ptr<bidirectional_iterator_int_ptr_>;
    fn operator_post_inc_8(&self, mut _a0: ()) -> bidirectional_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_post_dec_9(&self, mut _a0: ()) -> bidirectional_iterator_int_ptr_ {
        unimplemented!()
    }
    fn move_assign(
        &self,
        _a0: Ptr<bidirectional_iterator_int_ptr_>,
    ) -> Ptr<bidirectional_iterator_int_ptr_>;
}
impl bidirectional_iterator_int_ptr_Impl for Ptr<bidirectional_iterator_int_ptr_> {
    fn operator_deref(&self) -> () {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_inc_6(&self) -> Ptr<bidirectional_iterator_int_ptr_> {
        field!((*self), it_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
    fn operator_dec_7(&self) -> Ptr<bidirectional_iterator_int_ptr_> {
        field!((*self), it_).with_mut(|__v| __v.prefix_dec());
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
pub trait binary_counting_predicate_std_greater_S___S__S_Impl {
    fn operator_call(&self, a1: Ptr<S>, a2: Ptr<S>) -> ();
    fn count(&self) -> ();
    fn reset(&self) {
        unimplemented!()
    }
}
impl binary_counting_predicate_std_greater_S___S__S_Impl
    for Ptr<binary_counting_predicate_std_greater_S___S__S_>
{
    fn operator_call(&self, a1: Ptr<S>, a2: Ptr<S>) -> () {
        field!((*self), count_).with_mut(|__v| __v.prefix_inc());
        return unimplemented!();
    }
    fn count(&self) -> () {
        return (*self).with(|__s| __s.count_);
    }
}
pub trait binary_counting_predicate_std_greater_int___int__int_Impl {
    fn operator_call(&self, a1: Ptr<()>, a2: Ptr<()>) -> ();
    fn count(&self) -> ();
    fn reset(&self) {
        unimplemented!()
    }
}
impl binary_counting_predicate_std_greater_int___int__int_Impl
    for Ptr<binary_counting_predicate_std_greater_int___int__int_>
{
    fn operator_call(&self, a1: Ptr<()>, a2: Ptr<()>) -> () {
        field!((*self), count_).with_mut(|__v| __v.prefix_inc());
        return unimplemented!();
    }
    fn count(&self) -> () {
        return (*self).with(|__s| __s.count_);
    }
}
pub trait cpp17_input_iterator_int_ptr__int_ptr_Impl {
    fn operator_deref(&self) -> () {
        unimplemented!()
    }
    fn operator_inc_5(&self) -> Ptr<cpp17_input_iterator_int_ptr__int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_6(&self, mut _a0: ()) -> cpp17_input_iterator_int_ptr__int_ptr_ {
        unimplemented!()
    }
}
impl cpp17_input_iterator_int_ptr__int_ptr_Impl for Ptr<cpp17_input_iterator_int_ptr__int_ptr_> {}
pub trait cpp17_output_iterator_int_ptr_Impl {
    fn operator_deref(&self) -> () {
        unimplemented!()
    }
    fn operator_inc_5(&self) -> Ptr<cpp17_output_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_6(&self, mut _a0: ()) -> cpp17_output_iterator_int_ptr_ {
        unimplemented!()
    }
}
impl cpp17_output_iterator_int_ptr_Impl for Ptr<cpp17_output_iterator_int_ptr_> {}
pub trait cpp20_input_iterator_int_ptr_Impl {
    fn move_assign(
        &self,
        _a0: Ptr<cpp20_input_iterator_int_ptr_>,
    ) -> Ptr<cpp20_input_iterator_int_ptr_>;
    fn operator_deref(&self) -> Ptr<()>;
    fn operator_inc_5(&self) -> Ptr<cpp20_input_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_6(&self, mut _a0: ()) {
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
    fn operator_deref(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.it_.clone());
    }
}
pub trait cpp20_output_iterator_int_ptr_Impl {
    fn move_assign(
        &self,
        _a0: Ptr<cpp20_output_iterator_int_ptr_>,
    ) -> Ptr<cpp20_output_iterator_int_ptr_>;
    fn operator_deref(&self) -> Ptr<()>;
    fn operator_inc_5(&self) -> Ptr<cpp20_output_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_6(&self, mut _a0: ()) -> cpp20_output_iterator_int_ptr_ {
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
    fn operator_deref(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.it_.clone());
    }
}
pub trait cpp20_random_access_iterator_int_ptr_Impl {
    fn operator_deref(&self) -> Ptr<()>;
    fn operator_index(&self, n: ()) -> Ptr<()>;
    fn operator_inc_7(&self) -> Ptr<cpp20_random_access_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_dec_8(&self) -> Ptr<cpp20_random_access_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_9(&self, mut _a0: ()) -> cpp20_random_access_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_post_dec_10(&self, mut _a0: ()) -> cpp20_random_access_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_add_assign(&self, mut n: ()) -> Ptr<cpp20_random_access_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_sub_assign(&self, mut n: ()) -> Ptr<cpp20_random_access_iterator_int_ptr_> {
        unimplemented!()
    }
}
impl cpp20_random_access_iterator_int_ptr_Impl for Ptr<cpp20_random_access_iterator_int_ptr_> {
    fn operator_deref(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_index(&self, mut n: ()) -> Ptr<()> {
        return (*self).with(|__s| __s.it_.clone()).offset((n) as isize);
    }
}
pub trait forward_sized_iterator_int_ptr_Impl {
    fn operator_deref(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_inc_4(&self) -> Ptr<forward_sized_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_5(&self, mut _a0: ()) -> forward_sized_iterator_int_ptr_ {
        unimplemented!()
    }
}
impl forward_sized_iterator_int_ptr_Impl for Ptr<forward_sized_iterator_int_ptr_> {}
pub trait indirect_lessImpl {
    fn operator_call_1_class_std_unique_ptr_int_(&self, x: Ptr<()>, y: Ptr<()>) -> ();
}
impl indirect_lessImpl for Ptr<indirect_less> {
    fn operator_call_1_class_std_unique_ptr_int_(&self, x: Ptr<()>, y: Ptr<()>) -> () {
        return ({ ((*x.upgrade().deref()).read()) } < { ((*y.upgrade().deref()).read()) });
    }
}
pub trait less_by_firstImpl {
    fn operator_call_1_struct_std_pair_int__int_(&self, lhs: Ptr<()>, rhs: Ptr<()>) -> ();
}
impl less_by_firstImpl for Ptr<less_by_first> {
    fn operator_call_1_struct_std_pair_int__int_(&self, lhs: Ptr<()>, rhs: Ptr<()>) -> () {
        return unimplemented!();
    }
}
pub trait random_access_iterator_S_ptr_Impl {
    fn operator_deref(&self) -> ();
    fn operator_index(&self, mut n: ()) -> () {
        unimplemented!()
    }
    fn operator_inc_7(&self) -> Ptr<random_access_iterator_S_ptr_>;
    fn operator_dec_8(&self) -> Ptr<random_access_iterator_S_ptr_>;
    fn operator_post_inc_9(&self, mut _a0: ()) -> random_access_iterator_S_ptr_ {
        unimplemented!()
    }
    fn operator_post_dec_10(&self, mut _a0: ()) -> random_access_iterator_S_ptr_ {
        unimplemented!()
    }
    fn operator_add_assign(&self, n: ()) -> Ptr<random_access_iterator_S_ptr_>;
    fn operator_sub_assign(&self, mut n: ()) -> Ptr<random_access_iterator_S_ptr_> {
        unimplemented!()
    }
    fn move_assign(
        &self,
        _a0: Ptr<random_access_iterator_S_ptr_>,
    ) -> Ptr<random_access_iterator_S_ptr_>;
}
impl random_access_iterator_S_ptr_Impl for Ptr<random_access_iterator_S_ptr_> {
    fn operator_deref(&self) -> () {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_inc_7(&self) -> Ptr<random_access_iterator_S_ptr_> {
        field!((*self), it_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
    fn operator_dec_8(&self) -> Ptr<random_access_iterator_S_ptr_> {
        field!((*self), it_).with_mut(|__v| __v.prefix_dec());
        return (*self).clone();
    }
    fn operator_add_assign(&self, mut n: ()) -> Ptr<random_access_iterator_S_ptr_> {
        {
            let _ptr = field!((*self), it_);
            _ptr.write(_ptr.read() + n)
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
    fn operator_deref(&self) -> ();
    fn operator_index(&self, mut n: ()) -> () {
        unimplemented!()
    }
    fn operator_inc_7(&self) -> Ptr<random_access_iterator_int_ptr_>;
    fn operator_dec_8(&self) -> Ptr<random_access_iterator_int_ptr_>;
    fn operator_post_inc_9(&self, mut _a0: ()) -> random_access_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_post_dec_10(&self, mut _a0: ()) -> random_access_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_add_assign(&self, n: ()) -> Ptr<random_access_iterator_int_ptr_>;
    fn operator_sub_assign(&self, mut n: ()) -> Ptr<random_access_iterator_int_ptr_> {
        unimplemented!()
    }
    fn move_assign(
        &self,
        _a0: Ptr<random_access_iterator_int_ptr_>,
    ) -> Ptr<random_access_iterator_int_ptr_>;
}
impl random_access_iterator_int_ptr_Impl for Ptr<random_access_iterator_int_ptr_> {
    fn operator_deref(&self) -> () {
        return (*self).with(|__s| __s.it_.clone());
    }
    fn operator_inc_7(&self) -> Ptr<random_access_iterator_int_ptr_> {
        field!((*self), it_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
    fn operator_dec_8(&self) -> Ptr<random_access_iterator_int_ptr_> {
        field!((*self), it_).with_mut(|__v| __v.prefix_dec());
        return (*self).clone();
    }
    fn operator_add_assign(&self, mut n: ()) -> Ptr<random_access_iterator_int_ptr_> {
        {
            let _ptr = field!((*self), it_);
            _ptr.write(_ptr.read() + n)
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
    fn operator_deref(&self) -> Ptr<Ptr<()>> {
        unimplemented!()
    }
    fn operator_inc_4(&self) -> Ptr<rvalue_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_inc_5(&self, mut _a0: ()) -> rvalue_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_dec_6(&self) -> Ptr<rvalue_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_post_dec_7(&self, mut _a0: ()) -> rvalue_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_add(&self, mut n: ()) -> rvalue_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_sub_9(&self, mut n: ()) -> rvalue_iterator_int_ptr_ {
        unimplemented!()
    }
    fn operator_sub_10(&self, other: Ptr<rvalue_iterator_int_ptr_>) -> () {
        unimplemented!()
    }
    fn operator_add_assign(&self, mut n: ()) -> Ptr<rvalue_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_sub_assign(&self, mut n: ()) -> Ptr<rvalue_iterator_int_ptr_> {
        unimplemented!()
    }
    fn operator_index(&self, mut n: ()) -> Ptr<Ptr<()>> {
        unimplemented!()
    }
    fn operator_cmp(&self, _a0: Ptr<rvalue_iterator_int_ptr_>) -> ();
    fn operator_eq(&self, _a0: Ptr<rvalue_iterator_int_ptr_>) -> ();
}
impl rvalue_iterator_int_ptr_Impl for Ptr<rvalue_iterator_int_ptr_> {
    fn operator_cmp(&self, _a0: Ptr<rvalue_iterator_int_ptr_>) -> () {
        {
            let cmp: Value<()> = Rc::new(RefCell::new(std::cmp::Ord::cmp(
                &((*self).with(|__s| __s.it_.clone())),
                &(_a0.with(|__s| __s.it_.clone())),
            )));
            if !(unimplemented!()) {
                return unimplemented!();
            }
        }
        return unimplemented!();
    }
    fn operator_eq(&self, _a0: Ptr<rvalue_iterator_int_ptr_>) -> () {
        return ({ (*self).with(|__s| __s.it_.clone()) } == { _a0.with(|__s| __s.it_.clone()) });
    }
}
pub trait support_detail_independent_bits_engine_support_simple_random_generator__unsigned_long_Impl
{
    fn operator_call(&self) -> ();
    fn generate_3(&self, _a0: ()) -> () {
        unimplemented!()
    }
    fn generate_4(&self, _a0: ()) -> ();
}
impl support_detail_independent_bits_engine_support_simple_random_generator__unsigned_long_Impl
    for Ptr<support_detail_independent_bits_engine_support_simple_random_generator__unsigned_long_>
{
    fn operator_call(&self) -> () {
        return ({
            support_detail_independent_bits_engine_support_simple_random_generator__unsigned_long_Impl :: generate_4 ( self , unimplemented!()  , )
        });
    }
    fn generate_4(&self, _a0: ()) -> () {
        let _a0: Value<()> = Rc::new(RefCell::new(_a0));
        let mut r_digits: () = digits_70.with(|rc| *rc.borrow());
        let result: Value<()> = Rc::new(RefCell::new(0_()));
        let mut k: () = 0_();
        'loop_: while (k < (*self).with(|__s| __s.round_count_regular_)) {
            let mut eng_result: u16 = 0_u16;
            let mut __do_while = true;
            'loop_: while __do_while || ((eng_result as ()) >= (*self).with(|__s| __s.y0_)) {
                __do_while = false;
                eng_result = (({
                    (({
                        support_simple_random_generatorImpl::operator_call(
                            &(*self).with(|__s| __s.eng_.clone()),
                        )
                    }) as ())
                } - { (({ support_simple_random_generator::min() }) as ()) })
                    as u16);
            }
            if ((*self).with(|__s| __s.wid0_) < r_digits) {
                (*result.borrow_mut()) <<= (*self).with(|__s| __s.wid0_);
            } else {
                (*result.borrow_mut()) = 0_();
            }
            (*result.borrow_mut()) = {
                (*result.borrow())
                    .wrapping_add(((eng_result as ()) & ((*self).with(|__s| __s.mask0_) as ())))
            };
            k.prefix_inc();
        }
        let mut k: () = (*self).with(|__s| __s.round_count_regular_);
        'loop_: while (k < (*self).with(|__s| __s.round_count_all_)) {
            let mut eng_result: u16 = 0_u16;
            let mut __do_while = true;
            'loop_: while __do_while || ((eng_result as ()) >= (*self).with(|__s| __s.y1_)) {
                __do_while = false;
                eng_result = (({
                    (({
                        support_simple_random_generatorImpl::operator_call(
                            &(*self).with(|__s| __s.eng_.clone()),
                        )
                    }) as ())
                } - { (({ support_simple_random_generator::min() }) as ()) })
                    as u16);
            }
            if ((*self).with(|__s| __s.wid0_) < (r_digits).wrapping_sub(1_())) {
                (*result.borrow_mut()) <<= ((*self).with(|__s| __s.wid0_)).wrapping_add(1_());
            } else {
                (*result.borrow_mut()) = 0_();
            }
            (*result.borrow_mut()) = {
                (*result.borrow())
                    .wrapping_add(((eng_result as ()) & ((*self).with(|__s| __s.mask1_) as ())))
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
        unimplemented!();
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
            ((((*self).with(|__s| __s.status_) as ()).wrapping_mul((214013_() as ())))
                .wrapping_add((2531011_() as ())) as u32)
        });
        return (((*self).with(|__s| __s.status_) >> 16_()) as u16);
    }
}
pub trait support_uniform_int_distribution_long_Impl {
    fn reset(&self) {
        unimplemented!()
    }
    fn a(&self) -> () {
        unimplemented!()
    }
    fn b(&self) -> () {
        unimplemented!()
    }
    fn param_9(&self) -> support_uniform_int_distribution_long__param_type {
        unimplemented!()
    }
    fn param_10(&self, param: Ptr<support_uniform_int_distribution_long__param_type>) {
        unimplemented!()
    }
    fn min(&self) -> () {
        unimplemented!()
    }
    fn max(&self) -> () {
        unimplemented!()
    }
    fn operator_call_6_class_support_simple_random_generator(
        &self,
        gen_: Ptr<support_simple_random_generator>,
        param: Ptr<support_uniform_int_distribution_long__param_type>,
    ) -> ();
}
impl support_uniform_int_distribution_long_Impl for Ptr<support_uniform_int_distribution_long_> {
    fn operator_call_6_class_support_simple_random_generator(
        &self,
        gen_: Ptr<support_simple_random_generator>,
        param: Ptr<support_uniform_int_distribution_long__param_type>,
    ) -> () {
        const _: () = assert!(true, "detail::is_valid_urng<URNG>::value");;
        let mut rp: () = (({ support_uniform_int_distribution_long__param_typeImpl::b(&param) })
            .wrapping_sub(({ support_uniform_int_distribution_long__param_typeImpl::a(&param) })))
        .wrapping_add((1_() as ()));
        if (rp == 1_()) {
            return ({ support_uniform_int_distribution_long__param_typeImpl::a(&param) });
        }
        let mut ur_digits: () = digits_70.with(|rc| *rc.borrow());
        if (rp == 0_()) {
            return ({
                support_detail_independent_bits_engine_support_simple_random_generator__unsigned_long_Impl :: operator_call ( &Rc::new(RefCell::new(support_detail_independent_bits_engine_support_simple_random_generator__unsigned_long_ :: new ( { (gen_ ).clone() } , { ur_digits  } , )  )).as_pointer() , )
            });
        }
        let mut width: () = ((ur_digits).wrapping_sub(({ countl_zero_66(rp) }))).wrapping_sub(1_());
        if ((rp
            & (({ std_numeric_limits_unsigned_long_::max() })
                >> ((ur_digits).wrapping_sub(width))))
            != 0_())
        {
            width.prefix_inc();
        }
        let eng : Value<support_detail_independent_bits_engine_support_simple_random_generator__unsigned_long_ > = Rc::new(RefCell::new(support_detail_independent_bits_engine_support_simple_random_generator__unsigned_long_ :: new ( { (gen_ ).clone() } , { width  } , )  )) ;
        let mut eng_result: () = 0_();
        let mut __do_while = true;
        'loop_: while __do_while || (eng_result >= rp) {
            __do_while = false;
            eng_result = ({
                support_detail_independent_bits_engine_support_simple_random_generator__unsigned_long_Impl :: operator_call ( &eng .as_pointer()  , )
            });
        }
        return (eng_result)
            .wrapping_add(({ support_uniform_int_distribution_long__param_typeImpl::a(&param) }));
    }
}
pub trait support_uniform_int_distribution_long__param_typeImpl {
    fn a(&self) -> ();
    fn b(&self) -> ();
}
impl support_uniform_int_distribution_long__param_typeImpl
    for Ptr<support_uniform_int_distribution_long__param_type>
{
    fn a(&self) -> () {
        return (*self).with(|__s| __s.a_);
    }
    fn b(&self) -> () {
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
    let _ = e_digits_71.with(|_| ());
    let _ = default_seed_75.with(|_| ());
}
