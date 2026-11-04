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
pub struct TestEachIntegralType_TestFetchXor_ {}
impl Callable0<()> for TestEachIntegralType_TestFetchXor_ {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            TestEachIntegralType_TestFetchXor_Impl::operator_call(__it)
        })
    }
}
impl CppMove for TestEachIntegralType_TestFetchXor_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestEachFloatingPointType_TestDoesNotHaveFetchXor_ {}
impl Callable0<()> for TestEachFloatingPointType_TestDoesNotHaveFetchXor_ {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            TestEachFloatingPointType_TestDoesNotHaveFetchXor_Impl::operator_call(__it)
        })
    }
}
impl CppMove for TestEachFloatingPointType_TestDoesNotHaveFetchXor_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestEachPointerType_TestDoesNotHaveFetchXor_ {}
impl Callable0<()> for TestEachPointerType_TestDoesNotHaveFetchXor_ {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            TestEachPointerType_TestDoesNotHaveFetchXor_Impl::operator_call(__it)
        })
    }
}
impl CppMove for TestEachPointerType_TestDoesNotHaveFetchXor_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestDoesNotHaveFetchXor_bool_ {}
impl Callable0<()> for TestDoesNotHaveFetchXor_bool_ {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            TestDoesNotHaveFetchXor_bool_Impl::operator_call(__it)
        })
    }
}
impl CppMove for TestDoesNotHaveFetchXor_bool_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestDoesNotHaveFetchXor_UserAtomicType_ {}
impl Callable0<()> for TestDoesNotHaveFetchXor_UserAtomicType_ {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            TestDoesNotHaveFetchXor_UserAtomicType_Impl::operator_call(__it)
        })
    }
}
impl CppMove for TestDoesNotHaveFetchXor_UserAtomicType_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestDoesNotHaveFetchXor_LargeUserAtomicType_ {}
impl Callable0<()> for TestDoesNotHaveFetchXor_LargeUserAtomicType_ {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            TestDoesNotHaveFetchXor_LargeUserAtomicType_Impl::operator_call(__it)
        })
    }
}
impl CppMove for TestDoesNotHaveFetchXor_LargeUserAtomicType_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestDoesNotHaveFetchXor_float_ {}
impl Callable0<()> for TestDoesNotHaveFetchXor_float_ {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            TestDoesNotHaveFetchXor_float_Impl::operator_call(__it)
        })
    }
}
impl CppMove for TestDoesNotHaveFetchXor_float_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestDoesNotHaveFetchXor_double_ {}
impl Callable0<()> for TestDoesNotHaveFetchXor_double_ {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            TestDoesNotHaveFetchXor_double_Impl::operator_call(__it)
        })
    }
}
impl CppMove for TestDoesNotHaveFetchXor_double_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestDoesNotHaveFetchXor_long_double_ {}
impl Callable0<()> for TestDoesNotHaveFetchXor_long_double_ {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            TestDoesNotHaveFetchXor_long_double_Impl::operator_call(__it)
        })
    }
}
impl CppMove for TestDoesNotHaveFetchXor_long_double_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestDoesNotHaveFetchXor_int_ptr_ {}
impl Callable0<()> for TestDoesNotHaveFetchXor_int_ptr_ {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            TestDoesNotHaveFetchXor_int_ptr_Impl::operator_call(__it)
        })
    }
}
impl CppMove for TestDoesNotHaveFetchXor_int_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestDoesNotHaveFetchXor_const_int_ptr_ {}
impl Callable0<()> for TestDoesNotHaveFetchXor_const_int_ptr_ {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            TestDoesNotHaveFetchXor_const_int_ptr_Impl::operator_call(__it)
        })
    }
}
impl CppMove for TestDoesNotHaveFetchXor_const_int_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFetchXor_char_ {}
impl Callable0<()> for TestFetchXor_char_ {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            TestFetchXor_char_Impl::operator_call(__it)
        })
    }
}
impl CppMove for TestFetchXor_char_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFetchXor_signed_char_ {}
impl Callable0<()> for TestFetchXor_signed_char_ {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            TestFetchXor_signed_char_Impl::operator_call(__it)
        })
    }
}
impl CppMove for TestFetchXor_signed_char_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFetchXor_unsigned_char_ {}
impl Callable0<()> for TestFetchXor_unsigned_char_ {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            TestFetchXor_unsigned_char_Impl::operator_call(__it)
        })
    }
}
impl CppMove for TestFetchXor_unsigned_char_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFetchXor_short_ {}
impl Callable0<()> for TestFetchXor_short_ {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            TestFetchXor_short_Impl::operator_call(__it)
        })
    }
}
impl CppMove for TestFetchXor_short_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFetchXor_unsigned_short_ {}
impl Callable0<()> for TestFetchXor_unsigned_short_ {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            TestFetchXor_unsigned_short_Impl::operator_call(__it)
        })
    }
}
impl CppMove for TestFetchXor_unsigned_short_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFetchXor_int_ {}
impl Callable0<()> for TestFetchXor_int_ {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            TestFetchXor_int_Impl::operator_call(__it)
        })
    }
}
impl CppMove for TestFetchXor_int_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFetchXor_unsigned_int_ {}
impl Callable0<()> for TestFetchXor_unsigned_int_ {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            TestFetchXor_unsigned_int_Impl::operator_call(__it)
        })
    }
}
impl CppMove for TestFetchXor_unsigned_int_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFetchXor_long_ {}
impl Callable0<()> for TestFetchXor_long_ {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            TestFetchXor_long_Impl::operator_call(__it)
        })
    }
}
impl CppMove for TestFetchXor_long_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFetchXor_unsigned_long_ {}
impl Callable0<()> for TestFetchXor_unsigned_long_ {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            TestFetchXor_unsigned_long_Impl::operator_call(__it)
        })
    }
}
impl CppMove for TestFetchXor_unsigned_long_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFetchXor_long_long_ {}
impl Callable0<()> for TestFetchXor_long_long_ {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            TestFetchXor_long_long_Impl::operator_call(__it)
        })
    }
}
impl CppMove for TestFetchXor_long_long_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFetchXor_unsigned_long_long_ {}
impl Callable0<()> for TestFetchXor_unsigned_long_long_ {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            TestFetchXor_unsigned_long_long_Impl::operator_call(__it)
        })
    }
}
impl CppMove for TestFetchXor_unsigned_long_long_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFetchXor_wchar_t_ {}
impl Callable0<()> for TestFetchXor_wchar_t_ {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            TestFetchXor_wchar_t_Impl::operator_call(__it)
        })
    }
}
impl CppMove for TestFetchXor_wchar_t_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFetchXor_char8_t_ {}
impl Callable0<()> for TestFetchXor_char8_t_ {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            TestFetchXor_char8_t_Impl::operator_call(__it)
        })
    }
}
impl CppMove for TestFetchXor_char8_t_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFetchXor_char16_t_ {}
impl Callable0<()> for TestFetchXor_char16_t_ {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            TestFetchXor_char16_t_Impl::operator_call(__it)
        })
    }
}
impl CppMove for TestFetchXor_char16_t_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFetchXor_char32_t_ {}
impl Callable0<()> for TestFetchXor_char32_t_ {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            TestFetchXor_char32_t_Impl::operator_call(__it)
        })
    }
}
impl CppMove for TestFetchXor_char32_t_ {
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
        TestEachIntegralType_TestFetchXor_Impl::operator_call(
            &Rc::new(RefCell::new(<TestEachIntegralType_TestFetchXor_>::default())).as_pointer(),
        )
    });
    ({
        TestEachFloatingPointType_TestDoesNotHaveFetchXor_Impl::operator_call(
            &Rc::new(RefCell::new(
                <TestEachFloatingPointType_TestDoesNotHaveFetchXor_>::default(),
            ))
            .as_pointer(),
        )
    });
    ({
        TestEachPointerType_TestDoesNotHaveFetchXor_Impl::operator_call(
            &Rc::new(RefCell::new(
                <TestEachPointerType_TestDoesNotHaveFetchXor_>::default(),
            ))
            .as_pointer(),
        )
    });
    ({
        TestDoesNotHaveFetchXor_bool_Impl::operator_call(
            &Rc::new(RefCell::new(<TestDoesNotHaveFetchXor_bool_>::default())).as_pointer(),
        )
    });
    ({
        TestDoesNotHaveFetchXor_UserAtomicType_Impl::operator_call(
            &Rc::new(RefCell::new(
                <TestDoesNotHaveFetchXor_UserAtomicType_>::default(),
            ))
            .as_pointer(),
        )
    });
    ({
        TestDoesNotHaveFetchXor_LargeUserAtomicType_Impl::operator_call(
            &Rc::new(RefCell::new(
                <TestDoesNotHaveFetchXor_LargeUserAtomicType_>::default(),
            ))
            .as_pointer(),
        )
    });
    return 0;
}
pub trait TestDoesNotHaveFetchXor_LargeUserAtomicType_Impl {
    fn operator_call(&self);
}
impl TestDoesNotHaveFetchXor_LargeUserAtomicType_Impl
    for Ptr<TestDoesNotHaveFetchXor_LargeUserAtomicType_>
{
    fn operator_call(&self) {
        const _: () = assert!(!(false), "!has_fetch_xor<std::atomic_ref<T>>");;
    }
}
pub trait TestDoesNotHaveFetchXor_UserAtomicType_Impl {
    fn operator_call(&self);
}
impl TestDoesNotHaveFetchXor_UserAtomicType_Impl for Ptr<TestDoesNotHaveFetchXor_UserAtomicType_> {
    fn operator_call(&self) {
        const _: () = assert!(!(false), "!has_fetch_xor<std::atomic_ref<T>>");;
    }
}
pub trait TestDoesNotHaveFetchXor_bool_Impl {
    fn operator_call(&self);
}
impl TestDoesNotHaveFetchXor_bool_Impl for Ptr<TestDoesNotHaveFetchXor_bool_> {
    fn operator_call(&self) {
        const _: () = assert!(!(false), "!has_fetch_xor<std::atomic_ref<T>>");;
    }
}
pub trait TestDoesNotHaveFetchXor_const_int_ptr_Impl {
    fn operator_call(&self);
}
impl TestDoesNotHaveFetchXor_const_int_ptr_Impl for Ptr<TestDoesNotHaveFetchXor_const_int_ptr_> {
    fn operator_call(&self) {
        const _: () = assert!(!(false), "!has_fetch_xor<std::atomic_ref<T>>");;
    }
}
pub trait TestDoesNotHaveFetchXor_double_Impl {
    fn operator_call(&self);
}
impl TestDoesNotHaveFetchXor_double_Impl for Ptr<TestDoesNotHaveFetchXor_double_> {
    fn operator_call(&self) {
        const _: () = assert!(!(false), "!has_fetch_xor<std::atomic_ref<T>>");;
    }
}
pub trait TestDoesNotHaveFetchXor_float_Impl {
    fn operator_call(&self);
}
impl TestDoesNotHaveFetchXor_float_Impl for Ptr<TestDoesNotHaveFetchXor_float_> {
    fn operator_call(&self) {
        const _: () = assert!(!(false), "!has_fetch_xor<std::atomic_ref<T>>");;
    }
}
pub trait TestDoesNotHaveFetchXor_int_ptr_Impl {
    fn operator_call(&self);
}
impl TestDoesNotHaveFetchXor_int_ptr_Impl for Ptr<TestDoesNotHaveFetchXor_int_ptr_> {
    fn operator_call(&self) {
        const _: () = assert!(!(false), "!has_fetch_xor<std::atomic_ref<T>>");;
    }
}
pub trait TestDoesNotHaveFetchXor_long_double_Impl {
    fn operator_call(&self);
}
impl TestDoesNotHaveFetchXor_long_double_Impl for Ptr<TestDoesNotHaveFetchXor_long_double_> {
    fn operator_call(&self) {
        const _: () = assert!(!(false), "!has_fetch_xor<std::atomic_ref<T>>");;
    }
}
pub trait TestEachFloatingPointType_TestDoesNotHaveFetchXor_Impl {
    fn operator_call(&self);
}
impl TestEachFloatingPointType_TestDoesNotHaveFetchXor_Impl
    for Ptr<TestEachFloatingPointType_TestDoesNotHaveFetchXor_>
{
    fn operator_call(&self) {
        ({
            TestDoesNotHaveFetchXor_float_Impl::operator_call(
                &Rc::new(RefCell::new(<TestDoesNotHaveFetchXor_float_>::default())).as_pointer(),
            )
        });
        ({
            TestDoesNotHaveFetchXor_double_Impl::operator_call(
                &Rc::new(RefCell::new(<TestDoesNotHaveFetchXor_double_>::default())).as_pointer(),
            )
        });
        ({
            TestDoesNotHaveFetchXor_long_double_Impl::operator_call(
                &Rc::new(RefCell::new(
                    <TestDoesNotHaveFetchXor_long_double_>::default(),
                ))
                .as_pointer(),
            )
        });
    }
}
pub trait TestEachIntegralType_TestFetchXor_Impl {
    fn operator_call(&self);
}
impl TestEachIntegralType_TestFetchXor_Impl for Ptr<TestEachIntegralType_TestFetchXor_> {
    fn operator_call(&self) {
        ({
            TestFetchXor_char_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFetchXor_char_>::default())).as_pointer(),
            )
        });
        ({
            TestFetchXor_signed_char_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFetchXor_signed_char_>::default())).as_pointer(),
            )
        });
        ({
            TestFetchXor_unsigned_char_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFetchXor_unsigned_char_>::default())).as_pointer(),
            )
        });
        ({
            TestFetchXor_short_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFetchXor_short_>::default())).as_pointer(),
            )
        });
        ({
            TestFetchXor_unsigned_short_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFetchXor_unsigned_short_>::default())).as_pointer(),
            )
        });
        ({
            TestFetchXor_int_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFetchXor_int_>::default())).as_pointer(),
            )
        });
        ({
            TestFetchXor_unsigned_int_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFetchXor_unsigned_int_>::default())).as_pointer(),
            )
        });
        ({
            TestFetchXor_long_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFetchXor_long_>::default())).as_pointer(),
            )
        });
        ({
            TestFetchXor_unsigned_long_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFetchXor_unsigned_long_>::default())).as_pointer(),
            )
        });
        ({
            TestFetchXor_long_long_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFetchXor_long_long_>::default())).as_pointer(),
            )
        });
        ({
            TestFetchXor_unsigned_long_long_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFetchXor_unsigned_long_long_>::default())).as_pointer(),
            )
        });
        ({
            TestFetchXor_wchar_t_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFetchXor_wchar_t_>::default())).as_pointer(),
            )
        });
        ({
            TestFetchXor_char8_t_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFetchXor_char8_t_>::default())).as_pointer(),
            )
        });
        ({
            TestFetchXor_char16_t_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFetchXor_char16_t_>::default())).as_pointer(),
            )
        });
        ({
            TestFetchXor_char32_t_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFetchXor_char32_t_>::default())).as_pointer(),
            )
        });
        ({
            TestFetchXor_signed_char_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFetchXor_signed_char_>::default())).as_pointer(),
            )
        });
        ({
            TestFetchXor_unsigned_char_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFetchXor_unsigned_char_>::default())).as_pointer(),
            )
        });
        ({
            TestFetchXor_short_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFetchXor_short_>::default())).as_pointer(),
            )
        });
        ({
            TestFetchXor_unsigned_short_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFetchXor_unsigned_short_>::default())).as_pointer(),
            )
        });
        ({
            TestFetchXor_int_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFetchXor_int_>::default())).as_pointer(),
            )
        });
        ({
            TestFetchXor_unsigned_int_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFetchXor_unsigned_int_>::default())).as_pointer(),
            )
        });
        ({
            TestFetchXor_long_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFetchXor_long_>::default())).as_pointer(),
            )
        });
        ({
            TestFetchXor_unsigned_long_Impl::operator_call(
                &Rc::new(RefCell::new(<TestFetchXor_unsigned_long_>::default())).as_pointer(),
            )
        });
    }
}
pub trait TestEachPointerType_TestDoesNotHaveFetchXor_Impl {
    fn operator_call(&self);
}
impl TestEachPointerType_TestDoesNotHaveFetchXor_Impl
    for Ptr<TestEachPointerType_TestDoesNotHaveFetchXor_>
{
    fn operator_call(&self) {
        ({
            TestDoesNotHaveFetchXor_int_ptr_Impl::operator_call(
                &Rc::new(RefCell::new(<TestDoesNotHaveFetchXor_int_ptr_>::default())).as_pointer(),
            )
        });
        ({
            TestDoesNotHaveFetchXor_const_int_ptr_Impl::operator_call(
                &Rc::new(RefCell::new(
                    <TestDoesNotHaveFetchXor_const_int_ptr_>::default(),
                ))
                .as_pointer(),
            )
        });
    }
}
pub trait TestFetchXor_char16_t_Impl {
    fn operator_call(&self);
}
impl TestFetchXor_char16_t_Impl for Ptr<TestFetchXor_char16_t_> {
    fn operator_call(&self) {
        const _: () = assert!(true, "std::is_integral_v<T>");;
        let x: Value<u16> = Rc::new(RefCell::new(1_u16));
        let a: Value<std_atomic_ref_char16_t_> =
            Rc::new(RefCell::new(std_atomic_ref_char16_t_::new({
                x.as_pointer()
            })));
        {
            let mut y: u16 = ({
                ((*a.borrow()) as std___atomic_ref_char16_t__true__false_).fetch_xor(2_u16, None)
            });
            assert!(((y as i32) == (1_u16 as i32)));
            assert!(({ (((*x.borrow()) as i32) == (3_u16 as i32)) }));
            const _: () = assert!(true, "");;
        }
        {
            let mut y: u16 = ({
                ((*a.borrow()) as std___atomic_ref_char16_t__true__false_)
                    .fetch_xor(2_u16, Some(memory_order_relaxed_14.with(|rc| *rc.borrow())))
            });
            assert!(((y as i32) == (3_u16 as i32)));
            assert!(({ (((*x.borrow()) as i32) == (1_u16 as i32)) }));
            const _: () = assert!(true, "");;
        }
    }
}
pub trait TestFetchXor_char32_t_Impl {
    fn operator_call(&self);
}
impl TestFetchXor_char32_t_Impl for Ptr<TestFetchXor_char32_t_> {
    fn operator_call(&self) {
        const _: () = assert!(true, "std::is_integral_v<T>");;
        let x: Value<u32> = Rc::new(RefCell::new(1_u32));
        let a: Value<std_atomic_ref_char32_t_> =
            Rc::new(RefCell::new(std_atomic_ref_char32_t_::new({
                x.as_pointer()
            })));
        {
            let mut y: u32 = ({
                ((*a.borrow()) as std___atomic_ref_char32_t__true__false_).fetch_xor(2_u32, None)
            });
            assert!((y == 1_u32));
            assert!(({ ((*x.borrow()) == 3_u32) }));
            const _: () = assert!(true, "");;
        }
        {
            let mut y: u32 = ({
                ((*a.borrow()) as std___atomic_ref_char32_t__true__false_)
                    .fetch_xor(2_u32, Some(memory_order_relaxed_14.with(|rc| *rc.borrow())))
            });
            assert!((y == 3_u32));
            assert!(({ ((*x.borrow()) == 1_u32) }));
            const _: () = assert!(true, "");;
        }
    }
}
pub trait TestFetchXor_char8_t_Impl {
    fn operator_call(&self);
}
impl TestFetchXor_char8_t_Impl for Ptr<TestFetchXor_char8_t_> {
    fn operator_call(&self) {
        const _: () = assert!(true, "std::is_integral_v<T>");;
        let x: Value<u8> = Rc::new(RefCell::new(1_u8));
        let a: Value<std_atomic_ref_char8_t_> =
            Rc::new(RefCell::new(std_atomic_ref_char8_t_::new({
                x.as_pointer()
            })));
        {
            let mut y: u8 = ({
                ((*a.borrow()) as std___atomic_ref_char8_t__true__false_).fetch_xor(2_u8, None)
            });
            assert!(((y as i32) == (1_u8 as i32)));
            assert!(({ (((*x.borrow()) as i32) == (3_u8 as i32)) }));
            const _: () = assert!(true, "");;
        }
        {
            let mut y: u8 = ({
                ((*a.borrow()) as std___atomic_ref_char8_t__true__false_)
                    .fetch_xor(2_u8, Some(memory_order_relaxed_14.with(|rc| *rc.borrow())))
            });
            assert!(((y as i32) == (3_u8 as i32)));
            assert!(({ (((*x.borrow()) as i32) == (1_u8 as i32)) }));
            const _: () = assert!(true, "");;
        }
    }
}
pub trait TestFetchXor_char_Impl {
    fn operator_call(&self);
}
impl TestFetchXor_char_Impl for Ptr<TestFetchXor_char_> {
    fn operator_call(&self) {
        const _: () = assert!(true, "std::is_integral_v<T>");;
        let x: Value<i8> = Rc::new(RefCell::new(1_i8));
        let a: Value<std_atomic_ref_char_> =
            Rc::new(RefCell::new(std_atomic_ref_char_::new({ x.as_pointer() })));
        {
            let mut y: i8 =
                ({ ((*a.borrow()) as std___atomic_ref_char__true__false_).fetch_xor(2_i8, None) });
            assert!(((y as i32) == (1_i8 as i32)));
            assert!(({ (((*x.borrow()) as i32) == (3_i8 as i32)) }));
            const _: () = assert!(true, "");;
        }
        {
            let mut y: i8 = ({
                ((*a.borrow()) as std___atomic_ref_char__true__false_)
                    .fetch_xor(2_i8, Some(memory_order_relaxed_14.with(|rc| *rc.borrow())))
            });
            assert!(((y as i32) == (3_i8 as i32)));
            assert!(({ (((*x.borrow()) as i32) == (1_i8 as i32)) }));
            const _: () = assert!(true, "");;
        }
    }
}
pub trait TestFetchXor_int_Impl {
    fn operator_call(&self);
}
impl TestFetchXor_int_Impl for Ptr<TestFetchXor_int_> {
    fn operator_call(&self) {
        const _: () = assert!(true, "std::is_integral_v<T>");;
        let x: Value<i32> = Rc::new(RefCell::new(1));
        let a: Value<std_atomic_ref_int_> =
            Rc::new(RefCell::new(std_atomic_ref_int_::new({ x.as_pointer() })));
        {
            let mut y: i32 =
                ({ ((*a.borrow()) as std___atomic_ref_int__true__false_).fetch_xor(2, None) });
            assert!((y == 1));
            assert!(({ ((*x.borrow()) == 3) }));
            const _: () = assert!(true, "");;
        }
        {
            let mut y: i32 = ({
                ((*a.borrow()) as std___atomic_ref_int__true__false_)
                    .fetch_xor(2, Some(memory_order_relaxed_14.with(|rc| *rc.borrow())))
            });
            assert!((y == 3));
            assert!(({ ((*x.borrow()) == 1) }));
            const _: () = assert!(true, "");;
        }
    }
}
pub trait TestFetchXor_long_Impl {
    fn operator_call(&self);
}
impl TestFetchXor_long_Impl for Ptr<TestFetchXor_long_> {
    fn operator_call(&self) {
        const _: () = assert!(true, "std::is_integral_v<T>");;
        let x: Value<i64> = Rc::new(RefCell::new(1_i64));
        let a: Value<std_atomic_ref_long_> =
            Rc::new(RefCell::new(std_atomic_ref_long_::new({ x.as_pointer() })));
        {
            let mut y: i64 =
                ({ ((*a.borrow()) as std___atomic_ref_long__true__false_).fetch_xor(2_i64, None) });
            assert!((y == 1_i64));
            assert!(({ ((*x.borrow()) == 3_i64) }));
            const _: () = assert!(true, "");;
        }
        {
            let mut y: i64 = ({
                ((*a.borrow()) as std___atomic_ref_long__true__false_)
                    .fetch_xor(2_i64, Some(memory_order_relaxed_14.with(|rc| *rc.borrow())))
            });
            assert!((y == 3_i64));
            assert!(({ ((*x.borrow()) == 1_i64) }));
            const _: () = assert!(true, "");;
        }
    }
}
pub trait TestFetchXor_long_long_Impl {
    fn operator_call(&self);
}
impl TestFetchXor_long_long_Impl for Ptr<TestFetchXor_long_long_> {
    fn operator_call(&self) {
        const _: () = assert!(true, "std::is_integral_v<T>");;
        let x: Value<i64> = Rc::new(RefCell::new(1_i64));
        let a: Value<std_atomic_ref_long_long_> =
            Rc::new(RefCell::new(std_atomic_ref_long_long_::new({
                x.as_pointer()
            })));
        {
            let mut y: i64 = ({
                ((*a.borrow()) as std___atomic_ref_long_long__true__false_).fetch_xor(2_i64, None)
            });
            assert!((y == 1_i64));
            assert!(({ ((*x.borrow()) == 3_i64) }));
            const _: () = assert!(true, "");;
        }
        {
            let mut y: i64 = ({
                ((*a.borrow()) as std___atomic_ref_long_long__true__false_)
                    .fetch_xor(2_i64, Some(memory_order_relaxed_14.with(|rc| *rc.borrow())))
            });
            assert!((y == 3_i64));
            assert!(({ ((*x.borrow()) == 1_i64) }));
            const _: () = assert!(true, "");;
        }
    }
}
pub trait TestFetchXor_short_Impl {
    fn operator_call(&self);
}
impl TestFetchXor_short_Impl for Ptr<TestFetchXor_short_> {
    fn operator_call(&self) {
        const _: () = assert!(true, "std::is_integral_v<T>");;
        let x: Value<i16> = Rc::new(RefCell::new(1_i16));
        let a: Value<std_atomic_ref_short_> =
            Rc::new(RefCell::new(std_atomic_ref_short_::new({ x.as_pointer() })));
        {
            let mut y: i16 = ({
                ((*a.borrow()) as std___atomic_ref_short__true__false_).fetch_xor(2_i16, None)
            });
            assert!(((y as i32) == (1_i16 as i32)));
            assert!(({ (((*x.borrow()) as i32) == (3_i16 as i32)) }));
            const _: () = assert!(true, "");;
        }
        {
            let mut y: i16 = ({
                ((*a.borrow()) as std___atomic_ref_short__true__false_)
                    .fetch_xor(2_i16, Some(memory_order_relaxed_14.with(|rc| *rc.borrow())))
            });
            assert!(((y as i32) == (3_i16 as i32)));
            assert!(({ (((*x.borrow()) as i32) == (1_i16 as i32)) }));
            const _: () = assert!(true, "");;
        }
    }
}
pub trait TestFetchXor_signed_char_Impl {
    fn operator_call(&self);
}
impl TestFetchXor_signed_char_Impl for Ptr<TestFetchXor_signed_char_> {
    fn operator_call(&self) {
        const _: () = assert!(true, "std::is_integral_v<T>");;
        let x: Value<i8> = Rc::new(RefCell::new(1_i8));
        let a: Value<std_atomic_ref_signed_char_> =
            Rc::new(RefCell::new(std_atomic_ref_signed_char_::new({
                x.as_pointer()
            })));
        {
            let mut y: i8 = ({
                ((*a.borrow()) as std___atomic_ref_signed_char__true__false_).fetch_xor(2_i8, None)
            });
            assert!(((y as i32) == (1_i8 as i32)));
            assert!(({ (((*x.borrow()) as i32) == (3_i8 as i32)) }));
            const _: () = assert!(true, "");;
        }
        {
            let mut y: i8 = ({
                ((*a.borrow()) as std___atomic_ref_signed_char__true__false_)
                    .fetch_xor(2_i8, Some(memory_order_relaxed_14.with(|rc| *rc.borrow())))
            });
            assert!(((y as i32) == (3_i8 as i32)));
            assert!(({ (((*x.borrow()) as i32) == (1_i8 as i32)) }));
            const _: () = assert!(true, "");;
        }
    }
}
pub trait TestFetchXor_unsigned_char_Impl {
    fn operator_call(&self);
}
impl TestFetchXor_unsigned_char_Impl for Ptr<TestFetchXor_unsigned_char_> {
    fn operator_call(&self) {
        const _: () = assert!(true, "std::is_integral_v<T>");;
        let x: Value<u8> = Rc::new(RefCell::new(1_u8));
        let a: Value<std_atomic_ref_unsigned_char_> =
            Rc::new(RefCell::new(std_atomic_ref_unsigned_char_::new({
                x.as_pointer()
            })));
        {
            let mut y: u8 = ({
                ((*a.borrow()) as std___atomic_ref_unsigned_char__true__false_)
                    .fetch_xor(2_u8, None)
            });
            assert!(((y as i32) == (1_u8 as i32)));
            assert!(({ (((*x.borrow()) as i32) == (3_u8 as i32)) }));
            const _: () = assert!(true, "");;
        }
        {
            let mut y: u8 = ({
                ((*a.borrow()) as std___atomic_ref_unsigned_char__true__false_)
                    .fetch_xor(2_u8, Some(memory_order_relaxed_14.with(|rc| *rc.borrow())))
            });
            assert!(((y as i32) == (3_u8 as i32)));
            assert!(({ (((*x.borrow()) as i32) == (1_u8 as i32)) }));
            const _: () = assert!(true, "");;
        }
    }
}
pub trait TestFetchXor_unsigned_int_Impl {
    fn operator_call(&self);
}
impl TestFetchXor_unsigned_int_Impl for Ptr<TestFetchXor_unsigned_int_> {
    fn operator_call(&self) {
        const _: () = assert!(true, "std::is_integral_v<T>");;
        let x: Value<u32> = Rc::new(RefCell::new(1_u32));
        let a: Value<std_atomic_ref_unsigned_int_> =
            Rc::new(RefCell::new(std_atomic_ref_unsigned_int_::new({
                x.as_pointer()
            })));
        {
            let mut y: u32 = ({
                ((*a.borrow()) as std___atomic_ref_unsigned_int__true__false_)
                    .fetch_xor(2_u32, None)
            });
            assert!((y == 1_u32));
            assert!(({ ((*x.borrow()) == 3_u32) }));
            const _: () = assert!(true, "");;
        }
        {
            let mut y: u32 = ({
                ((*a.borrow()) as std___atomic_ref_unsigned_int__true__false_)
                    .fetch_xor(2_u32, Some(memory_order_relaxed_14.with(|rc| *rc.borrow())))
            });
            assert!((y == 3_u32));
            assert!(({ ((*x.borrow()) == 1_u32) }));
            const _: () = assert!(true, "");;
        }
    }
}
pub trait TestFetchXor_unsigned_long_Impl {
    fn operator_call(&self);
}
impl TestFetchXor_unsigned_long_Impl for Ptr<TestFetchXor_unsigned_long_> {
    fn operator_call(&self) {
        const _: () = assert!(true, "std::is_integral_v<T>");;
        let x: Value<u64> = Rc::new(RefCell::new(1_u64));
        let a: Value<std_atomic_ref_unsigned_long_> =
            Rc::new(RefCell::new(std_atomic_ref_unsigned_long_::new({
                x.as_pointer()
            })));
        {
            let mut y: u64 = ({
                ((*a.borrow()) as std___atomic_ref_unsigned_long__true__false_)
                    .fetch_xor(2_u64, None)
            });
            assert!((y == 1_u64));
            assert!(({ ((*x.borrow()) == 3_u64) }));
            const _: () = assert!(true, "");;
        }
        {
            let mut y: u64 = ({
                ((*a.borrow()) as std___atomic_ref_unsigned_long__true__false_)
                    .fetch_xor(2_u64, Some(memory_order_relaxed_14.with(|rc| *rc.borrow())))
            });
            assert!((y == 3_u64));
            assert!(({ ((*x.borrow()) == 1_u64) }));
            const _: () = assert!(true, "");;
        }
    }
}
pub trait TestFetchXor_unsigned_long_long_Impl {
    fn operator_call(&self);
}
impl TestFetchXor_unsigned_long_long_Impl for Ptr<TestFetchXor_unsigned_long_long_> {
    fn operator_call(&self) {
        const _: () = assert!(true, "std::is_integral_v<T>");;
        let x: Value<u64> = Rc::new(RefCell::new(1_u64));
        let a: Value<std_atomic_ref_unsigned_long_long_> =
            Rc::new(RefCell::new(std_atomic_ref_unsigned_long_long_::new({
                x.as_pointer()
            })));
        {
            let mut y: u64 = ({
                ((*a.borrow()) as std___atomic_ref_unsigned_long_long__true__false_)
                    .fetch_xor(2_u64, None)
            });
            assert!((y == 1_u64));
            assert!(({ ((*x.borrow()) == 3_u64) }));
            const _: () = assert!(true, "");;
        }
        {
            let mut y: u64 = ({
                ((*a.borrow()) as std___atomic_ref_unsigned_long_long__true__false_)
                    .fetch_xor(2_u64, Some(memory_order_relaxed_14.with(|rc| *rc.borrow())))
            });
            assert!((y == 3_u64));
            assert!(({ ((*x.borrow()) == 1_u64) }));
            const _: () = assert!(true, "");;
        }
    }
}
pub trait TestFetchXor_unsigned_short_Impl {
    fn operator_call(&self);
}
impl TestFetchXor_unsigned_short_Impl for Ptr<TestFetchXor_unsigned_short_> {
    fn operator_call(&self) {
        const _: () = assert!(true, "std::is_integral_v<T>");;
        let x: Value<u16> = Rc::new(RefCell::new(1_u16));
        let a: Value<std_atomic_ref_unsigned_short_> =
            Rc::new(RefCell::new(std_atomic_ref_unsigned_short_::new({
                x.as_pointer()
            })));
        {
            let mut y: u16 = ({
                ((*a.borrow()) as std___atomic_ref_unsigned_short__true__false_)
                    .fetch_xor(2_u16, None)
            });
            assert!(((y as i32) == (1_u16 as i32)));
            assert!(({ (((*x.borrow()) as i32) == (3_u16 as i32)) }));
            const _: () = assert!(true, "");;
        }
        {
            let mut y: u16 = ({
                ((*a.borrow()) as std___atomic_ref_unsigned_short__true__false_)
                    .fetch_xor(2_u16, Some(memory_order_relaxed_14.with(|rc| *rc.borrow())))
            });
            assert!(((y as i32) == (3_u16 as i32)));
            assert!(({ (((*x.borrow()) as i32) == (1_u16 as i32)) }));
            const _: () = assert!(true, "");;
        }
    }
}
pub trait TestFetchXor_wchar_t_Impl {
    fn operator_call(&self);
}
impl TestFetchXor_wchar_t_Impl for Ptr<TestFetchXor_wchar_t_> {
    fn operator_call(&self) {
        const _: () = assert!(true, "std::is_integral_v<T>");;
        let x: Value<i32> = Rc::new(RefCell::new(1_i32));
        let a: Value<std_atomic_ref_wchar_t_> =
            Rc::new(RefCell::new(std_atomic_ref_wchar_t_::new({
                x.as_pointer()
            })));
        {
            let mut y: i32 = ({
                ((*a.borrow()) as std___atomic_ref_wchar_t__true__false_).fetch_xor(2_i32, None)
            });
            assert!((y == 1_i32));
            assert!(({ ((*x.borrow()) == 3_i32) }));
            const _: () = assert!(true, "");;
        }
        {
            let mut y: i32 = ({
                ((*a.borrow()) as std___atomic_ref_wchar_t__true__false_)
                    .fetch_xor(2_i32, Some(memory_order_relaxed_14.with(|rc| *rc.borrow())))
            });
            assert!((y == 3_i32));
            assert!(({ ((*x.borrow()) == 1_i32) }));
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
