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
impl CppEq for nasty_vector_int_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_0(__lhs.clone(), __rhs.clone())
    }
}
impl Default for nasty_vector_int_ {
    fn default() -> Self {
        { nasty_vector_int_::new_1() }
    }
}
pub fn operator_eq_0(x: Ptr<nasty_vector_int_>, y: Ptr<nasty_vector_int_>) -> () {
    return unimplemented!();
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
pub fn test_1(c1: (), mut j: (), mut x1: (), mut x2: (), mut x3: (), c2: Ptr<()>) {
    let c1: Value<()> = Rc::new(RefCell::new(c1));
    let q: Value<()> = Rc::new(RefCell::new(
        std_insert_iterator_std_vector_int__std_allocator_int___::new({ c1.as_pointer() }, {
            unimplemented!()
        }),
    ));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_2(
    c1: nasty_vector_int_,
    mut j: (),
    mut x1: (),
    mut x2: (),
    mut x3: (),
    c2: Ptr<nasty_vector_int_>,
) {
    let c1: Value<nasty_vector_int_> = Rc::new(RefCell::new(c1));
    let q: Value<()> = Rc::new(RefCell::new(std_insert_iterator_nasty_vector_int__::new(
        { c1.as_pointer() },
        { unimplemented!() },
    )));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn insert3at_3(c: Ptr<()>, i: (), mut x1: (), mut x2: (), mut x3: ()) {
    let i: Value<()> = Rc::new(RefCell::new(i));
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn insert3at_4(c: Ptr<nasty_vector_int_>, i: (), x1: (), x2: (), x3: ()) {
    let i: Value<()> = Rc::new(RefCell::new(i));
    let x1: Value<()> = Rc::new(RefCell::new(x1));
    let x2: Value<()> = Rc::new(RefCell::new(x2));
    let x3: Value<()> = Rc::new(RefCell::new(x3));
    unimplemented!();
    unimplemented!();
    ({
        let _pos: () = unimplemented!();
        let _x: Ptr<()> = x3.as_pointer();
        nasty_vector_int_Impl::insert_45(&c, _pos, _x)
    });
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
    {
        let mut c1: () = unimplemented!();
        let mut i: () = 0_();
        'loop_: while (i < 3_()) {
            unimplemented!();
            i.prefix_inc();
        }
        let c2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({
            let _c: Ptr<()> = c2.as_pointer();
            let _i: () = unimplemented!();
            insert3at_3(_c, _i, ('a' as ()), ('b' as ()), ('c' as ()))
        });
        ({
            test_1(
                unimplemented!(),
                0_(),
                ('a' as ()),
                ('b' as ()),
                ('c' as ()),
                c2.as_pointer(),
            )
        });
        unimplemented!();
        ({
            let _c: Ptr<()> = c2.as_pointer();
            let _i: () = unimplemented!();
            insert3at_3(_c, _i, ('a' as ()), ('b' as ()), ('c' as ()))
        });
        ({
            test_1(
                unimplemented!(),
                1_(),
                ('a' as ()),
                ('b' as ()),
                ('c' as ()),
                c2.as_pointer(),
            )
        });
        unimplemented!();
        ({
            let _c: Ptr<()> = c2.as_pointer();
            let _i: () = unimplemented!();
            insert3at_3(_c, _i, ('a' as ()), ('b' as ()), ('c' as ()))
        });
        ({
            test_1(
                unimplemented!(),
                2_(),
                ('a' as ()),
                ('b' as ()),
                ('c' as ()),
                c2.as_pointer(),
            )
        });
        unimplemented!();
        ({
            let _c: Ptr<()> = c2.as_pointer();
            let _i: () = unimplemented!();
            insert3at_3(_c, _i, ('a' as ()), ('b' as ()), ('c' as ()))
        });
        ({
            test_1(
                unimplemented!(),
                3_(),
                ('a' as ()),
                ('b' as ()),
                ('c' as ()),
                c2.as_pointer(),
            )
        });
    }
    {
        let c1: Value<nasty_vector_int_> = Rc::new(RefCell::new(nasty_vector_int_::new_1()));
        let _dtor_c1 = ScopedDestructor::new(&c1, |__p| __p.destructor());
        let i: Value<()> = Rc::new(RefCell::new(0_()));
        'loop_: while ((*i.borrow()) < 3_()) {
            ({ nasty_vector_int_Impl::push_back_40(&c1.as_pointer(), i.as_pointer()) });
            (*i.borrow_mut()).prefix_inc();
        }
        let c2: Value<nasty_vector_int_> = Rc::new(RefCell::new((*c1.borrow()).clone()));
        let _dtor_c2 = ScopedDestructor::new(&c2, |__p| __p.destructor());
        ({
            let _c: Ptr<nasty_vector_int_> = c2.as_pointer();
            let _i: () = ({ nasty_vector_int_Impl::begin_12(&c2.as_pointer()) });
            insert3at_4(_c, _i, ('a' as ()), ('b' as ()), ('c' as ()))
        });
        ({
            test_2(
                (*c1.borrow()).clone(),
                0_(),
                ('a' as ()),
                ('b' as ()),
                ('c' as ()),
                c2.as_pointer(),
            )
        });
        (*c2.borrow_mut()) = (*c1.borrow()).clone();
        ({
            let _c: Ptr<nasty_vector_int_> = c2.as_pointer();
            let _i: () = unimplemented!();
            insert3at_4(_c, _i, ('a' as ()), ('b' as ()), ('c' as ()))
        });
        ({
            test_2(
                (*c1.borrow()).clone(),
                1_(),
                ('a' as ()),
                ('b' as ()),
                ('c' as ()),
                c2.as_pointer(),
            )
        });
        (*c2.borrow_mut()) = (*c1.borrow()).clone();
        ({
            let _c: Ptr<nasty_vector_int_> = c2.as_pointer();
            let _i: () = unimplemented!();
            insert3at_4(_c, _i, ('a' as ()), ('b' as ()), ('c' as ()))
        });
        ({
            test_2(
                (*c1.borrow()).clone(),
                2_(),
                ('a' as ()),
                ('b' as ()),
                ('c' as ()),
                c2.as_pointer(),
            )
        });
        (*c2.borrow_mut()) = (*c1.borrow()).clone();
        ({
            let _c: Ptr<nasty_vector_int_> = c2.as_pointer();
            let _i: () = unimplemented!();
            insert3at_4(_c, _i, ('a' as ()), ('b' as ()), ('c' as ()))
        });
        ({
            test_2(
                (*c1.borrow()).clone(),
                3_(),
                ('a' as ()),
                ('b' as ()),
                ('c' as ()),
                c2.as_pointer(),
            )
        });
    }
    return 0_();
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
    fn end_14(&self) -> () {
        unimplemented!()
    }
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
    fn size(&self) -> () {
        unimplemented!()
    }
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
    fn back_36(&self) -> () {
        unimplemented!()
    }
    fn back_37(&self) -> () {
        unimplemented!()
    }
    fn data_38(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn data_39(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn push_back_40(&self, x: Ptr<()>);
    fn push_back_41(&self, x: Ptr<()>) {
        unimplemented!()
    }
    fn pop_back(&self) {
        unimplemented!()
    }
    fn insert_45(&self, pos: (), x: Ptr<()>) -> ();
    fn insert_46(&self, pos: (), x: Ptr<()>) -> () {
        unimplemented!()
    }
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
    fn push_back_40(&self, x: Ptr<()>) {
        unimplemented!();
    }
    fn insert_45(&self, pos: (), x: Ptr<()>) -> () {
        let pos: Value<()> = Rc::new(RefCell::new(pos));
        return unimplemented!();
    }
}
pub fn __cpp2rust_init_globals() {}
