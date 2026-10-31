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
pub struct nasty_vector_int_ {
    #[offset(0)]
    #[byte_size(24)]
    pub v_: Vec<i32>,
}
impl nasty_vector_int_ {
    pub fn new_1() -> Self {
        Self { v_: Vec::new() }
    }
}
impl Default for nasty_vector_int_ {
    fn default() -> Self {
        { nasty_vector_int_::new_1() }
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
pub fn test_0(c: Vec<i32>) -> bool {
    let c: Value<Vec<i32>> = Rc::new(RefCell::new(c));
    let i: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let r: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    assert!(((*c.borrow()).len() == 1_usize));
    assert!(((c.decay().to_last().read()) == 0));
    return true;
}
pub fn test_1(c: nasty_vector_int_) -> bool {
    let c: Value<nasty_vector_int_> = Rc::new(RefCell::new(c));
    let i: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let r: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    assert!((({ nasty_vector_int_Impl::size(&c.as_pointer(),) }) == 1_usize));
    assert!(((({ nasty_vector_int_Impl::back_36(&c.as_pointer(),) }).read()) == 0));
    return true;
}
pub fn test_2(c: ConstexprFixedCapacityDeque_int__10_) -> bool {
    let c: Value<ConstexprFixedCapacityDeque_int__10_> = Rc::new(RefCell::new(c));
    let i: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let r: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    assert!((({ ConstexprFixedCapacityDeque_int__10_Impl::size(&c.as_pointer(),) }) == 1_usize));
    assert!(((({ ConstexprFixedCapacityDeque_int__10_Impl::back(&c.as_pointer(),) }).read()) == 0));
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
    ({ test_0(Vec::new()) });
    ({ test_1(nasty_vector_int_::new_1()) });
    ({ test_2(<ConstexprFixedCapacityDeque_int__10_>::default()) });
    const _: () = assert!(true, "test(ConstexprFixedCapacityDeque<int, 10>())");;
    return 0;
}
pub trait ConstexprFixedCapacityDeque_int__10_Impl {
    fn begin_2(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn end_3(&self) -> Ptr<i32>;
    fn begin_4(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn end_5(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn size(&self) -> usize;
    fn front(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn back(&self) -> Ptr<i32>;
    fn insert(&self, pos: Ptr<i32>, t: i32) -> Ptr<i32>;
    fn push_back(&self, t: i32);
    fn push_front(&self, mut t: i32) {
        unimplemented!()
    }
}
impl ConstexprFixedCapacityDeque_int__10_Impl for Ptr<ConstexprFixedCapacityDeque_int__10_> {
    fn end_3(&self) -> Ptr<i32> {
        return (array_field_ptr!((*self), data_) as Ptr<i32>)
            .offset(((*self).with(|__s| __s.size_)) as isize);
    }
    fn size(&self) -> usize {
        return ((*self).with(|__s| __s.size_) as usize);
    }
    fn back(&self) -> Ptr<i32> {
        assert!(((*self).with(|__s| __s.size_) >= 1));
        return (array_field_ptr!((*self), data_) as Ptr<i32>)
            .offset(((*self).with(|__s| __s.size_) - 1) as isize);
    }
    fn insert(&self, mut pos: Ptr<i32>, mut t: i32) -> Ptr<i32> {
        let mut i: i32 =
            ((((pos).clone() - (array_field_ptr!((*self), data_) as Ptr<i32>)) as i64) as i32);
        if (i != (*self).with(|__s| __s.size_)) {
            unimplemented!();
        }
        array_elem!((*self), data_, i).write(t);
        {
            field!((*self), size_).with_mut(|__v| *__v = *__v + 1)
        };
        return (array_field_ptr!((*self), data_) as Ptr<i32>).offset((i) as isize);
    }
    fn push_back(&self, mut t: i32) {
        ({
            let _pos: Ptr<i32> = ({ ConstexprFixedCapacityDeque_int__10_Impl::end_3(self) });
            ConstexprFixedCapacityDeque_int__10_Impl::insert(self, _pos, t)
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
pub trait nasty_vector_int_Impl {
    fn destructor(&self);
    fn assign_10(&self, mut n: usize, u: Ptr<i32>) {
        unimplemented!()
    }
    fn assign_11(&self, il: Vec<i32>) {
        unimplemented!()
    }
    fn begin_12(&self) -> RandomAccessIterator<i32> {
        unimplemented!()
    }
    fn begin_13(&self) -> RandomAccessIterator<i32> {
        unimplemented!()
    }
    fn end_14(&self) -> RandomAccessIterator<i32> {
        unimplemented!()
    }
    fn end_15(&self) -> RandomAccessIterator<i32> {
        unimplemented!()
    }
    fn rbegin_16(&self) -> () {
        unimplemented!()
    }
    fn rbegin_17(&self) -> () {
        unimplemented!()
    }
    fn rend_18(&self) -> () {
        unimplemented!()
    }
    fn rend_19(&self) -> () {
        unimplemented!()
    }
    fn cbegin(&self) -> RandomAccessIterator<i32> {
        unimplemented!()
    }
    fn cend(&self) -> RandomAccessIterator<i32> {
        unimplemented!()
    }
    fn crbegin(&self) -> () {
        unimplemented!()
    }
    fn crend(&self) -> () {
        unimplemented!()
    }
    fn size(&self) -> usize;
    fn max_size(&self) -> usize {
        unimplemented!()
    }
    fn capacity(&self) -> usize {
        unimplemented!()
    }
    fn empty(&self) -> bool {
        unimplemented!()
    }
    fn reserve(&self, mut n: usize) {
        unimplemented!()
    }
    fn shrink_to_fit(&self) {
        unimplemented!()
    }
    fn operator_index_30(&self, mut n: usize) -> Ptr<i32> {
        unimplemented!()
    }
    fn operator_index_31(&self, mut n: usize) -> Ptr<i32> {
        unimplemented!()
    }
    fn at_32(&self, mut n: usize) -> Ptr<i32> {
        unimplemented!()
    }
    fn at_33(&self, mut n: usize) -> Ptr<i32> {
        unimplemented!()
    }
    fn front_34(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn front_35(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn back_36(&self) -> Ptr<i32>;
    fn back_37(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn data_38(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn data_39(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn push_back_40(&self, x: Ptr<i32>) {
        unimplemented!()
    }
    fn push_back_41(&self, x: Ptr<i32>);
    fn pop_back(&self) {
        unimplemented!()
    }
    fn insert_45(&self, pos: RandomAccessIterator<i32>, x: Ptr<i32>) -> RandomAccessIterator<i32> {
        unimplemented!()
    }
    fn insert_46(&self, pos: RandomAccessIterator<i32>, x: Ptr<i32>) -> RandomAccessIterator<i32> {
        unimplemented!()
    }
    fn insert_47(
        &self,
        pos: RandomAccessIterator<i32>,
        mut n: usize,
        x: Ptr<i32>,
    ) -> RandomAccessIterator<i32> {
        unimplemented!()
    }
    fn insert_49(&self, pos: RandomAccessIterator<i32>, il: Vec<i32>) -> RandomAccessIterator<i32> {
        unimplemented!()
    }
    fn erase_50(&self, pos: RandomAccessIterator<i32>) -> RandomAccessIterator<i32> {
        unimplemented!()
    }
    fn erase_51(
        &self,
        first: RandomAccessIterator<i32>,
        last: RandomAccessIterator<i32>,
    ) -> RandomAccessIterator<i32> {
        unimplemented!()
    }
    fn clear(&self) {
        unimplemented!()
    }
    fn resize_53(&self, mut sz: usize) {
        unimplemented!()
    }
    fn resize_54(&self, mut sz: usize, c: Ptr<i32>) {
        unimplemented!()
    }
    fn swap(&self, nv: Ptr<nasty_vector_int_>) {
        unimplemented!()
    }
    fn operator_addr_56(&self) -> Ptr<nasty_vector_int_> {
        unimplemented!()
    }
    fn operator_addr_57(&self) -> Ptr<nasty_vector_int_> {
        unimplemented!()
    }
}
impl nasty_vector_int_Impl for Ptr<nasty_vector_int_> {
    fn destructor(&self) {}
    fn size(&self) -> usize {
        return (*(*self).upgrade().deref()).v_.len();
    }
    fn back_36(&self) -> Ptr<i32> {
        return field_ref!((*self), v_).decay().to_last();
    }
    fn push_back_41(&self, x: Ptr<i32>) {
        {
            let __a1 = x.with_mut(std::mem::take);
            field!((*self), v_).with_mut(|__v: &mut Vec<i32>| __v.push(__a1))
        };
    }
}
pub fn __cpp2rust_init_globals() {}
