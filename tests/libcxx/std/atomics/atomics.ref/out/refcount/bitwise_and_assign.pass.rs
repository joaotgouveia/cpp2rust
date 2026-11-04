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
pub struct TestEachIntegralType_TestBitwiseAndAssign_ {}
impl Callable0<()> for TestEachIntegralType_TestBitwiseAndAssign_ {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            TestEachIntegralType_TestBitwiseAndAssign_Impl::operator_call(__it)
        })
    }
}
impl CppMove for TestEachIntegralType_TestBitwiseAndAssign_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestEachFloatingPointType_TestDoesNotHaveBitwiseAndAssign_ {}
impl Callable0<()> for TestEachFloatingPointType_TestDoesNotHaveBitwiseAndAssign_ {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            TestEachFloatingPointType_TestDoesNotHaveBitwiseAndAssign_Impl::operator_call(__it)
        })
    }
}
impl CppMove for TestEachFloatingPointType_TestDoesNotHaveBitwiseAndAssign_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestEachPointerType_TestDoesNotHaveBitwiseAndAssign_ {}
impl Callable0<()> for TestEachPointerType_TestDoesNotHaveBitwiseAndAssign_ {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            TestEachPointerType_TestDoesNotHaveBitwiseAndAssign_Impl::operator_call(__it)
        })
    }
}
impl CppMove for TestEachPointerType_TestDoesNotHaveBitwiseAndAssign_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestDoesNotHaveBitwiseAndAssign_bool_ {}
impl Callable0<()> for TestDoesNotHaveBitwiseAndAssign_bool_ {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            TestDoesNotHaveBitwiseAndAssign_bool_Impl::operator_call(__it)
        })
    }
}
impl CppMove for TestDoesNotHaveBitwiseAndAssign_bool_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestDoesNotHaveBitwiseAndAssign_UserAtomicType_ {}
impl Callable0<()> for TestDoesNotHaveBitwiseAndAssign_UserAtomicType_ {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            TestDoesNotHaveBitwiseAndAssign_UserAtomicType_Impl::operator_call(__it)
        })
    }
}
impl CppMove for TestDoesNotHaveBitwiseAndAssign_UserAtomicType_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestDoesNotHaveBitwiseAndAssign_LargeUserAtomicType_ {}
impl Callable0<()> for TestDoesNotHaveBitwiseAndAssign_LargeUserAtomicType_ {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            TestDoesNotHaveBitwiseAndAssign_LargeUserAtomicType_Impl::operator_call(__it)
        })
    }
}
impl CppMove for TestDoesNotHaveBitwiseAndAssign_LargeUserAtomicType_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestDoesNotHaveBitwiseAndAssign_float_ {}
impl Callable0<()> for TestDoesNotHaveBitwiseAndAssign_float_ {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            TestDoesNotHaveBitwiseAndAssign_float_Impl::operator_call(__it)
        })
    }
}
impl CppMove for TestDoesNotHaveBitwiseAndAssign_float_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestDoesNotHaveBitwiseAndAssign_double_ {}
impl Callable0<()> for TestDoesNotHaveBitwiseAndAssign_double_ {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            TestDoesNotHaveBitwiseAndAssign_double_Impl::operator_call(__it)
        })
    }
}
impl CppMove for TestDoesNotHaveBitwiseAndAssign_double_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestDoesNotHaveBitwiseAndAssign_long_double_ {}
impl Callable0<()> for TestDoesNotHaveBitwiseAndAssign_long_double_ {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            TestDoesNotHaveBitwiseAndAssign_long_double_Impl::operator_call(__it)
        })
    }
}
impl CppMove for TestDoesNotHaveBitwiseAndAssign_long_double_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestDoesNotHaveBitwiseAndAssign_int_ptr_ {}
impl Callable0<()> for TestDoesNotHaveBitwiseAndAssign_int_ptr_ {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            TestDoesNotHaveBitwiseAndAssign_int_ptr_Impl::operator_call(__it)
        })
    }
}
impl CppMove for TestDoesNotHaveBitwiseAndAssign_int_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestDoesNotHaveBitwiseAndAssign_const_int_ptr_ {}
impl Callable0<()> for TestDoesNotHaveBitwiseAndAssign_const_int_ptr_ {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            TestDoesNotHaveBitwiseAndAssign_const_int_ptr_Impl::operator_call(__it)
        })
    }
}
impl CppMove for TestDoesNotHaveBitwiseAndAssign_const_int_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestBitwiseAndAssign_char_ {}
impl Callable0<()> for TestBitwiseAndAssign_char_ {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            TestBitwiseAndAssign_char_Impl::operator_call(__it)
        })
    }
}
impl CppMove for TestBitwiseAndAssign_char_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestBitwiseAndAssign_signed_char_ {}
impl Callable0<()> for TestBitwiseAndAssign_signed_char_ {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            TestBitwiseAndAssign_signed_char_Impl::operator_call(__it)
        })
    }
}
impl CppMove for TestBitwiseAndAssign_signed_char_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestBitwiseAndAssign_unsigned_char_ {}
impl Callable0<()> for TestBitwiseAndAssign_unsigned_char_ {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            TestBitwiseAndAssign_unsigned_char_Impl::operator_call(__it)
        })
    }
}
impl CppMove for TestBitwiseAndAssign_unsigned_char_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestBitwiseAndAssign_short_ {}
impl Callable0<()> for TestBitwiseAndAssign_short_ {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            TestBitwiseAndAssign_short_Impl::operator_call(__it)
        })
    }
}
impl CppMove for TestBitwiseAndAssign_short_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestBitwiseAndAssign_unsigned_short_ {}
impl Callable0<()> for TestBitwiseAndAssign_unsigned_short_ {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            TestBitwiseAndAssign_unsigned_short_Impl::operator_call(__it)
        })
    }
}
impl CppMove for TestBitwiseAndAssign_unsigned_short_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestBitwiseAndAssign_int_ {}
impl Callable0<()> for TestBitwiseAndAssign_int_ {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            TestBitwiseAndAssign_int_Impl::operator_call(__it)
        })
    }
}
impl CppMove for TestBitwiseAndAssign_int_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestBitwiseAndAssign_unsigned_int_ {}
impl Callable0<()> for TestBitwiseAndAssign_unsigned_int_ {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            TestBitwiseAndAssign_unsigned_int_Impl::operator_call(__it)
        })
    }
}
impl CppMove for TestBitwiseAndAssign_unsigned_int_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestBitwiseAndAssign_long_ {}
impl Callable0<()> for TestBitwiseAndAssign_long_ {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            TestBitwiseAndAssign_long_Impl::operator_call(__it)
        })
    }
}
impl CppMove for TestBitwiseAndAssign_long_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestBitwiseAndAssign_unsigned_long_ {}
impl Callable0<()> for TestBitwiseAndAssign_unsigned_long_ {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            TestBitwiseAndAssign_unsigned_long_Impl::operator_call(__it)
        })
    }
}
impl CppMove for TestBitwiseAndAssign_unsigned_long_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestBitwiseAndAssign_long_long_ {}
impl Callable0<()> for TestBitwiseAndAssign_long_long_ {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            TestBitwiseAndAssign_long_long_Impl::operator_call(__it)
        })
    }
}
impl CppMove for TestBitwiseAndAssign_long_long_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestBitwiseAndAssign_unsigned_long_long_ {}
impl Callable0<()> for TestBitwiseAndAssign_unsigned_long_long_ {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            TestBitwiseAndAssign_unsigned_long_long_Impl::operator_call(__it)
        })
    }
}
impl CppMove for TestBitwiseAndAssign_unsigned_long_long_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestBitwiseAndAssign_wchar_t_ {}
impl Callable0<()> for TestBitwiseAndAssign_wchar_t_ {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            TestBitwiseAndAssign_wchar_t_Impl::operator_call(__it)
        })
    }
}
impl CppMove for TestBitwiseAndAssign_wchar_t_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestBitwiseAndAssign_char8_t_ {}
impl Callable0<()> for TestBitwiseAndAssign_char8_t_ {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            TestBitwiseAndAssign_char8_t_Impl::operator_call(__it)
        })
    }
}
impl CppMove for TestBitwiseAndAssign_char8_t_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestBitwiseAndAssign_char16_t_ {}
impl Callable0<()> for TestBitwiseAndAssign_char16_t_ {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            TestBitwiseAndAssign_char16_t_Impl::operator_call(__it)
        })
    }
}
impl CppMove for TestBitwiseAndAssign_char16_t_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestBitwiseAndAssign_char32_t_ {}
impl Callable0<()> for TestBitwiseAndAssign_char32_t_ {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            TestBitwiseAndAssign_char32_t_Impl::operator_call(__it)
        })
    }
}
impl CppMove for TestBitwiseAndAssign_char32_t_ {
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
        TestEachIntegralType_TestBitwiseAndAssign_Impl::operator_call(
            &Rc::new(RefCell::new(
                <TestEachIntegralType_TestBitwiseAndAssign_>::default(),
            ))
            .as_pointer(),
        )
    });
    ({
        TestEachFloatingPointType_TestDoesNotHaveBitwiseAndAssign_Impl::operator_call(
            &Rc::new(RefCell::new(
                <TestEachFloatingPointType_TestDoesNotHaveBitwiseAndAssign_>::default(),
            ))
            .as_pointer(),
        )
    });
    ({
        TestEachPointerType_TestDoesNotHaveBitwiseAndAssign_Impl::operator_call(
            &Rc::new(RefCell::new(
                <TestEachPointerType_TestDoesNotHaveBitwiseAndAssign_>::default(),
            ))
            .as_pointer(),
        )
    });
    ({
        TestDoesNotHaveBitwiseAndAssign_bool_Impl::operator_call(
            &Rc::new(RefCell::new(
                <TestDoesNotHaveBitwiseAndAssign_bool_>::default(),
            ))
            .as_pointer(),
        )
    });
    ({
        TestDoesNotHaveBitwiseAndAssign_UserAtomicType_Impl::operator_call(
            &Rc::new(RefCell::new(
                <TestDoesNotHaveBitwiseAndAssign_UserAtomicType_>::default(),
            ))
            .as_pointer(),
        )
    });
    ({
        TestDoesNotHaveBitwiseAndAssign_LargeUserAtomicType_Impl::operator_call(
            &Rc::new(RefCell::new(
                <TestDoesNotHaveBitwiseAndAssign_LargeUserAtomicType_>::default(),
            ))
            .as_pointer(),
        )
    });
    return 0;
}
pub trait TestBitwiseAndAssign_char16_t_Impl {
    fn operator_call(&self);
}
impl TestBitwiseAndAssign_char16_t_Impl for Ptr<TestBitwiseAndAssign_char16_t_> {
    fn operator_call(&self) {
        const _: () = assert!(true, "std::is_integral_v<T>");;
        let mut x: u16 = 1_u16;
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut y: u16 = (unimplemented!());
        assert!(((y as i32) == (1_u16 as i32)));
        assert!(((x as i32) == (1_u16 as i32)));
        const _: () = assert!(true, "");;
        y = (unimplemented!());
        assert!(((y as i32) == (0_u16 as i32)));
        assert!(((x as i32) == (0_u16 as i32)));
    }
}
pub trait TestBitwiseAndAssign_char32_t_Impl {
    fn operator_call(&self);
}
impl TestBitwiseAndAssign_char32_t_Impl for Ptr<TestBitwiseAndAssign_char32_t_> {
    fn operator_call(&self) {
        const _: () = assert!(true, "std::is_integral_v<T>");;
        let mut x: u32 = 1_u32;
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut y: u32 = (unimplemented!());
        assert!((y == 1_u32));
        assert!((x == 1_u32));
        const _: () = assert!(true, "");;
        y = (unimplemented!());
        assert!((y == 0_u32));
        assert!((x == 0_u32));
    }
}
pub trait TestBitwiseAndAssign_char8_t_Impl {
    fn operator_call(&self);
}
impl TestBitwiseAndAssign_char8_t_Impl for Ptr<TestBitwiseAndAssign_char8_t_> {
    fn operator_call(&self) {
        const _: () = assert!(true, "std::is_integral_v<T>");;
        let mut x: u8 = 1_u8;
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut y: u8 = (unimplemented!());
        assert!(((y as i32) == (1_u8 as i32)));
        assert!(((x as i32) == (1_u8 as i32)));
        const _: () = assert!(true, "");;
        y = (unimplemented!());
        assert!(((y as i32) == (0_u8 as i32)));
        assert!(((x as i32) == (0_u8 as i32)));
    }
}
pub trait TestBitwiseAndAssign_char_Impl {
    fn operator_call(&self);
}
impl TestBitwiseAndAssign_char_Impl for Ptr<TestBitwiseAndAssign_char_> {
    fn operator_call(&self) {
        const _: () = assert!(true, "std::is_integral_v<T>");;
        let mut x: i8 = 1_i8;
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut y: i8 = (unimplemented!());
        assert!(((y as i32) == (1_i8 as i32)));
        assert!(((x as i32) == (1_i8 as i32)));
        const _: () = assert!(true, "");;
        y = (unimplemented!());
        assert!(((y as i32) == (0_i8 as i32)));
        assert!(((x as i32) == (0_i8 as i32)));
    }
}
pub trait TestBitwiseAndAssign_int_Impl {
    fn operator_call(&self);
}
impl TestBitwiseAndAssign_int_Impl for Ptr<TestBitwiseAndAssign_int_> {
    fn operator_call(&self) {
        const _: () = assert!(true, "std::is_integral_v<T>");;
        let mut x: i32 = 1;
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut y: i32 = (unimplemented!());
        assert!((y == 1));
        assert!((x == 1));
        const _: () = assert!(true, "");;
        y = (unimplemented!());
        assert!((y == 0));
        assert!((x == 0));
    }
}
pub trait TestBitwiseAndAssign_long_Impl {
    fn operator_call(&self);
}
impl TestBitwiseAndAssign_long_Impl for Ptr<TestBitwiseAndAssign_long_> {
    fn operator_call(&self) {
        const _: () = assert!(true, "std::is_integral_v<T>");;
        let mut x: i64 = 1_i64;
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut y: i64 = (unimplemented!());
        assert!((y == 1_i64));
        assert!((x == 1_i64));
        const _: () = assert!(true, "");;
        y = (unimplemented!());
        assert!((y == 0_i64));
        assert!((x == 0_i64));
    }
}
pub trait TestBitwiseAndAssign_long_long_Impl {
    fn operator_call(&self);
}
impl TestBitwiseAndAssign_long_long_Impl for Ptr<TestBitwiseAndAssign_long_long_> {
    fn operator_call(&self) {
        const _: () = assert!(true, "std::is_integral_v<T>");;
        let mut x: i64 = 1_i64;
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut y: i64 = (unimplemented!());
        assert!((y == 1_i64));
        assert!((x == 1_i64));
        const _: () = assert!(true, "");;
        y = (unimplemented!());
        assert!((y == 0_i64));
        assert!((x == 0_i64));
    }
}
pub trait TestBitwiseAndAssign_short_Impl {
    fn operator_call(&self);
}
impl TestBitwiseAndAssign_short_Impl for Ptr<TestBitwiseAndAssign_short_> {
    fn operator_call(&self) {
        const _: () = assert!(true, "std::is_integral_v<T>");;
        let mut x: i16 = 1_i16;
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut y: i16 = (unimplemented!());
        assert!(((y as i32) == (1_i16 as i32)));
        assert!(((x as i32) == (1_i16 as i32)));
        const _: () = assert!(true, "");;
        y = (unimplemented!());
        assert!(((y as i32) == (0_i16 as i32)));
        assert!(((x as i32) == (0_i16 as i32)));
    }
}
pub trait TestBitwiseAndAssign_signed_char_Impl {
    fn operator_call(&self);
}
impl TestBitwiseAndAssign_signed_char_Impl for Ptr<TestBitwiseAndAssign_signed_char_> {
    fn operator_call(&self) {
        const _: () = assert!(true, "std::is_integral_v<T>");;
        let mut x: i8 = 1_i8;
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut y: i8 = (unimplemented!());
        assert!(((y as i32) == (1_i8 as i32)));
        assert!(((x as i32) == (1_i8 as i32)));
        const _: () = assert!(true, "");;
        y = (unimplemented!());
        assert!(((y as i32) == (0_i8 as i32)));
        assert!(((x as i32) == (0_i8 as i32)));
    }
}
pub trait TestBitwiseAndAssign_unsigned_char_Impl {
    fn operator_call(&self);
}
impl TestBitwiseAndAssign_unsigned_char_Impl for Ptr<TestBitwiseAndAssign_unsigned_char_> {
    fn operator_call(&self) {
        const _: () = assert!(true, "std::is_integral_v<T>");;
        let mut x: u8 = 1_u8;
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut y: u8 = (unimplemented!());
        assert!(((y as i32) == (1_u8 as i32)));
        assert!(((x as i32) == (1_u8 as i32)));
        const _: () = assert!(true, "");;
        y = (unimplemented!());
        assert!(((y as i32) == (0_u8 as i32)));
        assert!(((x as i32) == (0_u8 as i32)));
    }
}
pub trait TestBitwiseAndAssign_unsigned_int_Impl {
    fn operator_call(&self);
}
impl TestBitwiseAndAssign_unsigned_int_Impl for Ptr<TestBitwiseAndAssign_unsigned_int_> {
    fn operator_call(&self) {
        const _: () = assert!(true, "std::is_integral_v<T>");;
        let mut x: u32 = 1_u32;
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut y: u32 = (unimplemented!());
        assert!((y == 1_u32));
        assert!((x == 1_u32));
        const _: () = assert!(true, "");;
        y = (unimplemented!());
        assert!((y == 0_u32));
        assert!((x == 0_u32));
    }
}
pub trait TestBitwiseAndAssign_unsigned_long_Impl {
    fn operator_call(&self);
}
impl TestBitwiseAndAssign_unsigned_long_Impl for Ptr<TestBitwiseAndAssign_unsigned_long_> {
    fn operator_call(&self) {
        const _: () = assert!(true, "std::is_integral_v<T>");;
        let mut x: u64 = 1_u64;
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut y: u64 = (unimplemented!());
        assert!((y == 1_u64));
        assert!((x == 1_u64));
        const _: () = assert!(true, "");;
        y = (unimplemented!());
        assert!((y == 0_u64));
        assert!((x == 0_u64));
    }
}
pub trait TestBitwiseAndAssign_unsigned_long_long_Impl {
    fn operator_call(&self);
}
impl TestBitwiseAndAssign_unsigned_long_long_Impl
    for Ptr<TestBitwiseAndAssign_unsigned_long_long_>
{
    fn operator_call(&self) {
        const _: () = assert!(true, "std::is_integral_v<T>");;
        let mut x: u64 = 1_u64;
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut y: u64 = (unimplemented!());
        assert!((y == 1_u64));
        assert!((x == 1_u64));
        const _: () = assert!(true, "");;
        y = (unimplemented!());
        assert!((y == 0_u64));
        assert!((x == 0_u64));
    }
}
pub trait TestBitwiseAndAssign_unsigned_short_Impl {
    fn operator_call(&self);
}
impl TestBitwiseAndAssign_unsigned_short_Impl for Ptr<TestBitwiseAndAssign_unsigned_short_> {
    fn operator_call(&self) {
        const _: () = assert!(true, "std::is_integral_v<T>");;
        let mut x: u16 = 1_u16;
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut y: u16 = (unimplemented!());
        assert!(((y as i32) == (1_u16 as i32)));
        assert!(((x as i32) == (1_u16 as i32)));
        const _: () = assert!(true, "");;
        y = (unimplemented!());
        assert!(((y as i32) == (0_u16 as i32)));
        assert!(((x as i32) == (0_u16 as i32)));
    }
}
pub trait TestBitwiseAndAssign_wchar_t_Impl {
    fn operator_call(&self);
}
impl TestBitwiseAndAssign_wchar_t_Impl for Ptr<TestBitwiseAndAssign_wchar_t_> {
    fn operator_call(&self) {
        const _: () = assert!(true, "std::is_integral_v<T>");;
        let mut x: i32 = 1_i32;
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut y: i32 = (unimplemented!());
        assert!((y == 1_i32));
        assert!((x == 1_i32));
        const _: () = assert!(true, "");;
        y = (unimplemented!());
        assert!((y == 0_i32));
        assert!((x == 0_i32));
    }
}
pub trait TestDoesNotHaveBitwiseAndAssign_LargeUserAtomicType_Impl {
    fn operator_call(&self);
}
impl TestDoesNotHaveBitwiseAndAssign_LargeUserAtomicType_Impl
    for Ptr<TestDoesNotHaveBitwiseAndAssign_LargeUserAtomicType_>
{
    fn operator_call(&self) {
        const _: () = assert!(!(false), "!has_bitwise_and_assign<std::atomic_ref<T>>");;
    }
}
pub trait TestDoesNotHaveBitwiseAndAssign_UserAtomicType_Impl {
    fn operator_call(&self);
}
impl TestDoesNotHaveBitwiseAndAssign_UserAtomicType_Impl
    for Ptr<TestDoesNotHaveBitwiseAndAssign_UserAtomicType_>
{
    fn operator_call(&self) {
        const _: () = assert!(!(false), "!has_bitwise_and_assign<std::atomic_ref<T>>");;
    }
}
pub trait TestDoesNotHaveBitwiseAndAssign_bool_Impl {
    fn operator_call(&self);
}
impl TestDoesNotHaveBitwiseAndAssign_bool_Impl for Ptr<TestDoesNotHaveBitwiseAndAssign_bool_> {
    fn operator_call(&self) {
        const _: () = assert!(!(false), "!has_bitwise_and_assign<std::atomic_ref<T>>");;
    }
}
pub trait TestDoesNotHaveBitwiseAndAssign_const_int_ptr_Impl {
    fn operator_call(&self);
}
impl TestDoesNotHaveBitwiseAndAssign_const_int_ptr_Impl
    for Ptr<TestDoesNotHaveBitwiseAndAssign_const_int_ptr_>
{
    fn operator_call(&self) {
        const _: () = assert!(!(false), "!has_bitwise_and_assign<std::atomic_ref<T>>");;
    }
}
pub trait TestDoesNotHaveBitwiseAndAssign_double_Impl {
    fn operator_call(&self);
}
impl TestDoesNotHaveBitwiseAndAssign_double_Impl for Ptr<TestDoesNotHaveBitwiseAndAssign_double_> {
    fn operator_call(&self) {
        const _: () = assert!(!(false), "!has_bitwise_and_assign<std::atomic_ref<T>>");;
    }
}
pub trait TestDoesNotHaveBitwiseAndAssign_float_Impl {
    fn operator_call(&self);
}
impl TestDoesNotHaveBitwiseAndAssign_float_Impl for Ptr<TestDoesNotHaveBitwiseAndAssign_float_> {
    fn operator_call(&self) {
        const _: () = assert!(!(false), "!has_bitwise_and_assign<std::atomic_ref<T>>");;
    }
}
pub trait TestDoesNotHaveBitwiseAndAssign_int_ptr_Impl {
    fn operator_call(&self);
}
impl TestDoesNotHaveBitwiseAndAssign_int_ptr_Impl
    for Ptr<TestDoesNotHaveBitwiseAndAssign_int_ptr_>
{
    fn operator_call(&self) {
        const _: () = assert!(!(false), "!has_bitwise_and_assign<std::atomic_ref<T>>");;
    }
}
pub trait TestDoesNotHaveBitwiseAndAssign_long_double_Impl {
    fn operator_call(&self);
}
impl TestDoesNotHaveBitwiseAndAssign_long_double_Impl
    for Ptr<TestDoesNotHaveBitwiseAndAssign_long_double_>
{
    fn operator_call(&self) {
        const _: () = assert!(!(false), "!has_bitwise_and_assign<std::atomic_ref<T>>");;
    }
}
pub trait TestEachFloatingPointType_TestDoesNotHaveBitwiseAndAssign_Impl {
    fn operator_call(&self);
}
impl TestEachFloatingPointType_TestDoesNotHaveBitwiseAndAssign_Impl
    for Ptr<TestEachFloatingPointType_TestDoesNotHaveBitwiseAndAssign_>
{
    fn operator_call(&self) {
        ({
            TestDoesNotHaveBitwiseAndAssign_float_Impl::operator_call(
                &Rc::new(RefCell::new(
                    <TestDoesNotHaveBitwiseAndAssign_float_>::default(),
                ))
                .as_pointer(),
            )
        });
        ({
            TestDoesNotHaveBitwiseAndAssign_double_Impl::operator_call(
                &Rc::new(RefCell::new(
                    <TestDoesNotHaveBitwiseAndAssign_double_>::default(),
                ))
                .as_pointer(),
            )
        });
        ({
            TestDoesNotHaveBitwiseAndAssign_long_double_Impl::operator_call(
                &Rc::new(RefCell::new(
                    <TestDoesNotHaveBitwiseAndAssign_long_double_>::default(),
                ))
                .as_pointer(),
            )
        });
    }
}
pub trait TestEachIntegralType_TestBitwiseAndAssign_Impl {
    fn operator_call(&self);
}
impl TestEachIntegralType_TestBitwiseAndAssign_Impl
    for Ptr<TestEachIntegralType_TestBitwiseAndAssign_>
{
    fn operator_call(&self) {
        ({
            TestBitwiseAndAssign_char_Impl::operator_call(
                &Rc::new(RefCell::new(<TestBitwiseAndAssign_char_>::default())).as_pointer(),
            )
        });
        ({
            TestBitwiseAndAssign_signed_char_Impl::operator_call(
                &Rc::new(RefCell::new(<TestBitwiseAndAssign_signed_char_>::default())).as_pointer(),
            )
        });
        ({
            TestBitwiseAndAssign_unsigned_char_Impl::operator_call(
                &Rc::new(RefCell::new(
                    <TestBitwiseAndAssign_unsigned_char_>::default(),
                ))
                .as_pointer(),
            )
        });
        ({
            TestBitwiseAndAssign_short_Impl::operator_call(
                &Rc::new(RefCell::new(<TestBitwiseAndAssign_short_>::default())).as_pointer(),
            )
        });
        ({
            TestBitwiseAndAssign_unsigned_short_Impl::operator_call(
                &Rc::new(RefCell::new(
                    <TestBitwiseAndAssign_unsigned_short_>::default(),
                ))
                .as_pointer(),
            )
        });
        ({
            TestBitwiseAndAssign_int_Impl::operator_call(
                &Rc::new(RefCell::new(<TestBitwiseAndAssign_int_>::default())).as_pointer(),
            )
        });
        ({
            TestBitwiseAndAssign_unsigned_int_Impl::operator_call(
                &Rc::new(RefCell::new(<TestBitwiseAndAssign_unsigned_int_>::default()))
                    .as_pointer(),
            )
        });
        ({
            TestBitwiseAndAssign_long_Impl::operator_call(
                &Rc::new(RefCell::new(<TestBitwiseAndAssign_long_>::default())).as_pointer(),
            )
        });
        ({
            TestBitwiseAndAssign_unsigned_long_Impl::operator_call(
                &Rc::new(RefCell::new(
                    <TestBitwiseAndAssign_unsigned_long_>::default(),
                ))
                .as_pointer(),
            )
        });
        ({
            TestBitwiseAndAssign_long_long_Impl::operator_call(
                &Rc::new(RefCell::new(<TestBitwiseAndAssign_long_long_>::default())).as_pointer(),
            )
        });
        ({
            TestBitwiseAndAssign_unsigned_long_long_Impl::operator_call(
                &Rc::new(RefCell::new(
                    <TestBitwiseAndAssign_unsigned_long_long_>::default(),
                ))
                .as_pointer(),
            )
        });
        ({
            TestBitwiseAndAssign_wchar_t_Impl::operator_call(
                &Rc::new(RefCell::new(<TestBitwiseAndAssign_wchar_t_>::default())).as_pointer(),
            )
        });
        ({
            TestBitwiseAndAssign_char8_t_Impl::operator_call(
                &Rc::new(RefCell::new(<TestBitwiseAndAssign_char8_t_>::default())).as_pointer(),
            )
        });
        ({
            TestBitwiseAndAssign_char16_t_Impl::operator_call(
                &Rc::new(RefCell::new(<TestBitwiseAndAssign_char16_t_>::default())).as_pointer(),
            )
        });
        ({
            TestBitwiseAndAssign_char32_t_Impl::operator_call(
                &Rc::new(RefCell::new(<TestBitwiseAndAssign_char32_t_>::default())).as_pointer(),
            )
        });
        ({
            TestBitwiseAndAssign_signed_char_Impl::operator_call(
                &Rc::new(RefCell::new(<TestBitwiseAndAssign_signed_char_>::default())).as_pointer(),
            )
        });
        ({
            TestBitwiseAndAssign_unsigned_char_Impl::operator_call(
                &Rc::new(RefCell::new(
                    <TestBitwiseAndAssign_unsigned_char_>::default(),
                ))
                .as_pointer(),
            )
        });
        ({
            TestBitwiseAndAssign_short_Impl::operator_call(
                &Rc::new(RefCell::new(<TestBitwiseAndAssign_short_>::default())).as_pointer(),
            )
        });
        ({
            TestBitwiseAndAssign_unsigned_short_Impl::operator_call(
                &Rc::new(RefCell::new(
                    <TestBitwiseAndAssign_unsigned_short_>::default(),
                ))
                .as_pointer(),
            )
        });
        ({
            TestBitwiseAndAssign_int_Impl::operator_call(
                &Rc::new(RefCell::new(<TestBitwiseAndAssign_int_>::default())).as_pointer(),
            )
        });
        ({
            TestBitwiseAndAssign_unsigned_int_Impl::operator_call(
                &Rc::new(RefCell::new(<TestBitwiseAndAssign_unsigned_int_>::default()))
                    .as_pointer(),
            )
        });
        ({
            TestBitwiseAndAssign_long_Impl::operator_call(
                &Rc::new(RefCell::new(<TestBitwiseAndAssign_long_>::default())).as_pointer(),
            )
        });
        ({
            TestBitwiseAndAssign_unsigned_long_Impl::operator_call(
                &Rc::new(RefCell::new(
                    <TestBitwiseAndAssign_unsigned_long_>::default(),
                ))
                .as_pointer(),
            )
        });
    }
}
pub trait TestEachPointerType_TestDoesNotHaveBitwiseAndAssign_Impl {
    fn operator_call(&self);
}
impl TestEachPointerType_TestDoesNotHaveBitwiseAndAssign_Impl
    for Ptr<TestEachPointerType_TestDoesNotHaveBitwiseAndAssign_>
{
    fn operator_call(&self) {
        ({
            TestDoesNotHaveBitwiseAndAssign_int_ptr_Impl::operator_call(
                &Rc::new(RefCell::new(
                    <TestDoesNotHaveBitwiseAndAssign_int_ptr_>::default(),
                ))
                .as_pointer(),
            )
        });
        ({
            TestDoesNotHaveBitwiseAndAssign_const_int_ptr_Impl::operator_call(
                &Rc::new(RefCell::new(
                    <TestDoesNotHaveBitwiseAndAssign_const_int_ptr_>::default(),
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
