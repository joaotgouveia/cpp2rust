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
        let m : Value<std_map_int__Moveable__std_less_int___std_allocator_std_pair_const_int__Moveable___ > = Rc::new(RefCell::new(<std_map_int__Moveable__std_less_int___std_allocator_std_pair_const_int__Moveable___  >::default() )) ;
        let r: Value<std_pair_std__Rb_tree_iterator_std_pair_const_int__Moveable____bool_> =
            Rc::new(RefCell::new(
                <std_pair_std__Rb_tree_iterator_std_pair_const_int__Moveable____bool_>::default(),
            ));
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
        ({ cpp2rust_assert_fail_1((({ (*m.borrow()).size() }) == 10_u64)) });
        let i: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*i.borrow()) < 20) {
            let mv: Value<Moveable> =
                Rc::new(RefCell::new(Moveable::new_2({ ((*i.borrow()) + 1) }, {
                    (((*i.borrow()) + 1) as f64)
                })));
            (*r.borrow_mut()) =
                ({ (*m.borrow()).insert_or_assign(i.as_pointer(), mv.as_pointer()) });
            ({ cpp2rust_assert_fail_1((({ (*m.borrow()).size() }) == 10_u64)) });
            ({ cpp2rust_assert_fail_1(!({ (*r.borrow()).second })) });
            ({ cpp2rust_assert_fail_1(({ MoveableImpl::moved(&mv.as_pointer()) })) });
            ({ cpp2rust_assert_fail_1(({ ((*r.borrow()).first.read()).first } == (*i.borrow()))) });
            ({
                cpp2rust_assert_fail_1(
                    (({
                        MoveableImpl::get(&field_ptr!(field_ptr!(r.as_pointer(), first), second))
                    }) == ((*i.borrow()) + 1)),
                )
            });
            (*i.borrow_mut()) += 2;
        }
        let mv1: Value<Moveable> = Rc::new(RefCell::new(Moveable::new_2({ 5 }, { 5_f64 })));
        (*r.borrow_mut()) = ({
            let ___k: Value<i32> = Rc::new(RefCell::new(-1_i32));
            (*m.borrow()).insert_or_assign(___k.as_pointer(), mv1.as_pointer())
        });
        ({ cpp2rust_assert_fail_1((({ (*m.borrow()).size() }) == 11_u64)) });
        ({ cpp2rust_assert_fail_1(({ (*r.borrow()).second } as bool)) });
        ({ cpp2rust_assert_fail_1(({ MoveableImpl::moved(&mv1.as_pointer()) })) });
        ({ cpp2rust_assert_fail_1(({ ((*r.borrow()).first.read()).first } == -1_i32)) });
        ({
            cpp2rust_assert_fail_1(
                (({ MoveableImpl::get(&field_ptr!(field_ptr!(r.as_pointer(), first), second)) })
                    == 5),
            )
        });
        let mv2: Value<Moveable> = Rc::new(RefCell::new(Moveable::new_2({ 9 }, { 9_f64 })));
        (*r.borrow_mut()) = ({
            let ___k: Value<i32> = Rc::new(RefCell::new(3));
            (*m.borrow()).insert_or_assign(___k.as_pointer(), mv2.as_pointer())
        });
        ({ cpp2rust_assert_fail_1((({ (*m.borrow()).size() }) == 12_u64)) });
        ({ cpp2rust_assert_fail_1(({ (*r.borrow()).second } as bool)) });
        ({ cpp2rust_assert_fail_1(({ MoveableImpl::moved(&mv2.as_pointer()) })) });
        ({ cpp2rust_assert_fail_1(({ ((*r.borrow()).first.read()).first } == 3)) });
        ({
            cpp2rust_assert_fail_1(
                (({ MoveableImpl::get(&field_ptr!(field_ptr!(r.as_pointer(), first), second)) })
                    == 9),
            )
        });
        let mv3: Value<Moveable> = Rc::new(RefCell::new(Moveable::new_2({ -1_i32 }, { 5_f64 })));
        (*r.borrow_mut()) = ({
            let ___k: Value<i32> = Rc::new(RefCell::new(117));
            (*m.borrow()).insert_or_assign(___k.as_pointer(), mv3.as_pointer())
        });
        ({ cpp2rust_assert_fail_1((({ (*m.borrow()).size() }) == 13_u64)) });
        ({ cpp2rust_assert_fail_1(({ (*r.borrow()).second } as bool)) });
        ({ cpp2rust_assert_fail_1(({ MoveableImpl::moved(&mv3.as_pointer()) })) });
        ({ cpp2rust_assert_fail_1(({ ((*r.borrow()).first.read()).first } == 117)) });
        ({
            cpp2rust_assert_fail_1(
                (({ MoveableImpl::get(&field_ptr!(field_ptr!(r.as_pointer(), first), second)) })
                    == -1_i32),
            )
        });
    }
    {
        let m : Value<std_map_Moveable__Moveable__std_less_Moveable___std_allocator_std_pair_const_Moveable__Moveable___ > = Rc::new(RefCell::new(<std_map_Moveable__Moveable__std_less_Moveable___std_allocator_std_pair_const_Moveable__Moveable___  >::default() )) ;
        let r : Value<std_pair_std__Rb_tree_iterator_std_pair_const_Moveable__Moveable____bool_ > = Rc::new(RefCell::new(<std_pair_std__Rb_tree_iterator_std_pair_const_Moveable__Moveable____bool_  >::default() )) ;
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
        ({ cpp2rust_assert_fail_1((({ (*m.borrow()).size() }) == 10_u64)) });
        let mvkey1: Value<Moveable> = Rc::new(RefCell::new(Moveable::new_2({ 2 }, { 2_f64 })));
        let mv1: Value<Moveable> = Rc::new(RefCell::new(Moveable::new_2({ 4 }, { 4_f64 })));
        (*r.borrow_mut()) = ({
            let ___k: Ptr<Moveable> = mvkey1.as_pointer();
            let ___obj: Ptr<Moveable> = mv1.as_pointer();
            (*m.borrow()).insert_or_assign(___k, ___obj)
        });
        ({ cpp2rust_assert_fail_1((({ (*m.borrow()).size() }) == 10_u64)) });
        ({ cpp2rust_assert_fail_1(!({ (*r.borrow()).second })) });
        ({ cpp2rust_assert_fail_1(!({ MoveableImpl::moved(&mvkey1.as_pointer()) })) });
        ({ cpp2rust_assert_fail_1(({ MoveableImpl::moved(&mv1.as_pointer()) })) });
        ({
            cpp2rust_assert_fail_1(
                ({
                    MoveableImpl::operator_eq(
                        &field_ptr!(field_ptr!(r.as_pointer(), first), first),
                        mvkey1.as_pointer(),
                    )
                }),
            )
        });
        ({
            cpp2rust_assert_fail_1(
                (({ MoveableImpl::get(&field_ptr!(field_ptr!(r.as_pointer(), first), second)) })
                    == 4),
            )
        });
        let mvkey2: Value<Moveable> = Rc::new(RefCell::new(Moveable::new_2({ 3 }, { 3_f64 })));
        let mv2: Value<Moveable> = Rc::new(RefCell::new(Moveable::new_2({ 5 }, { 5_f64 })));
        (*r.borrow_mut()) = ({
            let ___k: Ptr<Moveable> = mvkey2.as_pointer();
            let ___args: Ptr<Moveable> = mv2.as_pointer();
            (*m.borrow()).try_emplace(___k, ___args)
        });
        ({ cpp2rust_assert_fail_1((({ (*m.borrow()).size() }) == 11_u64)) });
        ({ cpp2rust_assert_fail_1(({ (*r.borrow()).second } as bool)) });
        ({ cpp2rust_assert_fail_1(({ MoveableImpl::moved(&mv2.as_pointer()) })) });
        ({ cpp2rust_assert_fail_1(({ MoveableImpl::moved(&mvkey2.as_pointer()) })) });
        ({
            cpp2rust_assert_fail_1(
                (({ MoveableImpl::get(&field_ptr!(field_ptr!(r.as_pointer(), first), first)) })
                    == 3),
            )
        });
        ({
            cpp2rust_assert_fail_1(
                (({ MoveableImpl::get(&field_ptr!(field_ptr!(r.as_pointer(), first), second)) })
                    == 5),
            )
        });
    }
    {
        let m : Value<std_map_int__Moveable__std_less_int___std_allocator_std_pair_const_int__Moveable___ > = Rc::new(RefCell::new(<std_map_int__Moveable__std_less_int___std_allocator_std_pair_const_int__Moveable___  >::default() )) ;
        let r: Value<std__Rb_tree_iterator_std_pair_const_int__Moveable__> = Rc::new(RefCell::new(
            <std__Rb_tree_iterator_std_pair_const_int__Moveable__>::default(),
        ));
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
        ({ cpp2rust_assert_fail_1((({ (*m.borrow()).size() }) == 10_u64)) });
        let it: Value<std__Rb_tree_const_iterator_std_pair_const_int__Moveable__> =
            Rc::new(RefCell::new({
                let __tmp_0: Value<std__Rb_tree_iterator_std_pair_const_int__Moveable__> =
                    Rc::new(RefCell::new(
                        ({
                            let ___x: Value<i32> = Rc::new(RefCell::new(2));
                            (*m.borrow()).find_74(___x.as_pointer())
                        }),
                    ));
                std__Rb_tree_const_iterator_std_pair_const_int__Moveable__::new_1({
                    __tmp_0.as_pointer()
                })
            }));
        let mv1: Value<Moveable> = Rc::new(RefCell::new(Moveable::new_2({ 3 }, { 3_f64 })));
        let key1: Value<i32> = Rc::new(RefCell::new(2));
        (*r.borrow_mut()) = ({
            (*m.borrow()).insert_or_assign(
                (*it.borrow()).clone(),
                key1.as_pointer(),
                mv1.as_pointer(),
            )
        });
        ({ cpp2rust_assert_fail_1((({ (*m.borrow()).size() }) == 10_u64)) });
        ({ cpp2rust_assert_fail_1(({ MoveableImpl::moved(&mv1.as_pointer()) })) });
        ({ cpp2rust_assert_fail_1(({ ((*r.borrow()).read()).first } == 2)) });
        ({
            cpp2rust_assert_fail_1(
                (({ MoveableImpl::get(&field_ptr!(r.as_pointer(), second)) }) == 3),
            )
        });
        let mv2: Value<Moveable> = Rc::new(RefCell::new(Moveable::new_2({ 5 }, { 5_f64 })));
        let key2: Value<i32> = Rc::new(RefCell::new(3));
        (*r.borrow_mut()) = ({
            (*m.borrow()).insert_or_assign(
                (*it.borrow()).clone(),
                key2.as_pointer(),
                mv2.as_pointer(),
            )
        });
        ({ cpp2rust_assert_fail_1((({ (*m.borrow()).size() }) == 11_u64)) });
        ({ cpp2rust_assert_fail_1(({ MoveableImpl::moved(&mv2.as_pointer()) })) });
        ({ cpp2rust_assert_fail_1(({ ((*r.borrow()).read()).first } == 3)) });
        ({
            cpp2rust_assert_fail_1(
                (({ MoveableImpl::get(&field_ptr!(r.as_pointer(), second)) }) == 5),
            )
        });
        let mv3: Value<Moveable> = Rc::new(RefCell::new(Moveable::new_2({ 7 }, { 7_f64 })));
        let key3: Value<i32> = Rc::new(RefCell::new(4));
        (*r.borrow_mut()) = ({
            let ___hint: std__Rb_tree_const_iterator_std_pair_const_int__Moveable__ = {
                let __tmp_1: Value<std__Rb_tree_iterator_std_pair_const_int__Moveable__> =
                    Rc::new(RefCell::new(({ (*m.borrow()).begin_20() })));
                std__Rb_tree_const_iterator_std_pair_const_int__Moveable__::new_1({
                    __tmp_1.as_pointer()
                })
            };
            (*m.borrow()).insert_or_assign(___hint, key3.as_pointer(), mv3.as_pointer())
        });
        ({ cpp2rust_assert_fail_1((({ (*m.borrow()).size() }) == 11_u64)) });
        ({ cpp2rust_assert_fail_1(({ MoveableImpl::moved(&mv3.as_pointer()) })) });
        ({ cpp2rust_assert_fail_1(({ ((*r.borrow()).read()).first } == 4)) });
        ({
            cpp2rust_assert_fail_1(
                (({ MoveableImpl::get(&field_ptr!(r.as_pointer(), second)) }) == 7),
            )
        });
        let mv4: Value<Moveable> = Rc::new(RefCell::new(Moveable::new_2({ 9 }, { 9_f64 })));
        let key4: Value<i32> = Rc::new(RefCell::new(5));
        (*r.borrow_mut()) = ({
            let ___hint: std__Rb_tree_const_iterator_std_pair_const_int__Moveable__ = {
                let __tmp_2: Value<std__Rb_tree_iterator_std_pair_const_int__Moveable__> =
                    Rc::new(RefCell::new(({ (*m.borrow()).begin_20() })));
                std__Rb_tree_const_iterator_std_pair_const_int__Moveable__::new_1({
                    __tmp_2.as_pointer()
                })
            };
            (*m.borrow()).insert_or_assign(___hint, key4.as_pointer(), mv4.as_pointer())
        });
        ({ cpp2rust_assert_fail_1((({ (*m.borrow()).size() }) == 12_u64)) });
        ({ cpp2rust_assert_fail_1(({ MoveableImpl::moved(&mv4.as_pointer()) })) });
        ({ cpp2rust_assert_fail_1(({ ((*r.borrow()).read()).first } == 5)) });
        ({
            cpp2rust_assert_fail_1(
                (({ MoveableImpl::get(&field_ptr!(r.as_pointer(), second)) }) == 9),
            )
        });
        let mv5: Value<Moveable> = Rc::new(RefCell::new(Moveable::new_2({ 11 }, { 11_f64 })));
        let key5: Value<i32> = Rc::new(RefCell::new(6));
        (*r.borrow_mut()) = ({
            let ___hint: std__Rb_tree_const_iterator_std_pair_const_int__Moveable__ = {
                let __tmp_3: Value<std__Rb_tree_iterator_std_pair_const_int__Moveable__> =
                    Rc::new(RefCell::new(({ (*m.borrow()).end_22() })));
                std__Rb_tree_const_iterator_std_pair_const_int__Moveable__::new_1({
                    __tmp_3.as_pointer()
                })
            };
            (*m.borrow()).insert_or_assign(___hint, key5.as_pointer(), mv5.as_pointer())
        });
        ({ cpp2rust_assert_fail_1((({ (*m.borrow()).size() }) == 12_u64)) });
        ({ cpp2rust_assert_fail_1(({ MoveableImpl::moved(&mv5.as_pointer()) })) });
        ({ cpp2rust_assert_fail_1(({ ((*r.borrow()).read()).first } == 6)) });
        ({
            cpp2rust_assert_fail_1(
                (({ MoveableImpl::get(&field_ptr!(r.as_pointer(), second)) }) == 11),
            )
        });
        let mv6: Value<Moveable> = Rc::new(RefCell::new(Moveable::new_2({ 13 }, { 13_f64 })));
        let key6: Value<i32> = Rc::new(RefCell::new(7));
        (*r.borrow_mut()) = ({
            let ___hint: std__Rb_tree_const_iterator_std_pair_const_int__Moveable__ = {
                let __tmp_4: Value<std__Rb_tree_iterator_std_pair_const_int__Moveable__> =
                    Rc::new(RefCell::new(({ (*m.borrow()).end_22() })));
                std__Rb_tree_const_iterator_std_pair_const_int__Moveable__::new_1({
                    __tmp_4.as_pointer()
                })
            };
            (*m.borrow()).insert_or_assign(___hint, key6.as_pointer(), mv6.as_pointer())
        });
        ({ cpp2rust_assert_fail_1((({ (*m.borrow()).size() }) == 13_u64)) });
        ({ cpp2rust_assert_fail_1(({ MoveableImpl::moved(&mv6.as_pointer()) })) });
        ({ cpp2rust_assert_fail_1(({ ((*r.borrow()).read()).first } == 7)) });
        ({
            cpp2rust_assert_fail_1(
                (({ MoveableImpl::get(&field_ptr!(r.as_pointer(), second)) }) == 13),
            )
        });
        let mv7: Value<Moveable> = Rc::new(RefCell::new(Moveable::new_2({ 15 }, { 15_f64 })));
        let key7: Value<i32> = Rc::new(RefCell::new(8));
        (*r.borrow_mut()) = ({
            let ___hint: std__Rb_tree_const_iterator_std_pair_const_int__Moveable__ = {
                let __tmp_5: Value<std__Rb_tree_iterator_std_pair_const_int__Moveable__> = Rc::new(
                    RefCell::new(({ next_2(({ (*m.borrow()).begin_20() }), 2_i64) })),
                );
                std__Rb_tree_const_iterator_std_pair_const_int__Moveable__::new_1({
                    __tmp_5.as_pointer()
                })
            };
            (*m.borrow()).insert_or_assign(___hint, key7.as_pointer(), mv7.as_pointer())
        });
        ({ cpp2rust_assert_fail_1((({ (*m.borrow()).size() }) == 13_u64)) });
        ({ cpp2rust_assert_fail_1(({ MoveableImpl::moved(&mv7.as_pointer()) })) });
        ({ cpp2rust_assert_fail_1(({ ((*r.borrow()).read()).first } == 8)) });
        ({
            cpp2rust_assert_fail_1(
                (({ MoveableImpl::get(&field_ptr!(r.as_pointer(), second)) }) == 15),
            )
        });
        let mv8: Value<Moveable> = Rc::new(RefCell::new(Moveable::new_2({ 17 }, { 17_f64 })));
        let key8: Value<i32> = Rc::new(RefCell::new(9));
        (*r.borrow_mut()) = ({
            let ___hint: std__Rb_tree_const_iterator_std_pair_const_int__Moveable__ = {
                let __tmp_6: Value<std__Rb_tree_iterator_std_pair_const_int__Moveable__> = Rc::new(
                    RefCell::new(({ next_2(({ (*m.borrow()).begin_20() }), 2_i64) })),
                );
                std__Rb_tree_const_iterator_std_pair_const_int__Moveable__::new_1({
                    __tmp_6.as_pointer()
                })
            };
            (*m.borrow()).insert_or_assign(___hint, key8.as_pointer(), mv8.as_pointer())
        });
        ({ cpp2rust_assert_fail_1((({ (*m.borrow()).size() }) == 14_u64)) });
        ({ cpp2rust_assert_fail_1(({ MoveableImpl::moved(&mv8.as_pointer()) })) });
        ({ cpp2rust_assert_fail_1(({ ((*r.borrow()).read()).first } == 9)) });
        ({
            cpp2rust_assert_fail_1(
                (({ MoveableImpl::get(&field_ptr!(r.as_pointer(), second)) }) == 17),
            )
        });
    }
    {
        let m : Value<std_map_Moveable__Moveable__std_less_Moveable___std_allocator_std_pair_const_Moveable__Moveable___ > = Rc::new(RefCell::new(<std_map_Moveable__Moveable__std_less_Moveable___std_allocator_std_pair_const_Moveable__Moveable___  >::default() )) ;
        let r: Value<std__Rb_tree_iterator_std_pair_const_Moveable__Moveable__> = Rc::new(
            RefCell::new(<std__Rb_tree_iterator_std_pair_const_Moveable__Moveable__>::default()),
        );
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
        ({ cpp2rust_assert_fail_1((({ (*m.borrow()).size() }) == 10_u64)) });
        let it: Value<std__Rb_tree_const_iterator_std_pair_const_Moveable__Moveable__> = Rc::new(
            RefCell::new(({ next_3(({ (*m.borrow()).cbegin() }), None) })),
        );
        let mvkey1: Value<Moveable> = Rc::new(RefCell::new(Moveable::new_2({ 2 }, { 2_f64 })));
        let mv1: Value<Moveable> = Rc::new(RefCell::new(Moveable::new_2({ 4 }, { 4_f64 })));
        (*r.borrow_mut()) = ({
            let ___k: Ptr<Moveable> = mvkey1.as_pointer();
            let ___obj: Ptr<Moveable> = mv1.as_pointer();
            (*m.borrow()).insert_or_assign((*it.borrow()).clone(), ___k, ___obj)
        });
        ({ cpp2rust_assert_fail_1((({ (*m.borrow()).size() }) == 10_u64)) });
        ({ cpp2rust_assert_fail_1(({ MoveableImpl::moved(&mv1.as_pointer()) })) });
        ({ cpp2rust_assert_fail_1(!({ MoveableImpl::moved(&mvkey1.as_pointer()) })) });
        ({
            cpp2rust_assert_fail_1(
                ({
                    MoveableImpl::operator_eq(
                        &field_ptr!(r.as_pointer(), first),
                        mvkey1.as_pointer(),
                    )
                }),
            )
        });
        ({
            cpp2rust_assert_fail_1(
                (({ MoveableImpl::get(&field_ptr!(r.as_pointer(), second)) }) == 4),
            )
        });
        let mvkey2: Value<Moveable> = Rc::new(RefCell::new(Moveable::new_2({ 3 }, { 3_f64 })));
        let mv2: Value<Moveable> = Rc::new(RefCell::new(Moveable::new_2({ 5 }, { 5_f64 })));
        (*r.borrow_mut()) = ({
            let ___k: Ptr<Moveable> = mvkey2.as_pointer();
            let ___obj: Ptr<Moveable> = mv2.as_pointer();
            (*m.borrow()).insert_or_assign((*it.borrow()).clone(), ___k, ___obj)
        });
        ({ cpp2rust_assert_fail_1((({ (*m.borrow()).size() }) == 11_u64)) });
        ({ cpp2rust_assert_fail_1(({ MoveableImpl::moved(&mv2.as_pointer()) })) });
        ({ cpp2rust_assert_fail_1(({ MoveableImpl::moved(&mvkey2.as_pointer()) })) });
        ({
            cpp2rust_assert_fail_1(
                (({ MoveableImpl::get(&field_ptr!(r.as_pointer(), first)) }) == 3),
            )
        });
        ({
            cpp2rust_assert_fail_1(
                (({ MoveableImpl::get(&field_ptr!(r.as_pointer(), second)) }) == 5),
            )
        });
        let mvkey3: Value<Moveable> = Rc::new(RefCell::new(Moveable::new_2({ 6 }, { 6_f64 })));
        let mv3: Value<Moveable> = Rc::new(RefCell::new(Moveable::new_2({ 8 }, { 8_f64 })));
        (*r.borrow_mut()) = ({
            let ___hint: std__Rb_tree_const_iterator_std_pair_const_Moveable__Moveable__ = {
                let __tmp_7: Value<std__Rb_tree_iterator_std_pair_const_Moveable__Moveable__> =
                    Rc::new(RefCell::new(({ (*m.borrow()).begin_20() })));
                std__Rb_tree_const_iterator_std_pair_const_Moveable__Moveable__::new_1({
                    __tmp_7.as_pointer()
                })
            };
            let ___k: Ptr<Moveable> = mvkey3.as_pointer();
            let ___obj: Ptr<Moveable> = mv3.as_pointer();
            (*m.borrow()).insert_or_assign(___hint, ___k, ___obj)
        });
        ({ cpp2rust_assert_fail_1((({ (*m.borrow()).size() }) == 11_u64)) });
        ({ cpp2rust_assert_fail_1(({ MoveableImpl::moved(&mv3.as_pointer()) })) });
        ({ cpp2rust_assert_fail_1(!({ MoveableImpl::moved(&mvkey3.as_pointer()) })) });
        ({
            cpp2rust_assert_fail_1(
                ({
                    MoveableImpl::operator_eq(
                        &field_ptr!(r.as_pointer(), first),
                        mvkey3.as_pointer(),
                    )
                }),
            )
        });
        ({
            cpp2rust_assert_fail_1(
                (({ MoveableImpl::get(&field_ptr!(r.as_pointer(), second)) }) == 8),
            )
        });
        let mvkey4: Value<Moveable> = Rc::new(RefCell::new(Moveable::new_2({ 7 }, { 7_f64 })));
        let mv4: Value<Moveable> = Rc::new(RefCell::new(Moveable::new_2({ 9 }, { 9_f64 })));
        (*r.borrow_mut()) = ({
            let ___hint: std__Rb_tree_const_iterator_std_pair_const_Moveable__Moveable__ = {
                let __tmp_8: Value<std__Rb_tree_iterator_std_pair_const_Moveable__Moveable__> =
                    Rc::new(RefCell::new(({ (*m.borrow()).begin_20() })));
                std__Rb_tree_const_iterator_std_pair_const_Moveable__Moveable__::new_1({
                    __tmp_8.as_pointer()
                })
            };
            let ___k: Ptr<Moveable> = mvkey4.as_pointer();
            let ___obj: Ptr<Moveable> = mv4.as_pointer();
            (*m.borrow()).insert_or_assign(___hint, ___k, ___obj)
        });
        ({ cpp2rust_assert_fail_1((({ (*m.borrow()).size() }) == 12_u64)) });
        ({ cpp2rust_assert_fail_1(({ MoveableImpl::moved(&mv4.as_pointer()) })) });
        ({ cpp2rust_assert_fail_1(({ MoveableImpl::moved(&mvkey4.as_pointer()) })) });
        ({
            cpp2rust_assert_fail_1(
                (({ MoveableImpl::get(&field_ptr!(r.as_pointer(), first)) }) == 7),
            )
        });
        ({
            cpp2rust_assert_fail_1(
                (({ MoveableImpl::get(&field_ptr!(r.as_pointer(), second)) }) == 9),
            )
        });
        let mvkey5: Value<Moveable> = Rc::new(RefCell::new(Moveable::new_2({ 8 }, { 8_f64 })));
        let mv5: Value<Moveable> = Rc::new(RefCell::new(Moveable::new_2({ 10 }, { 10_f64 })));
        (*r.borrow_mut()) = ({
            let ___hint: std__Rb_tree_const_iterator_std_pair_const_Moveable__Moveable__ = {
                let __tmp_9: Value<std__Rb_tree_iterator_std_pair_const_Moveable__Moveable__> =
                    Rc::new(RefCell::new(({ (*m.borrow()).end_22() })));
                std__Rb_tree_const_iterator_std_pair_const_Moveable__Moveable__::new_1({
                    __tmp_9.as_pointer()
                })
            };
            let ___k: Ptr<Moveable> = mvkey5.as_pointer();
            let ___obj: Ptr<Moveable> = mv5.as_pointer();
            (*m.borrow()).insert_or_assign(___hint, ___k, ___obj)
        });
        ({ cpp2rust_assert_fail_1((({ (*m.borrow()).size() }) == 12_u64)) });
        ({ cpp2rust_assert_fail_1(({ MoveableImpl::moved(&mv5.as_pointer()) })) });
        ({ cpp2rust_assert_fail_1(!({ MoveableImpl::moved(&mvkey5.as_pointer()) })) });
        ({
            cpp2rust_assert_fail_1(
                ({
                    MoveableImpl::operator_eq(
                        &field_ptr!(r.as_pointer(), first),
                        mvkey5.as_pointer(),
                    )
                }),
            )
        });
        ({
            cpp2rust_assert_fail_1(
                (({ MoveableImpl::get(&field_ptr!(r.as_pointer(), second)) }) == 10),
            )
        });
        let mvkey6: Value<Moveable> = Rc::new(RefCell::new(Moveable::new_2({ 9 }, { 9_f64 })));
        let mv6: Value<Moveable> = Rc::new(RefCell::new(Moveable::new_2({ 11 }, { 11_f64 })));
        (*r.borrow_mut()) = ({
            let ___hint: std__Rb_tree_const_iterator_std_pair_const_Moveable__Moveable__ = {
                let __tmp_10: Value<std__Rb_tree_iterator_std_pair_const_Moveable__Moveable__> =
                    Rc::new(RefCell::new(({ (*m.borrow()).end_22() })));
                std__Rb_tree_const_iterator_std_pair_const_Moveable__Moveable__::new_1({
                    __tmp_10.as_pointer()
                })
            };
            let ___k: Ptr<Moveable> = mvkey6.as_pointer();
            let ___obj: Ptr<Moveable> = mv6.as_pointer();
            (*m.borrow()).insert_or_assign(___hint, ___k, ___obj)
        });
        ({ cpp2rust_assert_fail_1((({ (*m.borrow()).size() }) == 13_u64)) });
        ({ cpp2rust_assert_fail_1(({ MoveableImpl::moved(&mv6.as_pointer()) })) });
        ({ cpp2rust_assert_fail_1(({ MoveableImpl::moved(&mvkey6.as_pointer()) })) });
        ({
            cpp2rust_assert_fail_1(
                (({ MoveableImpl::get(&field_ptr!(r.as_pointer(), first)) }) == 9),
            )
        });
        ({
            cpp2rust_assert_fail_1(
                (({ MoveableImpl::get(&field_ptr!(r.as_pointer(), second)) }) == 11),
            )
        });
        let mvkey7: Value<Moveable> = Rc::new(RefCell::new(Moveable::new_2({ 10 }, { 10_f64 })));
        let mv7: Value<Moveable> = Rc::new(RefCell::new(Moveable::new_2({ 12 }, { 12_f64 })));
        (*r.borrow_mut()) = ({
            let ___hint: std__Rb_tree_const_iterator_std_pair_const_Moveable__Moveable__ = {
                let __tmp_11: Value<std__Rb_tree_iterator_std_pair_const_Moveable__Moveable__> =
                    Rc::new(RefCell::new(
                        ({ next_4(({ (*m.borrow()).begin_20() }), 2_i64) }),
                    ));
                std__Rb_tree_const_iterator_std_pair_const_Moveable__Moveable__::new_1({
                    __tmp_11.as_pointer()
                })
            };
            let ___k: Ptr<Moveable> = mvkey7.as_pointer();
            let ___obj: Ptr<Moveable> = mv7.as_pointer();
            (*m.borrow()).insert_or_assign(___hint, ___k, ___obj)
        });
        ({ cpp2rust_assert_fail_1((({ (*m.borrow()).size() }) == 13_u64)) });
        ({ cpp2rust_assert_fail_1(({ MoveableImpl::moved(&mv7.as_pointer()) })) });
        ({ cpp2rust_assert_fail_1(!({ MoveableImpl::moved(&mvkey7.as_pointer()) })) });
        ({
            cpp2rust_assert_fail_1(
                ({
                    MoveableImpl::operator_eq(
                        &field_ptr!(r.as_pointer(), first),
                        mvkey7.as_pointer(),
                    )
                }),
            )
        });
        ({
            cpp2rust_assert_fail_1(
                (({ MoveableImpl::get(&field_ptr!(r.as_pointer(), second)) }) == 12),
            )
        });
        let mvkey8: Value<Moveable> = Rc::new(RefCell::new(Moveable::new_2({ 11 }, { 11_f64 })));
        let mv8: Value<Moveable> = Rc::new(RefCell::new(Moveable::new_2({ 13 }, { 13_f64 })));
        (*r.borrow_mut()) = ({
            let ___hint: std__Rb_tree_const_iterator_std_pair_const_Moveable__Moveable__ = {
                let __tmp_12: Value<std__Rb_tree_iterator_std_pair_const_Moveable__Moveable__> =
                    Rc::new(RefCell::new(
                        ({ next_4(({ (*m.borrow()).begin_20() }), 2_i64) }),
                    ));
                std__Rb_tree_const_iterator_std_pair_const_Moveable__Moveable__::new_1({
                    __tmp_12.as_pointer()
                })
            };
            let ___k: Ptr<Moveable> = mvkey8.as_pointer();
            let ___obj: Ptr<Moveable> = mv8.as_pointer();
            (*m.borrow()).insert_or_assign(___hint, ___k, ___obj)
        });
        ({ cpp2rust_assert_fail_1((({ (*m.borrow()).size() }) == 14_u64)) });
        ({ cpp2rust_assert_fail_1(({ MoveableImpl::moved(&mv8.as_pointer()) })) });
        ({ cpp2rust_assert_fail_1(({ MoveableImpl::moved(&mvkey8.as_pointer()) })) });
        ({
            cpp2rust_assert_fail_1(
                (({ MoveableImpl::get(&field_ptr!(r.as_pointer(), first)) }) == 11),
            )
        });
        ({
            cpp2rust_assert_fail_1(
                (({ MoveableImpl::get(&field_ptr!(r.as_pointer(), second)) }) == 13),
            )
        });
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
