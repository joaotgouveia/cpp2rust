extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type LockFreeStatus = ();
pub const LockFreeStatus_unknown: LockFreeStatus = -1;
pub const LockFreeStatus_never: LockFreeStatus = 0;
pub const LockFreeStatus_sometimes: LockFreeStatus = 1;
pub const LockFreeStatus_always: LockFreeStatus = 2;
thread_local!(
    pub static value_0: Value<LockFreeStatus> = Rc::new(RefCell::new(
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
    pub static status_known_1: Value<()> = Rc::new(RefCell::new(
        (value_0.with(|rc| *rc.borrow()) != LockFreeStatus_unknown),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct LockFreeStatusInfo_char_ {}
thread_local!(
    pub static value_2: Value<LockFreeStatus> = Rc::new(RefCell::new(
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
    pub static status_known_3: Value<()> = Rc::new(RefCell::new(
        (value_2.with(|rc| *rc.borrow()) != LockFreeStatus_unknown),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct LockFreeStatusInfo_short_ {}
thread_local!(
    pub static value_4: Value<LockFreeStatus> = Rc::new(RefCell::new(
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
    pub static status_known_5: Value<()> = Rc::new(RefCell::new(
        (value_4.with(|rc| *rc.borrow()) != LockFreeStatus_unknown),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct LockFreeStatusInfo_int_ {}
thread_local!(
    pub static value_6: Value<LockFreeStatus> = Rc::new(RefCell::new(
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
    pub static status_known_7: Value<()> = Rc::new(RefCell::new(
        (value_6.with(|rc| *rc.borrow()) != LockFreeStatus_unknown),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct LockFreeStatusInfo_long_ {}
thread_local!(
    pub static value_8: Value<LockFreeStatus> = Rc::new(RefCell::new(
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
    pub static status_known_9: Value<()> = Rc::new(RefCell::new(
        (value_8.with(|rc| *rc.borrow()) != LockFreeStatus_unknown),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct LockFreeStatusInfo_void_ptr_ {}
thread_local!(
    pub static value_10: Value<LockFreeStatus> = Rc::new(RefCell::new(
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
    pub static status_known_11: Value<()> = Rc::new(RefCell::new(
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
pub fn operator_eq_12(x: Ptr<UserAtomicType>, y: Ptr<UserAtomicType>) -> () {
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
pub fn operator_eq_13(x: Ptr<LargeUserAtomicType>, y: Ptr<LargeUserAtomicType>) -> () {
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
pub struct TestEachIntegralType_TestFetchAdd_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestEachPointerType_TestFetchAdd_ {}
pub fn test_seq_cst_14(store_op: FnPtr<fn(Ptr<()>, (), ())>, load_op: FnPtr<fn(Ptr<()>) -> ()>) {
    let store_op: Value<FnPtr<fn(Ptr<()>, (), ())>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> ()>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_15(store_op: FnPtr<fn(Ptr<()>, (), ())>, load_op: FnPtr<fn(Ptr<()>) -> ()>) {
    let store_op: Value<FnPtr<fn(Ptr<()>, (), ())>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> ()>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_16(store_op: FnPtr<fn(Ptr<()>, (), ())>, load_op: FnPtr<fn(Ptr<()>) -> ()>) {
    let store_op: Value<FnPtr<fn(Ptr<()>, (), ())>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> ()>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_17(store_op: FnPtr<fn(Ptr<()>, (), ())>, load_op: FnPtr<fn(Ptr<()>) -> ()>) {
    let store_op: Value<FnPtr<fn(Ptr<()>, (), ())>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> ()>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_18(store_op: FnPtr<fn(Ptr<()>, (), ())>, load_op: FnPtr<fn(Ptr<()>) -> ()>) {
    let store_op: Value<FnPtr<fn(Ptr<()>, (), ())>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> ()>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_19(store_op: FnPtr<fn(Ptr<()>, (), ())>, load_op: FnPtr<fn(Ptr<()>) -> ()>) {
    let store_op: Value<FnPtr<fn(Ptr<()>, (), ())>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> ()>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_20(store_op: FnPtr<fn(Ptr<()>, (), ())>, load_op: FnPtr<fn(Ptr<()>) -> ()>) {
    let store_op: Value<FnPtr<fn(Ptr<()>, (), ())>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> ()>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_21(store_op: FnPtr<fn(Ptr<()>, (), ())>, load_op: FnPtr<fn(Ptr<()>) -> ()>) {
    let store_op: Value<FnPtr<fn(Ptr<()>, (), ())>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> ()>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_22(store_op: FnPtr<fn(Ptr<()>, (), ())>, load_op: FnPtr<fn(Ptr<()>) -> ()>) {
    let store_op: Value<FnPtr<fn(Ptr<()>, (), ())>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> ()>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_23(store_op: FnPtr<fn(Ptr<()>, (), ())>, load_op: FnPtr<fn(Ptr<()>) -> ()>) {
    let store_op: Value<FnPtr<fn(Ptr<()>, (), ())>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> ()>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_24(store_op: FnPtr<fn(Ptr<()>, (), ())>, load_op: FnPtr<fn(Ptr<()>) -> ()>) {
    let store_op: Value<FnPtr<fn(Ptr<()>, (), ())>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> ()>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_25(store_op: FnPtr<fn(Ptr<()>, (), ())>, load_op: FnPtr<fn(Ptr<()>) -> ()>) {
    let store_op: Value<FnPtr<fn(Ptr<()>, (), ())>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> ()>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_26(store_op: FnPtr<fn(Ptr<()>, (), ())>, load_op: FnPtr<fn(Ptr<()>) -> ()>) {
    let store_op: Value<FnPtr<fn(Ptr<()>, (), ())>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> ()>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_27(store_op: FnPtr<fn(Ptr<()>, (), ())>, load_op: FnPtr<fn(Ptr<()>) -> ()>) {
    let store_op: Value<FnPtr<fn(Ptr<()>, (), ())>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> ()>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_28(store_op: FnPtr<fn(Ptr<()>, (), ())>, load_op: FnPtr<fn(Ptr<()>) -> ()>) {
    let store_op: Value<FnPtr<fn(Ptr<()>, (), ())>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> ()>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_29(store_op: FnPtr<fn(Ptr<()>, (), ())>, load_op: FnPtr<fn(Ptr<()>) -> ()>) {
    let store_op: Value<FnPtr<fn(Ptr<()>, (), ())>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> ()>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_30(store_op: FnPtr<fn(Ptr<()>, (), ())>, load_op: FnPtr<fn(Ptr<()>) -> ()>) {
    let store_op: Value<FnPtr<fn(Ptr<()>, (), ())>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> ()>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_31(store_op: FnPtr<fn(Ptr<()>, (), ())>, load_op: FnPtr<fn(Ptr<()>) -> ()>) {
    let store_op: Value<FnPtr<fn(Ptr<()>, (), ())>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> ()>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_32(store_op: FnPtr<fn(Ptr<()>, (), ())>, load_op: FnPtr<fn(Ptr<()>) -> ()>) {
    let store_op: Value<FnPtr<fn(Ptr<()>, (), ())>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> ()>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_33(store_op: FnPtr<fn(Ptr<()>, (), ())>, load_op: FnPtr<fn(Ptr<()>) -> ()>) {
    let store_op: Value<FnPtr<fn(Ptr<()>, (), ())>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> ()>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_34(store_op: FnPtr<fn(Ptr<()>, (), ())>, load_op: FnPtr<fn(Ptr<()>) -> ()>) {
    let store_op: Value<FnPtr<fn(Ptr<()>, (), ())>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> ()>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_35(store_op: FnPtr<fn(Ptr<()>, (), ())>, load_op: FnPtr<fn(Ptr<()>) -> ()>) {
    let store_op: Value<FnPtr<fn(Ptr<()>, (), ())>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> ()>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_36(store_op: FnPtr<fn(Ptr<()>, (), ())>, load_op: FnPtr<fn(Ptr<()>) -> ()>) {
    let store_op: Value<FnPtr<fn(Ptr<()>, (), ())>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> ()>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_37(store_op: FnPtr<fn(Ptr<()>, (), ())>, load_op: FnPtr<fn(Ptr<()>) -> ()>) {
    let store_op: Value<FnPtr<fn(Ptr<()>, (), ())>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> ()>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_38(store_op: FnPtr<fn(Ptr<()>, (), ())>, load_op: FnPtr<fn(Ptr<()>) -> ()>) {
    let store_op: Value<FnPtr<fn(Ptr<()>, (), ())>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> ()>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_39(store_op: FnPtr<fn(Ptr<()>, (), ())>, load_op: FnPtr<fn(Ptr<()>) -> ()>) {
    let store_op: Value<FnPtr<fn(Ptr<()>, (), ())>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> ()>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_40(store_op: FnPtr<fn(Ptr<()>, (), ())>, load_op: FnPtr<fn(Ptr<()>) -> ()>) {
    let store_op: Value<FnPtr<fn(Ptr<()>, (), ())>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> ()>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_41(store_op: FnPtr<fn(Ptr<()>, (), ())>, load_op: FnPtr<fn(Ptr<()>) -> ()>) {
    let store_op: Value<FnPtr<fn(Ptr<()>, (), ())>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> ()>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_42(store_op: FnPtr<fn(Ptr<()>, (), ())>, load_op: FnPtr<fn(Ptr<()>) -> ()>) {
    let store_op: Value<FnPtr<fn(Ptr<()>, (), ())>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> ()>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_43(store_op: FnPtr<fn(Ptr<()>, (), ())>, load_op: FnPtr<fn(Ptr<()>) -> ()>) {
    let store_op: Value<FnPtr<fn(Ptr<()>, (), ())>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> ()>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_44(store_op: FnPtr<fn(Ptr<()>, (), ())>, load_op: FnPtr<fn(Ptr<()>) -> ()>) {
    let store_op: Value<FnPtr<fn(Ptr<()>, (), ())>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> ()>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_45(store_op: FnPtr<fn(Ptr<()>, (), ())>, load_op: FnPtr<fn(Ptr<()>) -> ()>) {
    let store_op: Value<FnPtr<fn(Ptr<()>, (), ())>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> ()>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_46(store_op: FnPtr<fn(Ptr<()>, (), ())>, load_op: FnPtr<fn(Ptr<()>) -> ()>) {
    let store_op: Value<FnPtr<fn(Ptr<()>, (), ())>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> ()>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_47(store_op: FnPtr<fn(Ptr<()>, (), ())>, load_op: FnPtr<fn(Ptr<()>) -> ()>) {
    let store_op: Value<FnPtr<fn(Ptr<()>, (), ())>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> ()>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_48(
    store_op: FnPtr<fn(Ptr<()>, Ptr<()>, Ptr<()>)>,
    load_op: FnPtr<fn(Ptr<()>) -> Ptr<()>>,
) {
    let store_op: Value<FnPtr<fn(Ptr<()>, Ptr<()>, Ptr<()>)>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> Ptr<()>>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_49(
    store_op: FnPtr<fn(Ptr<()>, Ptr<()>, Ptr<()>)>,
    load_op: FnPtr<fn(Ptr<()>) -> Ptr<()>>,
) {
    let store_op: Value<FnPtr<fn(Ptr<()>, Ptr<()>, Ptr<()>)>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> Ptr<()>>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_50(
    store_op: FnPtr<fn(Ptr<()>, Ptr<()>, Ptr<()>)>,
    load_op: FnPtr<fn(Ptr<()>) -> Ptr<()>>,
) {
    let store_op: Value<FnPtr<fn(Ptr<()>, Ptr<()>, Ptr<()>)>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> Ptr<()>>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_51(
    store_op: FnPtr<fn(Ptr<()>, Ptr<()>, Ptr<()>)>,
    load_op: FnPtr<fn(Ptr<()>) -> Ptr<()>>,
) {
    let store_op: Value<FnPtr<fn(Ptr<()>, Ptr<()>, Ptr<()>)>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> Ptr<()>>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_52(
    store_op: FnPtr<fn(Ptr<()>, (), ())>,
    load_op: FnPtr<fn(Ptr<()>) -> ()>,
) {
    let store_op: Value<FnPtr<fn(Ptr<()>, (), ())>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> ()>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_53(
    store_op: FnPtr<fn(Ptr<()>, (), ())>,
    load_op: FnPtr<fn(Ptr<()>) -> ()>,
) {
    let store_op: Value<FnPtr<fn(Ptr<()>, (), ())>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> ()>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_54(
    store_op: FnPtr<fn(Ptr<()>, (), ())>,
    load_op: FnPtr<fn(Ptr<()>) -> ()>,
) {
    let store_op: Value<FnPtr<fn(Ptr<()>, (), ())>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> ()>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_55(
    store_op: FnPtr<fn(Ptr<()>, (), ())>,
    load_op: FnPtr<fn(Ptr<()>) -> ()>,
) {
    let store_op: Value<FnPtr<fn(Ptr<()>, (), ())>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> ()>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_56(
    store_op: FnPtr<fn(Ptr<()>, (), ())>,
    load_op: FnPtr<fn(Ptr<()>) -> ()>,
) {
    let store_op: Value<FnPtr<fn(Ptr<()>, (), ())>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> ()>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_57(
    store_op: FnPtr<fn(Ptr<()>, (), ())>,
    load_op: FnPtr<fn(Ptr<()>) -> ()>,
) {
    let store_op: Value<FnPtr<fn(Ptr<()>, (), ())>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> ()>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_58(
    store_op: FnPtr<fn(Ptr<()>, (), ())>,
    load_op: FnPtr<fn(Ptr<()>) -> ()>,
) {
    let store_op: Value<FnPtr<fn(Ptr<()>, (), ())>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> ()>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_59(
    store_op: FnPtr<fn(Ptr<()>, (), ())>,
    load_op: FnPtr<fn(Ptr<()>) -> ()>,
) {
    let store_op: Value<FnPtr<fn(Ptr<()>, (), ())>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> ()>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_60(
    store_op: FnPtr<fn(Ptr<()>, (), ())>,
    load_op: FnPtr<fn(Ptr<()>) -> ()>,
) {
    let store_op: Value<FnPtr<fn(Ptr<()>, (), ())>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> ()>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_61(
    store_op: FnPtr<fn(Ptr<()>, (), ())>,
    load_op: FnPtr<fn(Ptr<()>) -> ()>,
) {
    let store_op: Value<FnPtr<fn(Ptr<()>, (), ())>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> ()>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_62(
    store_op: FnPtr<fn(Ptr<()>, (), ())>,
    load_op: FnPtr<fn(Ptr<()>) -> ()>,
) {
    let store_op: Value<FnPtr<fn(Ptr<()>, (), ())>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> ()>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_63(
    store_op: FnPtr<fn(Ptr<()>, (), ())>,
    load_op: FnPtr<fn(Ptr<()>) -> ()>,
) {
    let store_op: Value<FnPtr<fn(Ptr<()>, (), ())>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> ()>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_64(
    store_op: FnPtr<fn(Ptr<()>, (), ())>,
    load_op: FnPtr<fn(Ptr<()>) -> ()>,
) {
    let store_op: Value<FnPtr<fn(Ptr<()>, (), ())>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> ()>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_65(
    store_op: FnPtr<fn(Ptr<()>, (), ())>,
    load_op: FnPtr<fn(Ptr<()>) -> ()>,
) {
    let store_op: Value<FnPtr<fn(Ptr<()>, (), ())>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> ()>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_66(
    store_op: FnPtr<fn(Ptr<()>, (), ())>,
    load_op: FnPtr<fn(Ptr<()>) -> ()>,
) {
    let store_op: Value<FnPtr<fn(Ptr<()>, (), ())>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> ()>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_67(
    store_op: FnPtr<fn(Ptr<()>, (), ())>,
    load_op: FnPtr<fn(Ptr<()>) -> ()>,
) {
    let store_op: Value<FnPtr<fn(Ptr<()>, (), ())>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> ()>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_68(
    store_op: FnPtr<fn(Ptr<()>, (), ())>,
    load_op: FnPtr<fn(Ptr<()>) -> ()>,
) {
    let store_op: Value<FnPtr<fn(Ptr<()>, (), ())>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> ()>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_69(
    store_op: FnPtr<fn(Ptr<()>, Ptr<()>, Ptr<()>)>,
    load_op: FnPtr<fn(Ptr<()>) -> Ptr<()>>,
) {
    let store_op: Value<FnPtr<fn(Ptr<()>, Ptr<()>, Ptr<()>)>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> Ptr<()>>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_70(
    store_op: FnPtr<fn(Ptr<()>, Ptr<()>, Ptr<()>)>,
    load_op: FnPtr<fn(Ptr<()>) -> Ptr<()>>,
) {
    let store_op: Value<FnPtr<fn(Ptr<()>, Ptr<()>, Ptr<()>)>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> Ptr<()>>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestDoesNotHaveFetchAdd_bool_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestDoesNotHaveFetchAdd_UserAtomicType_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestDoesNotHaveFetchAdd_LargeUserAtomicType_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFetchAdd_float_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFetchAdd_double_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFetchAdd_char_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFetchAdd_signed_char_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFetchAdd_unsigned_char_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFetchAdd_short_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFetchAdd_unsigned_short_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFetchAdd_int_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFetchAdd_unsigned_int_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFetchAdd_long_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFetchAdd_unsigned_long_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFetchAdd_long_long_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFetchAdd_unsigned_long_long_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFetchAdd_wchar_t_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFetchAdd_char8_t_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFetchAdd_char16_t_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFetchAdd_char32_t_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFetchAdd_int_ptr_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFetchAdd_const_int_ptr_ {}
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
    return 0_();
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
            let mut x: () = 1_();
            let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            {
                let mut y: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                const _: () = assert!(true, "");;
            }
            {
                let mut y: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                const _: () = assert!(true, "");;
            }
        }
        {
            let fetch_add: Value<FnPtr<fn(Ptr<()>, (), ())>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, (), ())>::new(
                    |x: Ptr<()>, mut old_val: (), mut new_val: ()| {
                        unimplemented!();
                    },
                )));
            let load: Value<FnPtr<fn(Ptr<()>) -> ()>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>) -> ()>::new(
                    |x: Ptr<()>| -> () {
                        {
                            return unimplemented!();
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
            let fetch_add_no_arg: Value<FnPtr<fn(Ptr<()>, (), ())>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, (), ())>::new(
                    |x: Ptr<()>, mut old_val: (), mut new_val: ()| {
                        unimplemented!();
                    },
                )));
            let fetch_add_with_order: Value<FnPtr<fn(Ptr<()>, (), ())>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, (), ())>::new(
                    |x: Ptr<()>, mut old_val: (), mut new_val: ()| {
                        unimplemented!();
                    },
                )));
            let load: Value<FnPtr<fn(Ptr<()>) -> ()>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>) -> ()>::new(
                    |x: Ptr<()>| -> () {
                        {
                            return unimplemented!();
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
            let mut x: () = 1_();
            let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            {
                let mut y: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                const _: () = assert!(true, "");;
            }
            {
                let mut y: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                const _: () = assert!(true, "");;
            }
        }
        {
            let fetch_add: Value<FnPtr<fn(Ptr<()>, (), ())>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, (), ())>::new(
                    |x: Ptr<()>, mut old_val: (), mut new_val: ()| {
                        unimplemented!();
                    },
                )));
            let load: Value<FnPtr<fn(Ptr<()>) -> ()>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>) -> ()>::new(
                    |x: Ptr<()>| -> () {
                        {
                            return unimplemented!();
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
            let fetch_add_no_arg: Value<FnPtr<fn(Ptr<()>, (), ())>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, (), ())>::new(
                    |x: Ptr<()>, mut old_val: (), mut new_val: ()| {
                        unimplemented!();
                    },
                )));
            let fetch_add_with_order: Value<FnPtr<fn(Ptr<()>, (), ())>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, (), ())>::new(
                    |x: Ptr<()>, mut old_val: (), mut new_val: ()| {
                        unimplemented!();
                    },
                )));
            let load: Value<FnPtr<fn(Ptr<()>) -> ()>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>) -> ()>::new(
                    |x: Ptr<()>| -> () {
                        {
                            return unimplemented!();
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
            let mut x: () = 1_();
            let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            {
                let mut y: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                const _: () = assert!(true, "");;
            }
            {
                let mut y: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                const _: () = assert!(true, "");;
            }
        }
        {
            let fetch_add: Value<FnPtr<fn(Ptr<()>, (), ())>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, (), ())>::new(
                    |x: Ptr<()>, mut old_val: (), mut new_val: ()| {
                        unimplemented!();
                    },
                )));
            let load: Value<FnPtr<fn(Ptr<()>) -> ()>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>) -> ()>::new(
                    |x: Ptr<()>| -> () {
                        {
                            return unimplemented!();
                        }
                    },
                )));
            ({
                test_acquire_release_64(
                    (*fetch_add.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
        }
        {
            let fetch_add_no_arg: Value<FnPtr<fn(Ptr<()>, (), ())>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, (), ())>::new(
                    |x: Ptr<()>, mut old_val: (), mut new_val: ()| {
                        unimplemented!();
                    },
                )));
            let fetch_add_with_order: Value<FnPtr<fn(Ptr<()>, (), ())>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, (), ())>::new(
                    |x: Ptr<()>, mut old_val: (), mut new_val: ()| {
                        unimplemented!();
                    },
                )));
            let load: Value<FnPtr<fn(Ptr<()>) -> ()>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>) -> ()>::new(
                    |x: Ptr<()>| -> () {
                        {
                            return unimplemented!();
                        }
                    },
                )));
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
            let mut x: () = 1_();
            let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            {
                let mut y: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                const _: () = assert!(true, "");;
            }
            {
                let mut y: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                const _: () = assert!(true, "");;
            }
        }
        {
            let fetch_add: Value<FnPtr<fn(Ptr<()>, (), ())>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, (), ())>::new(
                    |x: Ptr<()>, mut old_val: (), mut new_val: ()| {
                        unimplemented!();
                    },
                )));
            let load: Value<FnPtr<fn(Ptr<()>) -> ()>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>) -> ()>::new(
                    |x: Ptr<()>| -> () {
                        {
                            return unimplemented!();
                        }
                    },
                )));
            ({
                test_acquire_release_52(
                    (*fetch_add.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
        }
        {
            let fetch_add_no_arg: Value<FnPtr<fn(Ptr<()>, (), ())>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, (), ())>::new(
                    |x: Ptr<()>, mut old_val: (), mut new_val: ()| {
                        unimplemented!();
                    },
                )));
            let fetch_add_with_order: Value<FnPtr<fn(Ptr<()>, (), ())>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, (), ())>::new(
                    |x: Ptr<()>, mut old_val: (), mut new_val: ()| {
                        unimplemented!();
                    },
                )));
            let load: Value<FnPtr<fn(Ptr<()>) -> ()>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>) -> ()>::new(
                    |x: Ptr<()>| -> () {
                        {
                            return unimplemented!();
                        }
                    },
                )));
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
                let mut p: Ptr<()> = ((t.as_pointer() as Ptr<()>).offset(1_()));
                let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                {
                    let mut y: Ptr<()> = ({
                        ((*a.borrow()) as std___atomic_ref_const_int_ptr__false__false_)
                            .fetch_add(2_(), None)
                    });
                    unimplemented!();
                    unimplemented!();
                    const _: () = assert!(true, "");;
                }
                {
                    let mut y: Ptr<()> = ({
                        ((*a.borrow()) as std___atomic_ref_const_int_ptr__false__false_)
                            .fetch_add(4_(), Some(unimplemented!()))
                    });
                    unimplemented!();
                    unimplemented!();
                    const _: () = assert!(true, "");;
                }
            }
        }
        {
            let fetch_add: Value<FnPtr<fn(Ptr<()>, Ptr<()>, Ptr<()>)>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, Ptr<()>, Ptr<()>)>::new(
                    |x: Ptr<()>, mut old_val: Ptr<()>, mut new_val: Ptr<()>| {
                        ({
                            ((*x.upgrade().deref())
                                as std___atomic_ref_const_int_ptr__false__false_)
                                .fetch_add(
                                    (((new_val).clone() - (old_val).clone()) as () as ()),
                                    Some(unimplemented!()),
                                )
                        });
                    },
                )));
            let load: Value<FnPtr<fn(Ptr<()>) -> Ptr<()>>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>) -> Ptr<()>>::new(
                    |x: Ptr<()>| -> Ptr<()> {
                        {
                            return ({
                                ((*x.upgrade().deref())
                                    as std___atomic_ref_const_int_ptr__false__false_)
                                    .load(Some(unimplemented!()))
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
            let fetch_add_no_arg: Value<FnPtr<fn(Ptr<()>, Ptr<()>, Ptr<()>)>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, Ptr<()>, Ptr<()>)>::new(
                    |x: Ptr<()>, mut old_val: Ptr<()>, mut new_val: Ptr<()>| {
                        ({
                            ((*x.upgrade().deref())
                                as std___atomic_ref_const_int_ptr__false__false_)
                                .fetch_add(
                                    (((new_val).clone() - (old_val).clone()) as () as ()),
                                    None,
                                )
                        });
                    },
                )));
            let fetch_add_with_order: Value<FnPtr<fn(Ptr<()>, Ptr<()>, Ptr<()>)>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, Ptr<()>, Ptr<()>)>::new(
                    |x: Ptr<()>, mut old_val: Ptr<()>, mut new_val: Ptr<()>| {
                        ({
                            ((*x.upgrade().deref())
                                as std___atomic_ref_const_int_ptr__false__false_)
                                .fetch_add(
                                    (((new_val).clone() - (old_val).clone()) as () as ()),
                                    Some(unimplemented!()),
                                )
                        });
                    },
                )));
            let load: Value<FnPtr<fn(Ptr<()>) -> Ptr<()>>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>) -> Ptr<()>>::new(
                    |x: Ptr<()>| -> Ptr<()> {
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
            let mut x: () = 1_();
            let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            {
                let mut y: () = ({
                    ((*a.borrow()) as std___atomic_ref_double__false__true_).fetch_add(2_(), None)
                });
                unimplemented!();
                unimplemented!();
                const _: () = assert!(true, "");;
            }
            {
                let mut y: () = ({
                    ((*a.borrow()) as std___atomic_ref_double__false__true_)
                        .fetch_add(4_(), Some(unimplemented!()))
                });
                unimplemented!();
                unimplemented!();
                const _: () = assert!(true, "");;
            }
        }
        {
            let fetch_add: Value<FnPtr<fn(Ptr<()>, (), ())>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, (), ())>::new(
                    |x: Ptr<()>, mut old_val: (), mut new_val: ()| {
                        ({
                            ((*x.upgrade().deref()) as std___atomic_ref_double__false__true_)
                                .fetch_add((new_val - old_val), Some(unimplemented!()))
                        });
                    },
                )));
            let load: Value<FnPtr<fn(Ptr<()>) -> ()>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>) -> ()>::new(
                    |x: Ptr<()>| -> () {
                        {
                            return ({
                                ((*x.upgrade().deref()) as std___atomic_ref_double__false__true_)
                                    .load(Some(unimplemented!()))
                            });
                        }
                    },
                )));
            ({
                test_acquire_release_68(
                    (*fetch_add.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
        }
        {
            let fetch_add_no_arg: Value<FnPtr<fn(Ptr<()>, (), ())>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, (), ())>::new(
                    |x: Ptr<()>, mut old_val: (), mut new_val: ()| {
                        ({
                            ((*x.upgrade().deref()) as std___atomic_ref_double__false__true_)
                                .fetch_add((new_val - old_val), None)
                        });
                    },
                )));
            let fetch_add_with_order: Value<FnPtr<fn(Ptr<()>, (), ())>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, (), ())>::new(
                    |x: Ptr<()>, mut old_val: (), mut new_val: ()| {
                        ({
                            ((*x.upgrade().deref()) as std___atomic_ref_double__false__true_)
                                .fetch_add((new_val - old_val), Some(unimplemented!()))
                        });
                    },
                )));
            let load: Value<FnPtr<fn(Ptr<()>) -> ()>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>) -> ()>::new(
                    |x: Ptr<()>| -> () {
                        {
                            return ({
                                ((*x.upgrade().deref()) as std___atomic_ref_double__false__true_)
                                    .load(None)
                            });
                        }
                    },
                )));
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
            let mut x: () = 1_();
            let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            {
                let mut y: () = ({
                    ((*a.borrow()) as std___atomic_ref_float__false__true_).fetch_add(2_(), None)
                });
                unimplemented!();
                unimplemented!();
                const _: () = assert!(true, "");;
            }
            {
                let mut y: () = ({
                    ((*a.borrow()) as std___atomic_ref_float__false__true_)
                        .fetch_add(4_(), Some(unimplemented!()))
                });
                unimplemented!();
                unimplemented!();
                const _: () = assert!(true, "");;
            }
        }
        {
            let fetch_add: Value<FnPtr<fn(Ptr<()>, (), ())>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, (), ())>::new(
                    |x: Ptr<()>, mut old_val: (), mut new_val: ()| {
                        ({
                            ((*x.upgrade().deref()) as std___atomic_ref_float__false__true_)
                                .fetch_add((new_val - old_val), Some(unimplemented!()))
                        });
                    },
                )));
            let load: Value<FnPtr<fn(Ptr<()>) -> ()>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>) -> ()>::new(
                    |x: Ptr<()>| -> () {
                        {
                            return ({
                                ((*x.upgrade().deref()) as std___atomic_ref_float__false__true_)
                                    .load(Some(unimplemented!()))
                            });
                        }
                    },
                )));
            ({
                test_acquire_release_67(
                    (*fetch_add.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
        }
        {
            let fetch_add_no_arg: Value<FnPtr<fn(Ptr<()>, (), ())>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, (), ())>::new(
                    |x: Ptr<()>, mut old_val: (), mut new_val: ()| {
                        ({
                            ((*x.upgrade().deref()) as std___atomic_ref_float__false__true_)
                                .fetch_add((new_val - old_val), None)
                        });
                    },
                )));
            let fetch_add_with_order: Value<FnPtr<fn(Ptr<()>, (), ())>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, (), ())>::new(
                    |x: Ptr<()>, mut old_val: (), mut new_val: ()| {
                        ({
                            ((*x.upgrade().deref()) as std___atomic_ref_float__false__true_)
                                .fetch_add((new_val - old_val), Some(unimplemented!()))
                        });
                    },
                )));
            let load: Value<FnPtr<fn(Ptr<()>) -> ()>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>) -> ()>::new(
                    |x: Ptr<()>| -> () {
                        {
                            return ({
                                ((*x.upgrade().deref()) as std___atomic_ref_float__false__true_)
                                    .load(None)
                            });
                        }
                    },
                )));
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
            let mut x: () = 1_();
            let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            {
                let mut y: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                const _: () = assert!(true, "");;
            }
            {
                let mut y: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                const _: () = assert!(true, "");;
            }
        }
        {
            let fetch_add: Value<FnPtr<fn(Ptr<()>, (), ())>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, (), ())>::new(
                    |x: Ptr<()>, mut old_val: (), mut new_val: ()| {
                        unimplemented!();
                    },
                )));
            let load: Value<FnPtr<fn(Ptr<()>) -> ()>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>) -> ()>::new(
                    |x: Ptr<()>| -> () {
                        {
                            return unimplemented!();
                        }
                    },
                )));
            ({
                test_acquire_release_57(
                    (*fetch_add.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
        }
        {
            let fetch_add_no_arg: Value<FnPtr<fn(Ptr<()>, (), ())>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, (), ())>::new(
                    |x: Ptr<()>, mut old_val: (), mut new_val: ()| {
                        unimplemented!();
                    },
                )));
            let fetch_add_with_order: Value<FnPtr<fn(Ptr<()>, (), ())>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, (), ())>::new(
                    |x: Ptr<()>, mut old_val: (), mut new_val: ()| {
                        unimplemented!();
                    },
                )));
            let load: Value<FnPtr<fn(Ptr<()>) -> ()>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>) -> ()>::new(
                    |x: Ptr<()>| -> () {
                        {
                            return unimplemented!();
                        }
                    },
                )));
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
                let mut p: Ptr<()> = ((t.as_pointer() as Ptr<()>).offset(1_()));
                let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                {
                    let mut y: Ptr<()> = ({
                        ((*a.borrow()) as std___atomic_ref_int_ptr__false__false_)
                            .fetch_add(2_(), None)
                    });
                    unimplemented!();
                    unimplemented!();
                    const _: () = assert!(true, "");;
                }
                {
                    let mut y: Ptr<()> = ({
                        ((*a.borrow()) as std___atomic_ref_int_ptr__false__false_)
                            .fetch_add(4_(), Some(unimplemented!()))
                    });
                    unimplemented!();
                    unimplemented!();
                    const _: () = assert!(true, "");;
                }
            }
        }
        {
            let fetch_add: Value<FnPtr<fn(Ptr<()>, Ptr<()>, Ptr<()>)>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, Ptr<()>, Ptr<()>)>::new(
                    |x: Ptr<()>, mut old_val: Ptr<()>, mut new_val: Ptr<()>| {
                        ({
                            ((*x.upgrade().deref()) as std___atomic_ref_int_ptr__false__false_)
                                .fetch_add(
                                    (((new_val).clone() - (old_val).clone()) as () as ()),
                                    Some(unimplemented!()),
                                )
                        });
                    },
                )));
            let load: Value<FnPtr<fn(Ptr<()>) -> Ptr<()>>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>) -> Ptr<()>>::new(
                    |x: Ptr<()>| -> Ptr<()> {
                        {
                            return ({
                                ((*x.upgrade().deref()) as std___atomic_ref_int_ptr__false__false_)
                                    .load(Some(unimplemented!()))
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
            let fetch_add_no_arg: Value<FnPtr<fn(Ptr<()>, Ptr<()>, Ptr<()>)>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, Ptr<()>, Ptr<()>)>::new(
                    |x: Ptr<()>, mut old_val: Ptr<()>, mut new_val: Ptr<()>| {
                        ({
                            ((*x.upgrade().deref()) as std___atomic_ref_int_ptr__false__false_)
                                .fetch_add(
                                    (((new_val).clone() - (old_val).clone()) as () as ()),
                                    None,
                                )
                        });
                    },
                )));
            let fetch_add_with_order: Value<FnPtr<fn(Ptr<()>, Ptr<()>, Ptr<()>)>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, Ptr<()>, Ptr<()>)>::new(
                    |x: Ptr<()>, mut old_val: Ptr<()>, mut new_val: Ptr<()>| {
                        ({
                            ((*x.upgrade().deref()) as std___atomic_ref_int_ptr__false__false_)
                                .fetch_add(
                                    (((new_val).clone() - (old_val).clone()) as () as ()),
                                    Some(unimplemented!()),
                                )
                        });
                    },
                )));
            let load: Value<FnPtr<fn(Ptr<()>) -> Ptr<()>>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>) -> Ptr<()>>::new(
                    |x: Ptr<()>| -> Ptr<()> {
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
            let mut x: () = 1_();
            let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            {
                let mut y: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                const _: () = assert!(true, "");;
            }
            {
                let mut y: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                const _: () = assert!(true, "");;
            }
        }
        {
            let fetch_add: Value<FnPtr<fn(Ptr<()>, (), ())>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, (), ())>::new(
                    |x: Ptr<()>, mut old_val: (), mut new_val: ()| {
                        unimplemented!();
                    },
                )));
            let load: Value<FnPtr<fn(Ptr<()>) -> ()>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>) -> ()>::new(
                    |x: Ptr<()>| -> () {
                        {
                            return unimplemented!();
                        }
                    },
                )));
            ({
                test_acquire_release_59(
                    (*fetch_add.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
        }
        {
            let fetch_add_no_arg: Value<FnPtr<fn(Ptr<()>, (), ())>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, (), ())>::new(
                    |x: Ptr<()>, mut old_val: (), mut new_val: ()| {
                        unimplemented!();
                    },
                )));
            let fetch_add_with_order: Value<FnPtr<fn(Ptr<()>, (), ())>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, (), ())>::new(
                    |x: Ptr<()>, mut old_val: (), mut new_val: ()| {
                        unimplemented!();
                    },
                )));
            let load: Value<FnPtr<fn(Ptr<()>) -> ()>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>) -> ()>::new(
                    |x: Ptr<()>| -> () {
                        {
                            return unimplemented!();
                        }
                    },
                )));
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
            let mut x: () = 1_();
            let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            {
                let mut y: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                const _: () = assert!(true, "");;
            }
            {
                let mut y: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                const _: () = assert!(true, "");;
            }
        }
        {
            let fetch_add: Value<FnPtr<fn(Ptr<()>, (), ())>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, (), ())>::new(
                    |x: Ptr<()>, mut old_val: (), mut new_val: ()| {
                        unimplemented!();
                    },
                )));
            let load: Value<FnPtr<fn(Ptr<()>) -> ()>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>) -> ()>::new(
                    |x: Ptr<()>| -> () {
                        {
                            return unimplemented!();
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
            let fetch_add_no_arg: Value<FnPtr<fn(Ptr<()>, (), ())>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, (), ())>::new(
                    |x: Ptr<()>, mut old_val: (), mut new_val: ()| {
                        unimplemented!();
                    },
                )));
            let fetch_add_with_order: Value<FnPtr<fn(Ptr<()>, (), ())>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, (), ())>::new(
                    |x: Ptr<()>, mut old_val: (), mut new_val: ()| {
                        unimplemented!();
                    },
                )));
            let load: Value<FnPtr<fn(Ptr<()>) -> ()>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>) -> ()>::new(
                    |x: Ptr<()>| -> () {
                        {
                            return unimplemented!();
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
            let mut x: () = 1_();
            let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            {
                let mut y: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                const _: () = assert!(true, "");;
            }
            {
                let mut y: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                const _: () = assert!(true, "");;
            }
        }
        {
            let fetch_add: Value<FnPtr<fn(Ptr<()>, (), ())>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, (), ())>::new(
                    |x: Ptr<()>, mut old_val: (), mut new_val: ()| {
                        unimplemented!();
                    },
                )));
            let load: Value<FnPtr<fn(Ptr<()>) -> ()>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>) -> ()>::new(
                    |x: Ptr<()>| -> () {
                        {
                            return unimplemented!();
                        }
                    },
                )));
            ({
                test_acquire_release_55(
                    (*fetch_add.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
        }
        {
            let fetch_add_no_arg: Value<FnPtr<fn(Ptr<()>, (), ())>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, (), ())>::new(
                    |x: Ptr<()>, mut old_val: (), mut new_val: ()| {
                        unimplemented!();
                    },
                )));
            let fetch_add_with_order: Value<FnPtr<fn(Ptr<()>, (), ())>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, (), ())>::new(
                    |x: Ptr<()>, mut old_val: (), mut new_val: ()| {
                        unimplemented!();
                    },
                )));
            let load: Value<FnPtr<fn(Ptr<()>) -> ()>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>) -> ()>::new(
                    |x: Ptr<()>| -> () {
                        {
                            return unimplemented!();
                        }
                    },
                )));
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
            let mut x: () = 1_();
            let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            {
                let mut y: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                const _: () = assert!(true, "");;
            }
            {
                let mut y: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                const _: () = assert!(true, "");;
            }
        }
        {
            let fetch_add: Value<FnPtr<fn(Ptr<()>, (), ())>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, (), ())>::new(
                    |x: Ptr<()>, mut old_val: (), mut new_val: ()| {
                        unimplemented!();
                    },
                )));
            let load: Value<FnPtr<fn(Ptr<()>) -> ()>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>) -> ()>::new(
                    |x: Ptr<()>| -> () {
                        {
                            return unimplemented!();
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
            let fetch_add_no_arg: Value<FnPtr<fn(Ptr<()>, (), ())>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, (), ())>::new(
                    |x: Ptr<()>, mut old_val: (), mut new_val: ()| {
                        unimplemented!();
                    },
                )));
            let fetch_add_with_order: Value<FnPtr<fn(Ptr<()>, (), ())>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, (), ())>::new(
                    |x: Ptr<()>, mut old_val: (), mut new_val: ()| {
                        unimplemented!();
                    },
                )));
            let load: Value<FnPtr<fn(Ptr<()>) -> ()>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>) -> ()>::new(
                    |x: Ptr<()>| -> () {
                        {
                            return unimplemented!();
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
            let mut x: () = 1_();
            let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            {
                let mut y: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                const _: () = assert!(true, "");;
            }
            {
                let mut y: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                const _: () = assert!(true, "");;
            }
        }
        {
            let fetch_add: Value<FnPtr<fn(Ptr<()>, (), ())>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, (), ())>::new(
                    |x: Ptr<()>, mut old_val: (), mut new_val: ()| {
                        unimplemented!();
                    },
                )));
            let load: Value<FnPtr<fn(Ptr<()>) -> ()>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>) -> ()>::new(
                    |x: Ptr<()>| -> () {
                        {
                            return unimplemented!();
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
            let fetch_add_no_arg: Value<FnPtr<fn(Ptr<()>, (), ())>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, (), ())>::new(
                    |x: Ptr<()>, mut old_val: (), mut new_val: ()| {
                        unimplemented!();
                    },
                )));
            let fetch_add_with_order: Value<FnPtr<fn(Ptr<()>, (), ())>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, (), ())>::new(
                    |x: Ptr<()>, mut old_val: (), mut new_val: ()| {
                        unimplemented!();
                    },
                )));
            let load: Value<FnPtr<fn(Ptr<()>) -> ()>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>) -> ()>::new(
                    |x: Ptr<()>| -> () {
                        {
                            return unimplemented!();
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
            let mut x: () = 1_();
            let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            {
                let mut y: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                const _: () = assert!(true, "");;
            }
            {
                let mut y: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                const _: () = assert!(true, "");;
            }
        }
        {
            let fetch_add: Value<FnPtr<fn(Ptr<()>, (), ())>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, (), ())>::new(
                    |x: Ptr<()>, mut old_val: (), mut new_val: ()| {
                        unimplemented!();
                    },
                )));
            let load: Value<FnPtr<fn(Ptr<()>) -> ()>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>) -> ()>::new(
                    |x: Ptr<()>| -> () {
                        {
                            return unimplemented!();
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
            let fetch_add_no_arg: Value<FnPtr<fn(Ptr<()>, (), ())>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, (), ())>::new(
                    |x: Ptr<()>, mut old_val: (), mut new_val: ()| {
                        unimplemented!();
                    },
                )));
            let fetch_add_with_order: Value<FnPtr<fn(Ptr<()>, (), ())>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, (), ())>::new(
                    |x: Ptr<()>, mut old_val: (), mut new_val: ()| {
                        unimplemented!();
                    },
                )));
            let load: Value<FnPtr<fn(Ptr<()>) -> ()>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>) -> ()>::new(
                    |x: Ptr<()>| -> () {
                        {
                            return unimplemented!();
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
            let mut x: () = 1_();
            let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            {
                let mut y: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                const _: () = assert!(true, "");;
            }
            {
                let mut y: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                const _: () = assert!(true, "");;
            }
        }
        {
            let fetch_add: Value<FnPtr<fn(Ptr<()>, (), ())>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, (), ())>::new(
                    |x: Ptr<()>, mut old_val: (), mut new_val: ()| {
                        unimplemented!();
                    },
                )));
            let load: Value<FnPtr<fn(Ptr<()>) -> ()>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>) -> ()>::new(
                    |x: Ptr<()>| -> () {
                        {
                            return unimplemented!();
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
            let fetch_add_no_arg: Value<FnPtr<fn(Ptr<()>, (), ())>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, (), ())>::new(
                    |x: Ptr<()>, mut old_val: (), mut new_val: ()| {
                        unimplemented!();
                    },
                )));
            let fetch_add_with_order: Value<FnPtr<fn(Ptr<()>, (), ())>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, (), ())>::new(
                    |x: Ptr<()>, mut old_val: (), mut new_val: ()| {
                        unimplemented!();
                    },
                )));
            let load: Value<FnPtr<fn(Ptr<()>) -> ()>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>) -> ()>::new(
                    |x: Ptr<()>| -> () {
                        {
                            return unimplemented!();
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
            let mut x: () = 1_();
            let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            {
                let mut y: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                const _: () = assert!(true, "");;
            }
            {
                let mut y: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                const _: () = assert!(true, "");;
            }
        }
        {
            let fetch_add: Value<FnPtr<fn(Ptr<()>, (), ())>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, (), ())>::new(
                    |x: Ptr<()>, mut old_val: (), mut new_val: ()| {
                        unimplemented!();
                    },
                )));
            let load: Value<FnPtr<fn(Ptr<()>) -> ()>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>) -> ()>::new(
                    |x: Ptr<()>| -> () {
                        {
                            return unimplemented!();
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
            let fetch_add_no_arg: Value<FnPtr<fn(Ptr<()>, (), ())>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, (), ())>::new(
                    |x: Ptr<()>, mut old_val: (), mut new_val: ()| {
                        unimplemented!();
                    },
                )));
            let fetch_add_with_order: Value<FnPtr<fn(Ptr<()>, (), ())>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, (), ())>::new(
                    |x: Ptr<()>, mut old_val: (), mut new_val: ()| {
                        unimplemented!();
                    },
                )));
            let load: Value<FnPtr<fn(Ptr<()>) -> ()>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>) -> ()>::new(
                    |x: Ptr<()>| -> () {
                        {
                            return unimplemented!();
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
            let mut x: () = 1_();
            let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            {
                let mut y: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                const _: () = assert!(true, "");;
            }
            {
                let mut y: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                const _: () = assert!(true, "");;
            }
        }
        {
            let fetch_add: Value<FnPtr<fn(Ptr<()>, (), ())>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, (), ())>::new(
                    |x: Ptr<()>, mut old_val: (), mut new_val: ()| {
                        unimplemented!();
                    },
                )));
            let load: Value<FnPtr<fn(Ptr<()>) -> ()>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>) -> ()>::new(
                    |x: Ptr<()>| -> () {
                        {
                            return unimplemented!();
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
            let fetch_add_no_arg: Value<FnPtr<fn(Ptr<()>, (), ())>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, (), ())>::new(
                    |x: Ptr<()>, mut old_val: (), mut new_val: ()| {
                        unimplemented!();
                    },
                )));
            let fetch_add_with_order: Value<FnPtr<fn(Ptr<()>, (), ())>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, (), ())>::new(
                    |x: Ptr<()>, mut old_val: (), mut new_val: ()| {
                        unimplemented!();
                    },
                )));
            let load: Value<FnPtr<fn(Ptr<()>) -> ()>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>) -> ()>::new(
                    |x: Ptr<()>| -> () {
                        {
                            return unimplemented!();
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
            let mut x: () = 1_();
            let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            {
                let mut y: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                const _: () = assert!(true, "");;
            }
            {
                let mut y: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                const _: () = assert!(true, "");;
            }
        }
        {
            let fetch_add: Value<FnPtr<fn(Ptr<()>, (), ())>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, (), ())>::new(
                    |x: Ptr<()>, mut old_val: (), mut new_val: ()| {
                        unimplemented!();
                    },
                )));
            let load: Value<FnPtr<fn(Ptr<()>) -> ()>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>) -> ()>::new(
                    |x: Ptr<()>| -> () {
                        {
                            return unimplemented!();
                        }
                    },
                )));
            ({
                test_acquire_release_63(
                    (*fetch_add.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
        }
        {
            let fetch_add_no_arg: Value<FnPtr<fn(Ptr<()>, (), ())>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, (), ())>::new(
                    |x: Ptr<()>, mut old_val: (), mut new_val: ()| {
                        unimplemented!();
                    },
                )));
            let fetch_add_with_order: Value<FnPtr<fn(Ptr<()>, (), ())>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, (), ())>::new(
                    |x: Ptr<()>, mut old_val: (), mut new_val: ()| {
                        unimplemented!();
                    },
                )));
            let load: Value<FnPtr<fn(Ptr<()>) -> ()>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>) -> ()>::new(
                    |x: Ptr<()>| -> () {
                        {
                            return unimplemented!();
                        }
                    },
                )));
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
