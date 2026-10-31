extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct transparent_less {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct transparent_less_not_referenceable {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct transparent_less_nonempty_is_transparent {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct transparent_less_nonempty {
    #[offset(0)]
    #[byte_size(1)]
    pub pad_: transparent_less_nonempty_is_transparent,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct transparent_less_no_type {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct transparent_less_private {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct transparent_less_not_a_type {
    #[offset(0)]
    pub is_transparent: i32,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom)]
#[cpp_from(i32, new_1, value)]
#[byte_size(4)]
pub struct C2Int {
    #[offset(0)]
    i_: i32,
}
impl C2Int {
    pub fn new_2() -> Self {
        Self { i_: 0 }
    }
    pub fn new_1(mut i: i32) -> Self {
        Self { i_: i }
    }
}
impl CppLess for C2Int {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_lt_0(__lhs.clone(), __rhs.clone())
    }
}
impl CppEq for C2Int {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        !(operator_lt_0(__lhs.clone(), __rhs.clone()))
            && !(operator_lt_0(__rhs.clone(), __lhs.clone()))
    }
}
impl Default for C2Int {
    fn default() -> Self {
        { C2Int::new_2() }
    }
}
pub fn operator_lt_1(mut rhs: i32, lhs: Ptr<C2Int>) -> bool {
    return ({ rhs } < { ({ C2IntImpl::get(&lhs) }) });
}
pub fn operator_lt_0(rhs: Ptr<C2Int>, lhs: Ptr<C2Int>) -> bool {
    return ({ ({ C2IntImpl::get(&rhs) }) } < { ({ C2IntImpl::get(&lhs) }) });
}
pub fn operator_lt_2(rhs: Ptr<C2Int>, mut lhs: i32) -> bool {
    return ({ ({ C2IntImpl::get(&rhs) }) } < { lhs });
}
pub fn operator_eq_3(lhs: Ptr<StoredType_int_>, rhs: Ptr<StoredType_int_>) -> bool {
    return ({ lhs.with(|__s| __s.value_) } == { rhs.with(|__s| __s.value_) });
}
pub fn operator_eq_4(lhs: Ptr<StoredType_int_>, rhs: Ptr<SearchedType_int_>) -> bool {
    return ({ lhs.with(|__s| __s.value_) } == { ({ SearchedType_int_Impl::get_value(&rhs) }) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(i32, new_1, value)]
#[byte_size(4)]
pub struct StoredType_int_ {
    #[offset(0)]
    value_: i32,
}
impl StoredType_int_ {
    pub fn new_1(mut value: i32) -> Self {
        Self { value_: value }
    }
}
impl CppEq for StoredType_int_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_3(__lhs.clone(), __rhs.clone())
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_into(StoredType_int_, to_StoredType_int_, ptr)]
#[byte_size(16)]
pub struct SearchedType_int_ {
    #[offset(0)]
    value_: i32,
    #[offset(8)]
    #[byte_size(8)]
    conversions_: Ptr<i32>,
}
impl SearchedType_int_ {
    pub fn new(mut value: i32, mut counter: Ptr<i32>) -> Self {
        Self {
            value_: value,
            conversions_: (counter).clone(),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct hash_impl {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct non_transparent_hash {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct transparent_hash {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct transparent_hash_final {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct transparent_equal_final {}
pub fn test_transparent_equal_range_5(c: ()) {
    let c: Value<()> = Rc::new(RefCell::new(c));
    let conversions: Value<i32> = Rc::new(RefCell::new(0));
    let iters: Value<(Value<()>, Value<()>)> = Rc::new(RefCell::new(unimplemented!()));
    assert!((unimplemented!() > 0_isize));
    iters.as_pointer().write(unimplemented!());
    assert!((unimplemented!() > 0_isize));
    iters.as_pointer().write(unimplemented!());
    assert!((unimplemented!() == 0_isize));
    assert!(((*conversions.borrow()) == 0));
}
pub fn test_transparent_equal_range_6(c: ()) {
    let c: Value<()> = Rc::new(RefCell::new(c));
    let conversions: Value<i32> = Rc::new(RefCell::new(0));
    let iters: Value<(Value<()>, Value<()>)> = Rc::new(RefCell::new(unimplemented!()));
    assert!((unimplemented!() > 0_isize));
    iters.as_pointer().write(unimplemented!());
    assert!((unimplemented!() > 0_isize));
    iters.as_pointer().write(unimplemented!());
    assert!((unimplemented!() == 0_isize));
    assert!(((*conversions.borrow()) == 0));
}
pub fn test_transparent_equal_range_7(c: ()) {
    let c: Value<()> = Rc::new(RefCell::new(c));
    let conversions: Value<i32> = Rc::new(RefCell::new(0));
    let iters: Value<(Value<()>, Value<()>)> = Rc::new(RefCell::new(unimplemented!()));
    assert!((unimplemented!() > 0_isize));
    iters.as_pointer().write(unimplemented!());
    assert!((unimplemented!() > 0_isize));
    iters.as_pointer().write(unimplemented!());
    assert!((unimplemented!() == 0_isize));
    assert!(((*conversions.borrow()) == 0));
}
pub fn test_transparent_equal_range_8(c: ()) {
    let c: Value<()> = Rc::new(RefCell::new(c));
    let conversions: Value<i32> = Rc::new(RefCell::new(0));
    let iters: Value<(Value<()>, Value<()>)> = Rc::new(RefCell::new(unimplemented!()));
    assert!((unimplemented!() > 0_isize));
    iters.as_pointer().write(unimplemented!());
    assert!((unimplemented!() > 0_isize));
    iters.as_pointer().write(unimplemented!());
    assert!((unimplemented!() == 0_isize));
    assert!(((*conversions.borrow()) == 0));
}
pub fn test_non_transparent_equal_range_9(c: ()) {
    let c: Value<()> = Rc::new(RefCell::new(c));
    let conversions: Value<i32> = Rc::new(RefCell::new(0));
    let iters: Value<(Value<()>, Value<()>)> = Rc::new(RefCell::new(unimplemented!()));
    assert!((unimplemented!() > 0_isize));
    assert!(((*conversions.borrow()) == 1));
    iters.as_pointer().write(unimplemented!());
    assert!((unimplemented!() > 0_isize));
    assert!(((*conversions.borrow()) == 2));
    iters.as_pointer().write(unimplemented!());
    assert!((unimplemented!() == 0_isize));
    assert!(((*conversions.borrow()) == 3));
}
pub fn test_non_transparent_equal_range_10(c: ()) {
    let c: Value<()> = Rc::new(RefCell::new(c));
    let conversions: Value<i32> = Rc::new(RefCell::new(0));
    let iters: Value<(Value<()>, Value<()>)> = Rc::new(RefCell::new(unimplemented!()));
    assert!((unimplemented!() > 0_isize));
    assert!(((*conversions.borrow()) == 1));
    iters.as_pointer().write(unimplemented!());
    assert!((unimplemented!() > 0_isize));
    assert!(((*conversions.borrow()) == 2));
    iters.as_pointer().write(unimplemented!());
    assert!((unimplemented!() == 0_isize));
    assert!(((*conversions.borrow()) == 3));
}
pub fn test_non_transparent_equal_range_11(c: ()) {
    let c: Value<()> = Rc::new(RefCell::new(c));
    let conversions: Value<i32> = Rc::new(RefCell::new(0));
    let iters: Value<(Value<()>, Value<()>)> = Rc::new(RefCell::new(unimplemented!()));
    assert!((unimplemented!() > 0_isize));
    assert!(((*conversions.borrow()) == 1));
    iters.as_pointer().write(unimplemented!());
    assert!((unimplemented!() > 0_isize));
    assert!(((*conversions.borrow()) == 2));
    iters.as_pointer().write(unimplemented!());
    assert!((unimplemented!() == 0_isize));
    assert!(((*conversions.borrow()) == 3));
}
pub fn test_non_transparent_equal_range_12(c: ()) {
    let c: Value<()> = Rc::new(RefCell::new(c));
    let conversions: Value<i32> = Rc::new(RefCell::new(0));
    let iters: Value<(Value<()>, Value<()>)> = Rc::new(RefCell::new(unimplemented!()));
    assert!((unimplemented!() > 0_isize));
    assert!(((*conversions.borrow()) == 1));
    iters.as_pointer().write(unimplemented!());
    assert!((unimplemented!() > 0_isize));
    assert!(((*conversions.borrow()) == 2));
    iters.as_pointer().write(unimplemented!());
    assert!((unimplemented!() == 0_isize));
    assert!(((*conversions.borrow()) == 3));
}
pub fn test_non_transparent_equal_range_13(c: ()) {
    let c: Value<()> = Rc::new(RefCell::new(c));
    let conversions: Value<i32> = Rc::new(RefCell::new(0));
    let iters: Value<(Value<()>, Value<()>)> = Rc::new(RefCell::new(unimplemented!()));
    assert!((unimplemented!() > 0_isize));
    assert!(((*conversions.borrow()) == 1));
    iters.as_pointer().write(unimplemented!());
    assert!((unimplemented!() > 0_isize));
    assert!(((*conversions.borrow()) == 2));
    iters.as_pointer().write(unimplemented!());
    assert!((unimplemented!() == 0_isize));
    assert!(((*conversions.borrow()) == 3));
}
pub fn test_non_transparent_equal_range_14(c: ()) {
    let c: Value<()> = Rc::new(RefCell::new(c));
    let conversions: Value<i32> = Rc::new(RefCell::new(0));
    let iters: Value<(Value<()>, Value<()>)> = Rc::new(RefCell::new(unimplemented!()));
    assert!((unimplemented!() > 0_isize));
    assert!(((*conversions.borrow()) == 1));
    iters.as_pointer().write(unimplemented!());
    assert!((unimplemented!() > 0_isize));
    assert!(((*conversions.borrow()) == 2));
    iters.as_pointer().write(unimplemented!());
    assert!((unimplemented!() == 0_isize));
    assert!(((*conversions.borrow()) == 3));
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
        ({ test_transparent_equal_range_5(unimplemented!()) });
        ({ test_transparent_equal_range_6(unimplemented!()) });
    }
    {
        ({ test_transparent_equal_range_7(unimplemented!()) });
        ({ test_transparent_equal_range_8(unimplemented!()) });
    }
    {
        ({ test_non_transparent_equal_range_9(unimplemented!()) });
        ({ test_non_transparent_equal_range_10(unimplemented!()) });
    }
    {
        ({ test_non_transparent_equal_range_11(unimplemented!()) });
        ({ test_non_transparent_equal_range_12(unimplemented!()) });
    }
    {
        ({ test_non_transparent_equal_range_13(unimplemented!()) });
        ({ test_non_transparent_equal_range_14(unimplemented!()) });
    }
    return 0;
}
pub trait C2IntImpl {
    fn get(&self) -> i32;
}
impl C2IntImpl for Ptr<C2Int> {
    fn get(&self) -> i32 {
        return (*self).with(|__s| __s.i_);
    }
}
pub trait SearchedType_int_Impl {
    fn to_StoredType_int_(&self) -> StoredType_int_;
    fn get_value(&self) -> i32;
}
impl SearchedType_int_Impl for Ptr<SearchedType_int_> {
    fn to_StoredType_int_(&self) -> StoredType_int_ {
        (*self)
            .with(|__s| __s.conversions_.clone())
            .with_mut(|__v| __v.prefix_inc());
        return StoredType_int_::new_1({ (*self).with(|__s| __s.value_) });
    }
    fn get_value(&self) -> i32 {
        return (*self).with(|__s| __s.value_);
    }
}
pub trait StoredType_int_Impl {
    fn get_value(&self) -> i32;
}
impl StoredType_int_Impl for Ptr<StoredType_int_> {
    fn get_value(&self) -> i32 {
        return (*self).with(|__s| __s.value_);
    }
}
pub trait hash_implImpl {
    fn operator_call_1_int(&self, t: Ptr<SearchedType_int_>) -> usize;
    fn operator_call_2_int(&self, t: Ptr<StoredType_int_>) -> usize;
}
impl hash_implImpl for Ptr<hash_impl> {
    fn operator_call_1_int(&self, t: Ptr<SearchedType_int_>) -> usize {
        return (({ SearchedType_int_Impl::get_value(&t) }) as usize);
    }
    fn operator_call_2_int(&self, t: Ptr<StoredType_int_>) -> usize {
        return (({ StoredType_int_Impl::get_value(&t) }) as usize);
    }
}
pub fn __cpp2rust_init_globals() {}
