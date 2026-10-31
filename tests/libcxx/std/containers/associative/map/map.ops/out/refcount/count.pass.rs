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
    pub fn outstanding_alloc() -> usize {
        assert!((alloc_count_1.with(|rc| *rc.borrow()) >= dealloc_count_2.with(|rc| *rc.borrow())));
        return ((alloc_count_1.with(|rc| *rc.borrow()))
            .wrapping_sub(dealloc_count_2.with(|rc| *rc.borrow())));
    }
    pub fn reset() {
        assert!((({ malloc_allocator_base::outstanding_alloc() }) == 0_usize));
        disable_default_constructor_3.with(|rc| *rc.borrow_mut() = false);
        outstanding_bytes_0.with(|rc| *rc.borrow_mut() = 0_usize);
        alloc_count_1.with(|rc| *rc.borrow_mut() = 0_usize);
        dealloc_count_2.with(|rc| *rc.borrow_mut() = 0_usize);
    }
}
thread_local!(
    pub static outstanding_bytes_0: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static alloc_count_1: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static dealloc_count_2: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static disable_default_constructor_3: Value<bool> = Rc::new(RefCell::new(false));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_std_pair_const_int__double___std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<(Value<i32>, Value<f64>)>,
}
impl min_pointer_std_pair_const_int__double___std_integral_constant_unsigned_long__0__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: AnyPtr,
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_std_pair_const_int__double__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(i32, new, value)]
#[byte_size(4)]
pub struct PrivateConstructor {
    #[offset(0)]
    val: i32,
}
impl PrivateConstructor {
    pub fn make(mut v: i32) -> PrivateConstructor {
        return PrivateConstructor::new({ v });
    }
    fn new(mut v: i32) -> Self {
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
pub fn operator_lt_4(lhs: Ptr<PrivateConstructor>, rhs: Ptr<PrivateConstructor>) -> bool {
    return ({ ({ PrivateConstructorImpl::get(&lhs) }) } < {
        ({ PrivateConstructorImpl::get(&rhs) })
    });
}
pub fn operator_lt_5(lhs: Ptr<PrivateConstructor>, mut rhs: i32) -> bool {
    return ({ ({ PrivateConstructorImpl::get(&lhs) }) } < { rhs });
}
pub fn operator_lt_6(mut lhs: i32, rhs: Ptr<PrivateConstructor>) -> bool {
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
        { C2Int::new_2() }
    }
}
pub fn operator_lt_8(mut rhs: i32, lhs: Ptr<C2Int>) -> bool {
    return ({ rhs } < { ({ C2IntImpl::get(&lhs) }) });
}
pub fn operator_lt_7(rhs: Ptr<C2Int>, lhs: Ptr<C2Int>) -> bool {
    return ({ ({ C2IntImpl::get(&rhs) }) } < { ({ C2IntImpl::get(&lhs) }) });
}
pub fn operator_lt_9(rhs: Ptr<C2Int>, mut lhs: i32) -> bool {
    return ({ ({ C2IntImpl::get(&rhs) }) } < { lhs });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct FinalCompare_int_ {}
impl Callable2<Ptr<i32>, Ptr<i32>, bool> for FinalCompare_int_ {
    fn call(&self, a1: Ptr<i32>, a2: Ptr<i32>) -> bool {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            FinalCompare_int_Impl::operator_call(__it, a1, a2)
        })
    }
}
pub fn test_10() -> bool {
    let ar: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        (Rc::new(RefCell::new(5)), Rc::new(RefCell::new((5 as f64)))),
        (Rc::new(RefCell::new(6)), Rc::new(RefCell::new((6 as f64)))),
        (Rc::new(RefCell::new(7)), Rc::new(RefCell::new((7 as f64)))),
        (Rc::new(RefCell::new(8)), Rc::new(RefCell::new((8 as f64)))),
        (Rc::new(RefCell::new(9)), Rc::new(RefCell::new((9 as f64)))),
        (
            Rc::new(RefCell::new(10)),
            Rc::new(RefCell::new((10 as f64))),
        ),
        (
            Rc::new(RefCell::new(11)),
            Rc::new(RefCell::new((11 as f64))),
        ),
        (
            Rc::new(RefCell::new(12)),
            Rc::new(RefCell::new((12 as f64))),
        ),
    ])));
    let m: Value<Map<i32, (Value<i32>, Value<f64>)>> = Rc::new(RefCell::new(unimplemented!()));
    let mut i: i32 = 0;
    'loop_: while (i < 5) {
        let mut r: usize = unimplemented!();
        assert!((r == 0_usize));
        i.prefix_inc();
    }
    let mut i: i32 = 5;
    'loop_: while (i < 13) {
        let mut r: usize = unimplemented!();
        assert!((r == 1_usize));
        i.prefix_inc();
    }
    return true;
}
pub fn test_11() -> bool {
    let ar: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        (Rc::new(RefCell::new(5)), Rc::new(RefCell::new((5 as f64)))),
        (Rc::new(RefCell::new(6)), Rc::new(RefCell::new((6 as f64)))),
        (Rc::new(RefCell::new(7)), Rc::new(RefCell::new((7 as f64)))),
        (Rc::new(RefCell::new(8)), Rc::new(RefCell::new((8 as f64)))),
        (Rc::new(RefCell::new(9)), Rc::new(RefCell::new((9 as f64)))),
        (
            Rc::new(RefCell::new(10)),
            Rc::new(RefCell::new((10 as f64))),
        ),
        (
            Rc::new(RefCell::new(11)),
            Rc::new(RefCell::new((11 as f64))),
        ),
        (
            Rc::new(RefCell::new(12)),
            Rc::new(RefCell::new((12 as f64))),
        ),
    ])));
    let m: Value<Map<i32, (Value<i32>, Value<f64>)>> = Rc::new(RefCell::new(unimplemented!()));
    let mut i: i32 = 0;
    'loop_: while (i < 5) {
        let mut r: usize = unimplemented!();
        assert!((r == 0_usize));
        i.prefix_inc();
    }
    let mut i: i32 = 5;
    'loop_: while (i < 13) {
        let mut r: usize = unimplemented!();
        assert!((r == 1_usize));
        i.prefix_inc();
    }
    return true;
}
pub fn test_12() -> bool {
    let ar: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        (Rc::new(RefCell::new(5)), Rc::new(RefCell::new((5 as f64)))),
        (Rc::new(RefCell::new(6)), Rc::new(RefCell::new((6 as f64)))),
        (Rc::new(RefCell::new(7)), Rc::new(RefCell::new((7 as f64)))),
        (Rc::new(RefCell::new(8)), Rc::new(RefCell::new((8 as f64)))),
        (Rc::new(RefCell::new(9)), Rc::new(RefCell::new((9 as f64)))),
        (
            Rc::new(RefCell::new(10)),
            Rc::new(RefCell::new((10 as f64))),
        ),
        (
            Rc::new(RefCell::new(11)),
            Rc::new(RefCell::new((11 as f64))),
        ),
        (
            Rc::new(RefCell::new(12)),
            Rc::new(RefCell::new((12 as f64))),
        ),
    ])));
    let m: Value<Map<i32, (Value<i32>, Value<f64>)>> = Rc::new(RefCell::new(unimplemented!()));
    let mut i: i32 = 0;
    'loop_: while (i < 5) {
        let mut r: usize = unimplemented!();
        assert!((r == 0_usize));
        i.prefix_inc();
    }
    let mut i: i32 = 5;
    'loop_: while (i < 13) {
        let mut r: usize = unimplemented!();
        assert!((r == 1_usize));
        i.prefix_inc();
    }
    return true;
}
pub fn test_13() -> bool {
    let ar: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        (Rc::new(RefCell::new(5)), Rc::new(RefCell::new((5 as f64)))),
        (Rc::new(RefCell::new(6)), Rc::new(RefCell::new((6 as f64)))),
        (Rc::new(RefCell::new(7)), Rc::new(RefCell::new((7 as f64)))),
        (Rc::new(RefCell::new(8)), Rc::new(RefCell::new((8 as f64)))),
        (Rc::new(RefCell::new(9)), Rc::new(RefCell::new((9 as f64)))),
        (
            Rc::new(RefCell::new(10)),
            Rc::new(RefCell::new((10 as f64))),
        ),
        (
            Rc::new(RefCell::new(11)),
            Rc::new(RefCell::new((11 as f64))),
        ),
        (
            Rc::new(RefCell::new(12)),
            Rc::new(RefCell::new((12 as f64))),
        ),
    ])));
    let m: Value<Map<i32, (Value<i32>, Value<f64>)>> = Rc::new(RefCell::new(unimplemented!()));
    let mut i: i32 = 0;
    'loop_: while (i < 5) {
        let mut r: usize = unimplemented!();
        assert!((r == 0_usize));
        i.prefix_inc();
    }
    let mut i: i32 = 5;
    'loop_: while (i < 13) {
        let mut r: usize = unimplemented!();
        assert!((r == 1_usize));
        i.prefix_inc();
    }
    return true;
}
pub fn test_14() -> bool {
    let ar: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        (Rc::new(RefCell::new(5)), Rc::new(RefCell::new((5 as f64)))),
        (Rc::new(RefCell::new(6)), Rc::new(RefCell::new((6 as f64)))),
        (Rc::new(RefCell::new(7)), Rc::new(RefCell::new((7 as f64)))),
        (Rc::new(RefCell::new(8)), Rc::new(RefCell::new((8 as f64)))),
        (Rc::new(RefCell::new(9)), Rc::new(RefCell::new((9 as f64)))),
        (
            Rc::new(RefCell::new(10)),
            Rc::new(RefCell::new((10 as f64))),
        ),
        (
            Rc::new(RefCell::new(11)),
            Rc::new(RefCell::new((11 as f64))),
        ),
        (
            Rc::new(RefCell::new(12)),
            Rc::new(RefCell::new((12 as f64))),
        ),
    ])));
    let m: Value<Map<i32, (Value<i32>, Value<f64>)>> = Rc::new(RefCell::new(unimplemented!()));
    let mut i: i32 = 0;
    'loop_: while (i < 5) {
        let mut r: usize = unimplemented!();
        assert!((r == 0_usize));
        i.prefix_inc();
    }
    let mut i: i32 = 5;
    'loop_: while (i < 13) {
        let mut r: usize = unimplemented!();
        assert!((r == 1_usize));
        i.prefix_inc();
    }
    return true;
}
pub fn test_15() -> bool {
    ({ test_10() });
    ({ test_11() });
    ({ test_12() });
    ({ test_13() });
    ({ test_14() });
    {
        let m: Value<Map<PrivateConstructor, (Value<PrivateConstructor>, Value<f64>)>> =
            Rc::new(RefCell::new(Map::new()));
        Map::subscript_move(
            m.as_pointer(),
            Rc::new(RefCell::new(({ PrivateConstructor::make(5) }))).as_pointer(),
            |__k| cpp_move!(__k),
        )
        .write(5_f64);
        Map::subscript_move(
            m.as_pointer(),
            Rc::new(RefCell::new(({ PrivateConstructor::make(6) }))).as_pointer(),
            |__k| cpp_move!(__k),
        )
        .write(6_f64);
        Map::subscript_move(
            m.as_pointer(),
            Rc::new(RefCell::new(({ PrivateConstructor::make(7) }))).as_pointer(),
            |__k| cpp_move!(__k),
        )
        .write(7_f64);
        Map::subscript_move(
            m.as_pointer(),
            Rc::new(RefCell::new(({ PrivateConstructor::make(8) }))).as_pointer(),
            |__k| cpp_move!(__k),
        )
        .write(8_f64);
        Map::subscript_move(
            m.as_pointer(),
            Rc::new(RefCell::new(({ PrivateConstructor::make(9) }))).as_pointer(),
            |__k| cpp_move!(__k),
        )
        .write(9_f64);
        Map::subscript_move(
            m.as_pointer(),
            Rc::new(RefCell::new(({ PrivateConstructor::make(10) }))).as_pointer(),
            |__k| cpp_move!(__k),
        )
        .write(10_f64);
        Map::subscript_move(
            m.as_pointer(),
            Rc::new(RefCell::new(({ PrivateConstructor::make(11) }))).as_pointer(),
            |__k| cpp_move!(__k),
        )
        .write(11_f64);
        Map::subscript_move(
            m.as_pointer(),
            Rc::new(RefCell::new(({ PrivateConstructor::make(12) }))).as_pointer(),
            |__k| cpp_move!(__k),
        )
        .write(12_f64);
        let mut i: i32 = 0;
        'loop_: while (i < 5) {
            let mut r: usize = unimplemented!();
            assert!((r == 0_usize));
            i.prefix_inc();
        }
        let mut i: i32 = 5;
        'loop_: while (i < 13) {
            let mut r: usize = unimplemented!();
            assert!((r == 1_usize));
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> i32 {
    ({ test_15() });
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
pub trait FinalCompare_int_Impl {
    fn operator_call(&self, x: Ptr<i32>, y: Ptr<i32>) -> bool;
}
impl FinalCompare_int_Impl for Ptr<FinalCompare_int_> {
    fn operator_call(&self, x: Ptr<i32>, y: Ptr<i32>) -> bool {
        return ({ (x.read()) } < { (y.read()) });
    }
}
pub trait PrivateConstructorImpl {
    fn get(&self) -> i32;
}
impl PrivateConstructorImpl for Ptr<PrivateConstructor> {
    fn get(&self) -> i32 {
        return (*self).with(|__s| __s.val);
    }
}
pub trait min_allocator_std_pair_const_int__double__Impl {
    fn allocate(
        &self,
        mut n: usize,
    ) -> min_pointer_std_pair_const_int__double___std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn deallocate(
        &self,
        mut p: min_pointer_std_pair_const_int__double___std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        unimplemented!()
    }
}
impl min_allocator_std_pair_const_int__double__Impl
    for Ptr<min_allocator_std_pair_const_int__double__>
{
}
pub trait min_pointer_std_pair_const_int__double___std_integral_constant_unsigned_long__0__Impl {
    fn to_bool(&self) -> bool {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<(Value<i32>, Value<f64>)> {
        unimplemented!()
    }
    fn operator_arrow(&self) -> Ptr<(Value<i32>, Value<f64>)> {
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
        mut _a0: i32,
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
        mut _a0: i32,
    ) -> min_pointer_std_pair_const_int__double___std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        mut n: isize,
    ) -> Ptr<min_pointer_std_pair_const_int__double___std_integral_constant_unsigned_long__0__>
    {
        unimplemented!()
    }
    fn operator_sub_assign(
        &self,
        mut n: isize,
    ) -> Ptr<min_pointer_std_pair_const_int__double___std_integral_constant_unsigned_long__0__>
    {
        unimplemented!()
    }
    fn operator_add(
        &self,
        mut n: isize,
    ) -> min_pointer_std_pair_const_int__double___std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_sub(
        &self,
        mut n: isize,
    ) -> min_pointer_std_pair_const_int__double___std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_index(&self, mut n: isize) -> Ptr<(Value<i32>, Value<f64>)> {
        unimplemented!()
    }
}
impl min_pointer_std_pair_const_int__double___std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_std_pair_const_int__double___std_integral_constant_unsigned_long__0__>
{
}
pub trait min_pointer_void__std_integral_constant_unsigned_long__0__Impl {
    fn to_bool(&self) -> bool {
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
