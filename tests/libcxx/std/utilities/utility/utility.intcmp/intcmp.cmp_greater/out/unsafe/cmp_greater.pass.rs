extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Tuple_signed_char_ {
    pub min: (),
    pub max: (),
    pub mid: (),
}
impl Tuple_signed_char_ {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            min: unimplemented!(),
            max: unimplemented!(),
            mid: unimplemented!(),
        };
        this.min = (unsafe { std_numeric_limits_signed_char_::min() });
        this.max = (unsafe { std_numeric_limits_signed_char_::max() });
        this.mid = unimplemented!();
        this
    }
}
impl Default for Tuple_signed_char_ {
    fn default() -> Self {
        unsafe { Tuple_signed_char_::new() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Tuple_unsigned_char_ {
    pub min: (),
    pub max: (),
    pub mid: (),
}
impl Tuple_unsigned_char_ {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            min: unimplemented!(),
            max: unimplemented!(),
            mid: unimplemented!(),
        };
        this.min = (unsafe { std_numeric_limits_unsigned_char_::min() });
        this.max = (unsafe { std_numeric_limits_unsigned_char_::max() });
        this.mid = unimplemented!();
        this
    }
}
impl Default for Tuple_unsigned_char_ {
    fn default() -> Self {
        unsafe { Tuple_unsigned_char_::new() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Tuple_short_ {
    pub min: (),
    pub max: (),
    pub mid: (),
}
impl Tuple_short_ {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            min: unimplemented!(),
            max: unimplemented!(),
            mid: unimplemented!(),
        };
        this.min = (unsafe { std_numeric_limits_short_::min() });
        this.max = (unsafe { std_numeric_limits_short_::max() });
        this.mid = unimplemented!();
        this
    }
}
impl Default for Tuple_short_ {
    fn default() -> Self {
        unsafe { Tuple_short_::new() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Tuple_unsigned_short_ {
    pub min: (),
    pub max: (),
    pub mid: (),
}
impl Tuple_unsigned_short_ {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            min: unimplemented!(),
            max: unimplemented!(),
            mid: unimplemented!(),
        };
        this.min = (unsafe { std_numeric_limits_unsigned_short_::min() });
        this.max = (unsafe { std_numeric_limits_unsigned_short_::max() });
        this.mid = unimplemented!();
        this
    }
}
impl Default for Tuple_unsigned_short_ {
    fn default() -> Self {
        unsafe { Tuple_unsigned_short_::new() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Tuple_int_ {
    pub min: (),
    pub max: (),
    pub mid: (),
}
impl Tuple_int_ {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            min: unimplemented!(),
            max: unimplemented!(),
            mid: unimplemented!(),
        };
        this.min = (unsafe { std_numeric_limits_int_::min() });
        this.max = (unsafe { std_numeric_limits_int_::max() });
        this.mid = unimplemented!();
        this
    }
}
impl Default for Tuple_int_ {
    fn default() -> Self {
        unsafe { Tuple_int_::new() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Tuple_unsigned_int_ {
    pub min: (),
    pub max: (),
    pub mid: (),
}
impl Tuple_unsigned_int_ {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            min: unimplemented!(),
            max: unimplemented!(),
            mid: unimplemented!(),
        };
        this.min = (unsafe { std_numeric_limits_unsigned_int_::min() });
        this.max = (unsafe { std_numeric_limits_unsigned_int_::max() });
        this.mid = unimplemented!();
        this
    }
}
impl Default for Tuple_unsigned_int_ {
    fn default() -> Self {
        unsafe { Tuple_unsigned_int_::new() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Tuple_long_ {
    pub min: (),
    pub max: (),
    pub mid: (),
}
impl Tuple_long_ {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            min: unimplemented!(),
            max: unimplemented!(),
            mid: unimplemented!(),
        };
        this.min = (unsafe { std_numeric_limits_long_::min() });
        this.max = (unsafe { std_numeric_limits_long_::max() });
        this.mid = unimplemented!();
        this
    }
}
impl Default for Tuple_long_ {
    fn default() -> Self {
        unsafe { Tuple_long_::new() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Tuple_unsigned_long_ {
    pub min: (),
    pub max: (),
    pub mid: (),
}
impl Tuple_unsigned_long_ {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            min: unimplemented!(),
            max: unimplemented!(),
            mid: unimplemented!(),
        };
        this.min = (unsafe { std_numeric_limits_unsigned_long_::min() });
        this.max = (unsafe { std_numeric_limits_unsigned_long_::max() });
        this.mid = unimplemented!();
        this
    }
}
impl Default for Tuple_unsigned_long_ {
    fn default() -> Self {
        unsafe { Tuple_unsigned_long_::new() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Tuple_long_long_ {
    pub min: (),
    pub max: (),
    pub mid: (),
}
impl Tuple_long_long_ {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            min: unimplemented!(),
            max: unimplemented!(),
            mid: unimplemented!(),
        };
        this.min = (unsafe { std_numeric_limits_long_long_::min() });
        this.max = (unsafe { std_numeric_limits_long_long_::max() });
        this.mid = unimplemented!();
        this
    }
}
impl Default for Tuple_long_long_ {
    fn default() -> Self {
        unsafe { Tuple_long_long_::new() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Tuple_unsigned_long_long_ {
    pub min: (),
    pub max: (),
    pub mid: (),
}
impl Tuple_unsigned_long_long_ {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            min: unimplemented!(),
            max: unimplemented!(),
            mid: unimplemented!(),
        };
        this.min = (unsafe { std_numeric_limits_unsigned_long_long_::min() });
        this.max = (unsafe { std_numeric_limits_unsigned_long_long_::max() });
        this.mid = unimplemented!();
        this
    }
}
impl Default for Tuple_unsigned_long_long_ {
    fn default() -> Self {
        unsafe { Tuple_unsigned_long_long_::new() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Tuple_unsigned___int128_ {
    pub min: (),
    pub max: (),
    pub mid: (),
}
impl Tuple_unsigned___int128_ {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            min: unimplemented!(),
            max: unimplemented!(),
            mid: unimplemented!(),
        };
        this.min = (unsafe { std_numeric_limits_unsigned___int128_::min() });
        this.max = (unsafe { std_numeric_limits_unsigned___int128_::max() });
        this.mid = unimplemented!();
        this
    }
}
impl Default for Tuple_unsigned___int128_ {
    fn default() -> Self {
        unsafe { Tuple_unsigned___int128_::new() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Tuple___int128_ {
    pub min: (),
    pub max: (),
    pub mid: (),
}
impl Tuple___int128_ {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            min: unimplemented!(),
            max: unimplemented!(),
            mid: unimplemented!(),
        };
        this.min = (unsafe { std_numeric_limits___int128_::min() });
        this.max = (unsafe { std_numeric_limits___int128_::max() });
        this.mid = unimplemented!();
        this
    }
}
impl Default for Tuple___int128_ {
    fn default() -> Self {
        unsafe { Tuple___int128_::new() }
    }
}
pub unsafe fn test_cmp_greater1_0() {
    let tup: Tuple_signed_char_ = Tuple_signed_char_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater1_1() {
    let tup: Tuple_unsigned_char_ = Tuple_unsigned_char_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater1_2() {
    let tup: Tuple_short_ = Tuple_short_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater1_3() {
    let tup: Tuple_unsigned_short_ = Tuple_unsigned_short_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater1_4() {
    let tup: Tuple_int_ = Tuple_int_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater1_5() {
    let tup: Tuple_unsigned_int_ = Tuple_unsigned_int_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater1_6() {
    let tup: Tuple_long_ = Tuple_long_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater1_7() {
    let tup: Tuple_unsigned_long_ = Tuple_unsigned_long_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater1_8() {
    let tup: Tuple_long_long_ = Tuple_long_long_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater1_9() {
    let tup: Tuple_unsigned_long_long_ = Tuple_unsigned_long_long_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater1_10() {
    let tup: Tuple_unsigned___int128_ = Tuple_unsigned___int128_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater1_11() {
    let tup: Tuple___int128_ = Tuple___int128_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_12() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_13() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_14() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_15() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_16() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_17() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_18() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_19() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_20() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_21() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_22() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_23() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_24() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_25() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_26() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_27() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_28() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_29() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_30() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_31() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_32() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_33() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_34() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_35() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_36() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_37() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_38() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_39() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_40() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_41() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_42() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_43() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_44() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_45() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_46() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_47() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_48() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_49() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_50() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_51() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_52() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_53() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_54() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_55() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_56() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_57() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_58() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_59() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_60() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_61() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_62() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_63() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_64() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_65() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_66() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_67() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_68() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_69() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_70() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_71() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_72() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_73() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_74() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_75() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_76() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_77() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_78() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_79() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_80() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_81() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_82() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_83() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_84() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_85() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_86() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_87() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_88() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_89() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_90() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_91() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_92() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_93() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_94() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_95() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_96() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_97() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_98() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_99() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_100() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_101() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_102() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_103() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_104() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_105() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_106() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_107() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_108() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_109() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_110() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_111() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_112() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_113() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_114() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_115() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_116() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_117() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_118() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_119() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_120() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_121() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_122() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_123() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_124() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_125() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_126() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_127() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_128() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_129() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_130() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_131() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_132() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_133() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_134() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_135() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_136() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_137() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_138() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_139() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_140() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_141() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_142() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_143() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_144() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_145() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_146() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_147() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_148() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_149() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_150() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_151() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_152() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_153() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_154() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_greater2_155() {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test1_156(_a0: *const ()) {
    {
        (unsafe { test_cmp_greater1_11() });
        {
            (unsafe { test_cmp_greater1_10() });
            {
                (unsafe { test_cmp_greater1_9() });
                {
                    (unsafe { test_cmp_greater1_8() });
                    {
                        (unsafe { test_cmp_greater1_7() });
                        {
                            (unsafe { test_cmp_greater1_6() });
                            {
                                (unsafe { test_cmp_greater1_5() });
                                {
                                    (unsafe { test_cmp_greater1_4() });
                                    {
                                        (unsafe { test_cmp_greater1_3() });
                                        {
                                            (unsafe { test_cmp_greater1_2() });
                                            {
                                                (unsafe { test_cmp_greater1_1() });
                                                (unsafe { test_cmp_greater1_0() })
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    };
}
pub unsafe fn test2_impl_157(_a0: *const ()) {
    {
        (unsafe { test_cmp_greater2_23() });
        {
            (unsafe { test_cmp_greater2_22() });
            {
                (unsafe { test_cmp_greater2_21() });
                {
                    (unsafe { test_cmp_greater2_20() });
                    {
                        (unsafe { test_cmp_greater2_19() });
                        {
                            (unsafe { test_cmp_greater2_18() });
                            {
                                (unsafe { test_cmp_greater2_17() });
                                {
                                    (unsafe { test_cmp_greater2_16() });
                                    {
                                        (unsafe { test_cmp_greater2_15() });
                                        {
                                            (unsafe { test_cmp_greater2_14() });
                                            {
                                                (unsafe { test_cmp_greater2_13() });
                                                (unsafe { test_cmp_greater2_12() })
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    };
}
pub unsafe fn test2_impl_158(_a0: *const ()) {
    {
        (unsafe { test_cmp_greater2_35() });
        {
            (unsafe { test_cmp_greater2_34() });
            {
                (unsafe { test_cmp_greater2_33() });
                {
                    (unsafe { test_cmp_greater2_32() });
                    {
                        (unsafe { test_cmp_greater2_31() });
                        {
                            (unsafe { test_cmp_greater2_30() });
                            {
                                (unsafe { test_cmp_greater2_29() });
                                {
                                    (unsafe { test_cmp_greater2_28() });
                                    {
                                        (unsafe { test_cmp_greater2_27() });
                                        {
                                            (unsafe { test_cmp_greater2_26() });
                                            {
                                                (unsafe { test_cmp_greater2_25() });
                                                (unsafe { test_cmp_greater2_24() })
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    };
}
pub unsafe fn test2_impl_159(_a0: *const ()) {
    {
        (unsafe { test_cmp_greater2_47() });
        {
            (unsafe { test_cmp_greater2_46() });
            {
                (unsafe { test_cmp_greater2_45() });
                {
                    (unsafe { test_cmp_greater2_44() });
                    {
                        (unsafe { test_cmp_greater2_43() });
                        {
                            (unsafe { test_cmp_greater2_42() });
                            {
                                (unsafe { test_cmp_greater2_41() });
                                {
                                    (unsafe { test_cmp_greater2_40() });
                                    {
                                        (unsafe { test_cmp_greater2_39() });
                                        {
                                            (unsafe { test_cmp_greater2_38() });
                                            {
                                                (unsafe { test_cmp_greater2_37() });
                                                (unsafe { test_cmp_greater2_36() })
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    };
}
pub unsafe fn test2_impl_160(_a0: *const ()) {
    {
        (unsafe { test_cmp_greater2_59() });
        {
            (unsafe { test_cmp_greater2_58() });
            {
                (unsafe { test_cmp_greater2_57() });
                {
                    (unsafe { test_cmp_greater2_56() });
                    {
                        (unsafe { test_cmp_greater2_55() });
                        {
                            (unsafe { test_cmp_greater2_54() });
                            {
                                (unsafe { test_cmp_greater2_53() });
                                {
                                    (unsafe { test_cmp_greater2_52() });
                                    {
                                        (unsafe { test_cmp_greater2_51() });
                                        {
                                            (unsafe { test_cmp_greater2_50() });
                                            {
                                                (unsafe { test_cmp_greater2_49() });
                                                (unsafe { test_cmp_greater2_48() })
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    };
}
pub unsafe fn test2_impl_161(_a0: *const ()) {
    {
        (unsafe { test_cmp_greater2_71() });
        {
            (unsafe { test_cmp_greater2_70() });
            {
                (unsafe { test_cmp_greater2_69() });
                {
                    (unsafe { test_cmp_greater2_68() });
                    {
                        (unsafe { test_cmp_greater2_67() });
                        {
                            (unsafe { test_cmp_greater2_66() });
                            {
                                (unsafe { test_cmp_greater2_65() });
                                {
                                    (unsafe { test_cmp_greater2_64() });
                                    {
                                        (unsafe { test_cmp_greater2_63() });
                                        {
                                            (unsafe { test_cmp_greater2_62() });
                                            {
                                                (unsafe { test_cmp_greater2_61() });
                                                (unsafe { test_cmp_greater2_60() })
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    };
}
pub unsafe fn test2_impl_162(_a0: *const ()) {
    {
        (unsafe { test_cmp_greater2_83() });
        {
            (unsafe { test_cmp_greater2_82() });
            {
                (unsafe { test_cmp_greater2_81() });
                {
                    (unsafe { test_cmp_greater2_80() });
                    {
                        (unsafe { test_cmp_greater2_79() });
                        {
                            (unsafe { test_cmp_greater2_78() });
                            {
                                (unsafe { test_cmp_greater2_77() });
                                {
                                    (unsafe { test_cmp_greater2_76() });
                                    {
                                        (unsafe { test_cmp_greater2_75() });
                                        {
                                            (unsafe { test_cmp_greater2_74() });
                                            {
                                                (unsafe { test_cmp_greater2_73() });
                                                (unsafe { test_cmp_greater2_72() })
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    };
}
pub unsafe fn test2_impl_163(_a0: *const ()) {
    {
        (unsafe { test_cmp_greater2_95() });
        {
            (unsafe { test_cmp_greater2_94() });
            {
                (unsafe { test_cmp_greater2_93() });
                {
                    (unsafe { test_cmp_greater2_92() });
                    {
                        (unsafe { test_cmp_greater2_91() });
                        {
                            (unsafe { test_cmp_greater2_90() });
                            {
                                (unsafe { test_cmp_greater2_89() });
                                {
                                    (unsafe { test_cmp_greater2_88() });
                                    {
                                        (unsafe { test_cmp_greater2_87() });
                                        {
                                            (unsafe { test_cmp_greater2_86() });
                                            {
                                                (unsafe { test_cmp_greater2_85() });
                                                (unsafe { test_cmp_greater2_84() })
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    };
}
pub unsafe fn test2_impl_164(_a0: *const ()) {
    {
        (unsafe { test_cmp_greater2_107() });
        {
            (unsafe { test_cmp_greater2_106() });
            {
                (unsafe { test_cmp_greater2_105() });
                {
                    (unsafe { test_cmp_greater2_104() });
                    {
                        (unsafe { test_cmp_greater2_103() });
                        {
                            (unsafe { test_cmp_greater2_102() });
                            {
                                (unsafe { test_cmp_greater2_101() });
                                {
                                    (unsafe { test_cmp_greater2_100() });
                                    {
                                        (unsafe { test_cmp_greater2_99() });
                                        {
                                            (unsafe { test_cmp_greater2_98() });
                                            {
                                                (unsafe { test_cmp_greater2_97() });
                                                (unsafe { test_cmp_greater2_96() })
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    };
}
pub unsafe fn test2_impl_165(_a0: *const ()) {
    {
        (unsafe { test_cmp_greater2_119() });
        {
            (unsafe { test_cmp_greater2_118() });
            {
                (unsafe { test_cmp_greater2_117() });
                {
                    (unsafe { test_cmp_greater2_116() });
                    {
                        (unsafe { test_cmp_greater2_115() });
                        {
                            (unsafe { test_cmp_greater2_114() });
                            {
                                (unsafe { test_cmp_greater2_113() });
                                {
                                    (unsafe { test_cmp_greater2_112() });
                                    {
                                        (unsafe { test_cmp_greater2_111() });
                                        {
                                            (unsafe { test_cmp_greater2_110() });
                                            {
                                                (unsafe { test_cmp_greater2_109() });
                                                (unsafe { test_cmp_greater2_108() })
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    };
}
pub unsafe fn test2_impl_166(_a0: *const ()) {
    {
        (unsafe { test_cmp_greater2_131() });
        {
            (unsafe { test_cmp_greater2_130() });
            {
                (unsafe { test_cmp_greater2_129() });
                {
                    (unsafe { test_cmp_greater2_128() });
                    {
                        (unsafe { test_cmp_greater2_127() });
                        {
                            (unsafe { test_cmp_greater2_126() });
                            {
                                (unsafe { test_cmp_greater2_125() });
                                {
                                    (unsafe { test_cmp_greater2_124() });
                                    {
                                        (unsafe { test_cmp_greater2_123() });
                                        {
                                            (unsafe { test_cmp_greater2_122() });
                                            {
                                                (unsafe { test_cmp_greater2_121() });
                                                (unsafe { test_cmp_greater2_120() })
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    };
}
pub unsafe fn test2_impl_167(_a0: *const ()) {
    {
        (unsafe { test_cmp_greater2_143() });
        {
            (unsafe { test_cmp_greater2_142() });
            {
                (unsafe { test_cmp_greater2_141() });
                {
                    (unsafe { test_cmp_greater2_140() });
                    {
                        (unsafe { test_cmp_greater2_139() });
                        {
                            (unsafe { test_cmp_greater2_138() });
                            {
                                (unsafe { test_cmp_greater2_137() });
                                {
                                    (unsafe { test_cmp_greater2_136() });
                                    {
                                        (unsafe { test_cmp_greater2_135() });
                                        {
                                            (unsafe { test_cmp_greater2_134() });
                                            {
                                                (unsafe { test_cmp_greater2_133() });
                                                (unsafe { test_cmp_greater2_132() })
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    };
}
pub unsafe fn test2_impl_168(_a0: *const ()) {
    {
        (unsafe { test_cmp_greater2_155() });
        {
            (unsafe { test_cmp_greater2_154() });
            {
                (unsafe { test_cmp_greater2_153() });
                {
                    (unsafe { test_cmp_greater2_152() });
                    {
                        (unsafe { test_cmp_greater2_151() });
                        {
                            (unsafe { test_cmp_greater2_150() });
                            {
                                (unsafe { test_cmp_greater2_149() });
                                {
                                    (unsafe { test_cmp_greater2_148() });
                                    {
                                        (unsafe { test_cmp_greater2_147() });
                                        {
                                            (unsafe { test_cmp_greater2_146() });
                                            {
                                                (unsafe { test_cmp_greater2_145() });
                                                (unsafe { test_cmp_greater2_144() })
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    };
}
pub unsafe fn test2_169(_a0: *const (), utuple: *const ()) {
    {
        (unsafe { test2_impl_168(utuple) });
        {
            (unsafe { test2_impl_167(utuple) });
            {
                (unsafe { test2_impl_166(utuple) });
                {
                    (unsafe { test2_impl_165(utuple) });
                    {
                        (unsafe { test2_impl_164(utuple) });
                        {
                            (unsafe { test2_impl_163(utuple) });
                            {
                                (unsafe { test2_impl_162(utuple) });
                                {
                                    (unsafe { test2_impl_161(utuple) });
                                    {
                                        (unsafe { test2_impl_160(utuple) });
                                        {
                                            (unsafe { test2_impl_159(utuple) });
                                            {
                                                (unsafe { test2_impl_158(utuple) });
                                                (unsafe { test2_impl_157(utuple) })
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    };
}
pub unsafe fn test_170() -> () {
    let mut types: () = unimplemented!();
    (unsafe { test1_156(&types) });
    (unsafe {
        let _arg0: *const () = &types;
        let _utuple: *const () = &types;
        test2_169(_arg0, _utuple)
    });
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
    const _: () = assert!(true, "");;
    (unsafe { test_170() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
