extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor)]
#[byte_size(24)]
pub struct nasty_list_Copyable_ {
    #[offset(0)]
    #[byte_size(24)]
    pub l_: std_list_Copyable__std_allocator_Copyable__,
}
impl nasty_list_Copyable_ {
    pub fn new_1() -> Self {
        Self {
            l_: <std_list_Copyable__std_allocator_Copyable__>::default(),
        }
    }
}
impl CppMove for nasty_list_Copyable_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for nasty_list_Copyable_ {
    fn default() -> Self {
        { nasty_list_Copyable_::new_1() }
    }
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CopyAssign, Destructor)]
#[byte_size(1)]
pub struct nasty_mutex {}
impl nasty_mutex {
    pub fn new() -> Self {
        Self {}
    }
    fn copy_from(_a0: Ptr<nasty_mutex>) -> Self {
        let __this: nasty_mutex = Self {};
        assert!(false);
        __this
    }
}
impl Clone for nasty_mutex {
    fn clone(&self) -> Self {
        let __src: Value<nasty_mutex> = Rc::new(RefCell::new(nasty_mutex {}));
        nasty_mutex::copy_from(__src.as_pointer())
    }
}
impl CppMove for nasty_mutex {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for nasty_mutex {
    fn default() -> Self {
        { nasty_mutex::new() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(44)]
pub struct ConstexprFixedCapacityDeque_int__10_ {
    #[offset(0)]
    #[byte_size(40)]
    data_: Box<[i32]>,
    #[offset(40)]
    size_: i32,
}
impl CppMove for ConstexprFixedCapacityDeque_int__10_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for ConstexprFixedCapacityDeque_int__10_ {
    fn default() -> Self {
        {
            ConstexprFixedCapacityDeque_int__10_ {
                data_: (0..10).map(|_| 0_i32).collect::<Box<[i32]>>(),
                size_: 0,
            }
        }
    }
}
pub fn test_1(c: nasty_list_Copyable_) -> bool {
    let c: Value<nasty_list_Copyable_> = Rc::new(RefCell::new(c));
    let _dtor_c = ScopedDestructor::new(&c, |__p| __p.destructor());
    let v: Value<Copyable> = Rc::new(RefCell::new(Copyable::new()));
    let _dtor_v = ScopedDestructor::new(&v, |__p| __p.destructor());
    let i: Value<std_front_insert_iterator_nasty_list_Copyable__> = Rc::new(RefCell::new(
        std_front_insert_iterator_nasty_list_Copyable__::new({ c.as_pointer() }),
    ));
    (*i.borrow_mut()) = (*v.borrow()).clone();
    assert!(
        ({
            let _x: Ptr<Copyable> = ({ nasty_list_Copyable_Impl::front_25(&c.as_pointer()) });
            operator_eq_2(_x, v.as_pointer())
        })
    );
    return true;
}
pub fn test_3(c: ConstexprFixedCapacityDeque_int__10_) -> bool {
    let c: Value<ConstexprFixedCapacityDeque_int__10_> = Rc::new(RefCell::new(c));
    let v: Value<i32> = Rc::new(RefCell::new(0_i32));
    let i: Value<std_front_insert_iterator_ConstexprFixedCapacityDeque_int__10__> =
        Rc::new(RefCell::new(
            std_front_insert_iterator_ConstexprFixedCapacityDeque_int__10__::new({
                c.as_pointer()
            }),
        ));
    (*i.borrow_mut()) = (*v.borrow());
    assert!(
        ({
            ((({ ConstexprFixedCapacityDeque_int__10_Impl::front(&c.as_pointer()) }).read())
                == (*v.borrow()))
        })
    );
    return true;
}
pub fn operator_eq_2(x: Ptr<Copyable>, y: Ptr<Copyable>) -> bool {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor)]
#[byte_size(4)]
pub struct Copyable {
    #[offset(0)]
    data_: i32,
}
impl Copyable {
    pub fn new() -> Self {
        Self { data_: 0 }
    }
}
impl CppEq for Copyable {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_2(__lhs.clone(), __rhs.clone())
    }
}
impl CppMove for Copyable {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for Copyable {
    fn default() -> Self {
        { Copyable::new() }
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
    ({ test_4(<std_list_Copyable__std_allocator_Copyable__>::default()) });
    ({ test_1(nasty_list_Copyable_::new_1()) });
    ({ test_3(<ConstexprFixedCapacityDeque_int__10_>::default()) });
    const _: () = assert!(true, "test(ConstexprFixedCapacityDeque<int, 10>())");;
    return 0;
}
pub trait ConstexprFixedCapacityDeque_int__10_Impl {
    fn begin_2(&self) -> Ptr<i32>;
    fn end_3(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn begin_4(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn end_5(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn size(&self) -> usize {
        unimplemented!()
    }
    fn front(&self) -> Ptr<i32>;
    fn back(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn insert(&self, pos: Ptr<i32>, t: i32) -> Ptr<i32>;
    fn push_back(&self, mut t: i32) {
        unimplemented!()
    }
    fn push_front(&self, t: i32);
}
impl ConstexprFixedCapacityDeque_int__10_Impl for Ptr<ConstexprFixedCapacityDeque_int__10_> {
    fn begin_2(&self) -> Ptr<i32> {
        return (array_field_ptr!((*self), data_) as Ptr<i32>);
    }
    fn front(&self) -> Ptr<i32> {
        assert!(((*self).with(|__s| __s.size_) >= 1));
        return (array_field_ptr!((*self), data_) as Ptr<i32>).offset((0) as isize);
    }
    fn insert(&self, mut pos: Ptr<i32>, mut t: i32) -> Ptr<i32> {
        let mut i: i32 =
            ((((pos).clone() - (array_field_ptr!((*self), data_) as Ptr<i32>)) as i64) as i32);
        if (i != (*self).with(|__s| __s.size_)) {
            ({
                let ___first: Ptr<i32> =
                    (array_field_ptr!((*self), data_) as Ptr<i32>).offset((i) as isize);
                let ___last: Ptr<i32> = (array_field_ptr!((*self), data_) as Ptr<i32>)
                    .offset(((*self).with(|__s| __s.size_)) as isize);
                let ___result: Ptr<i32> = (array_field_ptr!((*self), data_) as Ptr<i32>)
                    .offset(((*self).with(|__s| __s.size_)) as isize)
                    .offset((1) as isize);
                move_backward_0(___first, ___last, ___result)
            });
        }
        array_elem!((*self), data_, i).write(t);
        {
            field!((*self), size_).with_mut(|__v| *__v = *__v + 1)
        };
        return (array_field_ptr!((*self), data_) as Ptr<i32>).offset((i) as isize);
    }
    fn push_front(&self, mut t: i32) {
        ({
            let _pos: Ptr<i32> = ({ ConstexprFixedCapacityDeque_int__10_Impl::begin_2(self) });
            ConstexprFixedCapacityDeque_int__10_Impl::insert(self, _pos, t)
        });
    }
}
pub trait CopyableImpl {
    fn destructor(&self);
}
impl CopyableImpl for Ptr<Copyable> {
    fn destructor(&self) {
        field!((*self), data_).write(-1_i32);
    }
}
pub trait nasty_list_Copyable_Impl {
    fn destructor(&self);
    fn operator_assign_9(&self, il: Vec<Copyable>) -> Ptr<nasty_list_Copyable_> {
        unimplemented!()
    }
    fn assign_11(&self, mut n: u64, t: Ptr<Copyable>) {
        unimplemented!()
    }
    fn assign_12(&self, il: Vec<Copyable>) {
        unimplemented!()
    }
    fn begin_13(&self) -> BidirectionalIterator<Copyable> {
        unimplemented!()
    }
    fn begin_14(&self) -> BidirectionalIterator<Copyable> {
        unimplemented!()
    }
    fn end_15(&self) -> BidirectionalIterator<Copyable> {
        unimplemented!()
    }
    fn end_16(&self) -> BidirectionalIterator<Copyable> {
        unimplemented!()
    }
    fn rbegin_17(&self) -> BidirectionalIterator<Copyable> {
        unimplemented!()
    }
    fn rbegin_18(&self) -> BidirectionalIterator<Copyable> {
        unimplemented!()
    }
    fn rend_19(&self) -> BidirectionalIterator<Copyable> {
        unimplemented!()
    }
    fn rend_20(&self) -> BidirectionalIterator<Copyable> {
        unimplemented!()
    }
    fn cbegin(&self) -> BidirectionalIterator<Copyable> {
        unimplemented!()
    }
    fn cend(&self) -> BidirectionalIterator<Copyable> {
        unimplemented!()
    }
    fn crbegin(&self) -> BidirectionalIterator<Copyable> {
        unimplemented!()
    }
    fn crend(&self) -> BidirectionalIterator<Copyable> {
        unimplemented!()
    }
    fn front_25(&self) -> Ptr<Copyable>;
    fn front_26(&self) -> Ptr<Copyable> {
        unimplemented!()
    }
    fn back_27(&self) -> Ptr<Copyable> {
        unimplemented!()
    }
    fn back_28(&self) -> Ptr<Copyable> {
        unimplemented!()
    }
    fn size(&self) -> u64 {
        unimplemented!()
    }
    fn max_size(&self) -> u64 {
        unimplemented!()
    }
    fn empty(&self) -> bool {
        unimplemented!()
    }
    fn push_front_32(&self, x: Ptr<Copyable>);
    fn push_back_33(&self, x: Ptr<Copyable>) {
        unimplemented!()
    }
    fn push_back_34(&self, x: Ptr<Copyable>) {
        unimplemented!()
    }
    fn push_front_35(&self, x: Ptr<Copyable>) {
        unimplemented!()
    }
    fn pop_front(&self) {
        unimplemented!()
    }
    fn pop_back(&self) {
        unimplemented!()
    }
    fn insert_41(
        &self,
        pos: BidirectionalIterator<Copyable>,
        x: Ptr<Copyable>,
    ) -> BidirectionalIterator<Copyable> {
        unimplemented!()
    }
    fn insert_42(
        &self,
        pos: BidirectionalIterator<Copyable>,
        x: Ptr<Copyable>,
    ) -> BidirectionalIterator<Copyable> {
        unimplemented!()
    }
    fn insert_43(
        &self,
        pos: BidirectionalIterator<Copyable>,
        mut n: u64,
        x: Ptr<Copyable>,
    ) -> BidirectionalIterator<Copyable> {
        unimplemented!()
    }
    fn insert_45(
        &self,
        pos: BidirectionalIterator<Copyable>,
        il: Vec<Copyable>,
    ) -> BidirectionalIterator<Copyable> {
        unimplemented!()
    }
    fn erase_46(&self, pos: BidirectionalIterator<Copyable>) -> BidirectionalIterator<Copyable> {
        unimplemented!()
    }
    fn erase_47(
        &self,
        first: BidirectionalIterator<Copyable>,
        last: BidirectionalIterator<Copyable>,
    ) -> BidirectionalIterator<Copyable> {
        unimplemented!()
    }
    fn resize_48(&self, mut n: u64) {
        unimplemented!()
    }
    fn resize_49(&self, mut n: u64, c: Ptr<Copyable>) {
        unimplemented!()
    }
    fn swap(&self, nl: Ptr<nasty_list_Copyable_>) {
        unimplemented!()
    }
    fn clear(&self) {
        unimplemented!()
    }
    fn operator_addr_52(&self) -> Ptr<nasty_list_Copyable_> {
        unimplemented!()
    }
    fn operator_addr_53(&self) -> Ptr<nasty_list_Copyable_> {
        unimplemented!()
    }
}
impl nasty_list_Copyable_Impl for Ptr<nasty_list_Copyable_> {
    fn destructor(&self) {}
    fn front_25(&self) -> Ptr<Copyable> {
        return ({ (*(*self).upgrade().deref()).l_.front_41() });
    }
    fn push_front_32(&self, x: Ptr<Copyable>) {
        ({
            let ___x: Ptr<Copyable> = (x).clone();
            (*(*self).upgrade().deref()).l_.push_front_45(___x)
        });
    }
}
pub trait nasty_mutexImpl {
    fn destructor(&self);
    fn operator_addr(&self) -> Ptr<nasty_mutex>;
    fn copy_assign(&self, _a0: Ptr<nasty_mutex>) -> Ptr<nasty_mutex>;
    fn lock(&self);
    fn try_lock(&self) -> bool;
    fn unlock(&self);
    fn lock_shared(&self);
    fn try_lock_shared(&self) -> bool;
    fn unlock_shared(&self);
}
impl nasty_mutexImpl for Ptr<nasty_mutex> {
    fn destructor(&self) {}
    fn operator_addr(&self) -> Ptr<nasty_mutex> {
        assert!(false);
        return Ptr::<nasty_mutex>::null();
    }
    fn copy_assign(&self, _a0: Ptr<nasty_mutex>) -> Ptr<nasty_mutex> {
        assert!(false);
        return (*self).clone();
    }
    fn lock(&self) {}
    fn try_lock(&self) -> bool {
        return true;
    }
    fn unlock(&self) {}
    fn lock_shared(&self) {}
    fn try_lock_shared(&self) -> bool {
        return true;
    }
    fn unlock_shared(&self) {}
}
pub fn __cpp2rust_init_globals() {}
