extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(3)]
pub struct Tuple_signed_char_ {
    #[offset(0)]
    pub min: (),
    #[offset(1)]
    pub max: (),
    #[offset(2)]
    pub mid: (),
}
impl Tuple_signed_char_ {
    pub fn new() -> Self {
        let __this: Value<Tuple_signed_char_> = Rc::new(RefCell::new(Self {
            min: unimplemented!(),
            max: unimplemented!(),
            mid: unimplemented!(),
        }));
        let this: Ptr<Tuple_signed_char_> = __this.as_pointer();
        field!(this, min).write(({ std_numeric_limits_signed_char_::min() }));
        field!(this, max).write(({ std_numeric_limits_signed_char_::max() }));
        field!(this, mid).write(unimplemented!());
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for Tuple_signed_char_ {
    fn default() -> Self {
        { Tuple_signed_char_::new() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(3)]
pub struct Tuple_unsigned_char_ {
    #[offset(0)]
    pub min: (),
    #[offset(1)]
    pub max: (),
    #[offset(2)]
    pub mid: (),
}
impl Tuple_unsigned_char_ {
    pub fn new() -> Self {
        let __this: Value<Tuple_unsigned_char_> = Rc::new(RefCell::new(Self {
            min: unimplemented!(),
            max: unimplemented!(),
            mid: unimplemented!(),
        }));
        let this: Ptr<Tuple_unsigned_char_> = __this.as_pointer();
        field!(this, min).write(({ std_numeric_limits_unsigned_char_::min() }));
        field!(this, max).write(({ std_numeric_limits_unsigned_char_::max() }));
        field!(this, mid).write(unimplemented!());
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for Tuple_unsigned_char_ {
    fn default() -> Self {
        { Tuple_unsigned_char_::new() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(6)]
pub struct Tuple_short_ {
    #[offset(0)]
    pub min: (),
    #[offset(2)]
    pub max: (),
    #[offset(4)]
    pub mid: (),
}
impl Tuple_short_ {
    pub fn new() -> Self {
        let __this: Value<Tuple_short_> = Rc::new(RefCell::new(Self {
            min: unimplemented!(),
            max: unimplemented!(),
            mid: unimplemented!(),
        }));
        let this: Ptr<Tuple_short_> = __this.as_pointer();
        field!(this, min).write(({ std_numeric_limits_short_::min() }));
        field!(this, max).write(({ std_numeric_limits_short_::max() }));
        field!(this, mid).write(unimplemented!());
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for Tuple_short_ {
    fn default() -> Self {
        { Tuple_short_::new() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(6)]
pub struct Tuple_unsigned_short_ {
    #[offset(0)]
    pub min: (),
    #[offset(2)]
    pub max: (),
    #[offset(4)]
    pub mid: (),
}
impl Tuple_unsigned_short_ {
    pub fn new() -> Self {
        let __this: Value<Tuple_unsigned_short_> = Rc::new(RefCell::new(Self {
            min: unimplemented!(),
            max: unimplemented!(),
            mid: unimplemented!(),
        }));
        let this: Ptr<Tuple_unsigned_short_> = __this.as_pointer();
        field!(this, min).write(({ std_numeric_limits_unsigned_short_::min() }));
        field!(this, max).write(({ std_numeric_limits_unsigned_short_::max() }));
        field!(this, mid).write(unimplemented!());
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for Tuple_unsigned_short_ {
    fn default() -> Self {
        { Tuple_unsigned_short_::new() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(12)]
pub struct Tuple_int_ {
    #[offset(0)]
    pub min: (),
    #[offset(4)]
    pub max: (),
    #[offset(8)]
    pub mid: (),
}
impl Tuple_int_ {
    pub fn new() -> Self {
        let __this: Value<Tuple_int_> = Rc::new(RefCell::new(Self {
            min: unimplemented!(),
            max: unimplemented!(),
            mid: unimplemented!(),
        }));
        let this: Ptr<Tuple_int_> = __this.as_pointer();
        field!(this, min).write(({ std_numeric_limits_int_::min() }));
        field!(this, max).write(({ std_numeric_limits_int_::max() }));
        field!(this, mid).write(unimplemented!());
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for Tuple_int_ {
    fn default() -> Self {
        { Tuple_int_::new() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(12)]
pub struct Tuple_unsigned_int_ {
    #[offset(0)]
    pub min: (),
    #[offset(4)]
    pub max: (),
    #[offset(8)]
    pub mid: (),
}
impl Tuple_unsigned_int_ {
    pub fn new() -> Self {
        let __this: Value<Tuple_unsigned_int_> = Rc::new(RefCell::new(Self {
            min: unimplemented!(),
            max: unimplemented!(),
            mid: unimplemented!(),
        }));
        let this: Ptr<Tuple_unsigned_int_> = __this.as_pointer();
        field!(this, min).write(({ std_numeric_limits_unsigned_int_::min() }));
        field!(this, max).write(({ std_numeric_limits_unsigned_int_::max() }));
        field!(this, mid).write(unimplemented!());
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for Tuple_unsigned_int_ {
    fn default() -> Self {
        { Tuple_unsigned_int_::new() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(24)]
pub struct Tuple_long_ {
    #[offset(0)]
    pub min: (),
    #[offset(8)]
    pub max: (),
    #[offset(16)]
    pub mid: (),
}
impl Tuple_long_ {
    pub fn new() -> Self {
        let __this: Value<Tuple_long_> = Rc::new(RefCell::new(Self {
            min: unimplemented!(),
            max: unimplemented!(),
            mid: unimplemented!(),
        }));
        let this: Ptr<Tuple_long_> = __this.as_pointer();
        field!(this, min).write(({ std_numeric_limits_long_::min() }));
        field!(this, max).write(({ std_numeric_limits_long_::max() }));
        field!(this, mid).write(unimplemented!());
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for Tuple_long_ {
    fn default() -> Self {
        { Tuple_long_::new() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(24)]
pub struct Tuple_unsigned_long_ {
    #[offset(0)]
    pub min: (),
    #[offset(8)]
    pub max: (),
    #[offset(16)]
    pub mid: (),
}
impl Tuple_unsigned_long_ {
    pub fn new() -> Self {
        let __this: Value<Tuple_unsigned_long_> = Rc::new(RefCell::new(Self {
            min: unimplemented!(),
            max: unimplemented!(),
            mid: unimplemented!(),
        }));
        let this: Ptr<Tuple_unsigned_long_> = __this.as_pointer();
        field!(this, min).write(({ std_numeric_limits_unsigned_long_::min() }));
        field!(this, max).write(({ std_numeric_limits_unsigned_long_::max() }));
        field!(this, mid).write(unimplemented!());
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for Tuple_unsigned_long_ {
    fn default() -> Self {
        { Tuple_unsigned_long_::new() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(24)]
pub struct Tuple_long_long_ {
    #[offset(0)]
    pub min: (),
    #[offset(8)]
    pub max: (),
    #[offset(16)]
    pub mid: (),
}
impl Tuple_long_long_ {
    pub fn new() -> Self {
        let __this: Value<Tuple_long_long_> = Rc::new(RefCell::new(Self {
            min: unimplemented!(),
            max: unimplemented!(),
            mid: unimplemented!(),
        }));
        let this: Ptr<Tuple_long_long_> = __this.as_pointer();
        field!(this, min).write(({ std_numeric_limits_long_long_::min() }));
        field!(this, max).write(({ std_numeric_limits_long_long_::max() }));
        field!(this, mid).write(unimplemented!());
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for Tuple_long_long_ {
    fn default() -> Self {
        { Tuple_long_long_::new() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(24)]
pub struct Tuple_unsigned_long_long_ {
    #[offset(0)]
    pub min: (),
    #[offset(8)]
    pub max: (),
    #[offset(16)]
    pub mid: (),
}
impl Tuple_unsigned_long_long_ {
    pub fn new() -> Self {
        let __this: Value<Tuple_unsigned_long_long_> = Rc::new(RefCell::new(Self {
            min: unimplemented!(),
            max: unimplemented!(),
            mid: unimplemented!(),
        }));
        let this: Ptr<Tuple_unsigned_long_long_> = __this.as_pointer();
        field!(this, min).write(({ std_numeric_limits_unsigned_long_long_::min() }));
        field!(this, max).write(({ std_numeric_limits_unsigned_long_long_::max() }));
        field!(this, mid).write(unimplemented!());
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for Tuple_unsigned_long_long_ {
    fn default() -> Self {
        { Tuple_unsigned_long_long_::new() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(48)]
pub struct Tuple_unsigned___int128_ {
    #[offset(0)]
    pub min: (),
    #[offset(16)]
    pub max: (),
    #[offset(32)]
    pub mid: (),
}
impl Tuple_unsigned___int128_ {
    pub fn new() -> Self {
        let __this: Value<Tuple_unsigned___int128_> = Rc::new(RefCell::new(Self {
            min: unimplemented!(),
            max: unimplemented!(),
            mid: unimplemented!(),
        }));
        let this: Ptr<Tuple_unsigned___int128_> = __this.as_pointer();
        field!(this, min).write(({ std_numeric_limits_unsigned___int128_::min() }));
        field!(this, max).write(({ std_numeric_limits_unsigned___int128_::max() }));
        field!(this, mid).write(unimplemented!());
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for Tuple_unsigned___int128_ {
    fn default() -> Self {
        { Tuple_unsigned___int128_::new() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(48)]
pub struct Tuple___int128_ {
    #[offset(0)]
    pub min: (),
    #[offset(16)]
    pub max: (),
    #[offset(32)]
    pub mid: (),
}
impl Tuple___int128_ {
    pub fn new() -> Self {
        let __this: Value<Tuple___int128_> = Rc::new(RefCell::new(Self {
            min: unimplemented!(),
            max: unimplemented!(),
            mid: unimplemented!(),
        }));
        let this: Ptr<Tuple___int128_> = __this.as_pointer();
        field!(this, min).write(({ std_numeric_limits___int128_::min() }));
        field!(this, max).write(({ std_numeric_limits___int128_::max() }));
        field!(this, mid).write(unimplemented!());
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for Tuple___int128_ {
    fn default() -> Self {
        { Tuple___int128_::new() }
    }
}
pub fn test_cmp_greater1_0() {
    let mut tup: Tuple_signed_char_ = Tuple_signed_char_::new();
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
pub fn test_cmp_greater1_1() {
    let mut tup: Tuple_unsigned_char_ = Tuple_unsigned_char_::new();
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
pub fn test_cmp_greater1_2() {
    let mut tup: Tuple_short_ = Tuple_short_::new();
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
pub fn test_cmp_greater1_3() {
    let mut tup: Tuple_unsigned_short_ = Tuple_unsigned_short_::new();
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
pub fn test_cmp_greater1_4() {
    let mut tup: Tuple_int_ = Tuple_int_::new();
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
pub fn test_cmp_greater1_5() {
    let mut tup: Tuple_unsigned_int_ = Tuple_unsigned_int_::new();
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
pub fn test_cmp_greater1_6() {
    let mut tup: Tuple_long_ = Tuple_long_::new();
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
pub fn test_cmp_greater1_7() {
    let mut tup: Tuple_unsigned_long_ = Tuple_unsigned_long_::new();
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
pub fn test_cmp_greater1_8() {
    let mut tup: Tuple_long_long_ = Tuple_long_long_::new();
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
pub fn test_cmp_greater1_9() {
    let mut tup: Tuple_unsigned_long_long_ = Tuple_unsigned_long_long_::new();
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
pub fn test_cmp_greater1_10() {
    let mut tup: Tuple_unsigned___int128_ = Tuple_unsigned___int128_::new();
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
pub fn test_cmp_greater1_11() {
    let mut tup: Tuple___int128_ = Tuple___int128_::new();
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
pub fn test_cmp_greater2_12() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_13() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_14() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_15() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_16() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_17() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_18() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_19() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_20() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_21() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_22() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_23() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_24() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_25() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_26() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_27() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_28() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_29() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_30() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_31() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_32() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_33() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_34() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_35() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_36() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_37() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_38() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_39() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_40() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_41() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_42() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_43() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_44() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_45() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_46() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_47() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_48() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_49() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_50() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_51() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_52() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_53() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_54() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_55() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_56() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_57() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_58() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_59() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_60() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_61() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_62() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_63() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_64() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_65() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_66() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_67() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_68() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_69() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_70() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_71() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_72() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_73() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_74() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_75() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_76() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_77() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_78() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_79() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_80() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_81() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_82() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_83() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_84() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_85() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_86() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_87() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_88() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_89() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_90() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_91() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_92() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_93() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_94() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_95() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_96() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_97() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_98() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_99() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_100() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_101() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_102() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_103() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_104() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_105() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_106() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_107() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_108() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_109() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_110() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_111() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_112() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_113() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_114() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_115() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_116() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_117() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_118() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_119() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_120() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_121() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_122() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_123() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_124() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_125() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_126() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_127() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_128() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_129() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_130() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_131() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_132() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_133() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_134() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_135() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_136() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_137() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_138() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_139() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_140() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_141() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_142() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_143() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_144() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_145() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_146() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_147() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_148() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_149() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_150() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_151() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_152() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_153() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_154() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_greater2_155() {
    unimplemented!();
    unimplemented!();
}
pub fn test1_156(_a0: Ptr<()>) {
    {
        ({ test_cmp_greater1_11() });
        {
            ({ test_cmp_greater1_10() });
            {
                ({ test_cmp_greater1_9() });
                {
                    ({ test_cmp_greater1_8() });
                    {
                        ({ test_cmp_greater1_7() });
                        {
                            ({ test_cmp_greater1_6() });
                            {
                                ({ test_cmp_greater1_5() });
                                {
                                    ({ test_cmp_greater1_4() });
                                    {
                                        ({ test_cmp_greater1_3() });
                                        {
                                            ({ test_cmp_greater1_2() });
                                            {
                                                ({ test_cmp_greater1_1() });
                                                ({ test_cmp_greater1_0() })
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
pub fn test2_impl_157(_a0: Ptr<()>) {
    {
        ({ test_cmp_greater2_23() });
        {
            ({ test_cmp_greater2_22() });
            {
                ({ test_cmp_greater2_21() });
                {
                    ({ test_cmp_greater2_20() });
                    {
                        ({ test_cmp_greater2_19() });
                        {
                            ({ test_cmp_greater2_18() });
                            {
                                ({ test_cmp_greater2_17() });
                                {
                                    ({ test_cmp_greater2_16() });
                                    {
                                        ({ test_cmp_greater2_15() });
                                        {
                                            ({ test_cmp_greater2_14() });
                                            {
                                                ({ test_cmp_greater2_13() });
                                                ({ test_cmp_greater2_12() })
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
pub fn test2_impl_158(_a0: Ptr<()>) {
    {
        ({ test_cmp_greater2_35() });
        {
            ({ test_cmp_greater2_34() });
            {
                ({ test_cmp_greater2_33() });
                {
                    ({ test_cmp_greater2_32() });
                    {
                        ({ test_cmp_greater2_31() });
                        {
                            ({ test_cmp_greater2_30() });
                            {
                                ({ test_cmp_greater2_29() });
                                {
                                    ({ test_cmp_greater2_28() });
                                    {
                                        ({ test_cmp_greater2_27() });
                                        {
                                            ({ test_cmp_greater2_26() });
                                            {
                                                ({ test_cmp_greater2_25() });
                                                ({ test_cmp_greater2_24() })
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
pub fn test2_impl_159(_a0: Ptr<()>) {
    {
        ({ test_cmp_greater2_47() });
        {
            ({ test_cmp_greater2_46() });
            {
                ({ test_cmp_greater2_45() });
                {
                    ({ test_cmp_greater2_44() });
                    {
                        ({ test_cmp_greater2_43() });
                        {
                            ({ test_cmp_greater2_42() });
                            {
                                ({ test_cmp_greater2_41() });
                                {
                                    ({ test_cmp_greater2_40() });
                                    {
                                        ({ test_cmp_greater2_39() });
                                        {
                                            ({ test_cmp_greater2_38() });
                                            {
                                                ({ test_cmp_greater2_37() });
                                                ({ test_cmp_greater2_36() })
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
pub fn test2_impl_160(_a0: Ptr<()>) {
    {
        ({ test_cmp_greater2_59() });
        {
            ({ test_cmp_greater2_58() });
            {
                ({ test_cmp_greater2_57() });
                {
                    ({ test_cmp_greater2_56() });
                    {
                        ({ test_cmp_greater2_55() });
                        {
                            ({ test_cmp_greater2_54() });
                            {
                                ({ test_cmp_greater2_53() });
                                {
                                    ({ test_cmp_greater2_52() });
                                    {
                                        ({ test_cmp_greater2_51() });
                                        {
                                            ({ test_cmp_greater2_50() });
                                            {
                                                ({ test_cmp_greater2_49() });
                                                ({ test_cmp_greater2_48() })
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
pub fn test2_impl_161(_a0: Ptr<()>) {
    {
        ({ test_cmp_greater2_71() });
        {
            ({ test_cmp_greater2_70() });
            {
                ({ test_cmp_greater2_69() });
                {
                    ({ test_cmp_greater2_68() });
                    {
                        ({ test_cmp_greater2_67() });
                        {
                            ({ test_cmp_greater2_66() });
                            {
                                ({ test_cmp_greater2_65() });
                                {
                                    ({ test_cmp_greater2_64() });
                                    {
                                        ({ test_cmp_greater2_63() });
                                        {
                                            ({ test_cmp_greater2_62() });
                                            {
                                                ({ test_cmp_greater2_61() });
                                                ({ test_cmp_greater2_60() })
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
pub fn test2_impl_162(_a0: Ptr<()>) {
    {
        ({ test_cmp_greater2_83() });
        {
            ({ test_cmp_greater2_82() });
            {
                ({ test_cmp_greater2_81() });
                {
                    ({ test_cmp_greater2_80() });
                    {
                        ({ test_cmp_greater2_79() });
                        {
                            ({ test_cmp_greater2_78() });
                            {
                                ({ test_cmp_greater2_77() });
                                {
                                    ({ test_cmp_greater2_76() });
                                    {
                                        ({ test_cmp_greater2_75() });
                                        {
                                            ({ test_cmp_greater2_74() });
                                            {
                                                ({ test_cmp_greater2_73() });
                                                ({ test_cmp_greater2_72() })
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
pub fn test2_impl_163(_a0: Ptr<()>) {
    {
        ({ test_cmp_greater2_95() });
        {
            ({ test_cmp_greater2_94() });
            {
                ({ test_cmp_greater2_93() });
                {
                    ({ test_cmp_greater2_92() });
                    {
                        ({ test_cmp_greater2_91() });
                        {
                            ({ test_cmp_greater2_90() });
                            {
                                ({ test_cmp_greater2_89() });
                                {
                                    ({ test_cmp_greater2_88() });
                                    {
                                        ({ test_cmp_greater2_87() });
                                        {
                                            ({ test_cmp_greater2_86() });
                                            {
                                                ({ test_cmp_greater2_85() });
                                                ({ test_cmp_greater2_84() })
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
pub fn test2_impl_164(_a0: Ptr<()>) {
    {
        ({ test_cmp_greater2_107() });
        {
            ({ test_cmp_greater2_106() });
            {
                ({ test_cmp_greater2_105() });
                {
                    ({ test_cmp_greater2_104() });
                    {
                        ({ test_cmp_greater2_103() });
                        {
                            ({ test_cmp_greater2_102() });
                            {
                                ({ test_cmp_greater2_101() });
                                {
                                    ({ test_cmp_greater2_100() });
                                    {
                                        ({ test_cmp_greater2_99() });
                                        {
                                            ({ test_cmp_greater2_98() });
                                            {
                                                ({ test_cmp_greater2_97() });
                                                ({ test_cmp_greater2_96() })
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
pub fn test2_impl_165(_a0: Ptr<()>) {
    {
        ({ test_cmp_greater2_119() });
        {
            ({ test_cmp_greater2_118() });
            {
                ({ test_cmp_greater2_117() });
                {
                    ({ test_cmp_greater2_116() });
                    {
                        ({ test_cmp_greater2_115() });
                        {
                            ({ test_cmp_greater2_114() });
                            {
                                ({ test_cmp_greater2_113() });
                                {
                                    ({ test_cmp_greater2_112() });
                                    {
                                        ({ test_cmp_greater2_111() });
                                        {
                                            ({ test_cmp_greater2_110() });
                                            {
                                                ({ test_cmp_greater2_109() });
                                                ({ test_cmp_greater2_108() })
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
pub fn test2_impl_166(_a0: Ptr<()>) {
    {
        ({ test_cmp_greater2_131() });
        {
            ({ test_cmp_greater2_130() });
            {
                ({ test_cmp_greater2_129() });
                {
                    ({ test_cmp_greater2_128() });
                    {
                        ({ test_cmp_greater2_127() });
                        {
                            ({ test_cmp_greater2_126() });
                            {
                                ({ test_cmp_greater2_125() });
                                {
                                    ({ test_cmp_greater2_124() });
                                    {
                                        ({ test_cmp_greater2_123() });
                                        {
                                            ({ test_cmp_greater2_122() });
                                            {
                                                ({ test_cmp_greater2_121() });
                                                ({ test_cmp_greater2_120() })
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
pub fn test2_impl_167(_a0: Ptr<()>) {
    {
        ({ test_cmp_greater2_143() });
        {
            ({ test_cmp_greater2_142() });
            {
                ({ test_cmp_greater2_141() });
                {
                    ({ test_cmp_greater2_140() });
                    {
                        ({ test_cmp_greater2_139() });
                        {
                            ({ test_cmp_greater2_138() });
                            {
                                ({ test_cmp_greater2_137() });
                                {
                                    ({ test_cmp_greater2_136() });
                                    {
                                        ({ test_cmp_greater2_135() });
                                        {
                                            ({ test_cmp_greater2_134() });
                                            {
                                                ({ test_cmp_greater2_133() });
                                                ({ test_cmp_greater2_132() })
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
pub fn test2_impl_168(_a0: Ptr<()>) {
    {
        ({ test_cmp_greater2_155() });
        {
            ({ test_cmp_greater2_154() });
            {
                ({ test_cmp_greater2_153() });
                {
                    ({ test_cmp_greater2_152() });
                    {
                        ({ test_cmp_greater2_151() });
                        {
                            ({ test_cmp_greater2_150() });
                            {
                                ({ test_cmp_greater2_149() });
                                {
                                    ({ test_cmp_greater2_148() });
                                    {
                                        ({ test_cmp_greater2_147() });
                                        {
                                            ({ test_cmp_greater2_146() });
                                            {
                                                ({ test_cmp_greater2_145() });
                                                ({ test_cmp_greater2_144() })
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
pub fn test2_169(_a0: Ptr<()>, utuple: Ptr<()>) {
    {
        ({ test2_impl_168((utuple).clone()) });
        {
            ({ test2_impl_167((utuple).clone()) });
            {
                ({ test2_impl_166((utuple).clone()) });
                {
                    ({ test2_impl_165((utuple).clone()) });
                    {
                        ({ test2_impl_164((utuple).clone()) });
                        {
                            ({ test2_impl_163((utuple).clone()) });
                            {
                                ({ test2_impl_162((utuple).clone()) });
                                {
                                    ({ test2_impl_161((utuple).clone()) });
                                    {
                                        ({ test2_impl_160((utuple).clone()) });
                                        {
                                            ({ test2_impl_159((utuple).clone()) });
                                            {
                                                ({ test2_impl_158((utuple).clone()) });
                                                ({ test2_impl_157((utuple).clone()) })
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
pub fn test_170() -> () {
    let types: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    ({ test1_156(types.as_pointer()) });
    ({
        let _arg0: Ptr<()> = types.as_pointer();
        let _utuple: Ptr<()> = types.as_pointer();
        test2_169(_arg0, _utuple)
    });
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
    const _: () = assert!(true, "");;
    ({ test_170() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
