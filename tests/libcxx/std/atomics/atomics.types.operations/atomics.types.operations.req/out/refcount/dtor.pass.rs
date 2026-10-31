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
pub struct TestEachIntegralType_CheckTriviallyDestructible_ {}
impl Callable0<()> for TestEachIntegralType_CheckTriviallyDestructible_ {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            TestEachIntegralType_CheckTriviallyDestructible_Impl::operator_call(__it)
        })
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestEachFloatingPointType_CheckTriviallyDestructible_ {}
impl Callable0<()> for TestEachFloatingPointType_CheckTriviallyDestructible_ {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            TestEachFloatingPointType_CheckTriviallyDestructible_Impl::operator_call(__it)
        })
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestEachPointerType_CheckTriviallyDestructible_ {}
impl Callable0<()> for TestEachPointerType_CheckTriviallyDestructible_ {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            TestEachPointerType_CheckTriviallyDestructible_Impl::operator_call(__it)
        })
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct CheckTriviallyDestructible_bool_ {}
impl Callable0<()> for CheckTriviallyDestructible_bool_ {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            CheckTriviallyDestructible_bool_Impl::operator_call(__it)
        })
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct CheckTriviallyDestructible_char_ {}
impl Callable0<()> for CheckTriviallyDestructible_char_ {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            CheckTriviallyDestructible_char_Impl::operator_call(__it)
        })
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct CheckTriviallyDestructible_signed_char_ {}
impl Callable0<()> for CheckTriviallyDestructible_signed_char_ {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            CheckTriviallyDestructible_signed_char_Impl::operator_call(__it)
        })
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct CheckTriviallyDestructible_unsigned_char_ {}
impl Callable0<()> for CheckTriviallyDestructible_unsigned_char_ {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            CheckTriviallyDestructible_unsigned_char_Impl::operator_call(__it)
        })
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct CheckTriviallyDestructible_short_ {}
impl Callable0<()> for CheckTriviallyDestructible_short_ {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            CheckTriviallyDestructible_short_Impl::operator_call(__it)
        })
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct CheckTriviallyDestructible_unsigned_short_ {}
impl Callable0<()> for CheckTriviallyDestructible_unsigned_short_ {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            CheckTriviallyDestructible_unsigned_short_Impl::operator_call(__it)
        })
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct CheckTriviallyDestructible_int_ {}
impl Callable0<()> for CheckTriviallyDestructible_int_ {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            CheckTriviallyDestructible_int_Impl::operator_call(__it)
        })
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct CheckTriviallyDestructible_unsigned_int_ {}
impl Callable0<()> for CheckTriviallyDestructible_unsigned_int_ {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            CheckTriviallyDestructible_unsigned_int_Impl::operator_call(__it)
        })
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct CheckTriviallyDestructible_long_ {}
impl Callable0<()> for CheckTriviallyDestructible_long_ {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            CheckTriviallyDestructible_long_Impl::operator_call(__it)
        })
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct CheckTriviallyDestructible_unsigned_long_ {}
impl Callable0<()> for CheckTriviallyDestructible_unsigned_long_ {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            CheckTriviallyDestructible_unsigned_long_Impl::operator_call(__it)
        })
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct CheckTriviallyDestructible_long_long_ {}
impl Callable0<()> for CheckTriviallyDestructible_long_long_ {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            CheckTriviallyDestructible_long_long_Impl::operator_call(__it)
        })
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct CheckTriviallyDestructible_unsigned_long_long_ {}
impl Callable0<()> for CheckTriviallyDestructible_unsigned_long_long_ {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            CheckTriviallyDestructible_unsigned_long_long_Impl::operator_call(__it)
        })
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct CheckTriviallyDestructible_wchar_t_ {}
impl Callable0<()> for CheckTriviallyDestructible_wchar_t_ {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            CheckTriviallyDestructible_wchar_t_Impl::operator_call(__it)
        })
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct CheckTriviallyDestructible_char8_t_ {}
impl Callable0<()> for CheckTriviallyDestructible_char8_t_ {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            CheckTriviallyDestructible_char8_t_Impl::operator_call(__it)
        })
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct CheckTriviallyDestructible_char16_t_ {}
impl Callable0<()> for CheckTriviallyDestructible_char16_t_ {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            CheckTriviallyDestructible_char16_t_Impl::operator_call(__it)
        })
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct CheckTriviallyDestructible_char32_t_ {}
impl Callable0<()> for CheckTriviallyDestructible_char32_t_ {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            CheckTriviallyDestructible_char32_t_Impl::operator_call(__it)
        })
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct CheckTriviallyDestructible_float_ {}
impl Callable0<()> for CheckTriviallyDestructible_float_ {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            CheckTriviallyDestructible_float_Impl::operator_call(__it)
        })
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct CheckTriviallyDestructible_double_ {}
impl Callable0<()> for CheckTriviallyDestructible_double_ {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            CheckTriviallyDestructible_double_Impl::operator_call(__it)
        })
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct CheckTriviallyDestructible_long_double_ {}
impl Callable0<()> for CheckTriviallyDestructible_long_double_ {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            CheckTriviallyDestructible_long_double_Impl::operator_call(__it)
        })
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct CheckTriviallyDestructible_int_ptr_ {}
impl Callable0<()> for CheckTriviallyDestructible_int_ptr_ {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            CheckTriviallyDestructible_int_ptr_Impl::operator_call(__it)
        })
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct CheckTriviallyDestructible_const_int_ptr_ {}
impl Callable0<()> for CheckTriviallyDestructible_const_int_ptr_ {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            CheckTriviallyDestructible_const_int_ptr_Impl::operator_call(__it)
        })
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
        CheckTriviallyDestructible_bool_Impl::operator_call(
            &Rc::new(RefCell::new(<CheckTriviallyDestructible_bool_>::default())).as_pointer(),
        )
    });
    ({
        TestEachIntegralType_CheckTriviallyDestructible_Impl::operator_call(
            &Rc::new(RefCell::new(
                <TestEachIntegralType_CheckTriviallyDestructible_>::default(),
            ))
            .as_pointer(),
        )
    });
    ({
        TestEachFloatingPointType_CheckTriviallyDestructible_Impl::operator_call(
            &Rc::new(RefCell::new(
                <TestEachFloatingPointType_CheckTriviallyDestructible_>::default(),
            ))
            .as_pointer(),
        )
    });
    ({
        TestEachPointerType_CheckTriviallyDestructible_Impl::operator_call(
            &Rc::new(RefCell::new(
                <TestEachPointerType_CheckTriviallyDestructible_>::default(),
            ))
            .as_pointer(),
        )
    });
    return 0;
}
pub trait CheckTriviallyDestructible_bool_Impl {
    fn operator_call(&self);
}
impl CheckTriviallyDestructible_bool_Impl for Ptr<CheckTriviallyDestructible_bool_> {
    fn operator_call(&self) {
        const _: () = assert!(true, "std::is_trivially_destructible<Atomic>::value");;
    }
}
pub trait CheckTriviallyDestructible_char16_t_Impl {
    fn operator_call(&self);
}
impl CheckTriviallyDestructible_char16_t_Impl for Ptr<CheckTriviallyDestructible_char16_t_> {
    fn operator_call(&self) {
        const _: () = assert!(true, "std::is_trivially_destructible<Atomic>::value");;
    }
}
pub trait CheckTriviallyDestructible_char32_t_Impl {
    fn operator_call(&self);
}
impl CheckTriviallyDestructible_char32_t_Impl for Ptr<CheckTriviallyDestructible_char32_t_> {
    fn operator_call(&self) {
        const _: () = assert!(true, "std::is_trivially_destructible<Atomic>::value");;
    }
}
pub trait CheckTriviallyDestructible_char8_t_Impl {
    fn operator_call(&self);
}
impl CheckTriviallyDestructible_char8_t_Impl for Ptr<CheckTriviallyDestructible_char8_t_> {
    fn operator_call(&self) {
        const _: () = assert!(true, "std::is_trivially_destructible<Atomic>::value");;
    }
}
pub trait CheckTriviallyDestructible_char_Impl {
    fn operator_call(&self);
}
impl CheckTriviallyDestructible_char_Impl for Ptr<CheckTriviallyDestructible_char_> {
    fn operator_call(&self) {
        const _: () = assert!(true, "std::is_trivially_destructible<Atomic>::value");;
    }
}
pub trait CheckTriviallyDestructible_const_int_ptr_Impl {
    fn operator_call(&self);
}
impl CheckTriviallyDestructible_const_int_ptr_Impl
    for Ptr<CheckTriviallyDestructible_const_int_ptr_>
{
    fn operator_call(&self) {
        const _: () = assert!(true, "std::is_trivially_destructible<Atomic>::value");;
    }
}
pub trait CheckTriviallyDestructible_double_Impl {
    fn operator_call(&self);
}
impl CheckTriviallyDestructible_double_Impl for Ptr<CheckTriviallyDestructible_double_> {
    fn operator_call(&self) {
        const _: () = assert!(true, "std::is_trivially_destructible<Atomic>::value");;
    }
}
pub trait CheckTriviallyDestructible_float_Impl {
    fn operator_call(&self);
}
impl CheckTriviallyDestructible_float_Impl for Ptr<CheckTriviallyDestructible_float_> {
    fn operator_call(&self) {
        const _: () = assert!(true, "std::is_trivially_destructible<Atomic>::value");;
    }
}
pub trait CheckTriviallyDestructible_int_Impl {
    fn operator_call(&self);
}
impl CheckTriviallyDestructible_int_Impl for Ptr<CheckTriviallyDestructible_int_> {
    fn operator_call(&self) {
        const _: () = assert!(true, "std::is_trivially_destructible<Atomic>::value");;
    }
}
pub trait CheckTriviallyDestructible_int_ptr_Impl {
    fn operator_call(&self);
}
impl CheckTriviallyDestructible_int_ptr_Impl for Ptr<CheckTriviallyDestructible_int_ptr_> {
    fn operator_call(&self) {
        const _: () = assert!(true, "std::is_trivially_destructible<Atomic>::value");;
    }
}
pub trait CheckTriviallyDestructible_long_Impl {
    fn operator_call(&self);
}
impl CheckTriviallyDestructible_long_Impl for Ptr<CheckTriviallyDestructible_long_> {
    fn operator_call(&self) {
        const _: () = assert!(true, "std::is_trivially_destructible<Atomic>::value");;
    }
}
pub trait CheckTriviallyDestructible_long_double_Impl {
    fn operator_call(&self);
}
impl CheckTriviallyDestructible_long_double_Impl for Ptr<CheckTriviallyDestructible_long_double_> {
    fn operator_call(&self) {
        const _: () = assert!(true, "std::is_trivially_destructible<Atomic>::value");;
    }
}
pub trait CheckTriviallyDestructible_long_long_Impl {
    fn operator_call(&self);
}
impl CheckTriviallyDestructible_long_long_Impl for Ptr<CheckTriviallyDestructible_long_long_> {
    fn operator_call(&self) {
        const _: () = assert!(true, "std::is_trivially_destructible<Atomic>::value");;
    }
}
pub trait CheckTriviallyDestructible_short_Impl {
    fn operator_call(&self);
}
impl CheckTriviallyDestructible_short_Impl for Ptr<CheckTriviallyDestructible_short_> {
    fn operator_call(&self) {
        const _: () = assert!(true, "std::is_trivially_destructible<Atomic>::value");;
    }
}
pub trait CheckTriviallyDestructible_signed_char_Impl {
    fn operator_call(&self);
}
impl CheckTriviallyDestructible_signed_char_Impl for Ptr<CheckTriviallyDestructible_signed_char_> {
    fn operator_call(&self) {
        const _: () = assert!(true, "std::is_trivially_destructible<Atomic>::value");;
    }
}
pub trait CheckTriviallyDestructible_unsigned_char_Impl {
    fn operator_call(&self);
}
impl CheckTriviallyDestructible_unsigned_char_Impl
    for Ptr<CheckTriviallyDestructible_unsigned_char_>
{
    fn operator_call(&self) {
        const _: () = assert!(true, "std::is_trivially_destructible<Atomic>::value");;
    }
}
pub trait CheckTriviallyDestructible_unsigned_int_Impl {
    fn operator_call(&self);
}
impl CheckTriviallyDestructible_unsigned_int_Impl
    for Ptr<CheckTriviallyDestructible_unsigned_int_>
{
    fn operator_call(&self) {
        const _: () = assert!(true, "std::is_trivially_destructible<Atomic>::value");;
    }
}
pub trait CheckTriviallyDestructible_unsigned_long_Impl {
    fn operator_call(&self);
}
impl CheckTriviallyDestructible_unsigned_long_Impl
    for Ptr<CheckTriviallyDestructible_unsigned_long_>
{
    fn operator_call(&self) {
        const _: () = assert!(true, "std::is_trivially_destructible<Atomic>::value");;
    }
}
pub trait CheckTriviallyDestructible_unsigned_long_long_Impl {
    fn operator_call(&self);
}
impl CheckTriviallyDestructible_unsigned_long_long_Impl
    for Ptr<CheckTriviallyDestructible_unsigned_long_long_>
{
    fn operator_call(&self) {
        const _: () = assert!(true, "std::is_trivially_destructible<Atomic>::value");;
    }
}
pub trait CheckTriviallyDestructible_unsigned_short_Impl {
    fn operator_call(&self);
}
impl CheckTriviallyDestructible_unsigned_short_Impl
    for Ptr<CheckTriviallyDestructible_unsigned_short_>
{
    fn operator_call(&self) {
        const _: () = assert!(true, "std::is_trivially_destructible<Atomic>::value");;
    }
}
pub trait CheckTriviallyDestructible_wchar_t_Impl {
    fn operator_call(&self);
}
impl CheckTriviallyDestructible_wchar_t_Impl for Ptr<CheckTriviallyDestructible_wchar_t_> {
    fn operator_call(&self) {
        const _: () = assert!(true, "std::is_trivially_destructible<Atomic>::value");;
    }
}
pub trait TestEachFloatingPointType_CheckTriviallyDestructible_Impl {
    fn operator_call(&self);
}
impl TestEachFloatingPointType_CheckTriviallyDestructible_Impl
    for Ptr<TestEachFloatingPointType_CheckTriviallyDestructible_>
{
    fn operator_call(&self) {
        ({
            CheckTriviallyDestructible_float_Impl::operator_call(
                &Rc::new(RefCell::new(<CheckTriviallyDestructible_float_>::default())).as_pointer(),
            )
        });
        ({
            CheckTriviallyDestructible_double_Impl::operator_call(
                &Rc::new(RefCell::new(<CheckTriviallyDestructible_double_>::default()))
                    .as_pointer(),
            )
        });
        ({
            CheckTriviallyDestructible_long_double_Impl::operator_call(
                &Rc::new(RefCell::new(
                    <CheckTriviallyDestructible_long_double_>::default(),
                ))
                .as_pointer(),
            )
        });
    }
}
pub trait TestEachIntegralType_CheckTriviallyDestructible_Impl {
    fn operator_call(&self);
}
impl TestEachIntegralType_CheckTriviallyDestructible_Impl
    for Ptr<TestEachIntegralType_CheckTriviallyDestructible_>
{
    fn operator_call(&self) {
        ({
            CheckTriviallyDestructible_char_Impl::operator_call(
                &Rc::new(RefCell::new(<CheckTriviallyDestructible_char_>::default())).as_pointer(),
            )
        });
        ({
            CheckTriviallyDestructible_signed_char_Impl::operator_call(
                &Rc::new(RefCell::new(
                    <CheckTriviallyDestructible_signed_char_>::default(),
                ))
                .as_pointer(),
            )
        });
        ({
            CheckTriviallyDestructible_unsigned_char_Impl::operator_call(
                &Rc::new(RefCell::new(
                    <CheckTriviallyDestructible_unsigned_char_>::default(),
                ))
                .as_pointer(),
            )
        });
        ({
            CheckTriviallyDestructible_short_Impl::operator_call(
                &Rc::new(RefCell::new(<CheckTriviallyDestructible_short_>::default())).as_pointer(),
            )
        });
        ({
            CheckTriviallyDestructible_unsigned_short_Impl::operator_call(
                &Rc::new(RefCell::new(
                    <CheckTriviallyDestructible_unsigned_short_>::default(),
                ))
                .as_pointer(),
            )
        });
        ({
            CheckTriviallyDestructible_int_Impl::operator_call(
                &Rc::new(RefCell::new(<CheckTriviallyDestructible_int_>::default())).as_pointer(),
            )
        });
        ({
            CheckTriviallyDestructible_unsigned_int_Impl::operator_call(
                &Rc::new(RefCell::new(
                    <CheckTriviallyDestructible_unsigned_int_>::default(),
                ))
                .as_pointer(),
            )
        });
        ({
            CheckTriviallyDestructible_long_Impl::operator_call(
                &Rc::new(RefCell::new(<CheckTriviallyDestructible_long_>::default())).as_pointer(),
            )
        });
        ({
            CheckTriviallyDestructible_unsigned_long_Impl::operator_call(
                &Rc::new(RefCell::new(
                    <CheckTriviallyDestructible_unsigned_long_>::default(),
                ))
                .as_pointer(),
            )
        });
        ({
            CheckTriviallyDestructible_long_long_Impl::operator_call(
                &Rc::new(RefCell::new(
                    <CheckTriviallyDestructible_long_long_>::default(),
                ))
                .as_pointer(),
            )
        });
        ({
            CheckTriviallyDestructible_unsigned_long_long_Impl::operator_call(
                &Rc::new(RefCell::new(
                    <CheckTriviallyDestructible_unsigned_long_long_>::default(),
                ))
                .as_pointer(),
            )
        });
        ({
            CheckTriviallyDestructible_wchar_t_Impl::operator_call(
                &Rc::new(RefCell::new(
                    <CheckTriviallyDestructible_wchar_t_>::default(),
                ))
                .as_pointer(),
            )
        });
        ({
            CheckTriviallyDestructible_char8_t_Impl::operator_call(
                &Rc::new(RefCell::new(
                    <CheckTriviallyDestructible_char8_t_>::default(),
                ))
                .as_pointer(),
            )
        });
        ({
            CheckTriviallyDestructible_char16_t_Impl::operator_call(
                &Rc::new(RefCell::new(
                    <CheckTriviallyDestructible_char16_t_>::default(),
                ))
                .as_pointer(),
            )
        });
        ({
            CheckTriviallyDestructible_char32_t_Impl::operator_call(
                &Rc::new(RefCell::new(
                    <CheckTriviallyDestructible_char32_t_>::default(),
                ))
                .as_pointer(),
            )
        });
        ({
            CheckTriviallyDestructible_signed_char_Impl::operator_call(
                &Rc::new(RefCell::new(
                    <CheckTriviallyDestructible_signed_char_>::default(),
                ))
                .as_pointer(),
            )
        });
        ({
            CheckTriviallyDestructible_unsigned_char_Impl::operator_call(
                &Rc::new(RefCell::new(
                    <CheckTriviallyDestructible_unsigned_char_>::default(),
                ))
                .as_pointer(),
            )
        });
        ({
            CheckTriviallyDestructible_short_Impl::operator_call(
                &Rc::new(RefCell::new(<CheckTriviallyDestructible_short_>::default())).as_pointer(),
            )
        });
        ({
            CheckTriviallyDestructible_unsigned_short_Impl::operator_call(
                &Rc::new(RefCell::new(
                    <CheckTriviallyDestructible_unsigned_short_>::default(),
                ))
                .as_pointer(),
            )
        });
        ({
            CheckTriviallyDestructible_int_Impl::operator_call(
                &Rc::new(RefCell::new(<CheckTriviallyDestructible_int_>::default())).as_pointer(),
            )
        });
        ({
            CheckTriviallyDestructible_unsigned_int_Impl::operator_call(
                &Rc::new(RefCell::new(
                    <CheckTriviallyDestructible_unsigned_int_>::default(),
                ))
                .as_pointer(),
            )
        });
        ({
            CheckTriviallyDestructible_long_Impl::operator_call(
                &Rc::new(RefCell::new(<CheckTriviallyDestructible_long_>::default())).as_pointer(),
            )
        });
        ({
            CheckTriviallyDestructible_unsigned_long_Impl::operator_call(
                &Rc::new(RefCell::new(
                    <CheckTriviallyDestructible_unsigned_long_>::default(),
                ))
                .as_pointer(),
            )
        });
    }
}
pub trait TestEachPointerType_CheckTriviallyDestructible_Impl {
    fn operator_call(&self);
}
impl TestEachPointerType_CheckTriviallyDestructible_Impl
    for Ptr<TestEachPointerType_CheckTriviallyDestructible_>
{
    fn operator_call(&self) {
        ({
            CheckTriviallyDestructible_int_ptr_Impl::operator_call(
                &Rc::new(RefCell::new(
                    <CheckTriviallyDestructible_int_ptr_>::default(),
                ))
                .as_pointer(),
            )
        });
        ({
            CheckTriviallyDestructible_const_int_ptr_Impl::operator_call(
                &Rc::new(RefCell::new(
                    <CheckTriviallyDestructible_const_int_ptr_>::default(),
                ))
                .as_pointer(),
            )
        });
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
