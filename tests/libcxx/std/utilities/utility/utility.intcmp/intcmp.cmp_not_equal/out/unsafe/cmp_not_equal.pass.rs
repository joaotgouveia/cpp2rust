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
pub unsafe fn test_cmp_not_equal1_0() {
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
}
pub unsafe fn test_cmp_not_equal1_1() {
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
}
pub unsafe fn test_cmp_not_equal1_2() {
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
}
pub unsafe fn test_cmp_not_equal1_3() {
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
}
pub unsafe fn test_cmp_not_equal1_4() {
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
}
pub unsafe fn test_cmp_not_equal1_5() {
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
}
pub unsafe fn test_cmp_not_equal1_6() {
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
}
pub unsafe fn test_cmp_not_equal1_7() {
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
}
pub unsafe fn test_cmp_not_equal1_8() {
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
}
pub unsafe fn test_cmp_not_equal1_9() {
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
}
pub unsafe fn test_cmp_not_equal1_10() {
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
}
pub unsafe fn test_cmp_not_equal1_11() {
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
}
pub unsafe fn test_cmp_not_equal2_12() {
    let ttup: Tuple_signed_char_ = Tuple_signed_char_::new();
    let utup: Tuple_signed_char_ = Tuple_signed_char_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_13() {
    let ttup: Tuple_signed_char_ = Tuple_signed_char_::new();
    let utup: Tuple_unsigned_char_ = Tuple_unsigned_char_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_14() {
    let ttup: Tuple_signed_char_ = Tuple_signed_char_::new();
    let utup: Tuple_short_ = Tuple_short_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_15() {
    let ttup: Tuple_signed_char_ = Tuple_signed_char_::new();
    let utup: Tuple_unsigned_short_ = Tuple_unsigned_short_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_16() {
    let ttup: Tuple_signed_char_ = Tuple_signed_char_::new();
    let utup: Tuple_int_ = Tuple_int_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_17() {
    let ttup: Tuple_signed_char_ = Tuple_signed_char_::new();
    let utup: Tuple_unsigned_int_ = Tuple_unsigned_int_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_18() {
    let ttup: Tuple_signed_char_ = Tuple_signed_char_::new();
    let utup: Tuple_long_ = Tuple_long_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_19() {
    let ttup: Tuple_signed_char_ = Tuple_signed_char_::new();
    let utup: Tuple_unsigned_long_ = Tuple_unsigned_long_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_20() {
    let ttup: Tuple_signed_char_ = Tuple_signed_char_::new();
    let utup: Tuple_long_long_ = Tuple_long_long_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_21() {
    let ttup: Tuple_signed_char_ = Tuple_signed_char_::new();
    let utup: Tuple_unsigned_long_long_ = Tuple_unsigned_long_long_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_22() {
    let ttup: Tuple_signed_char_ = Tuple_signed_char_::new();
    let utup: Tuple_unsigned___int128_ = Tuple_unsigned___int128_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_23() {
    let ttup: Tuple_signed_char_ = Tuple_signed_char_::new();
    let utup: Tuple___int128_ = Tuple___int128_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_24() {
    let ttup: Tuple_unsigned_char_ = Tuple_unsigned_char_::new();
    let utup: Tuple_signed_char_ = Tuple_signed_char_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_25() {
    let ttup: Tuple_unsigned_char_ = Tuple_unsigned_char_::new();
    let utup: Tuple_unsigned_char_ = Tuple_unsigned_char_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_26() {
    let ttup: Tuple_unsigned_char_ = Tuple_unsigned_char_::new();
    let utup: Tuple_short_ = Tuple_short_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_27() {
    let ttup: Tuple_unsigned_char_ = Tuple_unsigned_char_::new();
    let utup: Tuple_unsigned_short_ = Tuple_unsigned_short_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_28() {
    let ttup: Tuple_unsigned_char_ = Tuple_unsigned_char_::new();
    let utup: Tuple_int_ = Tuple_int_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_29() {
    let ttup: Tuple_unsigned_char_ = Tuple_unsigned_char_::new();
    let utup: Tuple_unsigned_int_ = Tuple_unsigned_int_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_30() {
    let ttup: Tuple_unsigned_char_ = Tuple_unsigned_char_::new();
    let utup: Tuple_long_ = Tuple_long_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_31() {
    let ttup: Tuple_unsigned_char_ = Tuple_unsigned_char_::new();
    let utup: Tuple_unsigned_long_ = Tuple_unsigned_long_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_32() {
    let ttup: Tuple_unsigned_char_ = Tuple_unsigned_char_::new();
    let utup: Tuple_long_long_ = Tuple_long_long_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_33() {
    let ttup: Tuple_unsigned_char_ = Tuple_unsigned_char_::new();
    let utup: Tuple_unsigned_long_long_ = Tuple_unsigned_long_long_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_34() {
    let ttup: Tuple_unsigned_char_ = Tuple_unsigned_char_::new();
    let utup: Tuple_unsigned___int128_ = Tuple_unsigned___int128_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_35() {
    let ttup: Tuple_unsigned_char_ = Tuple_unsigned_char_::new();
    let utup: Tuple___int128_ = Tuple___int128_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_36() {
    let ttup: Tuple_short_ = Tuple_short_::new();
    let utup: Tuple_signed_char_ = Tuple_signed_char_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_37() {
    let ttup: Tuple_short_ = Tuple_short_::new();
    let utup: Tuple_unsigned_char_ = Tuple_unsigned_char_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_38() {
    let ttup: Tuple_short_ = Tuple_short_::new();
    let utup: Tuple_short_ = Tuple_short_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_39() {
    let ttup: Tuple_short_ = Tuple_short_::new();
    let utup: Tuple_unsigned_short_ = Tuple_unsigned_short_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_40() {
    let ttup: Tuple_short_ = Tuple_short_::new();
    let utup: Tuple_int_ = Tuple_int_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_41() {
    let ttup: Tuple_short_ = Tuple_short_::new();
    let utup: Tuple_unsigned_int_ = Tuple_unsigned_int_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_42() {
    let ttup: Tuple_short_ = Tuple_short_::new();
    let utup: Tuple_long_ = Tuple_long_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_43() {
    let ttup: Tuple_short_ = Tuple_short_::new();
    let utup: Tuple_unsigned_long_ = Tuple_unsigned_long_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_44() {
    let ttup: Tuple_short_ = Tuple_short_::new();
    let utup: Tuple_long_long_ = Tuple_long_long_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_45() {
    let ttup: Tuple_short_ = Tuple_short_::new();
    let utup: Tuple_unsigned_long_long_ = Tuple_unsigned_long_long_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_46() {
    let ttup: Tuple_short_ = Tuple_short_::new();
    let utup: Tuple_unsigned___int128_ = Tuple_unsigned___int128_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_47() {
    let ttup: Tuple_short_ = Tuple_short_::new();
    let utup: Tuple___int128_ = Tuple___int128_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_48() {
    let ttup: Tuple_unsigned_short_ = Tuple_unsigned_short_::new();
    let utup: Tuple_signed_char_ = Tuple_signed_char_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_49() {
    let ttup: Tuple_unsigned_short_ = Tuple_unsigned_short_::new();
    let utup: Tuple_unsigned_char_ = Tuple_unsigned_char_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_50() {
    let ttup: Tuple_unsigned_short_ = Tuple_unsigned_short_::new();
    let utup: Tuple_short_ = Tuple_short_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_51() {
    let ttup: Tuple_unsigned_short_ = Tuple_unsigned_short_::new();
    let utup: Tuple_unsigned_short_ = Tuple_unsigned_short_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_52() {
    let ttup: Tuple_unsigned_short_ = Tuple_unsigned_short_::new();
    let utup: Tuple_int_ = Tuple_int_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_53() {
    let ttup: Tuple_unsigned_short_ = Tuple_unsigned_short_::new();
    let utup: Tuple_unsigned_int_ = Tuple_unsigned_int_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_54() {
    let ttup: Tuple_unsigned_short_ = Tuple_unsigned_short_::new();
    let utup: Tuple_long_ = Tuple_long_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_55() {
    let ttup: Tuple_unsigned_short_ = Tuple_unsigned_short_::new();
    let utup: Tuple_unsigned_long_ = Tuple_unsigned_long_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_56() {
    let ttup: Tuple_unsigned_short_ = Tuple_unsigned_short_::new();
    let utup: Tuple_long_long_ = Tuple_long_long_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_57() {
    let ttup: Tuple_unsigned_short_ = Tuple_unsigned_short_::new();
    let utup: Tuple_unsigned_long_long_ = Tuple_unsigned_long_long_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_58() {
    let ttup: Tuple_unsigned_short_ = Tuple_unsigned_short_::new();
    let utup: Tuple_unsigned___int128_ = Tuple_unsigned___int128_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_59() {
    let ttup: Tuple_unsigned_short_ = Tuple_unsigned_short_::new();
    let utup: Tuple___int128_ = Tuple___int128_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_60() {
    let ttup: Tuple_int_ = Tuple_int_::new();
    let utup: Tuple_signed_char_ = Tuple_signed_char_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_61() {
    let ttup: Tuple_int_ = Tuple_int_::new();
    let utup: Tuple_unsigned_char_ = Tuple_unsigned_char_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_62() {
    let ttup: Tuple_int_ = Tuple_int_::new();
    let utup: Tuple_short_ = Tuple_short_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_63() {
    let ttup: Tuple_int_ = Tuple_int_::new();
    let utup: Tuple_unsigned_short_ = Tuple_unsigned_short_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_64() {
    let ttup: Tuple_int_ = Tuple_int_::new();
    let utup: Tuple_int_ = Tuple_int_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_65() {
    let ttup: Tuple_int_ = Tuple_int_::new();
    let utup: Tuple_unsigned_int_ = Tuple_unsigned_int_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_66() {
    let ttup: Tuple_int_ = Tuple_int_::new();
    let utup: Tuple_long_ = Tuple_long_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_67() {
    let ttup: Tuple_int_ = Tuple_int_::new();
    let utup: Tuple_unsigned_long_ = Tuple_unsigned_long_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_68() {
    let ttup: Tuple_int_ = Tuple_int_::new();
    let utup: Tuple_long_long_ = Tuple_long_long_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_69() {
    let ttup: Tuple_int_ = Tuple_int_::new();
    let utup: Tuple_unsigned_long_long_ = Tuple_unsigned_long_long_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_70() {
    let ttup: Tuple_int_ = Tuple_int_::new();
    let utup: Tuple_unsigned___int128_ = Tuple_unsigned___int128_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_71() {
    let ttup: Tuple_int_ = Tuple_int_::new();
    let utup: Tuple___int128_ = Tuple___int128_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_72() {
    let ttup: Tuple_unsigned_int_ = Tuple_unsigned_int_::new();
    let utup: Tuple_signed_char_ = Tuple_signed_char_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_73() {
    let ttup: Tuple_unsigned_int_ = Tuple_unsigned_int_::new();
    let utup: Tuple_unsigned_char_ = Tuple_unsigned_char_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_74() {
    let ttup: Tuple_unsigned_int_ = Tuple_unsigned_int_::new();
    let utup: Tuple_short_ = Tuple_short_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_75() {
    let ttup: Tuple_unsigned_int_ = Tuple_unsigned_int_::new();
    let utup: Tuple_unsigned_short_ = Tuple_unsigned_short_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_76() {
    let ttup: Tuple_unsigned_int_ = Tuple_unsigned_int_::new();
    let utup: Tuple_int_ = Tuple_int_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_77() {
    let ttup: Tuple_unsigned_int_ = Tuple_unsigned_int_::new();
    let utup: Tuple_unsigned_int_ = Tuple_unsigned_int_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_78() {
    let ttup: Tuple_unsigned_int_ = Tuple_unsigned_int_::new();
    let utup: Tuple_long_ = Tuple_long_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_79() {
    let ttup: Tuple_unsigned_int_ = Tuple_unsigned_int_::new();
    let utup: Tuple_unsigned_long_ = Tuple_unsigned_long_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_80() {
    let ttup: Tuple_unsigned_int_ = Tuple_unsigned_int_::new();
    let utup: Tuple_long_long_ = Tuple_long_long_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_81() {
    let ttup: Tuple_unsigned_int_ = Tuple_unsigned_int_::new();
    let utup: Tuple_unsigned_long_long_ = Tuple_unsigned_long_long_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_82() {
    let ttup: Tuple_unsigned_int_ = Tuple_unsigned_int_::new();
    let utup: Tuple_unsigned___int128_ = Tuple_unsigned___int128_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_83() {
    let ttup: Tuple_unsigned_int_ = Tuple_unsigned_int_::new();
    let utup: Tuple___int128_ = Tuple___int128_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_84() {
    let ttup: Tuple_long_ = Tuple_long_::new();
    let utup: Tuple_signed_char_ = Tuple_signed_char_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_85() {
    let ttup: Tuple_long_ = Tuple_long_::new();
    let utup: Tuple_unsigned_char_ = Tuple_unsigned_char_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_86() {
    let ttup: Tuple_long_ = Tuple_long_::new();
    let utup: Tuple_short_ = Tuple_short_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_87() {
    let ttup: Tuple_long_ = Tuple_long_::new();
    let utup: Tuple_unsigned_short_ = Tuple_unsigned_short_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_88() {
    let ttup: Tuple_long_ = Tuple_long_::new();
    let utup: Tuple_int_ = Tuple_int_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_89() {
    let ttup: Tuple_long_ = Tuple_long_::new();
    let utup: Tuple_unsigned_int_ = Tuple_unsigned_int_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_90() {
    let ttup: Tuple_long_ = Tuple_long_::new();
    let utup: Tuple_long_ = Tuple_long_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_91() {
    let ttup: Tuple_long_ = Tuple_long_::new();
    let utup: Tuple_unsigned_long_ = Tuple_unsigned_long_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_92() {
    let ttup: Tuple_long_ = Tuple_long_::new();
    let utup: Tuple_long_long_ = Tuple_long_long_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_93() {
    let ttup: Tuple_long_ = Tuple_long_::new();
    let utup: Tuple_unsigned_long_long_ = Tuple_unsigned_long_long_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_94() {
    let ttup: Tuple_long_ = Tuple_long_::new();
    let utup: Tuple_unsigned___int128_ = Tuple_unsigned___int128_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_95() {
    let ttup: Tuple_long_ = Tuple_long_::new();
    let utup: Tuple___int128_ = Tuple___int128_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_96() {
    let ttup: Tuple_unsigned_long_ = Tuple_unsigned_long_::new();
    let utup: Tuple_signed_char_ = Tuple_signed_char_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_97() {
    let ttup: Tuple_unsigned_long_ = Tuple_unsigned_long_::new();
    let utup: Tuple_unsigned_char_ = Tuple_unsigned_char_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_98() {
    let ttup: Tuple_unsigned_long_ = Tuple_unsigned_long_::new();
    let utup: Tuple_short_ = Tuple_short_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_99() {
    let ttup: Tuple_unsigned_long_ = Tuple_unsigned_long_::new();
    let utup: Tuple_unsigned_short_ = Tuple_unsigned_short_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_100() {
    let ttup: Tuple_unsigned_long_ = Tuple_unsigned_long_::new();
    let utup: Tuple_int_ = Tuple_int_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_101() {
    let ttup: Tuple_unsigned_long_ = Tuple_unsigned_long_::new();
    let utup: Tuple_unsigned_int_ = Tuple_unsigned_int_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_102() {
    let ttup: Tuple_unsigned_long_ = Tuple_unsigned_long_::new();
    let utup: Tuple_long_ = Tuple_long_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_103() {
    let ttup: Tuple_unsigned_long_ = Tuple_unsigned_long_::new();
    let utup: Tuple_unsigned_long_ = Tuple_unsigned_long_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_104() {
    let ttup: Tuple_unsigned_long_ = Tuple_unsigned_long_::new();
    let utup: Tuple_long_long_ = Tuple_long_long_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_105() {
    let ttup: Tuple_unsigned_long_ = Tuple_unsigned_long_::new();
    let utup: Tuple_unsigned_long_long_ = Tuple_unsigned_long_long_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_106() {
    let ttup: Tuple_unsigned_long_ = Tuple_unsigned_long_::new();
    let utup: Tuple_unsigned___int128_ = Tuple_unsigned___int128_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_107() {
    let ttup: Tuple_unsigned_long_ = Tuple_unsigned_long_::new();
    let utup: Tuple___int128_ = Tuple___int128_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_108() {
    let ttup: Tuple_long_long_ = Tuple_long_long_::new();
    let utup: Tuple_signed_char_ = Tuple_signed_char_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_109() {
    let ttup: Tuple_long_long_ = Tuple_long_long_::new();
    let utup: Tuple_unsigned_char_ = Tuple_unsigned_char_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_110() {
    let ttup: Tuple_long_long_ = Tuple_long_long_::new();
    let utup: Tuple_short_ = Tuple_short_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_111() {
    let ttup: Tuple_long_long_ = Tuple_long_long_::new();
    let utup: Tuple_unsigned_short_ = Tuple_unsigned_short_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_112() {
    let ttup: Tuple_long_long_ = Tuple_long_long_::new();
    let utup: Tuple_int_ = Tuple_int_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_113() {
    let ttup: Tuple_long_long_ = Tuple_long_long_::new();
    let utup: Tuple_unsigned_int_ = Tuple_unsigned_int_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_114() {
    let ttup: Tuple_long_long_ = Tuple_long_long_::new();
    let utup: Tuple_long_ = Tuple_long_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_115() {
    let ttup: Tuple_long_long_ = Tuple_long_long_::new();
    let utup: Tuple_unsigned_long_ = Tuple_unsigned_long_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_116() {
    let ttup: Tuple_long_long_ = Tuple_long_long_::new();
    let utup: Tuple_long_long_ = Tuple_long_long_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_117() {
    let ttup: Tuple_long_long_ = Tuple_long_long_::new();
    let utup: Tuple_unsigned_long_long_ = Tuple_unsigned_long_long_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_118() {
    let ttup: Tuple_long_long_ = Tuple_long_long_::new();
    let utup: Tuple_unsigned___int128_ = Tuple_unsigned___int128_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_119() {
    let ttup: Tuple_long_long_ = Tuple_long_long_::new();
    let utup: Tuple___int128_ = Tuple___int128_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_120() {
    let ttup: Tuple_unsigned_long_long_ = Tuple_unsigned_long_long_::new();
    let utup: Tuple_signed_char_ = Tuple_signed_char_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_121() {
    let ttup: Tuple_unsigned_long_long_ = Tuple_unsigned_long_long_::new();
    let utup: Tuple_unsigned_char_ = Tuple_unsigned_char_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_122() {
    let ttup: Tuple_unsigned_long_long_ = Tuple_unsigned_long_long_::new();
    let utup: Tuple_short_ = Tuple_short_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_123() {
    let ttup: Tuple_unsigned_long_long_ = Tuple_unsigned_long_long_::new();
    let utup: Tuple_unsigned_short_ = Tuple_unsigned_short_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_124() {
    let ttup: Tuple_unsigned_long_long_ = Tuple_unsigned_long_long_::new();
    let utup: Tuple_int_ = Tuple_int_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_125() {
    let ttup: Tuple_unsigned_long_long_ = Tuple_unsigned_long_long_::new();
    let utup: Tuple_unsigned_int_ = Tuple_unsigned_int_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_126() {
    let ttup: Tuple_unsigned_long_long_ = Tuple_unsigned_long_long_::new();
    let utup: Tuple_long_ = Tuple_long_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_127() {
    let ttup: Tuple_unsigned_long_long_ = Tuple_unsigned_long_long_::new();
    let utup: Tuple_unsigned_long_ = Tuple_unsigned_long_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_128() {
    let ttup: Tuple_unsigned_long_long_ = Tuple_unsigned_long_long_::new();
    let utup: Tuple_long_long_ = Tuple_long_long_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_129() {
    let ttup: Tuple_unsigned_long_long_ = Tuple_unsigned_long_long_::new();
    let utup: Tuple_unsigned_long_long_ = Tuple_unsigned_long_long_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_130() {
    let ttup: Tuple_unsigned_long_long_ = Tuple_unsigned_long_long_::new();
    let utup: Tuple_unsigned___int128_ = Tuple_unsigned___int128_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_131() {
    let ttup: Tuple_unsigned_long_long_ = Tuple_unsigned_long_long_::new();
    let utup: Tuple___int128_ = Tuple___int128_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_132() {
    let ttup: Tuple_unsigned___int128_ = Tuple_unsigned___int128_::new();
    let utup: Tuple_signed_char_ = Tuple_signed_char_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_133() {
    let ttup: Tuple_unsigned___int128_ = Tuple_unsigned___int128_::new();
    let utup: Tuple_unsigned_char_ = Tuple_unsigned_char_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_134() {
    let ttup: Tuple_unsigned___int128_ = Tuple_unsigned___int128_::new();
    let utup: Tuple_short_ = Tuple_short_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_135() {
    let ttup: Tuple_unsigned___int128_ = Tuple_unsigned___int128_::new();
    let utup: Tuple_unsigned_short_ = Tuple_unsigned_short_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_136() {
    let ttup: Tuple_unsigned___int128_ = Tuple_unsigned___int128_::new();
    let utup: Tuple_int_ = Tuple_int_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_137() {
    let ttup: Tuple_unsigned___int128_ = Tuple_unsigned___int128_::new();
    let utup: Tuple_unsigned_int_ = Tuple_unsigned_int_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_138() {
    let ttup: Tuple_unsigned___int128_ = Tuple_unsigned___int128_::new();
    let utup: Tuple_long_ = Tuple_long_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_139() {
    let ttup: Tuple_unsigned___int128_ = Tuple_unsigned___int128_::new();
    let utup: Tuple_unsigned_long_ = Tuple_unsigned_long_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_140() {
    let ttup: Tuple_unsigned___int128_ = Tuple_unsigned___int128_::new();
    let utup: Tuple_long_long_ = Tuple_long_long_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_141() {
    let ttup: Tuple_unsigned___int128_ = Tuple_unsigned___int128_::new();
    let utup: Tuple_unsigned_long_long_ = Tuple_unsigned_long_long_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_142() {
    let ttup: Tuple_unsigned___int128_ = Tuple_unsigned___int128_::new();
    let utup: Tuple_unsigned___int128_ = Tuple_unsigned___int128_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_143() {
    let ttup: Tuple_unsigned___int128_ = Tuple_unsigned___int128_::new();
    let utup: Tuple___int128_ = Tuple___int128_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_144() {
    let ttup: Tuple___int128_ = Tuple___int128_::new();
    let utup: Tuple_signed_char_ = Tuple_signed_char_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_145() {
    let ttup: Tuple___int128_ = Tuple___int128_::new();
    let utup: Tuple_unsigned_char_ = Tuple_unsigned_char_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_146() {
    let ttup: Tuple___int128_ = Tuple___int128_::new();
    let utup: Tuple_short_ = Tuple_short_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_147() {
    let ttup: Tuple___int128_ = Tuple___int128_::new();
    let utup: Tuple_unsigned_short_ = Tuple_unsigned_short_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_148() {
    let ttup: Tuple___int128_ = Tuple___int128_::new();
    let utup: Tuple_int_ = Tuple_int_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_149() {
    let ttup: Tuple___int128_ = Tuple___int128_::new();
    let utup: Tuple_unsigned_int_ = Tuple_unsigned_int_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_150() {
    let ttup: Tuple___int128_ = Tuple___int128_::new();
    let utup: Tuple_long_ = Tuple_long_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_151() {
    let ttup: Tuple___int128_ = Tuple___int128_::new();
    let utup: Tuple_unsigned_long_ = Tuple_unsigned_long_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_152() {
    let ttup: Tuple___int128_ = Tuple___int128_::new();
    let utup: Tuple_long_long_ = Tuple_long_long_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_153() {
    let ttup: Tuple___int128_ = Tuple___int128_::new();
    let utup: Tuple_unsigned_long_long_ = Tuple_unsigned_long_long_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_154() {
    let ttup: Tuple___int128_ = Tuple___int128_::new();
    let utup: Tuple_unsigned___int128_ = Tuple_unsigned___int128_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_cmp_not_equal2_155() {
    let ttup: Tuple___int128_ = Tuple___int128_::new();
    let utup: Tuple___int128_ = Tuple___int128_::new();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test1_156(_a0: *const ()) {
    {
        (unsafe { test_cmp_not_equal1_11() });
        {
            (unsafe { test_cmp_not_equal1_10() });
            {
                (unsafe { test_cmp_not_equal1_9() });
                {
                    (unsafe { test_cmp_not_equal1_8() });
                    {
                        (unsafe { test_cmp_not_equal1_7() });
                        {
                            (unsafe { test_cmp_not_equal1_6() });
                            {
                                (unsafe { test_cmp_not_equal1_5() });
                                {
                                    (unsafe { test_cmp_not_equal1_4() });
                                    {
                                        (unsafe { test_cmp_not_equal1_3() });
                                        {
                                            (unsafe { test_cmp_not_equal1_2() });
                                            {
                                                (unsafe { test_cmp_not_equal1_1() });
                                                (unsafe { test_cmp_not_equal1_0() })
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
        (unsafe { test_cmp_not_equal2_23() });
        {
            (unsafe { test_cmp_not_equal2_22() });
            {
                (unsafe { test_cmp_not_equal2_21() });
                {
                    (unsafe { test_cmp_not_equal2_20() });
                    {
                        (unsafe { test_cmp_not_equal2_19() });
                        {
                            (unsafe { test_cmp_not_equal2_18() });
                            {
                                (unsafe { test_cmp_not_equal2_17() });
                                {
                                    (unsafe { test_cmp_not_equal2_16() });
                                    {
                                        (unsafe { test_cmp_not_equal2_15() });
                                        {
                                            (unsafe { test_cmp_not_equal2_14() });
                                            {
                                                (unsafe { test_cmp_not_equal2_13() });
                                                (unsafe { test_cmp_not_equal2_12() })
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
        (unsafe { test_cmp_not_equal2_35() });
        {
            (unsafe { test_cmp_not_equal2_34() });
            {
                (unsafe { test_cmp_not_equal2_33() });
                {
                    (unsafe { test_cmp_not_equal2_32() });
                    {
                        (unsafe { test_cmp_not_equal2_31() });
                        {
                            (unsafe { test_cmp_not_equal2_30() });
                            {
                                (unsafe { test_cmp_not_equal2_29() });
                                {
                                    (unsafe { test_cmp_not_equal2_28() });
                                    {
                                        (unsafe { test_cmp_not_equal2_27() });
                                        {
                                            (unsafe { test_cmp_not_equal2_26() });
                                            {
                                                (unsafe { test_cmp_not_equal2_25() });
                                                (unsafe { test_cmp_not_equal2_24() })
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
        (unsafe { test_cmp_not_equal2_47() });
        {
            (unsafe { test_cmp_not_equal2_46() });
            {
                (unsafe { test_cmp_not_equal2_45() });
                {
                    (unsafe { test_cmp_not_equal2_44() });
                    {
                        (unsafe { test_cmp_not_equal2_43() });
                        {
                            (unsafe { test_cmp_not_equal2_42() });
                            {
                                (unsafe { test_cmp_not_equal2_41() });
                                {
                                    (unsafe { test_cmp_not_equal2_40() });
                                    {
                                        (unsafe { test_cmp_not_equal2_39() });
                                        {
                                            (unsafe { test_cmp_not_equal2_38() });
                                            {
                                                (unsafe { test_cmp_not_equal2_37() });
                                                (unsafe { test_cmp_not_equal2_36() })
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
        (unsafe { test_cmp_not_equal2_59() });
        {
            (unsafe { test_cmp_not_equal2_58() });
            {
                (unsafe { test_cmp_not_equal2_57() });
                {
                    (unsafe { test_cmp_not_equal2_56() });
                    {
                        (unsafe { test_cmp_not_equal2_55() });
                        {
                            (unsafe { test_cmp_not_equal2_54() });
                            {
                                (unsafe { test_cmp_not_equal2_53() });
                                {
                                    (unsafe { test_cmp_not_equal2_52() });
                                    {
                                        (unsafe { test_cmp_not_equal2_51() });
                                        {
                                            (unsafe { test_cmp_not_equal2_50() });
                                            {
                                                (unsafe { test_cmp_not_equal2_49() });
                                                (unsafe { test_cmp_not_equal2_48() })
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
        (unsafe { test_cmp_not_equal2_71() });
        {
            (unsafe { test_cmp_not_equal2_70() });
            {
                (unsafe { test_cmp_not_equal2_69() });
                {
                    (unsafe { test_cmp_not_equal2_68() });
                    {
                        (unsafe { test_cmp_not_equal2_67() });
                        {
                            (unsafe { test_cmp_not_equal2_66() });
                            {
                                (unsafe { test_cmp_not_equal2_65() });
                                {
                                    (unsafe { test_cmp_not_equal2_64() });
                                    {
                                        (unsafe { test_cmp_not_equal2_63() });
                                        {
                                            (unsafe { test_cmp_not_equal2_62() });
                                            {
                                                (unsafe { test_cmp_not_equal2_61() });
                                                (unsafe { test_cmp_not_equal2_60() })
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
        (unsafe { test_cmp_not_equal2_83() });
        {
            (unsafe { test_cmp_not_equal2_82() });
            {
                (unsafe { test_cmp_not_equal2_81() });
                {
                    (unsafe { test_cmp_not_equal2_80() });
                    {
                        (unsafe { test_cmp_not_equal2_79() });
                        {
                            (unsafe { test_cmp_not_equal2_78() });
                            {
                                (unsafe { test_cmp_not_equal2_77() });
                                {
                                    (unsafe { test_cmp_not_equal2_76() });
                                    {
                                        (unsafe { test_cmp_not_equal2_75() });
                                        {
                                            (unsafe { test_cmp_not_equal2_74() });
                                            {
                                                (unsafe { test_cmp_not_equal2_73() });
                                                (unsafe { test_cmp_not_equal2_72() })
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
        (unsafe { test_cmp_not_equal2_95() });
        {
            (unsafe { test_cmp_not_equal2_94() });
            {
                (unsafe { test_cmp_not_equal2_93() });
                {
                    (unsafe { test_cmp_not_equal2_92() });
                    {
                        (unsafe { test_cmp_not_equal2_91() });
                        {
                            (unsafe { test_cmp_not_equal2_90() });
                            {
                                (unsafe { test_cmp_not_equal2_89() });
                                {
                                    (unsafe { test_cmp_not_equal2_88() });
                                    {
                                        (unsafe { test_cmp_not_equal2_87() });
                                        {
                                            (unsafe { test_cmp_not_equal2_86() });
                                            {
                                                (unsafe { test_cmp_not_equal2_85() });
                                                (unsafe { test_cmp_not_equal2_84() })
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
        (unsafe { test_cmp_not_equal2_107() });
        {
            (unsafe { test_cmp_not_equal2_106() });
            {
                (unsafe { test_cmp_not_equal2_105() });
                {
                    (unsafe { test_cmp_not_equal2_104() });
                    {
                        (unsafe { test_cmp_not_equal2_103() });
                        {
                            (unsafe { test_cmp_not_equal2_102() });
                            {
                                (unsafe { test_cmp_not_equal2_101() });
                                {
                                    (unsafe { test_cmp_not_equal2_100() });
                                    {
                                        (unsafe { test_cmp_not_equal2_99() });
                                        {
                                            (unsafe { test_cmp_not_equal2_98() });
                                            {
                                                (unsafe { test_cmp_not_equal2_97() });
                                                (unsafe { test_cmp_not_equal2_96() })
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
        (unsafe { test_cmp_not_equal2_119() });
        {
            (unsafe { test_cmp_not_equal2_118() });
            {
                (unsafe { test_cmp_not_equal2_117() });
                {
                    (unsafe { test_cmp_not_equal2_116() });
                    {
                        (unsafe { test_cmp_not_equal2_115() });
                        {
                            (unsafe { test_cmp_not_equal2_114() });
                            {
                                (unsafe { test_cmp_not_equal2_113() });
                                {
                                    (unsafe { test_cmp_not_equal2_112() });
                                    {
                                        (unsafe { test_cmp_not_equal2_111() });
                                        {
                                            (unsafe { test_cmp_not_equal2_110() });
                                            {
                                                (unsafe { test_cmp_not_equal2_109() });
                                                (unsafe { test_cmp_not_equal2_108() })
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
        (unsafe { test_cmp_not_equal2_131() });
        {
            (unsafe { test_cmp_not_equal2_130() });
            {
                (unsafe { test_cmp_not_equal2_129() });
                {
                    (unsafe { test_cmp_not_equal2_128() });
                    {
                        (unsafe { test_cmp_not_equal2_127() });
                        {
                            (unsafe { test_cmp_not_equal2_126() });
                            {
                                (unsafe { test_cmp_not_equal2_125() });
                                {
                                    (unsafe { test_cmp_not_equal2_124() });
                                    {
                                        (unsafe { test_cmp_not_equal2_123() });
                                        {
                                            (unsafe { test_cmp_not_equal2_122() });
                                            {
                                                (unsafe { test_cmp_not_equal2_121() });
                                                (unsafe { test_cmp_not_equal2_120() })
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
        (unsafe { test_cmp_not_equal2_143() });
        {
            (unsafe { test_cmp_not_equal2_142() });
            {
                (unsafe { test_cmp_not_equal2_141() });
                {
                    (unsafe { test_cmp_not_equal2_140() });
                    {
                        (unsafe { test_cmp_not_equal2_139() });
                        {
                            (unsafe { test_cmp_not_equal2_138() });
                            {
                                (unsafe { test_cmp_not_equal2_137() });
                                {
                                    (unsafe { test_cmp_not_equal2_136() });
                                    {
                                        (unsafe { test_cmp_not_equal2_135() });
                                        {
                                            (unsafe { test_cmp_not_equal2_134() });
                                            {
                                                (unsafe { test_cmp_not_equal2_133() });
                                                (unsafe { test_cmp_not_equal2_132() })
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
        (unsafe { test_cmp_not_equal2_155() });
        {
            (unsafe { test_cmp_not_equal2_154() });
            {
                (unsafe { test_cmp_not_equal2_153() });
                {
                    (unsafe { test_cmp_not_equal2_152() });
                    {
                        (unsafe { test_cmp_not_equal2_151() });
                        {
                            (unsafe { test_cmp_not_equal2_150() });
                            {
                                (unsafe { test_cmp_not_equal2_149() });
                                {
                                    (unsafe { test_cmp_not_equal2_148() });
                                    {
                                        (unsafe { test_cmp_not_equal2_147() });
                                        {
                                            (unsafe { test_cmp_not_equal2_146() });
                                            {
                                                (unsafe { test_cmp_not_equal2_145() });
                                                (unsafe { test_cmp_not_equal2_144() })
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
