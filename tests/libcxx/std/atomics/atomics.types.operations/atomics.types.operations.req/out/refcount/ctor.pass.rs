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
pub struct TestEachIntegralType_TestFunc_ {}
pub fn operator_eq_14(x: Ptr<UserType>, y: Ptr<UserType>) -> bool {
    return ({ x.with(|__s| __s.i) } == { y.with(|__s| __s.i) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct UserType {
    #[offset(0)]
    pub i: i32,
}
impl UserType {
    pub fn new_1() -> Self {
        Self { i: 0_i32 }
    }
    pub fn new_2(mut d: i32) -> Self {
        Self { i: d }
    }
}
impl CppEq for UserType {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_14(__lhs.clone(), __rhs.clone())
    }
}
impl Default for UserType {
    fn default() -> Self {
        { UserType::new_1() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFunc_UserType_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFunc_char_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFunc_signed_char_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFunc_unsigned_char_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFunc_short_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFunc_unsigned_short_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFunc_int_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFunc_unsigned_int_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFunc_long_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFunc_unsigned_long_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFunc_long_long_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFunc_unsigned_long_long_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFunc_wchar_t_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFunc_char8_t_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFunc_char16_t_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFunc_char32_t_ {}
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
        TestFunc_UserType_Impl::operator_call(
            &Rc::new(RefCell::new(<TestFunc_UserType_>::default())).as_pointer(),
        )
    });
    ({
        TestEachIntegralType_TestFunc_Impl::operator_call(
            &Rc::new(RefCell::new(<TestEachIntegralType_TestFunc_>::default())).as_pointer(),
        )
    });
    return 0;
}
pub trait TestEachIntegralType_TestFunc_Impl {
    fn operator_call(&self);
}
impl TestEachIntegralType_TestFunc_Impl for Ptr<TestEachIntegralType_TestFunc_> {
    fn operator_call(&self) {
        ({
            TestFunc_char_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFunc_char_>::default())).as_pointer(),
            )
        });
        ({
            TestFunc_signed_char_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFunc_signed_char_>::default())).as_pointer(),
            )
        });
        ({
            TestFunc_unsigned_char_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFunc_unsigned_char_>::default())).as_pointer(),
            )
        });
        ({
            TestFunc_short_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFunc_short_>::default())).as_pointer(),
            )
        });
        ({
            TestFunc_unsigned_short_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFunc_unsigned_short_>::default())).as_pointer(),
            )
        });
        ({
            TestFunc_int_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFunc_int_>::default())).as_pointer(),
            )
        });
        ({
            TestFunc_unsigned_int_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFunc_unsigned_int_>::default())).as_pointer(),
            )
        });
        ({
            TestFunc_long_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFunc_long_>::default())).as_pointer(),
            )
        });
        ({
            TestFunc_unsigned_long_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFunc_unsigned_long_>::default())).as_pointer(),
            )
        });
        ({
            TestFunc_long_long_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFunc_long_long_>::default())).as_pointer(),
            )
        });
        ({
            TestFunc_unsigned_long_long_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFunc_unsigned_long_long_>::default())).as_pointer(),
            )
        });
        ({
            TestFunc_wchar_t_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFunc_wchar_t_>::default())).as_pointer(),
            )
        });
        ({
            TestFunc_char8_t_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFunc_char8_t_>::default())).as_pointer(),
            )
        });
        ({
            TestFunc_char16_t_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFunc_char16_t_>::default())).as_pointer(),
            )
        });
        ({
            TestFunc_char32_t_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFunc_char32_t_>::default())).as_pointer(),
            )
        });
        ({
            TestFunc_signed_char_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFunc_signed_char_>::default())).as_pointer(),
            )
        });
        ({
            TestFunc_unsigned_char_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFunc_unsigned_char_>::default())).as_pointer(),
            )
        });
        ({
            TestFunc_short_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFunc_short_>::default())).as_pointer(),
            )
        });
        ({
            TestFunc_unsigned_short_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFunc_unsigned_short_>::default())).as_pointer(),
            )
        });
        ({
            TestFunc_int_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFunc_int_>::default())).as_pointer(),
            )
        });
        ({
            TestFunc_unsigned_int_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFunc_unsigned_int_>::default())).as_pointer(),
            )
        });
        ({
            TestFunc_long_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFunc_long_>::default())).as_pointer(),
            )
        });
        ({
            TestFunc_unsigned_long_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFunc_unsigned_long_>::default())).as_pointer(),
            )
        });
    }
}
pub trait TestFunc_UserType_Impl {
    fn operator_call(&self);
}
impl TestFunc_UserType_Impl for Ptr<TestFunc_UserType_> {
    fn operator_call(&self) {
        let t: Value<UserType> = Rc::new(RefCell::new(UserType::new_2({ 42 })));
        {
            let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            assert!(
                ({
                    let _x: Value<UserType> = Rc::new(RefCell::new(unimplemented!()));
                    operator_eq_14(_x.as_pointer(), t.as_pointer())
                })
            );
        }
        {
            let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            assert!(
                ({
                    let _x: Value<UserType> = Rc::new(RefCell::new(unimplemented!()));
                    operator_eq_14(_x.as_pointer(), t.as_pointer())
                })
            );
        }
    }
}
pub trait TestFunc_char16_t_Impl {
    fn operator_call(&self);
}
impl TestFunc_char16_t_Impl for Ptr<TestFunc_char16_t_> {
    fn operator_call(&self) {
        let mut t: u16 = 42_u16;
        {
            let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            assert!(((unimplemented!() as i32) == (t as i32)));
        }
        {
            let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            assert!(((unimplemented!() as i32) == (t as i32)));
        }
    }
}
pub trait TestFunc_char32_t_Impl {
    fn operator_call(&self);
}
impl TestFunc_char32_t_Impl for Ptr<TestFunc_char32_t_> {
    fn operator_call(&self) {
        let mut t: u32 = 42_u32;
        {
            let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            assert!(((unimplemented!() as u32) == t));
        }
        {
            let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            assert!(((unimplemented!() as u32) == t));
        }
    }
}
pub trait TestFunc_char8_t_Impl {
    fn operator_call(&self);
}
impl TestFunc_char8_t_Impl for Ptr<TestFunc_char8_t_> {
    fn operator_call(&self) {
        let mut t: u8 = 42_u8;
        {
            let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            assert!(((unimplemented!() as i32) == (t as i32)));
        }
        {
            let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            assert!(((unimplemented!() as i32) == (t as i32)));
        }
    }
}
pub trait TestFunc_char_Impl {
    fn operator_call(&self);
}
impl TestFunc_char_Impl for Ptr<TestFunc_char_> {
    fn operator_call(&self) {
        let mut t: i8 = 42_i8;
        {
            let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            assert!(((unimplemented!() as i32) == (t as i32)));
        }
        {
            let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            assert!(((unimplemented!() as i32) == (t as i32)));
        }
    }
}
pub trait TestFunc_int_Impl {
    fn operator_call(&self);
}
impl TestFunc_int_Impl for Ptr<TestFunc_int_> {
    fn operator_call(&self) {
        let mut t: i32 = 42;
        {
            let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            assert!((unimplemented!() == t));
        }
        {
            let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            assert!((unimplemented!() == t));
        }
    }
}
pub trait TestFunc_long_Impl {
    fn operator_call(&self);
}
impl TestFunc_long_Impl for Ptr<TestFunc_long_> {
    fn operator_call(&self) {
        let mut t: i64 = 42_i64;
        {
            let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            assert!((unimplemented!() == t));
        }
        {
            let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            assert!((unimplemented!() == t));
        }
    }
}
pub trait TestFunc_long_long_Impl {
    fn operator_call(&self);
}
impl TestFunc_long_long_Impl for Ptr<TestFunc_long_long_> {
    fn operator_call(&self) {
        let mut t: i64 = 42_i64;
        {
            let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            assert!((unimplemented!() == t));
        }
        {
            let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            assert!((unimplemented!() == t));
        }
    }
}
pub trait TestFunc_short_Impl {
    fn operator_call(&self);
}
impl TestFunc_short_Impl for Ptr<TestFunc_short_> {
    fn operator_call(&self) {
        let mut t: i16 = 42_i16;
        {
            let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            assert!(((unimplemented!() as i32) == (t as i32)));
        }
        {
            let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            assert!(((unimplemented!() as i32) == (t as i32)));
        }
    }
}
pub trait TestFunc_signed_char_Impl {
    fn operator_call(&self);
}
impl TestFunc_signed_char_Impl for Ptr<TestFunc_signed_char_> {
    fn operator_call(&self) {
        let mut t: i8 = 42_i8;
        {
            let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            assert!(((unimplemented!() as i32) == (t as i32)));
        }
        {
            let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            assert!(((unimplemented!() as i32) == (t as i32)));
        }
    }
}
pub trait TestFunc_unsigned_char_Impl {
    fn operator_call(&self);
}
impl TestFunc_unsigned_char_Impl for Ptr<TestFunc_unsigned_char_> {
    fn operator_call(&self) {
        let mut t: u8 = 42_u8;
        {
            let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            assert!(((unimplemented!() as i32) == (t as i32)));
        }
        {
            let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            assert!(((unimplemented!() as i32) == (t as i32)));
        }
    }
}
pub trait TestFunc_unsigned_int_Impl {
    fn operator_call(&self);
}
impl TestFunc_unsigned_int_Impl for Ptr<TestFunc_unsigned_int_> {
    fn operator_call(&self) {
        let mut t: u32 = 42_u32;
        {
            let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            assert!((unimplemented!() == t));
        }
        {
            let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            assert!((unimplemented!() == t));
        }
    }
}
pub trait TestFunc_unsigned_long_Impl {
    fn operator_call(&self);
}
impl TestFunc_unsigned_long_Impl for Ptr<TestFunc_unsigned_long_> {
    fn operator_call(&self) {
        let mut t: u64 = 42_u64;
        {
            let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            assert!((unimplemented!() == t));
        }
        {
            let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            assert!((unimplemented!() == t));
        }
    }
}
pub trait TestFunc_unsigned_long_long_Impl {
    fn operator_call(&self);
}
impl TestFunc_unsigned_long_long_Impl for Ptr<TestFunc_unsigned_long_long_> {
    fn operator_call(&self) {
        let mut t: u64 = 42_u64;
        {
            let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            assert!((unimplemented!() == t));
        }
        {
            let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            assert!((unimplemented!() == t));
        }
    }
}
pub trait TestFunc_unsigned_short_Impl {
    fn operator_call(&self);
}
impl TestFunc_unsigned_short_Impl for Ptr<TestFunc_unsigned_short_> {
    fn operator_call(&self) {
        let mut t: u16 = 42_u16;
        {
            let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            assert!(((unimplemented!() as i32) == (t as i32)));
        }
        {
            let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            assert!(((unimplemented!() as i32) == (t as i32)));
        }
    }
}
pub trait TestFunc_wchar_t_Impl {
    fn operator_call(&self);
}
impl TestFunc_wchar_t_Impl for Ptr<TestFunc_wchar_t_> {
    fn operator_call(&self) {
        let mut t: i32 = 42_i32;
        {
            let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            assert!(((unimplemented!() as i32) == t));
        }
        {
            let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            assert!(((unimplemented!() as i32) == t));
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
