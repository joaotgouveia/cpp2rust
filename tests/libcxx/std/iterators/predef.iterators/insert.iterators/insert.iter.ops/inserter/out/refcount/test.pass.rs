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
    pub v_: (),
}
impl nasty_vector_int_ {
    pub fn new_1() -> Self {
        Self {
            v_: unimplemented!(),
        }
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
        unimplemented!();
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
    data_: Box<[()]>,
    #[offset(40)]
    size_: (),
}
impl Default for ConstexprFixedCapacityDeque_int__10_ {
    fn default() -> Self {
        {
            ConstexprFixedCapacityDeque_int__10_ {
                data_: (0..10).map(|_| unimplemented!()).collect::<Box<[()]>>(),
                size_: 0_(),
            }
        }
    }
}
pub fn test_0(c: ()) -> () {
    let c: Value<()> = Rc::new(RefCell::new(c));
    let i: Value<()> = Rc::new(RefCell::new(
        ({
            let ___x: Ptr<()> = c.as_pointer();
            let ___i: () = unimplemented!();
            inserter_1(___x, ___i)
        }),
    ));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
pub fn test_2(c: nasty_vector_int_) -> () {
    let c: Value<nasty_vector_int_> = Rc::new(RefCell::new(c));
    let i: Value<()> = Rc::new(RefCell::new(
        ({
            let ___x: Ptr<nasty_vector_int_> = c.as_pointer();
            let ___i: () = ({ nasty_vector_int_Impl::end_14(&c.as_pointer()) });
            inserter_3(___x, ___i)
        }),
    ));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
pub fn test_4(c: ConstexprFixedCapacityDeque_int__10_) -> () {
    let c: Value<ConstexprFixedCapacityDeque_int__10_> = Rc::new(RefCell::new(c));
    let i: Value<()> = Rc::new(RefCell::new(
        ({
            let ___x: Ptr<ConstexprFixedCapacityDeque_int__10_> = c.as_pointer();
            let ___i: Ptr<()> =
                ({ ConstexprFixedCapacityDeque_int__10_Impl::end_3(&c.as_pointer()) });
            inserter_5(___x, ___i)
        }),
    ));
    unimplemented!();
    unimplemented!();
    unimplemented!();
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
    ({ test_0(unimplemented!()) });
    ({ test_2(nasty_vector_int_::new_1()) });
    ({ test_4(<ConstexprFixedCapacityDeque_int__10_>::default()) });
    const _: () = assert!(true, "test(ConstexprFixedCapacityDeque<int, 10>())");;
    return 0_();
}
pub trait ConstexprFixedCapacityDeque_int__10_Impl {
    fn begin_2(&self) -> Ptr<()>;
    fn end_3(&self) -> Ptr<()>;
    fn begin_4(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn end_5(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn size(&self) -> ();
    fn front(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn back(&self) -> Ptr<()>;
    fn insert(&self, pos: Ptr<()>, t: ()) -> Ptr<()>;
    fn push_back(&self, mut t: ()) {
        unimplemented!()
    }
    fn push_front(&self, mut t: ()) {
        unimplemented!()
    }
}
impl ConstexprFixedCapacityDeque_int__10_Impl for Ptr<ConstexprFixedCapacityDeque_int__10_> {
    fn begin_2(&self) -> Ptr<()> {
        return (array_field_ptr!((*self), data_) as Ptr<()>);
    }
    fn end_3(&self) -> Ptr<()> {
        return (array_field_ptr!((*self), data_) as Ptr<()>)
            .offset(((*self).with(|__s| __s.size_)) as isize);
    }
    fn size(&self) -> () {
        return (*self).with(|__s| __s.size_);
    }
    fn back(&self) -> Ptr<()> {
        unimplemented!();
        return (array_field_ptr!((*self), data_) as Ptr<()>)
            .offset(((*self).with(|__s| __s.size_) - 1_()) as isize);
    }
    fn insert(&self, mut pos: Ptr<()>, mut t: ()) -> Ptr<()> {
        let mut i: () = ((pos).clone() - (array_field_ptr!((*self), data_) as Ptr<()>)) as ();
        if (i != (*self).with(|__s| __s.size_)) {
            unimplemented!();
        }
        array_elem!((*self), data_, i).write(t);
        {
            field!((*self), size_).with_mut(|__v| *__v = *__v + 1_())
        };
        return (array_field_ptr!((*self), data_) as Ptr<()>).offset((i) as isize);
    }
}
pub trait nasty_mutexImpl {
    fn destructor(&self);
    fn operator_addr(&self) -> Ptr<nasty_mutex>;
    fn copy_assign(&self, _a0: Ptr<nasty_mutex>) -> Ptr<nasty_mutex>;
    fn lock(&self);
    fn try_lock(&self) -> ();
    fn unlock(&self);
    fn lock_shared(&self);
    fn try_lock_shared(&self) -> ();
    fn unlock_shared(&self);
}
impl nasty_mutexImpl for Ptr<nasty_mutex> {
    fn destructor(&self) {}
    fn operator_addr(&self) -> Ptr<nasty_mutex> {
        unimplemented!();
        return Ptr::<nasty_mutex>::null();
    }
    fn copy_assign(&self, _a0: Ptr<nasty_mutex>) -> Ptr<nasty_mutex> {
        unimplemented!();
        return (*self).clone();
    }
    fn lock(&self) {}
    fn try_lock(&self) -> () {
        return true;
    }
    fn unlock(&self) {}
    fn lock_shared(&self) {}
    fn try_lock_shared(&self) -> () {
        return true;
    }
    fn unlock_shared(&self) {}
}
pub trait nasty_vector_int_Impl {
    fn destructor(&self);
    fn assign_10(&self, mut n: (), u: Ptr<()>) {
        unimplemented!()
    }
    fn assign_11(&self, il: ()) {
        unimplemented!()
    }
    fn begin_12(&self) -> ();
    fn begin_13(&self) -> () {
        unimplemented!()
    }
    fn end_14(&self) -> ();
    fn end_15(&self) -> () {
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
    fn cbegin(&self) -> () {
        unimplemented!()
    }
    fn cend(&self) -> () {
        unimplemented!()
    }
    fn crbegin(&self) -> () {
        unimplemented!()
    }
    fn crend(&self) -> () {
        unimplemented!()
    }
    fn size(&self) -> ();
    fn max_size(&self) -> () {
        unimplemented!()
    }
    fn capacity(&self) -> () {
        unimplemented!()
    }
    fn empty(&self) -> () {
        unimplemented!()
    }
    fn reserve(&self, mut n: ()) {
        unimplemented!()
    }
    fn shrink_to_fit(&self) {
        unimplemented!()
    }
    fn operator_index_30(&self, mut n: ()) -> () {
        unimplemented!()
    }
    fn operator_index_31(&self, mut n: ()) -> () {
        unimplemented!()
    }
    fn at_32(&self, mut n: ()) -> () {
        unimplemented!()
    }
    fn at_33(&self, mut n: ()) -> () {
        unimplemented!()
    }
    fn front_34(&self) -> () {
        unimplemented!()
    }
    fn front_35(&self) -> () {
        unimplemented!()
    }
    fn back_36(&self) -> ();
    fn back_37(&self) -> () {
        unimplemented!()
    }
    fn data_38(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn data_39(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn push_back_40(&self, x: Ptr<()>) {
        unimplemented!()
    }
    fn push_back_41(&self, x: Ptr<()>) {
        unimplemented!()
    }
    fn pop_back(&self) {
        unimplemented!()
    }
    fn insert_45(&self, pos: (), x: Ptr<()>) -> () {
        unimplemented!()
    }
    fn insert_46(&self, pos: (), x: Ptr<()>) -> ();
    fn insert_47(&self, pos: (), mut n: (), x: Ptr<()>) -> () {
        unimplemented!()
    }
    fn insert_49(&self, pos: (), il: ()) -> () {
        unimplemented!()
    }
    fn erase_50(&self, pos: ()) -> () {
        unimplemented!()
    }
    fn erase_51(&self, first: (), last: ()) -> () {
        unimplemented!()
    }
    fn clear(&self) {
        unimplemented!()
    }
    fn resize_53(&self, mut sz: ()) {
        unimplemented!()
    }
    fn resize_54(&self, mut sz: (), c: Ptr<()>) {
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
    fn begin_12(&self) -> () {
        return unimplemented!();
    }
    fn end_14(&self) -> () {
        return unimplemented!();
    }
    fn size(&self) -> () {
        return unimplemented!();
    }
    fn back_36(&self) -> () {
        return unimplemented!();
    }
    fn insert_46(&self, pos: (), x: Ptr<()>) -> () {
        let pos: Value<()> = Rc::new(RefCell::new(pos));
        return unimplemented!();
    }
}
pub fn __cpp2rust_init_globals() {}
