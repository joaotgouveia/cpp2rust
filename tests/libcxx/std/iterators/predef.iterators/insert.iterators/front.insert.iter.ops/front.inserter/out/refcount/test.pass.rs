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
pub struct nasty_list_int_ {
    #[offset(0)]
    #[byte_size(24)]
    pub l_: std_list_int__std_allocator_int__,
}
impl nasty_list_int_ {
    pub fn new_1() -> Self {
        Self {
            l_: <std_list_int__std_allocator_int__>::default(),
        }
    }
}
impl CppMove for nasty_list_int_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for nasty_list_int_ {
    fn default() -> Self {
        { nasty_list_int_::new_1() }
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
pub fn test_1(c: nasty_list_int_) -> bool {
    let c: Value<nasty_list_int_> = Rc::new(RefCell::new(c));
    let _dtor_c = ScopedDestructor::new(&c, |__p| __p.destructor());
    let i: Value<std_front_insert_iterator_nasty_list_int__> =
        Rc::new(RefCell::new(({ front_inserter_2(c.as_pointer()) })));
    (*i.borrow_mut()) = 3;
    assert!((({ nasty_list_int_Impl::size(&c.as_pointer(),) }) == 1_u64));
    assert!(((({ nasty_list_int_Impl::front_25(&c.as_pointer(),) }).read()) == 3));
    (*i.borrow_mut()) = 4;
    assert!((({ nasty_list_int_Impl::size(&c.as_pointer(),) }) == 2_u64));
    assert!(((({ nasty_list_int_Impl::front_25(&c.as_pointer(),) }).read()) == 4));
    return true;
}
pub fn test_3(c: ConstexprFixedCapacityDeque_int__10_) -> bool {
    let c: Value<ConstexprFixedCapacityDeque_int__10_> = Rc::new(RefCell::new(c));
    let i: Value<std_front_insert_iterator_ConstexprFixedCapacityDeque_int__10__> =
        Rc::new(RefCell::new(({ front_inserter_4(c.as_pointer()) })));
    (*i.borrow_mut()) = 3;
    assert!((({ ConstexprFixedCapacityDeque_int__10_Impl::size(&c.as_pointer(),) }) == 1_usize));
    assert!(
        ((({ ConstexprFixedCapacityDeque_int__10_Impl::front(&c.as_pointer(),) }).read()) == 3)
    );
    (*i.borrow_mut()) = 4;
    assert!((({ ConstexprFixedCapacityDeque_int__10_Impl::size(&c.as_pointer(),) }) == 2_usize));
    assert!(
        ((({ ConstexprFixedCapacityDeque_int__10_Impl::front(&c.as_pointer(),) }).read()) == 4)
    );
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
    ({ test_5(<std_list_int__std_allocator_int__>::default()) });
    ({ test_1(nasty_list_int_::new_1()) });
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
    fn size(&self) -> usize;
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
    fn size(&self) -> usize {
        return ((*self).with(|__s| __s.size_) as usize);
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
pub trait nasty_list_int_Impl {
    fn destructor(&self);
    fn operator_assign_9(&self, il: Vec<i32>) -> Ptr<nasty_list_int_> {
        unimplemented!()
    }
    fn assign_11(&self, mut n: u64, t: Ptr<i32>) {
        unimplemented!()
    }
    fn assign_12(&self, il: Vec<i32>) {
        unimplemented!()
    }
    fn begin_13(&self) -> BidirectionalIterator<i32> {
        unimplemented!()
    }
    fn begin_14(&self) -> BidirectionalIterator<i32> {
        unimplemented!()
    }
    fn end_15(&self) -> BidirectionalIterator<i32> {
        unimplemented!()
    }
    fn end_16(&self) -> BidirectionalIterator<i32> {
        unimplemented!()
    }
    fn rbegin_17(&self) -> BidirectionalIterator<i32> {
        unimplemented!()
    }
    fn rbegin_18(&self) -> BidirectionalIterator<i32> {
        unimplemented!()
    }
    fn rend_19(&self) -> BidirectionalIterator<i32> {
        unimplemented!()
    }
    fn rend_20(&self) -> BidirectionalIterator<i32> {
        unimplemented!()
    }
    fn cbegin(&self) -> BidirectionalIterator<i32> {
        unimplemented!()
    }
    fn cend(&self) -> BidirectionalIterator<i32> {
        unimplemented!()
    }
    fn crbegin(&self) -> BidirectionalIterator<i32> {
        unimplemented!()
    }
    fn crend(&self) -> BidirectionalIterator<i32> {
        unimplemented!()
    }
    fn front_25(&self) -> Ptr<i32>;
    fn front_26(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn back_27(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn back_28(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn size(&self) -> u64;
    fn max_size(&self) -> u64 {
        unimplemented!()
    }
    fn empty(&self) -> bool {
        unimplemented!()
    }
    fn push_front_32(&self, x: Ptr<i32>) {
        unimplemented!()
    }
    fn push_back_33(&self, x: Ptr<i32>) {
        unimplemented!()
    }
    fn push_back_34(&self, x: Ptr<i32>) {
        unimplemented!()
    }
    fn push_front_35(&self, x: Ptr<i32>);
    fn pop_front(&self) {
        unimplemented!()
    }
    fn pop_back(&self) {
        unimplemented!()
    }
    fn insert_41(
        &self,
        pos: BidirectionalIterator<i32>,
        x: Ptr<i32>,
    ) -> BidirectionalIterator<i32> {
        unimplemented!()
    }
    fn insert_42(
        &self,
        pos: BidirectionalIterator<i32>,
        x: Ptr<i32>,
    ) -> BidirectionalIterator<i32> {
        unimplemented!()
    }
    fn insert_43(
        &self,
        pos: BidirectionalIterator<i32>,
        mut n: u64,
        x: Ptr<i32>,
    ) -> BidirectionalIterator<i32> {
        unimplemented!()
    }
    fn insert_45(
        &self,
        pos: BidirectionalIterator<i32>,
        il: Vec<i32>,
    ) -> BidirectionalIterator<i32> {
        unimplemented!()
    }
    fn erase_46(&self, pos: BidirectionalIterator<i32>) -> BidirectionalIterator<i32> {
        unimplemented!()
    }
    fn erase_47(
        &self,
        first: BidirectionalIterator<i32>,
        last: BidirectionalIterator<i32>,
    ) -> BidirectionalIterator<i32> {
        unimplemented!()
    }
    fn resize_48(&self, mut n: u64) {
        unimplemented!()
    }
    fn resize_49(&self, mut n: u64, c: Ptr<i32>) {
        unimplemented!()
    }
    fn swap(&self, nl: Ptr<nasty_list_int_>) {
        unimplemented!()
    }
    fn clear(&self) {
        unimplemented!()
    }
    fn operator_addr_52(&self) -> Ptr<nasty_list_int_> {
        unimplemented!()
    }
    fn operator_addr_53(&self) -> Ptr<nasty_list_int_> {
        unimplemented!()
    }
}
impl nasty_list_int_Impl for Ptr<nasty_list_int_> {
    fn destructor(&self) {}
    fn front_25(&self) -> Ptr<i32> {
        return ({ (*(*self).upgrade().deref()).l_.front_41() });
    }
    fn size(&self) -> u64 {
        return ({ (*(*self).upgrade().deref()).l_.size() });
    }
    fn push_front_35(&self, x: Ptr<i32>) {
        ({
            let ___x: Ptr<i32> = (x).clone();
            (*(*self).upgrade().deref()).l_.push_front_46(___x)
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
