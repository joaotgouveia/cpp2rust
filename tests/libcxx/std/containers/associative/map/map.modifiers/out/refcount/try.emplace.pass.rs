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
impl CppMove for Moveable {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
impl Default for Moveable {
    fn default() -> Self {
        { Moveable::new_1() }
    }
}
pub fn test_0() -> bool {
    {
        let m: Value<Map<i32, (Value<i32>, Value<Moveable>)>> = Rc::new(RefCell::new(Map::new()));
        let r: Value<(
            Value<BidirectionalIterator<(Value<i32>, Value<Moveable>)>>,
            Value<bool>,
        )> = Rc::new(RefCell::new((
            Rc::new(RefCell::new(BidirectionalIterator::default())),
            Rc::new(RefCell::new(false)),
        )));
        let i: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*i.borrow()) < 20) {
            {
                let (__it, __inserted) = Map::emplace(
                    m.as_pointer(),
                    (
                        Rc::new(RefCell::new(({ (*i.borrow()) }))),
                        Rc::new(RefCell::new(Moveable::new_2({ (*i.borrow()) }, {
                            ((*i.borrow()) as f64)
                        }))),
                    ),
                );
                (
                    Rc::new(RefCell::new(__it)),
                    Rc::new(RefCell::new(__inserted)),
                )
            };
            (*i.borrow_mut()) += 2;
        }
        assert!(({ ((*m.borrow()).len() == 10_usize) }));
        let mv1: Value<Moveable> = Rc::new(RefCell::new(Moveable::new_2({ 3 }, { 3_f64 })));
        let i: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*i.borrow()) < 20) {
            {
                r.as_pointer().write(CppMove::cpp_move(
                    Rc::new(RefCell::new({
                        let __inserted =
                            m.as_pointer()
                                .with(|__m: &Map<i32, (Value<i32>, Value<Moveable>)>| {
                                    __m.get(&i.as_pointer()).is_none()
                                });
                        if __inserted {
                            Map::subscript(m.as_pointer().clone(), i.as_pointer().clone())
                                .write(Moveable::move_from({ mv1.as_pointer() }));
                        }
                        (
                            Rc::new(RefCell::new(m.as_pointer().with(
                                |__m: &Map<i32, (Value<i32>, Value<Moveable>)>| {
                                    __m.find(&i.as_pointer())
                                },
                            ))),
                            Rc::new(RefCell::new(__inserted)),
                        )
                    }))
                    .as_pointer(),
                ));
                r.as_pointer()
            };
            assert!(({ ((*m.borrow()).len() == 10_usize) }));
            assert!(({ !(*(*r.borrow()).1.borrow()) }));
            assert!(!({ MoveableImpl::moved(&mv1.as_pointer(),) }));
            assert!(
                ({
                    ((*(*(*(*r.borrow()).0.borrow()).iter_deref().upgrade().deref())
                        .0
                        .borrow())
                        == (*i.borrow()))
                })
            );
            (*i.borrow_mut()) += 2;
        }
        {
            r.as_pointer().write(CppMove::cpp_move(
                Rc::new(RefCell::new(
                    ({
                        let ___k: Value<i32> = Rc::new(RefCell::new(-1_i32));
                        (*m.borrow()).try_emplace(___k.as_pointer(), mv1.as_pointer())
                    }),
                ))
                .as_pointer(),
            ));
            r.as_pointer()
        };
        assert!(({ ((*m.borrow()).len() == 11_usize) }));
        assert!(({ ((*(*r.borrow()).1.borrow()) as bool) }));
        assert!(({ MoveableImpl::moved(&mv1.as_pointer(),) }));
        assert!(
            ({
                ((*(*(*(*r.borrow()).0.borrow()).iter_deref().upgrade().deref())
                    .0
                    .borrow())
                    == -1_i32)
            })
        );
        assert!(
            ({
                (({
                    MoveableImpl::get(
                        &(*(*(*r.borrow()).0.borrow()).iter_deref().upgrade().deref())
                            .1
                            .as_pointer(),
                    )
                }) == 3)
            })
        );
        let mv2: Value<Moveable> = Rc::new(RefCell::new(Moveable::new_2({ 5 }, { 3_f64 })));
        {
            r.as_pointer().write(CppMove::cpp_move(
                Rc::new(RefCell::new(
                    ({
                        let ___k: Value<i32> = Rc::new(RefCell::new(5));
                        (*m.borrow()).try_emplace(___k.as_pointer(), mv2.as_pointer())
                    }),
                ))
                .as_pointer(),
            ));
            r.as_pointer()
        };
        assert!(({ ((*m.borrow()).len() == 12_usize) }));
        assert!(({ ((*(*r.borrow()).1.borrow()) as bool) }));
        assert!(({ MoveableImpl::moved(&mv2.as_pointer(),) }));
        assert!(
            ({
                ((*(*(*(*r.borrow()).0.borrow()).iter_deref().upgrade().deref())
                    .0
                    .borrow())
                    == 5)
            })
        );
        assert!(
            ({
                (({
                    MoveableImpl::get(
                        &(*(*(*r.borrow()).0.borrow()).iter_deref().upgrade().deref())
                            .1
                            .as_pointer(),
                    )
                }) == 5)
            })
        );
        let mv3: Value<Moveable> = Rc::new(RefCell::new(Moveable::new_2({ -1_i32 }, { 3_f64 })));
        {
            r.as_pointer().write(CppMove::cpp_move(
                Rc::new(RefCell::new(
                    ({
                        let ___k: Value<i32> = Rc::new(RefCell::new(117));
                        (*m.borrow()).try_emplace(___k.as_pointer(), mv3.as_pointer())
                    }),
                ))
                .as_pointer(),
            ));
            r.as_pointer()
        };
        assert!(({ ((*m.borrow()).len() == 13_usize) }));
        assert!(({ ((*(*r.borrow()).1.borrow()) as bool) }));
        assert!(({ MoveableImpl::moved(&mv3.as_pointer(),) }));
        assert!(
            ({
                ((*(*(*(*r.borrow()).0.borrow()).iter_deref().upgrade().deref())
                    .0
                    .borrow())
                    == 117)
            })
        );
        assert!(
            ({
                (({
                    MoveableImpl::get(
                        &(*(*(*r.borrow()).0.borrow()).iter_deref().upgrade().deref())
                            .1
                            .as_pointer(),
                    )
                }) == -1_i32)
            })
        );
    }
    {
        let m: Value<Map<Moveable, (Value<Moveable>, Value<Moveable>)>> =
            Rc::new(RefCell::new(Map::new()));
        let r: Value<(
            Value<BidirectionalIterator<(Value<Moveable>, Value<Moveable>)>>,
            Value<bool>,
        )> = Rc::new(RefCell::new((
            Rc::new(RefCell::new(BidirectionalIterator::default())),
            Rc::new(RefCell::new(false)),
        )));
        let mut i: i32 = 0;
        'loop_: while (i < 20) {
            {
                let (__it, __inserted) = Map::emplace(
                    m.as_pointer(),
                    (
                        Rc::new(RefCell::new(Moveable::new_2({ i }, { (i as f64) }))),
                        Rc::new(RefCell::new(Moveable::new_2({ (i + 1) }, {
                            ((i as f64) + 1_f64)
                        }))),
                    ),
                );
                (
                    Rc::new(RefCell::new(__it)),
                    Rc::new(RefCell::new(__inserted)),
                )
            };
            i += 2;
        }
        assert!(({ ((*m.borrow()).len() == 10_usize) }));
        let mvkey1: Value<Moveable> = Rc::new(RefCell::new(Moveable::new_2({ 2 }, { 2_f64 })));
        let mv1: Value<Moveable> = Rc::new(RefCell::new(Moveable::new_2({ 4 }, { 4_f64 })));
        {
            r.as_pointer().write(CppMove::cpp_move(
                Rc::new(RefCell::new(
                    ({
                        let ___k: Ptr<Moveable> = mvkey1.as_pointer();
                        let ___args: Ptr<Moveable> = mv1.as_pointer();
                        (*m.borrow()).try_emplace(___k, ___args)
                    }),
                ))
                .as_pointer(),
            ));
            r.as_pointer()
        };
        assert!(({ ((*m.borrow()).len() == 10_usize) }));
        assert!(({ !(*(*r.borrow()).1.borrow()) }));
        assert!(!({ MoveableImpl::moved(&mv1.as_pointer(),) }));
        assert!(!({ MoveableImpl::moved(&mvkey1.as_pointer(),) }));
        assert!(
            ({
                ({
                    MoveableImpl::operator_eq(
                        &(*(*(*r.borrow()).0.borrow()).iter_deref().upgrade().deref())
                            .0
                            .as_pointer(),
                        mvkey1.as_pointer(),
                    )
                })
            })
        );
        let mvkey2: Value<Moveable> = Rc::new(RefCell::new(Moveable::new_2({ 3 }, { 3_f64 })));
        {
            r.as_pointer().write(CppMove::cpp_move(
                Rc::new(RefCell::new(
                    ({
                        let ___k: Ptr<Moveable> = mvkey2.as_pointer();
                        let ___args: Ptr<Moveable> = mv1.as_pointer();
                        (*m.borrow()).try_emplace(___k, ___args)
                    }),
                ))
                .as_pointer(),
            ));
            r.as_pointer()
        };
        assert!(({ ((*m.borrow()).len() == 11_usize) }));
        assert!(({ ((*(*r.borrow()).1.borrow()) as bool) }));
        assert!(({ MoveableImpl::moved(&mv1.as_pointer(),) }));
        assert!(({ MoveableImpl::moved(&mvkey2.as_pointer(),) }));
        assert!(
            ({
                (({
                    MoveableImpl::get(
                        &(*(*(*r.borrow()).0.borrow()).iter_deref().upgrade().deref())
                            .0
                            .as_pointer(),
                    )
                }) == 3)
            })
        );
        assert!(
            ({
                (({
                    MoveableImpl::get(
                        &(*(*(*r.borrow()).0.borrow()).iter_deref().upgrade().deref())
                            .1
                            .as_pointer(),
                    )
                }) == 4)
            })
        );
    }
    {
        let m: Value<Map<i32, (Value<i32>, Value<Moveable>)>> = Rc::new(RefCell::new(Map::new()));
        let r: Value<BidirectionalIterator<(Value<i32>, Value<Moveable>)>> =
            Rc::new(RefCell::new(BidirectionalIterator::default()));
        let i: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*i.borrow()) < 20) {
            {
                let __inserted =
                    m.as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<Moveable>)>| {
                            __m.get(&i.as_pointer()).is_none()
                        });
                if __inserted {
                    Map::subscript(m.as_pointer().clone(), i.as_pointer().clone()).write(
                        Moveable::new_2({ (*i.borrow()) }, { ((*i.borrow()) as f64) }),
                    );
                }
                (
                    Rc::new(RefCell::new(m.as_pointer().with(
                        |__m: &Map<i32, (Value<i32>, Value<Moveable>)>| __m.find(&i.as_pointer()),
                    ))),
                    Rc::new(RefCell::new(__inserted)),
                )
            };
            (*i.borrow_mut()) += 2;
        }
        assert!(({ ((*m.borrow()).len() == 10_usize) }));
        let it: Value<BidirectionalIterator<(Value<i32>, Value<Moveable>)>> = Rc::new(
            RefCell::new(
                <BidirectionalIterator<(Value<i32>, Value<Moveable>)>>::from(
                    ({
                        let __tmp_0: Value<i32> = Rc::new(RefCell::new(2));
                        m.as_pointer()
                            .with(|__m: &Map<i32, (Value<i32>, Value<Moveable>)>| {
                                __m.find(&__tmp_0.as_pointer())
                            })
                    }),
                ),
            ),
        );
        let mv1: Value<Moveable> = Rc::new(RefCell::new(Moveable::new_2({ 3 }, { 3_f64 })));
        let i: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*i.borrow()) < 20) {
            {
                let __a1 = ({
                    (*m.borrow()).try_emplace(
                        (*it.borrow()).clone(),
                        i.as_pointer(),
                        mv1.as_pointer(),
                    )
                });
                r.as_pointer().with_mut(|__it| *__it = __a1);
                r.as_pointer()
            };
            assert!(({ ((*m.borrow()).len() == 10_usize) }));
            assert!(!({ MoveableImpl::moved(&mv1.as_pointer(),) }));
            assert!(
                ({
                    ((*(*(*r.borrow()).iter_deref().upgrade().deref()).0.borrow()) == (*i.borrow()))
                })
            );
            assert!(
                ({
                    (({
                        MoveableImpl::get(
                            &(*(*r.borrow()).iter_deref().upgrade().deref())
                                .1
                                .as_pointer(),
                        )
                    }) == (*i.borrow()))
                })
            );
            (*i.borrow_mut()) += 2;
        }
        {
            let __a1 = ({
                let ___k: Value<i32> = Rc::new(RefCell::new(3));
                (*m.borrow()).try_emplace(
                    (*it.borrow()).clone(),
                    ___k.as_pointer(),
                    mv1.as_pointer(),
                )
            });
            r.as_pointer().with_mut(|__it| *__it = __a1);
            r.as_pointer()
        };
        assert!(({ ((*m.borrow()).len() == 11_usize) }));
        assert!(({ MoveableImpl::moved(&mv1.as_pointer(),) }));
        assert!(({ ((*(*(*r.borrow()).iter_deref().upgrade().deref()).0.borrow()) == 3) }));
        assert!(
            ({
                (({
                    MoveableImpl::get(
                        &(*(*r.borrow()).iter_deref().upgrade().deref())
                            .1
                            .as_pointer(),
                    )
                }) == 3)
            })
        );
    }
    {
        let m: Value<Map<Moveable, (Value<Moveable>, Value<Moveable>)>> =
            Rc::new(RefCell::new(Map::new()));
        let r: Value<BidirectionalIterator<(Value<Moveable>, Value<Moveable>)>> =
            Rc::new(RefCell::new(BidirectionalIterator::default()));
        let mut i: i32 = 0;
        'loop_: while (i < 20) {
            {
                let (__it, __inserted) = Map::emplace(
                    m.as_pointer(),
                    (
                        Rc::new(RefCell::new(Moveable::new_2({ i }, { (i as f64) }))),
                        Rc::new(RefCell::new(Moveable::new_2({ (i + 1) }, {
                            ((i as f64) + 1_f64)
                        }))),
                    ),
                );
                (
                    Rc::new(RefCell::new(__it)),
                    Rc::new(RefCell::new(__inserted)),
                )
            };
            i += 2;
        }
        assert!(({ ((*m.borrow()).len() == 10_usize) }));
        let it: Value<BidirectionalIterator<(Value<Moveable>, Value<Moveable>)>> =
            Rc::new(RefCell::new(libcc2rs::next_refcount(
                ({ ({ (*m.borrow()).cbegin() }) }),
                1_isize as isize,
            )));
        let mvkey1: Value<Moveable> = Rc::new(RefCell::new(Moveable::new_2({ 2 }, { 2_f64 })));
        let mv1: Value<Moveable> = Rc::new(RefCell::new(Moveable::new_2({ 4 }, { 4_f64 })));
        {
            let __a1 = ({
                let ___k: Ptr<Moveable> = mvkey1.as_pointer();
                let ___args: Ptr<Moveable> = mv1.as_pointer();
                (*m.borrow()).try_emplace((*it.borrow()).clone(), ___k, ___args)
            });
            r.as_pointer().with_mut(|__it| *__it = __a1);
            r.as_pointer()
        };
        assert!(({ ((*m.borrow()).len() == 10_usize) }));
        assert!(!({ MoveableImpl::moved(&mv1.as_pointer(),) }));
        assert!(!({ MoveableImpl::moved(&mvkey1.as_pointer(),) }));
        assert!(
            ({
                ({
                    MoveableImpl::operator_eq(
                        &(*(*r.borrow()).iter_deref().upgrade().deref())
                            .0
                            .as_pointer(),
                        mvkey1.as_pointer(),
                    )
                })
            })
        );
        let mvkey2: Value<Moveable> = Rc::new(RefCell::new(Moveable::new_2({ 3 }, { 3_f64 })));
        {
            let __a1 = ({
                let ___k: Ptr<Moveable> = mvkey2.as_pointer();
                let ___args: Ptr<Moveable> = mv1.as_pointer();
                (*m.borrow()).try_emplace((*it.borrow()).clone(), ___k, ___args)
            });
            r.as_pointer().with_mut(|__it| *__it = __a1);
            r.as_pointer()
        };
        assert!(({ ((*m.borrow()).len() == 11_usize) }));
        assert!(({ MoveableImpl::moved(&mv1.as_pointer(),) }));
        assert!(({ MoveableImpl::moved(&mvkey2.as_pointer(),) }));
        assert!(
            ({
                (({
                    MoveableImpl::get(
                        &(*(*r.borrow()).iter_deref().upgrade().deref())
                            .0
                            .as_pointer(),
                    )
                }) == 3)
            })
        );
        assert!(
            ({
                (({
                    MoveableImpl::get(
                        &(*(*r.borrow()).iter_deref().upgrade().deref())
                            .1
                            .as_pointer(),
                    )
                }) == 4)
            })
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
    ({ test_0() });
    return 0;
}
pub trait MoveableImpl {
    fn copy_assign(&self, _a0: Ptr<Moveable>) -> Ptr<Moveable> {
        unimplemented!()
    }
    fn move_assign(&self, x: Ptr<Moveable>) -> Ptr<Moveable>;
    fn operator_eq(&self, x: Ptr<Moveable>) -> bool;
    fn operator_lt(&self, x: Ptr<Moveable>) -> bool;
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
    fn get(&self) -> i32 {
        return (*self).with(|__s| __s.int_);
    }
    fn moved(&self) -> bool {
        return ((*self).with(|__s| __s.int_) == -1_i32);
    }
}
pub fn __cpp2rust_init_globals() {}
