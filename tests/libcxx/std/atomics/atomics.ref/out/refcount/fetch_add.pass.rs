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
impl CppMove for LockFreeStatusInfo_char_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
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
impl CppMove for LockFreeStatusInfo_short_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
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
impl CppMove for LockFreeStatusInfo_int_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
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
impl CppMove for LockFreeStatusInfo_long_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
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
impl CppMove for LockFreeStatusInfo_void_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
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
impl CppMove for LockFreeStatusInfo_long_long_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
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
impl CppMove for UserAtomicType {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
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
impl CppMove for LargeUserAtomicType {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for LargeUserAtomicType {
    fn default() -> Self {
        { LargeUserAtomicType::new(None) }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestEachIntegralType_TestFetchAdd_ {}
impl Callable0<()> for TestEachIntegralType_TestFetchAdd_ {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            TestEachIntegralType_TestFetchAdd_Impl::operator_call(__it)
        })
    }
}
impl CppMove for TestEachIntegralType_TestFetchAdd_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestEachPointerType_TestFetchAdd_ {}
impl Callable0<()> for TestEachPointerType_TestFetchAdd_ {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            TestEachPointerType_TestFetchAdd_Impl::operator_call(__it)
        })
    }
}
impl CppMove for TestEachPointerType_TestFetchAdd_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn test_seq_cst_14(
    store_op: FnPtr<fn(Ptr<std_atomic_ref_char_>, i8, i8)>,
    load_op: FnPtr<fn(Ptr<std_atomic_ref_char_>) -> i8>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_ref_char_>, i8, i8)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_ref_char_>) -> i8>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_15(
    store_op: FnPtr<fn(Ptr<std_atomic_ref_char_>, i8, i8)>,
    load_op: FnPtr<fn(Ptr<std_atomic_ref_char_>) -> i8>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_ref_char_>, i8, i8)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_ref_char_>) -> i8>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_16(
    store_op: FnPtr<fn(Ptr<std_atomic_ref_signed_char_>, i8, i8)>,
    load_op: FnPtr<fn(Ptr<std_atomic_ref_signed_char_>) -> i8>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_ref_signed_char_>, i8, i8)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_ref_signed_char_>) -> i8>> =
        Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_17(
    store_op: FnPtr<fn(Ptr<std_atomic_ref_signed_char_>, i8, i8)>,
    load_op: FnPtr<fn(Ptr<std_atomic_ref_signed_char_>) -> i8>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_ref_signed_char_>, i8, i8)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_ref_signed_char_>) -> i8>> =
        Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_18(
    store_op: FnPtr<fn(Ptr<std_atomic_ref_unsigned_char_>, u8, u8)>,
    load_op: FnPtr<fn(Ptr<std_atomic_ref_unsigned_char_>) -> u8>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_ref_unsigned_char_>, u8, u8)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_ref_unsigned_char_>) -> u8>> =
        Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_19(
    store_op: FnPtr<fn(Ptr<std_atomic_ref_unsigned_char_>, u8, u8)>,
    load_op: FnPtr<fn(Ptr<std_atomic_ref_unsigned_char_>) -> u8>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_ref_unsigned_char_>, u8, u8)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_ref_unsigned_char_>) -> u8>> =
        Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_20(
    store_op: FnPtr<fn(Ptr<std_atomic_ref_short_>, i16, i16)>,
    load_op: FnPtr<fn(Ptr<std_atomic_ref_short_>) -> i16>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_ref_short_>, i16, i16)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_ref_short_>) -> i16>> =
        Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_21(
    store_op: FnPtr<fn(Ptr<std_atomic_ref_short_>, i16, i16)>,
    load_op: FnPtr<fn(Ptr<std_atomic_ref_short_>) -> i16>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_ref_short_>, i16, i16)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_ref_short_>) -> i16>> =
        Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_22(
    store_op: FnPtr<fn(Ptr<std_atomic_ref_unsigned_short_>, u16, u16)>,
    load_op: FnPtr<fn(Ptr<std_atomic_ref_unsigned_short_>) -> u16>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_ref_unsigned_short_>, u16, u16)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_ref_unsigned_short_>) -> u16>> =
        Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_23(
    store_op: FnPtr<fn(Ptr<std_atomic_ref_unsigned_short_>, u16, u16)>,
    load_op: FnPtr<fn(Ptr<std_atomic_ref_unsigned_short_>) -> u16>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_ref_unsigned_short_>, u16, u16)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_ref_unsigned_short_>) -> u16>> =
        Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_24(
    store_op: FnPtr<fn(Ptr<std_atomic_ref_int_>, i32, i32)>,
    load_op: FnPtr<fn(Ptr<std_atomic_ref_int_>) -> i32>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_ref_int_>, i32, i32)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_ref_int_>) -> i32>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_25(
    store_op: FnPtr<fn(Ptr<std_atomic_ref_int_>, i32, i32)>,
    load_op: FnPtr<fn(Ptr<std_atomic_ref_int_>) -> i32>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_ref_int_>, i32, i32)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_ref_int_>) -> i32>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_26(
    store_op: FnPtr<fn(Ptr<std_atomic_ref_unsigned_int_>, u32, u32)>,
    load_op: FnPtr<fn(Ptr<std_atomic_ref_unsigned_int_>) -> u32>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_ref_unsigned_int_>, u32, u32)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_ref_unsigned_int_>) -> u32>> =
        Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_27(
    store_op: FnPtr<fn(Ptr<std_atomic_ref_unsigned_int_>, u32, u32)>,
    load_op: FnPtr<fn(Ptr<std_atomic_ref_unsigned_int_>) -> u32>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_ref_unsigned_int_>, u32, u32)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_ref_unsigned_int_>) -> u32>> =
        Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_28(
    store_op: FnPtr<fn(Ptr<std_atomic_ref_long_>, i64, i64)>,
    load_op: FnPtr<fn(Ptr<std_atomic_ref_long_>) -> i64>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_ref_long_>, i64, i64)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_ref_long_>) -> i64>> =
        Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_29(
    store_op: FnPtr<fn(Ptr<std_atomic_ref_long_>, i64, i64)>,
    load_op: FnPtr<fn(Ptr<std_atomic_ref_long_>) -> i64>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_ref_long_>, i64, i64)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_ref_long_>) -> i64>> =
        Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_30(
    store_op: FnPtr<fn(Ptr<std_atomic_ref_unsigned_long_>, u64, u64)>,
    load_op: FnPtr<fn(Ptr<std_atomic_ref_unsigned_long_>) -> u64>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_ref_unsigned_long_>, u64, u64)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_ref_unsigned_long_>) -> u64>> =
        Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_31(
    store_op: FnPtr<fn(Ptr<std_atomic_ref_unsigned_long_>, u64, u64)>,
    load_op: FnPtr<fn(Ptr<std_atomic_ref_unsigned_long_>) -> u64>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_ref_unsigned_long_>, u64, u64)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_ref_unsigned_long_>) -> u64>> =
        Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_32(
    store_op: FnPtr<fn(Ptr<std_atomic_ref_long_long_>, i64, i64)>,
    load_op: FnPtr<fn(Ptr<std_atomic_ref_long_long_>) -> i64>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_ref_long_long_>, i64, i64)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_ref_long_long_>) -> i64>> =
        Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_33(
    store_op: FnPtr<fn(Ptr<std_atomic_ref_long_long_>, i64, i64)>,
    load_op: FnPtr<fn(Ptr<std_atomic_ref_long_long_>) -> i64>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_ref_long_long_>, i64, i64)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_ref_long_long_>) -> i64>> =
        Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_34(
    store_op: FnPtr<fn(Ptr<std_atomic_ref_unsigned_long_long_>, u64, u64)>,
    load_op: FnPtr<fn(Ptr<std_atomic_ref_unsigned_long_long_>) -> u64>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_ref_unsigned_long_long_>, u64, u64)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_ref_unsigned_long_long_>) -> u64>> =
        Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_35(
    store_op: FnPtr<fn(Ptr<std_atomic_ref_unsigned_long_long_>, u64, u64)>,
    load_op: FnPtr<fn(Ptr<std_atomic_ref_unsigned_long_long_>) -> u64>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_ref_unsigned_long_long_>, u64, u64)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_ref_unsigned_long_long_>) -> u64>> =
        Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_36(
    store_op: FnPtr<fn(Ptr<std_atomic_ref_wchar_t_>, i32, i32)>,
    load_op: FnPtr<fn(Ptr<std_atomic_ref_wchar_t_>) -> i32>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_ref_wchar_t_>, i32, i32)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_ref_wchar_t_>) -> i32>> =
        Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_37(
    store_op: FnPtr<fn(Ptr<std_atomic_ref_wchar_t_>, i32, i32)>,
    load_op: FnPtr<fn(Ptr<std_atomic_ref_wchar_t_>) -> i32>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_ref_wchar_t_>, i32, i32)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_ref_wchar_t_>) -> i32>> =
        Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_38(
    store_op: FnPtr<fn(Ptr<std_atomic_ref_char8_t_>, u8, u8)>,
    load_op: FnPtr<fn(Ptr<std_atomic_ref_char8_t_>) -> u8>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_ref_char8_t_>, u8, u8)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_ref_char8_t_>) -> u8>> =
        Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_39(
    store_op: FnPtr<fn(Ptr<std_atomic_ref_char8_t_>, u8, u8)>,
    load_op: FnPtr<fn(Ptr<std_atomic_ref_char8_t_>) -> u8>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_ref_char8_t_>, u8, u8)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_ref_char8_t_>) -> u8>> =
        Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_40(
    store_op: FnPtr<fn(Ptr<std_atomic_ref_char16_t_>, u16, u16)>,
    load_op: FnPtr<fn(Ptr<std_atomic_ref_char16_t_>) -> u16>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_ref_char16_t_>, u16, u16)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_ref_char16_t_>) -> u16>> =
        Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_41(
    store_op: FnPtr<fn(Ptr<std_atomic_ref_char16_t_>, u16, u16)>,
    load_op: FnPtr<fn(Ptr<std_atomic_ref_char16_t_>) -> u16>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_ref_char16_t_>, u16, u16)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_ref_char16_t_>) -> u16>> =
        Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_42(
    store_op: FnPtr<fn(Ptr<std_atomic_ref_char32_t_>, u32, u32)>,
    load_op: FnPtr<fn(Ptr<std_atomic_ref_char32_t_>) -> u32>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_ref_char32_t_>, u32, u32)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_ref_char32_t_>) -> u32>> =
        Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_43(
    store_op: FnPtr<fn(Ptr<std_atomic_ref_char32_t_>, u32, u32)>,
    load_op: FnPtr<fn(Ptr<std_atomic_ref_char32_t_>) -> u32>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_ref_char32_t_>, u32, u32)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_ref_char32_t_>) -> u32>> =
        Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_44(
    store_op: FnPtr<fn(Ptr<std_atomic_ref_float_>, f32, f32)>,
    load_op: FnPtr<fn(Ptr<std_atomic_ref_float_>) -> f32>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_ref_float_>, f32, f32)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_ref_float_>) -> f32>> =
        Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_45(
    store_op: FnPtr<fn(Ptr<std_atomic_ref_float_>, f32, f32)>,
    load_op: FnPtr<fn(Ptr<std_atomic_ref_float_>) -> f32>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_ref_float_>, f32, f32)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_ref_float_>) -> f32>> =
        Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_46(
    store_op: FnPtr<fn(Ptr<std_atomic_ref_double_>, f64, f64)>,
    load_op: FnPtr<fn(Ptr<std_atomic_ref_double_>) -> f64>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_ref_double_>, f64, f64)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_ref_double_>) -> f64>> =
        Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_47(
    store_op: FnPtr<fn(Ptr<std_atomic_ref_double_>, f64, f64)>,
    load_op: FnPtr<fn(Ptr<std_atomic_ref_double_>) -> f64>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_ref_double_>, f64, f64)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_ref_double_>) -> f64>> =
        Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_48(
    store_op: FnPtr<fn(Ptr<std_atomic_ref_int_ptr_>, Ptr<i32>, Ptr<i32>)>,
    load_op: FnPtr<fn(Ptr<std_atomic_ref_int_ptr_>) -> Ptr<i32>>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_ref_int_ptr_>, Ptr<i32>, Ptr<i32>)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_ref_int_ptr_>) -> Ptr<i32>>> =
        Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_49(
    store_op: FnPtr<fn(Ptr<std_atomic_ref_int_ptr_>, Ptr<i32>, Ptr<i32>)>,
    load_op: FnPtr<fn(Ptr<std_atomic_ref_int_ptr_>) -> Ptr<i32>>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_ref_int_ptr_>, Ptr<i32>, Ptr<i32>)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_ref_int_ptr_>) -> Ptr<i32>>> =
        Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_50(
    store_op: FnPtr<fn(Ptr<std_atomic_ref_const_int_ptr_>, Ptr<i32>, Ptr<i32>)>,
    load_op: FnPtr<fn(Ptr<std_atomic_ref_const_int_ptr_>) -> Ptr<i32>>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_ref_const_int_ptr_>, Ptr<i32>, Ptr<i32>)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_ref_const_int_ptr_>) -> Ptr<i32>>> =
        Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_51(
    store_op: FnPtr<fn(Ptr<std_atomic_ref_const_int_ptr_>, Ptr<i32>, Ptr<i32>)>,
    load_op: FnPtr<fn(Ptr<std_atomic_ref_const_int_ptr_>) -> Ptr<i32>>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_ref_const_int_ptr_>, Ptr<i32>, Ptr<i32>)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_ref_const_int_ptr_>) -> Ptr<i32>>> =
        Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_52(
    store_op: FnPtr<fn(Ptr<std_atomic_ref_char_>, i8, i8)>,
    load_op: FnPtr<fn(Ptr<std_atomic_ref_char_>) -> i8>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_ref_char_>, i8, i8)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_ref_char_>) -> i8>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_53(
    store_op: FnPtr<fn(Ptr<std_atomic_ref_signed_char_>, i8, i8)>,
    load_op: FnPtr<fn(Ptr<std_atomic_ref_signed_char_>) -> i8>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_ref_signed_char_>, i8, i8)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_ref_signed_char_>) -> i8>> =
        Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_54(
    store_op: FnPtr<fn(Ptr<std_atomic_ref_unsigned_char_>, u8, u8)>,
    load_op: FnPtr<fn(Ptr<std_atomic_ref_unsigned_char_>) -> u8>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_ref_unsigned_char_>, u8, u8)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_ref_unsigned_char_>) -> u8>> =
        Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_55(
    store_op: FnPtr<fn(Ptr<std_atomic_ref_short_>, i16, i16)>,
    load_op: FnPtr<fn(Ptr<std_atomic_ref_short_>) -> i16>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_ref_short_>, i16, i16)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_ref_short_>) -> i16>> =
        Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_56(
    store_op: FnPtr<fn(Ptr<std_atomic_ref_unsigned_short_>, u16, u16)>,
    load_op: FnPtr<fn(Ptr<std_atomic_ref_unsigned_short_>) -> u16>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_ref_unsigned_short_>, u16, u16)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_ref_unsigned_short_>) -> u16>> =
        Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_57(
    store_op: FnPtr<fn(Ptr<std_atomic_ref_int_>, i32, i32)>,
    load_op: FnPtr<fn(Ptr<std_atomic_ref_int_>) -> i32>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_ref_int_>, i32, i32)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_ref_int_>) -> i32>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_58(
    store_op: FnPtr<fn(Ptr<std_atomic_ref_unsigned_int_>, u32, u32)>,
    load_op: FnPtr<fn(Ptr<std_atomic_ref_unsigned_int_>) -> u32>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_ref_unsigned_int_>, u32, u32)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_ref_unsigned_int_>) -> u32>> =
        Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_59(
    store_op: FnPtr<fn(Ptr<std_atomic_ref_long_>, i64, i64)>,
    load_op: FnPtr<fn(Ptr<std_atomic_ref_long_>) -> i64>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_ref_long_>, i64, i64)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_ref_long_>) -> i64>> =
        Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_60(
    store_op: FnPtr<fn(Ptr<std_atomic_ref_unsigned_long_>, u64, u64)>,
    load_op: FnPtr<fn(Ptr<std_atomic_ref_unsigned_long_>) -> u64>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_ref_unsigned_long_>, u64, u64)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_ref_unsigned_long_>) -> u64>> =
        Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_61(
    store_op: FnPtr<fn(Ptr<std_atomic_ref_long_long_>, i64, i64)>,
    load_op: FnPtr<fn(Ptr<std_atomic_ref_long_long_>) -> i64>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_ref_long_long_>, i64, i64)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_ref_long_long_>) -> i64>> =
        Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_62(
    store_op: FnPtr<fn(Ptr<std_atomic_ref_unsigned_long_long_>, u64, u64)>,
    load_op: FnPtr<fn(Ptr<std_atomic_ref_unsigned_long_long_>) -> u64>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_ref_unsigned_long_long_>, u64, u64)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_ref_unsigned_long_long_>) -> u64>> =
        Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_63(
    store_op: FnPtr<fn(Ptr<std_atomic_ref_wchar_t_>, i32, i32)>,
    load_op: FnPtr<fn(Ptr<std_atomic_ref_wchar_t_>) -> i32>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_ref_wchar_t_>, i32, i32)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_ref_wchar_t_>) -> i32>> =
        Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_64(
    store_op: FnPtr<fn(Ptr<std_atomic_ref_char8_t_>, u8, u8)>,
    load_op: FnPtr<fn(Ptr<std_atomic_ref_char8_t_>) -> u8>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_ref_char8_t_>, u8, u8)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_ref_char8_t_>) -> u8>> =
        Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_65(
    store_op: FnPtr<fn(Ptr<std_atomic_ref_char16_t_>, u16, u16)>,
    load_op: FnPtr<fn(Ptr<std_atomic_ref_char16_t_>) -> u16>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_ref_char16_t_>, u16, u16)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_ref_char16_t_>) -> u16>> =
        Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_66(
    store_op: FnPtr<fn(Ptr<std_atomic_ref_char32_t_>, u32, u32)>,
    load_op: FnPtr<fn(Ptr<std_atomic_ref_char32_t_>) -> u32>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_ref_char32_t_>, u32, u32)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_ref_char32_t_>) -> u32>> =
        Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_67(
    store_op: FnPtr<fn(Ptr<std_atomic_ref_float_>, f32, f32)>,
    load_op: FnPtr<fn(Ptr<std_atomic_ref_float_>) -> f32>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_ref_float_>, f32, f32)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_ref_float_>) -> f32>> =
        Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_68(
    store_op: FnPtr<fn(Ptr<std_atomic_ref_double_>, f64, f64)>,
    load_op: FnPtr<fn(Ptr<std_atomic_ref_double_>) -> f64>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_ref_double_>, f64, f64)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_ref_double_>) -> f64>> =
        Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_69(
    store_op: FnPtr<fn(Ptr<std_atomic_ref_int_ptr_>, Ptr<i32>, Ptr<i32>)>,
    load_op: FnPtr<fn(Ptr<std_atomic_ref_int_ptr_>) -> Ptr<i32>>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_ref_int_ptr_>, Ptr<i32>, Ptr<i32>)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_ref_int_ptr_>) -> Ptr<i32>>> =
        Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_70(
    store_op: FnPtr<fn(Ptr<std_atomic_ref_const_int_ptr_>, Ptr<i32>, Ptr<i32>)>,
    load_op: FnPtr<fn(Ptr<std_atomic_ref_const_int_ptr_>) -> Ptr<i32>>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_ref_const_int_ptr_>, Ptr<i32>, Ptr<i32>)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_ref_const_int_ptr_>) -> Ptr<i32>>> =
        Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestDoesNotHaveFetchAdd_bool_ {}
