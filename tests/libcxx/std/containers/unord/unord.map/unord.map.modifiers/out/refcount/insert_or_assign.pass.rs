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
pub struct Moveable {
    #[offset(0)]
    int_: i32,
    #[offset(8)]
    double_: f64,
}
impl Moveable {
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
    pub fn move_from(x: Ptr<Moveable>) -> Self {
        let __this: Moveable = Self {
            int_: x.with(|__s| __s.int_),
            double_: x.with(|__s| __s.double_),
        };
        field!(x, int_).write(-1_i32);
        field!(x, double_).write((-1_i32 as f64));
        __this
    }
}
impl CppLess for Moveable {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        MoveableImpl::operator_lt(__lhs, __rhs.clone())
    }
}
impl CppEq for Moveable {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        MoveableImpl::operator_eq(__lhs, __rhs.clone())
    }
}
impl Default for Moveable {
    fn default() -> Self {
        { Moveable::new_1() }
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
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let r: Value<(Value<()>, Value<bool>)> = Rc::new(RefCell::new(unimplemented!()));
        let mut i: i32 = 0;
        'loop_: while (i < 20) {
            unimplemented!();
            i += 2;
        }
        assert!((unimplemented!() == 10_()));
        let mut i: i32 = 0;
        'loop_: while (i < 20) {
            let mv: Value<Moveable> = Rc::new(RefCell::new(Moveable::new_2({ (i + 1) }, {
                ((i + 1) as f64)
            })));
            r.as_pointer().write(unimplemented!());
            assert!((unimplemented!() == 10_()));
            assert!(!(*(*r.borrow()).1.borrow()));
            assert!(({ MoveableImpl::moved(&mv.as_pointer(),) }));
            assert!(
                ((*(*(*(*r.borrow()).0.borrow()).iter_deref().upgrade().deref())
                    .0
                    .borrow())
                    == i)
            );
            assert!(
                (({
                    MoveableImpl::get(
                        &(*(*(*r.borrow()).0.borrow()).iter_deref().upgrade().deref())
                            .1
                            .as_pointer(),
                    )
                }) == (i + 1))
            );
            i += 2;
        }
        let mv1: Value<Moveable> = Rc::new(RefCell::new(Moveable::new_2({ 5 }, { 5_f64 })));
        r.as_pointer().write(unimplemented!());
        assert!((unimplemented!() == 11_()));
        assert!(((*(*r.borrow()).1.borrow()) as bool));
        assert!(({ MoveableImpl::moved(&mv1.as_pointer(),) }));
        assert!(
            ((*(*(*(*r.borrow()).0.borrow()).iter_deref().upgrade().deref())
                .0
                .borrow())
                == -1_i32)
        );
        assert!(
            (({
                MoveableImpl::get(
                    &(*(*(*r.borrow()).0.borrow()).iter_deref().upgrade().deref())
                        .1
                        .as_pointer(),
                )
            }) == 5)
        );
        let mv2: Value<Moveable> = Rc::new(RefCell::new(Moveable::new_2({ 9 }, { 9_f64 })));
        r.as_pointer().write(unimplemented!());
        assert!((unimplemented!() == 12_()));
        assert!(((*(*r.borrow()).1.borrow()) as bool));
        assert!(({ MoveableImpl::moved(&mv2.as_pointer(),) }));
        assert!(
            ((*(*(*(*r.borrow()).0.borrow()).iter_deref().upgrade().deref())
                .0
                .borrow())
                == 3)
        );
        assert!(
            (({
                MoveableImpl::get(
                    &(*(*(*r.borrow()).0.borrow()).iter_deref().upgrade().deref())
                        .1
                        .as_pointer(),
                )
            }) == 9)
        );
        let mv3: Value<Moveable> = Rc::new(RefCell::new(Moveable::new_2({ -1_i32 }, { 5_f64 })));
        r.as_pointer().write(unimplemented!());
        assert!((unimplemented!() == 13_()));
        assert!(((*(*r.borrow()).1.borrow()) as bool));
        assert!(({ MoveableImpl::moved(&mv3.as_pointer(),) }));
        assert!(
            ((*(*(*(*r.borrow()).0.borrow()).iter_deref().upgrade().deref())
                .0
                .borrow())
                == 117)
        );
        assert!(
            (({
                MoveableImpl::get(
                    &(*(*(*r.borrow()).0.borrow()).iter_deref().upgrade().deref())
                        .1
                        .as_pointer(),
                )
            }) == -1_i32)
        );
    }
    {
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let r: Value<(Value<()>, Value<bool>)> = Rc::new(RefCell::new(unimplemented!()));
        let mut i: i32 = 0;
        'loop_: while (i < 20) {
            unimplemented!();
            i += 2;
        }
        assert!((unimplemented!() == 10_()));
        let mvkey1: Value<Moveable> = Rc::new(RefCell::new(Moveable::new_2({ 2 }, { 2_f64 })));
        let mv1: Value<Moveable> = Rc::new(RefCell::new(Moveable::new_2({ 4 }, { 4_f64 })));
        r.as_pointer().write(unimplemented!());
        assert!((unimplemented!() == 10_()));
        assert!(!(*(*r.borrow()).1.borrow()));
        assert!(!({ MoveableImpl::moved(&mvkey1.as_pointer(),) }));
        assert!(({ MoveableImpl::moved(&mv1.as_pointer(),) }));
        assert!(
            ({
                MoveableImpl::operator_eq(
                    &(*(*(*r.borrow()).0.borrow()).iter_deref().upgrade().deref())
                        .0
                        .as_pointer(),
                    mvkey1.as_pointer(),
                )
            })
        );
        assert!(
            (({
                MoveableImpl::get(
                    &(*(*(*r.borrow()).0.borrow()).iter_deref().upgrade().deref())
                        .1
                        .as_pointer(),
                )
            }) == 4)
        );
        let mvkey2: Value<Moveable> = Rc::new(RefCell::new(Moveable::new_2({ 3 }, { 3_f64 })));
        let mv2: Value<Moveable> = Rc::new(RefCell::new(Moveable::new_2({ 5 }, { 5_f64 })));
        r.as_pointer().write(unimplemented!());
        assert!((unimplemented!() == 11_()));
        assert!(((*(*r.borrow()).1.borrow()) as bool));
        assert!(({ MoveableImpl::moved(&mv2.as_pointer(),) }));
        assert!(({ MoveableImpl::moved(&mvkey2.as_pointer(),) }));
        assert!(
            (({
                MoveableImpl::get(
                    &(*(*(*r.borrow()).0.borrow()).iter_deref().upgrade().deref())
                        .0
                        .as_pointer(),
                )
            }) == 3)
        );
        assert!(
            (({
                MoveableImpl::get(
                    &(*(*(*r.borrow()).0.borrow()).iter_deref().upgrade().deref())
                        .1
                        .as_pointer(),
                )
            }) == 5)
        );
    }
    {
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let r: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut i: i32 = 0;
        'loop_: while (i < 20) {
            unimplemented!();
            i += 2;
        }
        assert!((unimplemented!() == 10_()));
        let it: Value<()> = Rc::new(RefCell::new(<()>::from(unimplemented!())));
        let mv1: Value<Moveable> = Rc::new(RefCell::new(Moveable::new_2({ 3 }, { 3_f64 })));
        (*r.borrow_mut()) = unimplemented!();
        assert!((unimplemented!() == 10_()));
        assert!(({ MoveableImpl::moved(&mv1.as_pointer(),) }));
        assert!(((*(*(*r.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 2));
        assert!(
            (({
                MoveableImpl::get(
                    &(*(*r.borrow()).iter_deref().upgrade().deref())
                        .1
                        .as_pointer(),
                )
            }) == 3)
        );
        let mv2: Value<Moveable> = Rc::new(RefCell::new(Moveable::new_2({ 5 }, { 5_f64 })));
        (*r.borrow_mut()) = unimplemented!();
        assert!((unimplemented!() == 11_()));
        assert!(({ MoveableImpl::moved(&mv2.as_pointer(),) }));
        assert!(((*(*(*r.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 3));
        assert!(
            (({
                MoveableImpl::get(
                    &(*(*r.borrow()).iter_deref().upgrade().deref())
                        .1
                        .as_pointer(),
                )
            }) == 5)
        );
    }
    {
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let r: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut i: i32 = 0;
        'loop_: while (i < 20) {
            unimplemented!();
            i += 2;
        }
        assert!((unimplemented!() == 10_()));
        let it: Value<()> = Rc::new(RefCell::new(libcc2rs::next_refcount(
            unimplemented!(),
            1_isize,
        )));
        let mvkey1: Value<Moveable> = Rc::new(RefCell::new(Moveable::new_2({ 2 }, { 2_f64 })));
        let mv1: Value<Moveable> = Rc::new(RefCell::new(Moveable::new_2({ 4 }, { 4_f64 })));
        (*r.borrow_mut()) = unimplemented!();
        assert!((unimplemented!() == 10_()));
        assert!(({ MoveableImpl::moved(&mv1.as_pointer(),) }));
        assert!(!({ MoveableImpl::moved(&mvkey1.as_pointer(),) }));
        assert!(
            ({
                MoveableImpl::operator_eq(
                    &(*(*r.borrow()).iter_deref().upgrade().deref())
                        .0
                        .as_pointer(),
                    mvkey1.as_pointer(),
                )
            })
        );
        assert!(
            (({
                MoveableImpl::get(
                    &(*(*r.borrow()).iter_deref().upgrade().deref())
                        .1
                        .as_pointer(),
                )
            }) == 4)
        );
        let mvkey2: Value<Moveable> = Rc::new(RefCell::new(Moveable::new_2({ 3 }, { 3_f64 })));
        let mv2: Value<Moveable> = Rc::new(RefCell::new(Moveable::new_2({ 5 }, { 5_f64 })));
        (*r.borrow_mut()) = unimplemented!();
        assert!((unimplemented!() == 11_()));
        assert!(({ MoveableImpl::moved(&mv2.as_pointer(),) }));
        assert!(({ MoveableImpl::moved(&mvkey2.as_pointer(),) }));
        assert!(
            (({
                MoveableImpl::get(
                    &(*(*r.borrow()).iter_deref().upgrade().deref())
                        .0
                        .as_pointer(),
                )
            }) == 3)
        );
        assert!(
            (({
                MoveableImpl::get(
                    &(*(*r.borrow()).iter_deref().upgrade().deref())
                        .1
                        .as_pointer(),
                )
            }) == 5)
        );
    }
    return 0;
}
pub trait MoveableImpl {
    fn copy_assign(&self, _a0: Ptr<Moveable>) -> Ptr<Moveable> {
        unimplemented!()
    }
    fn move_assign(&self, x: Ptr<Moveable>) -> Ptr<Moveable>;
    fn operator_eq(&self, x: Ptr<Moveable>) -> bool;
    fn operator_lt(&self, x: Ptr<Moveable>) -> bool;
    fn hash(&self) -> usize;
    fn get(&self) -> i32;
    fn moved(&self) -> bool;
}
impl MoveableImpl for Ptr<Moveable> {
    fn move_assign(&self, x: Ptr<Moveable>) -> Ptr<Moveable> {
        field!((*self), int_).write({ x.with(|__s| __s.int_) });
        field!(x, int_).write(-1_i32);
        field!((*self), double_).write({ x.with(|__s| __s.double_) });
        field!(x, double_).write((-1_i32 as f64));
        return (*self).clone();
    }
    fn operator_eq(&self, x: Ptr<Moveable>) -> bool {
        return ({ (*self).with(|__s| __s.int_) } == { x.with(|__s| __s.int_) })
            && ({ (*self).with(|__s| __s.double_) } == { x.with(|__s| __s.double_) });
    }
    fn operator_lt(&self, x: Ptr<Moveable>) -> bool {
        return ({ (*self).with(|__s| __s.int_) } < { x.with(|__s| __s.int_) })
            || (({ (*self).with(|__s| __s.int_) } == { x.with(|__s| __s.int_) })
                && ({ (*self).with(|__s| __s.double_) } < { x.with(|__s| __s.double_) }));
    }
    fn hash(&self) -> usize {
        return (unimplemented!()).wrapping_add(unimplemented!());
    }
    fn get(&self) -> i32 {
        return (*self).with(|__s| __s.int_);
    }
    fn moved(&self) -> bool {
        return ((*self).with(|__s| __s.int_) == -1_i32);
    }
}
pub fn __cpp2rust_init_globals() {}
