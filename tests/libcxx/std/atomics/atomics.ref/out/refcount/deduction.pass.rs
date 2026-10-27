extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type anon_0 = ();
pub const anon_0_value: anon_0 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_atomic_ref_char___std_atomic_ref_char__ {}
pub type anon_1 = ();
pub const anon_1_value: anon_1 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_atomic_ref_signed_char___std_atomic_ref_signed_char__ {}
pub type anon_2 = ();
pub const anon_2_value: anon_2 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_atomic_ref_unsigned_char___std_atomic_ref_unsigned_char__
{}
pub type anon_3 = ();
pub const anon_3_value: anon_3 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_atomic_ref_short___std_atomic_ref_short__ {}
pub type anon_4 = ();
pub const anon_4_value: anon_4 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_atomic_ref_unsigned_short___std_atomic_ref_unsigned_short__
{}
pub type anon_5 = ();
pub const anon_5_value: anon_5 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_atomic_ref_int___std_atomic_ref_int__ {}
pub type anon_6 = ();
pub const anon_6_value: anon_6 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_atomic_ref_unsigned_int___std_atomic_ref_unsigned_int__ {
}
pub type anon_7 = ();
pub const anon_7_value: anon_7 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_atomic_ref_long___std_atomic_ref_long__ {}
pub type anon_8 = ();
pub const anon_8_value: anon_8 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_atomic_ref_unsigned_long___std_atomic_ref_unsigned_long__
{}
pub type anon_9 = ();
pub const anon_9_value: anon_9 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_atomic_ref_long_long___std_atomic_ref_long_long__ {}
pub type anon_10 = ();
pub const anon_10_value: anon_10 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_atomic_ref_unsigned_long_long___std_atomic_ref_unsigned_long_long__
{}
pub type anon_11 = ();
pub const anon_11_value: anon_11 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_atomic_ref_wchar_t___std_atomic_ref_wchar_t__ {}
pub type anon_12 = ();
pub const anon_12_value: anon_12 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_atomic_ref_char8_t___std_atomic_ref_char8_t__ {}
pub type anon_13 = ();
pub const anon_13_value: anon_13 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_atomic_ref_char16_t___std_atomic_ref_char16_t__ {}
pub type anon_14 = ();
pub const anon_14_value: anon_14 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_atomic_ref_char32_t___std_atomic_ref_char32_t__ {}
pub type anon_15 = ();
pub const anon_15_value: anon_15 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_atomic_ref_int_ptr___std_atomic_ref_int_ptr__ {}
pub type anon_16 = ();
pub const anon_16_value: anon_16 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_atomic_ref_const_int_ptr___std_atomic_ref_const_int_ptr__
{}
pub type anon_17 = ();
pub const anon_17_value: anon_17 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_atomic_ref_UserAtomicType___std_atomic_ref_UserAtomicType__
{}
pub type anon_18 = ();
pub const anon_18_value: anon_18 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_atomic_ref_LargeUserAtomicType___std_atomic_ref_LargeUserAtomicType__
{}
pub type anon_19 = ();
pub const anon_19_value: anon_19 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_atomic_ref_float___std_atomic_ref_float__ {}
pub type anon_20 = ();
pub const anon_20_value: anon_20 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_atomic_ref_double___std_atomic_ref_double__ {}
pub type LockFreeStatus = ();
pub const LockFreeStatus_unknown: LockFreeStatus = -1;
pub const LockFreeStatus_never: LockFreeStatus = 0;
pub const LockFreeStatus_sometimes: LockFreeStatus = 1;
pub const LockFreeStatus_always: LockFreeStatus = 2;
thread_local!(
    pub static value_21: Value<LockFreeStatus> = Rc::new(RefCell::new(
        ((if ((::std::mem::size_of::<()>() == ::std::mem::size_of::<()>())
            && (::std::mem::align_of::<()>() >= ::std::mem::size_of::<()>()))
        {
            2_()
        } else {
            (if ((::std::mem::size_of::<()>() == ::std::mem::size_of::<()>())
                && (::std::mem::align_of::<()>() >= ::std::mem::size_of::<()>()))
            {
                2_()
            } else {
                (if ((::std::mem::size_of::<()>() == ::std::mem::size_of::<()>())
                    && (::std::mem::align_of::<()>() >= ::std::mem::size_of::<()>()))
                {
                    2_()
                } else {
                    (if ((::std::mem::size_of::<()>() == ::std::mem::size_of::<()>())
                        && (::std::mem::align_of::<()>() >= ::std::mem::size_of::<()>()))
                    {
                        2_()
                    } else {
                        (if ((::std::mem::size_of::<()>() == ::std::mem::size_of::<()>())
                            && (::std::mem::align_of::<()>() >= ::std::mem::size_of::<()>()))
                        {
                            2_()
                        } else {
                            (if ((::std::mem::size_of::<()>() == 8usize)
                                && (::std::mem::align_of::<()>() >= ::std::mem::size_of::<()>()))
                            {
                                2_()
                            } else {
                                -1_()
                            })
                        })
                    })
                })
            })
        }) as LockFreeStatus),
    ));
);
thread_local!(
    pub static status_known_22: Value<()> = Rc::new(RefCell::new(
        (value_21.with(|rc| *rc.borrow()) != LockFreeStatus_unknown),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct LockFreeStatusInfo_char_ {}
thread_local!(
    pub static value_23: Value<LockFreeStatus> = Rc::new(RefCell::new(
        ((if ((::std::mem::size_of::<()>() == ::std::mem::size_of::<()>())
            && (::std::mem::align_of::<()>() >= ::std::mem::size_of::<()>()))
        {
            2_()
        } else {
            (if ((::std::mem::size_of::<()>() == ::std::mem::size_of::<()>())
                && (::std::mem::align_of::<()>() >= ::std::mem::size_of::<()>()))
            {
                2_()
            } else {
                (if ((::std::mem::size_of::<()>() == ::std::mem::size_of::<()>())
                    && (::std::mem::align_of::<()>() >= ::std::mem::size_of::<()>()))
                {
                    2_()
                } else {
                    (if ((::std::mem::size_of::<()>() == ::std::mem::size_of::<()>())
                        && (::std::mem::align_of::<()>() >= ::std::mem::size_of::<()>()))
                    {
                        2_()
                    } else {
                        (if ((::std::mem::size_of::<()>() == ::std::mem::size_of::<()>())
                            && (::std::mem::align_of::<()>() >= ::std::mem::size_of::<()>()))
                        {
                            2_()
                        } else {
                            (if ((::std::mem::size_of::<()>() == 8usize)
                                && (::std::mem::align_of::<()>() >= ::std::mem::size_of::<()>()))
                            {
                                2_()
                            } else {
                                -1_()
                            })
                        })
                    })
                })
            })
        }) as LockFreeStatus),
    ));
);
thread_local!(
    pub static status_known_24: Value<()> = Rc::new(RefCell::new(
        (value_23.with(|rc| *rc.borrow()) != LockFreeStatus_unknown),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct LockFreeStatusInfo_short_ {}
thread_local!(
    pub static value_25: Value<LockFreeStatus> = Rc::new(RefCell::new(
        ((if ((::std::mem::size_of::<()>() == ::std::mem::size_of::<()>())
            && (::std::mem::align_of::<()>() >= ::std::mem::size_of::<()>()))
        {
            2_()
        } else {
            (if ((::std::mem::size_of::<()>() == ::std::mem::size_of::<()>())
                && (::std::mem::align_of::<()>() >= ::std::mem::size_of::<()>()))
            {
                2_()
            } else {
                (if ((::std::mem::size_of::<()>() == ::std::mem::size_of::<()>())
                    && (::std::mem::align_of::<()>() >= ::std::mem::size_of::<()>()))
                {
                    2_()
                } else {
                    (if ((::std::mem::size_of::<()>() == ::std::mem::size_of::<()>())
                        && (::std::mem::align_of::<()>() >= ::std::mem::size_of::<()>()))
                    {
                        2_()
                    } else {
                        (if ((::std::mem::size_of::<()>() == ::std::mem::size_of::<()>())
                            && (::std::mem::align_of::<()>() >= ::std::mem::size_of::<()>()))
                        {
                            2_()
                        } else {
                            (if ((::std::mem::size_of::<()>() == 8usize)
                                && (::std::mem::align_of::<()>() >= ::std::mem::size_of::<()>()))
                            {
                                2_()
                            } else {
                                -1_()
                            })
                        })
                    })
                })
            })
        }) as LockFreeStatus),
    ));
);
thread_local!(
    pub static status_known_26: Value<()> = Rc::new(RefCell::new(
        (value_25.with(|rc| *rc.borrow()) != LockFreeStatus_unknown),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct LockFreeStatusInfo_int_ {}
thread_local!(
    pub static value_27: Value<LockFreeStatus> = Rc::new(RefCell::new(
        ((if ((::std::mem::size_of::<()>() == ::std::mem::size_of::<()>())
            && (::std::mem::align_of::<()>() >= ::std::mem::size_of::<()>()))
        {
            2_()
        } else {
            (if ((::std::mem::size_of::<()>() == ::std::mem::size_of::<()>())
                && (::std::mem::align_of::<()>() >= ::std::mem::size_of::<()>()))
            {
                2_()
            } else {
                (if ((::std::mem::size_of::<()>() == ::std::mem::size_of::<()>())
                    && (::std::mem::align_of::<()>() >= ::std::mem::size_of::<()>()))
                {
                    2_()
                } else {
                    (if ((::std::mem::size_of::<()>() == ::std::mem::size_of::<()>())
                        && (::std::mem::align_of::<()>() >= ::std::mem::size_of::<()>()))
                    {
                        2_()
                    } else {
                        (if ((::std::mem::size_of::<()>() == ::std::mem::size_of::<()>())
                            && (::std::mem::align_of::<()>() >= ::std::mem::size_of::<()>()))
                        {
                            2_()
                        } else {
                            (if ((::std::mem::size_of::<()>() == 8usize)
                                && (::std::mem::align_of::<()>() >= ::std::mem::size_of::<()>()))
                            {
                                2_()
                            } else {
                                -1_()
                            })
                        })
                    })
                })
            })
        }) as LockFreeStatus),
    ));
);
thread_local!(
    pub static status_known_28: Value<()> = Rc::new(RefCell::new(
        (value_27.with(|rc| *rc.borrow()) != LockFreeStatus_unknown),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct LockFreeStatusInfo_long_ {}
thread_local!(
    pub static value_29: Value<LockFreeStatus> = Rc::new(RefCell::new(
        ((if ((8usize == ::std::mem::size_of::<()>()) && (8usize >= 8usize)) {
            2_()
        } else {
            (if ((8usize == ::std::mem::size_of::<()>()) && (8usize >= 8usize)) {
                2_()
            } else {
                (if ((8usize == ::std::mem::size_of::<()>()) && (8usize >= 8usize)) {
                    2_()
                } else {
                    (if ((8usize == ::std::mem::size_of::<()>()) && (8usize >= 8usize)) {
                        2_()
                    } else {
                        (if ((8usize == ::std::mem::size_of::<()>()) && (8usize >= 8usize)) {
                            2_()
                        } else {
                            (if ((8usize == 8usize) && (8usize >= 8usize)) {
                                2_()
                            } else {
                                -1_()
                            })
                        })
                    })
                })
            })
        }) as LockFreeStatus),
    ));
);
thread_local!(
    pub static status_known_30: Value<()> = Rc::new(RefCell::new(
        (value_29.with(|rc| *rc.borrow()) != LockFreeStatus_unknown),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct LockFreeStatusInfo_void_ptr_ {}
thread_local!(
    pub static value_31: Value<LockFreeStatus> = Rc::new(RefCell::new(
        ((if ((::std::mem::size_of::<()>() == ::std::mem::size_of::<()>())
            && (::std::mem::align_of::<()>() >= ::std::mem::size_of::<()>()))
        {
            2_()
        } else {
            (if ((::std::mem::size_of::<()>() == ::std::mem::size_of::<()>())
                && (::std::mem::align_of::<()>() >= ::std::mem::size_of::<()>()))
            {
                2_()
            } else {
                (if ((::std::mem::size_of::<()>() == ::std::mem::size_of::<()>())
                    && (::std::mem::align_of::<()>() >= ::std::mem::size_of::<()>()))
                {
                    2_()
                } else {
                    (if ((::std::mem::size_of::<()>() == ::std::mem::size_of::<()>())
                        && (::std::mem::align_of::<()>() >= ::std::mem::size_of::<()>()))
                    {
                        2_()
                    } else {
                        (if ((::std::mem::size_of::<()>() == ::std::mem::size_of::<()>())
                            && (::std::mem::align_of::<()>() >= ::std::mem::size_of::<()>()))
                        {
                            2_()
                        } else {
                            (if ((::std::mem::size_of::<()>() == 8usize)
                                && (::std::mem::align_of::<()>() >= ::std::mem::size_of::<()>()))
                            {
                                2_()
                            } else {
                                -1_()
                            })
                        })
                    })
                })
            })
        }) as LockFreeStatus),
    ));
);
thread_local!(
    pub static status_known_32: Value<()> = Rc::new(RefCell::new(
        (value_31.with(|rc| *rc.borrow()) != LockFreeStatus_unknown),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct LockFreeStatusInfo_long_long_ {}
const _: () = assert!(true, "LockFreeStatusInfo<char>::status_known");
const _: () = assert!(true, "LockFreeStatusInfo<short>::status_known");
const _: () = assert!(true, "LockFreeStatusInfo<int>::status_known");
const _: () = assert!(true, "LockFreeStatusInfo<long>::status_known");
const _: () = assert!(true, "LockFreeStatusInfo<void*>::status_known");
const _: () = assert!(
    true,
    "alignof(long long) == sizeof(long long) ? LockFreeStatusInfo<long long>::status_known : true"
);
const _: () = assert!(
    true,
    "LockFreeStatusInfo<char>::value == LockFreeStatus::always"
);
const _: () = assert!(
    true,
    "LockFreeStatusInfo<short>::value == LockFreeStatus::always"
);
const _: () = assert!(
    true,
    "LockFreeStatusInfo<int>::value == LockFreeStatus::always"
);
pub fn operator_eq_33(x: Ptr<UserAtomicType>, y: Ptr<UserAtomicType>) -> () {
    return ({ x.with(|__s| __s.i) } == { y.with(|__s| __s.i) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct UserAtomicType {
    #[offset(0)]
    pub i: (),
}
impl UserAtomicType {
    pub fn new(d: Option<()>) -> Self {
        let mut d: () = d.unwrap_or_else(|| 0_());
        Self { i: d }
    }
}
impl std::cmp::PartialEq for UserAtomicType {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_33(
                Rc::new(RefCell::new(UserAtomicType { i: self.i.clone() })).as_pointer(),
                Rc::new(RefCell::new(UserAtomicType { i: other.i.clone() })).as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for UserAtomicType {}
impl Default for UserAtomicType {
    fn default() -> Self {
        { UserAtomicType::new(None) }
    }
}
pub fn operator_eq_34(x: Ptr<LargeUserAtomicType>, y: Ptr<LargeUserAtomicType>) -> () {
    let mut i: () = 0_();
    'loop_: while (i < 128_()) {
        if ({ (array_elem!(x, a, i).read()) } != { (array_elem!(y, a, i).read()) }) {
            return false;
        }
        i.prefix_inc();
    }
    return true;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(512)]
pub struct LargeUserAtomicType {
    #[offset(0)]
    #[byte_size(512)]
    pub a: Box<[()]>,
}
impl LargeUserAtomicType {
    pub fn new(d: Option<()>) -> Self {
        let mut d: () = d.unwrap_or_else(|| 0_());
        let __this: Value<LargeUserAtomicType> = Rc::new(RefCell::new(Self {
            a: (0..128).map(|_| unimplemented!()).collect::<Box<[()]>>(),
        }));
        let this: Ptr<LargeUserAtomicType> = __this.as_pointer();
        'loop_: for e in IterRange::new(
            (array_field_ptr!(this, a) as Ptr<()>),
            (array_field_ptr!(this, a) as Ptr<()>).offset((128_()) as isize),
        ) {
            let e: Ptr<()> = (e).clone();
            let __rhs = d.postfix_inc();
            e.write(__rhs);
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl std::cmp::PartialEq for LargeUserAtomicType {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_34(
                Rc::new(RefCell::new(LargeUserAtomicType { a: self.a.clone() })).as_pointer(),
                Rc::new(RefCell::new(LargeUserAtomicType { a: other.a.clone() })).as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for LargeUserAtomicType {}
impl Default for LargeUserAtomicType {
    fn default() -> Self {
        { LargeUserAtomicType::new(None) }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestEachIntegralType_TestDeduction_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestEachPointerType_TestDeduction_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestEachAtomicType_TestDeduction_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestDeduction_UserAtomicType_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestDeduction_LargeUserAtomicType_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestDeduction_float_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestDeduction_double_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestDeduction_char_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestDeduction_signed_char_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestDeduction_unsigned_char_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestDeduction_short_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestDeduction_unsigned_short_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestDeduction_int_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestDeduction_unsigned_int_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestDeduction_long_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestDeduction_unsigned_long_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestDeduction_long_long_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestDeduction_unsigned_long_long_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestDeduction_wchar_t_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestDeduction_char8_t_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestDeduction_char16_t_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestDeduction_char32_t_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestDeduction_int_ptr_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestDeduction_const_int_ptr_ {}
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
    ({
        TestEachAtomicType_TestDeduction_Impl::operator_call(
            &Rc::new(RefCell::new(<TestEachAtomicType_TestDeduction_>::default())).as_pointer(),
        )
    });
    return 0_();
}
pub trait TestDeduction_LargeUserAtomicType_Impl {
    fn operator_call(&self);
}
impl TestDeduction_LargeUserAtomicType_Impl for Ptr<TestDeduction_LargeUserAtomicType_> {
    fn operator_call(&self) {
        let mut x: LargeUserAtomicType = LargeUserAtomicType::new({ Some(0_()) });
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
    }
}
pub trait TestDeduction_UserAtomicType_Impl {
    fn operator_call(&self);
}
impl TestDeduction_UserAtomicType_Impl for Ptr<TestDeduction_UserAtomicType_> {
    fn operator_call(&self) {
        let mut x: UserAtomicType = UserAtomicType::new({ Some(0_()) });
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
    }
}
pub trait TestDeduction_char16_t_Impl {
    fn operator_call(&self);
}
impl TestDeduction_char16_t_Impl for Ptr<TestDeduction_char16_t_> {
    fn operator_call(&self) {
        let mut x: () = 0_();
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
    }
}
pub trait TestDeduction_char32_t_Impl {
    fn operator_call(&self);
}
impl TestDeduction_char32_t_Impl for Ptr<TestDeduction_char32_t_> {
    fn operator_call(&self) {
        let mut x: () = 0_();
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
    }
}
pub trait TestDeduction_char8_t_Impl {
    fn operator_call(&self);
}
impl TestDeduction_char8_t_Impl for Ptr<TestDeduction_char8_t_> {
    fn operator_call(&self) {
        let mut x: () = 0_();
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
    }
}
pub trait TestDeduction_char_Impl {
    fn operator_call(&self);
}
impl TestDeduction_char_Impl for Ptr<TestDeduction_char_> {
    fn operator_call(&self) {
        let mut x: () = 0_();
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
    }
}
pub trait TestDeduction_const_int_ptr_Impl {
    fn operator_call(&self);
}
impl TestDeduction_const_int_ptr_Impl for Ptr<TestDeduction_const_int_ptr_> {
    fn operator_call(&self) {
        let mut x: Ptr<()> = Ptr::<()>::null();
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
    }
}
pub trait TestDeduction_double_Impl {
    fn operator_call(&self);
}
impl TestDeduction_double_Impl for Ptr<TestDeduction_double_> {
    fn operator_call(&self) {
        let mut x: () = 0_();
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
    }
}
pub trait TestDeduction_float_Impl {
    fn operator_call(&self);
}
impl TestDeduction_float_Impl for Ptr<TestDeduction_float_> {
    fn operator_call(&self) {
        let mut x: () = 0_();
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
    }
}
pub trait TestDeduction_int_Impl {
    fn operator_call(&self);
}
impl TestDeduction_int_Impl for Ptr<TestDeduction_int_> {
    fn operator_call(&self) {
        let mut x: () = 0_();
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
    }
}
pub trait TestDeduction_int_ptr_Impl {
    fn operator_call(&self);
}
impl TestDeduction_int_ptr_Impl for Ptr<TestDeduction_int_ptr_> {
    fn operator_call(&self) {
        let mut x: Ptr<()> = Ptr::<()>::null();
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
    }
}
pub trait TestDeduction_long_Impl {
    fn operator_call(&self);
}
impl TestDeduction_long_Impl for Ptr<TestDeduction_long_> {
    fn operator_call(&self) {
        let mut x: () = 0_();
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
    }
}
pub trait TestDeduction_long_long_Impl {
    fn operator_call(&self);
}
impl TestDeduction_long_long_Impl for Ptr<TestDeduction_long_long_> {
    fn operator_call(&self) {
        let mut x: () = 0_();
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
    }
}
pub trait TestDeduction_short_Impl {
    fn operator_call(&self);
}
impl TestDeduction_short_Impl for Ptr<TestDeduction_short_> {
    fn operator_call(&self) {
        let mut x: () = 0_();
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
    }
}
pub trait TestDeduction_signed_char_Impl {
    fn operator_call(&self);
}
impl TestDeduction_signed_char_Impl for Ptr<TestDeduction_signed_char_> {
    fn operator_call(&self) {
        let mut x: () = 0_();
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
    }
}
pub trait TestDeduction_unsigned_char_Impl {
    fn operator_call(&self);
}
impl TestDeduction_unsigned_char_Impl for Ptr<TestDeduction_unsigned_char_> {
    fn operator_call(&self) {
        let mut x: () = 0_();
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
    }
}
pub trait TestDeduction_unsigned_int_Impl {
    fn operator_call(&self);
}
impl TestDeduction_unsigned_int_Impl for Ptr<TestDeduction_unsigned_int_> {
    fn operator_call(&self) {
        let mut x: () = 0_();
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
    }
}
pub trait TestDeduction_unsigned_long_Impl {
    fn operator_call(&self);
}
impl TestDeduction_unsigned_long_Impl for Ptr<TestDeduction_unsigned_long_> {
    fn operator_call(&self) {
        let mut x: () = 0_();
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
    }
}
pub trait TestDeduction_unsigned_long_long_Impl {
    fn operator_call(&self);
}
impl TestDeduction_unsigned_long_long_Impl for Ptr<TestDeduction_unsigned_long_long_> {
    fn operator_call(&self) {
        let mut x: () = 0_();
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
    }
}
pub trait TestDeduction_unsigned_short_Impl {
    fn operator_call(&self);
}
impl TestDeduction_unsigned_short_Impl for Ptr<TestDeduction_unsigned_short_> {
    fn operator_call(&self) {
        let mut x: () = 0_();
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
    }
}
pub trait TestDeduction_wchar_t_Impl {
    fn operator_call(&self);
}
impl TestDeduction_wchar_t_Impl for Ptr<TestDeduction_wchar_t_> {
    fn operator_call(&self) {
        let mut x: () = 0_();
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
    }
}
pub trait TestEachAtomicType_TestDeduction_Impl {
    fn operator_call(&self);
}
impl TestEachAtomicType_TestDeduction_Impl for Ptr<TestEachAtomicType_TestDeduction_> {
    fn operator_call(&self) {
        ({
            TestEachIntegralType_TestDeduction_Impl::operator_call(
                &Rc::new(RefCell::new(
                    <TestEachIntegralType_TestDeduction_>::default(),
                ))
                .as_pointer(),
            )
        });
        ({
            TestEachPointerType_TestDeduction_Impl::operator_call(
                &Rc::new(RefCell::new(<TestEachPointerType_TestDeduction_>::default()))
                    .as_pointer(),
            )
        });
        ({
            TestDeduction_UserAtomicType_Impl::operator_call(
                &Rc::new(RefCell::new(<TestDeduction_UserAtomicType_>::default())).as_pointer(),
            )
        });
        ({
            TestDeduction_LargeUserAtomicType_Impl::operator_call(
                &Rc::new(RefCell::new(<TestDeduction_LargeUserAtomicType_>::default()))
                    .as_pointer(),
            )
        });
        ({
            TestDeduction_float_Impl::operator_call(
                &Rc::new(RefCell::new(<TestDeduction_float_>::default())).as_pointer(),
            )
        });
        ({
            TestDeduction_double_Impl::operator_call(
                &Rc::new(RefCell::new(<TestDeduction_double_>::default())).as_pointer(),
            )
        });
    }
}
pub trait TestEachIntegralType_TestDeduction_Impl {
    fn operator_call(&self);
}
impl TestEachIntegralType_TestDeduction_Impl for Ptr<TestEachIntegralType_TestDeduction_> {
    fn operator_call(&self) {
        ({
            TestDeduction_char_Impl::operator_call(
                &Rc::new(RefCell::new(<TestDeduction_char_>::default())).as_pointer(),
            )
        });
        ({
            TestDeduction_signed_char_Impl::operator_call(
                &Rc::new(RefCell::new(<TestDeduction_signed_char_>::default())).as_pointer(),
            )
        });
        ({
            TestDeduction_unsigned_char_Impl::operator_call(
                &Rc::new(RefCell::new(<TestDeduction_unsigned_char_>::default())).as_pointer(),
            )
        });
        ({
            TestDeduction_short_Impl::operator_call(
                &Rc::new(RefCell::new(<TestDeduction_short_>::default())).as_pointer(),
            )
        });
        ({
            TestDeduction_unsigned_short_Impl::operator_call(
                &Rc::new(RefCell::new(<TestDeduction_unsigned_short_>::default())).as_pointer(),
            )
        });
        ({
            TestDeduction_int_Impl::operator_call(
                &Rc::new(RefCell::new(<TestDeduction_int_>::default())).as_pointer(),
            )
        });
        ({
            TestDeduction_unsigned_int_Impl::operator_call(
                &Rc::new(RefCell::new(<TestDeduction_unsigned_int_>::default())).as_pointer(),
            )
        });
        ({
            TestDeduction_long_Impl::operator_call(
                &Rc::new(RefCell::new(<TestDeduction_long_>::default())).as_pointer(),
            )
        });
        ({
            TestDeduction_unsigned_long_Impl::operator_call(
                &Rc::new(RefCell::new(<TestDeduction_unsigned_long_>::default())).as_pointer(),
            )
        });
        ({
            TestDeduction_long_long_Impl::operator_call(
                &Rc::new(RefCell::new(<TestDeduction_long_long_>::default())).as_pointer(),
            )
        });
        ({
            TestDeduction_unsigned_long_long_Impl::operator_call(
                &Rc::new(RefCell::new(<TestDeduction_unsigned_long_long_>::default())).as_pointer(),
            )
        });
        ({
            TestDeduction_wchar_t_Impl::operator_call(
                &Rc::new(RefCell::new(<TestDeduction_wchar_t_>::default())).as_pointer(),
            )
        });
        ({
            TestDeduction_char8_t_Impl::operator_call(
                &Rc::new(RefCell::new(<TestDeduction_char8_t_>::default())).as_pointer(),
            )
        });
        ({
            TestDeduction_char16_t_Impl::operator_call(
                &Rc::new(RefCell::new(<TestDeduction_char16_t_>::default())).as_pointer(),
            )
        });
        ({
            TestDeduction_char32_t_Impl::operator_call(
                &Rc::new(RefCell::new(<TestDeduction_char32_t_>::default())).as_pointer(),
            )
        });
        ({
            TestDeduction_signed_char_Impl::operator_call(
                &Rc::new(RefCell::new(<TestDeduction_signed_char_>::default())).as_pointer(),
            )
        });
        ({
            TestDeduction_unsigned_char_Impl::operator_call(
                &Rc::new(RefCell::new(<TestDeduction_unsigned_char_>::default())).as_pointer(),
            )
        });
        ({
            TestDeduction_short_Impl::operator_call(
                &Rc::new(RefCell::new(<TestDeduction_short_>::default())).as_pointer(),
            )
        });
        ({
            TestDeduction_unsigned_short_Impl::operator_call(
                &Rc::new(RefCell::new(<TestDeduction_unsigned_short_>::default())).as_pointer(),
            )
        });
        ({
            TestDeduction_int_Impl::operator_call(
                &Rc::new(RefCell::new(<TestDeduction_int_>::default())).as_pointer(),
            )
        });
        ({
            TestDeduction_unsigned_int_Impl::operator_call(
                &Rc::new(RefCell::new(<TestDeduction_unsigned_int_>::default())).as_pointer(),
            )
        });
        ({
            TestDeduction_long_Impl::operator_call(
                &Rc::new(RefCell::new(<TestDeduction_long_>::default())).as_pointer(),
            )
        });
        ({
            TestDeduction_unsigned_long_Impl::operator_call(
                &Rc::new(RefCell::new(<TestDeduction_unsigned_long_>::default())).as_pointer(),
            )
        });
    }
}
pub trait TestEachPointerType_TestDeduction_Impl {
    fn operator_call(&self);
}
impl TestEachPointerType_TestDeduction_Impl for Ptr<TestEachPointerType_TestDeduction_> {
    fn operator_call(&self) {
        ({
            TestDeduction_int_ptr_Impl::operator_call(
                &Rc::new(RefCell::new(<TestDeduction_int_ptr_>::default())).as_pointer(),
            )
        });
        ({
            TestDeduction_const_int_ptr_Impl::operator_call(
                &Rc::new(RefCell::new(<TestDeduction_const_int_ptr_>::default())).as_pointer(),
            )
        });
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = value_21.with(|_| ());
    let _ = status_known_22.with(|_| ());
    let _ = value_23.with(|_| ());
    let _ = status_known_24.with(|_| ());
    let _ = value_25.with(|_| ());
    let _ = status_known_26.with(|_| ());
    let _ = value_27.with(|_| ());
    let _ = status_known_28.with(|_| ());
    let _ = value_29.with(|_| ());
    let _ = status_known_30.with(|_| ());
    let _ = value_31.with(|_| ());
    let _ = status_known_32.with(|_| ());
}
