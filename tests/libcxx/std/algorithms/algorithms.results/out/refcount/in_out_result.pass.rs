extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn operator_eq_0(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
pub fn operator_ne_1(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } != { y.with(|__s| __s.data_) });
}
pub fn operator_lt_2(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } < { y.with(|__s| __s.data_) });
}
pub fn operator_le_3(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } <= { y.with(|__s| __s.data_) });
}
pub fn operator_gt_4(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } > { y.with(|__s| __s.data_) });
}
pub fn operator_ge_5(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } >= { y.with(|__s| __s.data_) });
}
pub fn operator_cmp_6(_a0: Ptr<MoveOnly>, _a1: Ptr<MoveOnly>) -> () {
    {
        let cmp: Value<()> = Rc::new(RefCell::new(std::cmp::Ord::cmp(
            &(_a0.with(|__s| __s.data_)),
            &(_a1.with(|__s| __s.data_)),
        )));
        if !(unimplemented!()) {
            return unimplemented!();
        }
    }
    return unimplemented!();
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign)]
#[byte_size(4)]
pub struct MoveOnly {
    #[offset(0)]
    data_: (),
}
impl MoveOnly {
    pub fn new(data: Option<()>) -> Self {
        let mut data: () = data.unwrap_or_else(|| 1_());
        Self { data_: data }
    }
    pub fn move_from(x: Ptr<MoveOnly>) -> Self {
        let __this: MoveOnly = Self {
            data_: x.with(|__s| __s.data_),
        };
        field!(x, data_).write(0_());
        __this
    }
}
impl CppLess for MoveOnly {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_lt_2(__lhs.clone(), __rhs.clone())
    }
}
impl CppEq for MoveOnly {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_0(__lhs.clone(), __rhs.clone())
    }
}
impl Default for MoveOnly {
    fn default() -> Self {
        { MoveOnly::new(None) }
    }
}
pub fn operator_eq_7(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
pub fn operator_ne_8(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } != { y.with(|__s| __s.data_) });
}
pub fn operator_lt_9(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } < { y.with(|__s| __s.data_) });
}
pub fn operator_le_10(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } <= { y.with(|__s| __s.data_) });
}
pub fn operator_gt_11(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } > { y.with(|__s| __s.data_) });
}
pub fn operator_ge_12(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } >= { y.with(|__s| __s.data_) });
}
pub fn operator_cmp_13(_a0: Ptr<TrivialMoveOnly>, _a1: Ptr<TrivialMoveOnly>) -> () {
    {
        let cmp: Value<()> = Rc::new(RefCell::new(std::cmp::Ord::cmp(
            &(_a0.with(|__s| __s.data_)),
            &(_a1.with(|__s| __s.data_)),
        )));
        if !(unimplemented!()) {
            return unimplemented!();
        }
    }
    return unimplemented!();
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign)]
#[byte_size(4)]
pub struct TrivialMoveOnly {
    #[offset(0)]
    data_: (),
}
impl TrivialMoveOnly {
    pub fn new(data: Option<()>) -> Self {
        let mut data: () = data.unwrap_or_else(|| 1_());
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
        operator_lt_9(__lhs.clone(), __rhs.clone())
    }
}
impl CppEq for TrivialMoveOnly {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_7(__lhs.clone(), __rhs.clone())
    }
}
impl Default for TrivialMoveOnly {
    fn default() -> Self {
        { TrivialMoveOnly::new(None) }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct A {}
impl A {}
const _: () = assert!(
    true,
    "!std::is_constructible_v<std::ranges::in_out_result<A, A>, std::ranges::in_out_result<int, int>>"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct B {}
impl B {}
const _: () = assert!(
    true,
    "std::is_constructible_v<std::ranges::in_out_result<B, B>, std::ranges::in_out_result<int, int>>"
);
const _: () = assert!(
    true,
    "std::is_constructible_v<std::ranges::in_out_result<B, B>, std::ranges::in_out_result<int, int>&>"
);
const _: () = assert!(
    true,
    "std::is_constructible_v<std::ranges::in_out_result<B, B>, const std::ranges::in_out_result<int, int>>"
);
const _: () = assert!(
    true,
    "std::is_constructible_v<std::ranges::in_out_result<B, B>, const std::ranges::in_out_result<int, int>&>"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct C {}
impl C {}
const _: () = assert!(
    true,
    "!std::is_constructible_v<std::ranges::in_out_result<C, C>, std::ranges::in_out_result<int, int>&>"
);
const _: () = assert!(
    true,
    "std::is_convertible_v<std::ranges::in_out_result<int, int>&, std::ranges::in_out_result<long, long>>"
);
const _: () = assert!(
    true,
    "std::is_convertible_v<const std::ranges::in_out_result<int, int>&, std::ranges::in_out_result<long, long>>"
);
const _: () = assert!(
    true,
    "std::is_convertible_v<std::ranges::in_out_result<int, int>&&, std::ranges::in_out_result<long, long>>"
);
const _: () = assert!(
    true,
    "std::is_convertible_v<const std::ranges::in_out_result<int, int>&&, std::ranges::in_out_result<long, long>>"
);
const _: () = assert!(
    true,
    "std::is_move_constructible_v<std::ranges::in_out_result<MoveOnly, int>>"
);
const _: () = assert!(
    true,
    "std::is_move_constructible_v<std::ranges::in_out_result<int, MoveOnly>>"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct NotConvertible {}
const _: () = assert!(
    true,
    "!std::is_convertible_v<std::ranges::in_out_result<NotConvertible, int>, std::ranges::in_out_result<int, NotConvertible>>"
);
const _: () = assert!(
    true,
    "!std::is_convertible_v<std::ranges::in_out_result<int, NotConvertible>, std::ranges::in_out_result<NotConvertible, int>>"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct ConvertibleFrom_double_ {
    #[offset(0)]
    pub content: (),
}
impl ConvertibleFrom_double_ {
    pub fn new(mut c: ()) -> Self {
        Self { content: c }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct ConvertibleFrom_int_ {
    #[offset(0)]
    pub content: (),
}
impl ConvertibleFrom_int_ {
    pub fn new(mut c: ()) -> Self {
        Self { content: c }
    }
}
pub fn test_14() -> () {
    {
        let res: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        let res2: Value<()> = Rc::new(RefCell::new(({ (*res.borrow())() })));
        unimplemented!();
        unimplemented!();
    }
    {
        let res: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        let res2: Value<()> = Rc::new(RefCell::new((*res.borrow_mut()).clone()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let __decomp_15: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    ({ test_14() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub trait MoveOnlyImpl {
    fn move_assign(&self, x: Ptr<MoveOnly>) -> Ptr<MoveOnly>;
    fn get(&self) -> ();
    fn operator_add(&self, x: Ptr<MoveOnly>) -> MoveOnly;
    fn operator_mul(&self, x: Ptr<MoveOnly>) -> MoveOnly;
}
impl MoveOnlyImpl for Ptr<MoveOnly> {
    fn move_assign(&self, x: Ptr<MoveOnly>) -> Ptr<MoveOnly> {
        field!((*self), data_).write({ x.with(|__s| __s.data_) });
        field!(x, data_).write(0_());
        return (*self).clone();
    }
    fn get(&self) -> () {
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
    fn get(&self) -> ();
    fn operator_add(&self, x: Ptr<TrivialMoveOnly>) -> TrivialMoveOnly;
    fn operator_mul(&self, x: Ptr<TrivialMoveOnly>) -> TrivialMoveOnly;
}
impl TrivialMoveOnlyImpl for Ptr<TrivialMoveOnly> {
    fn move_assign(&self, _a0: Ptr<TrivialMoveOnly>) -> Ptr<TrivialMoveOnly> {
        field!((*self), data_).write({ { (*_a0.upgrade().deref()).data_ } });
        return (*self).clone();
    }
    fn get(&self) -> () {
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
