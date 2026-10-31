extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_0(
    c1: Vec<Option<Value<i32>>>,
    mut j: (),
    mut x1: Option<Value<i32>>,
    mut x2: Option<Value<i32>>,
    mut x3: Option<Value<i32>>,
    c2: Ptr<Vec<Option<Value<i32>>>>,
) {
    let c1: Value<Vec<Option<Value<i32>>>> = Rc::new(RefCell::new(c1));
    let q: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    assert!(unimplemented!());
}
pub fn insert3at_1(
    c: Ptr<Vec<Option<Value<i32>>>>,
    i: RandomAccessIterator<Option<Value<i32>>>,
    mut x1: Option<Value<i32>>,
    mut x2: Option<Value<i32>>,
    mut x3: Option<Value<i32>>,
) {
    let i: Value<RandomAccessIterator<Option<Value<i32>>>> = Rc::new(RefCell::new(i));
    (*i.borrow_mut()) = {
        let __it = <RandomAccessIterator<Option<Value<i32>>>>::from((*i.borrow()).clone());
        let __off = __it.index();
        c.with_mut(|__v: &mut Vec<Option<Value<i32>>>| __v.insert(__off, (x1).clone()));
        __it
    };
    (*i.borrow_mut()) = {
        let __it = <RandomAccessIterator<Option<Value<i32>>>>::from(
            ((*i.borrow_mut()).iter_inc()).clone(),
        );
        let __off = __it.index();
        c.with_mut(|__v: &mut Vec<Option<Value<i32>>>| __v.insert(__off, (x2).clone()));
        __it
    };
    {
        let __it = <RandomAccessIterator<Option<Value<i32>>>>::from(
            ((*i.borrow_mut()).iter_inc()).clone(),
        );
        let __off = __it.index();
        c.with_mut(|__v: &mut Vec<Option<Value<i32>>>| __v.insert(__off, (x3).clone()));
        __it
    };
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct do_nothing {}
impl Callable1<AnyPtr, ()> for do_nothing {
    fn call(&self, a1: AnyPtr) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            do_nothingImpl::operator_call(__it, a1)
        })
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
    {
        let c1: Value<Vec<Option<Value<i32>>>> = Rc::new(RefCell::new(Vec::new()));
        let x: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([
            0, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32,
        ])));
        let mut i: i32 = 0;
        'loop_: while (i < 3) {
            {
                let __a1 = (x.as_pointer() as Ptr<i32>)
                    .offset((i) as isize)
                    .to_owned_opt();
                (*c1.borrow_mut()).push(__a1)
            };
            i.prefix_inc();
        }
        let c2: Value<Vec<Option<Value<i32>>>> = Rc::new(RefCell::new(Vec::new()));
        let mut i: i32 = 0;
        'loop_: while (i < 3) {
            {
                let __a1 = (x.as_pointer() as Ptr<i32>)
                    .offset((i) as isize)
                    .to_owned_opt();
                (*c2.borrow_mut()).push(__a1)
            };
            i.prefix_inc();
        }
        ({
            let _c: Ptr<Vec<Option<Value<i32>>>> = c2.as_pointer();
            let _i: RandomAccessIterator<Option<Value<i32>>> = c2.decay().begin();
            let _x1: Option<Value<i32>> = (x.as_pointer() as Ptr<i32>)
                .offset((3) as isize)
                .to_owned_opt();
            let _x2: Option<Value<i32>> = (x.as_pointer() as Ptr<i32>)
                .offset((4) as isize)
                .to_owned_opt();
            let _x3: Option<Value<i32>> = (x.as_pointer() as Ptr<i32>)
                .offset((5) as isize)
                .to_owned_opt();
            insert3at_1(_c, _i, _x1, _x2, _x3)
        });
        ({
            let _x1: Option<Value<i32>> = (x.as_pointer() as Ptr<i32>)
                .offset((3) as isize)
                .to_owned_opt();
            let _x2: Option<Value<i32>> = (x.as_pointer() as Ptr<i32>)
                .offset((4) as isize)
                .to_owned_opt();
            let _x3: Option<Value<i32>> = (x.as_pointer() as Ptr<i32>)
                .offset((5) as isize)
                .to_owned_opt();
            test_0(
                std::mem::take(&mut (*c1.borrow_mut())),
                0_(),
                _x1,
                _x2,
                _x3,
                c2.as_pointer(),
            )
        });
        (*c1.borrow_mut()).clear();
        let mut i: i32 = 0;
        'loop_: while (i < 3) {
            {
                let __a1 = (x.as_pointer() as Ptr<i32>)
                    .offset((i) as isize)
                    .to_owned_opt();
                (*c1.borrow_mut()).push(__a1)
            };
            i.prefix_inc();
        }
        (*c2.borrow_mut()).clear();
        let mut i: i32 = 0;
        'loop_: while (i < 3) {
            {
                let __a1 = (x.as_pointer() as Ptr<i32>)
                    .offset((i) as isize)
                    .to_owned_opt();
                (*c2.borrow_mut()).push(__a1)
            };
            i.prefix_inc();
        }
        ({
            let _c: Ptr<Vec<Option<Value<i32>>>> = c2.as_pointer();
            let _i: RandomAccessIterator<Option<Value<i32>>> = c2.decay().begin().iter_add(1_isize);
            let _x1: Option<Value<i32>> = (x.as_pointer() as Ptr<i32>)
                .offset((3) as isize)
                .to_owned_opt();
            let _x2: Option<Value<i32>> = (x.as_pointer() as Ptr<i32>)
                .offset((4) as isize)
                .to_owned_opt();
            let _x3: Option<Value<i32>> = (x.as_pointer() as Ptr<i32>)
                .offset((5) as isize)
                .to_owned_opt();
            insert3at_1(_c, _i, _x1, _x2, _x3)
        });
        ({
            let _x1: Option<Value<i32>> = (x.as_pointer() as Ptr<i32>)
                .offset((3) as isize)
                .to_owned_opt();
            let _x2: Option<Value<i32>> = (x.as_pointer() as Ptr<i32>)
                .offset((4) as isize)
                .to_owned_opt();
            let _x3: Option<Value<i32>> = (x.as_pointer() as Ptr<i32>)
                .offset((5) as isize)
                .to_owned_opt();
            test_0(
                std::mem::take(&mut (*c1.borrow_mut())),
                1_(),
                _x1,
                _x2,
                _x3,
                c2.as_pointer(),
            )
        });
        (*c1.borrow_mut()).clear();
        let mut i: i32 = 0;
        'loop_: while (i < 3) {
            {
                let __a1 = (x.as_pointer() as Ptr<i32>)
                    .offset((i) as isize)
                    .to_owned_opt();
                (*c1.borrow_mut()).push(__a1)
            };
            i.prefix_inc();
        }
        (*c2.borrow_mut()).clear();
        let mut i: i32 = 0;
        'loop_: while (i < 3) {
            {
                let __a1 = (x.as_pointer() as Ptr<i32>)
                    .offset((i) as isize)
                    .to_owned_opt();
                (*c2.borrow_mut()).push(__a1)
            };
            i.prefix_inc();
        }
        ({
            let _c: Ptr<Vec<Option<Value<i32>>>> = c2.as_pointer();
            let _i: RandomAccessIterator<Option<Value<i32>>> = c2.decay().begin().iter_add(2_isize);
            let _x1: Option<Value<i32>> = (x.as_pointer() as Ptr<i32>)
                .offset((3) as isize)
                .to_owned_opt();
            let _x2: Option<Value<i32>> = (x.as_pointer() as Ptr<i32>)
                .offset((4) as isize)
                .to_owned_opt();
            let _x3: Option<Value<i32>> = (x.as_pointer() as Ptr<i32>)
                .offset((5) as isize)
                .to_owned_opt();
            insert3at_1(_c, _i, _x1, _x2, _x3)
        });
        ({
            let _x1: Option<Value<i32>> = (x.as_pointer() as Ptr<i32>)
                .offset((3) as isize)
                .to_owned_opt();
            let _x2: Option<Value<i32>> = (x.as_pointer() as Ptr<i32>)
                .offset((4) as isize)
                .to_owned_opt();
            let _x3: Option<Value<i32>> = (x.as_pointer() as Ptr<i32>)
                .offset((5) as isize)
                .to_owned_opt();
            test_0(
                std::mem::take(&mut (*c1.borrow_mut())),
                2_(),
                _x1,
                _x2,
                _x3,
                c2.as_pointer(),
            )
        });
        (*c1.borrow_mut()).clear();
        let mut i: i32 = 0;
        'loop_: while (i < 3) {
            {
                let __a1 = (x.as_pointer() as Ptr<i32>)
                    .offset((i) as isize)
                    .to_owned_opt();
                (*c1.borrow_mut()).push(__a1)
            };
            i.prefix_inc();
        }
        (*c2.borrow_mut()).clear();
        let mut i: i32 = 0;
        'loop_: while (i < 3) {
            {
                let __a1 = (x.as_pointer() as Ptr<i32>)
                    .offset((i) as isize)
                    .to_owned_opt();
                (*c2.borrow_mut()).push(__a1)
            };
            i.prefix_inc();
        }
        ({
            let _c: Ptr<Vec<Option<Value<i32>>>> = c2.as_pointer();
            let _i: RandomAccessIterator<Option<Value<i32>>> = c2.decay().begin().iter_add(3_isize);
            let _x1: Option<Value<i32>> = (x.as_pointer() as Ptr<i32>)
                .offset((3) as isize)
                .to_owned_opt();
            let _x2: Option<Value<i32>> = (x.as_pointer() as Ptr<i32>)
                .offset((4) as isize)
                .to_owned_opt();
            let _x3: Option<Value<i32>> = (x.as_pointer() as Ptr<i32>)
                .offset((5) as isize)
                .to_owned_opt();
            insert3at_1(_c, _i, _x1, _x2, _x3)
        });
        ({
            let _x1: Option<Value<i32>> = (x.as_pointer() as Ptr<i32>)
                .offset((3) as isize)
                .to_owned_opt();
            let _x2: Option<Value<i32>> = (x.as_pointer() as Ptr<i32>)
                .offset((4) as isize)
                .to_owned_opt();
            let _x3: Option<Value<i32>> = (x.as_pointer() as Ptr<i32>)
                .offset((5) as isize)
                .to_owned_opt();
            test_0(
                std::mem::take(&mut (*c1.borrow_mut())),
                3_(),
                _x1,
                _x2,
                _x3,
                c2.as_pointer(),
            )
        });
    }
    return 0;
}
pub trait do_nothingImpl {
    fn operator_call(&self, _a0: AnyPtr);
}
impl do_nothingImpl for Ptr<do_nothing> {
    fn operator_call(&self, mut _a0: AnyPtr) {}
}
pub fn __cpp2rust_init_globals() {}
