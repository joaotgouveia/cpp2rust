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
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub fn outstanding_alloc() -> () {
        unimplemented!();
        return ((alloc_count_1.with(|rc| *rc.borrow()))
            .wrapping_sub(dealloc_count_2.with(|rc| *rc.borrow())));
    }
    pub fn reset() {
        unimplemented!();
        disable_default_constructor_3.with(|rc| *rc.borrow_mut() = false);
        outstanding_bytes_0.with(|rc| *rc.borrow_mut() = 0_());
        alloc_count_1.with(|rc| *rc.borrow_mut() = 0_());
        dealloc_count_2.with(|rc| *rc.borrow_mut() = 0_());
    }
}
thread_local!(
    pub static outstanding_bytes_0: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static alloc_count_1: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static dealloc_count_2: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static disable_default_constructor_3: Value<()> = Rc::new(RefCell::new(false));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_std_pair_const_int__double___std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<()>,
}
impl min_pointer_std_pair_const_int__double___std_integral_constant_unsigned_long__0__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: (),
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_std_pair_const_int__double__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct PrivateConstructor {
    #[offset(0)]
    val: (),
}
impl PrivateConstructor {
    pub fn make(mut v: ()) -> PrivateConstructor {
        return PrivateConstructor::new({ v });
    }
    fn new(mut v: ()) -> Self {
        Self { val: v }
    }
}
impl CppLess for PrivateConstructor {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_lt_4(__lhs.clone(), __rhs.clone())
    }
}
impl CppEq for PrivateConstructor {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        !(operator_lt_4(__lhs.clone(), __rhs.clone()))
            && !(operator_lt_4(__rhs.clone(), __lhs.clone()))
    }
}
pub fn operator_lt_4(lhs: Ptr<PrivateConstructor>, rhs: Ptr<PrivateConstructor>) -> () {
    return ({ ({ PrivateConstructorImpl::get(&lhs) }) } < {
        ({ PrivateConstructorImpl::get(&rhs) })
    });
}
pub fn operator_lt_5(lhs: Ptr<PrivateConstructor>, mut rhs: ()) -> () {
    return ({ ({ PrivateConstructorImpl::get(&lhs) }) } < { rhs });
}
pub fn operator_lt_6(mut lhs: (), rhs: Ptr<PrivateConstructor>) -> () {
    return ({ lhs } < { ({ PrivateConstructorImpl::get(&rhs) }) });
}
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
    pub is_transparent: (),
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct C2Int {
    #[offset(0)]
    i_: (),
}
impl C2Int {
    pub fn new_1() -> Self {
        Self { i_: 0_() }
    }
    pub fn new_2(mut i: ()) -> Self {
        Self { i_: i }
    }
}
impl CppLess for C2Int {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_lt_7(__lhs.clone(), __rhs.clone())
    }
}
impl CppEq for C2Int {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        !(operator_lt_7(__lhs.clone(), __rhs.clone()))
            && !(operator_lt_7(__rhs.clone(), __lhs.clone()))
    }
}
impl Default for C2Int {
    fn default() -> Self {
        { C2Int::new_1() }
    }
}
pub fn operator_lt_8(mut rhs: (), lhs: Ptr<C2Int>) -> () {
    return ({ rhs } < { ({ C2IntImpl::get(&lhs) }) });
}
pub fn operator_lt_7(rhs: Ptr<C2Int>, lhs: Ptr<C2Int>) -> () {
    return ({ ({ C2IntImpl::get(&rhs) }) } < { ({ C2IntImpl::get(&lhs) }) });
}
pub fn operator_lt_9(rhs: Ptr<C2Int>, mut lhs: ()) -> () {
    return ({ ({ C2IntImpl::get(&rhs) }) } < { lhs });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct FinalCompare_int_ {}
pub fn test_10() -> () {
    let ar: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ])));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut i: () = 0_();
    'loop_: while (i < 5_()) {
        let mut r: () = unimplemented!();
        unimplemented!();
        i.prefix_inc();
    }
    let mut i: () = 5_();
    'loop_: while (i < 13_()) {
        let mut r: () = unimplemented!();
        unimplemented!();
        i.prefix_inc();
    }
    return true;
}
pub fn test_11() -> () {
    let ar: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ])));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut i: () = 0_();
    'loop_: while (i < 5_()) {
        let mut r: () = unimplemented!();
        unimplemented!();
        i.prefix_inc();
    }
    let mut i: () = 5_();
    'loop_: while (i < 13_()) {
        let mut r: () = unimplemented!();
        unimplemented!();
        i.prefix_inc();
    }
    return true;
}
pub fn test_12() -> () {
    let ar: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ])));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut i: () = 0_();
    'loop_: while (i < 5_()) {
        let mut r: () = unimplemented!();
        unimplemented!();
        i.prefix_inc();
    }
    let mut i: () = 5_();
    'loop_: while (i < 13_()) {
        let mut r: () = unimplemented!();
        unimplemented!();
        i.prefix_inc();
    }
    return true;
}
pub fn test_13() -> () {
    let ar: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ])));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut i: () = 0_();
    'loop_: while (i < 5_()) {
        let mut r: () = unimplemented!();
        unimplemented!();
        i.prefix_inc();
    }
    let mut i: () = 5_();
    'loop_: while (i < 13_()) {
        let mut r: () = unimplemented!();
        unimplemented!();
        i.prefix_inc();
    }
    return true;
}
pub fn test_14() -> () {
    let ar: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ])));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut i: () = 0_();
    'loop_: while (i < 5_()) {
        let mut r: () = unimplemented!();
        unimplemented!();
        i.prefix_inc();
    }
    let mut i: () = 5_();
    'loop_: while (i < 13_()) {
        let mut r: () = unimplemented!();
        unimplemented!();
        i.prefix_inc();
    }
    return true;
}
pub fn test_15() -> () {
    ({ test_10() });
    ({ test_11() });
    ({ test_12() });
    ({ test_13() });
    ({ test_14() });
    {
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!().write(5_());
        unimplemented!().write(6_());
        unimplemented!().write(7_());
        unimplemented!().write(8_());
        unimplemented!().write(9_());
        unimplemented!().write(10_());
        unimplemented!().write(11_());
        unimplemented!().write(12_());
        let mut i: () = 0_();
        'loop_: while (i < 5_()) {
            let mut r: () = unimplemented!();
            unimplemented!();
            i.prefix_inc();
        }
        let mut i: () = 5_();
        'loop_: while (i < 13_()) {
            let mut r: () = unimplemented!();
            unimplemented!();
            i.prefix_inc();
        }
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
    ({ test_15() });
    return 0_();
}
pub trait C2IntImpl {
    fn get(&self) -> ();
}
impl C2IntImpl for Ptr<C2Int> {
    fn get(&self) -> () {
        return (*self).with(|__s| __s.i_);
    }
}
pub trait FinalCompare_int_Impl {
    fn operator_call(&self, x: Ptr<()>, y: Ptr<()>) -> ();
}
impl FinalCompare_int_Impl for Ptr<FinalCompare_int_> {
    fn operator_call(&self, x: Ptr<()>, y: Ptr<()>) -> () {
        return ({ (x.read()) } < { (y.read()) });
    }
}
pub trait PrivateConstructorImpl {
    fn get(&self) -> ();
}
impl PrivateConstructorImpl for Ptr<PrivateConstructor> {
    fn get(&self) -> () {
        return (*self).with(|__s| __s.val);
    }
}
pub trait min_allocator_std_pair_const_int__double__Impl {
    fn allocate(
        &self,
        mut n: (),
    ) -> min_pointer_std_pair_const_int__double___std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn deallocate(
        &self,
        mut p: min_pointer_std_pair_const_int__double___std_integral_constant_unsigned_long__0__,
        mut n: (),
    ) {
        unimplemented!()
    }
}
impl min_allocator_std_pair_const_int__double__Impl
    for Ptr<min_allocator_std_pair_const_int__double__>
{
}
pub trait min_pointer_std_pair_const_int__double___std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_arrow(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_inc_8(
        &self,
    ) -> Ptr<min_pointer_std_pair_const_int__double___std_integral_constant_unsigned_long__0__>
    {
        unimplemented!()
    }
    fn operator_post_inc_9(
        &self,
        mut _a0: (),
    ) -> min_pointer_std_pair_const_int__double___std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_dec_10(
        &self,
    ) -> Ptr<min_pointer_std_pair_const_int__double___std_integral_constant_unsigned_long__0__>
    {
        unimplemented!()
    }
    fn operator_post_dec_11(
        &self,
        mut _a0: (),
    ) -> min_pointer_std_pair_const_int__double___std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_std_pair_const_int__double___std_integral_constant_unsigned_long__0__>
    {
        unimplemented!()
    }
    fn operator_sub_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_std_pair_const_int__double___std_integral_constant_unsigned_long__0__>
    {
        unimplemented!()
    }
    fn operator_add(
        &self,
        mut n: (),
    ) -> min_pointer_std_pair_const_int__double___std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_sub(
        &self,
        mut n: (),
    ) -> min_pointer_std_pair_const_int__double___std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_index(&self, mut n: ()) -> Ptr<()> {
        unimplemented!()
    }
}
impl min_pointer_std_pair_const_int__double___std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_std_pair_const_int__double___std_integral_constant_unsigned_long__0__>
{
}
pub trait min_pointer_void__std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_void__std_integral_constant_unsigned_long__0__>
{
}
pub fn __cpp2rust_init_globals() {
    let _ = outstanding_bytes_0.with(|_| ());
    let _ = alloc_count_1.with(|_| ());
    let _ = dealloc_count_2.with(|_| ());
    let _ = disable_default_constructor_3.with(|_| ());
}
