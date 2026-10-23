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
pub struct TestEachIntegralType_TestFetchOr_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestEachFloatingPointType_TestDoesNotHaveFetchOr_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestEachPointerType_TestDoesNotHaveFetchOr_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestDoesNotHaveFetchOr_bool_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestDoesNotHaveFetchOr_UserAtomicType_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestDoesNotHaveFetchOr_LargeUserAtomicType_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestDoesNotHaveFetchOr_float_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestDoesNotHaveFetchOr_double_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestDoesNotHaveFetchOr_long_double_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestDoesNotHaveFetchOr_int_ptr_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestDoesNotHaveFetchOr_const_int_ptr_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFetchOr_char_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFetchOr_signed_char_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFetchOr_unsigned_char_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFetchOr_short_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFetchOr_unsigned_short_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFetchOr_int_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFetchOr_unsigned_int_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFetchOr_long_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFetchOr_unsigned_long_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFetchOr_long_long_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFetchOr_unsigned_long_long_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFetchOr_wchar_t_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFetchOr_char8_t_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFetchOr_char16_t_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFetchOr_char32_t_ {}
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
        TestEachIntegralType_TestFetchOr_Impl::operator_call(
            &Rc::new(RefCell::new(<TestEachIntegralType_TestFetchOr_>::default())).as_pointer(),
        )
    });
    ({
        TestEachFloatingPointType_TestDoesNotHaveFetchOr_Impl::operator_call(
            &Rc::new(RefCell::new(
                <TestEachFloatingPointType_TestDoesNotHaveFetchOr_>::default(),
            ))
            .as_pointer(),
        )
    });
    ({
        TestEachPointerType_TestDoesNotHaveFetchOr_Impl::operator_call(
            &Rc::new(RefCell::new(
                <TestEachPointerType_TestDoesNotHaveFetchOr_>::default(),
            ))
            .as_pointer(),
        )
    });
    ({
        TestDoesNotHaveFetchOr_bool_Impl::operator_call(
            &Rc::new(RefCell::new(<TestDoesNotHaveFetchOr_bool_>::default())).as_pointer(),
        )
    });
    ({
        TestDoesNotHaveFetchOr_UserAtomicType_Impl::operator_call(
            &Rc::new(RefCell::new(
                <TestDoesNotHaveFetchOr_UserAtomicType_>::default(),
            ))
            .as_pointer(),
        )
    });
    ({
        TestDoesNotHaveFetchOr_LargeUserAtomicType_Impl::operator_call(
            &Rc::new(RefCell::new(
                <TestDoesNotHaveFetchOr_LargeUserAtomicType_>::default(),
            ))
            .as_pointer(),
        )
    });
    return 0_();
}
pub trait TestDoesNotHaveFetchOr_LargeUserAtomicType_Impl {
    fn operator_call(&self);
}
impl TestDoesNotHaveFetchOr_LargeUserAtomicType_Impl
    for Ptr<TestDoesNotHaveFetchOr_LargeUserAtomicType_>
{
    fn operator_call(&self) {
        const _: () = assert!(!(false), "!has_fetch_or<std::atomic_ref<T>>");;
    }
}
pub trait TestDoesNotHaveFetchOr_UserAtomicType_Impl {
    fn operator_call(&self);
}
impl TestDoesNotHaveFetchOr_UserAtomicType_Impl for Ptr<TestDoesNotHaveFetchOr_UserAtomicType_> {
    fn operator_call(&self) {
        const _: () = assert!(!(false), "!has_fetch_or<std::atomic_ref<T>>");;
    }
}
pub trait TestDoesNotHaveFetchOr_bool_Impl {
    fn operator_call(&self);
}
impl TestDoesNotHaveFetchOr_bool_Impl for Ptr<TestDoesNotHaveFetchOr_bool_> {
    fn operator_call(&self) {
        const _: () = assert!(!(false), "!has_fetch_or<std::atomic_ref<T>>");;
    }
}
pub trait TestDoesNotHaveFetchOr_const_int_ptr_Impl {
    fn operator_call(&self);
}
impl TestDoesNotHaveFetchOr_const_int_ptr_Impl for Ptr<TestDoesNotHaveFetchOr_const_int_ptr_> {
    fn operator_call(&self) {
        const _: () = assert!(!(false), "!has_fetch_or<std::atomic_ref<T>>");;
    }
}
pub trait TestDoesNotHaveFetchOr_double_Impl {
    fn operator_call(&self);
}
impl TestDoesNotHaveFetchOr_double_Impl for Ptr<TestDoesNotHaveFetchOr_double_> {
    fn operator_call(&self) {
        const _: () = assert!(!(false), "!has_fetch_or<std::atomic_ref<T>>");;
    }
}
pub trait TestDoesNotHaveFetchOr_float_Impl {
    fn operator_call(&self);
}
impl TestDoesNotHaveFetchOr_float_Impl for Ptr<TestDoesNotHaveFetchOr_float_> {
    fn operator_call(&self) {
        const _: () = assert!(!(false), "!has_fetch_or<std::atomic_ref<T>>");;
    }
}
pub trait TestDoesNotHaveFetchOr_int_ptr_Impl {
    fn operator_call(&self);
}
impl TestDoesNotHaveFetchOr_int_ptr_Impl for Ptr<TestDoesNotHaveFetchOr_int_ptr_> {
    fn operator_call(&self) {
        const _: () = assert!(!(false), "!has_fetch_or<std::atomic_ref<T>>");;
    }
}
pub trait TestDoesNotHaveFetchOr_long_double_Impl {
    fn operator_call(&self);
}
impl TestDoesNotHaveFetchOr_long_double_Impl for Ptr<TestDoesNotHaveFetchOr_long_double_> {
    fn operator_call(&self) {
        const _: () = assert!(!(false), "!has_fetch_or<std::atomic_ref<T>>");;
    }
}
pub trait TestEachFloatingPointType_TestDoesNotHaveFetchOr_Impl {
    fn operator_call(&self);
}
impl TestEachFloatingPointType_TestDoesNotHaveFetchOr_Impl
    for Ptr<TestEachFloatingPointType_TestDoesNotHaveFetchOr_>
{
    fn operator_call(&self) {
        ({
            TestDoesNotHaveFetchOr_float_Impl::operator_call(
                &Rc::new(RefCell::new(<TestDoesNotHaveFetchOr_float_>::default())).as_pointer(),
            )
        });
        ({
            TestDoesNotHaveFetchOr_double_Impl::operator_call(
                &Rc::new(RefCell::new(<TestDoesNotHaveFetchOr_double_>::default())).as_pointer(),
            )
        });
        ({
            TestDoesNotHaveFetchOr_long_double_Impl::operator_call(
                &Rc::new(RefCell::new(
                    <TestDoesNotHaveFetchOr_long_double_>::default(),
                ))
                .as_pointer(),
            )
        });
    }
}
pub trait TestEachIntegralType_TestFetchOr_Impl {
    fn operator_call(&self);
}
impl TestEachIntegralType_TestFetchOr_Impl for Ptr<TestEachIntegralType_TestFetchOr_> {
    fn operator_call(&self) {
        ({
            TestFetchOr_char_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFetchOr_char_>::default())).as_pointer(),
            )
        });
        ({
            TestFetchOr_signed_char_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFetchOr_signed_char_>::default())).as_pointer(),
            )
        });
        ({
            TestFetchOr_unsigned_char_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFetchOr_unsigned_char_>::default())).as_pointer(),
            )
        });
        ({
            TestFetchOr_short_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFetchOr_short_>::default())).as_pointer(),
            )
        });
        ({
            TestFetchOr_unsigned_short_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFetchOr_unsigned_short_>::default())).as_pointer(),
            )
        });
        ({
            TestFetchOr_int_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFetchOr_int_>::default())).as_pointer(),
            )
        });
        ({
            TestFetchOr_unsigned_int_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFetchOr_unsigned_int_>::default())).as_pointer(),
            )
        });
        ({
            TestFetchOr_long_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFetchOr_long_>::default())).as_pointer(),
            )
        });
        ({
            TestFetchOr_unsigned_long_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFetchOr_unsigned_long_>::default())).as_pointer(),
            )
        });
        ({
            TestFetchOr_long_long_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFetchOr_long_long_>::default())).as_pointer(),
            )
        });
        ({
            TestFetchOr_unsigned_long_long_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFetchOr_unsigned_long_long_>::default())).as_pointer(),
            )
        });
        ({
            TestFetchOr_wchar_t_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFetchOr_wchar_t_>::default())).as_pointer(),
            )
        });
        ({
            TestFetchOr_char8_t_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFetchOr_char8_t_>::default())).as_pointer(),
            )
        });
        ({
            TestFetchOr_char16_t_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFetchOr_char16_t_>::default())).as_pointer(),
            )
        });
        ({
            TestFetchOr_char32_t_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFetchOr_char32_t_>::default())).as_pointer(),
            )
        });
        ({
            TestFetchOr_signed_char_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFetchOr_signed_char_>::default())).as_pointer(),
            )
        });
        ({
            TestFetchOr_unsigned_char_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFetchOr_unsigned_char_>::default())).as_pointer(),
            )
        });
        ({
            TestFetchOr_short_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFetchOr_short_>::default())).as_pointer(),
            )
        });
        ({
            TestFetchOr_unsigned_short_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFetchOr_unsigned_short_>::default())).as_pointer(),
            )
        });
        ({
            TestFetchOr_int_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFetchOr_int_>::default())).as_pointer(),
            )
        });
        ({
            TestFetchOr_unsigned_int_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFetchOr_unsigned_int_>::default())).as_pointer(),
            )
        });
        ({
            TestFetchOr_long_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFetchOr_long_>::default())).as_pointer(),
            )
        });
        ({
            TestFetchOr_unsigned_long_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFetchOr_unsigned_long_>::default())).as_pointer(),
            )
        });
    }
}
pub trait TestEachPointerType_TestDoesNotHaveFetchOr_Impl {
    fn operator_call(&self);
}
impl TestEachPointerType_TestDoesNotHaveFetchOr_Impl
    for Ptr<TestEachPointerType_TestDoesNotHaveFetchOr_>
{
    fn operator_call(&self) {
        ({
            TestDoesNotHaveFetchOr_int_ptr_Impl::operator_call(
                &Rc::new(RefCell::new(<TestDoesNotHaveFetchOr_int_ptr_>::default())).as_pointer(),
            )
        });
        ({
            TestDoesNotHaveFetchOr_const_int_ptr_Impl::operator_call(
                &Rc::new(RefCell::new(
                    <TestDoesNotHaveFetchOr_const_int_ptr_>::default(),
                ))
                .as_pointer(),
            )
        });
    }
}
pub trait TestFetchOr_char16_t_Impl {
    fn operator_call(&self);
}
impl TestFetchOr_char16_t_Impl for Ptr<TestFetchOr_char16_t_> {
    fn operator_call(&self) {
        const _: () = assert!(true, "std::is_integral_v<T>");;
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
}
pub trait TestFetchOr_char32_t_Impl {
    fn operator_call(&self);
}
impl TestFetchOr_char32_t_Impl for Ptr<TestFetchOr_char32_t_> {
    fn operator_call(&self) {
        const _: () = assert!(true, "std::is_integral_v<T>");;
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
}
pub trait TestFetchOr_char8_t_Impl {
    fn operator_call(&self);
}
impl TestFetchOr_char8_t_Impl for Ptr<TestFetchOr_char8_t_> {
    fn operator_call(&self) {
        const _: () = assert!(true, "std::is_integral_v<T>");;
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
}
pub trait TestFetchOr_char_Impl {
    fn operator_call(&self);
}
impl TestFetchOr_char_Impl for Ptr<TestFetchOr_char_> {
    fn operator_call(&self) {
        const _: () = assert!(true, "std::is_integral_v<T>");;
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
}
pub trait TestFetchOr_int_Impl {
    fn operator_call(&self);
}
impl TestFetchOr_int_Impl for Ptr<TestFetchOr_int_> {
    fn operator_call(&self) {
        const _: () = assert!(true, "std::is_integral_v<T>");;
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
}
pub trait TestFetchOr_long_Impl {
    fn operator_call(&self);
}
impl TestFetchOr_long_Impl for Ptr<TestFetchOr_long_> {
    fn operator_call(&self) {
        const _: () = assert!(true, "std::is_integral_v<T>");;
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
}
pub trait TestFetchOr_long_long_Impl {
    fn operator_call(&self);
}
impl TestFetchOr_long_long_Impl for Ptr<TestFetchOr_long_long_> {
    fn operator_call(&self) {
        const _: () = assert!(true, "std::is_integral_v<T>");;
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
}
pub trait TestFetchOr_short_Impl {
    fn operator_call(&self);
}
impl TestFetchOr_short_Impl for Ptr<TestFetchOr_short_> {
    fn operator_call(&self) {
        const _: () = assert!(true, "std::is_integral_v<T>");;
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
}
pub trait TestFetchOr_signed_char_Impl {
    fn operator_call(&self);
}
impl TestFetchOr_signed_char_Impl for Ptr<TestFetchOr_signed_char_> {
    fn operator_call(&self) {
        const _: () = assert!(true, "std::is_integral_v<T>");;
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
}
pub trait TestFetchOr_unsigned_char_Impl {
    fn operator_call(&self);
}
impl TestFetchOr_unsigned_char_Impl for Ptr<TestFetchOr_unsigned_char_> {
    fn operator_call(&self) {
        const _: () = assert!(true, "std::is_integral_v<T>");;
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
}
pub trait TestFetchOr_unsigned_int_Impl {
    fn operator_call(&self);
}
impl TestFetchOr_unsigned_int_Impl for Ptr<TestFetchOr_unsigned_int_> {
    fn operator_call(&self) {
        const _: () = assert!(true, "std::is_integral_v<T>");;
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
}
pub trait TestFetchOr_unsigned_long_Impl {
    fn operator_call(&self);
}
impl TestFetchOr_unsigned_long_Impl for Ptr<TestFetchOr_unsigned_long_> {
    fn operator_call(&self) {
        const _: () = assert!(true, "std::is_integral_v<T>");;
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
}
pub trait TestFetchOr_unsigned_long_long_Impl {
    fn operator_call(&self);
}
impl TestFetchOr_unsigned_long_long_Impl for Ptr<TestFetchOr_unsigned_long_long_> {
    fn operator_call(&self) {
        const _: () = assert!(true, "std::is_integral_v<T>");;
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
}
pub trait TestFetchOr_unsigned_short_Impl {
    fn operator_call(&self);
}
impl TestFetchOr_unsigned_short_Impl for Ptr<TestFetchOr_unsigned_short_> {
    fn operator_call(&self) {
        const _: () = assert!(true, "std::is_integral_v<T>");;
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
}
pub trait TestFetchOr_wchar_t_Impl {
    fn operator_call(&self);
}
impl TestFetchOr_wchar_t_Impl for Ptr<TestFetchOr_wchar_t_> {
    fn operator_call(&self) {
        const _: () = assert!(true, "std::is_integral_v<T>");;
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
