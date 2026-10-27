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
        field!(this, min).write(unimplemented!());
        field!(this, max).write(unimplemented!());
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
        field!(this, min).write(unimplemented!());
        field!(this, max).write(unimplemented!());
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
        field!(this, min).write(unimplemented!());
        field!(this, max).write(unimplemented!());
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
        field!(this, min).write(unimplemented!());
        field!(this, max).write(unimplemented!());
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
        field!(this, min).write(unimplemented!());
        field!(this, max).write(unimplemented!());
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
        field!(this, min).write(unimplemented!());
        field!(this, max).write(unimplemented!());
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
        field!(this, min).write(unimplemented!());
        field!(this, max).write(unimplemented!());
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
        field!(this, min).write(unimplemented!());
        field!(this, max).write(unimplemented!());
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
        field!(this, min).write(unimplemented!());
        field!(this, max).write(unimplemented!());
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
        field!(this, min).write(unimplemented!());
        field!(this, max).write(unimplemented!());
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
        field!(this, min).write(unimplemented!());
        field!(this, max).write(unimplemented!());
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
        field!(this, min).write(unimplemented!());
        field!(this, max).write(unimplemented!());
        field!(this, mid).write(unimplemented!());
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for Tuple___int128_ {
    fn default() -> Self {
        { Tuple___int128_::new() }
    }
}
pub fn test_cmp_less1_0() {
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
pub fn test_cmp_less1_1() {
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
pub fn test_cmp_less1_2() {
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
pub fn test_cmp_less1_3() {
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
pub fn test_cmp_less1_4() {
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
pub fn test_cmp_less1_5() {
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
pub fn test_cmp_less1_6() {
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
pub fn test_cmp_less1_7() {
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
pub fn test_cmp_less1_8() {
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
pub fn test_cmp_less1_9() {
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
pub fn test_cmp_less1_10() {
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
pub fn test_cmp_less1_11() {
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
pub fn test_cmp_less2_12() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_13() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_14() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_15() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_16() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_17() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_18() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_19() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_20() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_21() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_22() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_23() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_24() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_25() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_26() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_27() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_28() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_29() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_30() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_31() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_32() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_33() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_34() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_35() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_36() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_37() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_38() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_39() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_40() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_41() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_42() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_43() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_44() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_45() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_46() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_47() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_48() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_49() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_50() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_51() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_52() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_53() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_54() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_55() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_56() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_57() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_58() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_59() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_60() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_61() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_62() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_63() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_64() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_65() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_66() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_67() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_68() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_69() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_70() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_71() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_72() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_73() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_74() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_75() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_76() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_77() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_78() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_79() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_80() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_81() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_82() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_83() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_84() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_85() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_86() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_87() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_88() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_89() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_90() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_91() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_92() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_93() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_94() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_95() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_96() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_97() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_98() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_99() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_100() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_101() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_102() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_103() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_104() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_105() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_106() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_107() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_108() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_109() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_110() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_111() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_112() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_113() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_114() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_115() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_116() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_117() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_118() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_119() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_120() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_121() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_122() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_123() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_124() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_125() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_126() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_127() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_128() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_129() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_130() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_131() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_132() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_133() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_134() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_135() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_136() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_137() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_138() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_139() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_140() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_141() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_142() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_143() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_144() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_145() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_146() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_147() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_148() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_149() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_150() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_151() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_152() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_153() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_154() {
    unimplemented!();
    unimplemented!();
}
pub fn test_cmp_less2_155() {
    unimplemented!();
    unimplemented!();
}
pub fn test1_156(_a0: Ptr<()>) {
    {
        ({ test_cmp_less1_11() });
        {
            ({ test_cmp_less1_10() });
            {
                ({ test_cmp_less1_9() });
                {
                    ({ test_cmp_less1_8() });
                    {
                        ({ test_cmp_less1_7() });
                        {
                            ({ test_cmp_less1_6() });
                            {
                                ({ test_cmp_less1_5() });
                                {
                                    ({ test_cmp_less1_4() });
                                    {
                                        ({ test_cmp_less1_3() });
                                        {
                                            ({ test_cmp_less1_2() });
                                            {
                                                ({ test_cmp_less1_1() });
                                                ({ test_cmp_less1_0() })
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
        ({ test_cmp_less2_23() });
        {
            ({ test_cmp_less2_22() });
            {
                ({ test_cmp_less2_21() });
                {
                    ({ test_cmp_less2_20() });
                    {
                        ({ test_cmp_less2_19() });
                        {
                            ({ test_cmp_less2_18() });
                            {
                                ({ test_cmp_less2_17() });
                                {
                                    ({ test_cmp_less2_16() });
                                    {
                                        ({ test_cmp_less2_15() });
                                        {
                                            ({ test_cmp_less2_14() });
                                            {
                                                ({ test_cmp_less2_13() });
                                                ({ test_cmp_less2_12() })
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
        ({ test_cmp_less2_35() });
        {
            ({ test_cmp_less2_34() });
            {
                ({ test_cmp_less2_33() });
                {
                    ({ test_cmp_less2_32() });
                    {
                        ({ test_cmp_less2_31() });
                        {
                            ({ test_cmp_less2_30() });
                            {
                                ({ test_cmp_less2_29() });
                                {
                                    ({ test_cmp_less2_28() });
                                    {
                                        ({ test_cmp_less2_27() });
                                        {
                                            ({ test_cmp_less2_26() });
                                            {
                                                ({ test_cmp_less2_25() });
                                                ({ test_cmp_less2_24() })
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
        ({ test_cmp_less2_47() });
        {
            ({ test_cmp_less2_46() });
            {
                ({ test_cmp_less2_45() });
                {
                    ({ test_cmp_less2_44() });
                    {
                        ({ test_cmp_less2_43() });
                        {
                            ({ test_cmp_less2_42() });
                            {
                                ({ test_cmp_less2_41() });
                                {
                                    ({ test_cmp_less2_40() });
                                    {
                                        ({ test_cmp_less2_39() });
                                        {
                                            ({ test_cmp_less2_38() });
                                            {
                                                ({ test_cmp_less2_37() });
                                                ({ test_cmp_less2_36() })
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
        ({ test_cmp_less2_59() });
        {
            ({ test_cmp_less2_58() });
            {
                ({ test_cmp_less2_57() });
                {
                    ({ test_cmp_less2_56() });
                    {
                        ({ test_cmp_less2_55() });
                        {
                            ({ test_cmp_less2_54() });
                            {
                                ({ test_cmp_less2_53() });
                                {
                                    ({ test_cmp_less2_52() });
                                    {
                                        ({ test_cmp_less2_51() });
                                        {
                                            ({ test_cmp_less2_50() });
                                            {
                                                ({ test_cmp_less2_49() });
                                                ({ test_cmp_less2_48() })
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
        ({ test_cmp_less2_71() });
        {
            ({ test_cmp_less2_70() });
            {
                ({ test_cmp_less2_69() });
                {
                    ({ test_cmp_less2_68() });
                    {
                        ({ test_cmp_less2_67() });
                        {
                            ({ test_cmp_less2_66() });
                            {
                                ({ test_cmp_less2_65() });
                                {
                                    ({ test_cmp_less2_64() });
                                    {
                                        ({ test_cmp_less2_63() });
                                        {
                                            ({ test_cmp_less2_62() });
                                            {
                                                ({ test_cmp_less2_61() });
                                                ({ test_cmp_less2_60() })
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
        ({ test_cmp_less2_83() });
        {
            ({ test_cmp_less2_82() });
            {
                ({ test_cmp_less2_81() });
                {
                    ({ test_cmp_less2_80() });
                    {
                        ({ test_cmp_less2_79() });
                        {
                            ({ test_cmp_less2_78() });
                            {
                                ({ test_cmp_less2_77() });
                                {
                                    ({ test_cmp_less2_76() });
                                    {
                                        ({ test_cmp_less2_75() });
                                        {
                                            ({ test_cmp_less2_74() });
                                            {
                                                ({ test_cmp_less2_73() });
                                                ({ test_cmp_less2_72() })
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
        ({ test_cmp_less2_95() });
        {
            ({ test_cmp_less2_94() });
            {
                ({ test_cmp_less2_93() });
                {
                    ({ test_cmp_less2_92() });
                    {
                        ({ test_cmp_less2_91() });
                        {
                            ({ test_cmp_less2_90() });
                            {
                                ({ test_cmp_less2_89() });
                                {
                                    ({ test_cmp_less2_88() });
                                    {
                                        ({ test_cmp_less2_87() });
                                        {
                                            ({ test_cmp_less2_86() });
                                            {
                                                ({ test_cmp_less2_85() });
                                                ({ test_cmp_less2_84() })
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
        ({ test_cmp_less2_107() });
        {
            ({ test_cmp_less2_106() });
            {
                ({ test_cmp_less2_105() });
                {
                    ({ test_cmp_less2_104() });
                    {
                        ({ test_cmp_less2_103() });
                        {
                            ({ test_cmp_less2_102() });
                            {
                                ({ test_cmp_less2_101() });
                                {
                                    ({ test_cmp_less2_100() });
                                    {
                                        ({ test_cmp_less2_99() });
                                        {
                                            ({ test_cmp_less2_98() });
                                            {
                                                ({ test_cmp_less2_97() });
                                                ({ test_cmp_less2_96() })
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
        ({ test_cmp_less2_119() });
        {
            ({ test_cmp_less2_118() });
            {
                ({ test_cmp_less2_117() });
                {
                    ({ test_cmp_less2_116() });
                    {
                        ({ test_cmp_less2_115() });
                        {
                            ({ test_cmp_less2_114() });
                            {
                                ({ test_cmp_less2_113() });
                                {
                                    ({ test_cmp_less2_112() });
                                    {
                                        ({ test_cmp_less2_111() });
                                        {
                                            ({ test_cmp_less2_110() });
                                            {
                                                ({ test_cmp_less2_109() });
                                                ({ test_cmp_less2_108() })
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
        ({ test_cmp_less2_131() });
        {
            ({ test_cmp_less2_130() });
            {
                ({ test_cmp_less2_129() });
                {
                    ({ test_cmp_less2_128() });
                    {
                        ({ test_cmp_less2_127() });
                        {
                            ({ test_cmp_less2_126() });
                            {
                                ({ test_cmp_less2_125() });
                                {
                                    ({ test_cmp_less2_124() });
                                    {
                                        ({ test_cmp_less2_123() });
                                        {
                                            ({ test_cmp_less2_122() });
                                            {
                                                ({ test_cmp_less2_121() });
                                                ({ test_cmp_less2_120() })
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
        ({ test_cmp_less2_143() });
        {
            ({ test_cmp_less2_142() });
            {
                ({ test_cmp_less2_141() });
                {
                    ({ test_cmp_less2_140() });
                    {
                        ({ test_cmp_less2_139() });
                        {
                            ({ test_cmp_less2_138() });
                            {
                                ({ test_cmp_less2_137() });
                                {
                                    ({ test_cmp_less2_136() });
                                    {
                                        ({ test_cmp_less2_135() });
                                        {
                                            ({ test_cmp_less2_134() });
                                            {
                                                ({ test_cmp_less2_133() });
                                                ({ test_cmp_less2_132() })
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
        ({ test_cmp_less2_155() });
        {
            ({ test_cmp_less2_154() });
            {
                ({ test_cmp_less2_153() });
                {
                    ({ test_cmp_less2_152() });
                    {
                        ({ test_cmp_less2_151() });
                        {
                            ({ test_cmp_less2_150() });
                            {
                                ({ test_cmp_less2_149() });
                                {
                                    ({ test_cmp_less2_148() });
                                    {
                                        ({ test_cmp_less2_147() });
                                        {
                                            ({ test_cmp_less2_146() });
                                            {
                                                ({ test_cmp_less2_145() });
                                                ({ test_cmp_less2_144() })
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
