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
        let m : Value<std_unordered_map_int__Moveable__std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__Moveable___ > = Rc::new(RefCell::new(<std_unordered_map_int__Moveable__std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__Moveable___  >::default() )) ;
        let r: Value<(
            Value<std___detail__Node_iterator_std_pair_const_int__Moveable___false__false_>,
            Value<bool>,
        )> = Rc::new(RefCell::new((
            Rc::new(RefCell::new(
                <std___detail__Node_iterator_std_pair_const_int__Moveable___false__false_>::default(
                ),
            )),
            Rc::new(RefCell::new(false)),
        )));
        let i: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*i.borrow()) < 20) {
            ({
                let ___args_0: Ptr<i32> = i.as_pointer();
                let ___args_1: Value<Moveable> =
                    Rc::new(RefCell::new(Moveable::new_2({ (*i.borrow()) }, {
                        ((*i.borrow()) as f64)
                    })));
                (*m.borrow()).emplace(___args_0, ___args_1.as_pointer())
            });
            (*i.borrow_mut()) += 2;
        }
        assert!(({ (({ (*m.borrow()).size() }) == 10_u64) }));
        let mv1: Value<Moveable> = Rc::new(RefCell::new(Moveable::new_2({ 3 }, { 3_f64 })));
        let i: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*i.borrow()) < 20) {
            {
                r.as_pointer().write(CppMove::cpp_move(
                    Rc::new(RefCell::new(
                        ({ (*m.borrow()).try_emplace(i.as_pointer(), mv1.as_pointer()) }),
                    ))
                    .as_pointer(),
                ));
                r.as_pointer()
            };
            assert!(({ (({ (*m.borrow()).size() }) == 10_u64) }));
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
        assert!(({ (({ (*m.borrow()).size() }) == 11_u64) }));
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
        assert!(({ (({ (*m.borrow()).size() }) == 12_u64) }));
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
        let mut mv3: Moveable = Moveable::new_2({ -1_i32 }, { 3_f64 });
        {
            r.as_pointer().write(CppMove::cpp_move(
                Rc::new(RefCell::new(
                    ({
                        let ___k: Value<i32> = Rc::new(RefCell::new(117));
                        (*m.borrow()).try_emplace(___k.as_pointer(), mv2.as_pointer())
                    }),
                ))
                .as_pointer(),
            ));
            r.as_pointer()
        };
        assert!(({ (({ (*m.borrow()).size() }) == 13_u64) }));
        assert!(({ ((*(*r.borrow()).1.borrow()) as bool) }));
        assert!(({ MoveableImpl::moved(&mv2.as_pointer(),) }));
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
        let m : Value<std_unordered_map_Moveable__Moveable__std_hash_Moveable___std_equal_to_Moveable___std_allocator_std_pair_const_Moveable__Moveable___ > = Rc::new(RefCell::new(<std_unordered_map_Moveable__Moveable__std_hash_Moveable___std_equal_to_Moveable___std_allocator_std_pair_const_Moveable__Moveable___  >::default() )) ;
        let r : Value<(Value<std___detail__Node_iterator_std_pair_const_Moveable__Moveable___false__true_  >, Value<bool  >) > = Rc::new(RefCell::new((
        Rc::new(RefCell::new(<std___detail__Node_iterator_std_pair_const_Moveable__Moveable___false__true_  >::default())),
        Rc::new(RefCell::new(false)),
    ) )) ;
        let mut i: i32 = 0;
        'loop_: while (i < 20) {
            ({
                let ___args_0: Value<Moveable> =
                    Rc::new(RefCell::new(Moveable::new_2({ i }, { (i as f64) })));
                let ___args_1: Value<Moveable> =
                    Rc::new(RefCell::new(Moveable::new_2({ (i + 1) }, {
                        ((i as f64) + 1_f64)
                    })));
                (*m.borrow()).emplace(___args_0.as_pointer(), ___args_1.as_pointer())
            });
            i += 2;
        }
        assert!(({ (({ (*m.borrow()).size() }) == 10_u64) }));
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
        assert!(({ (({ (*m.borrow()).size() }) == 10_u64) }));
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
        assert!(({ (({ (*m.borrow()).size() }) == 11_u64) }));
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
        let m : Value<std_unordered_map_int__Moveable__std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__Moveable___ > = Rc::new(RefCell::new(<std_unordered_map_int__Moveable__std_hash_int___std_equal_to_int___std_allocator_std_pair_const_int__Moveable___  >::default() )) ;
        let r: Value<std___detail__Node_iterator_std_pair_const_int__Moveable___false__false_> =
            Rc::new(RefCell::new(
                <std___detail__Node_iterator_std_pair_const_int__Moveable___false__false_>::default(
                ),
            ));
        let i: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*i.borrow()) < 20) {
            ({
                let ___k: Ptr<i32> = i.as_pointer();
                let ___args: Value<Moveable> =
                    Rc::new(RefCell::new(Moveable::new_2({ (*i.borrow()) }, {
                        ((*i.borrow()) as f64)
                    })));
                (*m.borrow()).try_emplace(___k, ___args.as_pointer())
            });
            (*i.borrow_mut()) += 2;
        }
        assert!(({ (({ (*m.borrow()).size() }) == 10_u64) }));
        let it: Value<
            std___detail__Node_const_iterator_std_pair_const_int__Moveable___false__false_,
        > = Rc::new(RefCell::new(
            <std___detail__Node_const_iterator_std_pair_const_int__Moveable___false__false_>::from(
                ({
                    let ___x: Value<i32> = Rc::new(RefCell::new(2));
                    (*m.borrow()).find_68(___x.as_pointer())
                }),
            ),
        ));
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
            assert!(({ (({ (*m.borrow()).size() }) == 10_u64) }));
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
        assert!(({ (({ (*m.borrow()).size() }) == 11_u64) }));
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
        let m : Value<std_unordered_map_Moveable__Moveable__std_hash_Moveable___std_equal_to_Moveable___std_allocator_std_pair_const_Moveable__Moveable___ > = Rc::new(RefCell::new(<std_unordered_map_Moveable__Moveable__std_hash_Moveable___std_equal_to_Moveable___std_allocator_std_pair_const_Moveable__Moveable___  >::default() )) ;
        let r : Value<std___detail__Node_iterator_std_pair_const_Moveable__Moveable___false__true_ > = Rc::new(RefCell::new(<std___detail__Node_iterator_std_pair_const_Moveable__Moveable___false__true_  >::default() )) ;
        let mut i: i32 = 0;
        'loop_: while (i < 20) {
            ({
                let ___args_0: Value<Moveable> =
                    Rc::new(RefCell::new(Moveable::new_2({ i }, { (i as f64) })));
                let ___args_1: Value<Moveable> =
                    Rc::new(RefCell::new(Moveable::new_2({ (i + 1) }, {
                        ((i as f64) + 1_f64)
                    })));
                (*m.borrow()).emplace(___args_0.as_pointer(), ___args_1.as_pointer())
            });
            i += 2;
        }
        assert!(({ (({ (*m.borrow()).size() }) == 10_u64) }));
        let it: Value<
            std___detail__Node_const_iterator_std_pair_const_Moveable__Moveable___false__true_,
        > = Rc::new(RefCell::new(libcc2rs::next_refcount(
            ({ ({ (*m.borrow()).cbegin_29() }) }),
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
        assert!(({ (({ (*m.borrow()).size() }) == 10_u64) }));
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
        assert!(({ (({ (*m.borrow()).size() }) == 11_u64) }));
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
        return ({ <std_hash_int_>::default()((*self).with(|__s| __s.int_)) })
            .wrapping_add(({ <std_hash_double_>::default()((*self).with(|__s| __s.double_)) }));
    }
    fn get(&self) -> i32 {
        return (*self).with(|__s| __s.int_);
    }
    fn moved(&self) -> bool {
        return ((*self).with(|__s| __s.int_) == -1_i32);
    }
}
pub fn __cpp2rust_init_globals() {}