impl Callable0<()> for TestDoesNotHaveFetchAdd_bool_ {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            TestDoesNotHaveFetchAdd_bool_Impl::operator_call(__it)
        })
    }
}
impl CppMove for TestDoesNotHaveFetchAdd_bool_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestDoesNotHaveFetchAdd_UserAtomicType_ {}
impl Callable0<()> for TestDoesNotHaveFetchAdd_UserAtomicType_ {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            TestDoesNotHaveFetchAdd_UserAtomicType_Impl::operator_call(__it)
        })
    }
}
impl CppMove for TestDoesNotHaveFetchAdd_UserAtomicType_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestDoesNotHaveFetchAdd_LargeUserAtomicType_ {}
impl Callable0<()> for TestDoesNotHaveFetchAdd_LargeUserAtomicType_ {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            TestDoesNotHaveFetchAdd_LargeUserAtomicType_Impl::operator_call(__it)
        })
    }
}
impl CppMove for TestDoesNotHaveFetchAdd_LargeUserAtomicType_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFetchAdd_float_ {}
impl Callable0<()> for TestFetchAdd_float_ {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            TestFetchAdd_float_Impl::operator_call(__it)
        })
    }
}
impl CppMove for TestFetchAdd_float_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFetchAdd_double_ {}
impl Callable0<()> for TestFetchAdd_double_ {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            TestFetchAdd_double_Impl::operator_call(__it)
        })
    }
}
impl CppMove for TestFetchAdd_double_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFetchAdd_char_ {}
impl Callable0<()> for TestFetchAdd_char_ {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            TestFetchAdd_char_Impl::operator_call(__it)
        })
    }
}
impl CppMove for TestFetchAdd_char_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFetchAdd_signed_char_ {}
impl Callable0<()> for TestFetchAdd_signed_char_ {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            TestFetchAdd_signed_char_Impl::operator_call(__it)
        })
    }
}
impl CppMove for TestFetchAdd_signed_char_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFetchAdd_unsigned_char_ {}
impl Callable0<()> for TestFetchAdd_unsigned_char_ {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            TestFetchAdd_unsigned_char_Impl::operator_call(__it)
        })
    }
}
impl CppMove for TestFetchAdd_unsigned_char_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFetchAdd_short_ {}
impl Callable0<()> for TestFetchAdd_short_ {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            TestFetchAdd_short_Impl::operator_call(__it)
        })
    }
}
impl CppMove for TestFetchAdd_short_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFetchAdd_unsigned_short_ {}
impl Callable0<()> for TestFetchAdd_unsigned_short_ {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            TestFetchAdd_unsigned_short_Impl::operator_call(__it)
        })
    }
}
impl CppMove for TestFetchAdd_unsigned_short_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFetchAdd_int_ {}
impl Callable0<()> for TestFetchAdd_int_ {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            TestFetchAdd_int_Impl::operator_call(__it)
        })
    }
}
impl CppMove for TestFetchAdd_int_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFetchAdd_unsigned_int_ {}
impl Callable0<()> for TestFetchAdd_unsigned_int_ {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            TestFetchAdd_unsigned_int_Impl::operator_call(__it)
        })
    }
}
impl CppMove for TestFetchAdd_unsigned_int_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFetchAdd_long_ {}
impl Callable0<()> for TestFetchAdd_long_ {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            TestFetchAdd_long_Impl::operator_call(__it)
        })
    }
}
impl CppMove for TestFetchAdd_long_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFetchAdd_unsigned_long_ {}
impl Callable0<()> for TestFetchAdd_unsigned_long_ {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            TestFetchAdd_unsigned_long_Impl::operator_call(__it)
        })
    }
}
impl CppMove for TestFetchAdd_unsigned_long_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFetchAdd_long_long_ {}
impl Callable0<()> for TestFetchAdd_long_long_ {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            TestFetchAdd_long_long_Impl::operator_call(__it)
        })
    }
}
impl CppMove for TestFetchAdd_long_long_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFetchAdd_unsigned_long_long_ {}
impl Callable0<()> for TestFetchAdd_unsigned_long_long_ {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            TestFetchAdd_unsigned_long_long_Impl::operator_call(__it)
        })
    }
}
impl CppMove for TestFetchAdd_unsigned_long_long_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFetchAdd_wchar_t_ {}
impl Callable0<()> for TestFetchAdd_wchar_t_ {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            TestFetchAdd_wchar_t_Impl::operator_call(__it)
        })
    }
}
impl CppMove for TestFetchAdd_wchar_t_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFetchAdd_char8_t_ {}
impl Callable0<()> for TestFetchAdd_char8_t_ {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            TestFetchAdd_char8_t_Impl::operator_call(__it)
        })
    }
}
impl CppMove for TestFetchAdd_char8_t_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFetchAdd_char16_t_ {}
impl Callable0<()> for TestFetchAdd_char16_t_ {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            TestFetchAdd_char16_t_Impl::operator_call(__it)
        })
    }
}
impl CppMove for TestFetchAdd_char16_t_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFetchAdd_char32_t_ {}
impl Callable0<()> for TestFetchAdd_char32_t_ {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            TestFetchAdd_char32_t_Impl::operator_call(__it)
        })
    }
}
impl CppMove for TestFetchAdd_char32_t_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFetchAdd_int_ptr_ {}
impl Callable0<()> for TestFetchAdd_int_ptr_ {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            TestFetchAdd_int_ptr_Impl::operator_call(__it)
        })
    }
}
impl CppMove for TestFetchAdd_int_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFetchAdd_const_int_ptr_ {}
impl Callable0<()> for TestFetchAdd_const_int_ptr_ {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            TestFetchAdd_const_int_ptr_Impl::operator_call(__it)
        })
    }
}
impl CppMove for TestFetchAdd_const_int_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> i32 {
    ({
        TestEachIntegralType_TestFetchAdd_Impl::operator_call(
            &Rc::new(RefCell::new(<TestEachIntegralType_TestFetchAdd_>::default())).as_pointer(),
        )
    });
    ({
        TestFetchAdd_float_Impl::operator_call(
            &Rc::new(RefCell::new(<TestFetchAdd_float_>::default())).as_pointer(),
        )
    });
    ({
        TestFetchAdd_double_Impl::operator_call(
            &Rc::new(RefCell::new(<TestFetchAdd_double_>::default())).as_pointer(),
        )
    });
    ({
        TestEachPointerType_TestFetchAdd_Impl::operator_call(
            &Rc::new(RefCell::new(<TestEachPointerType_TestFetchAdd_>::default())).as_pointer(),
        )
    });
    ({
        TestDoesNotHaveFetchAdd_bool_Impl::operator_call(
            &Rc::new(RefCell::new(<TestDoesNotHaveFetchAdd_bool_>::default())).as_pointer(),
        )
    });
    ({
        TestDoesNotHaveFetchAdd_UserAtomicType_Impl::operator_call(
            &Rc::new(RefCell::new(
                <TestDoesNotHaveFetchAdd_UserAtomicType_>::default(),
            ))
            .as_pointer(),
        )
    });
    ({
        TestDoesNotHaveFetchAdd_LargeUserAtomicType_Impl::operator_call(
            &Rc::new(RefCell::new(
                <TestDoesNotHaveFetchAdd_LargeUserAtomicType_>::default(),
            ))
            .as_pointer(),
        )
    });
    return 0;
}
pub trait TestDoesNotHaveFetchAdd_LargeUserAtomicType_Impl {
    fn operator_call(&self);
}
impl TestDoesNotHaveFetchAdd_LargeUserAtomicType_Impl
    for Ptr<TestDoesNotHaveFetchAdd_LargeUserAtomicType_>
{
    fn operator_call(&self) {
        const _: () = assert!(!(false), "!has_fetch_add<std::atomic_ref<T>>");;
    }
}
pub trait TestDoesNotHaveFetchAdd_UserAtomicType_Impl {
    fn operator_call(&self);
}
impl TestDoesNotHaveFetchAdd_UserAtomicType_Impl for Ptr<TestDoesNotHaveFetchAdd_UserAtomicType_> {
    fn operator_call(&self) {
        const _: () = assert!(!(false), "!has_fetch_add<std::atomic_ref<T>>");;
    }
}
pub trait TestDoesNotHaveFetchAdd_bool_Impl {
    fn operator_call(&self);
}
impl TestDoesNotHaveFetchAdd_bool_Impl for Ptr<TestDoesNotHaveFetchAdd_bool_> {
    fn operator_call(&self) {
        const _: () = assert!(!(false), "!has_fetch_add<std::atomic_ref<T>>");;
    }
}
pub trait TestEachIntegralType_TestFetchAdd_Impl {
    fn operator_call(&self);
}
impl TestEachIntegralType_TestFetchAdd_Impl for Ptr<TestEachIntegralType_TestFetchAdd_> {
    fn operator_call(&self) {
        ({
            TestFetchAdd_char_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFetchAdd_char_>::default())).as_pointer(),
            )
        });
        ({
            TestFetchAdd_signed_char_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFetchAdd_signed_char_>::default())).as_pointer(),
            )
        });
        ({
            TestFetchAdd_unsigned_char_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFetchAdd_unsigned_char_>::default())).as_pointer(),
            )
        });
        ({
            TestFetchAdd_short_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFetchAdd_short_>::default())).as_pointer(),
            )
        });
        ({
            TestFetchAdd_unsigned_short_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFetchAdd_unsigned_short_>::default())).as_pointer(),
            )
        });
        ({
            TestFetchAdd_int_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFetchAdd_int_>::default())).as_pointer(),
            )
        });
        ({
            TestFetchAdd_unsigned_int_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFetchAdd_unsigned_int_>::default())).as_pointer(),
            )
        });
        ({
            TestFetchAdd_long_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFetchAdd_long_>::default())).as_pointer(),
            )
        });
        ({
            TestFetchAdd_unsigned_long_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFetchAdd_unsigned_long_>::default())).as_pointer(),
            )
        });
        ({
            TestFetchAdd_long_long_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFetchAdd_long_long_>::default())).as_pointer(),
            )
        });
        ({
            TestFetchAdd_unsigned_long_long_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFetchAdd_unsigned_long_long_>::default())).as_pointer(),
            )
        });
        ({
            TestFetchAdd_wchar_t_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFetchAdd_wchar_t_>::default())).as_pointer(),
            )
        });
        ({
            TestFetchAdd_char8_t_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFetchAdd_char8_t_>::default())).as_pointer(),
            )
        });
        ({
            TestFetchAdd_char16_t_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFetchAdd_char16_t_>::default())).as_pointer(),
            )
        });
        ({
            TestFetchAdd_char32_t_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFetchAdd_char32_t_>::default())).as_pointer(),
            )
        });
        ({
            TestFetchAdd_signed_char_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFetchAdd_signed_char_>::default())).as_pointer(),
            )
        });
        ({
            TestFetchAdd_unsigned_char_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFetchAdd_unsigned_char_>::default())).as_pointer(),
            )
        });
        ({
            TestFetchAdd_short_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFetchAdd_short_>::default())).as_pointer(),
            )
        });
        ({
            TestFetchAdd_unsigned_short_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFetchAdd_unsigned_short_>::default())).as_pointer(),
            )
        });
        ({
            TestFetchAdd_int_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFetchAdd_int_>::default())).as_pointer(),
            )
        });
        ({
            TestFetchAdd_unsigned_int_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFetchAdd_unsigned_int_>::default())).as_pointer(),
            )
        });
        ({
            TestFetchAdd_long_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFetchAdd_long_>::default())).as_pointer(),
            )
        });
        ({
            TestFetchAdd_unsigned_long_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFetchAdd_unsigned_long_>::default())).as_pointer(),
            )
        });
    }
}
pub trait TestEachPointerType_TestFetchAdd_Impl {
    fn operator_call(&self);
}
impl TestEachPointerType_TestFetchAdd_Impl for Ptr<TestEachPointerType_TestFetchAdd_> {
    fn operator_call(&self) {
        ({
            TestFetchAdd_int_ptr_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFetchAdd_int_ptr_>::default())).as_pointer(),
            )
        });
        ({
            TestFetchAdd_const_int_ptr_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFetchAdd_const_int_ptr_>::default())).as_pointer(),
            )
        });
    }
}
pub trait TestFetchAdd_char16_t_Impl {
    fn operator_call(&self);
}
impl TestFetchAdd_char16_t_Impl for Ptr<TestFetchAdd_char16_t_> {
    fn operator_call(&self) {
        {
            let x: Value<u16> = Rc::new(RefCell::new(1_u16));
            let a: Value<std_atomic_ref_char16_t_> =
                Rc::new(RefCell::new(std_atomic_ref_char16_t_::new({
                    x.as_pointer()
                })));
            {
                let mut y: u16 = ({
                    ((*a.borrow()) as std___atomic_ref_char16_t__true__false_)
                        .fetch_add(2_u16, None)
                });
                assert!(((y as i32) == (1_u16 as i32)));
                assert!(({ (((*x.borrow()) as i32) == (3_u16 as i32)) }));
                const _: () = assert!(true, "");;
            }
            {
                let mut y: u16 = ({
                    ((*a.borrow()) as std___atomic_ref_char16_t__true__false_)
                        .fetch_add(4_u16, Some(memory_order_relaxed_71.with(|rc| *rc.borrow())))
                });
                assert!(((y as i32) == (3_u16 as i32)));
                assert!(({ (((*x.borrow()) as i32) == (7_u16 as i32)) }));
                const _: () = assert!(true, "");;
            }
        }
        {
            let fetch_add: Value<FnPtr<fn(Ptr<std_atomic_ref_char16_t_>, u16, u16)>> =
                Rc::new(RefCell::new(FnPtr::<
                    fn(Ptr<std_atomic_ref_char16_t_>, u16, u16),
                >::new(
                    |x: Ptr<std_atomic_ref_char16_t_>, mut old_val: u16, mut new_val: u16| {
                        ({
                            ((*x.upgrade().deref()) as std___atomic_ref_char16_t__true__false_)
                                .fetch_add(
                                    (((new_val as i32) - (old_val as i32)) as u16),
                                    Some(std_memory_order_release),
                                )
                        });
                    },
                )));
            let load: Value<FnPtr<fn(Ptr<std_atomic_ref_char16_t_>) -> u16>> =
                Rc::new(RefCell::new(FnPtr::<
                    fn(Ptr<std_atomic_ref_char16_t_>) -> u16,
                >::new(
                    |x: Ptr<std_atomic_ref_char16_t_>| -> u16 {
                        {
                            return ({
                                ((*x.upgrade().deref()) as std___atomic_ref_char16_t__true__false_)
                                    .load(Some(std_memory_order_acquire))
                            });
                        }
                    },
                )));
            ({
                test_acquire_release_65(
                    (*fetch_add.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
        }
        {
            let fetch_add_no_arg: Value<FnPtr<fn(Ptr<std_atomic_ref_char16_t_>, u16, u16)>> =
                Rc::new(RefCell::new(FnPtr::<
                    fn(Ptr<std_atomic_ref_char16_t_>, u16, u16),
                >::new(
                    |x: Ptr<std_atomic_ref_char16_t_>, mut old_val: u16, mut new_val: u16| {
                        ({
                            ((*x.upgrade().deref()) as std___atomic_ref_char16_t__true__false_)
                                .fetch_add((((new_val as i32) - (old_val as i32)) as u16), None)
                        });
                    },
                )));
            let fetch_add_with_order: Value<FnPtr<fn(Ptr<std_atomic_ref_char16_t_>, u16, u16)>> =
                Rc::new(RefCell::new(FnPtr::<
                    fn(Ptr<std_atomic_ref_char16_t_>, u16, u16),
                >::new(
                    |x: Ptr<std_atomic_ref_char16_t_>, mut old_val: u16, mut new_val: u16| {
                        ({
                            ((*x.upgrade().deref()) as std___atomic_ref_char16_t__true__false_)
                                .fetch_add(
                                    (((new_val as i32) - (old_val as i32)) as u16),
                                    Some(std_memory_order_seq_cst),
                                )
                        });
                    },
                )));
            let load: Value<FnPtr<fn(Ptr<std_atomic_ref_char16_t_>) -> u16>> =
                Rc::new(RefCell::new(FnPtr::<
                    fn(Ptr<std_atomic_ref_char16_t_>) -> u16,
                >::new(
                    |x: Ptr<std_atomic_ref_char16_t_>| -> u16 {
                        {
                            return ({
                                ((*x.upgrade().deref()) as std___atomic_ref_char16_t__true__false_)
                                    .load(None)
                            });
                        }
                    },
                )));
            ({
                test_seq_cst_40(
                    (*fetch_add_no_arg.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
            ({
                test_seq_cst_41(
                    (*fetch_add_with_order.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
        }
    }
}
pub trait TestFetchAdd_char32_t_Impl {
    fn operator_call(&self);
}
impl TestFetchAdd_char32_t_Impl for Ptr<TestFetchAdd_char32_t_> {
    fn operator_call(&self) {
        {
            let x: Value<u32> = Rc::new(RefCell::new(1_u32));
            let a: Value<std_atomic_ref_char32_t_> =
                Rc::new(RefCell::new(std_atomic_ref_char32_t_::new({
                    x.as_pointer()
                })));
            {
                let mut y: u32 = ({
                    ((*a.borrow()) as std___atomic_ref_char32_t__true__false_)
                        .fetch_add(2_u32, None)
                });
                assert!((y == 1_u32));
                assert!(({ ((*x.borrow()) == 3_u32) }));
                const _: () = assert!(true, "");;
            }
            {
                let mut y: u32 = ({
                    ((*a.borrow()) as std___atomic_ref_char32_t__true__false_)
                        .fetch_add(4_u32, Some(memory_order_relaxed_71.with(|rc| *rc.borrow())))
                });
                assert!((y == 3_u32));
                assert!(({ ((*x.borrow()) == 7_u32) }));
                const _: () = assert!(true, "");;
            }
        }
        {
            let fetch_add: Value<FnPtr<fn(Ptr<std_atomic_ref_char32_t_>, u32, u32)>> =
                Rc::new(RefCell::new(FnPtr::<
                    fn(Ptr<std_atomic_ref_char32_t_>, u32, u32),
                >::new(
                    |x: Ptr<std_atomic_ref_char32_t_>, mut old_val: u32, mut new_val: u32| {
                        ({
                            ((*x.upgrade().deref()) as std___atomic_ref_char32_t__true__false_)
                                .fetch_add(
                                    (new_val).wrapping_sub(old_val),
                                    Some(std_memory_order_release),
                                )
                        });
                    },
                )));
            let load: Value<FnPtr<fn(Ptr<std_atomic_ref_char32_t_>) -> u32>> =
                Rc::new(RefCell::new(FnPtr::<
                    fn(Ptr<std_atomic_ref_char32_t_>) -> u32,
                >::new(
                    |x: Ptr<std_atomic_ref_char32_t_>| -> u32 {
                        {
                            return ({
                                ((*x.upgrade().deref()) as std___atomic_ref_char32_t__true__false_)
                                    .load(Some(std_memory_order_acquire))
                            });
                        }
                    },
                )));
            ({
                test_acquire_release_66(
                    (*fetch_add.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
        }
        {
            let fetch_add_no_arg: Value<FnPtr<fn(Ptr<std_atomic_ref_char32_t_>, u32, u32)>> =
                Rc::new(RefCell::new(FnPtr::<
                    fn(Ptr<std_atomic_ref_char32_t_>, u32, u32),
                >::new(
                    |x: Ptr<std_atomic_ref_char32_t_>, mut old_val: u32, mut new_val: u32| {
                        ({
                            ((*x.upgrade().deref()) as std___atomic_ref_char32_t__true__false_)
                                .fetch_add((new_val).wrapping_sub(old_val), None)
                        });
                    },
                )));
            let fetch_add_with_order: Value<FnPtr<fn(Ptr<std_atomic_ref_char32_t_>, u32, u32)>> =
                Rc::new(RefCell::new(FnPtr::<
                    fn(Ptr<std_atomic_ref_char32_t_>, u32, u32),
                >::new(
                    |x: Ptr<std_atomic_ref_char32_t_>, mut old_val: u32, mut new_val: u32| {
                        ({
                            ((*x.upgrade().deref()) as std___atomic_ref_char32_t__true__false_)
                                .fetch_add(
                                    (new_val).wrapping_sub(old_val),
                                    Some(std_memory_order_seq_cst),
                                )
                        });
                    },
                )));
            let load: Value<FnPtr<fn(Ptr<std_atomic_ref_char32_t_>) -> u32>> =
                Rc::new(RefCell::new(FnPtr::<
                    fn(Ptr<std_atomic_ref_char32_t_>) -> u32,
                >::new(
                    |x: Ptr<std_atomic_ref_char32_t_>| -> u32 {
                        {
                            return ({
                                ((*x.upgrade().deref()) as std___atomic_ref_char32_t__true__false_)
                                    .load(None)
                            });
                        }
                    },
                )));
            ({
                test_seq_cst_42(
                    (*fetch_add_no_arg.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
            ({
                test_seq_cst_43(
                    (*fetch_add_with_order.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
        }
    }
}
pub trait TestFetchAdd_char8_t_Impl {
    fn operator_call(&self);
}
impl TestFetchAdd_char8_t_Impl for Ptr<TestFetchAdd_char8_t_> {
    fn operator_call(&self) {
        {
            let x: Value<u8> = Rc::new(RefCell::new(1_u8));
            let a: Value<std_atomic_ref_char8_t_> =
                Rc::new(RefCell::new(std_atomic_ref_char8_t_::new({
                    x.as_pointer()
                })));
            {
                let mut y: u8 = ({
                    ((*a.borrow()) as std___atomic_ref_char8_t__true__false_).fetch_add(2_u8, None)
                });
                assert!(((y as i32) == (1_u8 as i32)));
                assert!(({ (((*x.borrow()) as i32) == (3_u8 as i32)) }));
                const _: () = assert!(true, "");;
            }
            {
                let mut y: u8 = ({
                    ((*a.borrow()) as std___atomic_ref_char8_t__true__false_)
                        .fetch_add(4_u8, Some(memory_order_relaxed_71.with(|rc| *rc.borrow())))
                });
                assert!(((y as i32) == (3_u8 as i32)));
                assert!(({ (((*x.borrow()) as i32) == (7_u8 as i32)) }));
                const _: () = assert!(true, "");;
            }
        }
        {
            let fetch_add: Value<FnPtr<fn(Ptr<std_atomic_ref_char8_t_>, u8, u8)>> =
                Rc::new(RefCell::new(FnPtr::<
                    fn(Ptr<std_atomic_ref_char8_t_>, u8, u8),
                >::new(
                    |x: Ptr<std_atomic_ref_char8_t_>, mut old_val: u8, mut new_val: u8| {
                        ({
                            ((*x.upgrade().deref()) as std___atomic_ref_char8_t__true__false_)
                                .fetch_add(
                                    (((new_val as i32) - (old_val as i32)) as u8),
                                    Some(std_memory_order_release),
                                )
                        });
                    },
                )));
            let load: Value<FnPtr<fn(Ptr<std_atomic_ref_char8_t_>) -> u8>> = Rc::new(RefCell::new(
                FnPtr::<fn(Ptr<std_atomic_ref_char8_t_>) -> u8>::new(
                    |x: Ptr<std_atomic_ref_char8_t_>| -> u8 {
                        {
                            return ({
                                ((*x.upgrade().deref()) as std___atomic_ref_char8_t__true__false_)
                                    .load(Some(std_memory_order_acquire))
                            });
                        }
                    },
                ),
            ));
            ({
                test_acquire_release_64(
                    (*fetch_add.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
        }
        {
            let fetch_add_no_arg: Value<FnPtr<fn(Ptr<std_atomic_ref_char8_t_>, u8, u8)>> =
                Rc::new(RefCell::new(FnPtr::<
                    fn(Ptr<std_atomic_ref_char8_t_>, u8, u8),
                >::new(
                    |x: Ptr<std_atomic_ref_char8_t_>, mut old_val: u8, mut new_val: u8| {
                        ({
                            ((*x.upgrade().deref()) as std___atomic_ref_char8_t__true__false_)
                                .fetch_add((((new_val as i32) - (old_val as i32)) as u8), None)
                        });
                    },
                )));
            let fetch_add_with_order: Value<FnPtr<fn(Ptr<std_atomic_ref_char8_t_>, u8, u8)>> =
                Rc::new(RefCell::new(FnPtr::<
                    fn(Ptr<std_atomic_ref_char8_t_>, u8, u8),
                >::new(
                    |x: Ptr<std_atomic_ref_char8_t_>, mut old_val: u8, mut new_val: u8| {
                        ({
                            ((*x.upgrade().deref()) as std___atomic_ref_char8_t__true__false_)
                                .fetch_add(
                                    (((new_val as i32) - (old_val as i32)) as u8),
                                    Some(std_memory_order_seq_cst),
                                )
                        });
                    },
                )));
            let load: Value<FnPtr<fn(Ptr<std_atomic_ref_char8_t_>) -> u8>> = Rc::new(RefCell::new(
                FnPtr::<fn(Ptr<std_atomic_ref_char8_t_>) -> u8>::new(
                    |x: Ptr<std_atomic_ref_char8_t_>| -> u8 {
                        {
                            return ({
                                ((*x.upgrade().deref()) as std___atomic_ref_char8_t__true__false_)
                                    .load(None)
                            });
                        }
                    },
                ),
            ));
            ({
                test_seq_cst_38(
                    (*fetch_add_no_arg.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
            ({
                test_seq_cst_39(
                    (*fetch_add_with_order.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
        }
    }
}
pub trait TestFetchAdd_char_Impl {
    fn operator_call(&self);
}
impl TestFetchAdd_char_Impl for Ptr<TestFetchAdd_char_> {
    fn operator_call(&self) {
        {
            let x: Value<i8> = Rc::new(RefCell::new(1_i8));
            let a: Value<std_atomic_ref_char_> =
                Rc::new(RefCell::new(std_atomic_ref_char_::new({ x.as_pointer() })));
            {
                let mut y: i8 = ({
                    ((*a.borrow()) as std___atomic_ref_char__true__false_).fetch_add(2_i8, None)
                });
                assert!(((y as i32) == (1_i8 as i32)));
                assert!(({ (((*x.borrow()) as i32) == (3_i8 as i32)) }));
                const _: () = assert!(true, "");;
            }
            {
                let mut y: i8 = ({
                    ((*a.borrow()) as std___atomic_ref_char__true__false_)
                        .fetch_add(4_i8, Some(memory_order_relaxed_71.with(|rc| *rc.borrow())))
                });
                assert!(((y as i32) == (3_i8 as i32)));
                assert!(({ (((*x.borrow()) as i32) == (7_i8 as i32)) }));
                const _: () = assert!(true, "");;
            }
        }
        {
            let fetch_add: Value<FnPtr<fn(Ptr<std_atomic_ref_char_>, i8, i8)>> = Rc::new(
                RefCell::new(FnPtr::<fn(Ptr<std_atomic_ref_char_>, i8, i8)>::new(
                    |x: Ptr<std_atomic_ref_char_>, mut old_val: i8, mut new_val: i8| {
                        ({
                            ((*x.upgrade().deref()) as std___atomic_ref_char__true__false_)
                                .fetch_add(
                                    (((new_val as i32) - (old_val as i32)) as i8),
                                    Some(std_memory_order_release),
                                )
                        });
                    },
                )),
            );
            let load: Value<FnPtr<fn(Ptr<std_atomic_ref_char_>) -> i8>> = Rc::new(RefCell::new(
                FnPtr::<fn(Ptr<std_atomic_ref_char_>) -> i8>::new(
                    |x: Ptr<std_atomic_ref_char_>| -> i8 {
                        {
                            return ({
                                ((*x.upgrade().deref()) as std___atomic_ref_char__true__false_)
                                    .load(Some(std_memory_order_acquire))
                            });
                        }
                    },
                ),
            ));
            ({
                test_acquire_release_52(
                    (*fetch_add.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
        }
        {
            let fetch_add_no_arg: Value<FnPtr<fn(Ptr<std_atomic_ref_char_>, i8, i8)>> = Rc::new(
                RefCell::new(FnPtr::<fn(Ptr<std_atomic_ref_char_>, i8, i8)>::new(
                    |x: Ptr<std_atomic_ref_char_>, mut old_val: i8, mut new_val: i8| {
                        ({
                            ((*x.upgrade().deref()) as std___atomic_ref_char__true__false_)
                                .fetch_add((((new_val as i32) - (old_val as i32)) as i8), None)
                        });
                    },
                )),
            );
            let fetch_add_with_order: Value<FnPtr<fn(Ptr<std_atomic_ref_char_>, i8, i8)>> = Rc::new(
                RefCell::new(FnPtr::<fn(Ptr<std_atomic_ref_char_>, i8, i8)>::new(
                    |x: Ptr<std_atomic_ref_char_>, mut old_val: i8, mut new_val: i8| {
                        ({
                            ((*x.upgrade().deref()) as std___atomic_ref_char__true__false_)
                                .fetch_add(
                                    (((new_val as i32) - (old_val as i32)) as i8),
                                    Some(std_memory_order_seq_cst),
                                )
                        });
                    },
                )),
            );
            let load: Value<FnPtr<fn(Ptr<std_atomic_ref_char_>) -> i8>> = Rc::new(RefCell::new(
                FnPtr::<fn(Ptr<std_atomic_ref_char_>) -> i8>::new(
                    |x: Ptr<std_atomic_ref_char_>| -> i8 {
                        {
                            return ({
                                ((*x.upgrade().deref()) as std___atomic_ref_char__true__false_)
                                    .load(None)
                            });
                        }
                    },
                ),
            ));
            ({
                test_seq_cst_14(
                    (*fetch_add_no_arg.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
            ({
                test_seq_cst_15(
                    (*fetch_add_with_order.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
        }
    }
}
pub trait TestFetchAdd_const_int_ptr_Impl {
    fn operator_call(&self);
}
impl TestFetchAdd_const_int_ptr_Impl for Ptr<TestFetchAdd_const_int_ptr_> {
    fn operator_call(&self) {
        {
            {
                let t: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([
                    0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32,
                ])));
                let p: Value<Ptr<i32>> =
                    Rc::new(RefCell::new(((t.as_pointer() as Ptr<i32>).offset(1))));
                let a: Value<std_atomic_ref_const_int_ptr_> =
                    Rc::new(RefCell::new(std_atomic_ref_const_int_ptr_::new({
                        p.as_pointer()
                    })));
                {
                    let mut y: Ptr<i32> = ({
                        ((*a.borrow()) as std___atomic_ref_const_int_ptr__false__false_)
                            .fetch_add(2_isize, None)
                    });
                    assert!(({ (y).clone() } == { ((t.as_pointer() as Ptr<i32>).offset(1)) }));
                    assert!(
                        ({
                            (({
                                ((*a.borrow()) as std___atomic_ref_const_int_ptr__false__false_)()
                            }) == ((t.as_pointer() as Ptr<i32>).offset(3)))
                        })
                    );
                    const _: () = assert!(true, "");;
                }
                {
                    let mut y: Ptr<i32> = ({
                        ((*a.borrow()) as std___atomic_ref_const_int_ptr__false__false_).fetch_add(
                            4_isize,
                            Some(memory_order_relaxed_71.with(|rc| *rc.borrow())),
                        )
                    });
                    assert!(({ (y).clone() } == { ((t.as_pointer() as Ptr<i32>).offset(3)) }));
                    assert!(
                        ({
                            (({
                                ((*a.borrow()) as std___atomic_ref_const_int_ptr__false__false_)()
                            }) == ((t.as_pointer() as Ptr<i32>).offset(7)))
                        })
                    );
                    const _: () = assert!(true, "");;
                }
            }
        }
        {
            let fetch_add: Value<
                FnPtr<fn(Ptr<std_atomic_ref_const_int_ptr_>, Ptr<i32>, Ptr<i32>)>,
            > = Rc::new(RefCell::new(FnPtr::<
                fn(Ptr<std_atomic_ref_const_int_ptr_>, Ptr<i32>, Ptr<i32>),
            >::new(
                |x: Ptr<std_atomic_ref_const_int_ptr_>,
                 mut old_val: Ptr<i32>,
                 mut new_val: Ptr<i32>| {
                    {
                        ({
                            ((*x.upgrade().deref())
                                as std___atomic_ref_const_int_ptr__false__false_)
                                .fetch_add(
                                    (((new_val).clone() - (old_val).clone()) as i64 as isize),
                                    Some(std_memory_order_release),
                                )
                        });
                    }
                },
            )));
            let load: Value<FnPtr<fn(Ptr<std_atomic_ref_const_int_ptr_>) -> Ptr<i32>>> =
                Rc::new(RefCell::new(FnPtr::<
                    fn(Ptr<std_atomic_ref_const_int_ptr_>) -> Ptr<i32>,
                >::new(
                    |x: Ptr<std_atomic_ref_const_int_ptr_>| -> Ptr<i32> {
                        {
                            return ({
                                ((*x.upgrade().deref())
                                    as std___atomic_ref_const_int_ptr__false__false_)
                                    .load(Some(std_memory_order_acquire))
                            });
                        }
                    },
                )));
            ({
                test_acquire_release_70(
                    (*fetch_add.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
        }
        {
            let fetch_add_no_arg: Value<
                FnPtr<fn(Ptr<std_atomic_ref_const_int_ptr_>, Ptr<i32>, Ptr<i32>)>,
            > = Rc::new(RefCell::new(FnPtr::<
                fn(Ptr<std_atomic_ref_const_int_ptr_>, Ptr<i32>, Ptr<i32>),
            >::new(
                |x: Ptr<std_atomic_ref_const_int_ptr_>,
                 mut old_val: Ptr<i32>,
                 mut new_val: Ptr<i32>| {
                    {
                        ({
                            ((*x.upgrade().deref())
                                as std___atomic_ref_const_int_ptr__false__false_)
                                .fetch_add(
                                    (((new_val).clone() - (old_val).clone()) as i64 as isize),
                                    None,
                                )
                        });
                    }
                },
            )));
            let fetch_add_with_order: Value<
                FnPtr<fn(Ptr<std_atomic_ref_const_int_ptr_>, Ptr<i32>, Ptr<i32>)>,
            > = Rc::new(RefCell::new(FnPtr::<
                fn(Ptr<std_atomic_ref_const_int_ptr_>, Ptr<i32>, Ptr<i32>),
            >::new(
                |x: Ptr<std_atomic_ref_const_int_ptr_>,
                 mut old_val: Ptr<i32>,
                 mut new_val: Ptr<i32>| {
                    {
                        ({
                            ((*x.upgrade().deref())
                                as std___atomic_ref_const_int_ptr__false__false_)
                                .fetch_add(
                                    (((new_val).clone() - (old_val).clone()) as i64 as isize),
                                    Some(std_memory_order_seq_cst),
                                )
                        });
                    }
                },
            )));
            let load: Value<FnPtr<fn(Ptr<std_atomic_ref_const_int_ptr_>) -> Ptr<i32>>> =
                Rc::new(RefCell::new(FnPtr::<
                    fn(Ptr<std_atomic_ref_const_int_ptr_>) -> Ptr<i32>,
                >::new(
                    |x: Ptr<std_atomic_ref_const_int_ptr_>| -> Ptr<i32> {
                        {
                            return ({
                                ((*x.upgrade().deref())
                                    as std___atomic_ref_const_int_ptr__false__false_)
                                    .load(None)
                            });
                        }
                    },
                )));
            ({
                test_seq_cst_50(
                    (*fetch_add_no_arg.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
            ({
                test_seq_cst_51(
                    (*fetch_add_with_order.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
        }
    }
}
pub trait TestFetchAdd_double_Impl {
    fn operator_call(&self);
}
impl TestFetchAdd_double_Impl for Ptr<TestFetchAdd_double_> {
    fn operator_call(&self) {
        {
            let x: Value<f64> = Rc::new(RefCell::new(1_f64));
            let a: Value<std_atomic_ref_double_> =
                Rc::new(RefCell::new(std_atomic_ref_double_::new({
                    x.as_pointer()
                })));
            {
                let mut y: f64 = ({
                    ((*a.borrow()) as std___atomic_ref_double__false__true_).fetch_add(2_f64, None)
                });
                assert!((y == 1_f64));
                assert!(({ ((*x.borrow()) == 3_f64) }));
                const _: () = assert!(true, "");;
            }
            {
                let mut y: f64 = ({
                    ((*a.borrow()) as std___atomic_ref_double__false__true_)
                        .fetch_add(4_f64, Some(memory_order_relaxed_71.with(|rc| *rc.borrow())))
                });
                assert!((y == 3_f64));
                assert!(({ ((*x.borrow()) == 7_f64) }));
                const _: () = assert!(true, "");;
            }
        }
        {
            let fetch_add: Value<FnPtr<fn(Ptr<std_atomic_ref_double_>, f64, f64)>> =
                Rc::new(RefCell::new(FnPtr::<
                    fn(Ptr<std_atomic_ref_double_>, f64, f64),
                >::new(
                    |x: Ptr<std_atomic_ref_double_>, mut old_val: f64, mut new_val: f64| {
                        ({
                            ((*x.upgrade().deref()) as std___atomic_ref_double__false__true_)
                                .fetch_add((new_val - old_val), Some(std_memory_order_release))
                        });
                    },
                )));
            let load: Value<FnPtr<fn(Ptr<std_atomic_ref_double_>) -> f64>> = Rc::new(RefCell::new(
                FnPtr::<fn(Ptr<std_atomic_ref_double_>) -> f64>::new(
                    |x: Ptr<std_atomic_ref_double_>| -> f64 {
                        {
                            return ({
                                ((*x.upgrade().deref()) as std___atomic_ref_double__false__true_)
                                    .load(Some(std_memory_order_acquire))
                            });
                        }
                    },
                ),
            ));
            ({
                test_acquire_release_68(
                    (*fetch_add.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
        }
        {
            let fetch_add_no_arg: Value<FnPtr<fn(Ptr<std_atomic_ref_double_>, f64, f64)>> =
                Rc::new(RefCell::new(FnPtr::<
                    fn(Ptr<std_atomic_ref_double_>, f64, f64),
                >::new(
                    |x: Ptr<std_atomic_ref_double_>, mut old_val: f64, mut new_val: f64| {
                        ({
                            ((*x.upgrade().deref()) as std___atomic_ref_double__false__true_)
                                .fetch_add((new_val - old_val), None)
                        });
                    },
                )));
            let fetch_add_with_order: Value<FnPtr<fn(Ptr<std_atomic_ref_double_>, f64, f64)>> =
                Rc::new(RefCell::new(FnPtr::<
                    fn(Ptr<std_atomic_ref_double_>, f64, f64),
                >::new(
                    |x: Ptr<std_atomic_ref_double_>, mut old_val: f64, mut new_val: f64| {
                        ({
                            ((*x.upgrade().deref()) as std___atomic_ref_double__false__true_)
                                .fetch_add((new_val - old_val), Some(std_memory_order_seq_cst))
                        });
                    },
                )));
            let load: Value<FnPtr<fn(Ptr<std_atomic_ref_double_>) -> f64>> = Rc::new(RefCell::new(
                FnPtr::<fn(Ptr<std_atomic_ref_double_>) -> f64>::new(
                    |x: Ptr<std_atomic_ref_double_>| -> f64 {
                        {
                            return ({
                                ((*x.upgrade().deref()) as std___atomic_ref_double__false__true_)
                                    .load(None)
                            });
                        }
                    },
                ),
            ));
            ({
                test_seq_cst_46(
                    (*fetch_add_no_arg.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
            ({
                test_seq_cst_47(
                    (*fetch_add_with_order.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
        }
    }
}
pub trait TestFetchAdd_float_Impl {
    fn operator_call(&self);
}
impl TestFetchAdd_float_Impl for Ptr<TestFetchAdd_float_> {
    fn operator_call(&self) {
        {
            let x: Value<f32> = Rc::new(RefCell::new(1_f32));
            let a: Value<std_atomic_ref_float_> =
                Rc::new(RefCell::new(std_atomic_ref_float_::new({ x.as_pointer() })));
            {
                let mut y: f32 = ({
                    ((*a.borrow()) as std___atomic_ref_float__false__true_).fetch_add(2_f32, None)
                });
                assert!((y == 1_f32));
                assert!(({ ((*x.borrow()) == 3_f32) }));
                const _: () = assert!(true, "");;
            }
            {
                let mut y: f32 = ({
                    ((*a.borrow()) as std___atomic_ref_float__false__true_)
                        .fetch_add(4_f32, Some(memory_order_relaxed_71.with(|rc| *rc.borrow())))
                });
                assert!((y == 3_f32));
                assert!(({ ((*x.borrow()) == 7_f32) }));
                const _: () = assert!(true, "");;
            }
        }
        {
            let fetch_add: Value<FnPtr<fn(Ptr<std_atomic_ref_float_>, f32, f32)>> =
                Rc::new(RefCell::new(FnPtr::<
                    fn(Ptr<std_atomic_ref_float_>, f32, f32),
                >::new(
                    |x: Ptr<std_atomic_ref_float_>, mut old_val: f32, mut new_val: f32| {
                        ({
                            ((*x.upgrade().deref()) as std___atomic_ref_float__false__true_)
                                .fetch_add((new_val - old_val), Some(std_memory_order_release))
                        });
                    },
                )));
            let load: Value<FnPtr<fn(Ptr<std_atomic_ref_float_>) -> f32>> = Rc::new(RefCell::new(
                FnPtr::<fn(Ptr<std_atomic_ref_float_>) -> f32>::new(
                    |x: Ptr<std_atomic_ref_float_>| -> f32 {
                        {
                            return ({
                                ((*x.upgrade().deref()) as std___atomic_ref_float__false__true_)
                                    .load(Some(std_memory_order_acquire))
                            });
                        }
                    },
                ),
            ));
            ({
                test_acquire_release_67(
                    (*fetch_add.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
        }
        {
            let fetch_add_no_arg: Value<FnPtr<fn(Ptr<std_atomic_ref_float_>, f32, f32)>> =
                Rc::new(RefCell::new(FnPtr::<
                    fn(Ptr<std_atomic_ref_float_>, f32, f32),
                >::new(
                    |x: Ptr<std_atomic_ref_float_>, mut old_val: f32, mut new_val: f32| {
                        ({
                            ((*x.upgrade().deref()) as std___atomic_ref_float__false__true_)
                                .fetch_add((new_val - old_val), None)
                        });
                    },
                )));
            let fetch_add_with_order: Value<FnPtr<fn(Ptr<std_atomic_ref_float_>, f32, f32)>> =
                Rc::new(RefCell::new(FnPtr::<
                    fn(Ptr<std_atomic_ref_float_>, f32, f32),
                >::new(
                    |x: Ptr<std_atomic_ref_float_>, mut old_val: f32, mut new_val: f32| {
                        ({
                            ((*x.upgrade().deref()) as std___atomic_ref_float__false__true_)
                                .fetch_add((new_val - old_val), Some(std_memory_order_seq_cst))
                        });
                    },
                )));
            let load: Value<FnPtr<fn(Ptr<std_atomic_ref_float_>) -> f32>> = Rc::new(RefCell::new(
                FnPtr::<fn(Ptr<std_atomic_ref_float_>) -> f32>::new(
                    |x: Ptr<std_atomic_ref_float_>| -> f32 {
                        {
                            return ({
                                ((*x.upgrade().deref()) as std___atomic_ref_float__false__true_)
                                    .load(None)
                            });
                        }
                    },
                ),
            ));
            ({
                test_seq_cst_44(
                    (*fetch_add_no_arg.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
            ({
                test_seq_cst_45(
                    (*fetch_add_with_order.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
        }
    }
}
pub trait TestFetchAdd_int_Impl {
    fn operator_call(&self);
}
impl TestFetchAdd_int_Impl for Ptr<TestFetchAdd_int_> {
    fn operator_call(&self) {
        {
            let x: Value<i32> = Rc::new(RefCell::new(1));
            let a: Value<std_atomic_ref_int_> =
                Rc::new(RefCell::new(std_atomic_ref_int_::new({ x.as_pointer() })));
            {
                let mut y: i32 =
                    ({ ((*a.borrow()) as std___atomic_ref_int__true__false_).fetch_add(2, None) });
                assert!((y == 1));
                assert!(({ ((*x.borrow()) == 3) }));
                const _: () = assert!(true, "");;
            }
            {
                let mut y: i32 = ({
                    ((*a.borrow()) as std___atomic_ref_int__true__false_)
                        .fetch_add(4, Some(memory_order_relaxed_71.with(|rc| *rc.borrow())))
                });
                assert!((y == 3));
                assert!(({ ((*x.borrow()) == 7) }));
                const _: () = assert!(true, "");;
            }
        }
        {
            let fetch_add: Value<FnPtr<fn(Ptr<std_atomic_ref_int_>, i32, i32)>> = Rc::new(
                RefCell::new(FnPtr::<fn(Ptr<std_atomic_ref_int_>, i32, i32)>::new(
                    |x: Ptr<std_atomic_ref_int_>, mut old_val: i32, mut new_val: i32| {
                        ({
                            ((*x.upgrade().deref()) as std___atomic_ref_int__true__false_)
                                .fetch_add((new_val - old_val), Some(std_memory_order_release))
                        });
                    },
                )),
            );
            let load: Value<FnPtr<fn(Ptr<std_atomic_ref_int_>) -> i32>> = Rc::new(RefCell::new(
                FnPtr::<fn(Ptr<std_atomic_ref_int_>) -> i32>::new(
                    |x: Ptr<std_atomic_ref_int_>| -> i32 {
                        {
                            return ({
                                ((*x.upgrade().deref()) as std___atomic_ref_int__true__false_)
                                    .load(Some(std_memory_order_acquire))
                            });
                        }
                    },
                ),
            ));
            ({
                test_acquire_release_57(
                    (*fetch_add.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
        }
        {
            let fetch_add_no_arg: Value<FnPtr<fn(Ptr<std_atomic_ref_int_>, i32, i32)>> = Rc::new(
                RefCell::new(FnPtr::<fn(Ptr<std_atomic_ref_int_>, i32, i32)>::new(
                    |x: Ptr<std_atomic_ref_int_>, mut old_val: i32, mut new_val: i32| {
                        ({
                            ((*x.upgrade().deref()) as std___atomic_ref_int__true__false_)
                                .fetch_add((new_val - old_val), None)
                        });
                    },
                )),
            );
            let fetch_add_with_order: Value<FnPtr<fn(Ptr<std_atomic_ref_int_>, i32, i32)>> =
                Rc::new(RefCell::new(
                    FnPtr::<fn(Ptr<std_atomic_ref_int_>, i32, i32)>::new(
                        |x: Ptr<std_atomic_ref_int_>, mut old_val: i32, mut new_val: i32| {
                            ({
                                ((*x.upgrade().deref()) as std___atomic_ref_int__true__false_)
                                    .fetch_add((new_val - old_val), Some(std_memory_order_seq_cst))
                            });
                        },
                    ),
                ));
            let load: Value<FnPtr<fn(Ptr<std_atomic_ref_int_>) -> i32>> = Rc::new(RefCell::new(
                FnPtr::<fn(Ptr<std_atomic_ref_int_>) -> i32>::new(
                    |x: Ptr<std_atomic_ref_int_>| -> i32 {
                        {
                            return ({
                                ((*x.upgrade().deref()) as std___atomic_ref_int__true__false_)
                                    .load(None)
                            });
                        }
                    },
                ),
            ));
            ({
                test_seq_cst_24(
                    (*fetch_add_no_arg.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
            ({
                test_seq_cst_25(
                    (*fetch_add_with_order.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
        }
    }
}
pub trait TestFetchAdd_int_ptr_Impl {
    fn operator_call(&self);
}
impl TestFetchAdd_int_ptr_Impl for Ptr<TestFetchAdd_int_ptr_> {
    fn operator_call(&self) {
        {
            {
                let t: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([
                    0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32,
                ])));
                let p: Value<Ptr<i32>> =
                    Rc::new(RefCell::new(((t.as_pointer() as Ptr<i32>).offset(1))));
                let a: Value<std_atomic_ref_int_ptr_> =
                    Rc::new(RefCell::new(std_atomic_ref_int_ptr_::new({
                        p.as_pointer()
                    })));
                {
                    let mut y: Ptr<i32> = ({
                        ((*a.borrow()) as std___atomic_ref_int_ptr__false__false_)
                            .fetch_add(2_isize, None)
                    });
                    assert!(({ (y).clone() } == { ((t.as_pointer() as Ptr<i32>).offset(1)) }));
                    assert!(
                        ({
                            (({ ((*a.borrow()) as std___atomic_ref_int_ptr__false__false_)() })
                                == ((t.as_pointer() as Ptr<i32>).offset(3)))
                        })
                    );
                    const _: () = assert!(true, "");;
                }
                {
                    let mut y: Ptr<i32> = ({
                        ((*a.borrow()) as std___atomic_ref_int_ptr__false__false_).fetch_add(
                            4_isize,
                            Some(memory_order_relaxed_71.with(|rc| *rc.borrow())),
                        )
                    });
                    assert!(({ (y).clone() } == { ((t.as_pointer() as Ptr<i32>).offset(3)) }));
                    assert!(
                        ({
                            (({ ((*a.borrow()) as std___atomic_ref_int_ptr__false__false_)() })
                                == ((t.as_pointer() as Ptr<i32>).offset(7)))
                        })
                    );
                    const _: () = assert!(true, "");;
                }
            }
        }
        {
            let fetch_add: Value<FnPtr<fn(Ptr<std_atomic_ref_int_ptr_>, Ptr<i32>, Ptr<i32>)>> =
                Rc::new(RefCell::new(FnPtr::<
                    fn(Ptr<std_atomic_ref_int_ptr_>, Ptr<i32>, Ptr<i32>),
                >::new(
                    |x: Ptr<std_atomic_ref_int_ptr_>,
                     mut old_val: Ptr<i32>,
                     mut new_val: Ptr<i32>| {
                        {
                            ({
                                ((*x.upgrade().deref()) as std___atomic_ref_int_ptr__false__false_)
                                    .fetch_add(
                                        (((new_val).clone() - (old_val).clone()) as i64 as isize),
                                        Some(std_memory_order_release),
                                    )
                            });
                        }
                    },
                )));
            let load: Value<FnPtr<fn(Ptr<std_atomic_ref_int_ptr_>) -> Ptr<i32>>> =
                Rc::new(RefCell::new(FnPtr::<
                    fn(Ptr<std_atomic_ref_int_ptr_>) -> Ptr<i32>,
                >::new(
                    |x: Ptr<std_atomic_ref_int_ptr_>| -> Ptr<i32> {
                        {
                            return ({
                                ((*x.upgrade().deref()) as std___atomic_ref_int_ptr__false__false_)
                                    .load(Some(std_memory_order_acquire))
                            });
                        }
                    },
                )));
            ({
                test_acquire_release_69(
                    (*fetch_add.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
        }
        {
            let fetch_add_no_arg: Value<
                FnPtr<fn(Ptr<std_atomic_ref_int_ptr_>, Ptr<i32>, Ptr<i32>)>,
            > = Rc::new(RefCell::new(FnPtr::<
                fn(Ptr<std_atomic_ref_int_ptr_>, Ptr<i32>, Ptr<i32>),
            >::new(
                |x: Ptr<std_atomic_ref_int_ptr_>, mut old_val: Ptr<i32>, mut new_val: Ptr<i32>| {
                    ({
                        ((*x.upgrade().deref()) as std___atomic_ref_int_ptr__false__false_)
                            .fetch_add(
                                (((new_val).clone() - (old_val).clone()) as i64 as isize),
                                None,
                            )
                    });
                },
            )));
            let fetch_add_with_order: Value<
                FnPtr<fn(Ptr<std_atomic_ref_int_ptr_>, Ptr<i32>, Ptr<i32>)>,
            > = Rc::new(RefCell::new(FnPtr::<
                fn(Ptr<std_atomic_ref_int_ptr_>, Ptr<i32>, Ptr<i32>),
            >::new(
                |x: Ptr<std_atomic_ref_int_ptr_>, mut old_val: Ptr<i32>, mut new_val: Ptr<i32>| {
                    ({
                        ((*x.upgrade().deref()) as std___atomic_ref_int_ptr__false__false_)
                            .fetch_add(
                                (((new_val).clone() - (old_val).clone()) as i64 as isize),
                                Some(std_memory_order_seq_cst),
                            )
                    });
                },
            )));
            let load: Value<FnPtr<fn(Ptr<std_atomic_ref_int_ptr_>) -> Ptr<i32>>> =
                Rc::new(RefCell::new(FnPtr::<
                    fn(Ptr<std_atomic_ref_int_ptr_>) -> Ptr<i32>,
                >::new(
                    |x: Ptr<std_atomic_ref_int_ptr_>| -> Ptr<i32> {
                        {
                            return ({
                                ((*x.upgrade().deref()) as std___atomic_ref_int_ptr__false__false_)
                                    .load(None)
                            });
                        }
                    },
                )));
            ({
                test_seq_cst_48(
                    (*fetch_add_no_arg.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
            ({
                test_seq_cst_49(
                    (*fetch_add_with_order.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
        }
    }
}
pub trait TestFetchAdd_long_Impl {
    fn operator_call(&self);
}
impl TestFetchAdd_long_Impl for Ptr<TestFetchAdd_long_> {
    fn operator_call(&self) {
        {
            let x: Value<i64> = Rc::new(RefCell::new(1_i64));
            let a: Value<std_atomic_ref_long_> =
                Rc::new(RefCell::new(std_atomic_ref_long_::new({ x.as_pointer() })));
            {
                let mut y: i64 = ({
                    ((*a.borrow()) as std___atomic_ref_long__true__false_).fetch_add(2_i64, None)
                });
                assert!((y == 1_i64));
                assert!(({ ((*x.borrow()) == 3_i64) }));
                const _: () = assert!(true, "");;
            }
            {
                let mut y: i64 = ({
                    ((*a.borrow()) as std___atomic_ref_long__true__false_)
                        .fetch_add(4_i64, Some(memory_order_relaxed_71.with(|rc| *rc.borrow())))
                });
                assert!((y == 3_i64));
                assert!(({ ((*x.borrow()) == 7_i64) }));
                const _: () = assert!(true, "");;
            }
        }
        {
            let fetch_add: Value<FnPtr<fn(Ptr<std_atomic_ref_long_>, i64, i64)>> = Rc::new(
                RefCell::new(FnPtr::<fn(Ptr<std_atomic_ref_long_>, i64, i64)>::new(
                    |x: Ptr<std_atomic_ref_long_>, mut old_val: i64, mut new_val: i64| {
                        ({
                            ((*x.upgrade().deref()) as std___atomic_ref_long__true__false_)
                                .fetch_add((new_val - old_val), Some(std_memory_order_release))
                        });
                    },
                )),
            );
            let load: Value<FnPtr<fn(Ptr<std_atomic_ref_long_>) -> i64>> = Rc::new(RefCell::new(
                FnPtr::<fn(Ptr<std_atomic_ref_long_>) -> i64>::new(
                    |x: Ptr<std_atomic_ref_long_>| -> i64 {
                        {
                            return ({
                                ((*x.upgrade().deref()) as std___atomic_ref_long__true__false_)
                                    .load(Some(std_memory_order_acquire))
                            });
                        }
                    },
                ),
            ));
            ({
                test_acquire_release_59(
                    (*fetch_add.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
        }
        {
            let fetch_add_no_arg: Value<FnPtr<fn(Ptr<std_atomic_ref_long_>, i64, i64)>> = Rc::new(
                RefCell::new(FnPtr::<fn(Ptr<std_atomic_ref_long_>, i64, i64)>::new(
                    |x: Ptr<std_atomic_ref_long_>, mut old_val: i64, mut new_val: i64| {
                        ({
                            ((*x.upgrade().deref()) as std___atomic_ref_long__true__false_)
                                .fetch_add((new_val - old_val), None)
                        });
                    },
                )),
            );
            let fetch_add_with_order: Value<FnPtr<fn(Ptr<std_atomic_ref_long_>, i64, i64)>> =
                Rc::new(RefCell::new(
                    FnPtr::<fn(Ptr<std_atomic_ref_long_>, i64, i64)>::new(
                        |x: Ptr<std_atomic_ref_long_>, mut old_val: i64, mut new_val: i64| {
                            ({
                                ((*x.upgrade().deref()) as std___atomic_ref_long__true__false_)
                                    .fetch_add((new_val - old_val), Some(std_memory_order_seq_cst))
                            });
                        },
                    ),
                ));
            let load: Value<FnPtr<fn(Ptr<std_atomic_ref_long_>) -> i64>> = Rc::new(RefCell::new(
                FnPtr::<fn(Ptr<std_atomic_ref_long_>) -> i64>::new(
                    |x: Ptr<std_atomic_ref_long_>| -> i64 {
                        {
                            return ({
                                ((*x.upgrade().deref()) as std___atomic_ref_long__true__false_)
                                    .load(None)
                            });
                        }
                    },
                ),
            ));
            ({
                test_seq_cst_28(
                    (*fetch_add_no_arg.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
            ({
                test_seq_cst_29(
                    (*fetch_add_with_order.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
        }
    }
}
pub trait TestFetchAdd_long_long_Impl {
    fn operator_call(&self);
}
impl TestFetchAdd_long_long_Impl for Ptr<TestFetchAdd_long_long_> {
    fn operator_call(&self) {
        {
            let x: Value<i64> = Rc::new(RefCell::new(1_i64));
            let a: Value<std_atomic_ref_long_long_> =
                Rc::new(RefCell::new(std_atomic_ref_long_long_::new({
                    x.as_pointer()
                })));
            {
                let mut y: i64 = ({
                    ((*a.borrow()) as std___atomic_ref_long_long__true__false_)
                        .fetch_add(2_i64, None)
                });
                assert!((y == 1_i64));
                assert!(({ ((*x.borrow()) == 3_i64) }));
                const _: () = assert!(true, "");;
            }
            {
                let mut y: i64 = ({
                    ((*a.borrow()) as std___atomic_ref_long_long__true__false_)
                        .fetch_add(4_i64, Some(memory_order_relaxed_71.with(|rc| *rc.borrow())))
                });
                assert!((y == 3_i64));
                assert!(({ ((*x.borrow()) == 7_i64) }));
                const _: () = assert!(true, "");;
            }
        }
        {
            let fetch_add: Value<FnPtr<fn(Ptr<std_atomic_ref_long_long_>, i64, i64)>> =
                Rc::new(RefCell::new(FnPtr::<
                    fn(Ptr<std_atomic_ref_long_long_>, i64, i64),
                >::new(
                    |x: Ptr<std_atomic_ref_long_long_>, mut old_val: i64, mut new_val: i64| {
                        ({
                            ((*x.upgrade().deref()) as std___atomic_ref_long_long__true__false_)
                                .fetch_add((new_val - old_val), Some(std_memory_order_release))
                        });
                    },
                )));
            let load: Value<FnPtr<fn(Ptr<std_atomic_ref_long_long_>) -> i64>> =
                Rc::new(RefCell::new(FnPtr::<
                    fn(Ptr<std_atomic_ref_long_long_>) -> i64,
                >::new(
                    |x: Ptr<std_atomic_ref_long_long_>| -> i64 {
                        {
                            return ({
                                ((*x.upgrade().deref()) as std___atomic_ref_long_long__true__false_)
                                    .load(Some(std_memory_order_acquire))
                            });
                        }
                    },
                )));
            ({
                test_acquire_release_61(
                    (*fetch_add.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
        }
        {
            let fetch_add_no_arg: Value<FnPtr<fn(Ptr<std_atomic_ref_long_long_>, i64, i64)>> =
                Rc::new(RefCell::new(FnPtr::<
                    fn(Ptr<std_atomic_ref_long_long_>, i64, i64),
                >::new(
                    |x: Ptr<std_atomic_ref_long_long_>, mut old_val: i64, mut new_val: i64| {
                        ({
                            ((*x.upgrade().deref()) as std___atomic_ref_long_long__true__false_)
                                .fetch_add((new_val - old_val), None)
                        });
                    },
                )));
            let fetch_add_with_order: Value<FnPtr<fn(Ptr<std_atomic_ref_long_long_>, i64, i64)>> =
                Rc::new(RefCell::new(FnPtr::<
                    fn(Ptr<std_atomic_ref_long_long_>, i64, i64),
                >::new(
                    |x: Ptr<std_atomic_ref_long_long_>, mut old_val: i64, mut new_val: i64| {
                        ({
                            ((*x.upgrade().deref()) as std___atomic_ref_long_long__true__false_)
                                .fetch_add((new_val - old_val), Some(std_memory_order_seq_cst))
                        });
                    },
                )));
            let load: Value<FnPtr<fn(Ptr<std_atomic_ref_long_long_>) -> i64>> =
                Rc::new(RefCell::new(FnPtr::<
                    fn(Ptr<std_atomic_ref_long_long_>) -> i64,
                >::new(
                    |x: Ptr<std_atomic_ref_long_long_>| -> i64 {
                        {
                            return ({
                                ((*x.upgrade().deref()) as std___atomic_ref_long_long__true__false_)
                                    .load(None)
                            });
                        }
                    },
                )));
            ({
                test_seq_cst_32(
                    (*fetch_add_no_arg.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
            ({
                test_seq_cst_33(
                    (*fetch_add_with_order.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
        }
    }
}
pub trait TestFetchAdd_short_Impl {
    fn operator_call(&self);
}
impl TestFetchAdd_short_Impl for Ptr<TestFetchAdd_short_> {
    fn operator_call(&self) {
        {
            let x: Value<i16> = Rc::new(RefCell::new(1_i16));
            let a: Value<std_atomic_ref_short_> =
                Rc::new(RefCell::new(std_atomic_ref_short_::new({ x.as_pointer() })));
            {
                let mut y: i16 = ({
                    ((*a.borrow()) as std___atomic_ref_short__true__false_).fetch_add(2_i16, None)
                });
                assert!(((y as i32) == (1_i16 as i32)));
                assert!(({ (((*x.borrow()) as i32) == (3_i16 as i32)) }));
                const _: () = assert!(true, "");;
            }
            {
                let mut y: i16 = ({
                    ((*a.borrow()) as std___atomic_ref_short__true__false_)
                        .fetch_add(4_i16, Some(memory_order_relaxed_71.with(|rc| *rc.borrow())))
                });
                assert!(((y as i32) == (3_i16 as i32)));
                assert!(({ (((*x.borrow()) as i32) == (7_i16 as i32)) }));
                const _: () = assert!(true, "");;
            }
        }
        {
            let fetch_add: Value<FnPtr<fn(Ptr<std_atomic_ref_short_>, i16, i16)>> =
                Rc::new(RefCell::new(FnPtr::<
                    fn(Ptr<std_atomic_ref_short_>, i16, i16),
                >::new(
                    |x: Ptr<std_atomic_ref_short_>, mut old_val: i16, mut new_val: i16| {
                        ({
                            ((*x.upgrade().deref()) as std___atomic_ref_short__true__false_)
                                .fetch_add(
                                    (((new_val as i32) - (old_val as i32)) as i16),
                                    Some(std_memory_order_release),
                                )
                        });
                    },
                )));
            let load: Value<FnPtr<fn(Ptr<std_atomic_ref_short_>) -> i16>> = Rc::new(RefCell::new(
                FnPtr::<fn(Ptr<std_atomic_ref_short_>) -> i16>::new(
                    |x: Ptr<std_atomic_ref_short_>| -> i16 {
                        {
                            return ({
                                ((*x.upgrade().deref()) as std___atomic_ref_short__true__false_)
                                    .load(Some(std_memory_order_acquire))
                            });
                        }
                    },
                ),
            ));
            ({
                test_acquire_release_55(
                    (*fetch_add.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
        }
        {
            let fetch_add_no_arg: Value<FnPtr<fn(Ptr<std_atomic_ref_short_>, i16, i16)>> =
                Rc::new(RefCell::new(FnPtr::<
                    fn(Ptr<std_atomic_ref_short_>, i16, i16),
                >::new(
                    |x: Ptr<std_atomic_ref_short_>, mut old_val: i16, mut new_val: i16| {
                        ({
                            ((*x.upgrade().deref()) as std___atomic_ref_short__true__false_)
                                .fetch_add((((new_val as i32) - (old_val as i32)) as i16), None)
                        });
                    },
                )));
            let fetch_add_with_order: Value<FnPtr<fn(Ptr<std_atomic_ref_short_>, i16, i16)>> =
                Rc::new(RefCell::new(FnPtr::<
                    fn(Ptr<std_atomic_ref_short_>, i16, i16),
                >::new(
                    |x: Ptr<std_atomic_ref_short_>, mut old_val: i16, mut new_val: i16| {
                        ({
                            ((*x.upgrade().deref()) as std___atomic_ref_short__true__false_)
                                .fetch_add(
                                    (((new_val as i32) - (old_val as i32)) as i16),
                                    Some(std_memory_order_seq_cst),
                                )
                        });
                    },
                )));
            let load: Value<FnPtr<fn(Ptr<std_atomic_ref_short_>) -> i16>> = Rc::new(RefCell::new(
                FnPtr::<fn(Ptr<std_atomic_ref_short_>) -> i16>::new(
                    |x: Ptr<std_atomic_ref_short_>| -> i16 {
                        {
                            return ({
                                ((*x.upgrade().deref()) as std___atomic_ref_short__true__false_)
                                    .load(None)
                            });
                        }
                    },
                ),
            ));
            ({
                test_seq_cst_20(
                    (*fetch_add_no_arg.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
            ({
                test_seq_cst_21(
                    (*fetch_add_with_order.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
        }
    }
}
pub trait TestFetchAdd_signed_char_Impl {
    fn operator_call(&self);
}
impl TestFetchAdd_signed_char_Impl for Ptr<TestFetchAdd_signed_char_> {
    fn operator_call(&self) {
        {
            let x: Value<i8> = Rc::new(RefCell::new(1_i8));
            let a: Value<std_atomic_ref_signed_char_> =
                Rc::new(RefCell::new(std_atomic_ref_signed_char_::new({
                    x.as_pointer()
                })));
            {
                let mut y: i8 = ({
                    ((*a.borrow()) as std___atomic_ref_signed_char__true__false_)
                        .fetch_add(2_i8, None)
                });
                assert!(((y as i32) == (1_i8 as i32)));
                assert!(({ (((*x.borrow()) as i32) == (3_i8 as i32)) }));
                const _: () = assert!(true, "");;
            }
            {
                let mut y: i8 = ({
                    ((*a.borrow()) as std___atomic_ref_signed_char__true__false_)
                        .fetch_add(4_i8, Some(memory_order_relaxed_71.with(|rc| *rc.borrow())))
                });
                assert!(((y as i32) == (3_i8 as i32)));
                assert!(({ (((*x.borrow()) as i32) == (7_i8 as i32)) }));
                const _: () = assert!(true, "");;
            }
        }
        {
            let fetch_add: Value<FnPtr<fn(Ptr<std_atomic_ref_signed_char_>, i8, i8)>> =
                Rc::new(RefCell::new(FnPtr::<
                    fn(Ptr<std_atomic_ref_signed_char_>, i8, i8),
                >::new(
                    |x: Ptr<std_atomic_ref_signed_char_>, mut old_val: i8, mut new_val: i8| {
                        ({
                            ((*x.upgrade().deref()) as std___atomic_ref_signed_char__true__false_)
                                .fetch_add(
                                    (((new_val as i32) - (old_val as i32)) as i8),
                                    Some(std_memory_order_release),
                                )
                        });
                    },
                )));
            let load: Value<FnPtr<fn(Ptr<std_atomic_ref_signed_char_>) -> i8>> =
                Rc::new(RefCell::new(FnPtr::<
                    fn(Ptr<std_atomic_ref_signed_char_>) -> i8,
                >::new(
                    |x: Ptr<std_atomic_ref_signed_char_>| -> i8 {
                        {
                            return ({
                                ((*x.upgrade().deref())
                                    as std___atomic_ref_signed_char__true__false_)
                                    .load(Some(std_memory_order_acquire))
                            });
                        }
                    },
                )));
            ({
                test_acquire_release_53(
                    (*fetch_add.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
        }
        {
            let fetch_add_no_arg: Value<FnPtr<fn(Ptr<std_atomic_ref_signed_char_>, i8, i8)>> =
                Rc::new(RefCell::new(FnPtr::<
                    fn(Ptr<std_atomic_ref_signed_char_>, i8, i8),
                >::new(
                    |x: Ptr<std_atomic_ref_signed_char_>, mut old_val: i8, mut new_val: i8| {
                        ({
                            ((*x.upgrade().deref()) as std___atomic_ref_signed_char__true__false_)
                                .fetch_add((((new_val as i32) - (old_val as i32)) as i8), None)
                        });
                    },
                )));
            let fetch_add_with_order: Value<FnPtr<fn(Ptr<std_atomic_ref_signed_char_>, i8, i8)>> =
                Rc::new(RefCell::new(FnPtr::<
                    fn(Ptr<std_atomic_ref_signed_char_>, i8, i8),
                >::new(
                    |x: Ptr<std_atomic_ref_signed_char_>, mut old_val: i8, mut new_val: i8| {
                        ({
                            ((*x.upgrade().deref()) as std___atomic_ref_signed_char__true__false_)
                                .fetch_add(
                                    (((new_val as i32) - (old_val as i32)) as i8),
                                    Some(std_memory_order_seq_cst),
                                )
                        });
                    },
                )));
            let load: Value<FnPtr<fn(Ptr<std_atomic_ref_signed_char_>) -> i8>> =
                Rc::new(RefCell::new(FnPtr::<
                    fn(Ptr<std_atomic_ref_signed_char_>) -> i8,
                >::new(
                    |x: Ptr<std_atomic_ref_signed_char_>| -> i8 {
                        {
                            return ({
                                ((*x.upgrade().deref())
                                    as std___atomic_ref_signed_char__true__false_)
                                    .load(None)
                            });
                        }
                    },
                )));
            ({
                test_seq_cst_16(
                    (*fetch_add_no_arg.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
            ({
                test_seq_cst_17(
                    (*fetch_add_with_order.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
        }
    }
}
pub trait TestFetchAdd_unsigned_char_Impl {
    fn operator_call(&self);
}
impl TestFetchAdd_unsigned_char_Impl for Ptr<TestFetchAdd_unsigned_char_> {
    fn operator_call(&self) {
        {
            let x: Value<u8> = Rc::new(RefCell::new(1_u8));
            let a: Value<std_atomic_ref_unsigned_char_> =
                Rc::new(RefCell::new(std_atomic_ref_unsigned_char_::new({
                    x.as_pointer()
                })));
            {
                let mut y: u8 = ({
                    ((*a.borrow()) as std___atomic_ref_unsigned_char__true__false_)
                        .fetch_add(2_u8, None)
                });
                assert!(((y as i32) == (1_u8 as i32)));
                assert!(({ (((*x.borrow()) as i32) == (3_u8 as i32)) }));
                const _: () = assert!(true, "");;
            }
            {
                let mut y: u8 = ({
                    ((*a.borrow()) as std___atomic_ref_unsigned_char__true__false_)
                        .fetch_add(4_u8, Some(memory_order_relaxed_71.with(|rc| *rc.borrow())))
                });
                assert!(((y as i32) == (3_u8 as i32)));
                assert!(({ (((*x.borrow()) as i32) == (7_u8 as i32)) }));
                const _: () = assert!(true, "");;
            }
        }
        {
            let fetch_add: Value<FnPtr<fn(Ptr<std_atomic_ref_unsigned_char_>, u8, u8)>> =
                Rc::new(RefCell::new(FnPtr::<
                    fn(Ptr<std_atomic_ref_unsigned_char_>, u8, u8),
                >::new(
                    |x: Ptr<std_atomic_ref_unsigned_char_>, mut old_val: u8, mut new_val: u8| {
                        ({
                            ((*x.upgrade().deref()) as std___atomic_ref_unsigned_char__true__false_)
                                .fetch_add(
                                    (((new_val as i32) - (old_val as i32)) as u8),
                                    Some(std_memory_order_release),
                                )
                        });
                    },
                )));
            let load: Value<FnPtr<fn(Ptr<std_atomic_ref_unsigned_char_>) -> u8>> =
                Rc::new(RefCell::new(FnPtr::<
                    fn(Ptr<std_atomic_ref_unsigned_char_>) -> u8,
                >::new(
                    |x: Ptr<std_atomic_ref_unsigned_char_>| -> u8 {
                        {
                            return ({
                                ((*x.upgrade().deref())
                                    as std___atomic_ref_unsigned_char__true__false_)
                                    .load(Some(std_memory_order_acquire))
                            });
                        }
                    },
                )));
            ({
                test_acquire_release_54(
                    (*fetch_add.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
        }
        {
            let fetch_add_no_arg: Value<FnPtr<fn(Ptr<std_atomic_ref_unsigned_char_>, u8, u8)>> =
                Rc::new(RefCell::new(FnPtr::<
                    fn(Ptr<std_atomic_ref_unsigned_char_>, u8, u8),
                >::new(
                    |x: Ptr<std_atomic_ref_unsigned_char_>, mut old_val: u8, mut new_val: u8| {
                        ({
                            ((*x.upgrade().deref()) as std___atomic_ref_unsigned_char__true__false_)
                                .fetch_add((((new_val as i32) - (old_val as i32)) as u8), None)
                        });
                    },
                )));
            let fetch_add_with_order: Value<FnPtr<fn(Ptr<std_atomic_ref_unsigned_char_>, u8, u8)>> =
                Rc::new(RefCell::new(FnPtr::<
                    fn(Ptr<std_atomic_ref_unsigned_char_>, u8, u8),
                >::new(
                    |x: Ptr<std_atomic_ref_unsigned_char_>, mut old_val: u8, mut new_val: u8| {
                        ({
                            ((*x.upgrade().deref()) as std___atomic_ref_unsigned_char__true__false_)
                                .fetch_add(
                                    (((new_val as i32) - (old_val as i32)) as u8),
                                    Some(std_memory_order_seq_cst),
                                )
                        });
                    },
                )));
            let load: Value<FnPtr<fn(Ptr<std_atomic_ref_unsigned_char_>) -> u8>> =
                Rc::new(RefCell::new(FnPtr::<
                    fn(Ptr<std_atomic_ref_unsigned_char_>) -> u8,
                >::new(
                    |x: Ptr<std_atomic_ref_unsigned_char_>| -> u8 {
                        {
                            return ({
                                ((*x.upgrade().deref())
                                    as std___atomic_ref_unsigned_char__true__false_)
                                    .load(None)
                            });
                        }
                    },
                )));
            ({
                test_seq_cst_18(
                    (*fetch_add_no_arg.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
            ({
                test_seq_cst_19(
                    (*fetch_add_with_order.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
        }
    }
}
pub trait TestFetchAdd_unsigned_int_Impl {
    fn operator_call(&self);
}
impl TestFetchAdd_unsigned_int_Impl for Ptr<TestFetchAdd_unsigned_int_> {
    fn operator_call(&self) {
        {
            let x: Value<u32> = Rc::new(RefCell::new(1_u32));
            let a: Value<std_atomic_ref_unsigned_int_> =
                Rc::new(RefCell::new(std_atomic_ref_unsigned_int_::new({
                    x.as_pointer()
                })));
            {
                let mut y: u32 = ({
                    ((*a.borrow()) as std___atomic_ref_unsigned_int__true__false_)
                        .fetch_add(2_u32, None)
                });
                assert!((y == 1_u32));
                assert!(({ ((*x.borrow()) == 3_u32) }));
                const _: () = assert!(true, "");;
            }
            {
                let mut y: u32 = ({
                    ((*a.borrow()) as std___atomic_ref_unsigned_int__true__false_)
                        .fetch_add(4_u32, Some(memory_order_relaxed_71.with(|rc| *rc.borrow())))
                });
                assert!((y == 3_u32));
                assert!(({ ((*x.borrow()) == 7_u32) }));
                const _: () = assert!(true, "");;
            }
        }
        {
            let fetch_add: Value<FnPtr<fn(Ptr<std_atomic_ref_unsigned_int_>, u32, u32)>> =
                Rc::new(RefCell::new(FnPtr::<
                    fn(Ptr<std_atomic_ref_unsigned_int_>, u32, u32),
                >::new(
                    |x: Ptr<std_atomic_ref_unsigned_int_>, mut old_val: u32, mut new_val: u32| {
                        ({
                            ((*x.upgrade().deref()) as std___atomic_ref_unsigned_int__true__false_)
                                .fetch_add(
                                    (new_val).wrapping_sub(old_val),
                                    Some(std_memory_order_release),
                                )
                        });
                    },
                )));
            let load: Value<FnPtr<fn(Ptr<std_atomic_ref_unsigned_int_>) -> u32>> =
                Rc::new(RefCell::new(FnPtr::<
                    fn(Ptr<std_atomic_ref_unsigned_int_>) -> u32,
                >::new(
                    |x: Ptr<std_atomic_ref_unsigned_int_>| -> u32 {
                        {
                            return ({
                                ((*x.upgrade().deref())
                                    as std___atomic_ref_unsigned_int__true__false_)
                                    .load(Some(std_memory_order_acquire))
                            });
                        }
                    },
                )));
            ({
                test_acquire_release_58(
                    (*fetch_add.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
        }
        {
            let fetch_add_no_arg: Value<FnPtr<fn(Ptr<std_atomic_ref_unsigned_int_>, u32, u32)>> =
                Rc::new(RefCell::new(FnPtr::<
                    fn(Ptr<std_atomic_ref_unsigned_int_>, u32, u32),
                >::new(
                    |x: Ptr<std_atomic_ref_unsigned_int_>, mut old_val: u32, mut new_val: u32| {
                        ({
                            ((*x.upgrade().deref()) as std___atomic_ref_unsigned_int__true__false_)
                                .fetch_add((new_val).wrapping_sub(old_val), None)
                        });
                    },
                )));
            let fetch_add_with_order: Value<
                FnPtr<fn(Ptr<std_atomic_ref_unsigned_int_>, u32, u32)>,
            > = Rc::new(RefCell::new(FnPtr::<
                fn(Ptr<std_atomic_ref_unsigned_int_>, u32, u32),
            >::new(
                |x: Ptr<std_atomic_ref_unsigned_int_>, mut old_val: u32, mut new_val: u32| {
                    ({
                        ((*x.upgrade().deref()) as std___atomic_ref_unsigned_int__true__false_)
                            .fetch_add(
                                (new_val).wrapping_sub(old_val),
                                Some(std_memory_order_seq_cst),
                            )
                    });
                },
            )));
            let load: Value<FnPtr<fn(Ptr<std_atomic_ref_unsigned_int_>) -> u32>> =
                Rc::new(RefCell::new(FnPtr::<
                    fn(Ptr<std_atomic_ref_unsigned_int_>) -> u32,
                >::new(
                    |x: Ptr<std_atomic_ref_unsigned_int_>| -> u32 {
                        {
                            return ({
                                ((*x.upgrade().deref())
                                    as std___atomic_ref_unsigned_int__true__false_)
                                    .load(None)
                            });
                        }
                    },
                )));
            ({
                test_seq_cst_26(
                    (*fetch_add_no_arg.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
            ({
                test_seq_cst_27(
                    (*fetch_add_with_order.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
        }
    }
}
pub trait TestFetchAdd_unsigned_long_Impl {
    fn operator_call(&self);
}
impl TestFetchAdd_unsigned_long_Impl for Ptr<TestFetchAdd_unsigned_long_> {
    fn operator_call(&self) {
        {
            let x: Value<u64> = Rc::new(RefCell::new(1_u64));
            let a: Value<std_atomic_ref_unsigned_long_> =
                Rc::new(RefCell::new(std_atomic_ref_unsigned_long_::new({
                    x.as_pointer()
                })));
            {
                let mut y: u64 = ({
                    ((*a.borrow()) as std___atomic_ref_unsigned_long__true__false_)
                        .fetch_add(2_u64, None)
                });
                assert!((y == 1_u64));
                assert!(({ ((*x.borrow()) == 3_u64) }));
                const _: () = assert!(true, "");;
            }
            {
                let mut y: u64 = ({
                    ((*a.borrow()) as std___atomic_ref_unsigned_long__true__false_)
                        .fetch_add(4_u64, Some(memory_order_relaxed_71.with(|rc| *rc.borrow())))
                });
                assert!((y == 3_u64));
                assert!(({ ((*x.borrow()) == 7_u64) }));
                const _: () = assert!(true, "");;
            }
        }
        {
            let fetch_add: Value<FnPtr<fn(Ptr<std_atomic_ref_unsigned_long_>, u64, u64)>> =
                Rc::new(RefCell::new(FnPtr::<
                    fn(Ptr<std_atomic_ref_unsigned_long_>, u64, u64),
                >::new(
                    |x: Ptr<std_atomic_ref_unsigned_long_>, mut old_val: u64, mut new_val: u64| {
                        ({
                            ((*x.upgrade().deref()) as std___atomic_ref_unsigned_long__true__false_)
                                .fetch_add(
                                    (new_val).wrapping_sub(old_val),
                                    Some(std_memory_order_release),
                                )
                        });
                    },
                )));
            let load: Value<FnPtr<fn(Ptr<std_atomic_ref_unsigned_long_>) -> u64>> =
                Rc::new(RefCell::new(FnPtr::<
                    fn(Ptr<std_atomic_ref_unsigned_long_>) -> u64,
                >::new(
                    |x: Ptr<std_atomic_ref_unsigned_long_>| -> u64 {
                        {
                            return ({
                                ((*x.upgrade().deref())
                                    as std___atomic_ref_unsigned_long__true__false_)
                                    .load(Some(std_memory_order_acquire))
                            });
                        }
                    },
                )));
            ({
                test_acquire_release_60(
                    (*fetch_add.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
        }
        {
            let fetch_add_no_arg: Value<FnPtr<fn(Ptr<std_atomic_ref_unsigned_long_>, u64, u64)>> =
                Rc::new(RefCell::new(FnPtr::<
                    fn(Ptr<std_atomic_ref_unsigned_long_>, u64, u64),
                >::new(
                    |x: Ptr<std_atomic_ref_unsigned_long_>, mut old_val: u64, mut new_val: u64| {
                        ({
                            ((*x.upgrade().deref()) as std___atomic_ref_unsigned_long__true__false_)
                                .fetch_add((new_val).wrapping_sub(old_val), None)
                        });
                    },
                )));
            let fetch_add_with_order: Value<
                FnPtr<fn(Ptr<std_atomic_ref_unsigned_long_>, u64, u64)>,
            > = Rc::new(RefCell::new(FnPtr::<
                fn(Ptr<std_atomic_ref_unsigned_long_>, u64, u64),
            >::new(
                |x: Ptr<std_atomic_ref_unsigned_long_>, mut old_val: u64, mut new_val: u64| {
                    ({
                        ((*x.upgrade().deref()) as std___atomic_ref_unsigned_long__true__false_)
                            .fetch_add(
                                (new_val).wrapping_sub(old_val),
                                Some(std_memory_order_seq_cst),
                            )
                    });
                },
            )));
            let load: Value<FnPtr<fn(Ptr<std_atomic_ref_unsigned_long_>) -> u64>> =
                Rc::new(RefCell::new(FnPtr::<
                    fn(Ptr<std_atomic_ref_unsigned_long_>) -> u64,
                >::new(
                    |x: Ptr<std_atomic_ref_unsigned_long_>| -> u64 {
                        {
                            return ({
                                ((*x.upgrade().deref())
                                    as std___atomic_ref_unsigned_long__true__false_)
                                    .load(None)
                            });
                        }
                    },
                )));
            ({
                test_seq_cst_30(
                    (*fetch_add_no_arg.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
            ({
                test_seq_cst_31(
                    (*fetch_add_with_order.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
        }
    }
}
pub trait TestFetchAdd_unsigned_long_long_Impl {
    fn operator_call(&self);
}
impl TestFetchAdd_unsigned_long_long_Impl for Ptr<TestFetchAdd_unsigned_long_long_> {
    fn operator_call(&self) {
        {
            let x: Value<u64> = Rc::new(RefCell::new(1_u64));
            let a: Value<std_atomic_ref_unsigned_long_long_> =
                Rc::new(RefCell::new(std_atomic_ref_unsigned_long_long_::new({
                    x.as_pointer()
                })));
            {
                let mut y: u64 = ({
                    ((*a.borrow()) as std___atomic_ref_unsigned_long_long__true__false_)
                        .fetch_add(2_u64, None)
                });
                assert!((y == 1_u64));
                assert!(({ ((*x.borrow()) == 3_u64) }));
                const _: () = assert!(true, "");;
            }
            {
                let mut y: u64 = ({
                    ((*a.borrow()) as std___atomic_ref_unsigned_long_long__true__false_)
                        .fetch_add(4_u64, Some(memory_order_relaxed_71.with(|rc| *rc.borrow())))
                });
                assert!((y == 3_u64));
                assert!(({ ((*x.borrow()) == 7_u64) }));
                const _: () = assert!(true, "");;
            }
        }
        {
            let fetch_add: Value<FnPtr<fn(Ptr<std_atomic_ref_unsigned_long_long_>, u64, u64)>> =
                Rc::new(RefCell::new(FnPtr::<
                    fn(Ptr<std_atomic_ref_unsigned_long_long_>, u64, u64),
                >::new(
                    |x: Ptr<std_atomic_ref_unsigned_long_long_>,
                     mut old_val: u64,
                     mut new_val: u64| {
                        {
                            ({
                                ((*x.upgrade().deref())
                                    as std___atomic_ref_unsigned_long_long__true__false_)
                                    .fetch_add(
                                        (new_val).wrapping_sub(old_val),
                                        Some(std_memory_order_release),
                                    )
                            });
                        }
                    },
                )));
            let load: Value<FnPtr<fn(Ptr<std_atomic_ref_unsigned_long_long_>) -> u64>> =
                Rc::new(RefCell::new(FnPtr::<
                    fn(Ptr<std_atomic_ref_unsigned_long_long_>) -> u64,
                >::new(
                    |x: Ptr<std_atomic_ref_unsigned_long_long_>| -> u64 {
                        {
                            return ({
                                ((*x.upgrade().deref())
                                    as std___atomic_ref_unsigned_long_long__true__false_)
                                    .load(Some(std_memory_order_acquire))
                            });
                        }
                    },
                )));
            ({
                test_acquire_release_62(
                    (*fetch_add.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
        }
        {
            let fetch_add_no_arg: Value<
                FnPtr<fn(Ptr<std_atomic_ref_unsigned_long_long_>, u64, u64)>,
            > = Rc::new(RefCell::new(FnPtr::<
                fn(Ptr<std_atomic_ref_unsigned_long_long_>, u64, u64),
            >::new(
                |x: Ptr<std_atomic_ref_unsigned_long_long_>, mut old_val: u64, mut new_val: u64| {
                    ({
                        ((*x.upgrade().deref())
                            as std___atomic_ref_unsigned_long_long__true__false_)
                            .fetch_add((new_val).wrapping_sub(old_val), None)
                    });
                },
            )));
            let fetch_add_with_order: Value<
                FnPtr<fn(Ptr<std_atomic_ref_unsigned_long_long_>, u64, u64)>,
            > = Rc::new(RefCell::new(FnPtr::<
                fn(Ptr<std_atomic_ref_unsigned_long_long_>, u64, u64),
            >::new(
                |x: Ptr<std_atomic_ref_unsigned_long_long_>, mut old_val: u64, mut new_val: u64| {
                    ({
                        ((*x.upgrade().deref())
                            as std___atomic_ref_unsigned_long_long__true__false_)
                            .fetch_add(
                                (new_val).wrapping_sub(old_val),
                                Some(std_memory_order_seq_cst),
                            )
                    });
                },
            )));
            let load: Value<FnPtr<fn(Ptr<std_atomic_ref_unsigned_long_long_>) -> u64>> =
                Rc::new(RefCell::new(FnPtr::<
                    fn(Ptr<std_atomic_ref_unsigned_long_long_>) -> u64,
                >::new(
                    |x: Ptr<std_atomic_ref_unsigned_long_long_>| -> u64 {
                        {
                            return ({
                                ((*x.upgrade().deref())
                                    as std___atomic_ref_unsigned_long_long__true__false_)
                                    .load(None)
                            });
                        }
                    },
                )));
            ({
                test_seq_cst_34(
                    (*fetch_add_no_arg.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
            ({
                test_seq_cst_35(
                    (*fetch_add_with_order.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
        }
    }
}
pub trait TestFetchAdd_unsigned_short_Impl {
    fn operator_call(&self);
}
impl TestFetchAdd_unsigned_short_Impl for Ptr<TestFetchAdd_unsigned_short_> {
    fn operator_call(&self) {
        {
            let x: Value<u16> = Rc::new(RefCell::new(1_u16));
            let a: Value<std_atomic_ref_unsigned_short_> =
                Rc::new(RefCell::new(std_atomic_ref_unsigned_short_::new({
                    x.as_pointer()
                })));
            {
                let mut y: u16 = ({
                    ((*a.borrow()) as std___atomic_ref_unsigned_short__true__false_)
                        .fetch_add(2_u16, None)
                });
                assert!(((y as i32) == (1_u16 as i32)));
                assert!(({ (((*x.borrow()) as i32) == (3_u16 as i32)) }));
                const _: () = assert!(true, "");;
            }
            {
                let mut y: u16 = ({
                    ((*a.borrow()) as std___atomic_ref_unsigned_short__true__false_)
                        .fetch_add(4_u16, Some(memory_order_relaxed_71.with(|rc| *rc.borrow())))
                });
                assert!(((y as i32) == (3_u16 as i32)));
                assert!(({ (((*x.borrow()) as i32) == (7_u16 as i32)) }));
                const _: () = assert!(true, "");;
            }
        }
        {
            let fetch_add: Value<FnPtr<fn(Ptr<std_atomic_ref_unsigned_short_>, u16, u16)>> =
                Rc::new(RefCell::new(FnPtr::<
                    fn(Ptr<std_atomic_ref_unsigned_short_>, u16, u16),
                >::new(
                    |x: Ptr<std_atomic_ref_unsigned_short_>, mut old_val: u16, mut new_val: u16| {
                        ({
                            ((*x.upgrade().deref())
                                as std___atomic_ref_unsigned_short__true__false_)
                                .fetch_add(
                                    (((new_val as i32) - (old_val as i32)) as u16),
                                    Some(std_memory_order_release),
                                )
                        });
                    },
                )));
            let load: Value<FnPtr<fn(Ptr<std_atomic_ref_unsigned_short_>) -> u16>> =
                Rc::new(RefCell::new(FnPtr::<
                    fn(Ptr<std_atomic_ref_unsigned_short_>) -> u16,
                >::new(
                    |x: Ptr<std_atomic_ref_unsigned_short_>| -> u16 {
                        {
                            return ({
                                ((*x.upgrade().deref())
                                    as std___atomic_ref_unsigned_short__true__false_)
                                    .load(Some(std_memory_order_acquire))
                            });
                        }
                    },
                )));
            ({
                test_acquire_release_56(
                    (*fetch_add.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
        }
        {
            let fetch_add_no_arg: Value<FnPtr<fn(Ptr<std_atomic_ref_unsigned_short_>, u16, u16)>> =
                Rc::new(RefCell::new(FnPtr::<
                    fn(Ptr<std_atomic_ref_unsigned_short_>, u16, u16),
                >::new(
                    |x: Ptr<std_atomic_ref_unsigned_short_>, mut old_val: u16, mut new_val: u16| {
                        ({
                            ((*x.upgrade().deref())
                                as std___atomic_ref_unsigned_short__true__false_)
                                .fetch_add((((new_val as i32) - (old_val as i32)) as u16), None)
                        });
                    },
                )));
            let fetch_add_with_order: Value<
                FnPtr<fn(Ptr<std_atomic_ref_unsigned_short_>, u16, u16)>,
            > = Rc::new(RefCell::new(FnPtr::<
                fn(Ptr<std_atomic_ref_unsigned_short_>, u16, u16),
            >::new(
                |x: Ptr<std_atomic_ref_unsigned_short_>, mut old_val: u16, mut new_val: u16| {
                    ({
                        ((*x.upgrade().deref()) as std___atomic_ref_unsigned_short__true__false_)
                            .fetch_add(
                                (((new_val as i32) - (old_val as i32)) as u16),
                                Some(std_memory_order_seq_cst),
                            )
                    });
                },
            )));
            let load: Value<FnPtr<fn(Ptr<std_atomic_ref_unsigned_short_>) -> u16>> =
                Rc::new(RefCell::new(FnPtr::<
                    fn(Ptr<std_atomic_ref_unsigned_short_>) -> u16,
                >::new(
                    |x: Ptr<std_atomic_ref_unsigned_short_>| -> u16 {
                        {
                            return ({
                                ((*x.upgrade().deref())
                                    as std___atomic_ref_unsigned_short__true__false_)
                                    .load(None)
                            });
                        }
                    },
                )));
            ({
                test_seq_cst_22(
                    (*fetch_add_no_arg.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
            ({
                test_seq_cst_23(
                    (*fetch_add_with_order.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
        }
    }
}
pub trait TestFetchAdd_wchar_t_Impl {
    fn operator_call(&self);
}
impl TestFetchAdd_wchar_t_Impl for Ptr<TestFetchAdd_wchar_t_> {
    fn operator_call(&self) {
        {
            let x: Value<i32> = Rc::new(RefCell::new(1_i32));
            let a: Value<std_atomic_ref_wchar_t_> =
                Rc::new(RefCell::new(std_atomic_ref_wchar_t_::new({
                    x.as_pointer()
                })));
            {
                let mut y: i32 = ({
                    ((*a.borrow()) as std___atomic_ref_wchar_t__true__false_).fetch_add(2_i32, None)
                });
                assert!((y == 1_i32));
                assert!(({ ((*x.borrow()) == 3_i32) }));
                const _: () = assert!(true, "");;
            }
            {
                let mut y: i32 = ({
                    ((*a.borrow()) as std___atomic_ref_wchar_t__true__false_)
                        .fetch_add(4_i32, Some(memory_order_relaxed_71.with(|rc| *rc.borrow())))
                });
                assert!((y == 3_i32));
                assert!(({ ((*x.borrow()) == 7_i32) }));
                const _: () = assert!(true, "");;
            }
        }
        {
            let fetch_add: Value<FnPtr<fn(Ptr<std_atomic_ref_wchar_t_>, i32, i32)>> =
                Rc::new(RefCell::new(FnPtr::<
                    fn(Ptr<std_atomic_ref_wchar_t_>, i32, i32),
                >::new(
                    |x: Ptr<std_atomic_ref_wchar_t_>, mut old_val: i32, mut new_val: i32| {
                        ({
                            ((*x.upgrade().deref()) as std___atomic_ref_wchar_t__true__false_)
                                .fetch_add((new_val - old_val), Some(std_memory_order_release))
                        });
                    },
                )));
            let load: Value<FnPtr<fn(Ptr<std_atomic_ref_wchar_t_>) -> i32>> = Rc::new(
                RefCell::new(FnPtr::<fn(Ptr<std_atomic_ref_wchar_t_>) -> i32>::new(
                    |x: Ptr<std_atomic_ref_wchar_t_>| -> i32 {
                        {
                            return ({
                                ((*x.upgrade().deref()) as std___atomic_ref_wchar_t__true__false_)
                                    .load(Some(std_memory_order_acquire))
                            });
                        }
                    },
                )),
            );
            ({
                test_acquire_release_63(
                    (*fetch_add.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
        }
        {
            let fetch_add_no_arg: Value<FnPtr<fn(Ptr<std_atomic_ref_wchar_t_>, i32, i32)>> =
                Rc::new(RefCell::new(FnPtr::<
                    fn(Ptr<std_atomic_ref_wchar_t_>, i32, i32),
                >::new(
                    |x: Ptr<std_atomic_ref_wchar_t_>, mut old_val: i32, mut new_val: i32| {
                        ({
                            ((*x.upgrade().deref()) as std___atomic_ref_wchar_t__true__false_)
                                .fetch_add((new_val - old_val), None)
                        });
                    },
                )));
            let fetch_add_with_order: Value<FnPtr<fn(Ptr<std_atomic_ref_wchar_t_>, i32, i32)>> =
                Rc::new(RefCell::new(FnPtr::<
                    fn(Ptr<std_atomic_ref_wchar_t_>, i32, i32),
                >::new(
                    |x: Ptr<std_atomic_ref_wchar_t_>, mut old_val: i32, mut new_val: i32| {
                        ({
                            ((*x.upgrade().deref()) as std___atomic_ref_wchar_t__true__false_)
                                .fetch_add((new_val - old_val), Some(std_memory_order_seq_cst))
                        });
                    },
                )));
            let load: Value<FnPtr<fn(Ptr<std_atomic_ref_wchar_t_>) -> i32>> = Rc::new(
                RefCell::new(FnPtr::<fn(Ptr<std_atomic_ref_wchar_t_>) -> i32>::new(
                    |x: Ptr<std_atomic_ref_wchar_t_>| -> i32 {
                        {
                            return ({
                                ((*x.upgrade().deref()) as std___atomic_ref_wchar_t__true__false_)
                                    .load(None)
                            });
                        }
                    },
                )),
            );
            ({
                test_seq_cst_36(
                    (*fetch_add_no_arg.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
            ({
                test_seq_cst_37(
                    (*fetch_add_with_order.borrow()).copy_from(),
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
