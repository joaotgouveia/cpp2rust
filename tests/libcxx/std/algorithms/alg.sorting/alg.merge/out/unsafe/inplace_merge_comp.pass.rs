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
pub static mut value_0: std::cell::LazyCell<usize> = std::cell::LazyCell::new(|| unsafe {
    (if (((65536_u64) & (((1) as u64) << (63_u64))) != 0) {
        63_u64
    } else {
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut value_1)) as u64)
    } as usize)
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__63_ {}
pub static mut value_1: std::cell::LazyCell<usize> = std::cell::LazyCell::new(|| unsafe {
    (if (((65536_u64) & (((1) as u64) << (62_u64))) != 0) {
        62_u64
    } else {
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut value_2)) as u64)
    } as usize)
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__62_ {}
pub static mut value_2: std::cell::LazyCell<usize> = std::cell::LazyCell::new(|| unsafe {
    (if (((65536_u64) & (((1) as u64) << (61_u64))) != 0) {
        61_u64
    } else {
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut value_3)) as u64)
    } as usize)
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__61_ {}
pub static mut value_3: std::cell::LazyCell<usize> = std::cell::LazyCell::new(|| unsafe {
    (if (((65536_u64) & (((1) as u64) << (60_u64))) != 0) {
        60_u64
    } else {
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut value_4)) as u64)
    } as usize)
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__60_ {}
pub static mut value_4: std::cell::LazyCell<usize> = std::cell::LazyCell::new(|| unsafe {
    (if (((65536_u64) & (((1) as u64) << (59_u64))) != 0) {
        59_u64
    } else {
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut value_5)) as u64)
    } as usize)
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__59_ {}
pub static mut value_5: std::cell::LazyCell<usize> = std::cell::LazyCell::new(|| unsafe {
    (if (((65536_u64) & (((1) as u64) << (58_u64))) != 0) {
        58_u64
    } else {
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut value_6)) as u64)
    } as usize)
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__58_ {}
pub static mut value_6: std::cell::LazyCell<usize> = std::cell::LazyCell::new(|| unsafe {
    (if (((65536_u64) & (((1) as u64) << (57_u64))) != 0) {
        57_u64
    } else {
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut value_7)) as u64)
    } as usize)
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__57_ {}
pub static mut value_7: std::cell::LazyCell<usize> = std::cell::LazyCell::new(|| unsafe {
    (if (((65536_u64) & (((1) as u64) << (56_u64))) != 0) {
        56_u64
    } else {
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut value_8)) as u64)
    } as usize)
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__56_ {}
pub static mut value_8: std::cell::LazyCell<usize> = std::cell::LazyCell::new(|| unsafe {
    (if (((65536_u64) & (((1) as u64) << (55_u64))) != 0) {
        55_u64
    } else {
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut value_9)) as u64)
    } as usize)
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__55_ {}
pub static mut value_9: std::cell::LazyCell<usize> = std::cell::LazyCell::new(|| unsafe {
    (if (((65536_u64) & (((1) as u64) << (54_u64))) != 0) {
        54_u64
    } else {
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut value_10)) as u64)
    } as usize)
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__54_ {}
pub static mut value_10: std::cell::LazyCell<usize> = std::cell::LazyCell::new(|| unsafe {
    (if (((65536_u64) & (((1) as u64) << (53_u64))) != 0) {
        53_u64
    } else {
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut value_11)) as u64)
    } as usize)
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__53_ {}
pub static mut value_11: std::cell::LazyCell<usize> = std::cell::LazyCell::new(|| unsafe {
    (if (((65536_u64) & (((1) as u64) << (52_u64))) != 0) {
        52_u64
    } else {
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut value_12)) as u64)
    } as usize)
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__52_ {}
pub static mut value_12: std::cell::LazyCell<usize> = std::cell::LazyCell::new(|| unsafe {
    (if (((65536_u64) & (((1) as u64) << (51_u64))) != 0) {
        51_u64
    } else {
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut value_13)) as u64)
    } as usize)
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__51_ {}
pub static mut value_13: std::cell::LazyCell<usize> = std::cell::LazyCell::new(|| unsafe {
    (if (((65536_u64) & (((1) as u64) << (50_u64))) != 0) {
        50_u64
    } else {
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut value_14)) as u64)
    } as usize)
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__50_ {}
pub static mut value_14: std::cell::LazyCell<usize> = std::cell::LazyCell::new(|| unsafe {
    (if (((65536_u64) & (((1) as u64) << (49_u64))) != 0) {
        49_u64
    } else {
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut value_15)) as u64)
    } as usize)
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__49_ {}
pub static mut value_15: std::cell::LazyCell<usize> = std::cell::LazyCell::new(|| unsafe {
    (if (((65536_u64) & (((1) as u64) << (48_u64))) != 0) {
        48_u64
    } else {
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut value_16)) as u64)
    } as usize)
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__48_ {}
pub static mut value_16: std::cell::LazyCell<usize> = std::cell::LazyCell::new(|| unsafe {
    (if (((65536_u64) & (((1) as u64) << (47_u64))) != 0) {
        47_u64
    } else {
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut value_17)) as u64)
    } as usize)
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__47_ {}
pub static mut value_17: std::cell::LazyCell<usize> = std::cell::LazyCell::new(|| unsafe {
    (if (((65536_u64) & (((1) as u64) << (46_u64))) != 0) {
        46_u64
    } else {
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut value_18)) as u64)
    } as usize)
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__46_ {}
pub static mut value_18: std::cell::LazyCell<usize> = std::cell::LazyCell::new(|| unsafe {
    (if (((65536_u64) & (((1) as u64) << (45_u64))) != 0) {
        45_u64
    } else {
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut value_19)) as u64)
    } as usize)
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__45_ {}
pub static mut value_19: std::cell::LazyCell<usize> = std::cell::LazyCell::new(|| unsafe {
    (if (((65536_u64) & (((1) as u64) << (44_u64))) != 0) {
        44_u64
    } else {
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut value_20)) as u64)
    } as usize)
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__44_ {}
pub static mut value_20: std::cell::LazyCell<usize> = std::cell::LazyCell::new(|| unsafe {
    (if (((65536_u64) & (((1) as u64) << (43_u64))) != 0) {
        43_u64
    } else {
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut value_21)) as u64)
    } as usize)
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__43_ {}
pub static mut value_21: std::cell::LazyCell<usize> = std::cell::LazyCell::new(|| unsafe {
    (if (((65536_u64) & (((1) as u64) << (42_u64))) != 0) {
        42_u64
    } else {
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut value_22)) as u64)
    } as usize)
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__42_ {}
pub static mut value_22: std::cell::LazyCell<usize> = std::cell::LazyCell::new(|| unsafe {
    (if (((65536_u64) & (((1) as u64) << (41_u64))) != 0) {
        41_u64
    } else {
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut value_23)) as u64)
    } as usize)
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__41_ {}
pub static mut value_23: std::cell::LazyCell<usize> = std::cell::LazyCell::new(|| unsafe {
    (if (((65536_u64) & (((1) as u64) << (40_u64))) != 0) {
        40_u64
    } else {
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut value_24)) as u64)
    } as usize)
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__40_ {}
pub static mut value_24: std::cell::LazyCell<usize> = std::cell::LazyCell::new(|| unsafe {
    (if (((65536_u64) & (((1) as u64) << (39_u64))) != 0) {
        39_u64
    } else {
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut value_25)) as u64)
    } as usize)
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__39_ {}
pub static mut value_25: std::cell::LazyCell<usize> = std::cell::LazyCell::new(|| unsafe {
    (if (((65536_u64) & (((1) as u64) << (38_u64))) != 0) {
        38_u64
    } else {
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut value_26)) as u64)
    } as usize)
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__38_ {}
pub static mut value_26: std::cell::LazyCell<usize> = std::cell::LazyCell::new(|| unsafe {
    (if (((65536_u64) & (((1) as u64) << (37_u64))) != 0) {
        37_u64
    } else {
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut value_27)) as u64)
    } as usize)
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__37_ {}
pub static mut value_27: std::cell::LazyCell<usize> = std::cell::LazyCell::new(|| unsafe {
    (if (((65536_u64) & (((1) as u64) << (36_u64))) != 0) {
        36_u64
    } else {
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut value_28)) as u64)
    } as usize)
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__36_ {}
pub static mut value_28: std::cell::LazyCell<usize> = std::cell::LazyCell::new(|| unsafe {
    (if (((65536_u64) & (((1) as u64) << (35_u64))) != 0) {
        35_u64
    } else {
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut value_29)) as u64)
    } as usize)
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__35_ {}
pub static mut value_29: std::cell::LazyCell<usize> = std::cell::LazyCell::new(|| unsafe {
    (if (((65536_u64) & (((1) as u64) << (34_u64))) != 0) {
        34_u64
    } else {
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut value_30)) as u64)
    } as usize)
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__34_ {}
pub static mut value_30: std::cell::LazyCell<usize> = std::cell::LazyCell::new(|| unsafe {
    (if (((65536_u64) & (((1) as u64) << (33_u64))) != 0) {
        33_u64
    } else {
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut value_31)) as u64)
    } as usize)
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__33_ {}
pub static mut value_31: std::cell::LazyCell<usize> = std::cell::LazyCell::new(|| unsafe {
    (if (((65536_u64) & (((1) as u64) << (32_u64))) != 0) {
        32_u64
    } else {
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut value_32)) as u64)
    } as usize)
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__32_ {}
pub static mut value_32: std::cell::LazyCell<usize> = std::cell::LazyCell::new(|| unsafe {
    (if (((65536_u64) & (((1) as u64) << (31_u64))) != 0) {
        31_u64
    } else {
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut value_33)) as u64)
    } as usize)
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__31_ {}
pub static mut value_33: std::cell::LazyCell<usize> = std::cell::LazyCell::new(|| unsafe {
    (if (((65536_u64) & (((1) as u64) << (30_u64))) != 0) {
        30_u64
    } else {
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut value_34)) as u64)
    } as usize)
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__30_ {}
pub static mut value_34: std::cell::LazyCell<usize> = std::cell::LazyCell::new(|| unsafe {
    (if (((65536_u64) & (((1) as u64) << (29_u64))) != 0) {
        29_u64
    } else {
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut value_35)) as u64)
    } as usize)
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__29_ {}
pub static mut value_35: std::cell::LazyCell<usize> = std::cell::LazyCell::new(|| unsafe {
    (if (((65536_u64) & (((1) as u64) << (28_u64))) != 0) {
        28_u64
    } else {
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut value_36)) as u64)
    } as usize)
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__28_ {}
pub static mut value_36: std::cell::LazyCell<usize> = std::cell::LazyCell::new(|| unsafe {
    (if (((65536_u64) & (((1) as u64) << (27_u64))) != 0) {
        27_u64
    } else {
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut value_37)) as u64)
    } as usize)
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__27_ {}
pub static mut value_37: std::cell::LazyCell<usize> = std::cell::LazyCell::new(|| unsafe {
    (if (((65536_u64) & (((1) as u64) << (26_u64))) != 0) {
        26_u64
    } else {
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut value_38)) as u64)
    } as usize)
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__26_ {}
pub static mut value_38: std::cell::LazyCell<usize> = std::cell::LazyCell::new(|| unsafe {
    (if (((65536_u64) & (((1) as u64) << (25_u64))) != 0) {
        25_u64
    } else {
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut value_39)) as u64)
    } as usize)
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__25_ {}
pub static mut value_39: std::cell::LazyCell<usize> = std::cell::LazyCell::new(|| unsafe {
    (if (((65536_u64) & (((1) as u64) << (24_u64))) != 0) {
        24_u64
    } else {
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut value_40)) as u64)
    } as usize)
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__24_ {}
pub static mut value_40: std::cell::LazyCell<usize> = std::cell::LazyCell::new(|| unsafe {
    (if (((65536_u64) & (((1) as u64) << (23_u64))) != 0) {
        23_u64
    } else {
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut value_41)) as u64)
    } as usize)
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__23_ {}
pub static mut value_41: std::cell::LazyCell<usize> = std::cell::LazyCell::new(|| unsafe {
    (if (((65536_u64) & (((1) as u64) << (22_u64))) != 0) {
        22_u64
    } else {
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut value_42)) as u64)
    } as usize)
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__22_ {}
pub static mut value_42: std::cell::LazyCell<usize> = std::cell::LazyCell::new(|| unsafe {
    (if (((65536_u64) & (((1) as u64) << (21_u64))) != 0) {
        21_u64
    } else {
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut value_43)) as u64)
    } as usize)
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__21_ {}
pub static mut value_43: std::cell::LazyCell<usize> = std::cell::LazyCell::new(|| unsafe {
    (if (((65536_u64) & (((1) as u64) << (20_u64))) != 0) {
        20_u64
    } else {
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut value_44)) as u64)
    } as usize)
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__20_ {}
pub static mut value_44: std::cell::LazyCell<usize> = std::cell::LazyCell::new(|| unsafe {
    (if (((65536_u64) & (((1) as u64) << (19_u64))) != 0) {
        19_u64
    } else {
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut value_45)) as u64)
    } as usize)
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__19_ {}
pub static mut value_45: std::cell::LazyCell<usize> = std::cell::LazyCell::new(|| unsafe {
    (if (((65536_u64) & (((1) as u64) << (18_u64))) != 0) {
        18_u64
    } else {
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut value_46)) as u64)
    } as usize)
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__18_ {}
pub static mut value_46: std::cell::LazyCell<usize> = std::cell::LazyCell::new(|| unsafe {
    (if (((65536_u64) & (((1) as u64) << (17_u64))) != 0) {
        17_u64
    } else {
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut value_47)) as u64)
    } as usize)
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__17_ {}
pub static mut value_47: std::cell::LazyCell<usize> = std::cell::LazyCell::new(|| unsafe {
    (if (((65536_u64) & (((1) as u64) << (16_u64))) != 0) {
        16_u64
    } else {
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut value_48)) as u64)
    } as usize)
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__16_ {}
pub static mut value_48: std::cell::LazyCell<usize> = std::cell::LazyCell::new(|| unsafe {
    (if (((65536_u64) & (((1) as u64) << (15_u64))) != 0) {
        15_u64
    } else {
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut value_49)) as u64)
    } as usize)
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__15_ {}
pub static mut value_49: std::cell::LazyCell<usize> = std::cell::LazyCell::new(|| unsafe {
    (if (((65536_u64) & (((1) as u64) << (14_u64))) != 0) {
        14_u64
    } else {
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut value_50)) as u64)
    } as usize)
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__14_ {}
pub static mut value_50: std::cell::LazyCell<usize> = std::cell::LazyCell::new(|| unsafe {
    (if (((65536_u64) & (((1) as u64) << (13_u64))) != 0) {
        13_u64
    } else {
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut value_51)) as u64)
    } as usize)
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__13_ {}
pub static mut value_51: std::cell::LazyCell<usize> = std::cell::LazyCell::new(|| unsafe {
    (if (((65536_u64) & (((1) as u64) << (12_u64))) != 0) {
        12_u64
    } else {
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut value_52)) as u64)
    } as usize)
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__12_ {}
pub static mut value_52: std::cell::LazyCell<usize> = std::cell::LazyCell::new(|| unsafe {
    (if (((65536_u64) & (((1) as u64) << (11_u64))) != 0) {
        11_u64
    } else {
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut value_53)) as u64)
    } as usize)
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__11_ {}
pub static mut value_53: std::cell::LazyCell<usize> = std::cell::LazyCell::new(|| unsafe {
    (if (((65536_u64) & (((1) as u64) << (10_u64))) != 0) {
        10_u64
    } else {
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut value_54)) as u64)
    } as usize)
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__10_ {}
pub static mut value_54: std::cell::LazyCell<usize> = std::cell::LazyCell::new(|| unsafe {
    (if (((65536_u64) & (((1) as u64) << (9_u64))) != 0) {
        9_u64
    } else {
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut value_55)) as u64)
    } as usize)
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__9_ {}
pub static mut value_55: std::cell::LazyCell<usize> = std::cell::LazyCell::new(|| unsafe {
    (if (((65536_u64) & (((1) as u64) << (8_u64))) != 0) {
        8_u64
    } else {
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut value_56)) as u64)
    } as usize)
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__8_ {}
pub static mut value_56: std::cell::LazyCell<usize> = std::cell::LazyCell::new(|| unsafe {
    (if (((65536_u64) & (((1) as u64) << (7_u64))) != 0) {
        7_u64
    } else {
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut value_57)) as u64)
    } as usize)
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__7_ {}
pub static mut value_57: std::cell::LazyCell<usize> = std::cell::LazyCell::new(|| unsafe {
    (if (((65536_u64) & (((1) as u64) << (6_u64))) != 0) {
        6_u64
    } else {
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut value_58)) as u64)
    } as usize)
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__6_ {}
pub static mut value_58: std::cell::LazyCell<usize> = std::cell::LazyCell::new(|| unsafe {
    (if (((65536_u64) & (((1) as u64) << (5_u64))) != 0) {
        5_u64
    } else {
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut value_59)) as u64)
    } as usize)
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__5_ {}
pub static mut value_59: std::cell::LazyCell<usize> = std::cell::LazyCell::new(|| unsafe {
    (if (((65536_u64) & (((1) as u64) << (4_u64))) != 0) {
        4_u64
    } else {
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut value_60)) as u64)
    } as usize)
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__4_ {}
pub static mut value_60: std::cell::LazyCell<usize> = std::cell::LazyCell::new(|| unsafe {
    (if (((65536_u64) & (((1) as u64) << (3_u64))) != 0) {
        3_u64
    } else {
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut value_61)) as u64)
    } as usize)
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__3_ {}
pub static mut value_61: std::cell::LazyCell<usize> = std::cell::LazyCell::new(|| unsafe {
    (if (((65536_u64) & (((1) as u64) << (2_u64))) != 0) {
        2_u64
    } else {
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut value_62)) as u64)
    } as usize)
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__2_ {}
pub static mut value_62: std::cell::LazyCell<usize> = std::cell::LazyCell::new(|| unsafe {
    (if (((65536_u64) & (((1) as u64) << (1_u64))) != 0) {
        1_u64
    } else {
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut value_63)) as u64)
    } as usize)
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__1_ {}
pub static mut value_63: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_imp_unsigned_long_long__65536__0_ {}
pub static mut value_64: std::cell::LazyCell<usize> = std::cell::LazyCell::new(|| unsafe {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut value_0))
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_meta_log2_unsigned_long__65536_ {}
pub unsafe fn countl_zero_65(mut n: u32) -> i32 {
    return n.leading_zeros() as i32;
}
pub unsafe fn countl_zero_66(mut n: u64) -> i32 {
    return n.leading_zeros() as i32;
}
static mut rp_67: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    ((((unsafe { support_simple_random_generator::max() }) as i32)
        - ((unsafe { support_simple_random_generator::min() }) as i32)) as ())
        .wrapping_add((1_() as ()))
});
static mut rp_log2_68: std::cell::LazyCell<usize> = std::cell::LazyCell::new(|| unsafe {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut value_64))
});
static mut w_digits_69: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { (unimplemented!() as usize) });
static mut e_digits_70: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { (unimplemented!() as usize) });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct support_detail_independent_bits_engine_support_simple_random_generator__unsigned_long_ {
    eng_: *mut support_simple_random_generator,
    width_: usize,
    wid0_: usize,
    round_count_all_: usize,
    round_count_regular_: usize,
    y0_: (),
    y1_: (),
    mask0_: u16,
    mask1_: u16,
}
impl support_detail_independent_bits_engine_support_simple_random_generator__unsigned_long_ {
    pub unsafe fn new(eng: *mut support_simple_random_generator, mut width: usize) -> Self {
        let mut this = Self {
            eng_: eng,
            width_: width,
            wid0_: 0_usize,
            round_count_all_: 0_usize,
            round_count_regular_: 0_usize,
            y0_: unimplemented!(),
            y1_: unimplemented!(),
            mask0_: 0_u16,
            mask1_: 0_u16,
        };
        this.round_count_all_ = ((this.width_)
            .wrapping_div((*std::cell::LazyCell::force_mut(&mut *&raw mut rp_log2_68))))
        .wrapping_add(
            ((((this.width_)
                .wrapping_rem((*std::cell::LazyCell::force_mut(&mut *&raw mut rp_log2_68))))
                != (0_usize)) as usize),
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
            > ((this.y0_ as u64).wrapping_div((this.round_count_all_ as u64))))
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
            < ((*std::cell::LazyCell::force_mut(&mut *&raw mut w_digits_69)).wrapping_sub(1_usize)))
        {
            this.y1_ = (((*std::cell::LazyCell::force_mut(&mut *&raw mut rp_67))
                >> ((this.wid0_).wrapping_add(1_usize)))
                << ((this.wid0_).wrapping_add(1_usize)));
        } else {
            this.y1_ = 0_();
        }
        this.mask0_ = if ((this.wid0_) > (0_usize)) {
            ((((!0 as u16) as i32)
                >> ((*std::cell::LazyCell::force_mut(&mut *&raw mut e_digits_70))
                    .wrapping_sub(this.wid0_))) as u16)
        } else {
            0_u16
        };
        this.mask1_ = if ((this.wid0_)
            < ((*std::cell::LazyCell::force_mut(&mut *&raw mut e_digits_70)).wrapping_sub(1_usize)))
        {
            ((((!0 as u16) as i32)
                >> ((*std::cell::LazyCell::force_mut(&mut *&raw mut e_digits_70))
                    .wrapping_sub(((this.wid0_).wrapping_add(1_usize))))) as u16)
        } else {
            (!0 as u16)
        };
        this
    }
    pub unsafe fn operator_call(&mut self) -> u64 {
        return (unsafe {
            support_detail_independent_bits_engine_support_simple_random_generator__unsigned_long_ :: generate_4 ( self , unimplemented!() , )
        });
    }
    unsafe fn generate_4(&mut self, mut _a0: ()) -> u64 {
        let r_digits: usize = (unimplemented!() as usize);
        let mut result: u64 = 0_u64;
        let mut k: usize = 0_usize;
        'loop_: while ((k) < (self.round_count_regular_)) {
            let mut eng_result: u16 = 0_u16;
            let mut __do_while = true;
            'loop_: while __do_while || ((eng_result as ()) >= (self.y0_)) {
                __do_while = false;
                eng_result = ((((unsafe {
                    support_simple_random_generator::operator_call(&mut (*self.eng_))
                }) as i32)
                    - ((unsafe { support_simple_random_generator::min() }) as i32))
                    as u16);
            }
            if ((self.wid0_) < (r_digits)) {
                result <<= (self.wid0_ as u64);
            } else {
                result = 0_u64;
            }
            result = (result).wrapping_add((((eng_result as i32) & (self.mask0_ as i32)) as u64));
            k.prefix_inc();
        }
        let mut k: usize = self.round_count_regular_;
        'loop_: while ((k) < (self.round_count_all_)) {
            let mut eng_result: u16 = 0_u16;
            let mut __do_while = true;
            'loop_: while __do_while || ((eng_result as ()) >= (self.y1_)) {
                __do_while = false;
                eng_result = ((((unsafe {
                    support_simple_random_generator::operator_call(&mut (*self.eng_))
                }) as i32)
                    - ((unsafe { support_simple_random_generator::min() }) as i32))
                    as u16);
            }
            if ((self.wid0_) < ((r_digits).wrapping_sub(1_usize))) {
                result <<= ((self.wid0_).wrapping_add(1_usize) as u64);
            } else {
                result = 0_u64;
            }
            result = (result).wrapping_add((((eng_result as i32) & (self.mask1_ as i32)) as u64));
            k.prefix_inc();
        }
        return result;
    }
}
pub unsafe fn operator_eq_71(
    _a0: *const support_uniform_int_distribution_long_,
    _a1: *const support_uniform_int_distribution_long_,
) -> bool {
    return (unsafe {
        let _arg0: *const support_uniform_int_distribution_long__param_type = &(*_a0).param_;
        let _arg1: *const support_uniform_int_distribution_long__param_type = &(*_a1).param_;
        operator_eq_72(_arg0, _arg1)
    });
}
pub unsafe fn operator_eq_72(
    _a0: *const support_uniform_int_distribution_long__param_type,
    _a1: *const support_uniform_int_distribution_long__param_type,
) -> bool {
    return (((*_a0).a_) == ((*_a1).a_)) && (((*_a0).b_) == ((*_a1).b_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct support_uniform_int_distribution_long__param_type {
    a_: i64,
    b_: i64,
}
impl support_uniform_int_distribution_long__param_type {
    pub unsafe fn new_1() -> Self {
        let mut this = support_uniform_int_distribution_long__param_type::new_2({ 0_i64 }, None);
        this
    }
    pub unsafe fn new_2(mut ax: i64, mut bx: Option<i64>) -> Self {
        let mut bx: i64 = bx.unwrap_or_else(|| unsafe { i64::MAX });
        let mut this = Self { a_: ax, b_: bx };
        this
    }
    pub unsafe fn a(&self) -> i64 {
        return self.a_;
    }
    pub unsafe fn b(&self) -> i64 {
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
    ) -> i64 {
        const _: () = assert!(true, "detail::is_valid_urng<URNG>::value");;
        let rp: () =
            (((unsafe { support_uniform_int_distribution_long__param_type::b(&(*param)) }) as ())
                .wrapping_sub(
                    ((unsafe { support_uniform_int_distribution_long__param_type::a(&(*param)) })
                        as ()),
                ))
            .wrapping_add((1_() as ()));
        if ((rp) == (1_())) {
            return (unsafe { support_uniform_int_distribution_long__param_type::a(&(*param)) });
        }
        let ur_digits: usize = (unimplemented!() as usize);
        if ((rp) == (0_())) {
            return ((unsafe {
                support_detail_independent_bits_engine_support_simple_random_generator__unsigned_long_ :: operator_call ( &mut support_detail_independent_bits_engine_support_simple_random_generator__unsigned_long_ :: new ( { gen_ } , { ur_digits } , )   , )
            }) as i64);
        }
        let mut width: usize = ((ur_digits)
            .wrapping_sub(((unsafe { countl_zero_66((rp as u64)) }) as usize)))
        .wrapping_sub(1_usize);
        if (((rp as u64) & ((u64::MAX) >> ((ur_digits).wrapping_sub(width)))) != (0_u64)) {
            width.prefix_inc();
        }
        let mut eng : support_detail_independent_bits_engine_support_simple_random_generator__unsigned_long_ = support_detail_independent_bits_engine_support_simple_random_generator__unsigned_long_ :: new ( { gen_ } , { width } , )  ;
        let mut eng_result: () = 0_();
        let mut __do_while = true;
        'loop_: while __do_while || ((eng_result) >= (rp)) {
            __do_while = false;
            eng_result = ((unsafe {
                support_detail_independent_bits_engine_support_simple_random_generator__unsigned_long_ :: operator_call ( &mut eng  , )
            }) as ());
        }
        return (((eng_result).wrapping_add(
            ((unsafe { support_uniform_int_distribution_long__param_type::a(&(*param)) }) as ()),
        )) as i64);
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
        return (-1_i32 as u16);
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
        self.status_ =
            ((self.status_).wrapping_mul((214013_u32 as u32))).wrapping_add((2531011_u32 as u32));
        return (((self.status_) >> (16)) as u16);
    }
}
impl Default for support_simple_random_generator {
    fn default() -> Self {
        unsafe { support_simple_random_generator::new_1() }
    }
}
pub unsafe fn shuffle_74(
    mut first: *mut Option<Box<i32>>,
    mut last: *mut Option<Box<i32>>,
    gen_: *mut support_simple_random_generator,
) {
    let mut last_iter: *mut Option<Box<i32>> = last;
    let mut diff: isize = (((last_iter as usize - first as usize)
        / ::std::mem::size_of::<Option<Box<i32>>>()) as i64 as isize);
    if ((diff) > (1_isize)) {
        let mut uid: support_uniform_int_distribution_long_ =
            <support_uniform_int_distribution_long_>::default();
        {
            last_iter.prefix_dec();
            &(diff.prefix_dec())
        };
        'loop_: while ((first) < (last)) {
            let mut index: isize = ((unsafe {
                let _gen_: *mut support_simple_random_generator = gen_;
                let mut _param: support_uniform_int_distribution_long__param_type =
                    support_uniform_int_distribution_long__param_type::new_2({ 0_i64 }, {
                        Some((diff as i64))
                    });
                support_uniform_int_distribution_long_ :: operator_call_6_class_support_simple_random_generator ( &mut uid  , _gen_ , & mut _param , )
            }) as isize);
            if ((index) != (0_isize)) {
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
    mut first: *mut i32,
    mut last: *mut i32,
    gen_: *mut support_simple_random_generator,
) {
    let mut last_iter: *mut i32 = last;
    let mut diff: isize =
        (((last_iter as usize - first as usize) / ::std::mem::size_of::<i32>()) as i64 as isize);
    if ((diff) > (1_isize)) {
        let mut uid: support_uniform_int_distribution_long_ =
            <support_uniform_int_distribution_long_>::default();
        {
            last_iter.prefix_dec();
            &(diff.prefix_dec())
        };
        'loop_: while ((first) < (last)) {
            let mut index: isize = ((unsafe {
                let _gen_: *mut support_simple_random_generator = gen_;
                let mut _param: support_uniform_int_distribution_long__param_type =
                    support_uniform_int_distribution_long__param_type::new_2({ 0_i64 }, {
                        Some((diff as i64))
                    });
                support_uniform_int_distribution_long_ :: operator_call_6_class_support_simple_random_generator ( &mut uid  , _gen_ , & mut _param , )
            }) as isize);
            if ((index) != (0_isize)) {
                cpp_swap_unsafe!(first, (first.offset((index) as isize)));
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
    let mut diff: isize =
        (((last_iter as usize - first as usize) / ::std::mem::size_of::<S>()) as i64 as isize);
    if ((diff) > (1_isize)) {
        let mut uid: support_uniform_int_distribution_long_ =
            <support_uniform_int_distribution_long_>::default();
        {
            last_iter.prefix_dec();
            &(diff.prefix_dec())
        };
        'loop_: while ((first) < (last)) {
            let mut index: isize = ((unsafe {
                let _gen_: *mut support_simple_random_generator = gen_;
                let mut _param: support_uniform_int_distribution_long__param_type =
                    support_uniform_int_distribution_long__param_type::new_2({ 0_i64 }, {
                        Some((diff as i64))
                    });
                support_uniform_int_distribution_long_ :: operator_call_6_class_support_simple_random_generator ( &mut uid  , _gen_ , & mut _param , )
            }) as isize);
            if ((index) != (0_isize)) {
                cpp_swap_unsafe!(first, (first.offset((index) as isize)));
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
        x: *const Option<Box<i32>>,
        y: *const Option<Box<i32>>,
    ) -> bool {
        return ((*(*x)
            .as_deref()
            .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
            < (*(*y)
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32)));
    }
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, CppFromUnsafe, CopyAssignUnsafe, MoveAssignUnsafe)]
#[cpp_from(i32, new_1, value)]
pub struct S {
    pub i_: i32,
}
impl S {
    pub unsafe fn new_2() -> Self {
        let mut this = Self { i_: 0 };
        this
    }
    pub unsafe fn new_1(mut i: i32) -> Self {
        let mut this = Self { i_: i };
        this
    }
    pub unsafe fn copy_from(rhs: *const S) -> Self {
        let mut this = Self { i_: (*rhs).i_ };
        this
    }
    pub unsafe fn move_from(rhs: *mut S) -> Self {
        let mut this = Self { i_: (*rhs).i_ };
        (*rhs).i_ = -1_i32;
        this
    }
    pub unsafe fn copy_assign(&mut self, rhs: *const S) -> *mut S {
        self.i_ = (*rhs).i_;
        return &mut (*(self as *mut S));
    }
    pub unsafe fn move_assign(&mut self, rhs: *mut S) -> *mut S {
        self.i_ = (*rhs).i_;
        (*rhs).i_ = -2_i32;
        assert!(((self as *mut S) != (rhs)));
        return &mut (*(self as *mut S));
    }
    pub unsafe fn operator_assign_7(&mut self, mut i: i32) -> *mut S {
        self.i_ = i;
        return &mut (*(self as *mut S));
    }
    pub unsafe fn operator_lt(&self, rhs: *const S) -> bool {
        return ((self.i_) < ((*rhs).i_));
    }
    pub unsafe fn operator_gt(&self, rhs: *const S) -> bool {
        return ((self.i_) > ((*rhs).i_));
    }
    pub unsafe fn operator_eq_10(&self, rhs: *const S) -> bool {
        return ((self.i_) == ((*rhs).i_));
    }
    pub unsafe fn operator_eq_11(&self, mut i: i32) -> bool {
        return ((self.i_) == (i));
    }
    pub unsafe fn set(&mut self, mut i: i32) {
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
        unsafe { S::new_2() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe)]
pub struct support_double_move_tracker {
    moved_from_: bool,
}
impl support_double_move_tracker {
    pub unsafe fn new() -> Self {
        let mut this = Self { moved_from_: false };
        this
    }
    pub unsafe fn move_from(other: *mut support_double_move_tracker) -> Self {
        let mut this = Self { moved_from_: false };
        assert!(!((*other).moved_from_));
        (*other).moved_from_ = true;
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        other: *mut support_double_move_tracker,
    ) -> *mut support_double_move_tracker {
        assert!(!((*other).moved_from_));
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
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct cpp17_output_iterator_int_ptr_ {
    it_: *mut i32,
    tracker_: support_double_move_tracker,
}
impl cpp17_output_iterator_int_ptr_ {
    pub unsafe fn move_from(_a0: *mut cpp17_output_iterator_int_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut cpp17_output_iterator_int_ptr_,
    ) -> *mut cpp17_output_iterator_int_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut cpp17_output_iterator_int_ptr_));
    }
}
const _: () = assert!(
    true,
    "std::output_iterator<cpp17_output_iterator<int*>, int>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct cpp17_input_iterator_int_ptr__int_ptr_ {
    it_: *mut i32,
    tracker_: support_double_move_tracker,
}
impl cpp17_input_iterator_int_ptr__int_ptr_ {
    pub unsafe fn move_from(_a0: *mut cpp17_input_iterator_int_ptr__int_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut cpp17_input_iterator_int_ptr__int_ptr_,
    ) -> *mut cpp17_input_iterator_int_ptr__int_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut cpp17_input_iterator_int_ptr__int_ptr_));
    }
}
const _: () = assert!(true, "std::input_iterator<cpp17_input_iterator<int*>>");
pub unsafe fn operator_eq_77(
    x: *const bidirectional_iterator_int_ptr_,
    y: *const bidirectional_iterator_int_ptr_,
) -> bool {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_78(
    x: *const bidirectional_iterator_int_ptr_,
    y: *const bidirectional_iterator_int_ptr_,
) -> bool {
    return (((*x).it_) != ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct bidirectional_iterator_int_ptr_ {
    it_: *mut i32,
    tracker_: support_double_move_tracker,
}
impl bidirectional_iterator_int_ptr_ {
    pub unsafe fn new_1(mut it: *mut i32) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> *mut i32 {
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
impl UnsafeCppAdvance for bidirectional_iterator_int_ptr_ {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_6();
    }
    unsafe fn cpp_dec(&mut self) {
        self.operator_dec_7();
    }
}
pub unsafe fn operator_eq_79(
    x: *const bidirectional_iterator_S_ptr_,
    y: *const bidirectional_iterator_S_ptr_,
) -> bool {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_80(
    x: *const bidirectional_iterator_S_ptr_,
    y: *const bidirectional_iterator_S_ptr_,
) -> bool {
    return (((*x).it_) != ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct bidirectional_iterator_S_ptr_ {
    it_: *mut S,
    tracker_: support_double_move_tracker,
}
impl bidirectional_iterator_S_ptr_ {
    pub unsafe fn new_1(mut it: *mut S) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> *mut S {
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
impl UnsafeCppAdvance for bidirectional_iterator_S_ptr_ {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_6();
    }
    unsafe fn cpp_dec(&mut self) {
        self.operator_dec_7();
    }
}
pub unsafe fn operator_add_81(
    mut x: random_access_iterator_int_ptr_,
    mut n: isize,
) -> random_access_iterator_int_ptr_ {
    (unsafe { random_access_iterator_int_ptr_::operator_add_assign(&mut x, n) });
    return random_access_iterator_int_ptr_::move_from({ &mut x });
}
pub unsafe fn operator_sub_82(
    mut x: random_access_iterator_int_ptr_,
    mut n: isize,
) -> random_access_iterator_int_ptr_ {
    (unsafe { random_access_iterator_int_ptr_::operator_sub_assign(&mut x, n) });
    return random_access_iterator_int_ptr_::move_from({ &mut x });
}
pub unsafe fn operator_sub_83(
    mut x: random_access_iterator_int_ptr_,
    mut y: random_access_iterator_int_ptr_,
) -> isize {
    return (((x.it_ as usize - y.it_ as usize) / ::std::mem::size_of::<i32>()) as i64 as isize);
}
pub unsafe fn operator_eq_84(
    x: *const random_access_iterator_int_ptr_,
    y: *const random_access_iterator_int_ptr_,
) -> bool {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_85(
    x: *const random_access_iterator_int_ptr_,
    y: *const random_access_iterator_int_ptr_,
) -> bool {
    return (((*x).it_) != ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct random_access_iterator_int_ptr_ {
    it_: *mut i32,
    tracker_: support_double_move_tracker,
}
impl random_access_iterator_int_ptr_ {
    pub unsafe fn new_1(mut it: *mut i32) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> *mut i32 {
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
        mut n: isize,
    ) -> *mut random_access_iterator_int_ptr_ {
        self.it_ = (self.it_).wrapping_add(n as usize);
        return &mut (*(self as *mut random_access_iterator_int_ptr_));
    }
    pub unsafe fn operator_sub_assign(
        &mut self,
        mut n: isize,
    ) -> *mut random_access_iterator_int_ptr_ {
        self.it_ = (self.it_).wrapping_sub(n as usize);
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
            operator_eq_84(
                self as *const random_access_iterator_int_ptr_,
                other as *const random_access_iterator_int_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for random_access_iterator_int_ptr_ {}
impl UnsafeCppAdvance for random_access_iterator_int_ptr_ {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_7();
    }
    unsafe fn cpp_dec(&mut self) {
        self.operator_dec_8();
    }
}
pub unsafe fn operator_add_86(
    mut x: random_access_iterator_S_ptr_,
    mut n: isize,
) -> random_access_iterator_S_ptr_ {
    (unsafe { random_access_iterator_S_ptr_::operator_add_assign(&mut x, n) });
    return random_access_iterator_S_ptr_::move_from({ &mut x });
}
pub unsafe fn operator_sub_87(
    mut x: random_access_iterator_S_ptr_,
    mut n: isize,
) -> random_access_iterator_S_ptr_ {
    (unsafe { random_access_iterator_S_ptr_::operator_sub_assign(&mut x, n) });
    return random_access_iterator_S_ptr_::move_from({ &mut x });
}
pub unsafe fn operator_sub_88(
    mut x: random_access_iterator_S_ptr_,
    mut y: random_access_iterator_S_ptr_,
) -> isize {
    return (((x.it_ as usize - y.it_ as usize) / ::std::mem::size_of::<S>()) as i64 as isize);
}
pub unsafe fn operator_eq_89(
    x: *const random_access_iterator_S_ptr_,
    y: *const random_access_iterator_S_ptr_,
) -> bool {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_90(
    x: *const random_access_iterator_S_ptr_,
    y: *const random_access_iterator_S_ptr_,
) -> bool {
    return (((*x).it_) != ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct random_access_iterator_S_ptr_ {
    it_: *mut S,
    tracker_: support_double_move_tracker,
}
impl random_access_iterator_S_ptr_ {
    pub unsafe fn new_1(mut it: *mut S) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> *mut S {
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
    pub unsafe fn operator_add_assign(
        &mut self,
        mut n: isize,
    ) -> *mut random_access_iterator_S_ptr_ {
        self.it_ = (self.it_).wrapping_add(n as usize);
        return &mut (*(self as *mut random_access_iterator_S_ptr_));
    }
    pub unsafe fn operator_sub_assign(
        &mut self,
        mut n: isize,
    ) -> *mut random_access_iterator_S_ptr_ {
        self.it_ = (self.it_).wrapping_sub(n as usize);
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
            operator_eq_89(
                self as *const random_access_iterator_S_ptr_,
                other as *const random_access_iterator_S_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for random_access_iterator_S_ptr_ {}
impl UnsafeCppAdvance for random_access_iterator_S_ptr_ {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_7();
    }
    unsafe fn cpp_dec(&mut self) {
        self.operator_dec_8();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct cpp20_random_access_iterator_int_ptr_ {
    it_: *mut i32,
    tracker_: support_double_move_tracker,
}
impl cpp20_random_access_iterator_int_ptr_ {
    pub unsafe fn operator_deref(&self) -> *mut i32 {
        return &mut (*self.it_);
    }
    pub unsafe fn operator_index(&self, mut n: isize) -> *mut i32 {
        return &mut (*self.it_.offset((n) as isize));
    }
    pub unsafe fn move_from(_a0: *mut cpp20_random_access_iterator_int_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut cpp20_random_access_iterator_int_ptr_,
    ) -> *mut cpp20_random_access_iterator_int_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut cpp20_random_access_iterator_int_ptr_));
    }
}
const _: () = assert!(
    true,
    "std::random_access_iterator<cpp20_random_access_iterator<int*>>"
);
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct cpp20_input_iterator_int_ptr_ {
    it_: *mut i32,
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
    pub unsafe fn operator_deref(&self) -> *mut i32 {
        return &mut (*self.it_);
    }
}
const _: () = assert!(true, "std::input_iterator<cpp20_input_iterator<int*>>");
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct cpp20_output_iterator_int_ptr_ {
    it_: *mut i32,
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
    pub unsafe fn operator_deref(&self) -> *mut i32 {
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
    pub increments: usize,
    pub decrements: usize,
    pub zero_moves: usize,
    pub equal_cmps: usize,
}
impl Default for IteratorOpCounts {
    fn default() -> Self {
        unsafe {
            IteratorOpCounts {
                increments: 0_usize,
                decrements: 0_usize,
                zero_moves: 0_usize,
                equal_cmps: 0_usize,
            }
        }
    }
}
pub unsafe fn operator_eq_91(
    _a0: *const forward_sized_iterator_int_ptr_,
    _a1: *const forward_sized_iterator_int_ptr_,
) -> bool {
    return (((*_a0).it_) == ((*_a1).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct forward_sized_iterator_int_ptr_ {
    pub it_: *mut i32,
}
impl forward_sized_iterator_int_ptr_ {}
impl std::cmp::PartialEq for forward_sized_iterator_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_91(
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
pub unsafe fn operator_add_92(mut i: adl_Iterator, mut n: isize) -> adl_Iterator {
    return adl_Iterator::new_1({ i.ptr_.offset((n) as isize) }, { i.iter_moves_ }, {
        i.iter_swaps_
    });
}
pub unsafe fn operator_add_93(mut n: isize, mut i: adl_Iterator) -> adl_Iterator {
    return (unsafe {
        let _i: adl_Iterator = i;
        operator_add_92(_i, n)
    });
}
pub unsafe fn iter_swap_94(mut a: adl_Iterator, mut b: adl_Iterator) {
    cpp_swap_unsafe!(a.ptr_, b.ptr_);
    if !(a.iter_swaps_).is_null() {
        (*a.iter_swaps_).prefix_inc();
    }
}
pub unsafe fn iter_move_95(mut iter: adl_Iterator) -> *mut i32 {
    if !(iter.iter_moves_).is_null() {
        (*iter.iter_moves_).prefix_inc();
    }
    return &mut (*(unsafe { adl_Iterator::operator_deref(&iter) }));
}
pub unsafe fn operator_eq_96(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> bool {
    return (((*lhs).ptr_) == ((*rhs).ptr_));
}
pub unsafe fn operator_cmp_97(
    lhs: *const adl_Iterator,
    rhs: *const adl_Iterator,
) -> std::cmp::Ordering {
    return std::cmp::Ord::cmp(&((*lhs).ptr_), &((*rhs).ptr_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct adl_Iterator {
    ptr_: *mut i32,
    iter_moves_: *mut i32,
    iter_swaps_: *mut i32,
}
impl adl_Iterator {
    unsafe fn new_1(mut p: *mut i32, mut iter_moves: *mut i32, mut iter_swaps: *mut i32) -> Self {
        let mut this = Self {
            ptr_: p,
            iter_moves_: iter_moves,
            iter_swaps_: iter_swaps,
        };
        this
    }
    pub unsafe fn TrackMoves(mut p: *mut i32, iter_moves: *mut i32) -> adl_Iterator {
        return adl_Iterator::new_1({ p }, { (iter_moves) }, { std::ptr::null_mut() });
    }
    pub unsafe fn TrackSwaps_4(iter_swaps: *mut i32) -> adl_Iterator {
        return adl_Iterator::new_1({ std::ptr::null_mut() }, { std::ptr::null_mut() }, {
            (iter_swaps)
        });
    }
    pub unsafe fn TrackSwaps_5(mut p: *mut i32, iter_swaps: *mut i32) -> adl_Iterator {
        return adl_Iterator::new_1({ p }, { std::ptr::null_mut() }, { (iter_swaps) });
    }
    pub unsafe fn iter_moves(&self) -> i32 {
        assert!(!(self.iter_moves_).is_null());
        return (*self.iter_moves_);
    }
    pub unsafe fn iter_swaps(&self) -> i32 {
        assert!(!(self.iter_swaps_).is_null());
        return (*self.iter_swaps_);
    }
    pub unsafe fn operator_deref(&self) -> *mut i32 {
        return &mut (*self.ptr_);
    }
    pub unsafe fn operator_index(&self, mut n: isize) -> *mut i32 {
        return &mut (*self.ptr_.offset((n) as isize));
    }
    pub unsafe fn operator_sub_10(&self, mut n: isize) -> adl_Iterator {
        return adl_Iterator::new_1(
            { self.ptr_.offset(-((n) as isize)) },
            { self.iter_moves_ },
            { self.iter_swaps_ },
        );
    }
    pub unsafe fn operator_sub_11(&self, mut rhs: adl_Iterator) -> isize {
        return (((self.ptr_ as usize - rhs.ptr_ as usize) / ::std::mem::size_of::<i32>()) as i64
            as isize);
    }
    pub unsafe fn operator_add_assign(&mut self, mut n: isize) -> *mut adl_Iterator {
        self.ptr_ = (self.ptr_).wrapping_add(n as usize);
        return &mut (*(self as *mut adl_Iterator));
    }
    pub unsafe fn operator_sub_assign(&mut self, mut n: isize) -> *mut adl_Iterator {
        self.ptr_ = (self.ptr_).wrapping_sub(n as usize);
        return &mut (*(self as *mut adl_Iterator));
    }
    pub unsafe fn operator_inc_14(&mut self) -> *mut adl_Iterator {
        self.ptr_.prefix_inc();
        return &mut (*(self as *mut adl_Iterator));
    }
    pub unsafe fn operator_post_inc_15(&mut self, mut _a0: i32) -> adl_Iterator {
        let mut prev: adl_Iterator = (*(self as *mut adl_Iterator));
        self.ptr_.prefix_inc();
        return prev;
    }
    pub unsafe fn operator_dec_16(&mut self) -> *mut adl_Iterator {
        self.ptr_.prefix_dec();
        return &mut (*(self as *mut adl_Iterator));
    }
    pub unsafe fn operator_post_dec_17(&mut self, mut _a0: i32) -> adl_Iterator {
        let mut prev: adl_Iterator = (*(self as *mut adl_Iterator));
        self.ptr_.prefix_dec();
        return prev;
    }
}
impl std::cmp::Ord for adl_Iterator {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { operator_cmp_97(self as *const adl_Iterator, other as *const adl_Iterator) }
    }
}
impl std::cmp::PartialOrd for adl_Iterator {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for adl_Iterator {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_96(self as *const adl_Iterator, other as *const adl_Iterator) }
    }
}
impl std::cmp::Eq for adl_Iterator {}
impl UnsafeCppAdvance for adl_Iterator {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_14();
    }
    unsafe fn cpp_dec(&mut self) {
        self.operator_dec_16();
    }
}
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
    it_: *mut *mut i32,
}
impl rvalue_iterator_int_ptr_ {
    pub unsafe fn operator_cmp(&self, _a0: *const rvalue_iterator_int_ptr_) -> std::cmp::Ordering {
        {
            let mut cmp: std::cmp::Ordering = std::cmp::Ord::cmp(
                &((*(self as *const rvalue_iterator_int_ptr_)).it_),
                &((*_a0).it_),
            );
            if !(cmp == std::cmp::Ordering::Equal) {
                return cmp;
            }
        }
        return std::cmp::Ordering::Equal;
    }
    pub unsafe fn operator_eq(&self, _a0: *const rvalue_iterator_int_ptr_) -> bool {
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
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(*const libc::c_char , new, value)]
pub struct util_iterator_error {}
impl util_iterator_error {
    pub unsafe fn new(mut what: *const libc::c_char) -> Self {
        let mut this = Self {};
        this
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, CppFromUnsafe)]
#[cpp_from(() , new, value)]
pub struct binary_counting_predicate_std_greater_int___int__int_ {
    p_: (),
    count_: usize,
}
impl binary_counting_predicate_std_greater_int___int__int_ {
    pub unsafe fn new(mut p: ()) -> Self {
        let mut this = Self {
            p_: p.clone(),
            count_: 0_usize,
        };
        this
    }
    pub unsafe fn operator_call(&self, a1: *const i32, a2: *const i32) -> bool {
        (*(&raw const self.count_).cast_mut()).prefix_inc();
        return unimplemented!();
    }
}
impl Default for binary_counting_predicate_std_greater_int___int__int_ {
    fn default() -> Self {
        unsafe {
            binary_counting_predicate_std_greater_int___int__int_ {
                p_: unimplemented!(),
                count_: 0_usize,
            }
        }
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, CppFromUnsafe)]
#[cpp_from(() , new, value)]
pub struct binary_counting_predicate_std_greater_S___S__S_ {
    p_: (),
    count_: usize,
}
impl binary_counting_predicate_std_greater_S___S__S_ {
    pub unsafe fn new(mut p: ()) -> Self {
        let mut this = Self {
            p_: p.clone(),
            count_: 0_usize,
        };
        this
    }
    pub unsafe fn operator_call(&self, a1: *const S, a2: *const S) -> bool {
        (*(&raw const self.count_).cast_mut()).prefix_inc();
        return unimplemented!();
    }
}
impl Default for binary_counting_predicate_std_greater_S___S__S_ {
    fn default() -> Self {
        unsafe {
            binary_counting_predicate_std_greater_S___S__S_ {
                p_: unimplemented!(),
                count_: 0_usize,
            }
        }
    }
}
pub unsafe fn test_one_98(
    mut N: u32,
    mut M: u32,
    randomness: *mut support_simple_random_generator,
) {
    assert!(((M) <= (N)));
    let mut ia: *mut i32 =
        Box::leak((0..(N as usize)).map(|_| 0_i32).collect::<Box<[i32]>>()).as_mut_ptr();
    let mut i: u32 = 0_u32;
    'loop_: while ((i) < (N)) {
        (*ia.offset((i) as isize)) = (i as i32);
        i.prefix_inc();
    }
    (unsafe {
        let _first: *mut i32 = ia;
        let _last: *mut i32 = ia.offset((N) as isize);
        let _gen_: *mut support_simple_random_generator = randomness;
        shuffle_75(_first, _last, _gen_)
    });
    unimplemented!();
    unimplemented!();
    let mut pred: binary_counting_predicate_std_greater_int___int__int_ =
        binary_counting_predicate_std_greater_int___int__int_::new({ (unimplemented!()) });
    unimplemented!();
    if ((N) > (0_u32)) {
        assert!(((*ia.offset((0) as isize)) == ((N as i32) - (1))));
        assert!(((*ia.offset(((N).wrapping_sub(1_u32)) as isize)) == (0)));
        assert!(unimplemented!());
    }
    {
        let __p = ia;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                __p,
                libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                    / ::std::mem::size_of::<i32>(),
            )))
        }
    };
}
pub unsafe fn test_one_99(
    mut N: u32,
    mut M: u32,
    randomness: *mut support_simple_random_generator,
) {
    assert!(((M) <= (N)));
    let mut ia: *mut i32 =
        Box::leak((0..(N as usize)).map(|_| 0_i32).collect::<Box<[i32]>>()).as_mut_ptr();
    let mut i: u32 = 0_u32;
    'loop_: while ((i) < (N)) {
        (*ia.offset((i) as isize)) = (i as i32);
        i.prefix_inc();
    }
    (unsafe {
        let _first: *mut i32 = ia;
        let _last: *mut i32 = ia.offset((N) as isize);
        let _gen_: *mut support_simple_random_generator = randomness;
        shuffle_75(_first, _last, _gen_)
    });
    unimplemented!();
    unimplemented!();
    let mut pred: binary_counting_predicate_std_greater_int___int__int_ =
        binary_counting_predicate_std_greater_int___int__int_::new({ (unimplemented!()) });
    unimplemented!();
    if ((N) > (0_u32)) {
        assert!(((*ia.offset((0) as isize)) == ((N as i32) - (1))));
        assert!(((*ia.offset(((N).wrapping_sub(1_u32)) as isize)) == (0)));
        assert!(unimplemented!());
    }
    {
        let __p = ia;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                __p,
                libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                    / ::std::mem::size_of::<i32>(),
            )))
        }
    };
}
pub unsafe fn test_one_100(
    mut N: u32,
    mut M: u32,
    randomness: *mut support_simple_random_generator,
) {
    assert!(((M) <= (N)));
    let mut ia: *mut i32 =
        Box::leak((0..(N as usize)).map(|_| 0_i32).collect::<Box<[i32]>>()).as_mut_ptr();
    let mut i: u32 = 0_u32;
    'loop_: while ((i) < (N)) {
        (*ia.offset((i) as isize)) = (i as i32);
        i.prefix_inc();
    }
    (unsafe {
        let _first: *mut i32 = ia;
        let _last: *mut i32 = ia.offset((N) as isize);
        let _gen_: *mut support_simple_random_generator = randomness;
        shuffle_75(_first, _last, _gen_)
    });
    unimplemented!();
    unimplemented!();
    let mut pred: binary_counting_predicate_std_greater_int___int__int_ =
        binary_counting_predicate_std_greater_int___int__int_::new({ (unimplemented!()) });
    unimplemented!();
    if ((N) > (0_u32)) {
        assert!(((*ia.offset((0) as isize)) == ((N as i32) - (1))));
        assert!(((*ia.offset(((N).wrapping_sub(1_u32)) as isize)) == (0)));
        assert!(unimplemented!());
    }
    {
        let __p = ia;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                __p,
                libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                    / ::std::mem::size_of::<i32>(),
            )))
        }
    };
}
pub unsafe fn test_one_101(
    mut N: u32,
    mut M: u32,
    randomness: *mut support_simple_random_generator,
) {
    assert!(((M) <= (N)));
    let mut ia: *mut S = Box::leak(
        (0..(N as usize))
            .map(|_| <S>::default())
            .collect::<Box<[S]>>(),
    )
    .as_mut_ptr();
    let mut i: u32 = 0_u32;
    'loop_: while ((i) < (N)) {
        (unsafe {
            let _i: i32 = (i as i32);
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
    if ((N) > (0_u32)) {
        assert!(
            (unsafe {
                let _i: i32 = ((N as i32) - (1));
                S::operator_eq_11(&(*ia.offset((0) as isize)), _i)
            })
        );
        assert!(
            (unsafe { S::operator_eq_11(&(*ia.offset(((N).wrapping_sub(1_u32)) as isize)), 0,) })
        );
        assert!(unimplemented!());
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
pub unsafe fn test_one_102(
    mut N: u32,
    mut M: u32,
    randomness: *mut support_simple_random_generator,
) {
    assert!(((M) <= (N)));
    let mut ia: *mut S = Box::leak(
        (0..(N as usize))
            .map(|_| <S>::default())
            .collect::<Box<[S]>>(),
    )
    .as_mut_ptr();
    let mut i: u32 = 0_u32;
    'loop_: while ((i) < (N)) {
        (unsafe {
            let _i: i32 = (i as i32);
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
    if ((N) > (0_u32)) {
        assert!(
            (unsafe {
                let _i: i32 = ((N as i32) - (1));
                S::operator_eq_11(&(*ia.offset((0) as isize)), _i)
            })
        );
        assert!(
            (unsafe { S::operator_eq_11(&(*ia.offset(((N).wrapping_sub(1_u32)) as isize)), 0,) })
        );
        assert!(unimplemented!());
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
pub unsafe fn test_one_103(
    mut N: u32,
    mut M: u32,
    randomness: *mut support_simple_random_generator,
) {
    assert!(((M) <= (N)));
    let mut ia: *mut S = Box::leak(
        (0..(N as usize))
            .map(|_| <S>::default())
            .collect::<Box<[S]>>(),
    )
    .as_mut_ptr();
    let mut i: u32 = 0_u32;
    'loop_: while ((i) < (N)) {
        (unsafe {
            let _i: i32 = (i as i32);
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
    if ((N) > (0_u32)) {
        assert!(
            (unsafe {
                let _i: i32 = ((N as i32) - (1));
                S::operator_eq_11(&(*ia.offset((0) as isize)), _i)
            })
        );
        assert!(
            (unsafe { S::operator_eq_11(&(*ia.offset(((N).wrapping_sub(1_u32)) as isize)), 0,) })
        );
        assert!(unimplemented!());
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
pub unsafe fn test_104(mut N: u32, randomness: *mut support_simple_random_generator) {
    (unsafe {
        let _N: u32 = N;
        let _randomness: *mut support_simple_random_generator = randomness;
        test_one_98(_N, 0_u32, _randomness)
    });
    (unsafe {
        let _N: u32 = N;
        let _M: u32 = (N).wrapping_div(4_u32);
        let _randomness: *mut support_simple_random_generator = randomness;
        test_one_98(_N, _M, _randomness)
    });
    (unsafe {
        let _N: u32 = N;
        let _M: u32 = (N).wrapping_div(2_u32);
        let _randomness: *mut support_simple_random_generator = randomness;
        test_one_98(_N, _M, _randomness)
    });
    (unsafe {
        let _N: u32 = N;
        let _M: u32 = ((3_u32).wrapping_mul(N)).wrapping_div(4_u32);
        let _randomness: *mut support_simple_random_generator = randomness;
        test_one_98(_N, _M, _randomness)
    });
    (unsafe {
        let _N: u32 = N;
        let _M: u32 = N;
        let _randomness: *mut support_simple_random_generator = randomness;
        test_one_98(_N, _M, _randomness)
    });
}
pub unsafe fn test_105(mut N: u32, randomness: *mut support_simple_random_generator) {
    (unsafe {
        let _N: u32 = N;
        let _randomness: *mut support_simple_random_generator = randomness;
        test_one_99(_N, 0_u32, _randomness)
    });
    (unsafe {
        let _N: u32 = N;
        let _M: u32 = (N).wrapping_div(4_u32);
        let _randomness: *mut support_simple_random_generator = randomness;
        test_one_99(_N, _M, _randomness)
    });
    (unsafe {
        let _N: u32 = N;
        let _M: u32 = (N).wrapping_div(2_u32);
        let _randomness: *mut support_simple_random_generator = randomness;
        test_one_99(_N, _M, _randomness)
    });
    (unsafe {
        let _N: u32 = N;
        let _M: u32 = ((3_u32).wrapping_mul(N)).wrapping_div(4_u32);
        let _randomness: *mut support_simple_random_generator = randomness;
        test_one_99(_N, _M, _randomness)
    });
    (unsafe {
        let _N: u32 = N;
        let _M: u32 = N;
        let _randomness: *mut support_simple_random_generator = randomness;
        test_one_99(_N, _M, _randomness)
    });
}
pub unsafe fn test_106(mut N: u32, randomness: *mut support_simple_random_generator) {
    (unsafe {
        let _N: u32 = N;
        let _randomness: *mut support_simple_random_generator = randomness;
        test_one_100(_N, 0_u32, _randomness)
    });
    (unsafe {
        let _N: u32 = N;
        let _M: u32 = (N).wrapping_div(4_u32);
        let _randomness: *mut support_simple_random_generator = randomness;
        test_one_100(_N, _M, _randomness)
    });
    (unsafe {
        let _N: u32 = N;
        let _M: u32 = (N).wrapping_div(2_u32);
        let _randomness: *mut support_simple_random_generator = randomness;
        test_one_100(_N, _M, _randomness)
    });
    (unsafe {
        let _N: u32 = N;
        let _M: u32 = ((3_u32).wrapping_mul(N)).wrapping_div(4_u32);
        let _randomness: *mut support_simple_random_generator = randomness;
        test_one_100(_N, _M, _randomness)
    });
    (unsafe {
        let _N: u32 = N;
        let _M: u32 = N;
        let _randomness: *mut support_simple_random_generator = randomness;
        test_one_100(_N, _M, _randomness)
    });
}
pub unsafe fn test_107(mut N: u32, randomness: *mut support_simple_random_generator) {
    (unsafe {
        let _N: u32 = N;
        let _randomness: *mut support_simple_random_generator = randomness;
        test_one_101(_N, 0_u32, _randomness)
    });
    (unsafe {
        let _N: u32 = N;
        let _M: u32 = (N).wrapping_div(4_u32);
        let _randomness: *mut support_simple_random_generator = randomness;
        test_one_101(_N, _M, _randomness)
    });
    (unsafe {
        let _N: u32 = N;
        let _M: u32 = (N).wrapping_div(2_u32);
        let _randomness: *mut support_simple_random_generator = randomness;
        test_one_101(_N, _M, _randomness)
    });
    (unsafe {
        let _N: u32 = N;
        let _M: u32 = ((3_u32).wrapping_mul(N)).wrapping_div(4_u32);
        let _randomness: *mut support_simple_random_generator = randomness;
        test_one_101(_N, _M, _randomness)
    });
    (unsafe {
        let _N: u32 = N;
        let _M: u32 = N;
        let _randomness: *mut support_simple_random_generator = randomness;
        test_one_101(_N, _M, _randomness)
    });
}
pub unsafe fn test_108(mut N: u32, randomness: *mut support_simple_random_generator) {
    (unsafe {
        let _N: u32 = N;
        let _randomness: *mut support_simple_random_generator = randomness;
        test_one_102(_N, 0_u32, _randomness)
    });
    (unsafe {
        let _N: u32 = N;
        let _M: u32 = (N).wrapping_div(4_u32);
        let _randomness: *mut support_simple_random_generator = randomness;
        test_one_102(_N, _M, _randomness)
    });
    (unsafe {
        let _N: u32 = N;
        let _M: u32 = (N).wrapping_div(2_u32);
        let _randomness: *mut support_simple_random_generator = randomness;
        test_one_102(_N, _M, _randomness)
    });
    (unsafe {
        let _N: u32 = N;
        let _M: u32 = ((3_u32).wrapping_mul(N)).wrapping_div(4_u32);
        let _randomness: *mut support_simple_random_generator = randomness;
        test_one_102(_N, _M, _randomness)
    });
    (unsafe {
        let _N: u32 = N;
        let _M: u32 = N;
        let _randomness: *mut support_simple_random_generator = randomness;
        test_one_102(_N, _M, _randomness)
    });
}
pub unsafe fn test_109(mut N: u32, randomness: *mut support_simple_random_generator) {
    (unsafe {
        let _N: u32 = N;
        let _randomness: *mut support_simple_random_generator = randomness;
        test_one_103(_N, 0_u32, _randomness)
    });
    (unsafe {
        let _N: u32 = N;
        let _M: u32 = (N).wrapping_div(4_u32);
        let _randomness: *mut support_simple_random_generator = randomness;
        test_one_103(_N, _M, _randomness)
    });
    (unsafe {
        let _N: u32 = N;
        let _M: u32 = (N).wrapping_div(2_u32);
        let _randomness: *mut support_simple_random_generator = randomness;
        test_one_103(_N, _M, _randomness)
    });
    (unsafe {
        let _N: u32 = N;
        let _M: u32 = ((3_u32).wrapping_mul(N)).wrapping_div(4_u32);
        let _randomness: *mut support_simple_random_generator = randomness;
        test_one_103(_N, _M, _randomness)
    });
    (unsafe {
        let _N: u32 = N;
        let _M: u32 = N;
        let _randomness: *mut support_simple_random_generator = randomness;
        test_one_103(_N, _M, _randomness)
    });
}
pub unsafe fn test_110(randomness: *mut support_simple_random_generator) {
    (unsafe { test_one_98(0_u32, 0_u32, randomness) });
    (unsafe { test_one_98(1_u32, 0_u32, randomness) });
    (unsafe { test_one_98(1_u32, 1_u32, randomness) });
    (unsafe { test_one_98(2_u32, 0_u32, randomness) });
    (unsafe { test_one_98(2_u32, 1_u32, randomness) });
    (unsafe { test_one_98(2_u32, 2_u32, randomness) });
    (unsafe { test_one_98(3_u32, 0_u32, randomness) });
    (unsafe { test_one_98(3_u32, 1_u32, randomness) });
    (unsafe { test_one_98(3_u32, 2_u32, randomness) });
    (unsafe { test_one_98(3_u32, 3_u32, randomness) });
    (unsafe { test_104(4_u32, randomness) });
    (unsafe { test_104(20_u32, randomness) });
    (unsafe { test_104(50_u32, randomness) });
    {
        (unsafe { test_104(100_u32, randomness) });
        (unsafe { test_104(1000_u32, randomness) });
    }
}
pub unsafe fn test_111(randomness: *mut support_simple_random_generator) {
    (unsafe { test_one_99(0_u32, 0_u32, randomness) });
    (unsafe { test_one_99(1_u32, 0_u32, randomness) });
    (unsafe { test_one_99(1_u32, 1_u32, randomness) });
    (unsafe { test_one_99(2_u32, 0_u32, randomness) });
    (unsafe { test_one_99(2_u32, 1_u32, randomness) });
    (unsafe { test_one_99(2_u32, 2_u32, randomness) });
    (unsafe { test_one_99(3_u32, 0_u32, randomness) });
    (unsafe { test_one_99(3_u32, 1_u32, randomness) });
    (unsafe { test_one_99(3_u32, 2_u32, randomness) });
    (unsafe { test_one_99(3_u32, 3_u32, randomness) });
    (unsafe { test_105(4_u32, randomness) });
    (unsafe { test_105(20_u32, randomness) });
    (unsafe { test_105(50_u32, randomness) });
    {
        (unsafe { test_105(100_u32, randomness) });
        (unsafe { test_105(1000_u32, randomness) });
    }
}
pub unsafe fn test_112(randomness: *mut support_simple_random_generator) {
    (unsafe { test_one_100(0_u32, 0_u32, randomness) });
    (unsafe { test_one_100(1_u32, 0_u32, randomness) });
    (unsafe { test_one_100(1_u32, 1_u32, randomness) });
    (unsafe { test_one_100(2_u32, 0_u32, randomness) });
    (unsafe { test_one_100(2_u32, 1_u32, randomness) });
    (unsafe { test_one_100(2_u32, 2_u32, randomness) });
    (unsafe { test_one_100(3_u32, 0_u32, randomness) });
    (unsafe { test_one_100(3_u32, 1_u32, randomness) });
    (unsafe { test_one_100(3_u32, 2_u32, randomness) });
    (unsafe { test_one_100(3_u32, 3_u32, randomness) });
    (unsafe { test_106(4_u32, randomness) });
    (unsafe { test_106(20_u32, randomness) });
    (unsafe { test_106(50_u32, randomness) });
    {
        (unsafe { test_106(100_u32, randomness) });
        (unsafe { test_106(1000_u32, randomness) });
    }
}
pub unsafe fn test_113(randomness: *mut support_simple_random_generator) {
    (unsafe { test_one_101(0_u32, 0_u32, randomness) });
    (unsafe { test_one_101(1_u32, 0_u32, randomness) });
    (unsafe { test_one_101(1_u32, 1_u32, randomness) });
    (unsafe { test_one_101(2_u32, 0_u32, randomness) });
    (unsafe { test_one_101(2_u32, 1_u32, randomness) });
    (unsafe { test_one_101(2_u32, 2_u32, randomness) });
    (unsafe { test_one_101(3_u32, 0_u32, randomness) });
    (unsafe { test_one_101(3_u32, 1_u32, randomness) });
    (unsafe { test_one_101(3_u32, 2_u32, randomness) });
    (unsafe { test_one_101(3_u32, 3_u32, randomness) });
    (unsafe { test_107(4_u32, randomness) });
    (unsafe { test_107(20_u32, randomness) });
    (unsafe { test_107(50_u32, randomness) });
    {
        (unsafe { test_107(100_u32, randomness) });
        (unsafe { test_107(1000_u32, randomness) });
    }
}
pub unsafe fn test_114(randomness: *mut support_simple_random_generator) {
    (unsafe { test_one_102(0_u32, 0_u32, randomness) });
    (unsafe { test_one_102(1_u32, 0_u32, randomness) });
    (unsafe { test_one_102(1_u32, 1_u32, randomness) });
    (unsafe { test_one_102(2_u32, 0_u32, randomness) });
    (unsafe { test_one_102(2_u32, 1_u32, randomness) });
    (unsafe { test_one_102(2_u32, 2_u32, randomness) });
    (unsafe { test_one_102(3_u32, 0_u32, randomness) });
    (unsafe { test_one_102(3_u32, 1_u32, randomness) });
    (unsafe { test_one_102(3_u32, 2_u32, randomness) });
    (unsafe { test_one_102(3_u32, 3_u32, randomness) });
    (unsafe { test_108(4_u32, randomness) });
    (unsafe { test_108(20_u32, randomness) });
    (unsafe { test_108(50_u32, randomness) });
    {
        (unsafe { test_108(100_u32, randomness) });
        (unsafe { test_108(1000_u32, randomness) });
    }
}
pub unsafe fn test_115(randomness: *mut support_simple_random_generator) {
    (unsafe { test_one_103(0_u32, 0_u32, randomness) });
    (unsafe { test_one_103(1_u32, 0_u32, randomness) });
    (unsafe { test_one_103(1_u32, 1_u32, randomness) });
    (unsafe { test_one_103(2_u32, 0_u32, randomness) });
    (unsafe { test_one_103(2_u32, 1_u32, randomness) });
    (unsafe { test_one_103(2_u32, 2_u32, randomness) });
    (unsafe { test_one_103(3_u32, 0_u32, randomness) });
    (unsafe { test_one_103(3_u32, 1_u32, randomness) });
    (unsafe { test_one_103(3_u32, 2_u32, randomness) });
    (unsafe { test_one_103(3_u32, 3_u32, randomness) });
    (unsafe { test_109(4_u32, randomness) });
    (unsafe { test_109(20_u32, randomness) });
    (unsafe { test_109(50_u32, randomness) });
    {
        (unsafe { test_109(100_u32, randomness) });
        (unsafe { test_109(1000_u32, randomness) });
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct less_by_first {}
impl less_by_first {
    pub unsafe fn operator_call_1_struct_std_pair_int__int_(
        &self,
        lhs: *const (i32, i32),
        rhs: *const (i32, i32),
    ) -> bool {
        return unimplemented!();
    }
}
pub unsafe fn test_PR31166_116() {
    let mut vec_: [(i32, i32); 5] = [(1, 0), (2, 0), (2, 1), (2, 2), (2, 3)];
    let mut i: i32 = 0;
    'loop_: while ((i) < (5)) {
        let mut res: Vec<(i32, i32)> = {
            let mut __v = Vec::new();
            let mut __it = vec_.as_mut_ptr();
            while __it != vec_.as_mut_ptr().offset((5) as isize) {
                __v.push(<(i32, i32) as CppFromUnsafe<(i32, i32)>>::cpp_from(
                    __it.iter_deref(),
                ));
                __it.iter_inc();
            }
            __v
        };
        unimplemented!();
        assert!(((res.len()) == (5_usize)));
        assert!(unimplemented!());
        i.prefix_inc();
    }
}
pub unsafe fn test_117() -> bool {
    let mut randomness: support_simple_random_generator = support_simple_random_generator::new_1();
    (unsafe { test_110(&mut randomness) });
    (unsafe { test_111(&mut randomness) });
    (unsafe { test_112(&mut randomness) });
    (unsafe { test_113(&mut randomness) });
    (unsafe { test_114(&mut randomness) });
    (unsafe { test_115(&mut randomness) });
    {
        let N: i32 = 100;
        let M: i32 = 50;
        let mut ia: *mut Option<Box<i32>> = Box::leak(
            (0..(N as usize))
                .map(|_| None)
                .collect::<Box<[Option<Box<i32>>]>>(),
        )
        .as_mut_ptr();
        let mut i: i32 = 0;
        'loop_: while ((i) < (N)) {
            {
                let _a0: *mut i32 = (Box::leak(Box::new(i)) as *mut i32);
                (*ia.offset((i) as isize)) = if _a0.is_null() {
                    None
                } else {
                    Some(Box::from_raw(_a0))
                }
            };
            i.prefix_inc();
        }
        (unsafe {
            let _first: *mut Option<Box<i32>> = ia;
            let _last: *mut Option<Box<i32>> = ia.offset((N) as isize);
            shuffle_74(_first, _last, &mut randomness)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        assert!(
            ((*(*ia.offset((0) as isize))
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        assert!(
            ((*(*ia.offset(((N) - (1)) as isize))
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == ((N) - (1)))
        );
        assert!(unimplemented!());
        {
            let __p = ia;
            if !__p.is_null() {
                ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                    __p,
                    libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                        / ::std::mem::size_of::<Option<Box<i32>>>(),
                )))
            }
        };
    }
    (unsafe { test_PR31166_116() });
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
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    (unsafe { test_117() });
    return 0;
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
