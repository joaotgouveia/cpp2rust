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
pub fn operator_eq_0(x: Ptr<nasty_vector_int_>, y: Ptr<nasty_vector_int_>) -> bool {
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
pub fn test_1(c1: Vec<i32>, mut j: (), mut x1: i32, mut x2: i32, mut x3: i32, c2: Ptr<Vec<i32>>) {
    let c1: Value<Vec<i32>> = Rc::new(RefCell::new(c1));
    let q: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    assert!(unimplemented!());
}
pub fn test_2(
    c1: nasty_vector_int_,
    mut j: (),
    mut x1: i32,
    mut x2: i32,
    mut x3: i32,
    c2: Ptr<nasty_vector_int_>,
) {
    let c1: Value<nasty_vector_int_> = Rc::new(RefCell::new(c1));
    let q: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    assert!(
        ({
            let _x: Ptr<nasty_vector_int_> = c1.as_pointer();
            let _y: Ptr<nasty_vector_int_> = (c2).clone();
            operator_eq_0(_x, _y)
        })
    );
}
pub fn insert3at_3(
    c: Ptr<Vec<i32>>,
    i: RandomAccessIterator<i32>,
    mut x1: i32,
    mut x2: i32,
    mut x3: i32,
) {
    let i: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new(i));
    (*i.borrow_mut()) = {
        let __it = <RandomAccessIterator<i32>>::from((*i.borrow()).clone());
        let __off = __it.index();
        c.with_mut(|__v: &mut Vec<i32>| __v.insert(__off, x1));
        __it
    };
    (*i.borrow_mut()) = {
        let __it = <RandomAccessIterator<i32>>::from(((*i.borrow_mut()).iter_inc()).clone());
        let __off = __it.index();
        c.with_mut(|__v: &mut Vec<i32>| __v.insert(__off, x2));
        __it
    };
    {
        let __it = <RandomAccessIterator<i32>>::from(((*i.borrow_mut()).iter_inc()).clone());
        let __off = __it.index();
        c.with_mut(|__v: &mut Vec<i32>| __v.insert(__off, x3));
        __it
    };
}
pub fn insert3at_4(
    c: Ptr<nasty_vector_int_>,
    i: RandomAccessIterator<i32>,
    x1: i32,
    x2: i32,
    x3: i32,
) {
    let i: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new(i));
    let x1: Value<i32> = Rc::new(RefCell::new(x1));
    let x2: Value<i32> = Rc::new(RefCell::new(x2));
    let x3: Value<i32> = Rc::new(RefCell::new(x3));
    (*i.borrow_mut()) = ({
        let _pos: RandomAccessIterator<i32> =
            <RandomAccessIterator<i32>>::from((*i.borrow()).clone());
        let _x: Ptr<i32> = x1.as_pointer();
        nasty_vector_int_Impl::insert_45(&c, _pos, _x)
    });
    (*i.borrow_mut()) = ({
        let _pos: RandomAccessIterator<i32> =
            <RandomAccessIterator<i32>>::from(((*i.borrow_mut()).iter_inc()).clone());
        let _x: Ptr<i32> = x2.as_pointer();
        nasty_vector_int_Impl::insert_45(&c, _pos, _x)
    });
    ({
        let _pos: RandomAccessIterator<i32> =
            <RandomAccessIterator<i32>>::from(((*i.borrow_mut()).iter_inc()).clone());
        let _x: Ptr<i32> = x3.as_pointer();
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> i32 {
    {
        let mut c1: Vec<i32> = Vec::new();
        let mut i: i32 = 0;
        'loop_: while (i < 3) {
            {
                let a0_clone = i.clone();
                c1.push(a0_clone)
            };
            i.prefix_inc();
        }
        let c2: Value<Vec<i32>> = Rc::new(RefCell::new(c1.clone()));
        ({
            let _c: Ptr<Vec<i32>> = c2.as_pointer();
            let _i: RandomAccessIterator<i32> = c2.decay().begin();
            insert3at_3(
                _c,
                _i,
                (('a' as i8) as i32),
                (('b' as i8) as i32),
                (('c' as i8) as i32),
            )
        });
        ({
            test_1(
                c1.clone(),
                0_(),
                (('a' as i8) as i32),
                (('b' as i8) as i32),
                (('c' as i8) as i32),
                c2.as_pointer(),
            )
        });
        c2.as_pointer().write((c1).clone());
        ({
            let _c: Ptr<Vec<i32>> = c2.as_pointer();
            let _i: RandomAccessIterator<i32> = c2.decay().begin().iter_add(1_isize);
            insert3at_3(
                _c,
                _i,
                (('a' as i8) as i32),
                (('b' as i8) as i32),
                (('c' as i8) as i32),
            )
        });
        ({
            test_1(
                c1.clone(),
                1_(),
                (('a' as i8) as i32),
                (('b' as i8) as i32),
                (('c' as i8) as i32),
                c2.as_pointer(),
            )
        });
        c2.as_pointer().write((c1).clone());
        ({
            let _c: Ptr<Vec<i32>> = c2.as_pointer();
            let _i: RandomAccessIterator<i32> = c2.decay().begin().iter_add(2_isize);
            insert3at_3(
                _c,
                _i,
                (('a' as i8) as i32),
                (('b' as i8) as i32),
                (('c' as i8) as i32),
            )
        });
        ({
            test_1(
                c1.clone(),
                2_(),
                (('a' as i8) as i32),
                (('b' as i8) as i32),
                (('c' as i8) as i32),
                c2.as_pointer(),
            )
        });
        c2.as_pointer().write((c1).clone());
        ({
            let _c: Ptr<Vec<i32>> = c2.as_pointer();
            let _i: RandomAccessIterator<i32> = c2.decay().begin().iter_add(3_isize);
            insert3at_3(
                _c,
                _i,
                (('a' as i8) as i32),
                (('b' as i8) as i32),
                (('c' as i8) as i32),
            )
        });
        ({
            test_1(
                c1.clone(),
                3_(),
                (('a' as i8) as i32),
                (('b' as i8) as i32),
                (('c' as i8) as i32),
                c2.as_pointer(),
            )
        });
    }
    {
        let c1: Value<nasty_vector_int_> = Rc::new(RefCell::new(nasty_vector_int_::new_1()));
        let _dtor_c1 = ScopedDestructor::new(&c1, |__p| __p.destructor());
        let i: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*i.borrow()) < 3) {
            ({ nasty_vector_int_Impl::push_back_40(&c1.as_pointer(), i.as_pointer()) });
            (*i.borrow_mut()).prefix_inc();
        }
        let c2: Value<nasty_vector_int_> = Rc::new(RefCell::new((*c1.borrow()).clone()));
        let _dtor_c2 = ScopedDestructor::new(&c2, |__p| __p.destructor());
        ({
            let _c: Ptr<nasty_vector_int_> = c2.as_pointer();
            let _i: RandomAccessIterator<i32> =
                ({ nasty_vector_int_Impl::begin_12(&c2.as_pointer()) });
            insert3at_4(
                _c,
                _i,
                (('a' as i8) as i32),
                (('b' as i8) as i32),
                (('c' as i8) as i32),
            )
        });
        ({
            test_2(
                (*c1.borrow()).clone(),
                0_(),
                (('a' as i8) as i32),
                (('b' as i8) as i32),
                (('c' as i8) as i32),
                c2.as_pointer(),
            )
        });
        (*c2.borrow_mut()) = (*c1.borrow()).clone();
        ({
            let _c: Ptr<nasty_vector_int_> = c2.as_pointer();
            let _i: RandomAccessIterator<i32> =
                ({ nasty_vector_int_Impl::begin_12(&c2.as_pointer()) }).iter_add(1_isize);
            insert3at_4(
                _c,
                _i,
                (('a' as i8) as i32),
                (('b' as i8) as i32),
                (('c' as i8) as i32),
            )
        });
        ({
            test_2(
                (*c1.borrow()).clone(),
                1_(),
                (('a' as i8) as i32),
                (('b' as i8) as i32),
                (('c' as i8) as i32),
                c2.as_pointer(),
            )
        });
        (*c2.borrow_mut()) = (*c1.borrow()).clone();
        ({
            let _c: Ptr<nasty_vector_int_> = c2.as_pointer();
            let _i: RandomAccessIterator<i32> =
                ({ nasty_vector_int_Impl::begin_12(&c2.as_pointer()) }).iter_add(2_isize);
            insert3at_4(
                _c,
                _i,
                (('a' as i8) as i32),
                (('b' as i8) as i32),
                (('c' as i8) as i32),
            )
        });
        ({
            test_2(
                (*c1.borrow()).clone(),
                2_(),
                (('a' as i8) as i32),
                (('b' as i8) as i32),
                (('c' as i8) as i32),
                c2.as_pointer(),
            )
        });
        (*c2.borrow_mut()) = (*c1.borrow()).clone();
        ({
            let _c: Ptr<nasty_vector_int_> = c2.as_pointer();
            let _i: RandomAccessIterator<i32> =
                ({ nasty_vector_int_Impl::begin_12(&c2.as_pointer()) }).iter_add(3_isize);
            insert3at_4(
                _c,
                _i,
                (('a' as i8) as i32),
                (('b' as i8) as i32),
                (('c' as i8) as i32),
            )
        });
        ({
            test_2(
                (*c1.borrow()).clone(),
                3_(),
                (('a' as i8) as i32),
                (('b' as i8) as i32),
                (('c' as i8) as i32),
                c2.as_pointer(),
            )
        });
    }
    return 0;
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
    fn begin_12(&self) -> RandomAccessIterator<i32>;
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
    fn size(&self) -> usize {
        unimplemented!()
    }
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
    fn back_36(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn back_37(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn data_38(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn data_39(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn push_back_40(&self, x: Ptr<i32>);
    fn push_back_41(&self, x: Ptr<i32>) {
        unimplemented!()
    }
    fn pop_back(&self) {
        unimplemented!()
    }
    fn insert_45(&self, pos: RandomAccessIterator<i32>, x: Ptr<i32>) -> RandomAccessIterator<i32>;
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
    fn begin_12(&self) -> RandomAccessIterator<i32> {
        return field_ref!((*self), v_).decay().begin();
    }
    fn push_back_40(&self, x: Ptr<i32>) {
        {
            let a0_clone = (x.read()).clone();
            field!((*self), v_).with_mut(|__v: &mut Vec<i32>| __v.push(a0_clone))
        };
    }
    fn insert_45(&self, pos: RandomAccessIterator<i32>, x: Ptr<i32>) -> RandomAccessIterator<i32> {
        let pos: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new(pos));
        return {
            let __it = (*pos.borrow()).clone();
            let __off = __it.index();
            field!((*self), v_).with_mut(|__v: &mut Vec<i32>| __v.insert(__off, (x.read())));
            __it
        };
    }
}
pub fn __cpp2rust_init_globals() {}
