extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_is_valid_integer_type_for_random_signed_char_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_is_valid_integer_type_for_random_short_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_is_valid_integer_type_for_random_int_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_is_valid_integer_type_for_random_long_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_is_valid_integer_type_for_random_long_long_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_is_valid_integer_type_for_random_unsigned_char_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_is_valid_integer_type_for_random_unsigned_short_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_is_valid_integer_type_for_random_unsigned_int_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_is_valid_integer_type_for_random_unsigned_long_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_is_valid_integer_type_for_random_unsigned_long_long_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_is_valid_integer_type_for_random___int128_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_is_valid_integer_type_for_random_unsigned___int128_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_is_valid_urng_support_simple_random_generator__void_ {}
pub static mut value_0: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    if (((65536_()) & ((1_()) << (63_()))) != 0) {
        63_()
    } else {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut value_1))
    }
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__63_ {}
pub static mut value_1: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    if (((65536_()) & ((1_()) << (62_()))) != 0) {
        62_()
    } else {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut value_2))
    }
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__62_ {}
pub static mut value_2: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    if (((65536_()) & ((1_()) << (61_()))) != 0) {
        61_()
    } else {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut value_3))
    }
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__61_ {}
pub static mut value_3: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    if (((65536_()) & ((1_()) << (60_()))) != 0) {
        60_()
    } else {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut value_4))
    }
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__60_ {}
pub static mut value_4: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    if (((65536_()) & ((1_()) << (59_()))) != 0) {
        59_()
    } else {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut value_5))
    }
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__59_ {}
pub static mut value_5: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    if (((65536_()) & ((1_()) << (58_()))) != 0) {
        58_()
    } else {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut value_6))
    }
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__58_ {}
pub static mut value_6: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    if (((65536_()) & ((1_()) << (57_()))) != 0) {
        57_()
    } else {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut value_7))
    }
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__57_ {}
pub static mut value_7: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    if (((65536_()) & ((1_()) << (56_()))) != 0) {
        56_()
    } else {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut value_8))
    }
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__56_ {}
pub static mut value_8: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    if (((65536_()) & ((1_()) << (55_()))) != 0) {
        55_()
    } else {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut value_9))
    }
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__55_ {}
pub static mut value_9: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    if (((65536_()) & ((1_()) << (54_()))) != 0) {
        54_()
    } else {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut value_10))
    }
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__54_ {}
pub static mut value_10: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    if (((65536_()) & ((1_()) << (53_()))) != 0) {
        53_()
    } else {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut value_11))
    }
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__53_ {}
pub static mut value_11: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    if (((65536_()) & ((1_()) << (52_()))) != 0) {
        52_()
    } else {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut value_12))
    }
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__52_ {}
pub static mut value_12: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    if (((65536_()) & ((1_()) << (51_()))) != 0) {
        51_()
    } else {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut value_13))
    }
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__51_ {}
pub static mut value_13: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    if (((65536_()) & ((1_()) << (50_()))) != 0) {
        50_()
    } else {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut value_14))
    }
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__50_ {}
pub static mut value_14: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    if (((65536_()) & ((1_()) << (49_()))) != 0) {
        49_()
    } else {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut value_15))
    }
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__49_ {}
pub static mut value_15: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    if (((65536_()) & ((1_()) << (48_()))) != 0) {
        48_()
    } else {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut value_16))
    }
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__48_ {}
pub static mut value_16: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    if (((65536_()) & ((1_()) << (47_()))) != 0) {
        47_()
    } else {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut value_17))
    }
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__47_ {}
pub static mut value_17: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    if (((65536_()) & ((1_()) << (46_()))) != 0) {
        46_()
    } else {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut value_18))
    }
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__46_ {}
pub static mut value_18: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    if (((65536_()) & ((1_()) << (45_()))) != 0) {
        45_()
    } else {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut value_19))
    }
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__45_ {}
pub static mut value_19: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    if (((65536_()) & ((1_()) << (44_()))) != 0) {
        44_()
    } else {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut value_20))
    }
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__44_ {}
pub static mut value_20: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    if (((65536_()) & ((1_()) << (43_()))) != 0) {
        43_()
    } else {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut value_21))
    }
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__43_ {}
pub static mut value_21: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    if (((65536_()) & ((1_()) << (42_()))) != 0) {
        42_()
    } else {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut value_22))
    }
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__42_ {}
pub static mut value_22: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    if (((65536_()) & ((1_()) << (41_()))) != 0) {
        41_()
    } else {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut value_23))
    }
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__41_ {}
pub static mut value_23: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    if (((65536_()) & ((1_()) << (40_()))) != 0) {
        40_()
    } else {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut value_24))
    }
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__40_ {}
pub static mut value_24: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    if (((65536_()) & ((1_()) << (39_()))) != 0) {
        39_()
    } else {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut value_25))
    }
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__39_ {}
pub static mut value_25: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    if (((65536_()) & ((1_()) << (38_()))) != 0) {
        38_()
    } else {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut value_26))
    }
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__38_ {}
pub static mut value_26: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    if (((65536_()) & ((1_()) << (37_()))) != 0) {
        37_()
    } else {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut value_27))
    }
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__37_ {}
pub static mut value_27: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    if (((65536_()) & ((1_()) << (36_()))) != 0) {
        36_()
    } else {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut value_28))
    }
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__36_ {}
pub static mut value_28: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    if (((65536_()) & ((1_()) << (35_()))) != 0) {
        35_()
    } else {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut value_29))
    }
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__35_ {}
pub static mut value_29: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    if (((65536_()) & ((1_()) << (34_()))) != 0) {
        34_()
    } else {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut value_30))
    }
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__34_ {}
pub static mut value_30: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    if (((65536_()) & ((1_()) << (33_()))) != 0) {
        33_()
    } else {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut value_31))
    }
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__33_ {}
pub static mut value_31: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    if (((65536_()) & ((1_()) << (32_()))) != 0) {
        32_()
    } else {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut value_32))
    }
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__32_ {}
pub static mut value_32: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    if (((65536_()) & ((1_()) << (31_()))) != 0) {
        31_()
    } else {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut value_33))
    }
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__31_ {}
pub static mut value_33: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    if (((65536_()) & ((1_()) << (30_()))) != 0) {
        30_()
    } else {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut value_34))
    }
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__30_ {}
pub static mut value_34: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    if (((65536_()) & ((1_()) << (29_()))) != 0) {
        29_()
    } else {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut value_35))
    }
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__29_ {}
pub static mut value_35: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    if (((65536_()) & ((1_()) << (28_()))) != 0) {
        28_()
    } else {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut value_36))
    }
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__28_ {}
pub static mut value_36: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    if (((65536_()) & ((1_()) << (27_()))) != 0) {
        27_()
    } else {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut value_37))
    }
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__27_ {}
pub static mut value_37: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    if (((65536_()) & ((1_()) << (26_()))) != 0) {
        26_()
    } else {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut value_38))
    }
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__26_ {}
pub static mut value_38: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    if (((65536_()) & ((1_()) << (25_()))) != 0) {
        25_()
    } else {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut value_39))
    }
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__25_ {}
pub static mut value_39: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    if (((65536_()) & ((1_()) << (24_()))) != 0) {
        24_()
    } else {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut value_40))
    }
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__24_ {}
pub static mut value_40: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    if (((65536_()) & ((1_()) << (23_()))) != 0) {
        23_()
    } else {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut value_41))
    }
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__23_ {}
pub static mut value_41: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    if (((65536_()) & ((1_()) << (22_()))) != 0) {
        22_()
    } else {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut value_42))
    }
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__22_ {}
pub static mut value_42: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    if (((65536_()) & ((1_()) << (21_()))) != 0) {
        21_()
    } else {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut value_43))
    }
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__21_ {}
pub static mut value_43: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    if (((65536_()) & ((1_()) << (20_()))) != 0) {
        20_()
    } else {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut value_44))
    }
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__20_ {}
pub static mut value_44: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    if (((65536_()) & ((1_()) << (19_()))) != 0) {
        19_()
    } else {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut value_45))
    }
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__19_ {}
pub static mut value_45: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    if (((65536_()) & ((1_()) << (18_()))) != 0) {
        18_()
    } else {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut value_46))
    }
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__18_ {}
pub static mut value_46: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    if (((65536_()) & ((1_()) << (17_()))) != 0) {
        17_()
    } else {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut value_47))
    }
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__17_ {}
pub static mut value_47: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    if (((65536_()) & ((1_()) << (16_()))) != 0) {
        16_()
    } else {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut value_48))
    }
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__16_ {}
pub static mut value_48: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    if (((65536_()) & ((1_()) << (15_()))) != 0) {
        15_()
    } else {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut value_49))
    }
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__15_ {}
pub static mut value_49: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    if (((65536_()) & ((1_()) << (14_()))) != 0) {
        14_()
    } else {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut value_50))
    }
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__14_ {}
pub static mut value_50: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    if (((65536_()) & ((1_()) << (13_()))) != 0) {
        13_()
    } else {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut value_51))
    }
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__13_ {}
pub static mut value_51: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    if (((65536_()) & ((1_()) << (12_()))) != 0) {
        12_()
    } else {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut value_52))
    }
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__12_ {}
pub static mut value_52: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    if (((65536_()) & ((1_()) << (11_()))) != 0) {
        11_()
    } else {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut value_53))
    }
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__11_ {}
pub static mut value_53: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    if (((65536_()) & ((1_()) << (10_()))) != 0) {
        10_()
    } else {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut value_54))
    }
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__10_ {}
pub static mut value_54: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    if (((65536_()) & ((1_()) << (9_()))) != 0) {
        9_()
    } else {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut value_55))
    }
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__9_ {}
pub static mut value_55: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    if (((65536_()) & ((1_()) << (8_()))) != 0) {
        8_()
    } else {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut value_56))
    }
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__8_ {}
pub static mut value_56: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    if (((65536_()) & ((1_()) << (7_()))) != 0) {
        7_()
    } else {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut value_57))
    }
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__7_ {}
pub static mut value_57: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    if (((65536_()) & ((1_()) << (6_()))) != 0) {
        6_()
    } else {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut value_58))
    }
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__6_ {}
pub static mut value_58: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    if (((65536_()) & ((1_()) << (5_()))) != 0) {
        5_()
    } else {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut value_59))
    }
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__5_ {}
pub static mut value_59: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    if (((65536_()) & ((1_()) << (4_()))) != 0) {
        4_()
    } else {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut value_60))
    }
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__4_ {}
pub static mut value_60: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    if (((65536_()) & ((1_()) << (3_()))) != 0) {
        3_()
    } else {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut value_61))
    }
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__3_ {}
pub static mut value_61: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    if (((65536_()) & ((1_()) << (2_()))) != 0) {
        2_()
    } else {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut value_62))
    }
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__2_ {}
pub static mut value_62: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    if (((65536_()) & ((1_()) << (1_()))) != 0) {
        1_()
    } else {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut value_63))
    }
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__1_ {}
pub static mut value_63: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__0_ {}
pub static mut value_64: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut value_0))
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_unsigned_long__65536_ {}
pub unsafe fn countl_zero_65(mut n: ()) -> () {
    return unimplemented!();
}
pub unsafe fn countl_zero_66(mut n: ()) -> () {
    return unimplemented!();
}
static mut rp_67: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    (((unsafe { support_simple_random_generator::max() }) as ())
        - ((unsafe { support_simple_random_generator::min() }) as ()))
        .wrapping_add((1_() as ()))
});
static mut rp_log2_68: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut value_64))
});
static mut w_digits_69: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
static mut e_digits_70: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_independent_bits_engine_support_simple_random_generator__unsigned_long_ {
    eng_: *mut support_simple_random_generator,
    width_: (),
    wid0_: (),
    round_count_all_: (),
    round_count_regular_: (),
    y0_: (),
    y1_: (),
    mask0_: u16,
    mask1_: u16,
}
impl support_detail_independent_bits_engine_support_simple_random_generator__unsigned_long_ {
    pub unsafe fn new(eng: *mut support_simple_random_generator, mut width: ()) -> Self {
        let mut this = Self {
            eng_: eng,
            width_: width,
            wid0_: unimplemented!(),
            round_count_all_: unimplemented!(),
            round_count_regular_: unimplemented!(),
            y0_: unimplemented!(),
            y1_: unimplemented!(),
            mask0_: 0_u16,
            mask1_: 0_u16,
        };
        this.round_count_all_ = ((this.width_)
            .wrapping_div((*std::cell::LazyCell::force_mut(&mut *&raw mut rp_log2_68))))
        .wrapping_add(
            (((this.width_)
                .wrapping_rem((*std::cell::LazyCell::force_mut(&mut *&raw mut rp_log2_68))))
                != (0_())),
        );
        this.wid0_ = (this.width_).wrapping_div(this.round_count_all_);
        {
            if ((this.wid0_) < (*std::cell::LazyCell::force_mut(&mut *&raw mut w_digits_69))) {
                this.y0_ = (((*std::cell::LazyCell::force_mut(&mut *&raw mut rp_67))
                    >> (this.wid0_))
                    << (this.wid0_));
            } else {
                this.y0_ = 0_();
            }
        }
        if (((*std::cell::LazyCell::force_mut(&mut *&raw mut rp_67)).wrapping_sub(this.y0_))
            > ((this.y0_).wrapping_div(this.round_count_all_)))
        {
            this.round_count_all_.prefix_inc();
            this.wid0_ = (this.width_).wrapping_div(this.round_count_all_);
            if ((this.wid0_) < (*std::cell::LazyCell::force_mut(&mut *&raw mut w_digits_69))) {
                this.y0_ = (((*std::cell::LazyCell::force_mut(&mut *&raw mut rp_67))
                    >> (this.wid0_))
                    << (this.wid0_));
            } else {
                this.y0_ = 0_();
            }
        }
        this.round_count_regular_ =
            (this.round_count_all_).wrapping_sub((this.width_).wrapping_rem(this.round_count_all_));
        if ((this.wid0_)
            < ((*std::cell::LazyCell::force_mut(&mut *&raw mut w_digits_69)).wrapping_sub(1_())))
        {
            this.y1_ = (((*std::cell::LazyCell::force_mut(&mut *&raw mut rp_67))
                >> ((this.wid0_).wrapping_add(1_())))
                << ((this.wid0_).wrapping_add(1_())));
        } else {
            this.y1_ = 0_();
        }
        this.mask0_ = if ((this.wid0_) > (0_())) {
            ((((!0_() as u16) as ())
                >> ((*std::cell::LazyCell::force_mut(&mut *&raw mut e_digits_70))
                    .wrapping_sub(this.wid0_))) as u16)
        } else {
            0_u16
        };
        this.mask1_ = if ((this.wid0_)
            < ((*std::cell::LazyCell::force_mut(&mut *&raw mut e_digits_70)).wrapping_sub(1_())))
        {
            ((((!0_() as u16) as ())
                >> ((*std::cell::LazyCell::force_mut(&mut *&raw mut e_digits_70))
                    .wrapping_sub(((this.wid0_).wrapping_add(1_()))))) as u16)
        } else {
            (!0_() as u16)
        };
        this
    }
    pub unsafe fn operator_call(&mut self) -> () {
        return (unsafe {
            support_detail_independent_bits_engine_support_simple_random_generator__unsigned_long_ :: generate_4 ( self , unimplemented!() , )
        });
    }
    unsafe fn generate_4(&mut self, mut _a0: ()) -> () {
        let r_digits: () = unimplemented!();
        let mut result: () = 0_();
        let mut k: () = 0_();
        'loop_: while ((k) < (self.round_count_regular_)) {
            let mut eng_result: u16 = 0_u16;
            let mut __do_while = true;
            'loop_: while __do_while || ((eng_result as ()) >= (self.y0_)) {
                __do_while = false;
                eng_result = ((((unsafe {
                    support_simple_random_generator::operator_call(&mut (*self.eng_))
                }) as ())
                    - ((unsafe { support_simple_random_generator::min() }) as ()))
                    as u16);
            }
            if ((self.wid0_) < (r_digits)) {
                result <<= self.wid0_;
            } else {
                result = 0_();
            }
            result = (result).wrapping_add(((eng_result as ()) & (self.mask0_ as ())));
            k.prefix_inc();
        }
        let mut k: () = self.round_count_regular_;
        'loop_: while ((k) < (self.round_count_all_)) {
            let mut eng_result: u16 = 0_u16;
            let mut __do_while = true;
            'loop_: while __do_while || ((eng_result as ()) >= (self.y1_)) {
                __do_while = false;
                eng_result = ((((unsafe {
                    support_simple_random_generator::operator_call(&mut (*self.eng_))
                }) as ())
                    - ((unsafe { support_simple_random_generator::min() }) as ()))
                    as u16);
            }
            if ((self.wid0_) < ((r_digits).wrapping_sub(1_()))) {
                result <<= (self.wid0_).wrapping_add(1_());
            } else {
                result = 0_();
            }
            result = (result).wrapping_add(((eng_result as ()) & (self.mask1_ as ())));
            k.prefix_inc();
        }
        return result;
    }
}
pub unsafe fn operator_eq_71(
    _a0: *const support_uniform_int_distribution_long_,
    _a1: *const support_uniform_int_distribution_long_,
) -> () {
    return (unsafe {
        let _arg0: *const support_uniform_int_distribution_long__param_type = &(*_a0).param_;
        let _arg1: *const support_uniform_int_distribution_long__param_type = &(*_a1).param_;
        operator_eq_72(_arg0, _arg1)
    });
}
pub unsafe fn operator_eq_72(
    _a0: *const support_uniform_int_distribution_long__param_type,
    _a1: *const support_uniform_int_distribution_long__param_type,
) -> () {
    return (((*_a0).a_) == ((*_a1).a_)) && (((*_a0).b_) == ((*_a1).b_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct support_uniform_int_distribution_long__param_type {
    a_: (),
    b_: (),
}
impl support_uniform_int_distribution_long__param_type {
    pub unsafe fn new_1() -> Self {
        let mut this = support_uniform_int_distribution_long__param_type::new_2({ 0_() }, None);
        this
    }
    pub unsafe fn new_2(mut ax: (), mut bx: Option<()>) -> Self {
        let mut bx: () = bx.unwrap_or_else(|| unsafe { unimplemented!() });
        let mut this = Self { a_: ax, b_: bx };
        this
    }
    pub unsafe fn a(&self) -> () {
        return self.a_;
    }
    pub unsafe fn b(&self) -> () {
        return self.b_;
    }
}
impl std::cmp::PartialEq for support_uniform_int_distribution_long__param_type {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_72(
                self as *const support_uniform_int_distribution_long__param_type,
                other as *const support_uniform_int_distribution_long__param_type,
            )
        }
    }
}
impl std::cmp::Eq for support_uniform_int_distribution_long__param_type {}
impl Default for support_uniform_int_distribution_long__param_type {
    fn default() -> Self {
        unsafe { support_uniform_int_distribution_long__param_type::new_1() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_uniform_int_distribution_long_ {
    param_: support_uniform_int_distribution_long__param_type,
}
impl support_uniform_int_distribution_long_ {
    pub unsafe fn operator_call_6_class_support_simple_random_generator(
        &mut self,
        gen_: *mut support_simple_random_generator,
        param: *const support_uniform_int_distribution_long__param_type,
    ) -> () {
        const _: () = assert!(true, "detail::is_valid_urng<URNG>::value");;
        let rp: () = ((unsafe { support_uniform_int_distribution_long__param_type::b(&(*param)) })
            .wrapping_sub(
                (unsafe { support_uniform_int_distribution_long__param_type::a(&(*param)) }),
            ))
        .wrapping_add((1_() as ()));
        if ((rp) == (1_())) {
            return (unsafe { support_uniform_int_distribution_long__param_type::a(&(*param)) });
        }
        let ur_digits: () = unimplemented!();
        if ((rp) == (0_())) {
            return (unsafe {
                support_detail_independent_bits_engine_support_simple_random_generator__unsigned_long_ :: operator_call ( &mut support_detail_independent_bits_engine_support_simple_random_generator__unsigned_long_ :: new ( { gen_ } , { ur_digits } , )   , )
            });
        }
        let mut width: () =
            ((ur_digits).wrapping_sub((unsafe { countl_zero_66(rp) }))).wrapping_sub(1_());
        if (((rp) & ((unimplemented!()) >> ((ur_digits).wrapping_sub(width)))) != (0_())) {
            width.prefix_inc();
        }
        let mut eng : support_detail_independent_bits_engine_support_simple_random_generator__unsigned_long_ = support_detail_independent_bits_engine_support_simple_random_generator__unsigned_long_ :: new ( { gen_ } , { width } , )  ;
        let mut eng_result: () = 0_();
        let mut __do_while = true;
        'loop_: while __do_while || ((eng_result) >= (rp)) {
            __do_while = false;
            eng_result = (unsafe {
                support_detail_independent_bits_engine_support_simple_random_generator__unsigned_long_ :: operator_call ( &mut eng  , )
            });
        }
        return (eng_result).wrapping_add(
            (unsafe { support_uniform_int_distribution_long__param_type::a(&(*param)) }),
        );
    }
}
impl std::cmp::PartialEq for support_uniform_int_distribution_long_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_71(
                self as *const support_uniform_int_distribution_long_,
                other as *const support_uniform_int_distribution_long_,
            )
        }
    }
}
impl std::cmp::Eq for support_uniform_int_distribution_long_ {}
pub static mut default_seed_73: std::cell::LazyCell<u16> =
    std::cell::LazyCell::new(|| unsafe { 19937_u16 });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct support_simple_random_generator {
    status_: u32,
}
impl support_simple_random_generator {
    pub unsafe fn min() -> u16 {
        return 0_u16;
    }
    pub unsafe fn max() -> u16 {
        return (-1_() as u16);
    }
    pub unsafe fn new_1() -> Self {
        let mut this = support_simple_random_generator::new_2({
            (*std::cell::LazyCell::force_mut(&mut *&raw mut default_seed_73))
        });
        this
    }
    pub unsafe fn new_2(mut s: u16) -> Self {
        let mut this = Self {
            status_: (s as u32),
        };
        this
    }
    pub unsafe fn operator_call(&mut self) -> u16 {
        self.status_ = (((self.status_ as ()).wrapping_mul((214013_() as ())))
            .wrapping_add((2531011_() as ())) as u32);
        return (((self.status_) >> (16_())) as u16);
    }
}
impl Default for support_simple_random_generator {
    fn default() -> Self {
        unsafe { support_simple_random_generator::new_1() }
    }
}
pub unsafe fn shuffle_74(
    mut first: *mut (),
    mut last: *mut (),
    gen_: *mut support_simple_random_generator,
) {
    let mut last_iter: *mut () = last;
    let mut diff: () =
        (((last_iter as usize - first as usize) / ::std::mem::size_of::<()>()) as () as ());
    if ((diff) > (1_())) {
        let mut uid: support_uniform_int_distribution_long_ =
            <support_uniform_int_distribution_long_>::default();
        {
            last_iter.prefix_dec();
            &(diff.prefix_dec())
        };
        'loop_: while ((first) < (last)) {
            let mut index: () = (unsafe {
                let _gen_: *mut support_simple_random_generator = gen_;
                let mut _param: support_uniform_int_distribution_long__param_type =
                    support_uniform_int_distribution_long__param_type::new_2({ 0_() }, {
                        Some(diff)
                    });
                support_uniform_int_distribution_long_ :: operator_call_6_class_support_simple_random_generator ( &mut uid  , _gen_ , & mut _param , )
            });
            if ((index) != (0_())) {
                unimplemented!();
            }
            {
                first.prefix_inc();
                &(diff.prefix_dec())
            };
        }
    }
}
pub unsafe fn shuffle_75(
    mut first: *mut (),
    mut last: *mut (),
    gen_: *mut support_simple_random_generator,
) {
    let mut last_iter: *mut () = last;
    let mut diff: () =
        (((last_iter as usize - first as usize) / ::std::mem::size_of::<()>()) as () as ());
    if ((diff) > (1_())) {
        let mut uid: support_uniform_int_distribution_long_ =
            <support_uniform_int_distribution_long_>::default();
        {
            last_iter.prefix_dec();
            &(diff.prefix_dec())
        };
        'loop_: while ((first) < (last)) {
            let mut index: () = (unsafe {
                let _gen_: *mut support_simple_random_generator = gen_;
                let mut _param: support_uniform_int_distribution_long__param_type =
                    support_uniform_int_distribution_long__param_type::new_2({ 0_() }, {
                        Some(diff)
                    });
                support_uniform_int_distribution_long_ :: operator_call_6_class_support_simple_random_generator ( &mut uid  , _gen_ , & mut _param , )
            });
            if ((index) != (0_())) {
                unimplemented!();
            }
            {
                first.prefix_inc();
                &(diff.prefix_dec())
            };
        }
    }
}
pub unsafe fn shuffle_76(
    mut first: *mut S,
    mut last: *mut S,
    gen_: *mut support_simple_random_generator,
) {
    let mut last_iter: *mut S = last;
    let mut diff: () =
        (((last_iter as usize - first as usize) / ::std::mem::size_of::<S>()) as () as ());
    if ((diff) > (1_())) {
        let mut uid: support_uniform_int_distribution_long_ =
            <support_uniform_int_distribution_long_>::default();
        {
            last_iter.prefix_dec();
            &(diff.prefix_dec())
        };
        'loop_: while ((first) < (last)) {
            let mut index: () = (unsafe {
                let _gen_: *mut support_simple_random_generator = gen_;
                let mut _param: support_uniform_int_distribution_long__param_type =
                    support_uniform_int_distribution_long__param_type::new_2({ 0_() }, {
                        Some(diff)
                    });
                support_uniform_int_distribution_long_ :: operator_call_6_class_support_simple_random_generator ( &mut uid  , _gen_ , & mut _param , )
            });
            if ((index) != (0_())) {
                unimplemented!();
            }
            {
                first.prefix_inc();
                &(diff.prefix_dec())
            };
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct indirect_less {}
impl indirect_less {
    pub unsafe fn operator_call_1_class_std_unique_ptr_int_(
        &self,
        x: *const (),
        y: *const (),
    ) -> () {
        return ((*unimplemented!()) < (*unimplemented!()));
    }
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, CopyAssignUnsafe, MoveAssignUnsafe)]
pub struct S {
    pub i_: (),
}
impl S {
    pub unsafe fn new_1() -> Self {
        let mut this = Self { i_: 0_() };
        this
    }
    pub unsafe fn new_2(mut i: ()) -> Self {
        let mut this = Self { i_: i };
        this
    }
    pub unsafe fn copy_from(rhs: *const S) -> Self {
        let mut this = Self { i_: (*rhs).i_ };
        this
    }
    pub unsafe fn move_from(rhs: *mut S) -> Self {
        let mut this = Self { i_: (*rhs).i_ };
        (*rhs).i_ = -1_();
        this
    }
    pub unsafe fn copy_assign(&mut self, rhs: *const S) -> *mut S {
        self.i_ = (*rhs).i_;
        return &mut (*(self as *mut S));
    }
    pub unsafe fn move_assign(&mut self, rhs: *mut S) -> *mut S {
        self.i_ = (*rhs).i_;
        (*rhs).i_ = -2_();
        unimplemented!();
        return &mut (*(self as *mut S));
    }
    pub unsafe fn operator_assign_7(&mut self, mut i: ()) -> *mut S {
        self.i_ = i;
        return &mut (*(self as *mut S));
    }
    pub unsafe fn operator_lt(&self, rhs: *const S) -> () {
        return ((self.i_) < ((*rhs).i_));
    }
    pub unsafe fn operator_gt(&self, rhs: *const S) -> () {
        return ((self.i_) > ((*rhs).i_));
    }
    pub unsafe fn operator_eq_10(&self, rhs: *const S) -> () {
        return ((self.i_) == ((*rhs).i_));
    }
    pub unsafe fn operator_eq_11(&self, mut i: ()) -> () {
        return ((self.i_) == (i));
    }
    pub unsafe fn set(&mut self, mut i: ()) {
        self.i_ = i;
    }
}
impl std::cmp::Ord for S {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if S::operator_lt(self, other as *const S) {
                std::cmp::Ordering::Less
            } else if S::operator_lt(other, self as *const S) {
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
        unsafe { S::operator_eq_10(self, other as *const S) }
    }
}
impl std::cmp::Eq for S {}
impl Clone for S {
    fn clone(&self) -> Self {
        unsafe { S::copy_from(self as *const S) }
    }
}
impl Default for S {
    fn default() -> Self {
        unsafe { S::new_1() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe)]
pub struct support_double_move_tracker {
    moved_from_: (),
}
impl support_double_move_tracker {
    pub unsafe fn new() -> Self {
        let mut this = Self { moved_from_: false };
        this
    }
    pub unsafe fn move_from(other: *mut support_double_move_tracker) -> Self {
        let mut this = Self { moved_from_: false };
        unimplemented!();
        (*other).moved_from_ = true;
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        other: *mut support_double_move_tracker,
    ) -> *mut support_double_move_tracker {
        unimplemented!();
        (*other).moved_from_ = true;
        self.moved_from_ = false;
        return &mut (*(self as *mut support_double_move_tracker));
    }
}
impl Default for support_double_move_tracker {
    fn default() -> Self {
        unsafe { support_double_move_tracker::new() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t___types_type_list_signed_char__short__int__long__long_long____int128___types_type_list_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128___types_type_list_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128___types_type_list_bool__
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__bool___types_type_list_float__double__long_double__
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct cpp17_output_iterator_int_ptr_ {
    it_: *mut (),
    tracker_: support_double_move_tracker,
}
impl cpp17_output_iterator_int_ptr_ {}
const _: () = assert!(
    true,
    "std::output_iterator<cpp17_output_iterator<int*>, int>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct cpp17_input_iterator_int_ptr__int_ptr_ {
    it_: *mut (),
    tracker_: support_double_move_tracker,
}
impl cpp17_input_iterator_int_ptr__int_ptr_ {}
const _: () = assert!(true, "std::input_iterator<cpp17_input_iterator<int*>>");
pub unsafe fn operator_eq_77(
    x: *const bidirectional_iterator_int_ptr_,
    y: *const bidirectional_iterator_int_ptr_,
) -> () {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_78(
    x: *const bidirectional_iterator_int_ptr_,
    y: *const bidirectional_iterator_int_ptr_,
) -> () {
    return (((*x).it_) != ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe)]
pub struct bidirectional_iterator_int_ptr_ {
    it_: *mut (),
    tracker_: support_double_move_tracker,
}
impl bidirectional_iterator_int_ptr_ {
    pub unsafe fn new_1() -> Self {
        let mut this = Self {
            it_: std::ptr::null_mut(),
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn new_2(mut it: *mut ()) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> () {
        return &mut (*self.it_);
    }
    pub unsafe fn operator_inc_6(&mut self) -> *mut bidirectional_iterator_int_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut bidirectional_iterator_int_ptr_));
    }
    pub unsafe fn operator_dec_7(&mut self) -> *mut bidirectional_iterator_int_ptr_ {
        self.it_.prefix_dec();
        return &mut (*(self as *mut bidirectional_iterator_int_ptr_));
    }
    pub unsafe fn move_from(_a0: *mut bidirectional_iterator_int_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut bidirectional_iterator_int_ptr_,
    ) -> *mut bidirectional_iterator_int_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut bidirectional_iterator_int_ptr_));
    }
}
impl std::cmp::PartialEq for bidirectional_iterator_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_77(
                self as *const bidirectional_iterator_int_ptr_,
                other as *const bidirectional_iterator_int_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for bidirectional_iterator_int_ptr_ {}
impl Default for bidirectional_iterator_int_ptr_ {
    fn default() -> Self {
        unsafe { bidirectional_iterator_int_ptr_::new_1() }
    }
}
pub unsafe fn operator_eq_79(
    x: *const bidirectional_iterator_S_ptr_,
    y: *const bidirectional_iterator_S_ptr_,
) -> () {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_80(
    x: *const bidirectional_iterator_S_ptr_,
    y: *const bidirectional_iterator_S_ptr_,
) -> () {
    return (((*x).it_) != ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe)]
pub struct bidirectional_iterator_S_ptr_ {
    it_: *mut S,
    tracker_: support_double_move_tracker,
}
impl bidirectional_iterator_S_ptr_ {
    pub unsafe fn new_1() -> Self {
        let mut this = Self {
            it_: std::ptr::null_mut(),
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn new_2(mut it: *mut S) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> () {
        return &mut (*self.it_);
    }
    pub unsafe fn operator_inc_6(&mut self) -> *mut bidirectional_iterator_S_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut bidirectional_iterator_S_ptr_));
    }
    pub unsafe fn operator_dec_7(&mut self) -> *mut bidirectional_iterator_S_ptr_ {
        self.it_.prefix_dec();
        return &mut (*(self as *mut bidirectional_iterator_S_ptr_));
    }
    pub unsafe fn move_from(_a0: *mut bidirectional_iterator_S_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut bidirectional_iterator_S_ptr_,
    ) -> *mut bidirectional_iterator_S_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut bidirectional_iterator_S_ptr_));
    }
}
impl std::cmp::PartialEq for bidirectional_iterator_S_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_79(
                self as *const bidirectional_iterator_S_ptr_,
                other as *const bidirectional_iterator_S_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for bidirectional_iterator_S_ptr_ {}
impl Default for bidirectional_iterator_S_ptr_ {
    fn default() -> Self {
        unsafe { bidirectional_iterator_S_ptr_::new_1() }
    }
}
pub unsafe fn operator_add_81(
    mut x: random_access_iterator_int_ptr_,
    mut n: (),
) -> random_access_iterator_int_ptr_ {
    (unsafe { random_access_iterator_int_ptr_::operator_add_assign(&mut x, n) });
    return random_access_iterator_int_ptr_::move_from({ &mut x });
}
pub unsafe fn operator_sub_82(
    mut x: random_access_iterator_int_ptr_,
    mut y: random_access_iterator_int_ptr_,
) -> () {
    return (((x.it_ as usize - y.it_ as usize) / ::std::mem::size_of::<()>()) as () as ());
}
pub unsafe fn operator_eq_83(
    x: *const random_access_iterator_int_ptr_,
    y: *const random_access_iterator_int_ptr_,
) -> () {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_84(
    x: *const random_access_iterator_int_ptr_,
    y: *const random_access_iterator_int_ptr_,
) -> () {
    return (((*x).it_) != ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe)]
pub struct random_access_iterator_int_ptr_ {
    it_: *mut (),
    tracker_: support_double_move_tracker,
}
impl random_access_iterator_int_ptr_ {
    pub unsafe fn new_1() -> Self {
        let mut this = Self {
            it_: std::ptr::null_mut(),
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn new_2(mut it: *mut ()) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> () {
        return &mut (*self.it_);
    }
    pub unsafe fn operator_inc_7(&mut self) -> *mut random_access_iterator_int_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut random_access_iterator_int_ptr_));
    }
    pub unsafe fn operator_dec_8(&mut self) -> *mut random_access_iterator_int_ptr_ {
        self.it_.prefix_dec();
        return &mut (*(self as *mut random_access_iterator_int_ptr_));
    }
    pub unsafe fn operator_add_assign(
        &mut self,
        mut n: (),
    ) -> *mut random_access_iterator_int_ptr_ {
        self.it_ = (self.it_).wrapping_add(n as usize);
        return &mut (*(self as *mut random_access_iterator_int_ptr_));
    }
    pub unsafe fn move_from(_a0: *mut random_access_iterator_int_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut random_access_iterator_int_ptr_,
    ) -> *mut random_access_iterator_int_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut random_access_iterator_int_ptr_));
    }
}
impl std::cmp::PartialEq for random_access_iterator_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_83(
                self as *const random_access_iterator_int_ptr_,
                other as *const random_access_iterator_int_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for random_access_iterator_int_ptr_ {}
impl Default for random_access_iterator_int_ptr_ {
    fn default() -> Self {
        unsafe { random_access_iterator_int_ptr_::new_1() }
    }
}
pub unsafe fn operator_add_85(
    mut x: random_access_iterator_S_ptr_,
    mut n: (),
) -> random_access_iterator_S_ptr_ {
    (unsafe { random_access_iterator_S_ptr_::operator_add_assign(&mut x, n) });
    return random_access_iterator_S_ptr_::move_from({ &mut x });
}
pub unsafe fn operator_sub_86(
    mut x: random_access_iterator_S_ptr_,
    mut y: random_access_iterator_S_ptr_,
) -> () {
    return (((x.it_ as usize - y.it_ as usize) / ::std::mem::size_of::<S>()) as () as ());
}
pub unsafe fn operator_eq_87(
    x: *const random_access_iterator_S_ptr_,
    y: *const random_access_iterator_S_ptr_,
) -> () {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_88(
    x: *const random_access_iterator_S_ptr_,
    y: *const random_access_iterator_S_ptr_,
) -> () {
    return (((*x).it_) != ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe)]
pub struct random_access_iterator_S_ptr_ {
    it_: *mut S,
    tracker_: support_double_move_tracker,
}
impl random_access_iterator_S_ptr_ {
    pub unsafe fn new_1() -> Self {
        let mut this = Self {
            it_: std::ptr::null_mut(),
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn new_2(mut it: *mut S) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> () {
        return &mut (*self.it_);
    }
    pub unsafe fn operator_inc_7(&mut self) -> *mut random_access_iterator_S_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut random_access_iterator_S_ptr_));
    }
    pub unsafe fn operator_dec_8(&mut self) -> *mut random_access_iterator_S_ptr_ {
        self.it_.prefix_dec();
        return &mut (*(self as *mut random_access_iterator_S_ptr_));
    }
    pub unsafe fn operator_add_assign(&mut self, mut n: ()) -> *mut random_access_iterator_S_ptr_ {
        self.it_ = (self.it_).wrapping_add(n as usize);
        return &mut (*(self as *mut random_access_iterator_S_ptr_));
    }
    pub unsafe fn move_from(_a0: *mut random_access_iterator_S_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut random_access_iterator_S_ptr_,
    ) -> *mut random_access_iterator_S_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut random_access_iterator_S_ptr_));
    }
}
impl std::cmp::PartialEq for random_access_iterator_S_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_87(
                self as *const random_access_iterator_S_ptr_,
                other as *const random_access_iterator_S_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for random_access_iterator_S_ptr_ {}
impl Default for random_access_iterator_S_ptr_ {
    fn default() -> Self {
        unsafe { random_access_iterator_S_ptr_::new_1() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct cpp20_random_access_iterator_int_ptr_ {
    it_: *mut (),
    tracker_: support_double_move_tracker,
}
impl cpp20_random_access_iterator_int_ptr_ {
    pub unsafe fn operator_deref(&self) -> *mut () {
        return &mut (*self.it_);
    }
    pub unsafe fn operator_index(&self, mut n: ()) -> *mut () {
        return &mut (*self.it_.offset((n) as isize));
    }
}
const _: () = assert!(
    true,
    "std::random_access_iterator<cpp20_random_access_iterator<int*>>"
);
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct cpp20_input_iterator_int_ptr_ {
    it_: *mut (),
    tracker_: support_double_move_tracker,
}
impl cpp20_input_iterator_int_ptr_ {
    pub unsafe fn move_from(_a0: *mut cpp20_input_iterator_int_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut cpp20_input_iterator_int_ptr_,
    ) -> *mut cpp20_input_iterator_int_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut cpp20_input_iterator_int_ptr_));
    }
    pub unsafe fn operator_deref(&self) -> *mut () {
        return &mut (*self.it_);
    }
}
const _: () = assert!(true, "std::input_iterator<cpp20_input_iterator<int*>>");
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct cpp20_output_iterator_int_ptr_ {
    it_: *mut (),
    tracker_: support_double_move_tracker,
}
impl cpp20_output_iterator_int_ptr_ {
    pub unsafe fn move_from(_a0: *mut cpp20_output_iterator_int_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut cpp20_output_iterator_int_ptr_,
    ) -> *mut cpp20_output_iterator_int_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut cpp20_output_iterator_int_ptr_));
    }
    pub unsafe fn operator_deref(&self) -> *mut () {
        return &mut (*self.it_);
    }
}
const _: () = assert!(
    true,
    "std::output_iterator<cpp20_output_iterator<int*>, int>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct IteratorOpCounts {
    pub increments: (),
    pub decrements: (),
    pub zero_moves: (),
    pub equal_cmps: (),
}
impl Default for IteratorOpCounts {
    fn default() -> Self {
        unsafe {
            IteratorOpCounts {
                increments: 0_(),
                decrements: 0_(),
                zero_moves: 0_(),
                equal_cmps: 0_(),
            }
        }
    }
}
pub unsafe fn operator_eq_89(
    _a0: *const forward_sized_iterator_int_ptr_,
    _a1: *const forward_sized_iterator_int_ptr_,
) -> () {
    return (((*_a0).it_) == ((*_a1).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct forward_sized_iterator_int_ptr_ {
    pub it_: *mut (),
}
impl forward_sized_iterator_int_ptr_ {}
impl std::cmp::PartialEq for forward_sized_iterator_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_89(
                self as *const forward_sized_iterator_int_ptr_,
                other as *const forward_sized_iterator_int_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for forward_sized_iterator_int_ptr_ {}
impl Default for forward_sized_iterator_int_ptr_ {
    fn default() -> Self {
        unsafe {
            forward_sized_iterator_int_ptr_ {
                it_: std::ptr::null_mut(),
            }
        }
    }
}
const _: () = assert!(true, "std::forward_iterator<forward_sized_iterator<>>");
const _: () = assert!(
    true,
    "std::sized_sentinel_for<forward_sized_iterator<>, forward_sized_iterator<>>"
);
pub unsafe fn operator_add_90(mut i: adl_Iterator, mut n: ()) -> adl_Iterator {
    return adl_Iterator::new_1({ i.ptr_.offset((n) as isize) }, { i.iter_moves_ }, {
        i.iter_swaps_
    });
}
pub unsafe fn operator_add_91(mut n: (), mut i: adl_Iterator) -> adl_Iterator {
    return (unsafe {
        let _i: adl_Iterator = i;
        operator_add_90(_i, n)
    });
}
pub unsafe fn iter_swap_92(mut a: adl_Iterator, mut b: adl_Iterator) {
    unimplemented!();
    if !(a.iter_swaps_).is_null() {
        (*a.iter_swaps_).prefix_inc();
    }
}
pub unsafe fn iter_move_93(mut iter: adl_Iterator) -> *mut () {
    if !(iter.iter_moves_).is_null() {
        (*iter.iter_moves_).prefix_inc();
    }
    return &mut (*(unsafe { adl_Iterator::operator_deref(&iter) }));
}
pub unsafe fn operator_eq_94(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> () {
    return (((*lhs).ptr_) == ((*rhs).ptr_));
}
pub unsafe fn operator_cmp_95(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> () {
    return std::cmp::Ord::cmp(&((*lhs).ptr_), &((*rhs).ptr_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct adl_Iterator {
    ptr_: *mut (),
    iter_moves_: *mut (),
    iter_swaps_: *mut (),
}
impl adl_Iterator {
    unsafe fn new_1(mut p: *mut (), mut iter_moves: *mut (), mut iter_swaps: *mut ()) -> Self {
        let mut this = Self {
            ptr_: p,
            iter_moves_: iter_moves,
            iter_swaps_: iter_swaps,
        };
        this
    }
    pub unsafe fn TrackMoves(mut p: *mut (), iter_moves: *mut ()) -> adl_Iterator {
        return adl_Iterator::new_1({ p }, { (iter_moves) }, { std::ptr::null_mut() });
    }
    pub unsafe fn TrackSwaps_4(iter_swaps: *mut ()) -> adl_Iterator {
        return adl_Iterator::new_1({ std::ptr::null_mut() }, { std::ptr::null_mut() }, {
            (iter_swaps)
        });
    }
    pub unsafe fn TrackSwaps_5(mut p: *mut (), iter_swaps: *mut ()) -> adl_Iterator {
        return adl_Iterator::new_1({ p }, { std::ptr::null_mut() }, { (iter_swaps) });
    }
    pub unsafe fn iter_moves(&self) -> () {
        unimplemented!();
        return (*self.iter_moves_);
    }
    pub unsafe fn iter_swaps(&self) -> () {
        unimplemented!();
        return (*self.iter_swaps_);
    }
    pub unsafe fn operator_deref(&self) -> *mut () {
        return &mut (*self.ptr_);
    }
    pub unsafe fn operator_index(&self, mut n: ()) -> *mut () {
        return &mut (*self.ptr_.offset((n) as isize));
    }
    pub unsafe fn operator_sub_10(&self, mut n: ()) -> adl_Iterator {
        return adl_Iterator::new_1(
            { self.ptr_.offset(-((n) as isize)) },
            { self.iter_moves_ },
            { self.iter_swaps_ },
        );
    }
    pub unsafe fn operator_sub_11(&self, mut rhs: adl_Iterator) -> () {
        return (((self.ptr_ as usize - rhs.ptr_ as usize) / ::std::mem::size_of::<()>()) as ()
            as ());
    }
    pub unsafe fn operator_add_assign(&mut self, mut n: ()) -> *mut adl_Iterator {
        self.ptr_ = (self.ptr_).wrapping_add(n as usize);
        return &mut (*(self as *mut adl_Iterator));
    }
    pub unsafe fn operator_sub_assign(&mut self, mut n: ()) -> *mut adl_Iterator {
        self.ptr_ = (self.ptr_).wrapping_sub(n as usize);
        return &mut (*(self as *mut adl_Iterator));
    }
    pub unsafe fn operator_inc_14(&mut self) -> *mut adl_Iterator {
        self.ptr_.prefix_inc();
        return &mut (*(self as *mut adl_Iterator));
    }
    pub unsafe fn operator_post_inc_15(&mut self, mut _a0: ()) -> adl_Iterator {
        let mut prev: adl_Iterator = (*(self as *mut adl_Iterator));
        self.ptr_.prefix_inc();
        return prev;
    }
    pub unsafe fn operator_dec_16(&mut self) -> *mut adl_Iterator {
        self.ptr_.prefix_dec();
        return &mut (*(self as *mut adl_Iterator));
    }
    pub unsafe fn operator_post_dec_17(&mut self, mut _a0: ()) -> adl_Iterator {
        let mut prev: adl_Iterator = (*(self as *mut adl_Iterator));
        self.ptr_.prefix_dec();
        return prev;
    }
}
impl std::cmp::Ord for adl_Iterator {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { operator_cmp_95(self as *const adl_Iterator, other as *const adl_Iterator) }
    }
}
impl std::cmp::PartialOrd for adl_Iterator {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for adl_Iterator {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_94(self as *const adl_Iterator, other as *const adl_Iterator) }
    }
}
impl std::cmp::Eq for adl_Iterator {}
impl Default for adl_Iterator {
    fn default() -> Self {
        unsafe {
            adl_Iterator {
                ptr_: std::ptr::null_mut(),
                iter_moves_: std::ptr::null_mut(),
                iter_swaps_: std::ptr::null_mut(),
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct rvalue_iterator_int_ptr_ {
    it_: *mut *mut (),
}
impl rvalue_iterator_int_ptr_ {
    pub unsafe fn operator_cmp(&self, _a0: *const rvalue_iterator_int_ptr_) -> () {
        {
            let mut cmp: () = std::cmp::Ord::cmp(
                &((*(self as *const rvalue_iterator_int_ptr_)).it_),
                &((*_a0).it_),
            );
            if !(unimplemented!()) {
                return unimplemented!();
            }
        }
        return unimplemented!();
    }
    pub unsafe fn operator_eq(&self, _a0: *const rvalue_iterator_int_ptr_) -> () {
        return (((*(self as *const rvalue_iterator_int_ptr_)).it_) == ((*_a0).it_));
    }
}
impl std::cmp::Ord for rvalue_iterator_int_ptr_ {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            rvalue_iterator_int_ptr_::operator_cmp(self, other as *const rvalue_iterator_int_ptr_)
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
        unsafe {
            rvalue_iterator_int_ptr_::operator_cmp(self, other as *const rvalue_iterator_int_ptr_)
                == std::cmp::Ordering::Equal
        }
    }
}
impl std::cmp::Eq for rvalue_iterator_int_ptr_ {}
const _: () = assert!(true, "std::random_access_iterator<rvalue_iterator<int*>>");
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct util_iterator_error {}
impl util_iterator_error {
    pub unsafe fn new(mut what: *const ()) -> Self {
        let mut this = Self {};
        this
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg)]
pub struct binary_counting_predicate_std_greater_int___int__int_ {
    p_: (),
    count_: (),
}
impl binary_counting_predicate_std_greater_int___int__int_ {
    pub unsafe fn new(mut p: ()) -> Self {
        let mut this = Self {
            p_: p.clone(),
            count_: 0_(),
        };
        this
    }
    pub unsafe fn operator_call(&self, a1: *const (), a2: *const ()) -> () {
        (*(&raw const self.count_).cast_mut()).prefix_inc();
        return unimplemented!();
    }
    pub unsafe fn count(&self) -> () {
        return self.count_;
    }
}
impl Default for binary_counting_predicate_std_greater_int___int__int_ {
    fn default() -> Self {
        unsafe {
            binary_counting_predicate_std_greater_int___int__int_ {
                p_: unimplemented!(),
                count_: unimplemented!(),
            }
        }
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg)]
pub struct binary_counting_predicate_std_greater_S___S__S_ {
    p_: (),
    count_: (),
}
impl binary_counting_predicate_std_greater_S___S__S_ {
    pub unsafe fn new(mut p: ()) -> Self {
        let mut this = Self {
            p_: p.clone(),
            count_: 0_(),
        };
        this
    }
    pub unsafe fn operator_call(&self, a1: *const S, a2: *const S) -> () {
        (*(&raw const self.count_).cast_mut()).prefix_inc();
        return unimplemented!();
    }
    pub unsafe fn count(&self) -> () {
        return self.count_;
    }
}
impl Default for binary_counting_predicate_std_greater_S___S__S_ {
    fn default() -> Self {
        unsafe {
            binary_counting_predicate_std_greater_S___S__S_ {
                p_: unimplemented!(),
                count_: unimplemented!(),
            }
        }
    }
}
pub unsafe fn test_one_96(mut N: (), mut M: (), randomness: *mut support_simple_random_generator) {
    unimplemented!();
    let mut ia: *mut () =
        Box::leak((0..N).map(|_| unimplemented!()).collect::<Box<[()]>>()).as_mut_ptr();
    let mut i: () = 0_();
    'loop_: while ((i) < (N)) {
        (*ia.offset((i) as isize)) = i;
        i.prefix_inc();
    }
    (unsafe {
        let _first: *mut () = ia;
        let _last: *mut () = ia.offset((N) as isize);
        let _gen_: *mut support_simple_random_generator = randomness;
        shuffle_75(_first, _last, _gen_)
    });
    unimplemented!();
    unimplemented!();
    let mut pred: binary_counting_predicate_std_greater_int___int__int_ =
        binary_counting_predicate_std_greater_int___int__int_::new({ (unimplemented!()) });
    unimplemented!();
    if ((N) > (0_())) {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let __p = ia;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                __p,
                libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                    / ::std::mem::size_of::<()>(),
            )))
        }
    };
}
pub unsafe fn test_one_97(mut N: (), mut M: (), randomness: *mut support_simple_random_generator) {
    unimplemented!();
    let mut ia: *mut () =
        Box::leak((0..N).map(|_| unimplemented!()).collect::<Box<[()]>>()).as_mut_ptr();
    let mut i: () = 0_();
    'loop_: while ((i) < (N)) {
        (*ia.offset((i) as isize)) = i;
        i.prefix_inc();
    }
    (unsafe {
        let _first: *mut () = ia;
        let _last: *mut () = ia.offset((N) as isize);
        let _gen_: *mut support_simple_random_generator = randomness;
        shuffle_75(_first, _last, _gen_)
    });
    unimplemented!();
    unimplemented!();
    let mut pred: binary_counting_predicate_std_greater_int___int__int_ =
        binary_counting_predicate_std_greater_int___int__int_::new({ (unimplemented!()) });
    unimplemented!();
    if ((N) > (0_())) {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let __p = ia;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                __p,
                libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                    / ::std::mem::size_of::<()>(),
            )))
        }
    };
}
pub unsafe fn test_one_98(mut N: (), mut M: (), randomness: *mut support_simple_random_generator) {
    unimplemented!();
    let mut ia: *mut () =
        Box::leak((0..N).map(|_| unimplemented!()).collect::<Box<[()]>>()).as_mut_ptr();
    let mut i: () = 0_();
    'loop_: while ((i) < (N)) {
        (*ia.offset((i) as isize)) = i;
        i.prefix_inc();
    }
    (unsafe {
        let _first: *mut () = ia;
        let _last: *mut () = ia.offset((N) as isize);
        let _gen_: *mut support_simple_random_generator = randomness;
        shuffle_75(_first, _last, _gen_)
    });
    unimplemented!();
    unimplemented!();
    let mut pred: binary_counting_predicate_std_greater_int___int__int_ =
        binary_counting_predicate_std_greater_int___int__int_::new({ (unimplemented!()) });
    unimplemented!();
    if ((N) > (0_())) {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let __p = ia;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                __p,
                libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                    / ::std::mem::size_of::<()>(),
            )))
        }
    };
}
pub unsafe fn test_one_99(mut N: (), mut M: (), randomness: *mut support_simple_random_generator) {
    unimplemented!();
    let mut ia: *mut S =
        Box::leak((0..N).map(|_| <S>::default()).collect::<Box<[S]>>()).as_mut_ptr();
    let mut i: () = 0_();
    'loop_: while ((i) < (N)) {
        (unsafe {
            let _i: () = i;
            S::operator_assign_7(&mut (*ia.offset((i) as isize)), _i)
        });
        i.prefix_inc();
    }
    (unsafe {
        let _first: *mut S = ia;
        let _last: *mut S = ia.offset((N) as isize);
        let _gen_: *mut support_simple_random_generator = randomness;
        shuffle_76(_first, _last, _gen_)
    });
    unimplemented!();
    unimplemented!();
    let mut pred: binary_counting_predicate_std_greater_S___S__S_ =
        binary_counting_predicate_std_greater_S___S__S_::new({ (unimplemented!()) });
    unimplemented!();
    if ((N) > (0_())) {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let __p = ia;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                __p,
                libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                    / ::std::mem::size_of::<S>(),
            )))
        }
    };
}
pub unsafe fn test_one_100(mut N: (), mut M: (), randomness: *mut support_simple_random_generator) {
    unimplemented!();
    let mut ia: *mut S =
        Box::leak((0..N).map(|_| <S>::default()).collect::<Box<[S]>>()).as_mut_ptr();
    let mut i: () = 0_();
    'loop_: while ((i) < (N)) {
        (unsafe {
            let _i: () = i;
            S::operator_assign_7(&mut (*ia.offset((i) as isize)), _i)
        });
        i.prefix_inc();
    }
    (unsafe {
        let _first: *mut S = ia;
        let _last: *mut S = ia.offset((N) as isize);
        let _gen_: *mut support_simple_random_generator = randomness;
        shuffle_76(_first, _last, _gen_)
    });
    unimplemented!();
    unimplemented!();
    let mut pred: binary_counting_predicate_std_greater_S___S__S_ =
        binary_counting_predicate_std_greater_S___S__S_::new({ (unimplemented!()) });
    unimplemented!();
    if ((N) > (0_())) {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let __p = ia;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                __p,
                libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                    / ::std::mem::size_of::<S>(),
            )))
        }
    };
}
pub unsafe fn test_one_101(mut N: (), mut M: (), randomness: *mut support_simple_random_generator) {
    unimplemented!();
    let mut ia: *mut S =
        Box::leak((0..N).map(|_| <S>::default()).collect::<Box<[S]>>()).as_mut_ptr();
    let mut i: () = 0_();
    'loop_: while ((i) < (N)) {
        (unsafe {
            let _i: () = i;
            S::operator_assign_7(&mut (*ia.offset((i) as isize)), _i)
        });
        i.prefix_inc();
    }
    (unsafe {
        let _first: *mut S = ia;
        let _last: *mut S = ia.offset((N) as isize);
        let _gen_: *mut support_simple_random_generator = randomness;
        shuffle_76(_first, _last, _gen_)
    });
    unimplemented!();
    unimplemented!();
    let mut pred: binary_counting_predicate_std_greater_S___S__S_ =
        binary_counting_predicate_std_greater_S___S__S_::new({ (unimplemented!()) });
    unimplemented!();
    if ((N) > (0_())) {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let __p = ia;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                __p,
                libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                    / ::std::mem::size_of::<S>(),
            )))
        }
    };
}
pub unsafe fn test_102(mut N: (), randomness: *mut support_simple_random_generator) {
    (unsafe {
        let _N: () = N;
        let _randomness: *mut support_simple_random_generator = randomness;
        test_one_96(_N, 0_(), _randomness)
    });
    (unsafe {
        let _N: () = N;
        let _M: () = (N).wrapping_div(4_());
        let _randomness: *mut support_simple_random_generator = randomness;
        test_one_96(_N, _M, _randomness)
    });
    (unsafe {
        let _N: () = N;
        let _M: () = (N).wrapping_div(2_());
        let _randomness: *mut support_simple_random_generator = randomness;
        test_one_96(_N, _M, _randomness)
    });
    (unsafe {
        let _N: () = N;
        let _M: () = ((3_()).wrapping_mul(N)).wrapping_div(4_());
        let _randomness: *mut support_simple_random_generator = randomness;
        test_one_96(_N, _M, _randomness)
    });
    (unsafe {
        let _N: () = N;
        let _M: () = N;
        let _randomness: *mut support_simple_random_generator = randomness;
        test_one_96(_N, _M, _randomness)
    });
}
pub unsafe fn test_103(mut N: (), randomness: *mut support_simple_random_generator) {
    (unsafe {
        let _N: () = N;
        let _randomness: *mut support_simple_random_generator = randomness;
        test_one_97(_N, 0_(), _randomness)
    });
    (unsafe {
        let _N: () = N;
        let _M: () = (N).wrapping_div(4_());
        let _randomness: *mut support_simple_random_generator = randomness;
        test_one_97(_N, _M, _randomness)
    });
    (unsafe {
        let _N: () = N;
        let _M: () = (N).wrapping_div(2_());
        let _randomness: *mut support_simple_random_generator = randomness;
        test_one_97(_N, _M, _randomness)
    });
    (unsafe {
        let _N: () = N;
        let _M: () = ((3_()).wrapping_mul(N)).wrapping_div(4_());
        let _randomness: *mut support_simple_random_generator = randomness;
        test_one_97(_N, _M, _randomness)
    });
    (unsafe {
        let _N: () = N;
        let _M: () = N;
        let _randomness: *mut support_simple_random_generator = randomness;
        test_one_97(_N, _M, _randomness)
    });
}
pub unsafe fn test_104(mut N: (), randomness: *mut support_simple_random_generator) {
    (unsafe {
        let _N: () = N;
        let _randomness: *mut support_simple_random_generator = randomness;
        test_one_98(_N, 0_(), _randomness)
    });
    (unsafe {
        let _N: () = N;
        let _M: () = (N).wrapping_div(4_());
        let _randomness: *mut support_simple_random_generator = randomness;
        test_one_98(_N, _M, _randomness)
    });
    (unsafe {
        let _N: () = N;
        let _M: () = (N).wrapping_div(2_());
        let _randomness: *mut support_simple_random_generator = randomness;
        test_one_98(_N, _M, _randomness)
    });
    (unsafe {
        let _N: () = N;
        let _M: () = ((3_()).wrapping_mul(N)).wrapping_div(4_());
        let _randomness: *mut support_simple_random_generator = randomness;
        test_one_98(_N, _M, _randomness)
    });
    (unsafe {
        let _N: () = N;
        let _M: () = N;
        let _randomness: *mut support_simple_random_generator = randomness;
        test_one_98(_N, _M, _randomness)
    });
}
pub unsafe fn test_105(mut N: (), randomness: *mut support_simple_random_generator) {
    (unsafe {
        let _N: () = N;
        let _randomness: *mut support_simple_random_generator = randomness;
        test_one_99(_N, 0_(), _randomness)
    });
    (unsafe {
        let _N: () = N;
        let _M: () = (N).wrapping_div(4_());
        let _randomness: *mut support_simple_random_generator = randomness;
        test_one_99(_N, _M, _randomness)
    });
    (unsafe {
        let _N: () = N;
        let _M: () = (N).wrapping_div(2_());
        let _randomness: *mut support_simple_random_generator = randomness;
        test_one_99(_N, _M, _randomness)
    });
    (unsafe {
        let _N: () = N;
        let _M: () = ((3_()).wrapping_mul(N)).wrapping_div(4_());
        let _randomness: *mut support_simple_random_generator = randomness;
        test_one_99(_N, _M, _randomness)
    });
    (unsafe {
        let _N: () = N;
        let _M: () = N;
        let _randomness: *mut support_simple_random_generator = randomness;
        test_one_99(_N, _M, _randomness)
    });
}
pub unsafe fn test_106(mut N: (), randomness: *mut support_simple_random_generator) {
    (unsafe {
        let _N: () = N;
        let _randomness: *mut support_simple_random_generator = randomness;
        test_one_100(_N, 0_(), _randomness)
    });
    (unsafe {
        let _N: () = N;
        let _M: () = (N).wrapping_div(4_());
        let _randomness: *mut support_simple_random_generator = randomness;
        test_one_100(_N, _M, _randomness)
    });
    (unsafe {
        let _N: () = N;
        let _M: () = (N).wrapping_div(2_());
        let _randomness: *mut support_simple_random_generator = randomness;
        test_one_100(_N, _M, _randomness)
    });
    (unsafe {
        let _N: () = N;
        let _M: () = ((3_()).wrapping_mul(N)).wrapping_div(4_());
        let _randomness: *mut support_simple_random_generator = randomness;
        test_one_100(_N, _M, _randomness)
    });
    (unsafe {
        let _N: () = N;
        let _M: () = N;
        let _randomness: *mut support_simple_random_generator = randomness;
        test_one_100(_N, _M, _randomness)
    });
}
pub unsafe fn test_107(mut N: (), randomness: *mut support_simple_random_generator) {
    (unsafe {
        let _N: () = N;
        let _randomness: *mut support_simple_random_generator = randomness;
        test_one_101(_N, 0_(), _randomness)
    });
    (unsafe {
        let _N: () = N;
        let _M: () = (N).wrapping_div(4_());
        let _randomness: *mut support_simple_random_generator = randomness;
        test_one_101(_N, _M, _randomness)
    });
    (unsafe {
        let _N: () = N;
        let _M: () = (N).wrapping_div(2_());
        let _randomness: *mut support_simple_random_generator = randomness;
        test_one_101(_N, _M, _randomness)
    });
    (unsafe {
        let _N: () = N;
        let _M: () = ((3_()).wrapping_mul(N)).wrapping_div(4_());
        let _randomness: *mut support_simple_random_generator = randomness;
        test_one_101(_N, _M, _randomness)
    });
    (unsafe {
        let _N: () = N;
        let _M: () = N;
        let _randomness: *mut support_simple_random_generator = randomness;
        test_one_101(_N, _M, _randomness)
    });
}
pub unsafe fn test_108(randomness: *mut support_simple_random_generator) {
    (unsafe { test_one_96(0_(), 0_(), randomness) });
    (unsafe { test_one_96(1_(), 0_(), randomness) });
    (unsafe { test_one_96(1_(), 1_(), randomness) });
    (unsafe { test_one_96(2_(), 0_(), randomness) });
    (unsafe { test_one_96(2_(), 1_(), randomness) });
    (unsafe { test_one_96(2_(), 2_(), randomness) });
    (unsafe { test_one_96(3_(), 0_(), randomness) });
    (unsafe { test_one_96(3_(), 1_(), randomness) });
    (unsafe { test_one_96(3_(), 2_(), randomness) });
    (unsafe { test_one_96(3_(), 3_(), randomness) });
    (unsafe { test_102(4_(), randomness) });
    (unsafe { test_102(20_(), randomness) });
    (unsafe { test_102(50_(), randomness) });
    {
        (unsafe { test_102(100_(), randomness) });
        (unsafe { test_102(1000_(), randomness) });
    }
}
pub unsafe fn test_109(randomness: *mut support_simple_random_generator) {
    (unsafe { test_one_97(0_(), 0_(), randomness) });
    (unsafe { test_one_97(1_(), 0_(), randomness) });
    (unsafe { test_one_97(1_(), 1_(), randomness) });
    (unsafe { test_one_97(2_(), 0_(), randomness) });
    (unsafe { test_one_97(2_(), 1_(), randomness) });
    (unsafe { test_one_97(2_(), 2_(), randomness) });
    (unsafe { test_one_97(3_(), 0_(), randomness) });
    (unsafe { test_one_97(3_(), 1_(), randomness) });
    (unsafe { test_one_97(3_(), 2_(), randomness) });
    (unsafe { test_one_97(3_(), 3_(), randomness) });
    (unsafe { test_103(4_(), randomness) });
    (unsafe { test_103(20_(), randomness) });
    (unsafe { test_103(50_(), randomness) });
    {
        (unsafe { test_103(100_(), randomness) });
        (unsafe { test_103(1000_(), randomness) });
    }
}
pub unsafe fn test_110(randomness: *mut support_simple_random_generator) {
    (unsafe { test_one_98(0_(), 0_(), randomness) });
    (unsafe { test_one_98(1_(), 0_(), randomness) });
    (unsafe { test_one_98(1_(), 1_(), randomness) });
    (unsafe { test_one_98(2_(), 0_(), randomness) });
    (unsafe { test_one_98(2_(), 1_(), randomness) });
    (unsafe { test_one_98(2_(), 2_(), randomness) });
    (unsafe { test_one_98(3_(), 0_(), randomness) });
    (unsafe { test_one_98(3_(), 1_(), randomness) });
    (unsafe { test_one_98(3_(), 2_(), randomness) });
    (unsafe { test_one_98(3_(), 3_(), randomness) });
    (unsafe { test_104(4_(), randomness) });
    (unsafe { test_104(20_(), randomness) });
    (unsafe { test_104(50_(), randomness) });
    {
        (unsafe { test_104(100_(), randomness) });
        (unsafe { test_104(1000_(), randomness) });
    }
}
pub unsafe fn test_111(randomness: *mut support_simple_random_generator) {
    (unsafe { test_one_99(0_(), 0_(), randomness) });
    (unsafe { test_one_99(1_(), 0_(), randomness) });
    (unsafe { test_one_99(1_(), 1_(), randomness) });
    (unsafe { test_one_99(2_(), 0_(), randomness) });
    (unsafe { test_one_99(2_(), 1_(), randomness) });
    (unsafe { test_one_99(2_(), 2_(), randomness) });
    (unsafe { test_one_99(3_(), 0_(), randomness) });
    (unsafe { test_one_99(3_(), 1_(), randomness) });
    (unsafe { test_one_99(3_(), 2_(), randomness) });
    (unsafe { test_one_99(3_(), 3_(), randomness) });
    (unsafe { test_105(4_(), randomness) });
    (unsafe { test_105(20_(), randomness) });
    (unsafe { test_105(50_(), randomness) });
    {
        (unsafe { test_105(100_(), randomness) });
        (unsafe { test_105(1000_(), randomness) });
    }
}
pub unsafe fn test_112(randomness: *mut support_simple_random_generator) {
    (unsafe { test_one_100(0_(), 0_(), randomness) });
    (unsafe { test_one_100(1_(), 0_(), randomness) });
    (unsafe { test_one_100(1_(), 1_(), randomness) });
    (unsafe { test_one_100(2_(), 0_(), randomness) });
    (unsafe { test_one_100(2_(), 1_(), randomness) });
    (unsafe { test_one_100(2_(), 2_(), randomness) });
    (unsafe { test_one_100(3_(), 0_(), randomness) });
    (unsafe { test_one_100(3_(), 1_(), randomness) });
    (unsafe { test_one_100(3_(), 2_(), randomness) });
    (unsafe { test_one_100(3_(), 3_(), randomness) });
    (unsafe { test_106(4_(), randomness) });
    (unsafe { test_106(20_(), randomness) });
    (unsafe { test_106(50_(), randomness) });
    {
        (unsafe { test_106(100_(), randomness) });
        (unsafe { test_106(1000_(), randomness) });
    }
}
pub unsafe fn test_113(randomness: *mut support_simple_random_generator) {
    (unsafe { test_one_101(0_(), 0_(), randomness) });
    (unsafe { test_one_101(1_(), 0_(), randomness) });
    (unsafe { test_one_101(1_(), 1_(), randomness) });
    (unsafe { test_one_101(2_(), 0_(), randomness) });
    (unsafe { test_one_101(2_(), 1_(), randomness) });
    (unsafe { test_one_101(2_(), 2_(), randomness) });
    (unsafe { test_one_101(3_(), 0_(), randomness) });
    (unsafe { test_one_101(3_(), 1_(), randomness) });
    (unsafe { test_one_101(3_(), 2_(), randomness) });
    (unsafe { test_one_101(3_(), 3_(), randomness) });
    (unsafe { test_107(4_(), randomness) });
    (unsafe { test_107(20_(), randomness) });
    (unsafe { test_107(50_(), randomness) });
    {
        (unsafe { test_107(100_(), randomness) });
        (unsafe { test_107(1000_(), randomness) });
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct less_by_first {}
impl less_by_first {
    pub unsafe fn operator_call_1_struct_std_pair_int__int_(
        &self,
        lhs: *const (),
        rhs: *const (),
    ) -> () {
        return unimplemented!();
    }
}
pub unsafe fn test_PR31166_114() {
    let mut vec_: [(); 5] = [
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ];
    let mut i: () = 0_();
    'loop_: while ((i) < (5_())) {
        let mut res: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        i.prefix_inc();
    }
}
pub unsafe fn test_115() -> () {
    let mut randomness: support_simple_random_generator = support_simple_random_generator::new_1();
    (unsafe { test_108(&mut randomness) });
    (unsafe { test_109(&mut randomness) });
    (unsafe { test_110(&mut randomness) });
    (unsafe { test_111(&mut randomness) });
    (unsafe { test_112(&mut randomness) });
    (unsafe { test_113(&mut randomness) });
    {
        let N: () = 100_();
        let M: () = 50_();
        let mut ia: *mut () =
            Box::leak((0..N).map(|_| unimplemented!()).collect::<Box<[()]>>()).as_mut_ptr();
        let mut i: () = 0_();
        'loop_: while ((i) < (N)) {
            unimplemented!();
            i.prefix_inc();
        }
        (unsafe {
            let _first: *mut () = ia;
            let _last: *mut () = ia.offset((N) as isize);
            shuffle_74(_first, _last, &mut randomness)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        {
            let __p = ia;
            if !__p.is_null() {
                ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                    __p,
                    libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                        / ::std::mem::size_of::<()>(),
                )))
            }
        };
    }
    (unsafe { test_PR31166_114() });
    return true;
}
pub fn main() {
    let mut args: Vec<Vec<u8>> = std::env::args()
        .map(|arg| arg.as_bytes().to_vec())
        .collect();
    args.iter_mut().for_each(|v| v.push(0));
    let mut argv: Vec<*mut libc::c_char> = args
        .iter()
        .map(|arg| arg.as_ptr() as *mut libc::c_char)
        .collect();
    argv.push(::std::ptr::null_mut());
    unsafe {
        __cpp2rust_init_globals();
        ::std::process::exit(main_0((argv.len() - 1) as i32, argv.as_mut_ptr()) as i32)
    }
}
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
    (unsafe { test_115() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const value_0);
    std::cell::LazyCell::force(&*&raw const value_1);
    std::cell::LazyCell::force(&*&raw const value_2);
    std::cell::LazyCell::force(&*&raw const value_3);
    std::cell::LazyCell::force(&*&raw const value_4);
    std::cell::LazyCell::force(&*&raw const value_5);
    std::cell::LazyCell::force(&*&raw const value_6);
    std::cell::LazyCell::force(&*&raw const value_7);
    std::cell::LazyCell::force(&*&raw const value_8);
    std::cell::LazyCell::force(&*&raw const value_9);
    std::cell::LazyCell::force(&*&raw const value_10);
    std::cell::LazyCell::force(&*&raw const value_11);
    std::cell::LazyCell::force(&*&raw const value_12);
    std::cell::LazyCell::force(&*&raw const value_13);
    std::cell::LazyCell::force(&*&raw const value_14);
    std::cell::LazyCell::force(&*&raw const value_15);
    std::cell::LazyCell::force(&*&raw const value_16);
    std::cell::LazyCell::force(&*&raw const value_17);
    std::cell::LazyCell::force(&*&raw const value_18);
    std::cell::LazyCell::force(&*&raw const value_19);
    std::cell::LazyCell::force(&*&raw const value_20);
    std::cell::LazyCell::force(&*&raw const value_21);
    std::cell::LazyCell::force(&*&raw const value_22);
    std::cell::LazyCell::force(&*&raw const value_23);
    std::cell::LazyCell::force(&*&raw const value_24);
    std::cell::LazyCell::force(&*&raw const value_25);
    std::cell::LazyCell::force(&*&raw const value_26);
    std::cell::LazyCell::force(&*&raw const value_27);
    std::cell::LazyCell::force(&*&raw const value_28);
    std::cell::LazyCell::force(&*&raw const value_29);
    std::cell::LazyCell::force(&*&raw const value_30);
    std::cell::LazyCell::force(&*&raw const value_31);
    std::cell::LazyCell::force(&*&raw const value_32);
    std::cell::LazyCell::force(&*&raw const value_33);
    std::cell::LazyCell::force(&*&raw const value_34);
    std::cell::LazyCell::force(&*&raw const value_35);
    std::cell::LazyCell::force(&*&raw const value_36);
    std::cell::LazyCell::force(&*&raw const value_37);
    std::cell::LazyCell::force(&*&raw const value_38);
    std::cell::LazyCell::force(&*&raw const value_39);
    std::cell::LazyCell::force(&*&raw const value_40);
    std::cell::LazyCell::force(&*&raw const value_41);
    std::cell::LazyCell::force(&*&raw const value_42);
    std::cell::LazyCell::force(&*&raw const value_43);
    std::cell::LazyCell::force(&*&raw const value_44);
    std::cell::LazyCell::force(&*&raw const value_45);
    std::cell::LazyCell::force(&*&raw const value_46);
    std::cell::LazyCell::force(&*&raw const value_47);
    std::cell::LazyCell::force(&*&raw const value_48);
    std::cell::LazyCell::force(&*&raw const value_49);
    std::cell::LazyCell::force(&*&raw const value_50);
    std::cell::LazyCell::force(&*&raw const value_51);
    std::cell::LazyCell::force(&*&raw const value_52);
    std::cell::LazyCell::force(&*&raw const value_53);
    std::cell::LazyCell::force(&*&raw const value_54);
    std::cell::LazyCell::force(&*&raw const value_55);
    std::cell::LazyCell::force(&*&raw const value_56);
    std::cell::LazyCell::force(&*&raw const value_57);
    std::cell::LazyCell::force(&*&raw const value_58);
    std::cell::LazyCell::force(&*&raw const value_59);
    std::cell::LazyCell::force(&*&raw const value_60);
    std::cell::LazyCell::force(&*&raw const value_61);
    std::cell::LazyCell::force(&*&raw const value_62);
    std::cell::LazyCell::force(&*&raw const value_63);
    std::cell::LazyCell::force(&*&raw const value_64);
    std::cell::LazyCell::force(&*&raw const rp_67);
    std::cell::LazyCell::force(&*&raw const rp_log2_68);
    std::cell::LazyCell::force(&*&raw const w_digits_69);
    std::cell::LazyCell::force(&*&raw const e_digits_70);
    std::cell::LazyCell::force(&*&raw const default_seed_73);
}
