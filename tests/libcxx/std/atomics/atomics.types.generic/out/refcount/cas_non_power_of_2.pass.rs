extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn operator_eq_0(lhs: Ptr<S_1_>, rhs: Ptr<S_1_>) -> () {
    return (unimplemented!() == 0_());
}
pub fn operator_ne_1(lhs: Ptr<S_1_>, rhs: Ptr<S_1_>) -> () {
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
    pub data: Box<[()]>,
}
impl S_1_ {
    pub fn new(v: Option<()>) -> Self {
        let mut v: () = v.unwrap_or_else(|| 0_());
        let __this: S_1_ = Self {
            data: (0..1).map(|_| unimplemented!()).collect::<Box<[()]>>(),
        };
        unimplemented!();
        __this
    }
}
impl CppEq for S_1_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_0(__lhs.clone(), __rhs.clone())
    }
}
impl Default for S_1_ {
    fn default() -> Self {
        { S_1_::new(None) }
    }
}
pub fn operator_eq_2(lhs: Ptr<S_2_>, rhs: Ptr<S_2_>) -> () {
    return (unimplemented!() == 0_());
}
pub fn operator_ne_3(lhs: Ptr<S_2_>, rhs: Ptr<S_2_>) -> () {
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
    pub data: Box<[()]>,
}
impl S_2_ {
    pub fn new(v: Option<()>) -> Self {
        let mut v: () = v.unwrap_or_else(|| 0_());
        let __this: S_2_ = Self {
            data: (0..2).map(|_| unimplemented!()).collect::<Box<[()]>>(),
        };
        unimplemented!();
        __this
    }
}
impl CppEq for S_2_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_2(__lhs.clone(), __rhs.clone())
    }
}
impl Default for S_2_ {
    fn default() -> Self {
        { S_2_::new(None) }
    }
}
pub fn operator_eq_4(lhs: Ptr<S_3_>, rhs: Ptr<S_3_>) -> () {
    return (unimplemented!() == 0_());
}
pub fn operator_ne_5(lhs: Ptr<S_3_>, rhs: Ptr<S_3_>) -> () {
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
    pub data: Box<[()]>,
}
impl S_3_ {
    pub fn new(v: Option<()>) -> Self {
        let mut v: () = v.unwrap_or_else(|| 0_());
        let __this: S_3_ = Self {
            data: (0..3).map(|_| unimplemented!()).collect::<Box<[()]>>(),
        };
        unimplemented!();
        __this
    }
}
impl CppEq for S_3_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_4(__lhs.clone(), __rhs.clone())
    }
}
impl Default for S_3_ {
    fn default() -> Self {
        { S_3_::new(None) }
    }
}
pub fn operator_eq_6(lhs: Ptr<S_4_>, rhs: Ptr<S_4_>) -> () {
    return (unimplemented!() == 0_());
}
pub fn operator_ne_7(lhs: Ptr<S_4_>, rhs: Ptr<S_4_>) -> () {
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
    pub data: Box<[()]>,
}
impl S_4_ {
    pub fn new(v: Option<()>) -> Self {
        let mut v: () = v.unwrap_or_else(|| 0_());
        let __this: S_4_ = Self {
            data: (0..4).map(|_| unimplemented!()).collect::<Box<[()]>>(),
        };
        unimplemented!();
        __this
    }
}
impl CppEq for S_4_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_6(__lhs.clone(), __rhs.clone())
    }
}
impl Default for S_4_ {
    fn default() -> Self {
        { S_4_::new(None) }
    }
}
pub fn operator_eq_8(lhs: Ptr<S_5_>, rhs: Ptr<S_5_>) -> () {
    return (unimplemented!() == 0_());
}
pub fn operator_ne_9(lhs: Ptr<S_5_>, rhs: Ptr<S_5_>) -> () {
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
    pub data: Box<[()]>,
}
impl S_5_ {
    pub fn new(v: Option<()>) -> Self {
        let mut v: () = v.unwrap_or_else(|| 0_());
        let __this: S_5_ = Self {
            data: (0..5).map(|_| unimplemented!()).collect::<Box<[()]>>(),
        };
        unimplemented!();
        __this
    }
}
impl CppEq for S_5_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_8(__lhs.clone(), __rhs.clone())
    }
}
impl Default for S_5_ {
    fn default() -> Self {
        { S_5_::new(None) }
    }
}
pub fn operator_eq_10(lhs: Ptr<S_6_>, rhs: Ptr<S_6_>) -> () {
    return (unimplemented!() == 0_());
}
pub fn operator_ne_11(lhs: Ptr<S_6_>, rhs: Ptr<S_6_>) -> () {
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
    pub data: Box<[()]>,
}
impl S_6_ {
    pub fn new(v: Option<()>) -> Self {
        let mut v: () = v.unwrap_or_else(|| 0_());
        let __this: S_6_ = Self {
            data: (0..6).map(|_| unimplemented!()).collect::<Box<[()]>>(),
        };
        unimplemented!();
        __this
    }
}
impl CppEq for S_6_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_10(__lhs.clone(), __rhs.clone())
    }
}
impl Default for S_6_ {
    fn default() -> Self {
        { S_6_::new(None) }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(2)]
pub struct Expected_1_ {
    #[offset(0)]
    #[byte_size(1)]
    pub s: S_1_,
    #[offset(1)]
    pub b: (),
}
impl Expected_1_ {
    pub fn new(mut ss: S_1_) -> Self {
        Self {
            s: (ss).clone(),
            b: true,
        }
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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(3)]
pub struct Expected_2_ {
    #[offset(0)]
    #[byte_size(2)]
    pub s: S_2_,
    #[offset(2)]
    pub b: (),
}
impl Expected_2_ {
    pub fn new(mut ss: S_2_) -> Self {
        Self {
            s: (ss).clone(),
            b: true,
        }
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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct Expected_3_ {
    #[offset(0)]
    #[byte_size(3)]
    pub s: S_3_,
    #[offset(3)]
    pub b: (),
}
impl Expected_3_ {
    pub fn new(mut ss: S_3_) -> Self {
        Self {
            s: (ss).clone(),
            b: true,
        }
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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(5)]
pub struct Expected_4_ {
    #[offset(0)]
    #[byte_size(4)]
    pub s: S_4_,
    #[offset(4)]
    pub b: (),
}
impl Expected_4_ {
    pub fn new(mut ss: S_4_) -> Self {
        Self {
            s: (ss).clone(),
            b: true,
        }
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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(6)]
pub struct Expected_5_ {
    #[offset(0)]
    #[byte_size(5)]
    pub s: S_5_,
    #[offset(5)]
    pub b: (),
}
impl Expected_5_ {
    pub fn new(mut ss: S_5_) -> Self {
        Self {
            s: (ss).clone(),
            b: true,
        }
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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(7)]
pub struct Expected_6_ {
    #[offset(0)]
    #[byte_size(6)]
    pub s: S_6_,
    #[offset(6)]
    pub b: (),
}
impl Expected_6_ {
    pub fn new(mut ss: S_6_) -> Self {
        Self {
            s: (ss).clone(),
            b: true,
        }
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
    let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let expected: Value<Expected_1_> = Rc::new(RefCell::new(Expected_1_::new({
        S_1_::new({ Some(17_()) })
    })));
    unimplemented!();
    unimplemented!();
    let mut r1: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut r2: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_13() {
    let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let expected: Value<Expected_2_> = Rc::new(RefCell::new(Expected_2_::new({
        S_2_::new({ Some(17_()) })
    })));
    unimplemented!();
    unimplemented!();
    let mut r1: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut r2: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_14() {
    let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let expected: Value<Expected_3_> = Rc::new(RefCell::new(Expected_3_::new({
        S_3_::new({ Some(17_()) })
    })));
    unimplemented!();
    unimplemented!();
    let mut r1: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut r2: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_15() {
    let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let expected: Value<Expected_4_> = Rc::new(RefCell::new(Expected_4_::new({
        S_4_::new({ Some(17_()) })
    })));
    unimplemented!();
    unimplemented!();
    let mut r1: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut r2: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_16() {
    let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let expected: Value<Expected_5_> = Rc::new(RefCell::new(Expected_5_::new({
        S_5_::new({ Some(17_()) })
    })));
    unimplemented!();
    unimplemented!();
    let mut r1: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut r2: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_17() {
    let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let expected: Value<Expected_6_> = Rc::new(RefCell::new(Expected_6_::new({
        S_6_::new({ Some(17_()) })
    })));
    unimplemented!();
    unimplemented!();
    let mut r1: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut r2: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    ({ test_12() });
    ({ test_13() });
    ({ test_14() });
    ({ test_15() });
    ({ test_16() });
    ({ test_17() });
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
