extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type LockFreeStatus = i32;
pub const LockFreeStatus_unknown: LockFreeStatus = -1;
pub const LockFreeStatus_never: LockFreeStatus = 0;
pub const LockFreeStatus_sometimes: LockFreeStatus = 1;
pub const LockFreeStatus_always: LockFreeStatus = 2;
thread_local!(
    pub static value_0: Value<LockFreeStatus> = Rc::new(RefCell::new(
        ((if ((::std::mem::size_of::<i8>() == ::std::mem::size_of::<i8>())
            && (::std::mem::align_of::<i8>() >= ::std::mem::size_of::<i8>()))
        {
            2
        } else {
            (if ((::std::mem::size_of::<i8>() == ::std::mem::size_of::<i16>())
                && (::std::mem::align_of::<i8>() >= ::std::mem::size_of::<i8>()))
            {
                2
            } else {
                (if ((::std::mem::size_of::<i8>() == ::std::mem::size_of::<i32>())
                    && (::std::mem::align_of::<i8>() >= ::std::mem::size_of::<i8>()))
                {
                    2
                } else {
                    (if ((::std::mem::size_of::<i8>() == ::std::mem::size_of::<i64>())
                        && (::std::mem::align_of::<i8>() >= ::std::mem::size_of::<i8>()))
                    {
                        2
                    } else {
                        (if ((::std::mem::size_of::<i8>() == ::std::mem::size_of::<i64>())
                            && (::std::mem::align_of::<i8>() >= ::std::mem::size_of::<i8>()))
                        {
                            2
                        } else {
                            (if ((::std::mem::size_of::<i8>() == 8usize)
                                && (::std::mem::align_of::<i8>() >= ::std::mem::size_of::<i8>()))
                            {
                                2
                            } else {
                                -1_i32
                            })
                        })
                    })
                })
            })
        }) as LockFreeStatus),
    ));
);
thread_local!(
    pub static status_known_1: Value<bool> = Rc::new(RefCell::new(
        (value_0.with(|rc| *rc.borrow()) != LockFreeStatus_unknown),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct LockFreeStatusInfo_char_ {}
thread_local!(
    pub static value_2: Value<LockFreeStatus> = Rc::new(RefCell::new(
        ((if ((::std::mem::size_of::<i16>() == ::std::mem::size_of::<i8>())
            && (::std::mem::align_of::<i16>() >= ::std::mem::size_of::<i16>()))
        {
            2
        } else {
            (if ((::std::mem::size_of::<i16>() == ::std::mem::size_of::<i16>())
                && (::std::mem::align_of::<i16>() >= ::std::mem::size_of::<i16>()))
            {
                2
            } else {
                (if ((::std::mem::size_of::<i16>() == ::std::mem::size_of::<i32>())
                    && (::std::mem::align_of::<i16>() >= ::std::mem::size_of::<i16>()))
                {
                    2
                } else {
                    (if ((::std::mem::size_of::<i16>() == ::std::mem::size_of::<i64>())
                        && (::std::mem::align_of::<i16>() >= ::std::mem::size_of::<i16>()))
                    {
                        2
                    } else {
                        (if ((::std::mem::size_of::<i16>() == ::std::mem::size_of::<i64>())
                            && (::std::mem::align_of::<i16>() >= ::std::mem::size_of::<i16>()))
                        {
                            2
                        } else {
                            (if ((::std::mem::size_of::<i16>() == 8usize)
                                && (::std::mem::align_of::<i16>() >= ::std::mem::size_of::<i16>()))
                            {
                                2
                            } else {
                                -1_i32
                            })
                        })
                    })
                })
            })
        }) as LockFreeStatus),
    ));
);
thread_local!(
    pub static status_known_3: Value<bool> = Rc::new(RefCell::new(
        (value_2.with(|rc| *rc.borrow()) != LockFreeStatus_unknown),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct LockFreeStatusInfo_short_ {}
thread_local!(
    pub static value_4: Value<LockFreeStatus> = Rc::new(RefCell::new(
        ((if ((::std::mem::size_of::<i32>() == ::std::mem::size_of::<i8>())
            && (::std::mem::align_of::<i32>() >= ::std::mem::size_of::<i32>()))
        {
            2
        } else {
            (if ((::std::mem::size_of::<i32>() == ::std::mem::size_of::<i16>())
                && (::std::mem::align_of::<i32>() >= ::std::mem::size_of::<i32>()))
            {
                2
            } else {
                (if ((::std::mem::size_of::<i32>() == ::std::mem::size_of::<i32>())
                    && (::std::mem::align_of::<i32>() >= ::std::mem::size_of::<i32>()))
                {
                    2
                } else {
                    (if ((::std::mem::size_of::<i32>() == ::std::mem::size_of::<i64>())
                        && (::std::mem::align_of::<i32>() >= ::std::mem::size_of::<i32>()))
                    {
                        2
                    } else {
                        (if ((::std::mem::size_of::<i32>() == ::std::mem::size_of::<i64>())
                            && (::std::mem::align_of::<i32>() >= ::std::mem::size_of::<i32>()))
                        {
                            2
                        } else {
                            (if ((::std::mem::size_of::<i32>() == 8usize)
                                && (::std::mem::align_of::<i32>() >= ::std::mem::size_of::<i32>()))
                            {
                                2
                            } else {
                                -1_i32
                            })
                        })
                    })
                })
            })
        }) as LockFreeStatus),
    ));
);
thread_local!(
    pub static status_known_5: Value<bool> = Rc::new(RefCell::new(
        (value_4.with(|rc| *rc.borrow()) != LockFreeStatus_unknown),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct LockFreeStatusInfo_int_ {}
thread_local!(
    pub static value_6: Value<LockFreeStatus> = Rc::new(RefCell::new(
        ((if ((::std::mem::size_of::<i64>() == ::std::mem::size_of::<i8>())
            && (::std::mem::align_of::<i64>() >= ::std::mem::size_of::<i64>()))
        {
            2
        } else {
            (if ((::std::mem::size_of::<i64>() == ::std::mem::size_of::<i16>())
                && (::std::mem::align_of::<i64>() >= ::std::mem::size_of::<i64>()))
            {
                2
            } else {
                (if ((::std::mem::size_of::<i64>() == ::std::mem::size_of::<i32>())
                    && (::std::mem::align_of::<i64>() >= ::std::mem::size_of::<i64>()))
                {
                    2
                } else {
                    (if ((::std::mem::size_of::<i64>() == ::std::mem::size_of::<i64>())
                        && (::std::mem::align_of::<i64>() >= ::std::mem::size_of::<i64>()))
                    {
                        2
                    } else {
                        (if ((::std::mem::size_of::<i64>() == ::std::mem::size_of::<i64>())
                            && (::std::mem::align_of::<i64>() >= ::std::mem::size_of::<i64>()))
                        {
                            2
                        } else {
                            (if ((::std::mem::size_of::<i64>() == 8usize)
                                && (::std::mem::align_of::<i64>() >= ::std::mem::size_of::<i64>()))
                            {
                                2
                            } else {
                                -1_i32
                            })
                        })
                    })
                })
            })
        }) as LockFreeStatus),
    ));
);
thread_local!(
    pub static status_known_7: Value<bool> = Rc::new(RefCell::new(
        (value_6.with(|rc| *rc.borrow()) != LockFreeStatus_unknown),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct LockFreeStatusInfo_long_ {}
thread_local!(
    pub static value_8: Value<LockFreeStatus> = Rc::new(RefCell::new(
        ((if ((8usize == ::std::mem::size_of::<i8>()) && (8usize >= 8usize)) {
            2
        } else {
            (if ((8usize == ::std::mem::size_of::<i16>()) && (8usize >= 8usize)) {
                2
            } else {
                (if ((8usize == ::std::mem::size_of::<i32>()) && (8usize >= 8usize)) {
                    2
                } else {
                    (if ((8usize == ::std::mem::size_of::<i64>()) && (8usize >= 8usize)) {
                        2
                    } else {
                        (if ((8usize == ::std::mem::size_of::<i64>()) && (8usize >= 8usize)) {
                            2
                        } else {
                            (if ((8usize == 8usize) && (8usize >= 8usize)) {
                                2
                            } else {
                                -1_i32
                            })
                        })
                    })
                })
            })
        }) as LockFreeStatus),
    ));
);
thread_local!(
    pub static status_known_9: Value<bool> = Rc::new(RefCell::new(
        (value_8.with(|rc| *rc.borrow()) != LockFreeStatus_unknown),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct LockFreeStatusInfo_void_ptr_ {}
thread_local!(
    pub static value_10: Value<LockFreeStatus> = Rc::new(RefCell::new(
        ((if ((::std::mem::size_of::<i64>() == ::std::mem::size_of::<i8>())
            && (::std::mem::align_of::<i64>() >= ::std::mem::size_of::<i64>()))
        {
            2
        } else {
            (if ((::std::mem::size_of::<i64>() == ::std::mem::size_of::<i16>())
                && (::std::mem::align_of::<i64>() >= ::std::mem::size_of::<i64>()))
            {
                2
            } else {
                (if ((::std::mem::size_of::<i64>() == ::std::mem::size_of::<i32>())
                    && (::std::mem::align_of::<i64>() >= ::std::mem::size_of::<i64>()))
                {
                    2
                } else {
                    (if ((::std::mem::size_of::<i64>() == ::std::mem::size_of::<i64>())
                        && (::std::mem::align_of::<i64>() >= ::std::mem::size_of::<i64>()))
                    {
                        2
                    } else {
                        (if ((::std::mem::size_of::<i64>() == ::std::mem::size_of::<i64>())
                            && (::std::mem::align_of::<i64>() >= ::std::mem::size_of::<i64>()))
                        {
                            2
                        } else {
                            (if ((::std::mem::size_of::<i64>() == 8usize)
                                && (::std::mem::align_of::<i64>() >= ::std::mem::size_of::<i64>()))
                            {
                                2
                            } else {
                                -1_i32
                            })
                        })
                    })
                })
            })
        }) as LockFreeStatus),
    ));
);
thread_local!(
    pub static status_known_11: Value<bool> = Rc::new(RefCell::new(
        (value_10.with(|rc| *rc.borrow()) != LockFreeStatus_unknown),
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
pub fn operator_eq_12(x: Ptr<UserAtomicType>, y: Ptr<UserAtomicType>) -> bool {
    return ({ x.with(|__s| __s.i) } == { y.with(|__s| __s.i) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct UserAtomicType {
    #[offset(0)]
    pub i: i32,
}
impl UserAtomicType {
    pub fn new(d: Option<i32>) -> Self {
        let mut d: i32 = d.unwrap_or_else(|| 0);
        Self { i: d }
    }
}
impl CppEq for UserAtomicType {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_12(__lhs.clone(), __rhs.clone())
    }
}
impl Default for UserAtomicType {
    fn default() -> Self {
        { UserAtomicType::new(None) }
    }
}
pub fn operator_eq_13(x: Ptr<LargeUserAtomicType>, y: Ptr<LargeUserAtomicType>) -> bool {
    let mut i: i32 = 0;
    'loop_: while (i < 128) {
        if ({ (array_elem!(x, a, i).read()) } != { (array_elem!(y, a, i).read()) }) {
            return false;
        }
        i.prefix_inc();
    }
    return true;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom)]
#[cpp_from(i32, new, some_value)]
#[byte_size(512)]
pub struct LargeUserAtomicType {
    #[offset(0)]
    #[byte_size(512)]
    pub a: Box<[i32]>,
}
impl LargeUserAtomicType {
    pub fn new(d: Option<i32>) -> Self {
        let mut d: i32 = d.unwrap_or_else(|| 0);
        let __this: Value<LargeUserAtomicType> = Rc::new(RefCell::new(Self {
            a: (0..128).map(|_| 0_i32).collect::<Box<[i32]>>(),
        }));
        let this: Ptr<LargeUserAtomicType> = __this.as_pointer();
        'loop_: for e in IterRange::new(
            (array_field_ptr!(this, a) as Ptr<i32>),
            (array_field_ptr!(this, a) as Ptr<i32>).offset((128_i64) as isize),
        ) {
            let e: Ptr<i32> = (e).clone();
            let __rhs = d.postfix_inc();
            e.write(__rhs);
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl CppEq for LargeUserAtomicType {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_13(__lhs.clone(), __rhs.clone())
    }
}
impl Default for LargeUserAtomicType {
    fn default() -> Self {
        { LargeUserAtomicType::new(None) }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestEachIntegralType_TestOperatorPlusEquals_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestEachPointerType_TestOperatorPlusEquals_ {}
pub fn test_seq_cst_14(store_op: FnPtr<fn(Ptr<()>, i8, i8)>, load_op: FnPtr<fn(Ptr<()>) -> i8>) {
    let store_op: Value<FnPtr<fn(Ptr<()>, i8, i8)>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> i8>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_15(store_op: FnPtr<fn(Ptr<()>, i8, i8)>, load_op: FnPtr<fn(Ptr<()>) -> i8>) {
    let store_op: Value<FnPtr<fn(Ptr<()>, i8, i8)>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> i8>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_16(store_op: FnPtr<fn(Ptr<()>, u8, u8)>, load_op: FnPtr<fn(Ptr<()>) -> u8>) {
    let store_op: Value<FnPtr<fn(Ptr<()>, u8, u8)>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> u8>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_17(store_op: FnPtr<fn(Ptr<()>, i16, i16)>, load_op: FnPtr<fn(Ptr<()>) -> i16>) {
    let store_op: Value<FnPtr<fn(Ptr<()>, i16, i16)>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> i16>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_18(store_op: FnPtr<fn(Ptr<()>, u16, u16)>, load_op: FnPtr<fn(Ptr<()>) -> u16>) {
    let store_op: Value<FnPtr<fn(Ptr<()>, u16, u16)>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> u16>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_19(store_op: FnPtr<fn(Ptr<()>, i32, i32)>, load_op: FnPtr<fn(Ptr<()>) -> i32>) {
    let store_op: Value<FnPtr<fn(Ptr<()>, i32, i32)>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> i32>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_20(store_op: FnPtr<fn(Ptr<()>, u32, u32)>, load_op: FnPtr<fn(Ptr<()>) -> u32>) {
    let store_op: Value<FnPtr<fn(Ptr<()>, u32, u32)>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> u32>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_21(store_op: FnPtr<fn(Ptr<()>, i64, i64)>, load_op: FnPtr<fn(Ptr<()>) -> i64>) {
    let store_op: Value<FnPtr<fn(Ptr<()>, i64, i64)>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> i64>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_22(store_op: FnPtr<fn(Ptr<()>, u64, u64)>, load_op: FnPtr<fn(Ptr<()>) -> u64>) {
    let store_op: Value<FnPtr<fn(Ptr<()>, u64, u64)>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> u64>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_23(store_op: FnPtr<fn(Ptr<()>, i64, i64)>, load_op: FnPtr<fn(Ptr<()>) -> i64>) {
    let store_op: Value<FnPtr<fn(Ptr<()>, i64, i64)>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> i64>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_24(store_op: FnPtr<fn(Ptr<()>, u64, u64)>, load_op: FnPtr<fn(Ptr<()>) -> u64>) {
    let store_op: Value<FnPtr<fn(Ptr<()>, u64, u64)>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> u64>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_25(store_op: FnPtr<fn(Ptr<()>, i32, i32)>, load_op: FnPtr<fn(Ptr<()>) -> i32>) {
    let store_op: Value<FnPtr<fn(Ptr<()>, i32, i32)>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> i32>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_26(store_op: FnPtr<fn(Ptr<()>, u8, u8)>, load_op: FnPtr<fn(Ptr<()>) -> u8>) {
    let store_op: Value<FnPtr<fn(Ptr<()>, u8, u8)>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> u8>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_27(store_op: FnPtr<fn(Ptr<()>, u16, u16)>, load_op: FnPtr<fn(Ptr<()>) -> u16>) {
    let store_op: Value<FnPtr<fn(Ptr<()>, u16, u16)>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> u16>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_28(store_op: FnPtr<fn(Ptr<()>, u32, u32)>, load_op: FnPtr<fn(Ptr<()>) -> u32>) {
    let store_op: Value<FnPtr<fn(Ptr<()>, u32, u32)>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> u32>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_29(store_op: FnPtr<fn(Ptr<()>, f32, f32)>, load_op: FnPtr<fn(Ptr<()>) -> f32>) {
    let store_op: Value<FnPtr<fn(Ptr<()>, f32, f32)>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> f32>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_30(store_op: FnPtr<fn(Ptr<()>, f64, f64)>, load_op: FnPtr<fn(Ptr<()>) -> f64>) {
    let store_op: Value<FnPtr<fn(Ptr<()>, f64, f64)>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> f64>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_31(
    store_op: FnPtr<fn(Ptr<()>, Ptr<i32>, Ptr<i32>)>,
    load_op: FnPtr<fn(Ptr<()>) -> Ptr<i32>>,
) {
    let store_op: Value<FnPtr<fn(Ptr<()>, Ptr<i32>, Ptr<i32>)>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> Ptr<i32>>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_32(
    store_op: FnPtr<fn(Ptr<()>, Ptr<i32>, Ptr<i32>)>,
    load_op: FnPtr<fn(Ptr<()>) -> Ptr<i32>>,
) {
    let store_op: Value<FnPtr<fn(Ptr<()>, Ptr<i32>, Ptr<i32>)>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> Ptr<i32>>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestDoesNotHaveOperatorPlusEquals_bool_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestDoesNotHaveOperatorPlusEquals_UserAtomicType_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestDoesNotHaveOperatorPlusEquals_LargeUserAtomicType_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestOperatorPlusEquals_float_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestOperatorPlusEquals_double_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestOperatorPlusEquals_char_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestOperatorPlusEquals_signed_char_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestOperatorPlusEquals_unsigned_char_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestOperatorPlusEquals_short_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestOperatorPlusEquals_unsigned_short_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestOperatorPlusEquals_int_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestOperatorPlusEquals_unsigned_int_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestOperatorPlusEquals_long_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestOperatorPlusEquals_unsigned_long_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestOperatorPlusEquals_long_long_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestOperatorPlusEquals_unsigned_long_long_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestOperatorPlusEquals_wchar_t_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestOperatorPlusEquals_char8_t_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestOperatorPlusEquals_char16_t_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestOperatorPlusEquals_char32_t_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestOperatorPlusEquals_int_ptr_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestOperatorPlusEquals_const_int_ptr_ {}
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
    ({
        TestEachIntegralType_TestOperatorPlusEquals_Impl::operator_call(
            &Rc::new(RefCell::new(
                <TestEachIntegralType_TestOperatorPlusEquals_>::default(),
            ))
            .as_pointer(),
        )
    });
    ({
        TestOperatorPlusEquals_float_Impl::operator_call(
            &Rc::new(RefCell::new(<TestOperatorPlusEquals_float_>::default())).as_pointer(),
        )
    });
    ({
        TestOperatorPlusEquals_double_Impl::operator_call(
            &Rc::new(RefCell::new(<TestOperatorPlusEquals_double_>::default())).as_pointer(),
        )
    });
    ({
        TestEachPointerType_TestOperatorPlusEquals_Impl::operator_call(
            &Rc::new(RefCell::new(
                <TestEachPointerType_TestOperatorPlusEquals_>::default(),
            ))
            .as_pointer(),
        )
    });
    ({
        TestDoesNotHaveOperatorPlusEquals_bool_Impl::operator_call(
            &Rc::new(RefCell::new(
                <TestDoesNotHaveOperatorPlusEquals_bool_>::default(),
            ))
            .as_pointer(),
        )
    });
    ({
        TestDoesNotHaveOperatorPlusEquals_UserAtomicType_Impl::operator_call(
            &Rc::new(RefCell::new(
                <TestDoesNotHaveOperatorPlusEquals_UserAtomicType_>::default(),
            ))
            .as_pointer(),
        )
    });
    ({
        TestDoesNotHaveOperatorPlusEquals_LargeUserAtomicType_Impl::operator_call(
            &Rc::new(RefCell::new(
                <TestDoesNotHaveOperatorPlusEquals_LargeUserAtomicType_>::default(),
            ))
            .as_pointer(),
        )
    });
    return 0;
}
pub trait TestDoesNotHaveOperatorPlusEquals_LargeUserAtomicType_Impl {
    fn operator_call(&self);
}
impl TestDoesNotHaveOperatorPlusEquals_LargeUserAtomicType_Impl
    for Ptr<TestDoesNotHaveOperatorPlusEquals_LargeUserAtomicType_>
{
    fn operator_call(&self) {
        const _: () = assert!(!(false), "!has_operator_plus_equals<std::atomic_ref<T>>");;
    }
}
pub trait TestDoesNotHaveOperatorPlusEquals_UserAtomicType_Impl {
    fn operator_call(&self);
}
impl TestDoesNotHaveOperatorPlusEquals_UserAtomicType_Impl
    for Ptr<TestDoesNotHaveOperatorPlusEquals_UserAtomicType_>
{
    fn operator_call(&self) {
        const _: () = assert!(!(false), "!has_operator_plus_equals<std::atomic_ref<T>>");;
    }
}
pub trait TestDoesNotHaveOperatorPlusEquals_bool_Impl {
    fn operator_call(&self);
}
impl TestDoesNotHaveOperatorPlusEquals_bool_Impl for Ptr<TestDoesNotHaveOperatorPlusEquals_bool_> {
    fn operator_call(&self) {
        const _: () = assert!(!(false), "!has_operator_plus_equals<std::atomic_ref<T>>");;
    }
}
pub trait TestEachIntegralType_TestOperatorPlusEquals_Impl {
    fn operator_call(&self);
}
impl TestEachIntegralType_TestOperatorPlusEquals_Impl
    for Ptr<TestEachIntegralType_TestOperatorPlusEquals_>
{
    fn operator_call(&self) {
        ({
            TestOperatorPlusEquals_char_Impl::operator_call(
                &Rc::new(RefCell::new(<TestOperatorPlusEquals_char_>::default())).as_pointer(),
            )
        });
        ({
            TestOperatorPlusEquals_signed_char_Impl::operator_call(
                &Rc::new(RefCell::new(
                    <TestOperatorPlusEquals_signed_char_>::default(),
                ))
                .as_pointer(),
            )
        });
        ({
            TestOperatorPlusEquals_unsigned_char_Impl::operator_call(
                &Rc::new(RefCell::new(
                    <TestOperatorPlusEquals_unsigned_char_>::default(),
                ))
                .as_pointer(),
            )
        });
        ({
            TestOperatorPlusEquals_short_Impl::operator_call(
                &Rc::new(RefCell::new(<TestOperatorPlusEquals_short_>::default())).as_pointer(),
            )
        });
        ({
            TestOperatorPlusEquals_unsigned_short_Impl::operator_call(
                &Rc::new(RefCell::new(
                    <TestOperatorPlusEquals_unsigned_short_>::default(),
                ))
                .as_pointer(),
            )
        });
        ({
            TestOperatorPlusEquals_int_Impl::operator_call(
                &Rc::new(RefCell::new(<TestOperatorPlusEquals_int_>::default())).as_pointer(),
            )
        });
        ({
            TestOperatorPlusEquals_unsigned_int_Impl::operator_call(
                &Rc::new(RefCell::new(
                    <TestOperatorPlusEquals_unsigned_int_>::default(),
                ))
                .as_pointer(),
            )
        });
        ({
            TestOperatorPlusEquals_long_Impl::operator_call(
                &Rc::new(RefCell::new(<TestOperatorPlusEquals_long_>::default())).as_pointer(),
            )
        });
        ({
            TestOperatorPlusEquals_unsigned_long_Impl::operator_call(
                &Rc::new(RefCell::new(
                    <TestOperatorPlusEquals_unsigned_long_>::default(),
                ))
                .as_pointer(),
            )
        });
        ({
            TestOperatorPlusEquals_long_long_Impl::operator_call(
                &Rc::new(RefCell::new(<TestOperatorPlusEquals_long_long_>::default())).as_pointer(),
            )
        });
        ({
            TestOperatorPlusEquals_unsigned_long_long_Impl::operator_call(
                &Rc::new(RefCell::new(
                    <TestOperatorPlusEquals_unsigned_long_long_>::default(),
                ))
                .as_pointer(),
            )
        });
        ({
            TestOperatorPlusEquals_wchar_t_Impl::operator_call(
                &Rc::new(RefCell::new(<TestOperatorPlusEquals_wchar_t_>::default())).as_pointer(),
            )
        });
        ({
            TestOperatorPlusEquals_char8_t_Impl::operator_call(
                &Rc::new(RefCell::new(<TestOperatorPlusEquals_char8_t_>::default())).as_pointer(),
            )
        });
        ({
            TestOperatorPlusEquals_char16_t_Impl::operator_call(
                &Rc::new(RefCell::new(<TestOperatorPlusEquals_char16_t_>::default())).as_pointer(),
            )
        });
        ({
            TestOperatorPlusEquals_char32_t_Impl::operator_call(
                &Rc::new(RefCell::new(<TestOperatorPlusEquals_char32_t_>::default())).as_pointer(),
            )
        });
        ({
            TestOperatorPlusEquals_signed_char_Impl::operator_call(
                &Rc::new(RefCell::new(
                    <TestOperatorPlusEquals_signed_char_>::default(),
                ))
                .as_pointer(),
            )
        });
        ({
            TestOperatorPlusEquals_unsigned_char_Impl::operator_call(
                &Rc::new(RefCell::new(
                    <TestOperatorPlusEquals_unsigned_char_>::default(),
                ))
                .as_pointer(),
            )
        });
        ({
            TestOperatorPlusEquals_short_Impl::operator_call(
                &Rc::new(RefCell::new(<TestOperatorPlusEquals_short_>::default())).as_pointer(),
            )
        });
        ({
            TestOperatorPlusEquals_unsigned_short_Impl::operator_call(
                &Rc::new(RefCell::new(
                    <TestOperatorPlusEquals_unsigned_short_>::default(),
                ))
                .as_pointer(),
            )
        });
        ({
            TestOperatorPlusEquals_int_Impl::operator_call(
                &Rc::new(RefCell::new(<TestOperatorPlusEquals_int_>::default())).as_pointer(),
            )
        });
        ({
            TestOperatorPlusEquals_unsigned_int_Impl::operator_call(
                &Rc::new(RefCell::new(
                    <TestOperatorPlusEquals_unsigned_int_>::default(),
                ))
                .as_pointer(),
            )
        });
        ({
            TestOperatorPlusEquals_long_Impl::operator_call(
                &Rc::new(RefCell::new(<TestOperatorPlusEquals_long_>::default())).as_pointer(),
            )
        });
        ({
            TestOperatorPlusEquals_unsigned_long_Impl::operator_call(
                &Rc::new(RefCell::new(
                    <TestOperatorPlusEquals_unsigned_long_>::default(),
                ))
                .as_pointer(),
            )
        });
    }
}
pub trait TestEachPointerType_TestOperatorPlusEquals_Impl {
    fn operator_call(&self);
}
impl TestEachPointerType_TestOperatorPlusEquals_Impl
    for Ptr<TestEachPointerType_TestOperatorPlusEquals_>
{
    fn operator_call(&self) {
        ({
            TestOperatorPlusEquals_int_ptr_Impl::operator_call(
                &Rc::new(RefCell::new(<TestOperatorPlusEquals_int_ptr_>::default())).as_pointer(),
            )
        });
        ({
            TestOperatorPlusEquals_const_int_ptr_Impl::operator_call(
                &Rc::new(RefCell::new(
                    <TestOperatorPlusEquals_const_int_ptr_>::default(),
                ))
                .as_pointer(),
            )
        });
    }
}
pub trait TestOperatorPlusEquals_char16_t_Impl {
    fn operator_call(&self);
}
impl TestOperatorPlusEquals_char16_t_Impl for Ptr<TestOperatorPlusEquals_char16_t_> {
    fn operator_call(&self) {
        {
            let mut x: u16 = 1_u16;
            let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let mut y: u16 = (unimplemented!());
            assert!(((y as i32) == (3_u16 as i32)));
            assert!(((x as i32) == (3_u16 as i32)));
            const _: () = assert!(true, "");;
        }
        {
            let plus_equals: Value<FnPtr<fn(Ptr<()>, u16, u16)>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, u16, u16)>::new(
                    |x: Ptr<()>, mut old_val: u16, mut new_val: u16| {
                        unimplemented!();
                    },
                )));
            let load: Value<FnPtr<fn(Ptr<()>) -> u16>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>) -> u16>::new(
                    |x: Ptr<()>| -> u16 {
                        {
                            return unimplemented!();
                        }
                    },
                )));
            ({
                test_seq_cst_27(
                    (*plus_equals.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
        }
    }
}
pub trait TestOperatorPlusEquals_char32_t_Impl {
    fn operator_call(&self);
}
impl TestOperatorPlusEquals_char32_t_Impl for Ptr<TestOperatorPlusEquals_char32_t_> {
    fn operator_call(&self) {
        {
            let mut x: u32 = 1_u32;
            let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let mut y: u32 = (unimplemented!());
            assert!((y == 3_u32));
            assert!((x == 3_u32));
            const _: () = assert!(true, "");;
        }
        {
            let plus_equals: Value<FnPtr<fn(Ptr<()>, u32, u32)>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, u32, u32)>::new(
                    |x: Ptr<()>, mut old_val: u32, mut new_val: u32| {
                        unimplemented!();
                    },
                )));
            let load: Value<FnPtr<fn(Ptr<()>) -> u32>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>) -> u32>::new(
                    |x: Ptr<()>| -> u32 {
                        {
                            return unimplemented!();
                        }
                    },
                )));
            ({
                test_seq_cst_28(
                    (*plus_equals.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
        }
    }
}
pub trait TestOperatorPlusEquals_char8_t_Impl {
    fn operator_call(&self);
}
impl TestOperatorPlusEquals_char8_t_Impl for Ptr<TestOperatorPlusEquals_char8_t_> {
    fn operator_call(&self) {
        {
            let mut x: u8 = 1_u8;
            let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let mut y: u8 = (unimplemented!());
            assert!(((y as i32) == (3_u8 as i32)));
            assert!(((x as i32) == (3_u8 as i32)));
            const _: () = assert!(true, "");;
        }
        {
            let plus_equals: Value<FnPtr<fn(Ptr<()>, u8, u8)>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, u8, u8)>::new(
                    |x: Ptr<()>, mut old_val: u8, mut new_val: u8| {
                        unimplemented!();
                    },
                )));
            let load: Value<FnPtr<fn(Ptr<()>) -> u8>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>) -> u8>::new(
                    |x: Ptr<()>| -> u8 {
                        {
                            return unimplemented!();
                        }
                    },
                )));
            ({
                test_seq_cst_26(
                    (*plus_equals.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
        }
    }
}
pub trait TestOperatorPlusEquals_char_Impl {
    fn operator_call(&self);
}
impl TestOperatorPlusEquals_char_Impl for Ptr<TestOperatorPlusEquals_char_> {
    fn operator_call(&self) {
        {
            let mut x: i8 = 1_i8;
            let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let mut y: i8 = (unimplemented!());
            assert!(((y as i32) == (3_i8 as i32)));
            assert!(((x as i32) == (3_i8 as i32)));
            const _: () = assert!(true, "");;
        }
        {
            let plus_equals: Value<FnPtr<fn(Ptr<()>, i8, i8)>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, i8, i8)>::new(
                    |x: Ptr<()>, mut old_val: i8, mut new_val: i8| {
                        unimplemented!();
                    },
                )));
            let load: Value<FnPtr<fn(Ptr<()>) -> i8>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>) -> i8>::new(
                    |x: Ptr<()>| -> i8 {
                        {
                            return unimplemented!();
                        }
                    },
                )));
            ({
                test_seq_cst_14(
                    (*plus_equals.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
        }
    }
}
pub trait TestOperatorPlusEquals_const_int_ptr_Impl {
    fn operator_call(&self);
}
impl TestOperatorPlusEquals_const_int_ptr_Impl for Ptr<TestOperatorPlusEquals_const_int_ptr_> {
    fn operator_call(&self) {
        {
            {
                let t: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                ])));
                let mut p: Ptr<i32> = ((t.as_pointer() as Ptr<()>).offset(1));
                let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut y: Ptr<i32> = (unimplemented!());
                assert!(({ (y).clone() } == { ((t.as_pointer() as Ptr<()>).offset(3)) }));
                assert!((unimplemented!() == ((t.as_pointer() as Ptr<()>).offset(3))));
                const _: () = assert!(true, "");;
            }
        }
        {
            let plus_equals: Value<FnPtr<fn(Ptr<()>, Ptr<i32>, Ptr<i32>)>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, Ptr<i32>, Ptr<i32>)>::new(
                    |x: Ptr<()>, mut old_val: Ptr<i32>, mut new_val: Ptr<i32>| {
                        unimplemented!();
                    },
                )));
            let load: Value<FnPtr<fn(Ptr<()>) -> Ptr<i32>>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>) -> Ptr<i32>>::new(
                    |x: Ptr<()>| -> Ptr<i32> {
                        {
                            return unimplemented!();
                        }
                    },
                )));
            ({
                test_seq_cst_32(
                    (*plus_equals.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
        }
    }
}
pub trait TestOperatorPlusEquals_double_Impl {
    fn operator_call(&self);
}
impl TestOperatorPlusEquals_double_Impl for Ptr<TestOperatorPlusEquals_double_> {
    fn operator_call(&self) {
        {
            let mut x: f64 = 1_f64;
            let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let mut y: f64 = (unimplemented!());
            assert!((y == 3_f64));
            assert!((x == 3_f64));
            const _: () = assert!(true, "");;
        }
        {
            let plus_equals: Value<FnPtr<fn(Ptr<()>, f64, f64)>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, f64, f64)>::new(
                    |x: Ptr<()>, mut old_val: f64, mut new_val: f64| {
                        unimplemented!();
                    },
                )));
            let load: Value<FnPtr<fn(Ptr<()>) -> f64>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>) -> f64>::new(
                    |x: Ptr<()>| -> f64 {
                        {
                            return unimplemented!();
                        }
                    },
                )));
            ({
                test_seq_cst_30(
                    (*plus_equals.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
        }
    }
}
pub trait TestOperatorPlusEquals_float_Impl {
    fn operator_call(&self);
}
impl TestOperatorPlusEquals_float_Impl for Ptr<TestOperatorPlusEquals_float_> {
    fn operator_call(&self) {
        {
            let mut x: f32 = 1_f32;
            let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let mut y: f32 = (unimplemented!());
            assert!((y == 3_f32));
            assert!((x == 3_f32));
            const _: () = assert!(true, "");;
        }
        {
            let plus_equals: Value<FnPtr<fn(Ptr<()>, f32, f32)>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, f32, f32)>::new(
                    |x: Ptr<()>, mut old_val: f32, mut new_val: f32| {
                        unimplemented!();
                    },
                )));
            let load: Value<FnPtr<fn(Ptr<()>) -> f32>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>) -> f32>::new(
                    |x: Ptr<()>| -> f32 {
                        {
                            return unimplemented!();
                        }
                    },
                )));
            ({
                test_seq_cst_29(
                    (*plus_equals.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
        }
    }
}
pub trait TestOperatorPlusEquals_int_Impl {
    fn operator_call(&self);
}
impl TestOperatorPlusEquals_int_Impl for Ptr<TestOperatorPlusEquals_int_> {
    fn operator_call(&self) {
        {
            let mut x: i32 = 1;
            let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let mut y: i32 = (unimplemented!());
            assert!((y == 3));
            assert!((x == 3));
            const _: () = assert!(true, "");;
        }
        {
            let plus_equals: Value<FnPtr<fn(Ptr<()>, i32, i32)>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, i32, i32)>::new(
                    |x: Ptr<()>, mut old_val: i32, mut new_val: i32| {
                        unimplemented!();
                    },
                )));
            let load: Value<FnPtr<fn(Ptr<()>) -> i32>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>) -> i32>::new(
                    |x: Ptr<()>| -> i32 {
                        {
                            return unimplemented!();
                        }
                    },
                )));
            ({
                test_seq_cst_19(
                    (*plus_equals.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
        }
    }
}
pub trait TestOperatorPlusEquals_int_ptr_Impl {
    fn operator_call(&self);
}
impl TestOperatorPlusEquals_int_ptr_Impl for Ptr<TestOperatorPlusEquals_int_ptr_> {
    fn operator_call(&self) {
        {
            {
                let t: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                ])));
                let mut p: Ptr<i32> = ((t.as_pointer() as Ptr<()>).offset(1));
                let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let mut y: Ptr<i32> = (unimplemented!());
                assert!(({ (y).clone() } == { ((t.as_pointer() as Ptr<()>).offset(3)) }));
                assert!((unimplemented!() == ((t.as_pointer() as Ptr<()>).offset(3))));
                const _: () = assert!(true, "");;
            }
        }
        {
            let plus_equals: Value<FnPtr<fn(Ptr<()>, Ptr<i32>, Ptr<i32>)>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, Ptr<i32>, Ptr<i32>)>::new(
                    |x: Ptr<()>, mut old_val: Ptr<i32>, mut new_val: Ptr<i32>| {
                        unimplemented!();
                    },
                )));
            let load: Value<FnPtr<fn(Ptr<()>) -> Ptr<i32>>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>) -> Ptr<i32>>::new(
                    |x: Ptr<()>| -> Ptr<i32> {
                        {
                            return unimplemented!();
                        }
                    },
                )));
            ({
                test_seq_cst_31(
                    (*plus_equals.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
        }
    }
}
pub trait TestOperatorPlusEquals_long_Impl {
    fn operator_call(&self);
}
impl TestOperatorPlusEquals_long_Impl for Ptr<TestOperatorPlusEquals_long_> {
    fn operator_call(&self) {
        {
            let mut x: i64 = 1_i64;
            let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let mut y: i64 = (unimplemented!());
            assert!((y == 3_i64));
            assert!((x == 3_i64));
            const _: () = assert!(true, "");;
        }
        {
            let plus_equals: Value<FnPtr<fn(Ptr<()>, i64, i64)>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, i64, i64)>::new(
                    |x: Ptr<()>, mut old_val: i64, mut new_val: i64| {
                        unimplemented!();
                    },
                )));
            let load: Value<FnPtr<fn(Ptr<()>) -> i64>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>) -> i64>::new(
                    |x: Ptr<()>| -> i64 {
                        {
                            return unimplemented!();
                        }
                    },
                )));
            ({
                test_seq_cst_21(
                    (*plus_equals.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
        }
    }
}
pub trait TestOperatorPlusEquals_long_long_Impl {
    fn operator_call(&self);
}
impl TestOperatorPlusEquals_long_long_Impl for Ptr<TestOperatorPlusEquals_long_long_> {
    fn operator_call(&self) {
        {
            let mut x: i64 = 1_i64;
            let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let mut y: i64 = (unimplemented!());
            assert!((y == 3_i64));
            assert!((x == 3_i64));
            const _: () = assert!(true, "");;
        }
        {
            let plus_equals: Value<FnPtr<fn(Ptr<()>, i64, i64)>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, i64, i64)>::new(
                    |x: Ptr<()>, mut old_val: i64, mut new_val: i64| {
                        unimplemented!();
                    },
                )));
            let load: Value<FnPtr<fn(Ptr<()>) -> i64>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>) -> i64>::new(
                    |x: Ptr<()>| -> i64 {
                        {
                            return unimplemented!();
                        }
                    },
                )));
            ({
                test_seq_cst_23(
                    (*plus_equals.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
        }
    }
}
pub trait TestOperatorPlusEquals_short_Impl {
    fn operator_call(&self);
}
impl TestOperatorPlusEquals_short_Impl for Ptr<TestOperatorPlusEquals_short_> {
    fn operator_call(&self) {
        {
            let mut x: i16 = 1_i16;
            let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let mut y: i16 = (unimplemented!());
            assert!(((y as i32) == (3_i16 as i32)));
            assert!(((x as i32) == (3_i16 as i32)));
            const _: () = assert!(true, "");;
        }
        {
            let plus_equals: Value<FnPtr<fn(Ptr<()>, i16, i16)>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, i16, i16)>::new(
                    |x: Ptr<()>, mut old_val: i16, mut new_val: i16| {
                        unimplemented!();
                    },
                )));
            let load: Value<FnPtr<fn(Ptr<()>) -> i16>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>) -> i16>::new(
                    |x: Ptr<()>| -> i16 {
                        {
                            return unimplemented!();
                        }
                    },
                )));
            ({
                test_seq_cst_17(
                    (*plus_equals.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
        }
    }
}
pub trait TestOperatorPlusEquals_signed_char_Impl {
    fn operator_call(&self);
}
impl TestOperatorPlusEquals_signed_char_Impl for Ptr<TestOperatorPlusEquals_signed_char_> {
    fn operator_call(&self) {
        {
            let mut x: i8 = 1_i8;
            let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let mut y: i8 = (unimplemented!());
            assert!(((y as i32) == (3_i8 as i32)));
            assert!(((x as i32) == (3_i8 as i32)));
            const _: () = assert!(true, "");;
        }
        {
            let plus_equals: Value<FnPtr<fn(Ptr<()>, i8, i8)>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, i8, i8)>::new(
                    |x: Ptr<()>, mut old_val: i8, mut new_val: i8| {
                        unimplemented!();
                    },
                )));
            let load: Value<FnPtr<fn(Ptr<()>) -> i8>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>) -> i8>::new(
                    |x: Ptr<()>| -> i8 {
                        {
                            return unimplemented!();
                        }
                    },
                )));
            ({
                test_seq_cst_15(
                    (*plus_equals.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
        }
    }
}
pub trait TestOperatorPlusEquals_unsigned_char_Impl {
    fn operator_call(&self);
}
impl TestOperatorPlusEquals_unsigned_char_Impl for Ptr<TestOperatorPlusEquals_unsigned_char_> {
    fn operator_call(&self) {
        {
            let mut x: u8 = 1_u8;
            let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let mut y: u8 = (unimplemented!());
            assert!(((y as i32) == (3_u8 as i32)));
            assert!(((x as i32) == (3_u8 as i32)));
            const _: () = assert!(true, "");;
        }
        {
            let plus_equals: Value<FnPtr<fn(Ptr<()>, u8, u8)>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, u8, u8)>::new(
                    |x: Ptr<()>, mut old_val: u8, mut new_val: u8| {
                        unimplemented!();
                    },
                )));
            let load: Value<FnPtr<fn(Ptr<()>) -> u8>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>) -> u8>::new(
                    |x: Ptr<()>| -> u8 {
                        {
                            return unimplemented!();
                        }
                    },
                )));
            ({
                test_seq_cst_16(
                    (*plus_equals.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
        }
    }
}
pub trait TestOperatorPlusEquals_unsigned_int_Impl {
    fn operator_call(&self);
}
impl TestOperatorPlusEquals_unsigned_int_Impl for Ptr<TestOperatorPlusEquals_unsigned_int_> {
    fn operator_call(&self) {
        {
            let mut x: u32 = 1_u32;
            let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let mut y: u32 = (unimplemented!());
            assert!((y == 3_u32));
            assert!((x == 3_u32));
            const _: () = assert!(true, "");;
        }
        {
            let plus_equals: Value<FnPtr<fn(Ptr<()>, u32, u32)>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, u32, u32)>::new(
                    |x: Ptr<()>, mut old_val: u32, mut new_val: u32| {
                        unimplemented!();
                    },
                )));
            let load: Value<FnPtr<fn(Ptr<()>) -> u32>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>) -> u32>::new(
                    |x: Ptr<()>| -> u32 {
                        {
                            return unimplemented!();
                        }
                    },
                )));
            ({
                test_seq_cst_20(
                    (*plus_equals.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
        }
    }
}
pub trait TestOperatorPlusEquals_unsigned_long_Impl {
    fn operator_call(&self);
}
impl TestOperatorPlusEquals_unsigned_long_Impl for Ptr<TestOperatorPlusEquals_unsigned_long_> {
    fn operator_call(&self) {
        {
            let mut x: u64 = 1_u64;
            let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let mut y: u64 = (unimplemented!());
            assert!((y == 3_u64));
            assert!((x == 3_u64));
            const _: () = assert!(true, "");;
        }
        {
            let plus_equals: Value<FnPtr<fn(Ptr<()>, u64, u64)>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, u64, u64)>::new(
                    |x: Ptr<()>, mut old_val: u64, mut new_val: u64| {
                        unimplemented!();
                    },
                )));
            let load: Value<FnPtr<fn(Ptr<()>) -> u64>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>) -> u64>::new(
                    |x: Ptr<()>| -> u64 {
                        {
                            return unimplemented!();
                        }
                    },
                )));
            ({
                test_seq_cst_22(
                    (*plus_equals.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
        }
    }
}
pub trait TestOperatorPlusEquals_unsigned_long_long_Impl {
    fn operator_call(&self);
}
impl TestOperatorPlusEquals_unsigned_long_long_Impl
    for Ptr<TestOperatorPlusEquals_unsigned_long_long_>
{
    fn operator_call(&self) {
        {
            let mut x: u64 = 1_u64;
            let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let mut y: u64 = (unimplemented!());
            assert!((y == 3_u64));
            assert!((x == 3_u64));
            const _: () = assert!(true, "");;
        }
        {
            let plus_equals: Value<FnPtr<fn(Ptr<()>, u64, u64)>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, u64, u64)>::new(
                    |x: Ptr<()>, mut old_val: u64, mut new_val: u64| {
                        unimplemented!();
                    },
                )));
            let load: Value<FnPtr<fn(Ptr<()>) -> u64>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>) -> u64>::new(
                    |x: Ptr<()>| -> u64 {
                        {
                            return unimplemented!();
                        }
                    },
                )));
            ({
                test_seq_cst_24(
                    (*plus_equals.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
        }
    }
}
pub trait TestOperatorPlusEquals_unsigned_short_Impl {
    fn operator_call(&self);
}
impl TestOperatorPlusEquals_unsigned_short_Impl for Ptr<TestOperatorPlusEquals_unsigned_short_> {
    fn operator_call(&self) {
        {
            let mut x: u16 = 1_u16;
            let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let mut y: u16 = (unimplemented!());
            assert!(((y as i32) == (3_u16 as i32)));
            assert!(((x as i32) == (3_u16 as i32)));
            const _: () = assert!(true, "");;
        }
        {
            let plus_equals: Value<FnPtr<fn(Ptr<()>, u16, u16)>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, u16, u16)>::new(
                    |x: Ptr<()>, mut old_val: u16, mut new_val: u16| {
                        unimplemented!();
                    },
                )));
            let load: Value<FnPtr<fn(Ptr<()>) -> u16>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>) -> u16>::new(
                    |x: Ptr<()>| -> u16 {
                        {
                            return unimplemented!();
                        }
                    },
                )));
            ({
                test_seq_cst_18(
                    (*plus_equals.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
        }
    }
}
pub trait TestOperatorPlusEquals_wchar_t_Impl {
    fn operator_call(&self);
}
impl TestOperatorPlusEquals_wchar_t_Impl for Ptr<TestOperatorPlusEquals_wchar_t_> {
    fn operator_call(&self) {
        {
            let mut x: i32 = 1_i32;
            let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let mut y: i32 = (unimplemented!());
            assert!((y == 3_i32));
            assert!((x == 3_i32));
            const _: () = assert!(true, "");;
        }
        {
            let plus_equals: Value<FnPtr<fn(Ptr<()>, i32, i32)>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, i32, i32)>::new(
                    |x: Ptr<()>, mut old_val: i32, mut new_val: i32| {
                        unimplemented!();
                    },
                )));
            let load: Value<FnPtr<fn(Ptr<()>) -> i32>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>) -> i32>::new(
                    |x: Ptr<()>| -> i32 {
                        {
                            return unimplemented!();
                        }
                    },
                )));
            ({
                test_seq_cst_25(
                    (*plus_equals.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
        }
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = value_0.with(|_| ());
    let _ = status_known_1.with(|_| ());
    let _ = value_2.with(|_| ());
    let _ = status_known_3.with(|_| ());
    let _ = value_4.with(|_| ());
    let _ = status_known_5.with(|_| ());
    let _ = value_6.with(|_| ());
    let _ = status_known_7.with(|_| ());
    let _ = value_8.with(|_| ());
    let _ = status_known_9.with(|_| ());
    let _ = value_10.with(|_| ());
    let _ = status_known_11.with(|_| ());
}
