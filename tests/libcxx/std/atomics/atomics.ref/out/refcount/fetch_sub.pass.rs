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
impl std::cmp::PartialEq for UserAtomicType {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_12(
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
impl std::cmp::PartialEq for LargeUserAtomicType {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_13(
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
pub struct TestEachIntegralType_TestFetchSub_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestEachPointerType_TestFetchSub_ {}
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
pub struct TestDoesNotHaveFetchSub_bool_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestDoesNotHaveFetchSub_UserAtomicType_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestDoesNotHaveFetchSub_LargeUserAtomicType_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFetchSub_float_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFetchSub_double_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFetchSub_char_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFetchSub_signed_char_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFetchSub_unsigned_char_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFetchSub_short_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFetchSub_unsigned_short_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFetchSub_int_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFetchSub_unsigned_int_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFetchSub_long_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFetchSub_unsigned_long_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFetchSub_long_long_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFetchSub_unsigned_long_long_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFetchSub_wchar_t_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFetchSub_char8_t_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFetchSub_char16_t_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFetchSub_char32_t_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFetchSub_int_ptr_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFetchSub_const_int_ptr_ {}
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
        TestEachIntegralType_TestFetchSub_Impl::operator_call(
            &Rc::new(RefCell::new(<TestEachIntegralType_TestFetchSub_>::default())).as_pointer(),
        )
    });
    ({
        TestFetchSub_float_Impl::operator_call(
            &Rc::new(RefCell::new(<TestFetchSub_float_>::default())).as_pointer(),
        )
    });
    ({
        TestFetchSub_double_Impl::operator_call(
            &Rc::new(RefCell::new(<TestFetchSub_double_>::default())).as_pointer(),
        )
    });
    ({
        TestEachPointerType_TestFetchSub_Impl::operator_call(
            &Rc::new(RefCell::new(<TestEachPointerType_TestFetchSub_>::default())).as_pointer(),
        )
    });
    ({
        TestDoesNotHaveFetchSub_bool_Impl::operator_call(
            &Rc::new(RefCell::new(<TestDoesNotHaveFetchSub_bool_>::default())).as_pointer(),
        )
    });
    ({
        TestDoesNotHaveFetchSub_UserAtomicType_Impl::operator_call(
            &Rc::new(RefCell::new(
                <TestDoesNotHaveFetchSub_UserAtomicType_>::default(),
            ))
            .as_pointer(),
        )
    });
    ({
        TestDoesNotHaveFetchSub_LargeUserAtomicType_Impl::operator_call(
            &Rc::new(RefCell::new(
                <TestDoesNotHaveFetchSub_LargeUserAtomicType_>::default(),
            ))
            .as_pointer(),
        )
    });
    return 0_();
}
pub trait TestDoesNotHaveFetchSub_LargeUserAtomicType_Impl {
    fn operator_call(&self);
}
impl TestDoesNotHaveFetchSub_LargeUserAtomicType_Impl
    for Ptr<TestDoesNotHaveFetchSub_LargeUserAtomicType_>
{
    fn operator_call(&self) {
        const _: () = assert!(!(false), "!has_fetch_sub<std::atomic_ref<T>>");;
    }
}
pub trait TestDoesNotHaveFetchSub_UserAtomicType_Impl {
    fn operator_call(&self);
}
impl TestDoesNotHaveFetchSub_UserAtomicType_Impl for Ptr<TestDoesNotHaveFetchSub_UserAtomicType_> {
    fn operator_call(&self) {
        const _: () = assert!(!(false), "!has_fetch_sub<std::atomic_ref<T>>");;
    }
}
pub trait TestDoesNotHaveFetchSub_bool_Impl {
    fn operator_call(&self);
}
impl TestDoesNotHaveFetchSub_bool_Impl for Ptr<TestDoesNotHaveFetchSub_bool_> {
    fn operator_call(&self) {
        const _: () = assert!(!(false), "!has_fetch_sub<std::atomic_ref<T>>");;
    }
}
pub trait TestEachIntegralType_TestFetchSub_Impl {
    fn operator_call(&self);
}
impl TestEachIntegralType_TestFetchSub_Impl for Ptr<TestEachIntegralType_TestFetchSub_> {
    fn operator_call(&self) {
        ({
            TestFetchSub_char_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFetchSub_char_>::default())).as_pointer(),
            )
        });
        ({
            TestFetchSub_signed_char_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFetchSub_signed_char_>::default())).as_pointer(),
            )
        });
        ({
            TestFetchSub_unsigned_char_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFetchSub_unsigned_char_>::default())).as_pointer(),
            )
        });
        ({
            TestFetchSub_short_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFetchSub_short_>::default())).as_pointer(),
            )
        });
        ({
            TestFetchSub_unsigned_short_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFetchSub_unsigned_short_>::default())).as_pointer(),
            )
        });
        ({
            TestFetchSub_int_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFetchSub_int_>::default())).as_pointer(),
            )
        });
        ({
            TestFetchSub_unsigned_int_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFetchSub_unsigned_int_>::default())).as_pointer(),
            )
        });
        ({
            TestFetchSub_long_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFetchSub_long_>::default())).as_pointer(),
            )
        });
        ({
            TestFetchSub_unsigned_long_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFetchSub_unsigned_long_>::default())).as_pointer(),
            )
        });
        ({
            TestFetchSub_long_long_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFetchSub_long_long_>::default())).as_pointer(),
            )
        });
        ({
            TestFetchSub_unsigned_long_long_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFetchSub_unsigned_long_long_>::default())).as_pointer(),
            )
        });
        ({
            TestFetchSub_wchar_t_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFetchSub_wchar_t_>::default())).as_pointer(),
            )
        });
        ({
            TestFetchSub_char8_t_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFetchSub_char8_t_>::default())).as_pointer(),
            )
        });
        ({
            TestFetchSub_char16_t_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFetchSub_char16_t_>::default())).as_pointer(),
            )
        });
        ({
            TestFetchSub_char32_t_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFetchSub_char32_t_>::default())).as_pointer(),
            )
        });
        ({
            TestFetchSub_signed_char_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFetchSub_signed_char_>::default())).as_pointer(),
            )
        });
        ({
            TestFetchSub_unsigned_char_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFetchSub_unsigned_char_>::default())).as_pointer(),
            )
        });
        ({
            TestFetchSub_short_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFetchSub_short_>::default())).as_pointer(),
            )
        });
        ({
            TestFetchSub_unsigned_short_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFetchSub_unsigned_short_>::default())).as_pointer(),
            )
        });
        ({
            TestFetchSub_int_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFetchSub_int_>::default())).as_pointer(),
            )
        });
        ({
            TestFetchSub_unsigned_int_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFetchSub_unsigned_int_>::default())).as_pointer(),
            )
        });
        ({
            TestFetchSub_long_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFetchSub_long_>::default())).as_pointer(),
            )
        });
        ({
            TestFetchSub_unsigned_long_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFetchSub_unsigned_long_>::default())).as_pointer(),
            )
        });
    }
}
pub trait TestEachPointerType_TestFetchSub_Impl {
    fn operator_call(&self);
}
impl TestEachPointerType_TestFetchSub_Impl for Ptr<TestEachPointerType_TestFetchSub_> {
    fn operator_call(&self) {
        ({
            TestFetchSub_int_ptr_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFetchSub_int_ptr_>::default())).as_pointer(),
            )
        });
        ({
            TestFetchSub_const_int_ptr_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFetchSub_const_int_ptr_>::default())).as_pointer(),
            )
        });
    }
}
pub trait TestFetchSub_char16_t_Impl {
    fn operator_call(&self);
}
impl TestFetchSub_char16_t_Impl for Ptr<TestFetchSub_char16_t_> {
    fn operator_call(&self) {
        {
            let mut x: () = 7_();
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
            let fetch_sub: Value<FnPtr<fn(Ptr<()>, (), ())>> =
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
                    (*fetch_sub.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
        }
        {
            let fetch_sub_no_arg: Value<FnPtr<fn(Ptr<()>, (), ())>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, (), ())>::new(
                    |x: Ptr<()>, mut old_val: (), mut new_val: ()| {
                        unimplemented!();
                    },
                )));
            let fetch_sub_with_order: Value<FnPtr<fn(Ptr<()>, (), ())>> =
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
                    (*fetch_sub_no_arg.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
            ({
                test_seq_cst_41(
                    (*fetch_sub_with_order.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
        }
    }
}
pub trait TestFetchSub_char32_t_Impl {
    fn operator_call(&self);
}
impl TestFetchSub_char32_t_Impl for Ptr<TestFetchSub_char32_t_> {
    fn operator_call(&self) {
        {
            let mut x: () = 7_();
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
            let fetch_sub: Value<FnPtr<fn(Ptr<()>, (), ())>> =
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
                    (*fetch_sub.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
        }
        {
            let fetch_sub_no_arg: Value<FnPtr<fn(Ptr<()>, (), ())>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, (), ())>::new(
                    |x: Ptr<()>, mut old_val: (), mut new_val: ()| {
                        unimplemented!();
                    },
                )));
            let fetch_sub_with_order: Value<FnPtr<fn(Ptr<()>, (), ())>> =
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
                    (*fetch_sub_no_arg.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
            ({
                test_seq_cst_43(
                    (*fetch_sub_with_order.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
        }
    }
}
pub trait TestFetchSub_char8_t_Impl {
    fn operator_call(&self);
}
impl TestFetchSub_char8_t_Impl for Ptr<TestFetchSub_char8_t_> {
    fn operator_call(&self) {
        {
            let mut x: () = 7_();
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
            let fetch_sub: Value<FnPtr<fn(Ptr<()>, (), ())>> =
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
                    (*fetch_sub.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
        }
        {
            let fetch_sub_no_arg: Value<FnPtr<fn(Ptr<()>, (), ())>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, (), ())>::new(
                    |x: Ptr<()>, mut old_val: (), mut new_val: ()| {
                        unimplemented!();
                    },
                )));
            let fetch_sub_with_order: Value<FnPtr<fn(Ptr<()>, (), ())>> =
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
                    (*fetch_sub_no_arg.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
            ({
                test_seq_cst_39(
                    (*fetch_sub_with_order.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
        }
    }
}
pub trait TestFetchSub_char_Impl {
    fn operator_call(&self);
}
impl TestFetchSub_char_Impl for Ptr<TestFetchSub_char_> {
    fn operator_call(&self) {
        {
            let mut x: () = 7_();
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
            let fetch_sub: Value<FnPtr<fn(Ptr<()>, (), ())>> =
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
                    (*fetch_sub.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
        }
        {
            let fetch_sub_no_arg: Value<FnPtr<fn(Ptr<()>, (), ())>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, (), ())>::new(
                    |x: Ptr<()>, mut old_val: (), mut new_val: ()| {
                        unimplemented!();
                    },
                )));
            let fetch_sub_with_order: Value<FnPtr<fn(Ptr<()>, (), ())>> =
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
                    (*fetch_sub_no_arg.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
            ({
                test_seq_cst_15(
                    (*fetch_sub_with_order.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
        }
    }
}
pub trait TestFetchSub_const_int_ptr_Impl {
    fn operator_call(&self);
}
impl TestFetchSub_const_int_ptr_Impl for Ptr<TestFetchSub_const_int_ptr_> {
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
                let mut p: Ptr<()> = ((t.as_pointer() as Ptr<()>).offset(7_()));
                let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                {
                    let mut y: Ptr<()> = unimplemented!();
                    unimplemented!();
                    unimplemented!();
                    const _: () = assert!(true, "");;
                }
                {
                    let mut y: Ptr<()> = unimplemented!();
                    unimplemented!();
                    unimplemented!();
                    const _: () = assert!(true, "");;
                }
            }
        }
        {
            let fetch_sub: Value<FnPtr<fn(Ptr<()>, Ptr<()>, Ptr<()>)>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, Ptr<()>, Ptr<()>)>::new(
                    |x: Ptr<()>, mut old_val: Ptr<()>, mut new_val: Ptr<()>| {
                        unimplemented!();
                    },
                )));
            let load: Value<FnPtr<fn(Ptr<()>) -> Ptr<()>>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>) -> Ptr<()>>::new(
                    |x: Ptr<()>| -> Ptr<()> {
                        {
                            return unimplemented!();
                        }
                    },
                )));
            ({
                test_acquire_release_70(
                    (*fetch_sub.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
        }
        {
            let fetch_sub_no_arg: Value<FnPtr<fn(Ptr<()>, Ptr<()>, Ptr<()>)>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, Ptr<()>, Ptr<()>)>::new(
                    |x: Ptr<()>, mut old_val: Ptr<()>, mut new_val: Ptr<()>| {
                        unimplemented!();
                    },
                )));
            let fetch_sub_with_order: Value<FnPtr<fn(Ptr<()>, Ptr<()>, Ptr<()>)>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, Ptr<()>, Ptr<()>)>::new(
                    |x: Ptr<()>, mut old_val: Ptr<()>, mut new_val: Ptr<()>| {
                        unimplemented!();
                    },
                )));
            let load: Value<FnPtr<fn(Ptr<()>) -> Ptr<()>>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>) -> Ptr<()>>::new(
                    |x: Ptr<()>| -> Ptr<()> {
                        {
                            return unimplemented!();
                        }
                    },
                )));
            ({
                test_seq_cst_50(
                    (*fetch_sub_no_arg.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
            ({
                test_seq_cst_51(
                    (*fetch_sub_with_order.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
        }
    }
}
pub trait TestFetchSub_double_Impl {
    fn operator_call(&self);
}
impl TestFetchSub_double_Impl for Ptr<TestFetchSub_double_> {
    fn operator_call(&self) {
        {
            let mut x: () = 7_();
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
            let fetch_sub: Value<FnPtr<fn(Ptr<()>, (), ())>> =
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
                test_acquire_release_68(
                    (*fetch_sub.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
        }
        {
            let fetch_sub_no_arg: Value<FnPtr<fn(Ptr<()>, (), ())>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, (), ())>::new(
                    |x: Ptr<()>, mut old_val: (), mut new_val: ()| {
                        unimplemented!();
                    },
                )));
            let fetch_sub_with_order: Value<FnPtr<fn(Ptr<()>, (), ())>> =
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
                test_seq_cst_46(
                    (*fetch_sub_no_arg.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
            ({
                test_seq_cst_47(
                    (*fetch_sub_with_order.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
        }
    }
}
pub trait TestFetchSub_float_Impl {
    fn operator_call(&self);
}
impl TestFetchSub_float_Impl for Ptr<TestFetchSub_float_> {
    fn operator_call(&self) {
        {
            let mut x: () = 7_();
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
            let fetch_sub: Value<FnPtr<fn(Ptr<()>, (), ())>> =
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
                test_acquire_release_67(
                    (*fetch_sub.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
        }
        {
            let fetch_sub_no_arg: Value<FnPtr<fn(Ptr<()>, (), ())>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, (), ())>::new(
                    |x: Ptr<()>, mut old_val: (), mut new_val: ()| {
                        unimplemented!();
                    },
                )));
            let fetch_sub_with_order: Value<FnPtr<fn(Ptr<()>, (), ())>> =
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
                test_seq_cst_44(
                    (*fetch_sub_no_arg.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
            ({
                test_seq_cst_45(
                    (*fetch_sub_with_order.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
        }
    }
}
pub trait TestFetchSub_int_Impl {
    fn operator_call(&self);
}
impl TestFetchSub_int_Impl for Ptr<TestFetchSub_int_> {
    fn operator_call(&self) {
        {
            let mut x: () = 7_();
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
            let fetch_sub: Value<FnPtr<fn(Ptr<()>, (), ())>> =
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
                    (*fetch_sub.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
        }
        {
            let fetch_sub_no_arg: Value<FnPtr<fn(Ptr<()>, (), ())>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, (), ())>::new(
                    |x: Ptr<()>, mut old_val: (), mut new_val: ()| {
                        unimplemented!();
                    },
                )));
            let fetch_sub_with_order: Value<FnPtr<fn(Ptr<()>, (), ())>> =
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
                    (*fetch_sub_no_arg.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
            ({
                test_seq_cst_25(
                    (*fetch_sub_with_order.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
        }
    }
}
pub trait TestFetchSub_int_ptr_Impl {
    fn operator_call(&self);
}
impl TestFetchSub_int_ptr_Impl for Ptr<TestFetchSub_int_ptr_> {
    fn operator_call(&self) {
        {
            {
                let t: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
                    <()>::default(),
                    <()>::default(),
                    <()>::default(),
                    <()>::default(),
                    <()>::default(),
                    <()>::default(),
                    <()>::default(),
                    <()>::default(),
                    <()>::default(),
                ])));
                let mut p: Ptr<()> = ((t.as_pointer() as Ptr<()>).offset(7_()));
                let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                {
                    let mut y: Ptr<()> = unimplemented!();
                    unimplemented!();
                    unimplemented!();
                    const _: () = assert!(true, "");;
                }
                {
                    let mut y: Ptr<()> = unimplemented!();
                    unimplemented!();
                    unimplemented!();
                    const _: () = assert!(true, "");;
                }
            }
        }
        {
            let fetch_sub: Value<FnPtr<fn(Ptr<()>, Ptr<()>, Ptr<()>)>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, Ptr<()>, Ptr<()>)>::new(
                    |x: Ptr<()>, mut old_val: Ptr<()>, mut new_val: Ptr<()>| {
                        unimplemented!();
                    },
                )));
            let load: Value<FnPtr<fn(Ptr<()>) -> Ptr<()>>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>) -> Ptr<()>>::new(
                    |x: Ptr<()>| -> Ptr<()> {
                        {
                            return unimplemented!();
                        }
                    },
                )));
            ({
                test_acquire_release_69(
                    (*fetch_sub.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
        }
        {
            let fetch_sub_no_arg: Value<FnPtr<fn(Ptr<()>, Ptr<()>, Ptr<()>)>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, Ptr<()>, Ptr<()>)>::new(
                    |x: Ptr<()>, mut old_val: Ptr<()>, mut new_val: Ptr<()>| {
                        unimplemented!();
                    },
                )));
            let fetch_sub_with_order: Value<FnPtr<fn(Ptr<()>, Ptr<()>, Ptr<()>)>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, Ptr<()>, Ptr<()>)>::new(
                    |x: Ptr<()>, mut old_val: Ptr<()>, mut new_val: Ptr<()>| {
                        unimplemented!();
                    },
                )));
            let load: Value<FnPtr<fn(Ptr<()>) -> Ptr<()>>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>) -> Ptr<()>>::new(
                    |x: Ptr<()>| -> Ptr<()> {
                        {
                            return unimplemented!();
                        }
                    },
                )));
            ({
                test_seq_cst_48(
                    (*fetch_sub_no_arg.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
            ({
                test_seq_cst_49(
                    (*fetch_sub_with_order.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
        }
    }
}
pub trait TestFetchSub_long_Impl {
    fn operator_call(&self);
}
impl TestFetchSub_long_Impl for Ptr<TestFetchSub_long_> {
    fn operator_call(&self) {
        {
            let mut x: () = 7_();
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
            let fetch_sub: Value<FnPtr<fn(Ptr<()>, (), ())>> =
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
                    (*fetch_sub.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
        }
        {
            let fetch_sub_no_arg: Value<FnPtr<fn(Ptr<()>, (), ())>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, (), ())>::new(
                    |x: Ptr<()>, mut old_val: (), mut new_val: ()| {
                        unimplemented!();
                    },
                )));
            let fetch_sub_with_order: Value<FnPtr<fn(Ptr<()>, (), ())>> =
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
                    (*fetch_sub_no_arg.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
            ({
                test_seq_cst_29(
                    (*fetch_sub_with_order.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
        }
    }
}
pub trait TestFetchSub_long_long_Impl {
    fn operator_call(&self);
}
impl TestFetchSub_long_long_Impl for Ptr<TestFetchSub_long_long_> {
    fn operator_call(&self) {
        {
            let mut x: () = 7_();
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
            let fetch_sub: Value<FnPtr<fn(Ptr<()>, (), ())>> =
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
                    (*fetch_sub.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
        }
        {
            let fetch_sub_no_arg: Value<FnPtr<fn(Ptr<()>, (), ())>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, (), ())>::new(
                    |x: Ptr<()>, mut old_val: (), mut new_val: ()| {
                        unimplemented!();
                    },
                )));
            let fetch_sub_with_order: Value<FnPtr<fn(Ptr<()>, (), ())>> =
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
                    (*fetch_sub_no_arg.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
            ({
                test_seq_cst_33(
                    (*fetch_sub_with_order.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
        }
    }
}
pub trait TestFetchSub_short_Impl {
    fn operator_call(&self);
}
impl TestFetchSub_short_Impl for Ptr<TestFetchSub_short_> {
    fn operator_call(&self) {
        {
            let mut x: () = 7_();
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
            let fetch_sub: Value<FnPtr<fn(Ptr<()>, (), ())>> =
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
                    (*fetch_sub.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
        }
        {
            let fetch_sub_no_arg: Value<FnPtr<fn(Ptr<()>, (), ())>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, (), ())>::new(
                    |x: Ptr<()>, mut old_val: (), mut new_val: ()| {
                        unimplemented!();
                    },
                )));
            let fetch_sub_with_order: Value<FnPtr<fn(Ptr<()>, (), ())>> =
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
                    (*fetch_sub_no_arg.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
            ({
                test_seq_cst_21(
                    (*fetch_sub_with_order.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
        }
    }
}
pub trait TestFetchSub_signed_char_Impl {
    fn operator_call(&self);
}
impl TestFetchSub_signed_char_Impl for Ptr<TestFetchSub_signed_char_> {
    fn operator_call(&self) {
        {
            let mut x: () = 7_();
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
            let fetch_sub: Value<FnPtr<fn(Ptr<()>, (), ())>> =
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
                    (*fetch_sub.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
        }
        {
            let fetch_sub_no_arg: Value<FnPtr<fn(Ptr<()>, (), ())>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, (), ())>::new(
                    |x: Ptr<()>, mut old_val: (), mut new_val: ()| {
                        unimplemented!();
                    },
                )));
            let fetch_sub_with_order: Value<FnPtr<fn(Ptr<()>, (), ())>> =
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
                    (*fetch_sub_no_arg.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
            ({
                test_seq_cst_17(
                    (*fetch_sub_with_order.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
        }
    }
}
pub trait TestFetchSub_unsigned_char_Impl {
    fn operator_call(&self);
}
impl TestFetchSub_unsigned_char_Impl for Ptr<TestFetchSub_unsigned_char_> {
    fn operator_call(&self) {
        {
            let mut x: () = 7_();
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
            let fetch_sub: Value<FnPtr<fn(Ptr<()>, (), ())>> =
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
                    (*fetch_sub.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
        }
        {
            let fetch_sub_no_arg: Value<FnPtr<fn(Ptr<()>, (), ())>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, (), ())>::new(
                    |x: Ptr<()>, mut old_val: (), mut new_val: ()| {
                        unimplemented!();
                    },
                )));
            let fetch_sub_with_order: Value<FnPtr<fn(Ptr<()>, (), ())>> =
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
                    (*fetch_sub_no_arg.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
            ({
                test_seq_cst_19(
                    (*fetch_sub_with_order.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
        }
    }
}
pub trait TestFetchSub_unsigned_int_Impl {
    fn operator_call(&self);
}
impl TestFetchSub_unsigned_int_Impl for Ptr<TestFetchSub_unsigned_int_> {
    fn operator_call(&self) {
        {
            let mut x: () = 7_();
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
            let fetch_sub: Value<FnPtr<fn(Ptr<()>, (), ())>> =
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
                    (*fetch_sub.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
        }
        {
            let fetch_sub_no_arg: Value<FnPtr<fn(Ptr<()>, (), ())>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, (), ())>::new(
                    |x: Ptr<()>, mut old_val: (), mut new_val: ()| {
                        unimplemented!();
                    },
                )));
            let fetch_sub_with_order: Value<FnPtr<fn(Ptr<()>, (), ())>> =
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
                    (*fetch_sub_no_arg.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
            ({
                test_seq_cst_27(
                    (*fetch_sub_with_order.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
        }
    }
}
pub trait TestFetchSub_unsigned_long_Impl {
    fn operator_call(&self);
}
impl TestFetchSub_unsigned_long_Impl for Ptr<TestFetchSub_unsigned_long_> {
    fn operator_call(&self) {
        {
            let mut x: () = 7_();
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
            let fetch_sub: Value<FnPtr<fn(Ptr<()>, (), ())>> =
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
                    (*fetch_sub.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
        }
        {
            let fetch_sub_no_arg: Value<FnPtr<fn(Ptr<()>, (), ())>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, (), ())>::new(
                    |x: Ptr<()>, mut old_val: (), mut new_val: ()| {
                        unimplemented!();
                    },
                )));
            let fetch_sub_with_order: Value<FnPtr<fn(Ptr<()>, (), ())>> =
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
                    (*fetch_sub_no_arg.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
            ({
                test_seq_cst_31(
                    (*fetch_sub_with_order.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
        }
    }
}
pub trait TestFetchSub_unsigned_long_long_Impl {
    fn operator_call(&self);
}
impl TestFetchSub_unsigned_long_long_Impl for Ptr<TestFetchSub_unsigned_long_long_> {
    fn operator_call(&self) {
        {
            let mut x: () = 7_();
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
            let fetch_sub: Value<FnPtr<fn(Ptr<()>, (), ())>> =
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
                    (*fetch_sub.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
        }
        {
            let fetch_sub_no_arg: Value<FnPtr<fn(Ptr<()>, (), ())>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, (), ())>::new(
                    |x: Ptr<()>, mut old_val: (), mut new_val: ()| {
                        unimplemented!();
                    },
                )));
            let fetch_sub_with_order: Value<FnPtr<fn(Ptr<()>, (), ())>> =
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
                    (*fetch_sub_no_arg.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
            ({
                test_seq_cst_35(
                    (*fetch_sub_with_order.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
        }
    }
}
pub trait TestFetchSub_unsigned_short_Impl {
    fn operator_call(&self);
}
impl TestFetchSub_unsigned_short_Impl for Ptr<TestFetchSub_unsigned_short_> {
    fn operator_call(&self) {
        {
            let mut x: () = 7_();
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
            let fetch_sub: Value<FnPtr<fn(Ptr<()>, (), ())>> =
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
                    (*fetch_sub.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
        }
        {
            let fetch_sub_no_arg: Value<FnPtr<fn(Ptr<()>, (), ())>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, (), ())>::new(
                    |x: Ptr<()>, mut old_val: (), mut new_val: ()| {
                        unimplemented!();
                    },
                )));
            let fetch_sub_with_order: Value<FnPtr<fn(Ptr<()>, (), ())>> =
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
                    (*fetch_sub_no_arg.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
            ({
                test_seq_cst_23(
                    (*fetch_sub_with_order.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
        }
    }
}
pub trait TestFetchSub_wchar_t_Impl {
    fn operator_call(&self);
}
impl TestFetchSub_wchar_t_Impl for Ptr<TestFetchSub_wchar_t_> {
    fn operator_call(&self) {
        {
            let mut x: () = 7_();
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
            let fetch_sub: Value<FnPtr<fn(Ptr<()>, (), ())>> =
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
                    (*fetch_sub.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
        }
        {
            let fetch_sub_no_arg: Value<FnPtr<fn(Ptr<()>, (), ())>> =
                Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, (), ())>::new(
                    |x: Ptr<()>, mut old_val: (), mut new_val: ()| {
                        unimplemented!();
                    },
                )));
            let fetch_sub_with_order: Value<FnPtr<fn(Ptr<()>, (), ())>> =
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
                    (*fetch_sub_no_arg.borrow()).copy_from(),
                    (*load.borrow()).copy_from(),
                )
            });
            ({
                test_seq_cst_37(
                    (*fetch_sub_with_order.borrow()).copy_from(),
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
