extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn operator_eq_0(lhs: Ptr<S_1_>, rhs: Ptr<S_1_>) -> bool {
    return ((((array_field_ptr!(lhs, data) as Ptr<i8>).offset((0) as isize)) as Ptr<i8>)
        .to_any()
        .memcmp(
            &(((array_field_ptr!(rhs, data) as Ptr<i8>).offset((0) as isize)) as Ptr<i8>).to_any(),
            ::std::mem::size_of::<[i8; 1]>(),
        )
        == 0);
}
pub fn operator_ne_1(lhs: Ptr<S_1_>, rhs: Ptr<S_1_>) -> bool {
    return !({
        let _lhs: Ptr<S_1_> = (lhs).clone();
        let _rhs: Ptr<S_1_> = (rhs).clone();
        operator_eq_0(_lhs, _rhs)
    });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct S_1_ {
    #[offset(0)]
    #[byte_size(1)]
    pub data: Box<[i8]>,
}
impl S_1_ {
    pub fn new(v: Option<i8>) -> Self {
        let mut v: i8 = v.unwrap_or_else(|| 0_i8);
        let __this: Value<S_1_> = Rc::new(RefCell::new(Self {
            data: (0..1).map(|_| 0_i8).collect::<Box<[i8]>>(),
        }));
        let this: Ptr<S_1_> = __this.as_pointer();
        match ({
            (((array_field_ptr!(this, data) as Ptr<i8>).offset((0) as isize)) as Ptr<i8>).to_any()
        },)
        {
            (mut __tmp_0,) => {
                __tmp_0.memset((v as i32) as u8, ::std::mem::size_of::<[i8; 1]>() as usize);
                (__tmp_0).clone()
            }
        };
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl CppEq for S_1_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_0(__lhs.clone(), __rhs.clone())
    }
}
impl CppMove for S_1_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for S_1_ {
    fn default() -> Self {
        { S_1_::new(None) }
    }
}
pub fn operator_eq_2(lhs: Ptr<S_2_>, rhs: Ptr<S_2_>) -> bool {
    return ((((array_field_ptr!(lhs, data) as Ptr<i8>).offset((0) as isize)) as Ptr<i8>)
        .to_any()
        .memcmp(
            &(((array_field_ptr!(rhs, data) as Ptr<i8>).offset((0) as isize)) as Ptr<i8>).to_any(),
            ::std::mem::size_of::<[i8; 2]>(),
        )
        == 0);
}
pub fn operator_ne_3(lhs: Ptr<S_2_>, rhs: Ptr<S_2_>) -> bool {
    return !({
        let _lhs: Ptr<S_2_> = (lhs).clone();
        let _rhs: Ptr<S_2_> = (rhs).clone();
        operator_eq_2(_lhs, _rhs)
    });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(2)]
pub struct S_2_ {
    #[offset(0)]
    #[byte_size(2)]
    pub data: Box<[i8]>,
}
impl S_2_ {
    pub fn new(v: Option<i8>) -> Self {
        let mut v: i8 = v.unwrap_or_else(|| 0_i8);
        let __this: Value<S_2_> = Rc::new(RefCell::new(Self {
            data: (0..2).map(|_| 0_i8).collect::<Box<[i8]>>(),
        }));
        let this: Ptr<S_2_> = __this.as_pointer();
        match ({
            (((array_field_ptr!(this, data) as Ptr<i8>).offset((0) as isize)) as Ptr<i8>).to_any()
        },)
        {
            (mut __tmp_1,) => {
                __tmp_1.memset((v as i32) as u8, ::std::mem::size_of::<[i8; 2]>() as usize);
                (__tmp_1).clone()
            }
        };
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl CppEq for S_2_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_2(__lhs.clone(), __rhs.clone())
    }
}
impl CppMove for S_2_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for S_2_ {
    fn default() -> Self {
        { S_2_::new(None) }
    }
}
pub fn operator_eq_4(lhs: Ptr<S_3_>, rhs: Ptr<S_3_>) -> bool {
    return ((((array_field_ptr!(lhs, data) as Ptr<i8>).offset((0) as isize)) as Ptr<i8>)
        .to_any()
        .memcmp(
            &(((array_field_ptr!(rhs, data) as Ptr<i8>).offset((0) as isize)) as Ptr<i8>).to_any(),
            ::std::mem::size_of::<[i8; 3]>(),
        )
        == 0);
}
pub fn operator_ne_5(lhs: Ptr<S_3_>, rhs: Ptr<S_3_>) -> bool {
    return !({
        let _lhs: Ptr<S_3_> = (lhs).clone();
        let _rhs: Ptr<S_3_> = (rhs).clone();
        operator_eq_4(_lhs, _rhs)
    });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(3)]
pub struct S_3_ {
    #[offset(0)]
    #[byte_size(3)]
    pub data: Box<[i8]>,
}
impl S_3_ {
    pub fn new(v: Option<i8>) -> Self {
        let mut v: i8 = v.unwrap_or_else(|| 0_i8);
        let __this: Value<S_3_> = Rc::new(RefCell::new(Self {
            data: (0..3).map(|_| 0_i8).collect::<Box<[i8]>>(),
        }));
        let this: Ptr<S_3_> = __this.as_pointer();
        match ({
            (((array_field_ptr!(this, data) as Ptr<i8>).offset((0) as isize)) as Ptr<i8>).to_any()
        },)
        {
            (mut __tmp_2,) => {
                __tmp_2.memset((v as i32) as u8, ::std::mem::size_of::<[i8; 3]>() as usize);
                (__tmp_2).clone()
            }
        };
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl CppEq for S_3_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_4(__lhs.clone(), __rhs.clone())
    }
}
impl CppMove for S_3_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for S_3_ {
    fn default() -> Self {
        { S_3_::new(None) }
    }
}
pub fn operator_eq_6(lhs: Ptr<S_4_>, rhs: Ptr<S_4_>) -> bool {
    return ((((array_field_ptr!(lhs, data) as Ptr<i8>).offset((0) as isize)) as Ptr<i8>)
        .to_any()
        .memcmp(
            &(((array_field_ptr!(rhs, data) as Ptr<i8>).offset((0) as isize)) as Ptr<i8>).to_any(),
            ::std::mem::size_of::<[i8; 4]>(),
        )
        == 0);
}
pub fn operator_ne_7(lhs: Ptr<S_4_>, rhs: Ptr<S_4_>) -> bool {
    return !({
        let _lhs: Ptr<S_4_> = (lhs).clone();
        let _rhs: Ptr<S_4_> = (rhs).clone();
        operator_eq_6(_lhs, _rhs)
    });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct S_4_ {
    #[offset(0)]
    #[byte_size(4)]
    pub data: Box<[i8]>,
}
impl S_4_ {
    pub fn new(v: Option<i8>) -> Self {
        let mut v: i8 = v.unwrap_or_else(|| 0_i8);
        let __this: Value<S_4_> = Rc::new(RefCell::new(Self {
            data: (0..4).map(|_| 0_i8).collect::<Box<[i8]>>(),
        }));
        let this: Ptr<S_4_> = __this.as_pointer();
        match ({
            (((array_field_ptr!(this, data) as Ptr<i8>).offset((0) as isize)) as Ptr<i8>).to_any()
        },)
        {
            (mut __tmp_3,) => {
                __tmp_3.memset((v as i32) as u8, ::std::mem::size_of::<[i8; 4]>() as usize);
                (__tmp_3).clone()
            }
        };
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl CppEq for S_4_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_6(__lhs.clone(), __rhs.clone())
    }
}
impl CppMove for S_4_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for S_4_ {
    fn default() -> Self {
        { S_4_::new(None) }
    }
}
pub fn operator_eq_8(lhs: Ptr<S_5_>, rhs: Ptr<S_5_>) -> bool {
    return ((((array_field_ptr!(lhs, data) as Ptr<i8>).offset((0) as isize)) as Ptr<i8>)
        .to_any()
        .memcmp(
            &(((array_field_ptr!(rhs, data) as Ptr<i8>).offset((0) as isize)) as Ptr<i8>).to_any(),
            ::std::mem::size_of::<[i8; 5]>(),
        )
        == 0);
}
pub fn operator_ne_9(lhs: Ptr<S_5_>, rhs: Ptr<S_5_>) -> bool {
    return !({
        let _lhs: Ptr<S_5_> = (lhs).clone();
        let _rhs: Ptr<S_5_> = (rhs).clone();
        operator_eq_8(_lhs, _rhs)
    });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(5)]
pub struct S_5_ {
    #[offset(0)]
    #[byte_size(5)]
    pub data: Box<[i8]>,
}
impl S_5_ {
    pub fn new(v: Option<i8>) -> Self {
        let mut v: i8 = v.unwrap_or_else(|| 0_i8);
        let __this: Value<S_5_> = Rc::new(RefCell::new(Self {
            data: (0..5).map(|_| 0_i8).collect::<Box<[i8]>>(),
        }));
        let this: Ptr<S_5_> = __this.as_pointer();
        match ({
            (((array_field_ptr!(this, data) as Ptr<i8>).offset((0) as isize)) as Ptr<i8>).to_any()
        },)
        {
            (mut __tmp_4,) => {
                __tmp_4.memset((v as i32) as u8, ::std::mem::size_of::<[i8; 5]>() as usize);
                (__tmp_4).clone()
            }
        };
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl CppEq for S_5_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_8(__lhs.clone(), __rhs.clone())
    }
}
impl CppMove for S_5_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for S_5_ {
    fn default() -> Self {
        { S_5_::new(None) }
    }
}
pub fn operator_eq_10(lhs: Ptr<S_6_>, rhs: Ptr<S_6_>) -> bool {
    return ((((array_field_ptr!(lhs, data) as Ptr<i8>).offset((0) as isize)) as Ptr<i8>)
        .to_any()
        .memcmp(
            &(((array_field_ptr!(rhs, data) as Ptr<i8>).offset((0) as isize)) as Ptr<i8>).to_any(),
            ::std::mem::size_of::<[i8; 6]>(),
        )
        == 0);
}
pub fn operator_ne_11(lhs: Ptr<S_6_>, rhs: Ptr<S_6_>) -> bool {
    return !({
        let _lhs: Ptr<S_6_> = (lhs).clone();
        let _rhs: Ptr<S_6_> = (rhs).clone();
        operator_eq_10(_lhs, _rhs)
    });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(6)]
pub struct S_6_ {
    #[offset(0)]
    #[byte_size(6)]
    pub data: Box<[i8]>,
}
impl S_6_ {
    pub fn new(v: Option<i8>) -> Self {
        let mut v: i8 = v.unwrap_or_else(|| 0_i8);
        let __this: Value<S_6_> = Rc::new(RefCell::new(Self {
            data: (0..6).map(|_| 0_i8).collect::<Box<[i8]>>(),
        }));
        let this: Ptr<S_6_> = __this.as_pointer();
        match ({
            (((array_field_ptr!(this, data) as Ptr<i8>).offset((0) as isize)) as Ptr<i8>).to_any()
        },)
        {
            (mut __tmp_5,) => {
                __tmp_5.memset((v as i32) as u8, ::std::mem::size_of::<[i8; 6]>() as usize);
                (__tmp_5).clone()
            }
        };
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl CppEq for S_6_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_10(__lhs.clone(), __rhs.clone())
    }
}
impl CppMove for S_6_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for S_6_ {
    fn default() -> Self {
        { S_6_::new(None) }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom)]
#[cpp_from(S_1_, new, value)]
#[byte_size(2)]
pub struct Expected_1_ {
    #[offset(0)]
    #[byte_size(1)]
    pub s: S_1_,
    #[offset(1)]
    pub b: bool,
}
impl Expected_1_ {
    pub fn new(mut ss: S_1_) -> Self {
        Self {
            s: (ss).clone(),
            b: true,
        }
    }
}
impl CppMove for Expected_1_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for Expected_1_ {
    fn default() -> Self {
        {
            Expected_1_ {
                s: <S_1_>::default(),
                b: true,
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom)]
#[cpp_from(S_2_, new, value)]
#[byte_size(3)]
pub struct Expected_2_ {
    #[offset(0)]
    #[byte_size(2)]
    pub s: S_2_,
    #[offset(2)]
    pub b: bool,
}
impl Expected_2_ {
    pub fn new(mut ss: S_2_) -> Self {
        Self {
            s: (ss).clone(),
            b: true,
        }
    }
}
impl CppMove for Expected_2_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for Expected_2_ {
    fn default() -> Self {
        {
            Expected_2_ {
                s: <S_2_>::default(),
                b: true,
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom)]
#[cpp_from(S_3_, new, value)]
#[byte_size(4)]
pub struct Expected_3_ {
    #[offset(0)]
    #[byte_size(3)]
    pub s: S_3_,
    #[offset(3)]
    pub b: bool,
}
impl Expected_3_ {
    pub fn new(mut ss: S_3_) -> Self {
        Self {
            s: (ss).clone(),
            b: true,
        }
    }
}
impl CppMove for Expected_3_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for Expected_3_ {
    fn default() -> Self {
        {
            Expected_3_ {
                s: <S_3_>::default(),
                b: true,
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom)]
#[cpp_from(S_4_, new, value)]
#[byte_size(5)]
pub struct Expected_4_ {
    #[offset(0)]
    #[byte_size(4)]
    pub s: S_4_,
    #[offset(4)]
    pub b: bool,
}
impl Expected_4_ {
    pub fn new(mut ss: S_4_) -> Self {
        Self {
            s: (ss).clone(),
            b: true,
        }
    }
}
impl CppMove for Expected_4_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for Expected_4_ {
    fn default() -> Self {
        {
            Expected_4_ {
                s: <S_4_>::default(),
                b: true,
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom)]
#[cpp_from(S_5_, new, value)]
#[byte_size(6)]
pub struct Expected_5_ {
    #[offset(0)]
    #[byte_size(5)]
    pub s: S_5_,
    #[offset(5)]
    pub b: bool,
}
impl Expected_5_ {
    pub fn new(mut ss: S_5_) -> Self {
        Self {
            s: (ss).clone(),
            b: true,
        }
    }
}
impl CppMove for Expected_5_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for Expected_5_ {
    fn default() -> Self {
        {
            Expected_5_ {
                s: <S_5_>::default(),
                b: true,
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom)]
#[cpp_from(S_6_, new, value)]
#[byte_size(7)]
pub struct Expected_6_ {
    #[offset(0)]
    #[byte_size(6)]
    pub s: S_6_,
    #[offset(6)]
    pub b: bool,
}
impl Expected_6_ {
    pub fn new(mut ss: S_6_) -> Self {
        Self {
            s: (ss).clone(),
            b: true,
        }
    }
}
impl CppMove for Expected_6_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for Expected_6_ {
    fn default() -> Self {
        {
            Expected_6_ {
                s: <S_6_>::default(),
                b: true,
            }
        }
    }
}
pub fn test_12() {
    let a: Value<std_atomic_S_1__> = Rc::new(RefCell::new(std_atomic_S_1__::new_1({
        S_1_::new({ Some(0_i8) })
    })));
    let expected: Value<Expected_1_> = Rc::new(RefCell::new(Expected_1_::new({
        S_1_::new({ Some(17_i8) })
    })));
    assert!(
        ({
            let _lhs: Value<S_1_> = Rc::new(RefCell::new(({ (*a.borrow()).load_15(None) })));
            operator_ne_1(_lhs.as_pointer(), field_ptr!(expected.as_pointer(), s))
        })
    );
    assert!(({ { (*expected.borrow()).b } }));
    let mut r1: bool = ({
        (*a.borrow()).compare_exchange_strong_25(
            field_ptr!(expected.as_pointer(), s),
            S_1_::new({ Some(18_i8) }),
            memory_order_relaxed_13.with(|rc| *rc.borrow()),
        )
    });
    assert!(!(r1));
    assert!(
        ({
            let _lhs: Ptr<S_1_> = field_ptr!(expected.as_pointer(), s);
            let _rhs: Value<S_1_> = Rc::new(RefCell::new(S_1_::new({ Some(0_i8) })));
            operator_eq_0(_lhs, _rhs.as_pointer())
        })
    );
    assert!(({ { (*expected.borrow()).b } }));
    assert!(
        ({
            let _lhs: Value<S_1_> = Rc::new(RefCell::new(({ (*a.borrow()).load_15(None) })));
            let _rhs: Value<S_1_> = Rc::new(RefCell::new(S_1_::new({ Some(0_i8) })));
            operator_eq_0(_lhs.as_pointer(), _rhs.as_pointer())
        })
    );
    let mut r2: bool = ({
        (*a.borrow()).compare_exchange_strong_25(
            field_ptr!(expected.as_pointer(), s),
            S_1_::new({ Some(18_i8) }),
            memory_order_relaxed_13.with(|rc| *rc.borrow()),
        )
    });
    assert!((r2 as bool));
    assert!(
        ({
            let _lhs: Value<S_1_> = Rc::new(RefCell::new(({ (*a.borrow()).load_15(None) })));
            let _rhs: Value<S_1_> = Rc::new(RefCell::new(S_1_::new({ Some(18_i8) })));
            operator_eq_0(_lhs.as_pointer(), _rhs.as_pointer())
        })
    );
    assert!(
        ({
            let _lhs: Ptr<S_1_> = field_ptr!(expected.as_pointer(), s);
            let _rhs: Value<S_1_> = Rc::new(RefCell::new(S_1_::new({ Some(0_i8) })));
            operator_eq_0(_lhs, _rhs.as_pointer())
        })
    );
    assert!(({ { (*expected.borrow()).b } }));
}
pub fn test_14() {
    let a: Value<std_atomic_S_2__> = Rc::new(RefCell::new(std_atomic_S_2__::new_1({
        S_2_::new({ Some(0_i8) })
    })));
    let expected: Value<Expected_2_> = Rc::new(RefCell::new(Expected_2_::new({
        S_2_::new({ Some(17_i8) })
    })));
    assert!(
        ({
            let _lhs: Value<S_2_> = Rc::new(RefCell::new(({ (*a.borrow()).load_15(None) })));
            operator_ne_3(_lhs.as_pointer(), field_ptr!(expected.as_pointer(), s))
        })
    );
    assert!(({ { (*expected.borrow()).b } }));
    let mut r1: bool = ({
        (*a.borrow()).compare_exchange_strong_25(
            field_ptr!(expected.as_pointer(), s),
            S_2_::new({ Some(18_i8) }),
            memory_order_relaxed_13.with(|rc| *rc.borrow()),
        )
    });
    assert!(!(r1));
    assert!(
        ({
            let _lhs: Ptr<S_2_> = field_ptr!(expected.as_pointer(), s);
            let _rhs: Value<S_2_> = Rc::new(RefCell::new(S_2_::new({ Some(0_i8) })));
            operator_eq_2(_lhs, _rhs.as_pointer())
        })
    );
    assert!(({ { (*expected.borrow()).b } }));
    assert!(
        ({
            let _lhs: Value<S_2_> = Rc::new(RefCell::new(({ (*a.borrow()).load_15(None) })));
            let _rhs: Value<S_2_> = Rc::new(RefCell::new(S_2_::new({ Some(0_i8) })));
            operator_eq_2(_lhs.as_pointer(), _rhs.as_pointer())
        })
    );
    let mut r2: bool = ({
        (*a.borrow()).compare_exchange_strong_25(
            field_ptr!(expected.as_pointer(), s),
            S_2_::new({ Some(18_i8) }),
            memory_order_relaxed_13.with(|rc| *rc.borrow()),
        )
    });
    assert!((r2 as bool));
    assert!(
        ({
            let _lhs: Value<S_2_> = Rc::new(RefCell::new(({ (*a.borrow()).load_15(None) })));
            let _rhs: Value<S_2_> = Rc::new(RefCell::new(S_2_::new({ Some(18_i8) })));
            operator_eq_2(_lhs.as_pointer(), _rhs.as_pointer())
        })
    );
    assert!(
        ({
            let _lhs: Ptr<S_2_> = field_ptr!(expected.as_pointer(), s);
            let _rhs: Value<S_2_> = Rc::new(RefCell::new(S_2_::new({ Some(0_i8) })));
            operator_eq_2(_lhs, _rhs.as_pointer())
        })
    );
    assert!(({ { (*expected.borrow()).b } }));
}
pub fn test_15() {
    let a: Value<std_atomic_S_3__> = Rc::new(RefCell::new(std_atomic_S_3__::new_1({
        S_3_::new({ Some(0_i8) })
    })));
    let expected: Value<Expected_3_> = Rc::new(RefCell::new(Expected_3_::new({
        S_3_::new({ Some(17_i8) })
    })));
    assert!(
        ({
            let _lhs: Value<S_3_> = Rc::new(RefCell::new(({ (*a.borrow()).load_15(None) })));
            operator_ne_5(_lhs.as_pointer(), field_ptr!(expected.as_pointer(), s))
        })
    );
    assert!(({ { (*expected.borrow()).b } }));
    let mut r1: bool = ({
        (*a.borrow()).compare_exchange_strong_25(
            field_ptr!(expected.as_pointer(), s),
            S_3_::new({ Some(18_i8) }),
            memory_order_relaxed_13.with(|rc| *rc.borrow()),
        )
    });
    assert!(!(r1));
    assert!(
        ({
            let _lhs: Ptr<S_3_> = field_ptr!(expected.as_pointer(), s);
            let _rhs: Value<S_3_> = Rc::new(RefCell::new(S_3_::new({ Some(0_i8) })));
            operator_eq_4(_lhs, _rhs.as_pointer())
        })
    );
    assert!(({ { (*expected.borrow()).b } }));
    assert!(
        ({
            let _lhs: Value<S_3_> = Rc::new(RefCell::new(({ (*a.borrow()).load_15(None) })));
            let _rhs: Value<S_3_> = Rc::new(RefCell::new(S_3_::new({ Some(0_i8) })));
            operator_eq_4(_lhs.as_pointer(), _rhs.as_pointer())
        })
    );
    let mut r2: bool = ({
        (*a.borrow()).compare_exchange_strong_25(
            field_ptr!(expected.as_pointer(), s),
            S_3_::new({ Some(18_i8) }),
            memory_order_relaxed_13.with(|rc| *rc.borrow()),
        )
    });
    assert!((r2 as bool));
    assert!(
        ({
            let _lhs: Value<S_3_> = Rc::new(RefCell::new(({ (*a.borrow()).load_15(None) })));
            let _rhs: Value<S_3_> = Rc::new(RefCell::new(S_3_::new({ Some(18_i8) })));
            operator_eq_4(_lhs.as_pointer(), _rhs.as_pointer())
        })
    );
    assert!(
        ({
            let _lhs: Ptr<S_3_> = field_ptr!(expected.as_pointer(), s);
            let _rhs: Value<S_3_> = Rc::new(RefCell::new(S_3_::new({ Some(0_i8) })));
            operator_eq_4(_lhs, _rhs.as_pointer())
        })
    );
    assert!(({ { (*expected.borrow()).b } }));
}
pub fn test_16() {
    let a: Value<std_atomic_S_4__> = Rc::new(RefCell::new(std_atomic_S_4__::new_1({
        S_4_::new({ Some(0_i8) })
    })));
    let expected: Value<Expected_4_> = Rc::new(RefCell::new(Expected_4_::new({
        S_4_::new({ Some(17_i8) })
    })));
    assert!(
        ({
            let _lhs: Value<S_4_> = Rc::new(RefCell::new(({ (*a.borrow()).load_15(None) })));
            operator_ne_7(_lhs.as_pointer(), field_ptr!(expected.as_pointer(), s))
        })
    );
    assert!(({ { (*expected.borrow()).b } }));
    let mut r1: bool = ({
        (*a.borrow()).compare_exchange_strong_25(
            field_ptr!(expected.as_pointer(), s),
            S_4_::new({ Some(18_i8) }),
            memory_order_relaxed_13.with(|rc| *rc.borrow()),
        )
    });
    assert!(!(r1));
    assert!(
        ({
            let _lhs: Ptr<S_4_> = field_ptr!(expected.as_pointer(), s);
            let _rhs: Value<S_4_> = Rc::new(RefCell::new(S_4_::new({ Some(0_i8) })));
            operator_eq_6(_lhs, _rhs.as_pointer())
        })
    );
    assert!(({ { (*expected.borrow()).b } }));
    assert!(
        ({
            let _lhs: Value<S_4_> = Rc::new(RefCell::new(({ (*a.borrow()).load_15(None) })));
            let _rhs: Value<S_4_> = Rc::new(RefCell::new(S_4_::new({ Some(0_i8) })));
            operator_eq_6(_lhs.as_pointer(), _rhs.as_pointer())
        })
    );
    let mut r2: bool = ({
        (*a.borrow()).compare_exchange_strong_25(
            field_ptr!(expected.as_pointer(), s),
            S_4_::new({ Some(18_i8) }),
            memory_order_relaxed_13.with(|rc| *rc.borrow()),
        )
    });
    assert!((r2 as bool));
    assert!(
        ({
            let _lhs: Value<S_4_> = Rc::new(RefCell::new(({ (*a.borrow()).load_15(None) })));
            let _rhs: Value<S_4_> = Rc::new(RefCell::new(S_4_::new({ Some(18_i8) })));
            operator_eq_6(_lhs.as_pointer(), _rhs.as_pointer())
        })
    );
    assert!(
        ({
            let _lhs: Ptr<S_4_> = field_ptr!(expected.as_pointer(), s);
            let _rhs: Value<S_4_> = Rc::new(RefCell::new(S_4_::new({ Some(0_i8) })));
            operator_eq_6(_lhs, _rhs.as_pointer())
        })
    );
    assert!(({ { (*expected.borrow()).b } }));
}
pub fn test_17() {
    let a: Value<std_atomic_S_5__> = Rc::new(RefCell::new(std_atomic_S_5__::new_1({
        S_5_::new({ Some(0_i8) })
    })));
    let expected: Value<Expected_5_> = Rc::new(RefCell::new(Expected_5_::new({
        S_5_::new({ Some(17_i8) })
    })));
    assert!(
        ({
            let _lhs: Value<S_5_> = Rc::new(RefCell::new(({ (*a.borrow()).load_15(None) })));
            operator_ne_9(_lhs.as_pointer(), field_ptr!(expected.as_pointer(), s))
        })
    );
    assert!(({ { (*expected.borrow()).b } }));
    let mut r1: bool = ({
        (*a.borrow()).compare_exchange_strong_25(
            field_ptr!(expected.as_pointer(), s),
            S_5_::new({ Some(18_i8) }),
            memory_order_relaxed_13.with(|rc| *rc.borrow()),
        )
    });
    assert!(!(r1));
    assert!(
        ({
            let _lhs: Ptr<S_5_> = field_ptr!(expected.as_pointer(), s);
            let _rhs: Value<S_5_> = Rc::new(RefCell::new(S_5_::new({ Some(0_i8) })));
            operator_eq_8(_lhs, _rhs.as_pointer())
        })
    );
    assert!(({ { (*expected.borrow()).b } }));
    assert!(
        ({
            let _lhs: Value<S_5_> = Rc::new(RefCell::new(({ (*a.borrow()).load_15(None) })));
            let _rhs: Value<S_5_> = Rc::new(RefCell::new(S_5_::new({ Some(0_i8) })));
            operator_eq_8(_lhs.as_pointer(), _rhs.as_pointer())
        })
    );
    let mut r2: bool = ({
        (*a.borrow()).compare_exchange_strong_25(
            field_ptr!(expected.as_pointer(), s),
            S_5_::new({ Some(18_i8) }),
            memory_order_relaxed_13.with(|rc| *rc.borrow()),
        )
    });
    assert!((r2 as bool));
    assert!(
        ({
            let _lhs: Value<S_5_> = Rc::new(RefCell::new(({ (*a.borrow()).load_15(None) })));
            let _rhs: Value<S_5_> = Rc::new(RefCell::new(S_5_::new({ Some(18_i8) })));
            operator_eq_8(_lhs.as_pointer(), _rhs.as_pointer())
        })
    );
    assert!(
        ({
            let _lhs: Ptr<S_5_> = field_ptr!(expected.as_pointer(), s);
            let _rhs: Value<S_5_> = Rc::new(RefCell::new(S_5_::new({ Some(0_i8) })));
            operator_eq_8(_lhs, _rhs.as_pointer())
        })
    );
    assert!(({ { (*expected.borrow()).b } }));
}
pub fn test_18() {
    let a: Value<std_atomic_S_6__> = Rc::new(RefCell::new(std_atomic_S_6__::new_1({
        S_6_::new({ Some(0_i8) })
    })));
    let expected: Value<Expected_6_> = Rc::new(RefCell::new(Expected_6_::new({
        S_6_::new({ Some(17_i8) })
    })));
    assert!(
        ({
            let _lhs: Value<S_6_> = Rc::new(RefCell::new(({ (*a.borrow()).load_15(None) })));
            operator_ne_11(_lhs.as_pointer(), field_ptr!(expected.as_pointer(), s))
        })
    );
    assert!(({ { (*expected.borrow()).b } }));
    let mut r1: bool = ({
        (*a.borrow()).compare_exchange_strong_25(
            field_ptr!(expected.as_pointer(), s),
            S_6_::new({ Some(18_i8) }),
            memory_order_relaxed_13.with(|rc| *rc.borrow()),
        )
    });
    assert!(!(r1));
    assert!(
        ({
            let _lhs: Ptr<S_6_> = field_ptr!(expected.as_pointer(), s);
            let _rhs: Value<S_6_> = Rc::new(RefCell::new(S_6_::new({ Some(0_i8) })));
            operator_eq_10(_lhs, _rhs.as_pointer())
        })
    );
    assert!(({ { (*expected.borrow()).b } }));
    assert!(
        ({
            let _lhs: Value<S_6_> = Rc::new(RefCell::new(({ (*a.borrow()).load_15(None) })));
            let _rhs: Value<S_6_> = Rc::new(RefCell::new(S_6_::new({ Some(0_i8) })));
            operator_eq_10(_lhs.as_pointer(), _rhs.as_pointer())
        })
    );
    let mut r2: bool = ({
        (*a.borrow()).compare_exchange_strong_25(
            field_ptr!(expected.as_pointer(), s),
            S_6_::new({ Some(18_i8) }),
            memory_order_relaxed_13.with(|rc| *rc.borrow()),
        )
    });
    assert!((r2 as bool));
    assert!(
        ({
            let _lhs: Value<S_6_> = Rc::new(RefCell::new(({ (*a.borrow()).load_15(None) })));
            let _rhs: Value<S_6_> = Rc::new(RefCell::new(S_6_::new({ Some(18_i8) })));
            operator_eq_10(_lhs.as_pointer(), _rhs.as_pointer())
        })
    );
    assert!(
        ({
            let _lhs: Ptr<S_6_> = field_ptr!(expected.as_pointer(), s);
            let _rhs: Value<S_6_> = Rc::new(RefCell::new(S_6_::new({ Some(0_i8) })));
            operator_eq_10(_lhs, _rhs.as_pointer())
        })
    );
    assert!(({ { (*expected.borrow()).b } }));
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
    ({ test_12() });
    ({ test_14() });
    ({ test_15() });
    ({ test_16() });
    ({ test_17() });
    ({ test_18() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
