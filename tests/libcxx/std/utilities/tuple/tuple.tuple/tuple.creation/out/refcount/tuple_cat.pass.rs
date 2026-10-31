extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type anon_0 = u32;
pub const anon_0_value: anon_0 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_tuple_Unconstrained___std_tuple_Unconstrained__ {}
pub type anon_1 = u32;
pub const anon_1_value: anon_1 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_tuple_int__const_int__int_ref__const_int_ref__int_refref__int__const_int__int_ref__const_int_ref__int_refref__int__const_int__int_ref__const_int_ref__int__const_int__int_ref__const_int_ref___std_tuple_int__const_int__int_ref__const_int_ref__int_refref__int__const_int__int_ref__const_int_ref__int_refref__int__const_int__int_ref__const_int_ref__int__const_int__int_ref__const_int_ref__
{}
pub fn operator_eq_2(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
pub fn operator_ne_3(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } != { y.with(|__s| __s.data_) });
}
pub fn operator_lt_4(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } < { y.with(|__s| __s.data_) });
}
pub fn operator_le_5(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } <= { y.with(|__s| __s.data_) });
}
pub fn operator_gt_6(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } > { y.with(|__s| __s.data_) });
}
pub fn operator_ge_7(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } >= { y.with(|__s| __s.data_) });
}
pub fn operator_cmp_8(_a0: Ptr<MoveOnly>, _a1: Ptr<MoveOnly>) -> std::cmp::Ordering {
    {
        let cmp: Value<std::cmp::Ordering> = Rc::new(RefCell::new(std::cmp::Ord::cmp(
            &(_a0.with(|__s| __s.data_)),
            &(_a1.with(|__s| __s.data_)),
        )));
        if !((*cmp.borrow()) == std::cmp::Ordering::Equal) {
            return (*cmp.borrow());
        }
    }
    return std::cmp::Ordering::Equal;
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, CppFrom, MoveAssign)]
#[cpp_from(i32, new, some_value)]
#[byte_size(4)]
pub struct MoveOnly {
    #[offset(0)]
    data_: i32,
}
impl MoveOnly {
    pub fn new(data: Option<i32>) -> Self {
        let mut data: i32 = data.unwrap_or_else(|| 1);
        Self { data_: data }
    }
    pub fn move_from(x: Ptr<MoveOnly>) -> Self {
        let __this: MoveOnly = Self {
            data_: x.with(|__s| __s.data_),
        };
        field!(x, data_).write(0);
        __this
    }
}
impl CppLess for MoveOnly {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_lt_4(__lhs.clone(), __rhs.clone())
    }
}
impl CppEq for MoveOnly {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_2(__lhs.clone(), __rhs.clone())
    }
}
impl Default for MoveOnly {
    fn default() -> Self {
        { MoveOnly::new(None) }
    }
}
pub fn operator_eq_9(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
pub fn operator_ne_10(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } != { y.with(|__s| __s.data_) });
}
pub fn operator_lt_11(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } < { y.with(|__s| __s.data_) });
}
pub fn operator_le_12(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } <= { y.with(|__s| __s.data_) });
}
pub fn operator_gt_13(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } > { y.with(|__s| __s.data_) });
}
pub fn operator_ge_14(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } >= { y.with(|__s| __s.data_) });
}
pub fn operator_cmp_15(_a0: Ptr<TrivialMoveOnly>, _a1: Ptr<TrivialMoveOnly>) -> std::cmp::Ordering {
    {
        let cmp: Value<std::cmp::Ordering> = Rc::new(RefCell::new(std::cmp::Ord::cmp(
            &(_a0.with(|__s| __s.data_)),
            &(_a1.with(|__s| __s.data_)),
        )));
        if !((*cmp.borrow()) == std::cmp::Ordering::Equal) {
            return (*cmp.borrow());
        }
    }
    return std::cmp::Ordering::Equal;
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, CppFrom, MoveAssign)]
#[cpp_from(i32, new, some_value)]
#[byte_size(4)]
pub struct TrivialMoveOnly {
    #[offset(0)]
    data_: i32,
}
impl TrivialMoveOnly {
    pub fn new(data: Option<i32>) -> Self {
        let mut data: i32 = data.unwrap_or_else(|| 1);
        Self { data_: data }
    }
    pub fn move_from(_a0: Ptr<TrivialMoveOnly>) -> Self {
        Self {
            data_: { (*_a0.upgrade().deref()).data_ },
        }
    }
}
impl CppLess for TrivialMoveOnly {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_lt_11(__lhs.clone(), __rhs.clone())
    }
}
impl CppEq for TrivialMoveOnly {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_9(__lhs.clone(), __rhs.clone())
    }
}
impl Default for TrivialMoveOnly {
    fn default() -> Self {
        { TrivialMoveOnly::new(None) }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct NS_Namespaced {
    #[offset(0)]
    pub i: i32,
}
impl Unconstrained {
    pub fn new(mut arg: i32) -> Self {
        Self { data: arg }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(i32, new, value)]
#[byte_size(4)]
pub struct Unconstrained {
    #[offset(0)]
    pub data: i32,
}
pub fn test_tuple_cat_with_unconstrained_constructor_16() -> bool {
    {
        let tup_src: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let tup: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!((unimplemented!().with(|__s| __s.data) == 5));
    }
    {
        let tup: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!((unimplemented!().with(|__s| __s.data) == 6));
    }
    {
        let tup: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!((unimplemented!().with(|__s| __s.data) == 7));
    }
    {
        let tup_src: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let tup: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        assert!((unimplemented!().with(|__s| __s.data) == 8));
    }
    {
        let tup: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        assert!((unimplemented!().with(|__s| __s.data) == 9));
    }
    {
        let tup: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        assert!((unimplemented!().with(|__s| __s.data) == 10));
    }
    return true;
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
    {
        let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (&(*t.borrow_mut()));
    }
    {
        let t1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let t2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (&(*t2.borrow_mut()));
    }
    {
        let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (&(*t.borrow_mut()));
    }
    {
        let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (&(*t.borrow_mut()));
    }
    {
        let t1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(((unimplemented!().read()) == 1));
    }
    {
        let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (&(*t.borrow_mut()));
    }
    {
        let t1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let t2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (&(*t2.borrow_mut()));
    }
    {
        let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (&(*t.borrow_mut()));
    }
    {
        let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (&(*t.borrow_mut()));
    }
    {
        let t1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "std::get<0>(t) == 1");;
    }
    {
        let t1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "std::get<0>(t) == 1");;
        const _: () = assert!(true, "std::get<1>(t) == 1");;
    }
    {
        let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(((unimplemented!().read()) == 1));
        assert!(
            ({
                let _x: Ptr<MoveOnly> = unimplemented!();
                let _y: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ Some(2) })));
                operator_eq_2(_x, _y.as_pointer())
            })
        );
    }
    {
        let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(((unimplemented!().read()) == 0));
        assert!(((unimplemented!().read()) == 0));
        assert!(((unimplemented!().read()) == 0));
    }
    {
        let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(((unimplemented!().read()) == 2));
        assert!(
            ({
                let _x: Ptr<MoveOnly> = unimplemented!();
                let _y: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ Some(1) })));
                operator_eq_2(_x, _y.as_pointer())
            })
        );
    }
    {
        let t1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let t2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let t3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (&(*t3.borrow_mut()));
    }
    {
        let t1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let t2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let t3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(((unimplemented!().read()) == 2));
    }
    {
        let t1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let t2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let t3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(((unimplemented!().read()) == 2));
    }
    {
        let t1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let t2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let t3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!((unimplemented!().read()).is_null());
        assert!(((unimplemented!().read()) == 2));
    }
    {
        let t1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let t2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let t3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(((unimplemented!().read()) == 2));
        assert!((unimplemented!().read()).is_null());
    }
    {
        let t1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let t2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let t3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!((unimplemented!().read()).is_null());
        assert!(((unimplemented!().read()) == 2));
        assert!(((unimplemented!().read()) == 3.5_f64));
    }
    {
        let t1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let t2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let t3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(((unimplemented!().read()) == 2));
        assert!(((unimplemented!().read()) == 3.5_f64));
        assert!((unimplemented!().read()).is_null());
    }
    {
        let t1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let t2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let t3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!((unimplemented!().read()).is_null());
        assert!(
            ({
                let _x: Ptr<MoveOnly> = unimplemented!();
                let _y: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ Some(1) })));
                operator_eq_2(_x, _y.as_pointer())
            })
        );
        assert!(((unimplemented!().read()) == 2));
        assert!(((unimplemented!().read()) == 3.5_f64));
    }
    {
        let t1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let t2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let t3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(((unimplemented!().read()) == 2));
        assert!(((unimplemented!().read()) == 3.5_f64));
        assert!((unimplemented!().read()).is_null());
        assert!(
            ({
                let _x: Ptr<MoveOnly> = unimplemented!();
                let _y: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ Some(1) })));
                operator_eq_2(_x, _y.as_pointer())
            })
        );
    }
    {
        let t1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let t2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let t3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            ({
                let _x: Ptr<MoveOnly> = unimplemented!();
                let _y: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ Some(1) })));
                operator_eq_2(_x, _y.as_pointer())
            })
        );
        assert!(
            ({
                let _x: Ptr<MoveOnly> = unimplemented!();
                let _y: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ Some(2) })));
                operator_eq_2(_x, _y.as_pointer())
            })
        );
        assert!((unimplemented!().read()).is_null());
        assert!(
            ({
                let _x: Ptr<MoveOnly> = unimplemented!();
                let _y: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ Some(4) })));
                operator_eq_2(_x, _y.as_pointer())
            })
        );
    }
    {
        let t1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let t2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let t3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            ({
                let _x: Ptr<MoveOnly> = unimplemented!();
                let _y: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ Some(1) })));
                operator_eq_2(_x, _y.as_pointer())
            })
        );
        assert!(
            ({
                let _x: Ptr<MoveOnly> = unimplemented!();
                let _y: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ Some(2) })));
                operator_eq_2(_x, _y.as_pointer())
            })
        );
        assert!((unimplemented!().read()).is_null());
        assert!(
            ({
                let _x: Ptr<MoveOnly> = unimplemented!();
                let _y: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ Some(4) })));
                operator_eq_2(_x, _y.as_pointer())
            })
        );
    }
    {
        let t1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let t2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let t3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            ({
                let _x: Ptr<MoveOnly> = unimplemented!();
                let _y: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ Some(1) })));
                operator_eq_2(_x, _y.as_pointer())
            })
        );
        assert!(
            ({
                let _x: Ptr<MoveOnly> = unimplemented!();
                let _y: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ Some(2) })));
                operator_eq_2(_x, _y.as_pointer())
            })
        );
        assert!((unimplemented!().read()).is_null());
        assert!(
            ({
                let _x: Ptr<MoveOnly> = unimplemented!();
                let _y: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ Some(4) })));
                operator_eq_2(_x, _y.as_pointer())
            })
        );
    }
    {
        let t1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let t2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let t3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            ({
                let _x: Ptr<MoveOnly> = unimplemented!();
                let _y: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ Some(1) })));
                operator_eq_2(_x, _y.as_pointer())
            })
        );
        assert!(
            ({
                let _x: Ptr<MoveOnly> = unimplemented!();
                let _y: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ Some(2) })));
                operator_eq_2(_x, _y.as_pointer())
            })
        );
        assert!((unimplemented!().read()).is_null());
        assert!(
            ({
                let _x: Ptr<MoveOnly> = unimplemented!();
                let _y: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ Some(4) })));
                operator_eq_2(_x, _y.as_pointer())
            })
        );
    }
    {
        let t1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let t2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let t3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            ({
                let _x: Ptr<MoveOnly> = unimplemented!();
                let _y: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ Some(1) })));
                operator_eq_2(_x, _y.as_pointer())
            })
        );
        assert!(
            ({
                let _x: Ptr<MoveOnly> = unimplemented!();
                let _y: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ Some(2) })));
                operator_eq_2(_x, _y.as_pointer())
            })
        );
        assert!((unimplemented!().read()).is_null());
        assert!(
            ({
                let _x: Ptr<MoveOnly> = unimplemented!();
                let _y: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ Some(4) })));
                operator_eq_2(_x, _y.as_pointer())
            })
        );
        assert!(((unimplemented!().read()) == 5));
    }
    {
        let t1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(unimplemented!());
        let t2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(unimplemented!());
    }
    {
        let x: Value<i32> = Rc::new(RefCell::new(101));
        let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ct: Ptr<()> = t.as_pointer();
        let t2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ct2: Ptr<()> = t2.as_pointer();
        let r: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        (&(*r.borrow_mut()));
    }
    {
        let t1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let t2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!((unimplemented!().with(|__s| __s.i) == 1));
        assert!((unimplemented!().with(|__s| __s.i) == 1));
    }
    {
        ({ test_tuple_cat_with_unconstrained_constructor_16() });
        const _: () = assert!(true, "test_tuple_cat_with_unconstrained_constructor()");;
    }
    return 0;
}
pub trait MoveOnlyImpl {
    fn move_assign(&self, x: Ptr<MoveOnly>) -> Ptr<MoveOnly>;
    fn get(&self) -> i32;
    fn operator_add(&self, x: Ptr<MoveOnly>) -> MoveOnly;
    fn operator_mul(&self, x: Ptr<MoveOnly>) -> MoveOnly;
}
impl MoveOnlyImpl for Ptr<MoveOnly> {
    fn move_assign(&self, x: Ptr<MoveOnly>) -> Ptr<MoveOnly> {
        field!((*self), data_).write({ x.with(|__s| __s.data_) });
        field!(x, data_).write(0);
        return (*self).clone();
    }
    fn get(&self) -> i32 {
        return (*self).with(|__s| __s.data_);
    }
    fn operator_add(&self, x: Ptr<MoveOnly>) -> MoveOnly {
        return MoveOnly::new({
            Some(({ (*self).with(|__s| __s.data_) } + { x.with(|__s| __s.data_) }))
        });
    }
    fn operator_mul(&self, x: Ptr<MoveOnly>) -> MoveOnly {
        return MoveOnly::new({
            Some(({ (*self).with(|__s| __s.data_) } * { x.with(|__s| __s.data_) }))
        });
    }
}
pub trait TrivialMoveOnlyImpl {
    fn move_assign(&self, _a0: Ptr<TrivialMoveOnly>) -> Ptr<TrivialMoveOnly>;
    fn get(&self) -> i32;
    fn operator_add(&self, x: Ptr<TrivialMoveOnly>) -> TrivialMoveOnly;
    fn operator_mul(&self, x: Ptr<TrivialMoveOnly>) -> TrivialMoveOnly;
}
impl TrivialMoveOnlyImpl for Ptr<TrivialMoveOnly> {
    fn move_assign(&self, _a0: Ptr<TrivialMoveOnly>) -> Ptr<TrivialMoveOnly> {
        field!((*self), data_).write({ { (*_a0.upgrade().deref()).data_ } });
        return (*self).clone();
    }
    fn get(&self) -> i32 {
        return (*self).with(|__s| __s.data_);
    }
    fn operator_add(&self, x: Ptr<TrivialMoveOnly>) -> TrivialMoveOnly {
        return TrivialMoveOnly::new({
            Some(({ (*self).with(|__s| __s.data_) } + { x.with(|__s| __s.data_) }))
        });
    }
    fn operator_mul(&self, x: Ptr<TrivialMoveOnly>) -> TrivialMoveOnly {
        return TrivialMoveOnly::new({
            Some(({ (*self).with(|__s| __s.data_) } * { x.with(|__s| __s.data_) }))
        });
    }
}
pub fn __cpp2rust_init_globals() {}
