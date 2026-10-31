extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign)]
#[byte_size(16)]
pub struct Emplaceable {
    #[offset(0)]
    int_: i32,
    #[offset(8)]
    double_: f64,
}
impl Emplaceable {
    pub fn new_1() -> Self {
        Self {
            int_: 0,
            double_: 0_f64,
        }
    }
    pub fn new_2(mut i: i32, mut d: f64) -> Self {
        Self {
            int_: i,
            double_: d,
        }
    }
    pub fn move_from(x: Ptr<Emplaceable>) -> Self {
        let __this: Emplaceable = Self {
            int_: x.with(|__s| __s.int_),
            double_: x.with(|__s| __s.double_),
        };
        field!(x, int_).write(0);
        field!(x, double_).write(0_f64);
        __this
    }
}
impl CppLess for Emplaceable {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        EmplaceableImpl::operator_lt(__lhs, __rhs.clone())
    }
}
impl CppEq for Emplaceable {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        EmplaceableImpl::operator_eq(__lhs, __rhs.clone())
    }
}
impl Default for Emplaceable {
    fn default() -> Self {
        { Emplaceable::new_1() }
    }
}
pub fn operator_eq_1(x: Ptr<DefaultOnly>, y: Ptr<DefaultOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
pub fn operator_lt_2(x: Ptr<DefaultOnly>, y: Ptr<DefaultOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } < { y.with(|__s| __s.data_) });
}
#[derive(Record, ByteRepr, FnPtrArg, Destructor)]
#[byte_size(4)]
pub struct DefaultOnly {
    #[offset(0)]
    data_: i32,
}
impl DefaultOnly {
    pub fn new() -> Self {
        let __this: DefaultOnly = Self { data_: -1_i32 };
        (*count_0.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
}
impl CppLess for DefaultOnly {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_lt_2(__lhs.clone(), __rhs.clone())
    }
}
impl CppEq for DefaultOnly {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_1(__lhs.clone(), __rhs.clone())
    }
}
impl Default for DefaultOnly {
    fn default() -> Self {
        { DefaultOnly::new() }
    }
}
thread_local!(
    pub static count_0: Value<i32> = Rc::new(RefCell::new(0));
);
pub fn operator_eq_3(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
pub fn operator_ne_4(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } != { y.with(|__s| __s.data_) });
}
pub fn operator_lt_5(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } < { y.with(|__s| __s.data_) });
}
pub fn operator_le_6(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } <= { y.with(|__s| __s.data_) });
}
pub fn operator_gt_7(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } > { y.with(|__s| __s.data_) });
}
pub fn operator_ge_8(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } >= { y.with(|__s| __s.data_) });
}
pub fn operator_cmp_9(_a0: Ptr<MoveOnly>, _a1: Ptr<MoveOnly>) -> std::cmp::Ordering {
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
        operator_lt_5(__lhs.clone(), __rhs.clone())
    }
}
impl CppEq for MoveOnly {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_3(__lhs.clone(), __rhs.clone())
    }
}
impl Default for MoveOnly {
    fn default() -> Self {
        { MoveOnly::new(None) }
    }
}
pub fn operator_eq_10(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
pub fn operator_ne_11(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } != { y.with(|__s| __s.data_) });
}
pub fn operator_lt_12(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } < { y.with(|__s| __s.data_) });
}
pub fn operator_le_13(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } <= { y.with(|__s| __s.data_) });
}
pub fn operator_gt_14(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } > { y.with(|__s| __s.data_) });
}
pub fn operator_ge_15(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } >= { y.with(|__s| __s.data_) });
}
pub fn operator_cmp_16(_a0: Ptr<TrivialMoveOnly>, _a1: Ptr<TrivialMoveOnly>) -> std::cmp::Ordering {
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
        operator_lt_12(__lhs.clone(), __rhs.clone())
    }
}
impl CppEq for TrivialMoveOnly {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_10(__lhs.clone(), __rhs.clone())
    }
}
impl Default for TrivialMoveOnly {
    fn default() -> Self {
        { TrivialMoveOnly::new(None) }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub fn outstanding_alloc() -> usize {
        assert!(
            (alloc_count_18.with(|rc| *rc.borrow()) >= dealloc_count_19.with(|rc| *rc.borrow()))
        );
        return ((alloc_count_18.with(|rc| *rc.borrow()))
            .wrapping_sub(dealloc_count_19.with(|rc| *rc.borrow())));
    }
    pub fn reset() {
        assert!((({ malloc_allocator_base::outstanding_alloc() }) == 0_usize));
        disable_default_constructor_20.with(|rc| *rc.borrow_mut() = false);
        outstanding_bytes_17.with(|rc| *rc.borrow_mut() = 0_usize);
        alloc_count_18.with(|rc| *rc.borrow_mut() = 0_usize);
        dealloc_count_19.with(|rc| *rc.borrow_mut() = 0_usize);
    }
}
thread_local!(
    pub static outstanding_bytes_17: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static alloc_count_18: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static dealloc_count_19: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static disable_default_constructor_20: Value<bool> = Rc::new(RefCell::new(false));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_int__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<i32>,
}
impl min_pointer_int__std_integral_constant_unsigned_long__0__ {}
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
pub struct min_allocator_int_ {}
pub fn test_21() -> bool {
    {
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!((count_0.with(|rc| *rc.borrow()) == 0));
        let r: Value<(Value<BidirectionalIterator<DefaultOnly>>, Value<bool>)> =
            Rc::new(RefCell::new(unimplemented!()));
        assert!(((*(*r.borrow()).1.borrow()) as bool));
        assert!((*(*r.borrow()).0.borrow()) == unimplemented!());
        assert!((unimplemented!() == 1_()));
        assert!(
            ({
                let _x: Ptr<DefaultOnly> = unimplemented!().iter_deref();
                let _y: Value<DefaultOnly> = Rc::new(RefCell::new(DefaultOnly::new()));
                operator_eq_1(_x, _y.as_pointer())
            })
        );
        assert!((count_0.with(|rc| *rc.borrow()) == 1));
        r.as_pointer().write(unimplemented!());
        assert!(!(*(*r.borrow()).1.borrow()));
        assert!((*(*r.borrow()).0.borrow()) == unimplemented!());
        assert!((unimplemented!() == 1_()));
        assert!(
            ({
                let _x: Ptr<DefaultOnly> = unimplemented!().iter_deref();
                let _y: Value<DefaultOnly> = Rc::new(RefCell::new(DefaultOnly::new()));
                operator_eq_1(_x, _y.as_pointer())
            })
        );
        assert!((count_0.with(|rc| *rc.borrow()) == 1));
    }
    {
        assert!((count_0.with(|rc| *rc.borrow()) == 0));
    }
    {
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let r: Value<(Value<BidirectionalIterator<Emplaceable>>, Value<bool>)> =
            Rc::new(RefCell::new(unimplemented!()));
        assert!(((*(*r.borrow()).1.borrow()) as bool));
        assert!((*(*r.borrow()).0.borrow()) == unimplemented!());
        assert!((unimplemented!() == 1_()));
        assert!(
            ({
                let _x: Value<Emplaceable> = Rc::new(RefCell::new(Emplaceable::new_1()));
                EmplaceableImpl::operator_eq(&unimplemented!().iter_deref(), _x.as_pointer())
            })
        );
        r.as_pointer().write(unimplemented!());
        assert!(((*(*r.borrow()).1.borrow()) as bool));
        assert!((*(*r.borrow()).0.borrow()) == libcc2rs::next_refcount(unimplemented!(), 1_isize));
        assert!((unimplemented!() == 2_()));
        assert!(
            ({
                let _x: Value<Emplaceable> =
                    Rc::new(RefCell::new(Emplaceable::new_2({ 2 }, { 3.5_f64 })));
                EmplaceableImpl::operator_eq(
                    &(*(*r.borrow()).0.borrow()).iter_deref(),
                    _x.as_pointer(),
                )
            })
        );
        r.as_pointer().write(unimplemented!());
        assert!(!(*(*r.borrow()).1.borrow()));
        assert!((*(*r.borrow()).0.borrow()) == libcc2rs::next_refcount(unimplemented!(), 1_isize));
        assert!((unimplemented!() == 2_()));
        assert!(
            ({
                let _x: Value<Emplaceable> =
                    Rc::new(RefCell::new(Emplaceable::new_2({ 2 }, { 3.5_f64 })));
                EmplaceableImpl::operator_eq(
                    &(*(*r.borrow()).0.borrow()).iter_deref(),
                    _x.as_pointer(),
                )
            })
        );
    }
    {
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let r: Value<(Value<BidirectionalIterator<i32>>, Value<bool>)> =
            Rc::new(RefCell::new(unimplemented!()));
        assert!(((*(*r.borrow()).1.borrow()) as bool));
        assert!((*(*r.borrow()).0.borrow()) == unimplemented!());
        assert!((unimplemented!() == 1_()));
        assert!((((*(*r.borrow()).0.borrow()).iter_deref().read()) == 2));
    }
    {
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let r: Value<(Value<BidirectionalIterator<i32>>, Value<bool>)> =
            Rc::new(RefCell::new(unimplemented!()));
        assert!(((*(*r.borrow()).1.borrow()) as bool));
        assert!((*(*r.borrow()).0.borrow()) == unimplemented!());
        assert!((unimplemented!() == 1_()));
        assert!((((*(*r.borrow()).0.borrow()).iter_deref().read()) == 2));
    }
    {
        let set: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let res: Value<(
            Value<BidirectionalIterator<(Value<MoveOnly>, Value<MoveOnly>)>>,
            Value<bool>,
        )> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            ((<(
                Value<BidirectionalIterator<(Value<MoveOnly>, Value<MoveOnly>)>>,
                Value<bool>
            ) as PairGetRef<1>>::pair_get(&(*res.borrow()))
            .read()) as bool)
        );
        assert!(
            unimplemented!()
                == (*<(
                    Value<BidirectionalIterator<(Value<MoveOnly>, Value<MoveOnly>)>>,
                    Value<bool>
                ) as PairGetRef<0>>::pair_get(&(*res.borrow()))
                .upgrade()
                .deref())
        );
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
    ({ test_21() });
    return 0;
}
pub trait DefaultOnlyImpl {
    fn copy_assign(&self, _a0: Ptr<DefaultOnly>) -> Ptr<DefaultOnly> {
        unimplemented!()
    }
    fn destructor(&self);
}
impl DefaultOnlyImpl for Ptr<DefaultOnly> {
    fn destructor(&self) {
        field!((*self), data_).write(0);
        (*count_0.with(Value::clone).borrow_mut()).prefix_dec();
    }
}
pub trait EmplaceableImpl {
    fn copy_assign(&self, _a0: Ptr<Emplaceable>) -> Ptr<Emplaceable> {
        unimplemented!()
    }
    fn move_assign(&self, x: Ptr<Emplaceable>) -> Ptr<Emplaceable>;
    fn operator_eq(&self, x: Ptr<Emplaceable>) -> bool;
    fn operator_lt(&self, x: Ptr<Emplaceable>) -> bool;
    fn get(&self) -> i32;
}
impl EmplaceableImpl for Ptr<Emplaceable> {
    fn move_assign(&self, x: Ptr<Emplaceable>) -> Ptr<Emplaceable> {
        field!((*self), int_).write({ x.with(|__s| __s.int_) });
        field!(x, int_).write(0);
        field!((*self), double_).write({ x.with(|__s| __s.double_) });
        field!(x, double_).write(0_f64);
        return (*self).clone();
    }
    fn operator_eq(&self, x: Ptr<Emplaceable>) -> bool {
        return ({ (*self).with(|__s| __s.int_) } == { x.with(|__s| __s.int_) })
            && ({ (*self).with(|__s| __s.double_) } == { x.with(|__s| __s.double_) });
    }
    fn operator_lt(&self, x: Ptr<Emplaceable>) -> bool {
        return ({ (*self).with(|__s| __s.int_) } < { x.with(|__s| __s.int_) })
            || (({ (*self).with(|__s| __s.int_) } == { x.with(|__s| __s.int_) })
                && ({ (*self).with(|__s| __s.double_) } < { x.with(|__s| __s.double_) }));
    }
    fn get(&self) -> i32 {
        return (*self).with(|__s| __s.int_);
    }
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
pub trait min_allocator_int_Impl {
    fn allocate(&self, mut n: usize) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn deallocate(
        &self,
        mut p: min_pointer_int__std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        unimplemented!()
    }
}
impl min_allocator_int_Impl for Ptr<min_allocator_int_> {}
pub trait min_pointer_int__std_integral_constant_unsigned_long__0__Impl {
    fn to_bool(&self) -> bool {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn operator_arrow(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn operator_inc_8(&self) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_inc_9(
        &self,
        mut _a0: i32,
    ) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_dec_10(&self) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_dec_11(
        &self,
        mut _a0: i32,
    ) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        mut n: isize,
    ) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_sub_assign(
        &self,
        mut n: isize,
    ) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_add(
        &self,
        mut n: isize,
    ) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_sub(
        &self,
        mut n: isize,
    ) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_index(&self, mut n: isize) -> Ptr<i32> {
        unimplemented!()
    }
}
impl min_pointer_int__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__>
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
    let _ = count_0.with(|_| ());
    let _ = outstanding_bytes_17.with(|_| ());
    let _ = alloc_count_18.with(|_| ());
    let _ = dealloc_count_19.with(|_| ());
    let _ = disable_default_constructor_20.with(|_| ());
}
