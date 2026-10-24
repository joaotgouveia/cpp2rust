extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct IntType {
    #[offset(0)]
    pub val: (),
}
impl IntType {
    pub fn new_1(mut v: ()) -> Self {
        Self { val: v }
    }
}
impl std::cmp::PartialEq for IntType {
    fn eq(&self, other: &Self) -> bool {
        {
            IntTypeImpl::operator_eq(
                &Rc::new(RefCell::new(IntType {
                    val: self.val.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(IntType {
                    val: other.val.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for IntType {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct IntTypeNC {
    #[offset(0)]
    pub val: (),
}
impl IntTypeNC {
    pub fn new_1(mut v: ()) -> Self {
        Self { val: v }
    }
}
pub type anon_0 = ();
pub const anon_0_value: anon_0 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_int__int_ {}
pub type anon_1 = ();
pub const anon_1_value: anon_1 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_unsigned_int__unsigned_int_ {}
pub type anon_2 = ();
pub const anon_2_value: anon_2 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_signed_char__signed_char_ {}
pub type anon_3 = ();
pub const anon_3_value: anon_3 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_long_long__long_long_ {}
pub type anon_4 = ();
pub const anon_4_value: anon_4 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_unsigned_long__unsigned_long_ {}
pub fn test_runtime_observers_5(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_6(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_7(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_8(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_9(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_10(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_11(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_12(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_13(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_14(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_15(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_16(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_17(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_18(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_19(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_20(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_21(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_22(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_23(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_24(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_25(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_26(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_27(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_28(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_29(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_30(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_31(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_32(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_33(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_34(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_35(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_36(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_37(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_38(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_39(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_40(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_41(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_42(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_43(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_44(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_45(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_46(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_47(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_48(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_49(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_50(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_51(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_52(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_53(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_54(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_55(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_56(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_57(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_58(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_59(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_60(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_61(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_62(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_63(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_64(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_65(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_66(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_67(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_68(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_69(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_70(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_71(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_72(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_73(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_74(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_75(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_76(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_77(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_78(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_79(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_80(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_81(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_82(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_83(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_84(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_85(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_86(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_87(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_88(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_89(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_90(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_91(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_92(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_93(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_94(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_95(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_96(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_97(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_98(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_99(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_100(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_101(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_102(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_103(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_104(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_105(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_106(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_107(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_108(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_109(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_110(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_111(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_112(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_113(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_114(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_115(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_116(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_117(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_118(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_119(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_120(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_121(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_122(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_123(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_124(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_125(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_126(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_127(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_128(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_129(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_130(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_131(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_132(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_133(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_134(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_135(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_136(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_137(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_138(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_139(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_140(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_141(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_142(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_143(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_144(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_145(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_146(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_147(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_148(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_149(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_150(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_151(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_152(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_153(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_154(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_155(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_156(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_157(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_158(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_159(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_160(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_161(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_162(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_163(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_164(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_165(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_runtime_observers_166(ext: (), expected: ()) {
    let ext: Value<()> = Rc::new(RefCell::new(ext));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        r.postfix_inc();
    }
}
pub fn test_construction_167(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_int__struct_std_array_int__0__struct_std_array_int__0_ ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_int__struct_std_array_int__0__struct_std_array_int__0_ ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_169(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_int__3__struct_std_array_int__1__struct_std_array_int__1__0 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_int__3__struct_std_array_int__1__struct_std_array_int__0_ ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_170(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__struct_std_array_int__1__struct_std_array_int__1__0 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__struct_std_array_int__1__struct_std_array_int__1__0 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_171(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_int__3__7__struct_std_array_int__2__struct_std_array_int__2__0_1 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_int__3__7__struct_std_array_int__2__struct_std_array_int__0_ ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_172(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_int__3__18446744073709551615__struct_std_array_int__2__struct_std_array_int__2__0_1 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_int__3__18446744073709551615__struct_std_array_int__2__struct_std_array_int__1__0 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_173(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__7__struct_std_array_int__2__struct_std_array_int__2__0_1 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__7__struct_std_array_int__2__struct_std_array_int__1__0 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_174(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__18446744073709551615__struct_std_array_int__2__struct_std_array_int__2__0_1 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__18446744073709551615__struct_std_array_int__2__struct_std_array_int__2__0_1 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_175(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_int__3__7__9__struct_std_array_int__3__struct_std_array_int__3__0_1_2 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_int__3__7__9__struct_std_array_int__3__struct_std_array_int__0_ ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_176(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_int__3__7__18446744073709551615__struct_std_array_int__3__struct_std_array_int__3__0_1_2 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_int__3__7__18446744073709551615__struct_std_array_int__3__struct_std_array_int__1__0 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_177(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_int__3__18446744073709551615__18446744073709551615__struct_std_array_int__3__struct_std_array_int__3__0_1_2 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_int__3__18446744073709551615__18446744073709551615__struct_std_array_int__3__struct_std_array_int__2__0_1 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_178(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__7__18446744073709551615__struct_std_array_int__3__struct_std_array_int__3__0_1_2 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__7__18446744073709551615__struct_std_array_int__3__struct_std_array_int__2__0_1 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_179(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_int__3__struct_std_array_int__3__0_1_2 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_int__3__struct_std_array_int__3__0_1_2 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_180(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_int__3__18446744073709551615__9__struct_std_array_int__3__struct_std_array_int__3__0_1_2 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_int__3__18446744073709551615__9__struct_std_array_int__3__struct_std_array_int__1__0 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_181(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__18446744073709551615__9__struct_std_array_int__3__struct_std_array_int__3__0_1_2 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__18446744073709551615__9__struct_std_array_int__3__struct_std_array_int__2__0_1 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_182(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__7__9__struct_std_array_int__3__struct_std_array_int__3__0_1_2 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__7__9__struct_std_array_int__3__struct_std_array_int__1__0 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_183(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_int__1__2__3__4__5__6__7__8__9__struct_std_array_int__9__struct_std_array_int__9__0_1_2_3_4_5_6_7_8 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_int__1__2__3__4__5__6__7__8__9__struct_std_array_int__9__struct_std_array_int__0_ ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_184(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__2__3__18446744073709551615__5__18446744073709551615__7__18446744073709551615__9__struct_std_array_int__9__struct_std_array_int__9__0_1_2_3_4_5_6_7_8 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__2__3__18446744073709551615__5__18446744073709551615__7__18446744073709551615__9__struct_std_array_int__9__struct_std_array_int__4__0_1_2_3 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_185(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_int__9__struct_std_array_int__9__0_1_2_3_4_5_6_7_8 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_int__9__struct_std_array_int__9__0_1_2_3_4_5_6_7_8 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_186(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_int__struct_std_array_unsigned_long__0__struct_std_array_unsigned_long__0_ ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_int__struct_std_array_unsigned_long__0__struct_std_array_unsigned_long__0_ ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_187(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_int__3__struct_std_array_unsigned_long__1__struct_std_array_unsigned_long__1__0 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_int__3__struct_std_array_unsigned_long__1__struct_std_array_unsigned_long__0_ ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_188(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__struct_std_array_unsigned_long__1__struct_std_array_unsigned_long__1__0 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__struct_std_array_unsigned_long__1__struct_std_array_unsigned_long__1__0 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_189(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_int__3__7__struct_std_array_unsigned_long__2__struct_std_array_unsigned_long__2__0_1 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_int__3__7__struct_std_array_unsigned_long__2__struct_std_array_unsigned_long__0_ ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_190(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_int__3__18446744073709551615__struct_std_array_unsigned_long__2__struct_std_array_unsigned_long__2__0_1 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_int__3__18446744073709551615__struct_std_array_unsigned_long__2__struct_std_array_unsigned_long__1__0 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_191(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__7__struct_std_array_unsigned_long__2__struct_std_array_unsigned_long__2__0_1 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__7__struct_std_array_unsigned_long__2__struct_std_array_unsigned_long__1__0 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_192(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__18446744073709551615__struct_std_array_unsigned_long__2__struct_std_array_unsigned_long__2__0_1 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__18446744073709551615__struct_std_array_unsigned_long__2__struct_std_array_unsigned_long__2__0_1 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_193(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_int__3__7__9__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__3__0_1_2 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_int__3__7__9__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__0_ ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_194(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_int__3__7__18446744073709551615__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__3__0_1_2 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_int__3__7__18446744073709551615__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__1__0 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_195(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_int__3__18446744073709551615__18446744073709551615__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__3__0_1_2 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_int__3__18446744073709551615__18446744073709551615__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__2__0_1 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_196(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__7__18446744073709551615__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__3__0_1_2 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__7__18446744073709551615__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__2__0_1 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_197(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__3__0_1_2 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__3__0_1_2 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_198(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_int__3__18446744073709551615__9__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__3__0_1_2 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_int__3__18446744073709551615__9__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__1__0 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_199(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__18446744073709551615__9__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__3__0_1_2 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__18446744073709551615__9__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__2__0_1 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_200(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__7__9__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__3__0_1_2 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__7__9__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__1__0 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_201(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_int__1__2__3__4__5__6__7__8__9__struct_std_array_unsigned_long__9__struct_std_array_unsigned_long__9__0_1_2_3_4_5_6_7_8 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_int__1__2__3__4__5__6__7__8__9__struct_std_array_unsigned_long__9__struct_std_array_unsigned_long__0_ ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_202(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__2__3__18446744073709551615__5__18446744073709551615__7__18446744073709551615__9__struct_std_array_unsigned_long__9__struct_std_array_unsigned_long__9__0_1_2_3_4_5_6_7_8 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__2__3__18446744073709551615__5__18446744073709551615__7__18446744073709551615__9__struct_std_array_unsigned_long__9__struct_std_array_unsigned_long__4__0_1_2_3 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_203(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_unsigned_long__9__struct_std_array_unsigned_long__9__0_1_2_3_4_5_6_7_8 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_unsigned_long__9__struct_std_array_unsigned_long__9__0_1_2_3_4_5_6_7_8 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_204(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_int__struct_std_array_int__0__struct_std_array_int__0_ ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_int__struct_std_array_int__0__struct_std_array_int__0_ ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_205(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_int__3__struct_std_array_int__1__struct_std_array_int__1__0 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_int__3__struct_std_array_int__1__struct_std_array_int__0_ ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_206(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_int__18446744073709551615__struct_std_array_int__1__struct_std_array_int__1__0 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_int__18446744073709551615__struct_std_array_int__1__struct_std_array_int__1__0 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_207(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_int__3__7__struct_std_array_int__2__struct_std_array_int__2__0_1 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_int__3__7__struct_std_array_int__2__struct_std_array_int__0_ ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_208(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_int__3__18446744073709551615__struct_std_array_int__2__struct_std_array_int__2__0_1 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_int__3__18446744073709551615__struct_std_array_int__2__struct_std_array_int__1__0 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_209(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_int__18446744073709551615__7__struct_std_array_int__2__struct_std_array_int__2__0_1 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_int__18446744073709551615__7__struct_std_array_int__2__struct_std_array_int__1__0 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_210(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_int__18446744073709551615__18446744073709551615__struct_std_array_int__2__struct_std_array_int__2__0_1 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_int__18446744073709551615__18446744073709551615__struct_std_array_int__2__struct_std_array_int__2__0_1 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_211(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_int__3__7__9__struct_std_array_int__3__struct_std_array_int__3__0_1_2 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_int__3__7__9__struct_std_array_int__3__struct_std_array_int__0_ ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_212(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_int__3__7__18446744073709551615__struct_std_array_int__3__struct_std_array_int__3__0_1_2 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_int__3__7__18446744073709551615__struct_std_array_int__3__struct_std_array_int__1__0 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_213(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_int__3__18446744073709551615__18446744073709551615__struct_std_array_int__3__struct_std_array_int__3__0_1_2 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_int__3__18446744073709551615__18446744073709551615__struct_std_array_int__3__struct_std_array_int__2__0_1 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_214(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_int__18446744073709551615__7__18446744073709551615__struct_std_array_int__3__struct_std_array_int__3__0_1_2 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_int__18446744073709551615__7__18446744073709551615__struct_std_array_int__3__struct_std_array_int__2__0_1 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_215(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_int__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_int__3__struct_std_array_int__3__0_1_2 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_int__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_int__3__struct_std_array_int__3__0_1_2 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_216(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_int__3__18446744073709551615__9__struct_std_array_int__3__struct_std_array_int__3__0_1_2 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_int__3__18446744073709551615__9__struct_std_array_int__3__struct_std_array_int__1__0 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_217(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_int__18446744073709551615__18446744073709551615__9__struct_std_array_int__3__struct_std_array_int__3__0_1_2 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_int__18446744073709551615__18446744073709551615__9__struct_std_array_int__3__struct_std_array_int__2__0_1 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_218(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_int__18446744073709551615__7__9__struct_std_array_int__3__struct_std_array_int__3__0_1_2 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_int__18446744073709551615__7__9__struct_std_array_int__3__struct_std_array_int__1__0 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_219(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_int__1__2__3__4__5__6__7__8__9__struct_std_array_int__9__struct_std_array_int__9__0_1_2_3_4_5_6_7_8 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_int__1__2__3__4__5__6__7__8__9__struct_std_array_int__9__struct_std_array_int__0_ ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_220(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_int__18446744073709551615__2__3__18446744073709551615__5__18446744073709551615__7__18446744073709551615__9__struct_std_array_int__9__struct_std_array_int__9__0_1_2_3_4_5_6_7_8 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_int__18446744073709551615__2__3__18446744073709551615__5__18446744073709551615__7__18446744073709551615__9__struct_std_array_int__9__struct_std_array_int__4__0_1_2_3 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_221(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_int__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_int__9__struct_std_array_int__9__0_1_2_3_4_5_6_7_8 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_int__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_int__9__struct_std_array_int__9__0_1_2_3_4_5_6_7_8 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_222(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_signed_char__struct_std_array_unsigned_long__0__struct_std_array_unsigned_long__0_ ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_signed_char__struct_std_array_unsigned_long__0__struct_std_array_unsigned_long__0_ ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_223(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_signed_char__3__struct_std_array_unsigned_long__1__struct_std_array_unsigned_long__1__0 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_signed_char__3__struct_std_array_unsigned_long__1__struct_std_array_unsigned_long__0_ ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_224(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_signed_char__18446744073709551615__struct_std_array_unsigned_long__1__struct_std_array_unsigned_long__1__0 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_signed_char__18446744073709551615__struct_std_array_unsigned_long__1__struct_std_array_unsigned_long__1__0 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_225(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_signed_char__3__7__struct_std_array_unsigned_long__2__struct_std_array_unsigned_long__2__0_1 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_signed_char__3__7__struct_std_array_unsigned_long__2__struct_std_array_unsigned_long__0_ ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_226(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_signed_char__3__18446744073709551615__struct_std_array_unsigned_long__2__struct_std_array_unsigned_long__2__0_1 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_signed_char__3__18446744073709551615__struct_std_array_unsigned_long__2__struct_std_array_unsigned_long__1__0 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_227(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_signed_char__18446744073709551615__7__struct_std_array_unsigned_long__2__struct_std_array_unsigned_long__2__0_1 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_signed_char__18446744073709551615__7__struct_std_array_unsigned_long__2__struct_std_array_unsigned_long__1__0 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_228(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_signed_char__18446744073709551615__18446744073709551615__struct_std_array_unsigned_long__2__struct_std_array_unsigned_long__2__0_1 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_signed_char__18446744073709551615__18446744073709551615__struct_std_array_unsigned_long__2__struct_std_array_unsigned_long__2__0_1 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_229(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_signed_char__3__7__9__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__3__0_1_2 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_signed_char__3__7__9__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__0_ ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_230(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_signed_char__3__7__18446744073709551615__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__3__0_1_2 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_signed_char__3__7__18446744073709551615__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__1__0 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_231(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_signed_char__3__18446744073709551615__18446744073709551615__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__3__0_1_2 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_signed_char__3__18446744073709551615__18446744073709551615__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__2__0_1 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_232(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_signed_char__18446744073709551615__7__18446744073709551615__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__3__0_1_2 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_signed_char__18446744073709551615__7__18446744073709551615__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__2__0_1 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_233(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_signed_char__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__3__0_1_2 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_signed_char__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__3__0_1_2 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_234(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_signed_char__3__18446744073709551615__9__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__3__0_1_2 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_signed_char__3__18446744073709551615__9__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__1__0 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_235(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_signed_char__18446744073709551615__18446744073709551615__9__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__3__0_1_2 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_signed_char__18446744073709551615__18446744073709551615__9__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__2__0_1 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_236(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_signed_char__18446744073709551615__7__9__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__3__0_1_2 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_signed_char__18446744073709551615__7__9__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__1__0 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_237(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_signed_char__1__2__3__4__5__6__7__8__9__struct_std_array_unsigned_long__9__struct_std_array_unsigned_long__9__0_1_2_3_4_5_6_7_8 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_signed_char__1__2__3__4__5__6__7__8__9__struct_std_array_unsigned_long__9__struct_std_array_unsigned_long__0_ ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_238(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_signed_char__18446744073709551615__2__3__18446744073709551615__5__18446744073709551615__7__18446744073709551615__9__struct_std_array_unsigned_long__9__struct_std_array_unsigned_long__9__0_1_2_3_4_5_6_7_8 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_signed_char__18446744073709551615__2__3__18446744073709551615__5__18446744073709551615__7__18446744073709551615__9__struct_std_array_unsigned_long__9__struct_std_array_unsigned_long__4__0_1_2_3 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_239(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_signed_char__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_unsigned_long__9__struct_std_array_unsigned_long__9__0_1_2_3_4_5_6_7_8 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_signed_char__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_unsigned_long__9__struct_std_array_unsigned_long__9__0_1_2_3_4_5_6_7_8 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_240(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_long_long__struct_std_array_unsigned_int__0__struct_std_array_unsigned_int__0_ ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_long_long__struct_std_array_unsigned_int__0__struct_std_array_unsigned_int__0_ ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_241(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_long_long__3__struct_std_array_unsigned_int__1__struct_std_array_unsigned_int__1__0 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_long_long__3__struct_std_array_unsigned_int__1__struct_std_array_unsigned_int__0_ ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_242(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_long_long__18446744073709551615__struct_std_array_unsigned_int__1__struct_std_array_unsigned_int__1__0 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_long_long__18446744073709551615__struct_std_array_unsigned_int__1__struct_std_array_unsigned_int__1__0 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_243(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_long_long__3__7__struct_std_array_unsigned_int__2__struct_std_array_unsigned_int__2__0_1 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_long_long__3__7__struct_std_array_unsigned_int__2__struct_std_array_unsigned_int__0_ ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_244(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_long_long__3__18446744073709551615__struct_std_array_unsigned_int__2__struct_std_array_unsigned_int__2__0_1 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_long_long__3__18446744073709551615__struct_std_array_unsigned_int__2__struct_std_array_unsigned_int__1__0 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_245(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_long_long__18446744073709551615__7__struct_std_array_unsigned_int__2__struct_std_array_unsigned_int__2__0_1 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_long_long__18446744073709551615__7__struct_std_array_unsigned_int__2__struct_std_array_unsigned_int__1__0 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_246(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_long_long__18446744073709551615__18446744073709551615__struct_std_array_unsigned_int__2__struct_std_array_unsigned_int__2__0_1 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_long_long__18446744073709551615__18446744073709551615__struct_std_array_unsigned_int__2__struct_std_array_unsigned_int__2__0_1 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_247(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_long_long__3__7__9__struct_std_array_unsigned_int__3__struct_std_array_unsigned_int__3__0_1_2 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_long_long__3__7__9__struct_std_array_unsigned_int__3__struct_std_array_unsigned_int__0_ ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_248(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_long_long__3__7__18446744073709551615__struct_std_array_unsigned_int__3__struct_std_array_unsigned_int__3__0_1_2 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_long_long__3__7__18446744073709551615__struct_std_array_unsigned_int__3__struct_std_array_unsigned_int__1__0 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_249(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_long_long__3__18446744073709551615__18446744073709551615__struct_std_array_unsigned_int__3__struct_std_array_unsigned_int__3__0_1_2 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_long_long__3__18446744073709551615__18446744073709551615__struct_std_array_unsigned_int__3__struct_std_array_unsigned_int__2__0_1 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_250(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_long_long__18446744073709551615__7__18446744073709551615__struct_std_array_unsigned_int__3__struct_std_array_unsigned_int__3__0_1_2 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_long_long__18446744073709551615__7__18446744073709551615__struct_std_array_unsigned_int__3__struct_std_array_unsigned_int__2__0_1 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_251(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_long_long__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_unsigned_int__3__struct_std_array_unsigned_int__3__0_1_2 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_long_long__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_unsigned_int__3__struct_std_array_unsigned_int__3__0_1_2 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_252(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_long_long__3__18446744073709551615__9__struct_std_array_unsigned_int__3__struct_std_array_unsigned_int__3__0_1_2 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_long_long__3__18446744073709551615__9__struct_std_array_unsigned_int__3__struct_std_array_unsigned_int__1__0 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_253(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_long_long__18446744073709551615__18446744073709551615__9__struct_std_array_unsigned_int__3__struct_std_array_unsigned_int__3__0_1_2 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_long_long__18446744073709551615__18446744073709551615__9__struct_std_array_unsigned_int__3__struct_std_array_unsigned_int__2__0_1 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_254(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_long_long__18446744073709551615__7__9__struct_std_array_unsigned_int__3__struct_std_array_unsigned_int__3__0_1_2 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_long_long__18446744073709551615__7__9__struct_std_array_unsigned_int__3__struct_std_array_unsigned_int__1__0 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_255(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_long_long__1__2__3__4__5__6__7__8__9__struct_std_array_unsigned_int__9__struct_std_array_unsigned_int__9__0_1_2_3_4_5_6_7_8 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_long_long__1__2__3__4__5__6__7__8__9__struct_std_array_unsigned_int__9__struct_std_array_unsigned_int__0_ ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_256(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_long_long__18446744073709551615__2__3__18446744073709551615__5__18446744073709551615__7__18446744073709551615__9__struct_std_array_unsigned_int__9__struct_std_array_unsigned_int__9__0_1_2_3_4_5_6_7_8 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_long_long__18446744073709551615__2__3__18446744073709551615__5__18446744073709551615__7__18446744073709551615__9__struct_std_array_unsigned_int__9__struct_std_array_unsigned_int__4__0_1_2_3 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_257(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_long_long__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_unsigned_int__9__struct_std_array_unsigned_int__9__0_1_2_3_4_5_6_7_8 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_long_long__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_unsigned_int__9__struct_std_array_unsigned_int__9__0_1_2_3_4_5_6_7_8 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_258(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_long__struct_std_array_int__0__struct_std_array_int__0_ ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_long__struct_std_array_int__0__struct_std_array_int__0_ ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_259(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_long__3__struct_std_array_int__1__struct_std_array_int__1__0 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_long__3__struct_std_array_int__1__struct_std_array_int__0_ ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_260(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_long__18446744073709551615__struct_std_array_int__1__struct_std_array_int__1__0 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_long__18446744073709551615__struct_std_array_int__1__struct_std_array_int__1__0 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_261(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_long__3__7__struct_std_array_int__2__struct_std_array_int__2__0_1 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_long__3__7__struct_std_array_int__2__struct_std_array_int__0_ ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_262(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_long__3__18446744073709551615__struct_std_array_int__2__struct_std_array_int__2__0_1 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_long__3__18446744073709551615__struct_std_array_int__2__struct_std_array_int__1__0 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_263(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_long__18446744073709551615__7__struct_std_array_int__2__struct_std_array_int__2__0_1 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_long__18446744073709551615__7__struct_std_array_int__2__struct_std_array_int__1__0 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_264(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_long__18446744073709551615__18446744073709551615__struct_std_array_int__2__struct_std_array_int__2__0_1 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_long__18446744073709551615__18446744073709551615__struct_std_array_int__2__struct_std_array_int__2__0_1 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_265(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_long__3__7__9__struct_std_array_int__3__struct_std_array_int__3__0_1_2 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_long__3__7__9__struct_std_array_int__3__struct_std_array_int__0_ ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_266(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_long__3__7__18446744073709551615__struct_std_array_int__3__struct_std_array_int__3__0_1_2 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_long__3__7__18446744073709551615__struct_std_array_int__3__struct_std_array_int__1__0 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_267(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_long__3__18446744073709551615__18446744073709551615__struct_std_array_int__3__struct_std_array_int__3__0_1_2 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_long__3__18446744073709551615__18446744073709551615__struct_std_array_int__3__struct_std_array_int__2__0_1 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_268(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_long__18446744073709551615__7__18446744073709551615__struct_std_array_int__3__struct_std_array_int__3__0_1_2 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_long__18446744073709551615__7__18446744073709551615__struct_std_array_int__3__struct_std_array_int__2__0_1 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_269(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_long__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_int__3__struct_std_array_int__3__0_1_2 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_long__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_int__3__struct_std_array_int__3__0_1_2 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_270(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_long__3__18446744073709551615__9__struct_std_array_int__3__struct_std_array_int__3__0_1_2 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_long__3__18446744073709551615__9__struct_std_array_int__3__struct_std_array_int__1__0 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_271(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_long__18446744073709551615__18446744073709551615__9__struct_std_array_int__3__struct_std_array_int__3__0_1_2 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_long__18446744073709551615__18446744073709551615__9__struct_std_array_int__3__struct_std_array_int__2__0_1 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_272(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_long__18446744073709551615__7__9__struct_std_array_int__3__struct_std_array_int__3__0_1_2 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_long__18446744073709551615__7__9__struct_std_array_int__3__struct_std_array_int__1__0 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_273(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_long__1__2__3__4__5__6__7__8__9__struct_std_array_int__9__struct_std_array_int__9__0_1_2_3_4_5_6_7_8 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_long__1__2__3__4__5__6__7__8__9__struct_std_array_int__9__struct_std_array_int__0_ ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_274(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_long__18446744073709551615__2__3__18446744073709551615__5__18446744073709551615__7__18446744073709551615__9__struct_std_array_int__9__struct_std_array_int__9__0_1_2_3_4_5_6_7_8 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_long__18446744073709551615__2__3__18446744073709551615__5__18446744073709551615__7__18446744073709551615__9__struct_std_array_int__9__struct_std_array_int__4__0_1_2_3 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_275(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_long__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_int__9__struct_std_array_int__9__0_1_2_3_4_5_6_7_8 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_long__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_int__9__struct_std_array_int__9__0_1_2_3_4_5_6_7_8 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_276(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_long__struct_std_array_unsigned_long__0__struct_std_array_unsigned_long__0_ ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_long__struct_std_array_unsigned_long__0__struct_std_array_unsigned_long__0_ ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_277(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_long__3__struct_std_array_unsigned_long__1__struct_std_array_unsigned_long__1__0 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_long__3__struct_std_array_unsigned_long__1__struct_std_array_unsigned_long__0_ ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_278(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_long__18446744073709551615__struct_std_array_unsigned_long__1__struct_std_array_unsigned_long__1__0 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_long__18446744073709551615__struct_std_array_unsigned_long__1__struct_std_array_unsigned_long__1__0 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_279(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_long__3__7__struct_std_array_unsigned_long__2__struct_std_array_unsigned_long__2__0_1 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_long__3__7__struct_std_array_unsigned_long__2__struct_std_array_unsigned_long__0_ ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_280(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_long__3__18446744073709551615__struct_std_array_unsigned_long__2__struct_std_array_unsigned_long__2__0_1 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_long__3__18446744073709551615__struct_std_array_unsigned_long__2__struct_std_array_unsigned_long__1__0 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_281(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_long__18446744073709551615__7__struct_std_array_unsigned_long__2__struct_std_array_unsigned_long__2__0_1 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_long__18446744073709551615__7__struct_std_array_unsigned_long__2__struct_std_array_unsigned_long__1__0 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_282(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_long__18446744073709551615__18446744073709551615__struct_std_array_unsigned_long__2__struct_std_array_unsigned_long__2__0_1 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_long__18446744073709551615__18446744073709551615__struct_std_array_unsigned_long__2__struct_std_array_unsigned_long__2__0_1 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_283(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_long__3__7__9__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__3__0_1_2 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_long__3__7__9__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__0_ ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_284(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_long__3__7__18446744073709551615__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__3__0_1_2 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_long__3__7__18446744073709551615__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__1__0 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_285(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_long__3__18446744073709551615__18446744073709551615__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__3__0_1_2 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_long__3__18446744073709551615__18446744073709551615__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__2__0_1 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_286(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_long__18446744073709551615__7__18446744073709551615__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__3__0_1_2 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_long__18446744073709551615__7__18446744073709551615__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__2__0_1 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_287(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_long__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__3__0_1_2 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_long__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__3__0_1_2 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_288(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_long__3__18446744073709551615__9__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__3__0_1_2 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_long__3__18446744073709551615__9__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__1__0 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_289(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_long__18446744073709551615__18446744073709551615__9__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__3__0_1_2 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_long__18446744073709551615__18446744073709551615__9__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__2__0_1 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_290(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_long__18446744073709551615__7__9__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__3__0_1_2 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_long__18446744073709551615__7__9__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__1__0 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_291(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_long__1__2__3__4__5__6__7__8__9__struct_std_array_unsigned_long__9__struct_std_array_unsigned_long__9__0_1_2_3_4_5_6_7_8 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_long__1__2__3__4__5__6__7__8__9__struct_std_array_unsigned_long__9__struct_std_array_unsigned_long__0_ ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_292(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_long__18446744073709551615__2__3__18446744073709551615__5__18446744073709551615__7__18446744073709551615__9__struct_std_array_unsigned_long__9__struct_std_array_unsigned_long__9__0_1_2_3_4_5_6_7_8 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_long__18446744073709551615__2__3__18446744073709551615__5__18446744073709551615__7__18446744073709551615__9__struct_std_array_unsigned_long__9__struct_std_array_unsigned_long__4__0_1_2_3 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_293(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_long__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_unsigned_long__9__struct_std_array_unsigned_long__9__0_1_2_3_4_5_6_7_8 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (unimplemented!().read());
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_unsigned_long__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_unsigned_long__9__struct_std_array_unsigned_long__9__0_1_2_3_4_5_6_7_8 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_294(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_int__struct_std_array_struct_IntType__0__struct_std_array_struct_IntType__0_ ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (*unimplemented!().upgrade().deref()).clone();
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_int__struct_std_array_struct_IntType__0__struct_std_array_struct_IntType__0_ ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_295(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_int__3__struct_std_array_struct_IntType__1__struct_std_array_struct_IntType__1__0 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (*unimplemented!().upgrade().deref()).clone();
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_int__3__struct_std_array_struct_IntType__1__struct_std_array_struct_IntType__0_ ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_296(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__struct_std_array_struct_IntType__1__struct_std_array_struct_IntType__1__0 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (*unimplemented!().upgrade().deref()).clone();
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__struct_std_array_struct_IntType__1__struct_std_array_struct_IntType__1__0 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_297(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_int__3__7__struct_std_array_struct_IntType__2__struct_std_array_struct_IntType__2__0_1 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (*unimplemented!().upgrade().deref()).clone();
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_int__3__7__struct_std_array_struct_IntType__2__struct_std_array_struct_IntType__0_ ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_298(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_int__3__18446744073709551615__struct_std_array_struct_IntType__2__struct_std_array_struct_IntType__2__0_1 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (*unimplemented!().upgrade().deref()).clone();
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_int__3__18446744073709551615__struct_std_array_struct_IntType__2__struct_std_array_struct_IntType__1__0 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_299(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__7__struct_std_array_struct_IntType__2__struct_std_array_struct_IntType__2__0_1 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (*unimplemented!().upgrade().deref()).clone();
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__7__struct_std_array_struct_IntType__2__struct_std_array_struct_IntType__1__0 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_300(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__18446744073709551615__struct_std_array_struct_IntType__2__struct_std_array_struct_IntType__2__0_1 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (*unimplemented!().upgrade().deref()).clone();
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__18446744073709551615__struct_std_array_struct_IntType__2__struct_std_array_struct_IntType__2__0_1 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_301(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_int__3__7__9__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__3__0_1_2 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (*unimplemented!().upgrade().deref()).clone();
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_int__3__7__9__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__0_ ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_302(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_int__3__7__18446744073709551615__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__3__0_1_2 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (*unimplemented!().upgrade().deref()).clone();
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_int__3__7__18446744073709551615__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__1__0 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_303(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_int__3__18446744073709551615__18446744073709551615__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__3__0_1_2 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (*unimplemented!().upgrade().deref()).clone();
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_int__3__18446744073709551615__18446744073709551615__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__2__0_1 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_304(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__7__18446744073709551615__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__3__0_1_2 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (*unimplemented!().upgrade().deref()).clone();
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__7__18446744073709551615__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__2__0_1 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_305(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__3__0_1_2 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (*unimplemented!().upgrade().deref()).clone();
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__3__0_1_2 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_306(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_int__3__18446744073709551615__9__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__3__0_1_2 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (*unimplemented!().upgrade().deref()).clone();
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_int__3__18446744073709551615__9__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__1__0 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_307(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__18446744073709551615__9__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__3__0_1_2 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (*unimplemented!().upgrade().deref()).clone();
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__18446744073709551615__9__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__2__0_1 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_308(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__7__9__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__3__0_1_2 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (*unimplemented!().upgrade().deref()).clone();
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__7__9__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__1__0 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_309(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_int__1__2__3__4__5__6__7__8__9__struct_std_array_struct_IntType__9__struct_std_array_struct_IntType__9__0_1_2_3_4_5_6_7_8 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (*unimplemented!().upgrade().deref()).clone();
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_int__1__2__3__4__5__6__7__8__9__struct_std_array_struct_IntType__9__struct_std_array_struct_IntType__0_ ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_310(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__2__3__18446744073709551615__5__18446744073709551615__7__18446744073709551615__9__struct_std_array_struct_IntType__9__struct_std_array_struct_IntType__9__0_1_2_3_4_5_6_7_8 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (*unimplemented!().upgrade().deref()).clone();
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__2__3__18446744073709551615__5__18446744073709551615__7__18446744073709551615__9__struct_std_array_struct_IntType__9__struct_std_array_struct_IntType__4__0_1_2_3 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_311(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_struct_IntType__9__struct_std_array_struct_IntType__9__0_1_2_3_4_5_6_7_8 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (*unimplemented!().upgrade().deref()).clone();
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_int__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_struct_IntType__9__struct_std_array_struct_IntType__9__0_1_2_3_4_5_6_7_8 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_312(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_signed_char__struct_std_array_struct_IntType__0__struct_std_array_struct_IntType__0_ ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (*unimplemented!().upgrade().deref()).clone();
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_signed_char__struct_std_array_struct_IntType__0__struct_std_array_struct_IntType__0_ ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_313(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_signed_char__3__struct_std_array_struct_IntType__1__struct_std_array_struct_IntType__1__0 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (*unimplemented!().upgrade().deref()).clone();
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_signed_char__3__struct_std_array_struct_IntType__1__struct_std_array_struct_IntType__0_ ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_314(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_signed_char__18446744073709551615__struct_std_array_struct_IntType__1__struct_std_array_struct_IntType__1__0 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (*unimplemented!().upgrade().deref()).clone();
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_signed_char__18446744073709551615__struct_std_array_struct_IntType__1__struct_std_array_struct_IntType__1__0 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_315(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_signed_char__3__7__struct_std_array_struct_IntType__2__struct_std_array_struct_IntType__2__0_1 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (*unimplemented!().upgrade().deref()).clone();
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_signed_char__3__7__struct_std_array_struct_IntType__2__struct_std_array_struct_IntType__0_ ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_316(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_signed_char__3__18446744073709551615__struct_std_array_struct_IntType__2__struct_std_array_struct_IntType__2__0_1 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (*unimplemented!().upgrade().deref()).clone();
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_signed_char__3__18446744073709551615__struct_std_array_struct_IntType__2__struct_std_array_struct_IntType__1__0 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_317(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_signed_char__18446744073709551615__7__struct_std_array_struct_IntType__2__struct_std_array_struct_IntType__2__0_1 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (*unimplemented!().upgrade().deref()).clone();
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_signed_char__18446744073709551615__7__struct_std_array_struct_IntType__2__struct_std_array_struct_IntType__1__0 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_318(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_signed_char__18446744073709551615__18446744073709551615__struct_std_array_struct_IntType__2__struct_std_array_struct_IntType__2__0_1 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (*unimplemented!().upgrade().deref()).clone();
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_signed_char__18446744073709551615__18446744073709551615__struct_std_array_struct_IntType__2__struct_std_array_struct_IntType__2__0_1 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_319(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_signed_char__3__7__9__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__3__0_1_2 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (*unimplemented!().upgrade().deref()).clone();
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_signed_char__3__7__9__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__0_ ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_320(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_signed_char__3__7__18446744073709551615__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__3__0_1_2 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (*unimplemented!().upgrade().deref()).clone();
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_signed_char__3__7__18446744073709551615__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__1__0 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_321(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_signed_char__3__18446744073709551615__18446744073709551615__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__3__0_1_2 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (*unimplemented!().upgrade().deref()).clone();
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_signed_char__3__18446744073709551615__18446744073709551615__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__2__0_1 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_322(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_signed_char__18446744073709551615__7__18446744073709551615__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__3__0_1_2 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (*unimplemented!().upgrade().deref()).clone();
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_signed_char__18446744073709551615__7__18446744073709551615__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__2__0_1 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_323(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_signed_char__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__3__0_1_2 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (*unimplemented!().upgrade().deref()).clone();
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_signed_char__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__3__0_1_2 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_324(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_signed_char__3__18446744073709551615__9__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__3__0_1_2 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (*unimplemented!().upgrade().deref()).clone();
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_signed_char__3__18446744073709551615__9__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__1__0 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_325(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_signed_char__18446744073709551615__18446744073709551615__9__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__3__0_1_2 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (*unimplemented!().upgrade().deref()).clone();
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_signed_char__18446744073709551615__18446744073709551615__9__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__2__0_1 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_326(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_signed_char__18446744073709551615__7__9__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__3__0_1_2 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (*unimplemented!().upgrade().deref()).clone();
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_signed_char__18446744073709551615__7__9__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__1__0 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_327(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_signed_char__1__2__3__4__5__6__7__8__9__struct_std_array_struct_IntType__9__struct_std_array_struct_IntType__9__0_1_2_3_4_5_6_7_8 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (*unimplemented!().upgrade().deref()).clone();
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_signed_char__1__2__3__4__5__6__7__8__9__struct_std_array_struct_IntType__9__struct_std_array_struct_IntType__0_ ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_328(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_signed_char__18446744073709551615__2__3__18446744073709551615__5__18446744073709551615__7__18446744073709551615__9__struct_std_array_struct_IntType__9__struct_std_array_struct_IntType__9__0_1_2_3_4_5_6_7_8 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (*unimplemented!().upgrade().deref()).clone();
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_signed_char__18446744073709551615__2__3__18446744073709551615__5__18446744073709551615__7__18446744073709551615__9__struct_std_array_struct_IntType__9__struct_std_array_struct_IntType__4__0_1_2_3 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_construction_329(all_ext: ()) {
    let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
    ({
        let _all_ext: () = (*all_ext.borrow()).clone();
        let _arg1: () = (*all_ext.borrow()).clone();
        DefaultCtorTest::test_construction_1_class_std_extents_signed_char__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_struct_IntType__9__struct_std_array_struct_IntType__9__0_1_2_3_4_5_6_7_8 ( _all_ext , _arg1 , <()  >::default()  , )
    });
    let dyn_ext: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut dynamic_idx: () = 0_();
    let mut r: () = 0_();
    'loop_: while (r < unimplemented!()) {
        if (unimplemented!() == dynamic_extent_168.with(|rc| *rc.borrow())) {
            let __rhs = (*unimplemented!().upgrade().deref()).clone();
            unimplemented!().write(__rhs);
            dynamic_idx.postfix_inc();
        }
        r.postfix_inc();
    }
    ({
        DefaultCtorTest::test_construction_1_class_std_extents_signed_char__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_struct_IntType__9__struct_std_array_struct_IntType__9__0_1_2_3_4_5_6_7_8 ( ((*all_ext.borrow()) ).clone()  , ((*dyn_ext.borrow()) ).clone()  , <()  >::default()  , )
    });
}
pub fn test_330() {
    let mut D: () = dynamic_extent_168.with(|rc| *rc.borrow());
    ({ test_construction_167(unimplemented!()) });
    ({ test_construction_169(unimplemented!()) });
    ({ test_construction_170(unimplemented!()) });
    ({ test_construction_171(unimplemented!()) });
    ({ test_construction_172(unimplemented!()) });
    ({ test_construction_173(unimplemented!()) });
    ({ test_construction_174(unimplemented!()) });
    ({ test_construction_175(unimplemented!()) });
    ({ test_construction_176(unimplemented!()) });
    ({ test_construction_177(unimplemented!()) });
    ({ test_construction_178(unimplemented!()) });
    ({ test_construction_179(unimplemented!()) });
    ({ test_construction_180(unimplemented!()) });
    ({ test_construction_181(unimplemented!()) });
    ({ test_construction_182(unimplemented!()) });
    ({ test_construction_183(unimplemented!()) });
    ({ test_construction_184(unimplemented!()) });
    ({ test_construction_185(unimplemented!()) });
}
pub fn test_331() {
    let mut D: () = dynamic_extent_168.with(|rc| *rc.borrow());
    ({ test_construction_186(unimplemented!()) });
    ({ test_construction_187(unimplemented!()) });
    ({ test_construction_188(unimplemented!()) });
    ({ test_construction_189(unimplemented!()) });
    ({ test_construction_190(unimplemented!()) });
    ({ test_construction_191(unimplemented!()) });
    ({ test_construction_192(unimplemented!()) });
    ({ test_construction_193(unimplemented!()) });
    ({ test_construction_194(unimplemented!()) });
    ({ test_construction_195(unimplemented!()) });
    ({ test_construction_196(unimplemented!()) });
    ({ test_construction_197(unimplemented!()) });
    ({ test_construction_198(unimplemented!()) });
    ({ test_construction_199(unimplemented!()) });
    ({ test_construction_200(unimplemented!()) });
    ({ test_construction_201(unimplemented!()) });
    ({ test_construction_202(unimplemented!()) });
    ({ test_construction_203(unimplemented!()) });
}
pub fn test_332() {
    let mut D: () = dynamic_extent_168.with(|rc| *rc.borrow());
    ({ test_construction_204(unimplemented!()) });
    ({ test_construction_205(unimplemented!()) });
    ({ test_construction_206(unimplemented!()) });
    ({ test_construction_207(unimplemented!()) });
    ({ test_construction_208(unimplemented!()) });
    ({ test_construction_209(unimplemented!()) });
    ({ test_construction_210(unimplemented!()) });
    ({ test_construction_211(unimplemented!()) });
    ({ test_construction_212(unimplemented!()) });
    ({ test_construction_213(unimplemented!()) });
    ({ test_construction_214(unimplemented!()) });
    ({ test_construction_215(unimplemented!()) });
    ({ test_construction_216(unimplemented!()) });
    ({ test_construction_217(unimplemented!()) });
    ({ test_construction_218(unimplemented!()) });
    ({ test_construction_219(unimplemented!()) });
    ({ test_construction_220(unimplemented!()) });
    ({ test_construction_221(unimplemented!()) });
}
pub fn test_333() {
    let mut D: () = dynamic_extent_168.with(|rc| *rc.borrow());
    ({ test_construction_222(unimplemented!()) });
    ({ test_construction_223(unimplemented!()) });
    ({ test_construction_224(unimplemented!()) });
    ({ test_construction_225(unimplemented!()) });
    ({ test_construction_226(unimplemented!()) });
    ({ test_construction_227(unimplemented!()) });
    ({ test_construction_228(unimplemented!()) });
    ({ test_construction_229(unimplemented!()) });
    ({ test_construction_230(unimplemented!()) });
    ({ test_construction_231(unimplemented!()) });
    ({ test_construction_232(unimplemented!()) });
    ({ test_construction_233(unimplemented!()) });
    ({ test_construction_234(unimplemented!()) });
    ({ test_construction_235(unimplemented!()) });
    ({ test_construction_236(unimplemented!()) });
    ({ test_construction_237(unimplemented!()) });
    ({ test_construction_238(unimplemented!()) });
    ({ test_construction_239(unimplemented!()) });
}
pub fn test_334() {
    let mut D: () = dynamic_extent_168.with(|rc| *rc.borrow());
    ({ test_construction_240(unimplemented!()) });
    ({ test_construction_241(unimplemented!()) });
    ({ test_construction_242(unimplemented!()) });
    ({ test_construction_243(unimplemented!()) });
    ({ test_construction_244(unimplemented!()) });
    ({ test_construction_245(unimplemented!()) });
    ({ test_construction_246(unimplemented!()) });
    ({ test_construction_247(unimplemented!()) });
    ({ test_construction_248(unimplemented!()) });
    ({ test_construction_249(unimplemented!()) });
    ({ test_construction_250(unimplemented!()) });
    ({ test_construction_251(unimplemented!()) });
    ({ test_construction_252(unimplemented!()) });
    ({ test_construction_253(unimplemented!()) });
    ({ test_construction_254(unimplemented!()) });
    ({ test_construction_255(unimplemented!()) });
    ({ test_construction_256(unimplemented!()) });
    ({ test_construction_257(unimplemented!()) });
}
pub fn test_335() {
    let mut D: () = dynamic_extent_168.with(|rc| *rc.borrow());
    ({ test_construction_258(unimplemented!()) });
    ({ test_construction_259(unimplemented!()) });
    ({ test_construction_260(unimplemented!()) });
    ({ test_construction_261(unimplemented!()) });
    ({ test_construction_262(unimplemented!()) });
    ({ test_construction_263(unimplemented!()) });
    ({ test_construction_264(unimplemented!()) });
    ({ test_construction_265(unimplemented!()) });
    ({ test_construction_266(unimplemented!()) });
    ({ test_construction_267(unimplemented!()) });
    ({ test_construction_268(unimplemented!()) });
    ({ test_construction_269(unimplemented!()) });
    ({ test_construction_270(unimplemented!()) });
    ({ test_construction_271(unimplemented!()) });
    ({ test_construction_272(unimplemented!()) });
    ({ test_construction_273(unimplemented!()) });
    ({ test_construction_274(unimplemented!()) });
    ({ test_construction_275(unimplemented!()) });
}
pub fn test_336() {
    let mut D: () = dynamic_extent_168.with(|rc| *rc.borrow());
    ({ test_construction_276(unimplemented!()) });
    ({ test_construction_277(unimplemented!()) });
    ({ test_construction_278(unimplemented!()) });
    ({ test_construction_279(unimplemented!()) });
    ({ test_construction_280(unimplemented!()) });
    ({ test_construction_281(unimplemented!()) });
    ({ test_construction_282(unimplemented!()) });
    ({ test_construction_283(unimplemented!()) });
    ({ test_construction_284(unimplemented!()) });
    ({ test_construction_285(unimplemented!()) });
    ({ test_construction_286(unimplemented!()) });
    ({ test_construction_287(unimplemented!()) });
    ({ test_construction_288(unimplemented!()) });
    ({ test_construction_289(unimplemented!()) });
    ({ test_construction_290(unimplemented!()) });
    ({ test_construction_291(unimplemented!()) });
    ({ test_construction_292(unimplemented!()) });
    ({ test_construction_293(unimplemented!()) });
}
pub fn test_337() {
    let mut D: () = dynamic_extent_168.with(|rc| *rc.borrow());
    ({ test_construction_294(unimplemented!()) });
    ({ test_construction_295(unimplemented!()) });
    ({ test_construction_296(unimplemented!()) });
    ({ test_construction_297(unimplemented!()) });
    ({ test_construction_298(unimplemented!()) });
    ({ test_construction_299(unimplemented!()) });
    ({ test_construction_300(unimplemented!()) });
    ({ test_construction_301(unimplemented!()) });
    ({ test_construction_302(unimplemented!()) });
    ({ test_construction_303(unimplemented!()) });
    ({ test_construction_304(unimplemented!()) });
    ({ test_construction_305(unimplemented!()) });
    ({ test_construction_306(unimplemented!()) });
    ({ test_construction_307(unimplemented!()) });
    ({ test_construction_308(unimplemented!()) });
    ({ test_construction_309(unimplemented!()) });
    ({ test_construction_310(unimplemented!()) });
    ({ test_construction_311(unimplemented!()) });
}
pub fn test_338() {
    let mut D: () = dynamic_extent_168.with(|rc| *rc.borrow());
    ({ test_construction_312(unimplemented!()) });
    ({ test_construction_313(unimplemented!()) });
    ({ test_construction_314(unimplemented!()) });
    ({ test_construction_315(unimplemented!()) });
    ({ test_construction_316(unimplemented!()) });
    ({ test_construction_317(unimplemented!()) });
    ({ test_construction_318(unimplemented!()) });
    ({ test_construction_319(unimplemented!()) });
    ({ test_construction_320(unimplemented!()) });
    ({ test_construction_321(unimplemented!()) });
    ({ test_construction_322(unimplemented!()) });
    ({ test_construction_323(unimplemented!()) });
    ({ test_construction_324(unimplemented!()) });
    ({ test_construction_325(unimplemented!()) });
    ({ test_construction_326(unimplemented!()) });
    ({ test_construction_327(unimplemented!()) });
    ({ test_construction_328(unimplemented!()) });
    ({ test_construction_329(unimplemented!()) });
}
pub fn test_index_type_combo_339() -> () {
    ({ test_330() });
    ({ test_331() });
    ({ test_332() });
    ({ test_333() });
    ({ test_334() });
    ({ test_335() });
    ({ test_336() });
    ({ test_337() });
    ({ test_338() });
    return true;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct DefaultCtorTest {}
impl DefaultCtorTest {
    pub fn test_construction_1_class_std_extents_int__struct_std_array_int__0__struct_std_array_int__0_(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_5(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_int__3__struct_std_array_int__1__struct_std_array_int__1__0(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_6(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_int__3__struct_std_array_int__1__struct_std_array_int__0_(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_int__18446744073709551615__struct_std_array_int__1__struct_std_array_int__1__0(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_7(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_int__3__7__struct_std_array_int__2__struct_std_array_int__2__0_1(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_8(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_int__3__7__struct_std_array_int__2__struct_std_array_int__0_(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_int__3__18446744073709551615__struct_std_array_int__2__struct_std_array_int__2__0_1(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_9(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_int__3__18446744073709551615__struct_std_array_int__2__struct_std_array_int__1__0(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_int__18446744073709551615__7__struct_std_array_int__2__struct_std_array_int__2__0_1(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_10(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_int__18446744073709551615__7__struct_std_array_int__2__struct_std_array_int__1__0(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_int__18446744073709551615__18446744073709551615__struct_std_array_int__2__struct_std_array_int__2__0_1(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_11(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_int__3__7__9__struct_std_array_int__3__struct_std_array_int__3__0_1_2(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_12(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_int__3__7__9__struct_std_array_int__3__struct_std_array_int__0_(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_int__3__7__18446744073709551615__struct_std_array_int__3__struct_std_array_int__3__0_1_2(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_13(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_int__3__7__18446744073709551615__struct_std_array_int__3__struct_std_array_int__1__0(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_int__3__18446744073709551615__18446744073709551615__struct_std_array_int__3__struct_std_array_int__3__0_1_2(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_14(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_int__3__18446744073709551615__18446744073709551615__struct_std_array_int__3__struct_std_array_int__2__0_1(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_int__18446744073709551615__7__18446744073709551615__struct_std_array_int__3__struct_std_array_int__3__0_1_2(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_15(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_int__18446744073709551615__7__18446744073709551615__struct_std_array_int__3__struct_std_array_int__2__0_1(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_int__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_int__3__struct_std_array_int__3__0_1_2(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_16(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_int__3__18446744073709551615__9__struct_std_array_int__3__struct_std_array_int__3__0_1_2(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_17(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_int__3__18446744073709551615__9__struct_std_array_int__3__struct_std_array_int__1__0(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_int__18446744073709551615__18446744073709551615__9__struct_std_array_int__3__struct_std_array_int__3__0_1_2(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_18(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_int__18446744073709551615__18446744073709551615__9__struct_std_array_int__3__struct_std_array_int__2__0_1(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_int__18446744073709551615__7__9__struct_std_array_int__3__struct_std_array_int__3__0_1_2(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_19(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_int__18446744073709551615__7__9__struct_std_array_int__3__struct_std_array_int__1__0(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_int__1__2__3__4__5__6__7__8__9__struct_std_array_int__9__struct_std_array_int__9__0_1_2_3_4_5_6_7_8(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_20(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_int__1__2__3__4__5__6__7__8__9__struct_std_array_int__9__struct_std_array_int__0_(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_int__18446744073709551615__2__3__18446744073709551615__5__18446744073709551615__7__18446744073709551615__9__struct_std_array_int__9__struct_std_array_int__9__0_1_2_3_4_5_6_7_8(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_21(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_int__18446744073709551615__2__3__18446744073709551615__5__18446744073709551615__7__18446744073709551615__9__struct_std_array_int__9__struct_std_array_int__4__0_1_2_3(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_int__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_int__9__struct_std_array_int__9__0_1_2_3_4_5_6_7_8(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_22(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_int__struct_std_array_unsigned_long__0__struct_std_array_unsigned_long__0_(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_23(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_int__3__struct_std_array_unsigned_long__1__struct_std_array_unsigned_long__1__0(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_24(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_int__3__struct_std_array_unsigned_long__1__struct_std_array_unsigned_long__0_(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_int__18446744073709551615__struct_std_array_unsigned_long__1__struct_std_array_unsigned_long__1__0(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_25(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_int__3__7__struct_std_array_unsigned_long__2__struct_std_array_unsigned_long__2__0_1(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_26(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_int__3__7__struct_std_array_unsigned_long__2__struct_std_array_unsigned_long__0_(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_int__3__18446744073709551615__struct_std_array_unsigned_long__2__struct_std_array_unsigned_long__2__0_1(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_27(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_int__3__18446744073709551615__struct_std_array_unsigned_long__2__struct_std_array_unsigned_long__1__0(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_int__18446744073709551615__7__struct_std_array_unsigned_long__2__struct_std_array_unsigned_long__2__0_1(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_28(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_int__18446744073709551615__7__struct_std_array_unsigned_long__2__struct_std_array_unsigned_long__1__0(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_int__18446744073709551615__18446744073709551615__struct_std_array_unsigned_long__2__struct_std_array_unsigned_long__2__0_1(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_29(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_int__3__7__9__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__3__0_1_2(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_30(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_int__3__7__9__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__0_(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_int__3__7__18446744073709551615__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__3__0_1_2(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_31(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_int__3__7__18446744073709551615__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__1__0(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_int__3__18446744073709551615__18446744073709551615__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__3__0_1_2(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_32(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_int__3__18446744073709551615__18446744073709551615__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__2__0_1(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_int__18446744073709551615__7__18446744073709551615__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__3__0_1_2(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_33(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_int__18446744073709551615__7__18446744073709551615__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__2__0_1(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_int__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__3__0_1_2(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_34(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_int__3__18446744073709551615__9__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__3__0_1_2(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_35(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_int__3__18446744073709551615__9__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__1__0(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_int__18446744073709551615__18446744073709551615__9__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__3__0_1_2(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_36(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_int__18446744073709551615__18446744073709551615__9__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__2__0_1(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_int__18446744073709551615__7__9__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__3__0_1_2(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_37(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_int__18446744073709551615__7__9__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__1__0(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_int__1__2__3__4__5__6__7__8__9__struct_std_array_unsigned_long__9__struct_std_array_unsigned_long__9__0_1_2_3_4_5_6_7_8(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_38(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_int__1__2__3__4__5__6__7__8__9__struct_std_array_unsigned_long__9__struct_std_array_unsigned_long__0_(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_int__18446744073709551615__2__3__18446744073709551615__5__18446744073709551615__7__18446744073709551615__9__struct_std_array_unsigned_long__9__struct_std_array_unsigned_long__9__0_1_2_3_4_5_6_7_8(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_39(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_int__18446744073709551615__2__3__18446744073709551615__5__18446744073709551615__7__18446744073709551615__9__struct_std_array_unsigned_long__9__struct_std_array_unsigned_long__4__0_1_2_3(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_int__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_unsigned_long__9__struct_std_array_unsigned_long__9__0_1_2_3_4_5_6_7_8(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_40(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_unsigned_int__struct_std_array_int__0__struct_std_array_int__0_(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_41(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_unsigned_int__3__struct_std_array_int__1__struct_std_array_int__1__0(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_42(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_unsigned_int__3__struct_std_array_int__1__struct_std_array_int__0_(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_unsigned_int__18446744073709551615__struct_std_array_int__1__struct_std_array_int__1__0(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_43(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_unsigned_int__3__7__struct_std_array_int__2__struct_std_array_int__2__0_1(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_44(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_unsigned_int__3__7__struct_std_array_int__2__struct_std_array_int__0_(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_unsigned_int__3__18446744073709551615__struct_std_array_int__2__struct_std_array_int__2__0_1(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_45(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_unsigned_int__3__18446744073709551615__struct_std_array_int__2__struct_std_array_int__1__0(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_unsigned_int__18446744073709551615__7__struct_std_array_int__2__struct_std_array_int__2__0_1(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_46(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_unsigned_int__18446744073709551615__7__struct_std_array_int__2__struct_std_array_int__1__0(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_unsigned_int__18446744073709551615__18446744073709551615__struct_std_array_int__2__struct_std_array_int__2__0_1(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_47(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_unsigned_int__3__7__9__struct_std_array_int__3__struct_std_array_int__3__0_1_2(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_48(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_unsigned_int__3__7__9__struct_std_array_int__3__struct_std_array_int__0_(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_unsigned_int__3__7__18446744073709551615__struct_std_array_int__3__struct_std_array_int__3__0_1_2(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_49(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_unsigned_int__3__7__18446744073709551615__struct_std_array_int__3__struct_std_array_int__1__0(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_unsigned_int__3__18446744073709551615__18446744073709551615__struct_std_array_int__3__struct_std_array_int__3__0_1_2(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_50(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_unsigned_int__3__18446744073709551615__18446744073709551615__struct_std_array_int__3__struct_std_array_int__2__0_1(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_unsigned_int__18446744073709551615__7__18446744073709551615__struct_std_array_int__3__struct_std_array_int__3__0_1_2(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_51(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_unsigned_int__18446744073709551615__7__18446744073709551615__struct_std_array_int__3__struct_std_array_int__2__0_1(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_unsigned_int__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_int__3__struct_std_array_int__3__0_1_2(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_52(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_unsigned_int__3__18446744073709551615__9__struct_std_array_int__3__struct_std_array_int__3__0_1_2(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_53(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_unsigned_int__3__18446744073709551615__9__struct_std_array_int__3__struct_std_array_int__1__0(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_unsigned_int__18446744073709551615__18446744073709551615__9__struct_std_array_int__3__struct_std_array_int__3__0_1_2(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_54(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_unsigned_int__18446744073709551615__18446744073709551615__9__struct_std_array_int__3__struct_std_array_int__2__0_1(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_unsigned_int__18446744073709551615__7__9__struct_std_array_int__3__struct_std_array_int__3__0_1_2(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_55(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_unsigned_int__18446744073709551615__7__9__struct_std_array_int__3__struct_std_array_int__1__0(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_unsigned_int__1__2__3__4__5__6__7__8__9__struct_std_array_int__9__struct_std_array_int__9__0_1_2_3_4_5_6_7_8(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_56(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_unsigned_int__1__2__3__4__5__6__7__8__9__struct_std_array_int__9__struct_std_array_int__0_(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_unsigned_int__18446744073709551615__2__3__18446744073709551615__5__18446744073709551615__7__18446744073709551615__9__struct_std_array_int__9__struct_std_array_int__9__0_1_2_3_4_5_6_7_8(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_57(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_unsigned_int__18446744073709551615__2__3__18446744073709551615__5__18446744073709551615__7__18446744073709551615__9__struct_std_array_int__9__struct_std_array_int__4__0_1_2_3(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_unsigned_int__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_int__9__struct_std_array_int__9__0_1_2_3_4_5_6_7_8(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_58(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_signed_char__struct_std_array_unsigned_long__0__struct_std_array_unsigned_long__0_(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_59(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_signed_char__3__struct_std_array_unsigned_long__1__struct_std_array_unsigned_long__1__0(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_60(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_signed_char__3__struct_std_array_unsigned_long__1__struct_std_array_unsigned_long__0_(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_signed_char__18446744073709551615__struct_std_array_unsigned_long__1__struct_std_array_unsigned_long__1__0(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_61(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_signed_char__3__7__struct_std_array_unsigned_long__2__struct_std_array_unsigned_long__2__0_1(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_62(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_signed_char__3__7__struct_std_array_unsigned_long__2__struct_std_array_unsigned_long__0_(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_signed_char__3__18446744073709551615__struct_std_array_unsigned_long__2__struct_std_array_unsigned_long__2__0_1(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_63(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_signed_char__3__18446744073709551615__struct_std_array_unsigned_long__2__struct_std_array_unsigned_long__1__0(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_signed_char__18446744073709551615__7__struct_std_array_unsigned_long__2__struct_std_array_unsigned_long__2__0_1(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_64(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_signed_char__18446744073709551615__7__struct_std_array_unsigned_long__2__struct_std_array_unsigned_long__1__0(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_signed_char__18446744073709551615__18446744073709551615__struct_std_array_unsigned_long__2__struct_std_array_unsigned_long__2__0_1(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_65(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_signed_char__3__7__9__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__3__0_1_2(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_66(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_signed_char__3__7__9__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__0_(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_signed_char__3__7__18446744073709551615__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__3__0_1_2(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_67(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_signed_char__3__7__18446744073709551615__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__1__0(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_signed_char__3__18446744073709551615__18446744073709551615__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__3__0_1_2(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_68(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_signed_char__3__18446744073709551615__18446744073709551615__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__2__0_1(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_signed_char__18446744073709551615__7__18446744073709551615__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__3__0_1_2(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_69(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_signed_char__18446744073709551615__7__18446744073709551615__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__2__0_1(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_signed_char__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__3__0_1_2(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_70(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_signed_char__3__18446744073709551615__9__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__3__0_1_2(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_71(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_signed_char__3__18446744073709551615__9__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__1__0(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_signed_char__18446744073709551615__18446744073709551615__9__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__3__0_1_2(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_72(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_signed_char__18446744073709551615__18446744073709551615__9__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__2__0_1(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_signed_char__18446744073709551615__7__9__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__3__0_1_2(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_73(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_signed_char__18446744073709551615__7__9__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__1__0(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_signed_char__1__2__3__4__5__6__7__8__9__struct_std_array_unsigned_long__9__struct_std_array_unsigned_long__9__0_1_2_3_4_5_6_7_8(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_74(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_signed_char__1__2__3__4__5__6__7__8__9__struct_std_array_unsigned_long__9__struct_std_array_unsigned_long__0_(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_signed_char__18446744073709551615__2__3__18446744073709551615__5__18446744073709551615__7__18446744073709551615__9__struct_std_array_unsigned_long__9__struct_std_array_unsigned_long__9__0_1_2_3_4_5_6_7_8(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_75(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_signed_char__18446744073709551615__2__3__18446744073709551615__5__18446744073709551615__7__18446744073709551615__9__struct_std_array_unsigned_long__9__struct_std_array_unsigned_long__4__0_1_2_3(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_signed_char__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_unsigned_long__9__struct_std_array_unsigned_long__9__0_1_2_3_4_5_6_7_8(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_76(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_long_long__struct_std_array_unsigned_int__0__struct_std_array_unsigned_int__0_(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_77(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_long_long__3__struct_std_array_unsigned_int__1__struct_std_array_unsigned_int__1__0(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_78(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_long_long__3__struct_std_array_unsigned_int__1__struct_std_array_unsigned_int__0_(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_long_long__18446744073709551615__struct_std_array_unsigned_int__1__struct_std_array_unsigned_int__1__0(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_79(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_long_long__3__7__struct_std_array_unsigned_int__2__struct_std_array_unsigned_int__2__0_1(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_80(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_long_long__3__7__struct_std_array_unsigned_int__2__struct_std_array_unsigned_int__0_(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_long_long__3__18446744073709551615__struct_std_array_unsigned_int__2__struct_std_array_unsigned_int__2__0_1(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_81(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_long_long__3__18446744073709551615__struct_std_array_unsigned_int__2__struct_std_array_unsigned_int__1__0(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_long_long__18446744073709551615__7__struct_std_array_unsigned_int__2__struct_std_array_unsigned_int__2__0_1(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_82(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_long_long__18446744073709551615__7__struct_std_array_unsigned_int__2__struct_std_array_unsigned_int__1__0(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_long_long__18446744073709551615__18446744073709551615__struct_std_array_unsigned_int__2__struct_std_array_unsigned_int__2__0_1(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_83(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_long_long__3__7__9__struct_std_array_unsigned_int__3__struct_std_array_unsigned_int__3__0_1_2(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_84(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_long_long__3__7__9__struct_std_array_unsigned_int__3__struct_std_array_unsigned_int__0_(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_long_long__3__7__18446744073709551615__struct_std_array_unsigned_int__3__struct_std_array_unsigned_int__3__0_1_2(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_85(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_long_long__3__7__18446744073709551615__struct_std_array_unsigned_int__3__struct_std_array_unsigned_int__1__0(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_long_long__3__18446744073709551615__18446744073709551615__struct_std_array_unsigned_int__3__struct_std_array_unsigned_int__3__0_1_2(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_86(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_long_long__3__18446744073709551615__18446744073709551615__struct_std_array_unsigned_int__3__struct_std_array_unsigned_int__2__0_1(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_long_long__18446744073709551615__7__18446744073709551615__struct_std_array_unsigned_int__3__struct_std_array_unsigned_int__3__0_1_2(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_87(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_long_long__18446744073709551615__7__18446744073709551615__struct_std_array_unsigned_int__3__struct_std_array_unsigned_int__2__0_1(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_long_long__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_unsigned_int__3__struct_std_array_unsigned_int__3__0_1_2(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_88(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_long_long__3__18446744073709551615__9__struct_std_array_unsigned_int__3__struct_std_array_unsigned_int__3__0_1_2(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_89(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_long_long__3__18446744073709551615__9__struct_std_array_unsigned_int__3__struct_std_array_unsigned_int__1__0(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_long_long__18446744073709551615__18446744073709551615__9__struct_std_array_unsigned_int__3__struct_std_array_unsigned_int__3__0_1_2(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_90(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_long_long__18446744073709551615__18446744073709551615__9__struct_std_array_unsigned_int__3__struct_std_array_unsigned_int__2__0_1(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_long_long__18446744073709551615__7__9__struct_std_array_unsigned_int__3__struct_std_array_unsigned_int__3__0_1_2(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_91(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_long_long__18446744073709551615__7__9__struct_std_array_unsigned_int__3__struct_std_array_unsigned_int__1__0(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_long_long__1__2__3__4__5__6__7__8__9__struct_std_array_unsigned_int__9__struct_std_array_unsigned_int__9__0_1_2_3_4_5_6_7_8(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_92(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_long_long__1__2__3__4__5__6__7__8__9__struct_std_array_unsigned_int__9__struct_std_array_unsigned_int__0_(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_long_long__18446744073709551615__2__3__18446744073709551615__5__18446744073709551615__7__18446744073709551615__9__struct_std_array_unsigned_int__9__struct_std_array_unsigned_int__9__0_1_2_3_4_5_6_7_8(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_93(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_long_long__18446744073709551615__2__3__18446744073709551615__5__18446744073709551615__7__18446744073709551615__9__struct_std_array_unsigned_int__9__struct_std_array_unsigned_int__4__0_1_2_3(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_long_long__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_unsigned_int__9__struct_std_array_unsigned_int__9__0_1_2_3_4_5_6_7_8(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_94(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_unsigned_long__struct_std_array_int__0__struct_std_array_int__0_(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_95(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_unsigned_long__3__struct_std_array_int__1__struct_std_array_int__1__0(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_96(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_unsigned_long__3__struct_std_array_int__1__struct_std_array_int__0_(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_unsigned_long__18446744073709551615__struct_std_array_int__1__struct_std_array_int__1__0(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_97(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_unsigned_long__3__7__struct_std_array_int__2__struct_std_array_int__2__0_1(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_98(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_unsigned_long__3__7__struct_std_array_int__2__struct_std_array_int__0_(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_unsigned_long__3__18446744073709551615__struct_std_array_int__2__struct_std_array_int__2__0_1(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_99(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_unsigned_long__3__18446744073709551615__struct_std_array_int__2__struct_std_array_int__1__0(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_unsigned_long__18446744073709551615__7__struct_std_array_int__2__struct_std_array_int__2__0_1(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_100(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_unsigned_long__18446744073709551615__7__struct_std_array_int__2__struct_std_array_int__1__0(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_unsigned_long__18446744073709551615__18446744073709551615__struct_std_array_int__2__struct_std_array_int__2__0_1(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_101(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_unsigned_long__3__7__9__struct_std_array_int__3__struct_std_array_int__3__0_1_2(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_102(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_unsigned_long__3__7__9__struct_std_array_int__3__struct_std_array_int__0_(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_unsigned_long__3__7__18446744073709551615__struct_std_array_int__3__struct_std_array_int__3__0_1_2(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_103(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_unsigned_long__3__7__18446744073709551615__struct_std_array_int__3__struct_std_array_int__1__0(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_unsigned_long__3__18446744073709551615__18446744073709551615__struct_std_array_int__3__struct_std_array_int__3__0_1_2(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_104(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_unsigned_long__3__18446744073709551615__18446744073709551615__struct_std_array_int__3__struct_std_array_int__2__0_1(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_unsigned_long__18446744073709551615__7__18446744073709551615__struct_std_array_int__3__struct_std_array_int__3__0_1_2(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_105(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_unsigned_long__18446744073709551615__7__18446744073709551615__struct_std_array_int__3__struct_std_array_int__2__0_1(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_unsigned_long__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_int__3__struct_std_array_int__3__0_1_2(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_106(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_unsigned_long__3__18446744073709551615__9__struct_std_array_int__3__struct_std_array_int__3__0_1_2(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_107(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_unsigned_long__3__18446744073709551615__9__struct_std_array_int__3__struct_std_array_int__1__0(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_unsigned_long__18446744073709551615__18446744073709551615__9__struct_std_array_int__3__struct_std_array_int__3__0_1_2(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_108(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_unsigned_long__18446744073709551615__18446744073709551615__9__struct_std_array_int__3__struct_std_array_int__2__0_1(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_unsigned_long__18446744073709551615__7__9__struct_std_array_int__3__struct_std_array_int__3__0_1_2(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_109(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_unsigned_long__18446744073709551615__7__9__struct_std_array_int__3__struct_std_array_int__1__0(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_unsigned_long__1__2__3__4__5__6__7__8__9__struct_std_array_int__9__struct_std_array_int__9__0_1_2_3_4_5_6_7_8(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_110(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_unsigned_long__1__2__3__4__5__6__7__8__9__struct_std_array_int__9__struct_std_array_int__0_(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_unsigned_long__18446744073709551615__2__3__18446744073709551615__5__18446744073709551615__7__18446744073709551615__9__struct_std_array_int__9__struct_std_array_int__9__0_1_2_3_4_5_6_7_8(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_111(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_unsigned_long__18446744073709551615__2__3__18446744073709551615__5__18446744073709551615__7__18446744073709551615__9__struct_std_array_int__9__struct_std_array_int__4__0_1_2_3(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_unsigned_long__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_int__9__struct_std_array_int__9__0_1_2_3_4_5_6_7_8(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_112(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_unsigned_long__struct_std_array_unsigned_long__0__struct_std_array_unsigned_long__0_(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_113(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_unsigned_long__3__struct_std_array_unsigned_long__1__struct_std_array_unsigned_long__1__0(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_114(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_unsigned_long__3__struct_std_array_unsigned_long__1__struct_std_array_unsigned_long__0_(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_unsigned_long__18446744073709551615__struct_std_array_unsigned_long__1__struct_std_array_unsigned_long__1__0(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_115(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_unsigned_long__3__7__struct_std_array_unsigned_long__2__struct_std_array_unsigned_long__2__0_1(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_116(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_unsigned_long__3__7__struct_std_array_unsigned_long__2__struct_std_array_unsigned_long__0_(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_unsigned_long__3__18446744073709551615__struct_std_array_unsigned_long__2__struct_std_array_unsigned_long__2__0_1(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_117(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_unsigned_long__3__18446744073709551615__struct_std_array_unsigned_long__2__struct_std_array_unsigned_long__1__0(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_unsigned_long__18446744073709551615__7__struct_std_array_unsigned_long__2__struct_std_array_unsigned_long__2__0_1(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_118(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_unsigned_long__18446744073709551615__7__struct_std_array_unsigned_long__2__struct_std_array_unsigned_long__1__0(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_unsigned_long__18446744073709551615__18446744073709551615__struct_std_array_unsigned_long__2__struct_std_array_unsigned_long__2__0_1(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_119(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_unsigned_long__3__7__9__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__3__0_1_2(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_120(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_unsigned_long__3__7__9__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__0_(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_unsigned_long__3__7__18446744073709551615__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__3__0_1_2(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_121(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_unsigned_long__3__7__18446744073709551615__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__1__0(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_unsigned_long__3__18446744073709551615__18446744073709551615__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__3__0_1_2(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_122(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_unsigned_long__3__18446744073709551615__18446744073709551615__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__2__0_1(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_unsigned_long__18446744073709551615__7__18446744073709551615__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__3__0_1_2(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_123(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_unsigned_long__18446744073709551615__7__18446744073709551615__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__2__0_1(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_unsigned_long__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__3__0_1_2(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_124(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_unsigned_long__3__18446744073709551615__9__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__3__0_1_2(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_125(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_unsigned_long__3__18446744073709551615__9__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__1__0(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_unsigned_long__18446744073709551615__18446744073709551615__9__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__3__0_1_2(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_126(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_unsigned_long__18446744073709551615__18446744073709551615__9__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__2__0_1(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_unsigned_long__18446744073709551615__7__9__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__3__0_1_2(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_127(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_unsigned_long__18446744073709551615__7__9__struct_std_array_unsigned_long__3__struct_std_array_unsigned_long__1__0(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_unsigned_long__1__2__3__4__5__6__7__8__9__struct_std_array_unsigned_long__9__struct_std_array_unsigned_long__9__0_1_2_3_4_5_6_7_8(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_128(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_unsigned_long__1__2__3__4__5__6__7__8__9__struct_std_array_unsigned_long__9__struct_std_array_unsigned_long__0_(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_unsigned_long__18446744073709551615__2__3__18446744073709551615__5__18446744073709551615__7__18446744073709551615__9__struct_std_array_unsigned_long__9__struct_std_array_unsigned_long__9__0_1_2_3_4_5_6_7_8(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_129(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_unsigned_long__18446744073709551615__2__3__18446744073709551615__5__18446744073709551615__7__18446744073709551615__9__struct_std_array_unsigned_long__9__struct_std_array_unsigned_long__4__0_1_2_3(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_unsigned_long__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_unsigned_long__9__struct_std_array_unsigned_long__9__0_1_2_3_4_5_6_7_8(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_130(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_int__struct_std_array_struct_IntType__0__struct_std_array_struct_IntType__0_(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_131(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_int__3__struct_std_array_struct_IntType__1__struct_std_array_struct_IntType__1__0(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_132(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_int__3__struct_std_array_struct_IntType__1__struct_std_array_struct_IntType__0_(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_int__18446744073709551615__struct_std_array_struct_IntType__1__struct_std_array_struct_IntType__1__0(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_133(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_int__3__7__struct_std_array_struct_IntType__2__struct_std_array_struct_IntType__2__0_1(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_134(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_int__3__7__struct_std_array_struct_IntType__2__struct_std_array_struct_IntType__0_(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_int__3__18446744073709551615__struct_std_array_struct_IntType__2__struct_std_array_struct_IntType__2__0_1(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_135(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_int__3__18446744073709551615__struct_std_array_struct_IntType__2__struct_std_array_struct_IntType__1__0(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_int__18446744073709551615__7__struct_std_array_struct_IntType__2__struct_std_array_struct_IntType__2__0_1(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_136(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_int__18446744073709551615__7__struct_std_array_struct_IntType__2__struct_std_array_struct_IntType__1__0(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_int__18446744073709551615__18446744073709551615__struct_std_array_struct_IntType__2__struct_std_array_struct_IntType__2__0_1(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_137(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_int__3__7__9__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__3__0_1_2(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_138(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_int__3__7__9__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__0_(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_int__3__7__18446744073709551615__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__3__0_1_2(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_139(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_int__3__7__18446744073709551615__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__1__0(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_int__3__18446744073709551615__18446744073709551615__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__3__0_1_2(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_140(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_int__3__18446744073709551615__18446744073709551615__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__2__0_1(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_int__18446744073709551615__7__18446744073709551615__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__3__0_1_2(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_141(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_int__18446744073709551615__7__18446744073709551615__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__2__0_1(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_int__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__3__0_1_2(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_142(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_int__3__18446744073709551615__9__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__3__0_1_2(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_143(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_int__3__18446744073709551615__9__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__1__0(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_int__18446744073709551615__18446744073709551615__9__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__3__0_1_2(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_144(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_int__18446744073709551615__18446744073709551615__9__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__2__0_1(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_int__18446744073709551615__7__9__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__3__0_1_2(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_145(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_int__18446744073709551615__7__9__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__1__0(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_int__1__2__3__4__5__6__7__8__9__struct_std_array_struct_IntType__9__struct_std_array_struct_IntType__9__0_1_2_3_4_5_6_7_8(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_146(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_int__1__2__3__4__5__6__7__8__9__struct_std_array_struct_IntType__9__struct_std_array_struct_IntType__0_(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_int__18446744073709551615__2__3__18446744073709551615__5__18446744073709551615__7__18446744073709551615__9__struct_std_array_struct_IntType__9__struct_std_array_struct_IntType__9__0_1_2_3_4_5_6_7_8(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_147(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_int__18446744073709551615__2__3__18446744073709551615__5__18446744073709551615__7__18446744073709551615__9__struct_std_array_struct_IntType__9__struct_std_array_struct_IntType__4__0_1_2_3(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_int__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_struct_IntType__9__struct_std_array_struct_IntType__9__0_1_2_3_4_5_6_7_8(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_148(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_signed_char__struct_std_array_struct_IntType__0__struct_std_array_struct_IntType__0_(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_149(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_signed_char__3__struct_std_array_struct_IntType__1__struct_std_array_struct_IntType__1__0(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_150(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_signed_char__3__struct_std_array_struct_IntType__1__struct_std_array_struct_IntType__0_(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_signed_char__18446744073709551615__struct_std_array_struct_IntType__1__struct_std_array_struct_IntType__1__0(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_151(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_signed_char__3__7__struct_std_array_struct_IntType__2__struct_std_array_struct_IntType__2__0_1(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_152(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_signed_char__3__7__struct_std_array_struct_IntType__2__struct_std_array_struct_IntType__0_(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_signed_char__3__18446744073709551615__struct_std_array_struct_IntType__2__struct_std_array_struct_IntType__2__0_1(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_153(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_signed_char__3__18446744073709551615__struct_std_array_struct_IntType__2__struct_std_array_struct_IntType__1__0(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_signed_char__18446744073709551615__7__struct_std_array_struct_IntType__2__struct_std_array_struct_IntType__2__0_1(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_154(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_signed_char__18446744073709551615__7__struct_std_array_struct_IntType__2__struct_std_array_struct_IntType__1__0(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_signed_char__18446744073709551615__18446744073709551615__struct_std_array_struct_IntType__2__struct_std_array_struct_IntType__2__0_1(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_155(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_signed_char__3__7__9__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__3__0_1_2(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_156(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_signed_char__3__7__9__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__0_(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_signed_char__3__7__18446744073709551615__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__3__0_1_2(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_157(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_signed_char__3__7__18446744073709551615__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__1__0(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_signed_char__3__18446744073709551615__18446744073709551615__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__3__0_1_2(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_158(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_signed_char__3__18446744073709551615__18446744073709551615__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__2__0_1(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_signed_char__18446744073709551615__7__18446744073709551615__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__3__0_1_2(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_159(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_signed_char__18446744073709551615__7__18446744073709551615__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__2__0_1(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_signed_char__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__3__0_1_2(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_160(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_signed_char__3__18446744073709551615__9__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__3__0_1_2(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_161(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_signed_char__3__18446744073709551615__9__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__1__0(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_signed_char__18446744073709551615__18446744073709551615__9__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__3__0_1_2(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_162(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_signed_char__18446744073709551615__18446744073709551615__9__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__2__0_1(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_signed_char__18446744073709551615__7__9__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__3__0_1_2(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_163(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_signed_char__18446744073709551615__7__9__struct_std_array_struct_IntType__3__struct_std_array_struct_IntType__1__0(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_signed_char__1__2__3__4__5__6__7__8__9__struct_std_array_struct_IntType__9__struct_std_array_struct_IntType__9__0_1_2_3_4_5_6_7_8(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_164(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_signed_char__1__2__3__4__5__6__7__8__9__struct_std_array_struct_IntType__9__struct_std_array_struct_IntType__0_(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_signed_char__18446744073709551615__2__3__18446744073709551615__5__18446744073709551615__7__18446744073709551615__9__struct_std_array_struct_IntType__9__struct_std_array_struct_IntType__9__0_1_2_3_4_5_6_7_8(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_165(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
    pub fn test_construction_1_class_std_extents_signed_char__18446744073709551615__2__3__18446744073709551615__5__18446744073709551615__7__18446744073709551615__9__struct_std_array_struct_IntType__9__struct_std_array_struct_IntType__4__0_1_2_3(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {}
    }
    pub fn test_construction_1_class_std_extents_signed_char__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__struct_std_array_struct_IntType__9__struct_std_array_struct_IntType__9__0_1_2_3_4_5_6_7_8(
        all_ext: (),
        _a1: (),
        _a2: (),
    ) {
        let all_ext: Value<()> = Rc::new(RefCell::new(all_ext));
        let _a1: Value<()> = Rc::new(RefCell::new(_a1));
        let _a2: Value<()> = Rc::new(RefCell::new(_a2));
        {
            const _: () = assert!(true, "");;
            let expected_exts: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ test_runtime_observers_166(unimplemented!(), (*expected_exts.borrow()).clone()) });
        }
    }
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
    ({ test_index_type_combo_339() });
    const _: () = assert!(true, "test_index_type_combo<DefaultCtorTest>()");;
    return 0_();
}
pub trait IntTypeImpl {
    fn operator_eq(&self, rhs: Ptr<IntType>) -> ();
    fn to__4(&self) -> ();
    fn to__5(&self) -> ();
    fn to__6(&self) -> ();
}
impl IntTypeImpl for Ptr<IntType> {
    fn operator_eq(&self, rhs: Ptr<IntType>) -> () {
        return ({ (*self).with(|__s| __s.val) } == { rhs.with(|__s| __s.val) });
    }
    fn to__4(&self) -> () {
        return (*self).with(|__s| __s.val);
    }
    fn to__5(&self) -> () {
        return (*self).with(|__s| __s.val);
    }
    fn to__6(&self) -> () {
        return (*self).with(|__s| __s.val);
    }
}
pub trait IntTypeNCImpl {
    fn operator_eq(&self, rhs: Ptr<IntType>) -> ();
    fn to__4(&self) -> ();
    fn to__5(&self) -> ();
    fn to__6(&self) -> ();
}
impl IntTypeNCImpl for Ptr<IntTypeNC> {
    fn operator_eq(&self, rhs: Ptr<IntType>) -> () {
        return ({ (*self).with(|__s| __s.val) } == { rhs.with(|__s| __s.val) });
    }
    fn to__4(&self) -> () {
        return (*self).with(|__s| __s.val);
    }
    fn to__5(&self) -> () {
        return (*self).with(|__s| __s.val);
    }
    fn to__6(&self) -> () {
        return (*self).with(|__s| __s.val);
    }
}
pub fn __cpp2rust_init_globals() {}
