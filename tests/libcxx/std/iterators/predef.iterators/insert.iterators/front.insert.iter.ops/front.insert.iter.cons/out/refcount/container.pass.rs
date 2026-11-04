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
pub fn test_0(c: nasty_list_int_) -> bool {
    let c: Value<nasty_list_int_> = Rc::new(RefCell::new(c));
    let _dtor_c = ScopedDestructor::new(&c, |__p| __p.destructor());
    let i: Value<std_front_insert_iterator_nasty_list_int__> = Rc::new(RefCell::new(
        std_front_insert_iterator_nasty_list_int__::new({ c.as_pointer() }),
    ));
    return true;
}
pub fn test_1(c: ConstexprFixedCapacityDeque_int__10_) -> bool {
    let c: Value<ConstexprFixedCapacityDeque_int__10_> = Rc::new(RefCell::new(c));
    let i: Value<std_front_insert_iterator_ConstexprFixedCapacityDeque_int__10__> =
        Rc::new(RefCell::new(
            std_front_insert_iterator_ConstexprFixedCapacityDeque_int__10__::new({
                c.as_pointer()
            }),
        ));
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
    ({ test_2(<std_list_int__std_allocator_int__>::default()) });
    ({ test_0(nasty_list_int_::new_1()) });
    ({ test_1(<ConstexprFixedCapacityDeque_int__10_>::default()) });
    const _: () = assert!(true, "test(ConstexprFixedCapacityDeque<int, 10>())");;
    return 0;
}
pub trait ConstexprFixedCapacityDeque_int__10_Impl {
    fn begin_2(&self) -> Ptr<i32> {
        unimplemented!()
    }
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
    fn front(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn back(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn insert(&self, mut pos: Ptr<i32>, mut t: i32) -> Ptr<i32> {
        unimplemented!()
    }
    fn push_back(&self, mut t: i32) {
        unimplemented!()
    }
    fn push_front(&self, mut t: i32) {
        unimplemented!()
    }
}
impl ConstexprFixedCapacityDeque_int__10_Impl for Ptr<ConstexprFixedCapacityDeque_int__10_> {}
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
    fn front_25(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn front_26(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn back_27(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn back_28(&self) -> Ptr<i32> {
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
    fn push_front_32(&self, x: Ptr<i32>) {
        unimplemented!()
    }
    fn push_back_33(&self, x: Ptr<i32>) {
        unimplemented!()
    }
    fn push_back_34(&self, x: Ptr<i32>) {
        unimplemented!()
    }
    fn push_front_35(&self, x: Ptr<i32>) {
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
