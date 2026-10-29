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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
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
            a: (0..128).map(|_| <i32>::default()).collect::<Box<[i32]>>(),
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
pub struct TestEachIntegralType_TestFn_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFn_char_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFn_signed_char_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFn_unsigned_char_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFn_short_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFn_unsigned_short_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFn_int_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFn_unsigned_int_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFn_long_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFn_unsigned_long_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFn_long_long_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFn_unsigned_long_long_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFn_wchar_t_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFn_char8_t_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFn_char16_t_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFn_char32_t_ {}
pub fn testp_45() {
    {
        let a: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([
            0,
            <i32>::default(),
            <i32>::default(),
        ])));
        let t: Value<std_atomic_int_ptr_> = Rc::new(RefCell::new(std_atomic_int_ptr_::new_1({
            ((a.as_pointer() as Ptr<i32>).offset(2))
        })));
        assert!(
            (({
                atomic_fetch_sub_explicit_46(
                    (t.as_pointer()),
                    2_i64,
                    memory_order_seq_cst_15.with(|rc| *rc.borrow()),
                )
            }) == ((a.as_pointer() as Ptr<i32>).offset(2)))
        );
        ({
            atomic_fetch_sub_explicit_46(
                (t.as_pointer()),
                0_i64,
                memory_order_relaxed_47.with(|rc| *rc.borrow()),
            )
        });
        assert!((({ (*t.borrow()).to_pmuti32_7() }) == ((a.as_pointer() as Ptr<i32>).offset(0))));
        const _: () = assert!(true, "");;
    }
    {
        let a: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([
            0,
            <i32>::default(),
            <i32>::default(),
        ])));
        let t: Value<std_atomic_int_ptr_> = Rc::new(RefCell::new(std_atomic_int_ptr_::new_1({
            ((a.as_pointer() as Ptr<i32>).offset(2))
        })));
        assert!(
            (({
                atomic_fetch_sub_explicit_48(
                    (t.as_pointer()),
                    2_i64,
                    memory_order_seq_cst_15.with(|rc| *rc.borrow()),
                )
            }) == ((a.as_pointer() as Ptr<i32>).offset(2)))
        );
        ({
            atomic_fetch_sub_explicit_48(
                (t.as_pointer()),
                0_i64,
                memory_order_relaxed_47.with(|rc| *rc.borrow()),
            )
        });
        assert!((({ (*t.borrow()).to_pmuti32_8() }) == ((a.as_pointer() as Ptr<i32>).offset(0))));
        const _: () = assert!(true, "");;
    }
}
pub fn testp_49() {
    {
        let a: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([
            0,
            <i32>::default(),
            <i32>::default(),
        ])));
        let t: Value<std_atomic_const_int_ptr_> =
            Rc::new(RefCell::new(std_atomic_const_int_ptr_::new_1({
                ((a.as_pointer() as Ptr<i32>).offset(2))
            })));
        assert!(
            (({
                atomic_fetch_sub_explicit_50(
                    (t.as_pointer()),
                    2_i64,
                    memory_order_seq_cst_15.with(|rc| *rc.borrow()),
                )
            }) == ((a.as_pointer() as Ptr<i32>).offset(2)))
        );
        ({
            atomic_fetch_sub_explicit_50(
                (t.as_pointer()),
                0_i64,
                memory_order_relaxed_47.with(|rc| *rc.borrow()),
            )
        });
        assert!((({ (*t.borrow()).to_pconsti32_7() }) == ((a.as_pointer() as Ptr<i32>).offset(0))));
        const _: () = assert!(true, "");;
    }
    {
        let a: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([
            0,
            <i32>::default(),
            <i32>::default(),
        ])));
        let t: Value<std_atomic_const_int_ptr_> =
            Rc::new(RefCell::new(std_atomic_const_int_ptr_::new_1({
                ((a.as_pointer() as Ptr<i32>).offset(2))
            })));
        assert!(
            (({
                atomic_fetch_sub_explicit_51(
                    (t.as_pointer()),
                    2_i64,
                    memory_order_seq_cst_15.with(|rc| *rc.borrow()),
                )
            }) == ((a.as_pointer() as Ptr<i32>).offset(2)))
        );
        ({
            atomic_fetch_sub_explicit_51(
                (t.as_pointer()),
                0_i64,
                memory_order_relaxed_47.with(|rc| *rc.borrow()),
            )
        });
        assert!((({ (*t.borrow()).to_pconsti32_8() }) == ((a.as_pointer() as Ptr<i32>).offset(0))));
        const _: () = assert!(true, "");;
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
        TestEachIntegralType_TestFn_Impl::operator_call(
            &Rc::new(RefCell::new(<TestEachIntegralType_TestFn_>::default())).as_pointer(),
        )
    });
    ({ testp_45() });
    ({ testp_49() });
    return 0;
}
pub trait TestEachIntegralType_TestFn_Impl {
    fn operator_call(&self);
}
impl TestEachIntegralType_TestFn_Impl for Ptr<TestEachIntegralType_TestFn_> {
    fn operator_call(&self) {
        ({
            TestFn_char_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFn_char_>::default())).as_pointer(),
            )
        });
        ({
            TestFn_signed_char_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFn_signed_char_>::default())).as_pointer(),
            )
        });
        ({
            TestFn_unsigned_char_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFn_unsigned_char_>::default())).as_pointer(),
            )
        });
        ({
            TestFn_short_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFn_short_>::default())).as_pointer(),
            )
        });
        ({
            TestFn_unsigned_short_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFn_unsigned_short_>::default())).as_pointer(),
            )
        });
        ({
            TestFn_int_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFn_int_>::default())).as_pointer(),
            )
        });
        ({
            TestFn_unsigned_int_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFn_unsigned_int_>::default())).as_pointer(),
            )
        });
        ({
            TestFn_long_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFn_long_>::default())).as_pointer(),
            )
        });
        ({
            TestFn_unsigned_long_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFn_unsigned_long_>::default())).as_pointer(),
            )
        });
        ({
            TestFn_long_long_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFn_long_long_>::default())).as_pointer(),
            )
        });
        ({
            TestFn_unsigned_long_long_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFn_unsigned_long_long_>::default())).as_pointer(),
            )
        });
        ({
            TestFn_wchar_t_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFn_wchar_t_>::default())).as_pointer(),
            )
        });
        ({
            TestFn_char8_t_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFn_char8_t_>::default())).as_pointer(),
            )
        });
        ({
            TestFn_char16_t_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFn_char16_t_>::default())).as_pointer(),
            )
        });
        ({
            TestFn_char32_t_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFn_char32_t_>::default())).as_pointer(),
            )
        });
        ({
            TestFn_signed_char_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFn_signed_char_>::default())).as_pointer(),
            )
        });
        ({
            TestFn_unsigned_char_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFn_unsigned_char_>::default())).as_pointer(),
            )
        });
        ({
            TestFn_short_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFn_short_>::default())).as_pointer(),
            )
        });
        ({
            TestFn_unsigned_short_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFn_unsigned_short_>::default())).as_pointer(),
            )
        });
        ({
            TestFn_int_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFn_int_>::default())).as_pointer(),
            )
        });
        ({
            TestFn_unsigned_int_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFn_unsigned_int_>::default())).as_pointer(),
            )
        });
        ({
            TestFn_long_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFn_long_>::default())).as_pointer(),
            )
        });
        ({
            TestFn_unsigned_long_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFn_unsigned_long_>::default())).as_pointer(),
            )
        });
    }
}
pub trait TestFn_char16_t_Impl {
    fn operator_call(&self);
}
impl TestFn_char16_t_Impl for Ptr<TestFn_char16_t_> {
    fn operator_call(&self) {
        {
            let t: Value<std_atomic_char16_t_> =
                Rc::new(RefCell::new(std_atomic_char16_t_::new_1({ 3_u16 })));
            assert!(
                ((({
                    atomic_fetch_sub_explicit_41(
                        (t.as_pointer()),
                        2_u16,
                        memory_order_seq_cst_15.with(|rc| *rc.borrow()),
                    )
                }) as i32)
                    == (3_u16 as i32))
            );
            assert!(
                ((({ ((*t.borrow()) as std___atomic_base_char16_t_).to_u16_7() }) as i32)
                    == (1_u16 as i32))
            );
            const _: () = assert!(true, "");;
        }
        {
            let t: Value<std_atomic_char16_t_> =
                Rc::new(RefCell::new(std_atomic_char16_t_::new_1({ 3_u16 })));
            assert!(
                ((({
                    atomic_fetch_sub_explicit_42(
                        (t.as_pointer()),
                        2_u16,
                        memory_order_seq_cst_15.with(|rc| *rc.borrow()),
                    )
                }) as i32)
                    == (3_u16 as i32))
            );
            assert!(
                ((({ ((*t.borrow()) as std___atomic_base_char16_t_).to_u16_8() }) as i32)
                    == (1_u16 as i32))
            );
            const _: () = assert!(true, "");;
        }
    }
}
pub trait TestFn_char32_t_Impl {
    fn operator_call(&self);
}
impl TestFn_char32_t_Impl for Ptr<TestFn_char32_t_> {
    fn operator_call(&self) {
        {
            let t: Value<std_atomic_char32_t_> =
                Rc::new(RefCell::new(std_atomic_char32_t_::new_1({ 3_u32 })));
            assert!(
                (({
                    atomic_fetch_sub_explicit_43(
                        (t.as_pointer()),
                        2_u32,
                        memory_order_seq_cst_15.with(|rc| *rc.borrow()),
                    )
                }) == 3_u32)
            );
            assert!((({ ((*t.borrow()) as std___atomic_base_char32_t_).to_u32_7() }) == 1_u32));
            const _: () = assert!(true, "");;
        }
        {
            let t: Value<std_atomic_char32_t_> =
                Rc::new(RefCell::new(std_atomic_char32_t_::new_1({ 3_u32 })));
            assert!(
                (({
                    atomic_fetch_sub_explicit_44(
                        (t.as_pointer()),
                        2_u32,
                        memory_order_seq_cst_15.with(|rc| *rc.borrow()),
                    )
                }) == 3_u32)
            );
            assert!((({ ((*t.borrow()) as std___atomic_base_char32_t_).to_u32_8() }) == 1_u32));
            const _: () = assert!(true, "");;
        }
    }
}
pub trait TestFn_char8_t_Impl {
    fn operator_call(&self);
}
impl TestFn_char8_t_Impl for Ptr<TestFn_char8_t_> {
    fn operator_call(&self) {
        {
            let t: Value<std_atomic_char8_t_> =
                Rc::new(RefCell::new(std_atomic_char8_t_::new_1({ 3_u8 })));
            assert!(
                ((({
                    atomic_fetch_sub_explicit_39(
                        (t.as_pointer()),
                        2_u8,
                        memory_order_seq_cst_15.with(|rc| *rc.borrow()),
                    )
                }) as i32)
                    == (3_u8 as i32))
            );
            assert!(
                ((({ ((*t.borrow()) as std___atomic_base_char8_t_).to_u8_7() }) as i32)
                    == (1_u8 as i32))
            );
            const _: () = assert!(true, "");;
        }
        {
            let t: Value<std_atomic_char8_t_> =
                Rc::new(RefCell::new(std_atomic_char8_t_::new_1({ 3_u8 })));
            assert!(
                ((({
                    atomic_fetch_sub_explicit_40(
                        (t.as_pointer()),
                        2_u8,
                        memory_order_seq_cst_15.with(|rc| *rc.borrow()),
                    )
                }) as i32)
                    == (3_u8 as i32))
            );
            assert!(
                ((({ ((*t.borrow()) as std___atomic_base_char8_t_).to_u8_8() }) as i32)
                    == (1_u8 as i32))
            );
            const _: () = assert!(true, "");;
        }
    }
}
pub trait TestFn_char_Impl {
    fn operator_call(&self);
}
impl TestFn_char_Impl for Ptr<TestFn_char_> {
    fn operator_call(&self) {
        {
            let t: Value<std_atomic_char_> =
                Rc::new(RefCell::new(std_atomic_char_::new_1({ 3_i8 })));
            assert!(
                ((({
                    atomic_fetch_sub_explicit_14(
                        (t.as_pointer()),
                        2_i8,
                        memory_order_seq_cst_15.with(|rc| *rc.borrow()),
                    )
                }) as i32)
                    == (3_i8 as i32))
            );
            assert!(
                ((({ ((*t.borrow()) as std___atomic_base_char_).to_i8_7() }) as i32)
                    == (1_i8 as i32))
            );
            const _: () = assert!(true, "");;
        }
        {
            let t: Value<std_atomic_char_> =
                Rc::new(RefCell::new(std_atomic_char_::new_1({ 3_i8 })));
            assert!(
                ((({
                    atomic_fetch_sub_explicit_16(
                        (t.as_pointer()),
                        2_i8,
                        memory_order_seq_cst_15.with(|rc| *rc.borrow()),
                    )
                }) as i32)
                    == (3_i8 as i32))
            );
            assert!(
                ((({ ((*t.borrow()) as std___atomic_base_char_).to_i8_8() }) as i32)
                    == (1_i8 as i32))
            );
            const _: () = assert!(true, "");;
        }
    }
}
pub trait TestFn_int_Impl {
    fn operator_call(&self);
}
impl TestFn_int_Impl for Ptr<TestFn_int_> {
    fn operator_call(&self) {
        {
            let t: Value<std_atomic_int_> = Rc::new(RefCell::new(std_atomic_int_::new_1({ 3 })));
            assert!(
                (({
                    atomic_fetch_sub_explicit_25(
                        (t.as_pointer()),
                        2,
                        memory_order_seq_cst_15.with(|rc| *rc.borrow()),
                    )
                }) == 3)
            );
            assert!((({ ((*t.borrow()) as std___atomic_base_int_).to_i32_7() }) == 1));
            const _: () = assert!(true, "");;
        }
        {
            let t: Value<std_atomic_int_> = Rc::new(RefCell::new(std_atomic_int_::new_1({ 3 })));
            assert!(
                (({
                    atomic_fetch_sub_explicit_26(
                        (t.as_pointer()),
                        2,
                        memory_order_seq_cst_15.with(|rc| *rc.borrow()),
                    )
                }) == 3)
            );
            assert!((({ ((*t.borrow()) as std___atomic_base_int_).to_i32_8() }) == 1));
            const _: () = assert!(true, "");;
        }
    }
}
pub trait TestFn_long_Impl {
    fn operator_call(&self);
}
impl TestFn_long_Impl for Ptr<TestFn_long_> {
    fn operator_call(&self) {
        {
            let t: Value<std_atomic_long_> =
                Rc::new(RefCell::new(std_atomic_long_::new_1({ 3_i64 })));
            assert!(
                (({
                    atomic_fetch_sub_explicit_29(
                        (t.as_pointer()),
                        2_i64,
                        memory_order_seq_cst_15.with(|rc| *rc.borrow()),
                    )
                }) == 3_i64)
            );
            assert!((({ ((*t.borrow()) as std___atomic_base_long_).to_i64_7() }) == 1_i64));
            const _: () = assert!(true, "");;
        }
        {
            let t: Value<std_atomic_long_> =
                Rc::new(RefCell::new(std_atomic_long_::new_1({ 3_i64 })));
            assert!(
                (({
                    atomic_fetch_sub_explicit_30(
                        (t.as_pointer()),
                        2_i64,
                        memory_order_seq_cst_15.with(|rc| *rc.borrow()),
                    )
                }) == 3_i64)
            );
            assert!((({ ((*t.borrow()) as std___atomic_base_long_).to_i64_8() }) == 1_i64));
            const _: () = assert!(true, "");;
        }
    }
}
pub trait TestFn_long_long_Impl {
    fn operator_call(&self);
}
impl TestFn_long_long_Impl for Ptr<TestFn_long_long_> {
    fn operator_call(&self) {
        {
            let t: Value<std_atomic_long_long_> =
                Rc::new(RefCell::new(std_atomic_long_long_::new_1({ 3_i64 })));
            assert!(
                (({
                    atomic_fetch_sub_explicit_33(
                        (t.as_pointer()),
                        2_i64,
                        memory_order_seq_cst_15.with(|rc| *rc.borrow()),
                    )
                }) == 3_i64)
            );
            assert!((({ ((*t.borrow()) as std___atomic_base_long_long_).to_i64_7() }) == 1_i64));
            const _: () = assert!(true, "");;
        }
        {
            let t: Value<std_atomic_long_long_> =
                Rc::new(RefCell::new(std_atomic_long_long_::new_1({ 3_i64 })));
            assert!(
                (({
                    atomic_fetch_sub_explicit_34(
                        (t.as_pointer()),
                        2_i64,
                        memory_order_seq_cst_15.with(|rc| *rc.borrow()),
                    )
                }) == 3_i64)
            );
            assert!((({ ((*t.borrow()) as std___atomic_base_long_long_).to_i64_8() }) == 1_i64));
            const _: () = assert!(true, "");;
        }
    }
}
pub trait TestFn_short_Impl {
    fn operator_call(&self);
}
impl TestFn_short_Impl for Ptr<TestFn_short_> {
    fn operator_call(&self) {
        {
            let t: Value<std_atomic_short_> =
                Rc::new(RefCell::new(std_atomic_short_::new_1({ 3_i16 })));
            assert!(
                ((({
                    atomic_fetch_sub_explicit_21(
                        (t.as_pointer()),
                        2_i16,
                        memory_order_seq_cst_15.with(|rc| *rc.borrow()),
                    )
                }) as i32)
                    == (3_i16 as i32))
            );
            assert!(
                ((({ ((*t.borrow()) as std___atomic_base_short_).to_i16_7() }) as i32)
                    == (1_i16 as i32))
            );
            const _: () = assert!(true, "");;
        }
        {
            let t: Value<std_atomic_short_> =
                Rc::new(RefCell::new(std_atomic_short_::new_1({ 3_i16 })));
            assert!(
                ((({
                    atomic_fetch_sub_explicit_22(
                        (t.as_pointer()),
                        2_i16,
                        memory_order_seq_cst_15.with(|rc| *rc.borrow()),
                    )
                }) as i32)
                    == (3_i16 as i32))
            );
            assert!(
                ((({ ((*t.borrow()) as std___atomic_base_short_).to_i16_8() }) as i32)
                    == (1_i16 as i32))
            );
            const _: () = assert!(true, "");;
        }
    }
}
pub trait TestFn_signed_char_Impl {
    fn operator_call(&self);
}
impl TestFn_signed_char_Impl for Ptr<TestFn_signed_char_> {
    fn operator_call(&self) {
        {
            let t: Value<std_atomic_signed_char_> =
                Rc::new(RefCell::new(std_atomic_signed_char_::new_1({ 3_i8 })));
            assert!(
                ((({
                    atomic_fetch_sub_explicit_17(
                        (t.as_pointer()),
                        2_i8,
                        memory_order_seq_cst_15.with(|rc| *rc.borrow()),
                    )
                }) as i32)
                    == (3_i8 as i32))
            );
            assert!(
                ((({ ((*t.borrow()) as std___atomic_base_signed_char_).to_i8_7() }) as i32)
                    == (1_i8 as i32))
            );
            const _: () = assert!(true, "");;
        }
        {
            let t: Value<std_atomic_signed_char_> =
                Rc::new(RefCell::new(std_atomic_signed_char_::new_1({ 3_i8 })));
            assert!(
                ((({
                    atomic_fetch_sub_explicit_18(
                        (t.as_pointer()),
                        2_i8,
                        memory_order_seq_cst_15.with(|rc| *rc.borrow()),
                    )
                }) as i32)
                    == (3_i8 as i32))
            );
            assert!(
                ((({ ((*t.borrow()) as std___atomic_base_signed_char_).to_i8_8() }) as i32)
                    == (1_i8 as i32))
            );
            const _: () = assert!(true, "");;
        }
    }
}
pub trait TestFn_unsigned_char_Impl {
    fn operator_call(&self);
}
impl TestFn_unsigned_char_Impl for Ptr<TestFn_unsigned_char_> {
    fn operator_call(&self) {
        {
            let t: Value<std_atomic_unsigned_char_> =
                Rc::new(RefCell::new(std_atomic_unsigned_char_::new_1({ 3_u8 })));
            assert!(
                ((({
                    atomic_fetch_sub_explicit_19(
                        (t.as_pointer()),
                        2_u8,
                        memory_order_seq_cst_15.with(|rc| *rc.borrow()),
                    )
                }) as i32)
                    == (3_u8 as i32))
            );
            assert!(
                ((({ ((*t.borrow()) as std___atomic_base_unsigned_char_).to_u8_7() }) as i32)
                    == (1_u8 as i32))
            );
            const _: () = assert!(true, "");;
        }
        {
            let t: Value<std_atomic_unsigned_char_> =
                Rc::new(RefCell::new(std_atomic_unsigned_char_::new_1({ 3_u8 })));
            assert!(
                ((({
                    atomic_fetch_sub_explicit_20(
                        (t.as_pointer()),
                        2_u8,
                        memory_order_seq_cst_15.with(|rc| *rc.borrow()),
                    )
                }) as i32)
                    == (3_u8 as i32))
            );
            assert!(
                ((({ ((*t.borrow()) as std___atomic_base_unsigned_char_).to_u8_8() }) as i32)
                    == (1_u8 as i32))
            );
            const _: () = assert!(true, "");;
        }
    }
}
pub trait TestFn_unsigned_int_Impl {
    fn operator_call(&self);
}
impl TestFn_unsigned_int_Impl for Ptr<TestFn_unsigned_int_> {
    fn operator_call(&self) {
        {
            let t: Value<std_atomic_unsigned_int_> =
                Rc::new(RefCell::new(std_atomic_unsigned_int_::new_1({ 3_u32 })));
            assert!(
                (({
                    atomic_fetch_sub_explicit_27(
                        (t.as_pointer()),
                        2_u32,
                        memory_order_seq_cst_15.with(|rc| *rc.borrow()),
                    )
                }) == 3_u32)
            );
            assert!((({ ((*t.borrow()) as std___atomic_base_unsigned_int_).to_u32_7() }) == 1_u32));
            const _: () = assert!(true, "");;
        }
        {
            let t: Value<std_atomic_unsigned_int_> =
                Rc::new(RefCell::new(std_atomic_unsigned_int_::new_1({ 3_u32 })));
            assert!(
                (({
                    atomic_fetch_sub_explicit_28(
                        (t.as_pointer()),
                        2_u32,
                        memory_order_seq_cst_15.with(|rc| *rc.borrow()),
                    )
                }) == 3_u32)
            );
            assert!((({ ((*t.borrow()) as std___atomic_base_unsigned_int_).to_u32_8() }) == 1_u32));
            const _: () = assert!(true, "");;
        }
    }
}
pub trait TestFn_unsigned_long_Impl {
    fn operator_call(&self);
}
impl TestFn_unsigned_long_Impl for Ptr<TestFn_unsigned_long_> {
    fn operator_call(&self) {
        {
            let t: Value<std_atomic_unsigned_long_> =
                Rc::new(RefCell::new(std_atomic_unsigned_long_::new_1({ 3_u64 })));
            assert!(
                (({
                    atomic_fetch_sub_explicit_31(
                        (t.as_pointer()),
                        2_u64,
                        memory_order_seq_cst_15.with(|rc| *rc.borrow()),
                    )
                }) == 3_u64)
            );
            assert!(
                (({ ((*t.borrow()) as std___atomic_base_unsigned_long_).to_u64_7() }) == 1_u64)
            );
            const _: () = assert!(true, "");;
        }
        {
            let t: Value<std_atomic_unsigned_long_> =
                Rc::new(RefCell::new(std_atomic_unsigned_long_::new_1({ 3_u64 })));
            assert!(
                (({
                    atomic_fetch_sub_explicit_32(
                        (t.as_pointer()),
                        2_u64,
                        memory_order_seq_cst_15.with(|rc| *rc.borrow()),
                    )
                }) == 3_u64)
            );
            assert!(
                (({ ((*t.borrow()) as std___atomic_base_unsigned_long_).to_u64_8() }) == 1_u64)
            );
            const _: () = assert!(true, "");;
        }
    }
}
pub trait TestFn_unsigned_long_long_Impl {
    fn operator_call(&self);
}
impl TestFn_unsigned_long_long_Impl for Ptr<TestFn_unsigned_long_long_> {
    fn operator_call(&self) {
        {
            let t: Value<std_atomic_unsigned_long_long_> =
                Rc::new(RefCell::new(std_atomic_unsigned_long_long_::new_1({
                    3_u64
                })));
            assert!(
                (({
                    atomic_fetch_sub_explicit_35(
                        (t.as_pointer()),
                        2_u64,
                        memory_order_seq_cst_15.with(|rc| *rc.borrow()),
                    )
                }) == 3_u64)
            );
            assert!(
                (({ ((*t.borrow()) as std___atomic_base_unsigned_long_long_).to_u64_7() })
                    == 1_u64)
            );
            const _: () = assert!(true, "");;
        }
        {
            let t: Value<std_atomic_unsigned_long_long_> =
                Rc::new(RefCell::new(std_atomic_unsigned_long_long_::new_1({
                    3_u64
                })));
            assert!(
                (({
                    atomic_fetch_sub_explicit_36(
                        (t.as_pointer()),
                        2_u64,
                        memory_order_seq_cst_15.with(|rc| *rc.borrow()),
                    )
                }) == 3_u64)
            );
            assert!(
                (({ ((*t.borrow()) as std___atomic_base_unsigned_long_long_).to_u64_8() })
                    == 1_u64)
            );
            const _: () = assert!(true, "");;
        }
    }
}
pub trait TestFn_unsigned_short_Impl {
    fn operator_call(&self);
}
impl TestFn_unsigned_short_Impl for Ptr<TestFn_unsigned_short_> {
    fn operator_call(&self) {
        {
            let t: Value<std_atomic_unsigned_short_> =
                Rc::new(RefCell::new(std_atomic_unsigned_short_::new_1({ 3_u16 })));
            assert!(
                ((({
                    atomic_fetch_sub_explicit_23(
                        (t.as_pointer()),
                        2_u16,
                        memory_order_seq_cst_15.with(|rc| *rc.borrow()),
                    )
                }) as i32)
                    == (3_u16 as i32))
            );
            assert!(
                ((({ ((*t.borrow()) as std___atomic_base_unsigned_short_).to_u16_7() }) as i32)
                    == (1_u16 as i32))
            );
            const _: () = assert!(true, "");;
        }
        {
            let t: Value<std_atomic_unsigned_short_> =
                Rc::new(RefCell::new(std_atomic_unsigned_short_::new_1({ 3_u16 })));
            assert!(
                ((({
                    atomic_fetch_sub_explicit_24(
                        (t.as_pointer()),
                        2_u16,
                        memory_order_seq_cst_15.with(|rc| *rc.borrow()),
                    )
                }) as i32)
                    == (3_u16 as i32))
            );
            assert!(
                ((({ ((*t.borrow()) as std___atomic_base_unsigned_short_).to_u16_8() }) as i32)
                    == (1_u16 as i32))
            );
            const _: () = assert!(true, "");;
        }
    }
}
pub trait TestFn_wchar_t_Impl {
    fn operator_call(&self);
}
impl TestFn_wchar_t_Impl for Ptr<TestFn_wchar_t_> {
    fn operator_call(&self) {
        {
            let t: Value<std_atomic_wchar_t_> =
                Rc::new(RefCell::new(std_atomic_wchar_t_::new_1({ 3_i32 })));
            assert!(
                (({
                    atomic_fetch_sub_explicit_37(
                        (t.as_pointer()),
                        2_i32,
                        memory_order_seq_cst_15.with(|rc| *rc.borrow()),
                    )
                }) == 3_i32)
            );
            assert!((({ ((*t.borrow()) as std___atomic_base_wchar_t_).to_i32_7() }) == 1_i32));
            const _: () = assert!(true, "");;
        }
        {
            let t: Value<std_atomic_wchar_t_> =
                Rc::new(RefCell::new(std_atomic_wchar_t_::new_1({ 3_i32 })));
            assert!(
                (({
                    atomic_fetch_sub_explicit_38(
                        (t.as_pointer()),
                        2_i32,
                        memory_order_seq_cst_15.with(|rc| *rc.borrow()),
                    )
                }) == 3_i32)
            );
            assert!((({ ((*t.borrow()) as std___atomic_base_wchar_t_).to_i32_8() }) == 1_i32));
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
