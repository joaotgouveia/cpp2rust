extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(40)]
pub struct test_allocator_statistics {
    #[offset(0)]
    pub time_to_throw: i32,
    #[offset(4)]
    pub throw_after: i32,
    #[offset(8)]
    pub count: i32,
    #[offset(12)]
    pub alloc_count: i32,
    #[offset(16)]
    pub allocated_size: i32,
    #[offset(20)]
    pub construct_count: i32,
    #[offset(24)]
    pub destroy_count: i32,
    #[offset(28)]
    pub copied: i32,
    #[offset(32)]
    pub moved: i32,
    #[offset(36)]
    pub converted: i32,
}
impl CppMove for test_allocator_statistics {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for test_allocator_statistics {
    fn default() -> Self {
        {
            test_allocator_statistics {
                time_to_throw: 0,
                throw_after: 2147483647,
                count: 0,
                alloc_count: 0,
                allocated_size: 0,
                construct_count: 0,
                destroy_count: 0,
                copied: 0,
                moved: 0,
                converted: 0,
            }
        }
    }
}
thread_local!(
    pub static destructed_value_0: Value<i32> = Rc::new(RefCell::new(-1_i32));
);
thread_local!(
    pub static moved_value_1: Value<i32> = Rc::new(RefCell::new(2147483647));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_alloc_base {}
impl CppMove for test_alloc_base {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn operator_eq_2(x: Ptr<test_allocator_void_>, y: Ptr<test_allocator_void_>) -> bool {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
pub fn operator_ne_3(x: Ptr<test_allocator_void_>, y: Ptr<test_allocator_void_>) -> bool {
    return !({
        let _x: Ptr<test_allocator_void_> = (x).clone();
        let _y: Ptr<test_allocator_void_> = (y).clone();
        operator_eq_2(_x, _y)
    });
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CopyAssign, Destructor)]
#[byte_size(16)]
pub struct test_allocator_void_ {
    #[offset(0)]
    data_: i32,
    #[offset(4)]
    id_: i32,
    #[offset(8)]
    #[byte_size(8)]
    stats_: Ptr<test_allocator_statistics>,
}
impl test_allocator_void_ {
    pub fn new_1(mut stats: Ptr<test_allocator_statistics>) -> Self {
        Self {
            data_: 0,
            id_: 0,
            stats_: (stats).clone(),
        }
    }
    pub fn new_2(mut data: i32) -> Self {
        Self {
            data_: data,
            id_: 0,
            stats_: Ptr::<test_allocator_statistics>::null(),
        }
    }
    pub fn new_3(mut data: i32, mut stats: Ptr<test_allocator_statistics>) -> Self {
        Self {
            data_: data,
            id_: 0,
            stats_: (stats).clone(),
        }
    }
    pub fn new_4(mut data: i32, mut id: i32) -> Self {
        Self {
            data_: data,
            id_: id,
            stats_: Ptr::<test_allocator_statistics>::null(),
        }
    }
    pub fn new_5(mut data: i32, mut id: i32, mut stats: Ptr<test_allocator_statistics>) -> Self {
        Self {
            data_: data,
            id_: id,
            stats_: (stats).clone(),
        }
    }
    pub fn copy_from(a: Ptr<test_allocator_void_>) -> Self {
        Self {
            data_: a.with(|__s| __s.data_),
            id_: a.with(|__s| __s.id_),
            stats_: a.with(|__s| __s.stats_.clone()),
        }
    }
}
impl CppEq for test_allocator_void_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_2(__lhs.clone(), __rhs.clone())
    }
}
impl Clone for test_allocator_void_ {
    fn clone(&self) -> Self {
        let __src: Value<test_allocator_void_> = Rc::new(RefCell::new(test_allocator_void_ {
            data_: self.data_.clone(),
            id_: self.id_.clone(),
            stats_: self.stats_.clone(),
        }));
        test_allocator_void_::copy_from(__src.as_pointer())
    }
}
impl CppMove for test_allocator_void_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for test_allocator_void_ {
    fn default() -> Self {
        {
            test_allocator_void_ {
                data_: 0,
                id_: 0,
                stats_: Ptr::<test_allocator_statistics>::null(),
            }
        }
    }
}
pub fn operator_eq_4(x: Ptr<test_allocator_MoveOnly_>, y: Ptr<test_allocator_MoveOnly_>) -> bool {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_allocator_MoveOnly__rebind_MoveOnly_ {}
impl CppMove for test_allocator_MoveOnly__rebind_MoveOnly_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Record, ByteRepr, FnPtrArg, Destructor)]
#[byte_size(16)]
pub struct test_allocator_MoveOnly_ {
    #[offset(0)]
    data_: i32,
    #[offset(4)]
    id_: i32,
    #[offset(8)]
    #[byte_size(8)]
    stats_: Ptr<test_allocator_statistics>,
}
impl test_allocator_MoveOnly_ {
    pub fn new_1(mut data: i32) -> Self {
        Self {
            data_: data,
            id_: 0,
            stats_: Ptr::<test_allocator_statistics>::null(),
        }
    }
}
impl CppEq for test_allocator_MoveOnly_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_4(__lhs.clone(), __rhs.clone())
    }
}
impl Default for test_allocator_MoveOnly_ {
    fn default() -> Self {
        {
            test_allocator_MoveOnly_ {
                data_: 0_i32,
                id_: 0,
                stats_: Ptr::<test_allocator_statistics>::null(),
            }
        }
    }
}
pub fn operator_eq_5(x: Ptr<other_allocator_MoveOnly_>, y: Ptr<other_allocator_MoveOnly_>) -> bool {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct other_allocator_MoveOnly_ {
    #[offset(0)]
    data_: i32,
}
impl other_allocator_MoveOnly_ {
    pub fn new_1(mut i: i32) -> Self {
        Self { data_: i }
    }
}
impl CppEq for other_allocator_MoveOnly_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_5(__lhs.clone(), __rhs.clone())
    }
}
impl CppMove for other_allocator_MoveOnly_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for other_allocator_MoveOnly_ {
    fn default() -> Self {
        { other_allocator_MoveOnly_ { data_: 0_i32 } }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Ctor_Tag {}
impl CppMove for Ctor_Tag {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Record, ByteRepr, FnPtrArg, CppFrom, CopyAssign, MoveAssign, Default)]
#[cpp_from(Ctor_Tag, new_1, value)]
#[byte_size(1)]
pub struct Tag_X {}
impl Tag_X {
    pub fn new_1(mut _a0: Ctor_Tag) -> Self {
        Self {}
    }
    pub fn new_2(mut _a0: Ctor_Tag, _a1: Ptr<Tag_X>) -> Self {
        Self {}
    }
    pub fn new_3(mut _a0: Ctor_Tag, _a1: Ptr<Tag_X>) -> Self {
        Self {}
    }
}
pub fn operator_eq_6(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
pub fn operator_ne_7(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } != { y.with(|__s| __s.data_) });
}
pub fn operator_lt_8(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } < { y.with(|__s| __s.data_) });
}
pub fn operator_le_9(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } <= { y.with(|__s| __s.data_) });
}
pub fn operator_gt_10(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } > { y.with(|__s| __s.data_) });
}
pub fn operator_ge_11(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } >= { y.with(|__s| __s.data_) });
}
pub fn operator_cmp_12(_a0: Ptr<MoveOnly>, _a1: Ptr<MoveOnly>) -> std::cmp::Ordering {
    {
        let cmp: Value<std::cmp::Ordering> = Rc::new(RefCell::new(std::cmp::Ord::cmp(
            &(_a0.with(|__s| __s.data_)),
            &(_a1.with(|__s| __s.data_)),
        )));
        if !(({ (*cmp.borrow()) }) == std::cmp::Ordering::Equal) {
            return ({ (*cmp.borrow()) });
        }
    }
    return ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) });
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, CppFrom, MoveAssign)]
#[cpp_from(i32, new, some_value)]
#[byte_size(4)]
pub struct MoveOnly {
    #[offset(0)]
    data_: i32,
}
impl MoveOnly {
    pub fn new(data: Option<i32>) -> Self {
        let mut data: i32 = data.unwrap_or_else(|| 1);
        Self { data_: data }
    }
    pub fn move_from(x: Ptr<MoveOnly>) -> Self {
        let __this: MoveOnly = Self {
            data_: x.with(|__s| __s.data_),
        };
        field!(x, data_).write(0);
        __this
    }
}
impl CppLess for MoveOnly {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_lt_8(__lhs.clone(), __rhs.clone())
    }
}
impl CppEq for MoveOnly {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_6(__lhs.clone(), __rhs.clone())
    }
}
impl CppMove for MoveOnly {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
impl Default for MoveOnly {
    fn default() -> Self {
        { MoveOnly::new(None) }
    }
}
pub fn operator_eq_13(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
pub fn operator_ne_14(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } != { y.with(|__s| __s.data_) });
}
pub fn operator_lt_15(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } < { y.with(|__s| __s.data_) });
}
pub fn operator_le_16(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } <= { y.with(|__s| __s.data_) });
}
pub fn operator_gt_17(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } > { y.with(|__s| __s.data_) });
}
pub fn operator_ge_18(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } >= { y.with(|__s| __s.data_) });
}
pub fn operator_cmp_19(_a0: Ptr<TrivialMoveOnly>, _a1: Ptr<TrivialMoveOnly>) -> std::cmp::Ordering {
    {
        let cmp: Value<std::cmp::Ordering> = Rc::new(RefCell::new(std::cmp::Ord::cmp(
            &(_a0.with(|__s| __s.data_)),
            &(_a1.with(|__s| __s.data_)),
        )));
        if !(({ (*cmp.borrow()) }) == std::cmp::Ordering::Equal) {
            return ({ (*cmp.borrow()) });
        }
    }
    return ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) });
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, CppFrom, MoveAssign)]
#[cpp_from(i32, new, some_value)]
#[byte_size(4)]
pub struct TrivialMoveOnly {
    #[offset(0)]
    data_: i32,
}
impl TrivialMoveOnly {
    pub fn new(data: Option<i32>) -> Self {
        let mut data: i32 = data.unwrap_or_else(|| 1);
        Self { data_: data }
    }
    pub fn move_from(_a0: Ptr<TrivialMoveOnly>) -> Self {
        Self {
            data_: { (*_a0.upgrade().deref()).data_ },
        }
    }
}
impl CppLess for TrivialMoveOnly {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_lt_15(__lhs.clone(), __rhs.clone())
    }
}
impl CppEq for TrivialMoveOnly {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_13(__lhs.clone(), __rhs.clone())
    }
}
impl CppMove for TrivialMoveOnly {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
impl Default for TrivialMoveOnly {
    fn default() -> Self {
        { TrivialMoveOnly::new(None) }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub fn outstanding_alloc() -> usize {
        assert!(
            (alloc_count_21.with(|rc| *rc.borrow()) >= dealloc_count_22.with(|rc| *rc.borrow()))
        );
        return ((alloc_count_21.with(|rc| *rc.borrow()))
            .wrapping_sub(dealloc_count_22.with(|rc| *rc.borrow())));
    }
    pub fn reset() {
        assert!((({ malloc_allocator_base::outstanding_alloc() }) == 0_usize));
        disable_default_constructor_23.with(|rc| *rc.borrow_mut() = false);
        outstanding_bytes_20.with(|rc| *rc.borrow_mut() = 0_usize);
        alloc_count_21.with(|rc| *rc.borrow_mut() = 0_usize);
        dealloc_count_22.with(|rc| *rc.borrow_mut() = 0_usize);
    }
}
impl CppMove for malloc_allocator_base {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static outstanding_bytes_20: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static alloc_count_21: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static dealloc_count_22: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static disable_default_constructor_23: Value<bool> = Rc::new(RefCell::new(false));
);
pub fn operator_eq_24(mut _a0: min_allocator_MoveOnly_, mut _a1: min_allocator_MoveOnly_) -> bool {
    return true;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_MoveOnly_ {}
impl CppEq for min_allocator_MoveOnly_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_24(__lhs.read(), __rhs.read())
    }
}
impl CppMove for min_allocator_MoveOnly_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn test_25() -> bool {
    {
        let t0: Value<Box<[MoveOnly]>> = Rc::new(RefCell::new(Box::new([
            MoveOnly::new({ Some(0) }),
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(2) }),
            MoveOnly::new({ Some(3) }),
            MoveOnly::new({ Some(4) }),
            MoveOnly::new({ Some(5) }),
            MoveOnly::new({ Some(6) }),
            MoveOnly::new({ Some(7) }),
            MoveOnly::new({ Some(8) }),
            MoveOnly::new({ Some(9) }),
        ])));
        let t1: Value<Box<[MoveOnly]>> = Rc::new(RefCell::new(Box::new([
            MoveOnly::new({ Some(10) }),
            MoveOnly::new({ Some(11) }),
            MoveOnly::new({ Some(12) }),
            MoveOnly::new({ Some(13) }),
        ])));
        let c0: Value<std_forward_list_MoveOnly__test_allocator_MoveOnly__> =
            Rc::new(RefCell::new({
                let __tmp_0: Value<test_allocator_MoveOnly_> =
                    Rc::new(RefCell::new(test_allocator_MoveOnly_::new_1({ 10 })));
                std_forward_list_MoveOnly__test_allocator_MoveOnly__::new_1(
                    { std_move_iterator_MoveOnly_ptr_::new_1({ ({ begin_26(t0.as_pointer()) }) }) },
                    {
                        std_move_iterator_MoveOnly_ptr_::new_1({
                            (t0.as_pointer() as Ptr<MoveOnly>).to_end()
                        })
                    },
                    { __tmp_0.as_pointer() },
                )
            }));
        let c1: Value<std_forward_list_MoveOnly__test_allocator_MoveOnly__> =
            Rc::new(RefCell::new({
                let __tmp_1: Value<test_allocator_MoveOnly_> =
                    Rc::new(RefCell::new(test_allocator_MoveOnly_::new_1({ 10 })));
                std_forward_list_MoveOnly__test_allocator_MoveOnly__::new_1(
                    { std_move_iterator_MoveOnly_ptr_::new_1({ ({ begin_27(t1.as_pointer()) }) }) },
                    {
                        std_move_iterator_MoveOnly_ptr_::new_1({
                            (t1.as_pointer() as Ptr<MoveOnly>).to_end()
                        })
                    },
                    { __tmp_1.as_pointer() },
                )
            }));
        (*c1.borrow_mut()) = (*c0.borrow()).clone();
        let mut n: i32 = 0;
        let i: Value<std__Fwd_list_const_iterator_MoveOnly_> =
            Rc::new(RefCell::new(({ (*c1.borrow()).cbegin() })));
        'loop_: while (*i.borrow()) != ({ (*c1.borrow()).cend() }) {
            assert!(
                ({
                    let _x: Ptr<MoveOnly> = (*i.borrow()).iter_deref();
                    let _y: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ Some(n) })));
                    operator_eq_6(_x, _y.as_pointer())
                })
            );
            {
                {
                    i.as_pointer().with_mut(|__it| {
                        __it.iter_inc();
                    });
                    i.as_pointer()
                };
                &(n.prefix_inc())
            };
        }
        assert!((n == 10));
        assert!(
            ({
                let _x: Value<test_allocator_MoveOnly_> =
                    Rc::new(RefCell::new(({ (*c1.borrow()).get_allocator() })));
                let _y: Value<test_allocator_MoveOnly_> =
                    Rc::new(RefCell::new(test_allocator_MoveOnly_::new_1({ 10 })));
                operator_eq_4(_x.as_pointer(), _y.as_pointer())
            })
        );
        assert!(({ ({ (*c0.borrow()).empty() }) }));
    }
    {
        let t0: Value<Box<[MoveOnly]>> = Rc::new(RefCell::new(Box::new([
            MoveOnly::new({ Some(0) }),
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(2) }),
            MoveOnly::new({ Some(3) }),
            MoveOnly::new({ Some(4) }),
            MoveOnly::new({ Some(5) }),
            MoveOnly::new({ Some(6) }),
            MoveOnly::new({ Some(7) }),
            MoveOnly::new({ Some(8) }),
            MoveOnly::new({ Some(9) }),
        ])));
        let t1: Value<Box<[MoveOnly]>> = Rc::new(RefCell::new(Box::new([
            MoveOnly::new({ Some(10) }),
            MoveOnly::new({ Some(11) }),
            MoveOnly::new({ Some(12) }),
            MoveOnly::new({ Some(13) }),
        ])));
        let c0: Value<std_forward_list_MoveOnly__test_allocator_MoveOnly__> =
            Rc::new(RefCell::new({
                let __tmp_2: Value<test_allocator_MoveOnly_> =
                    Rc::new(RefCell::new(test_allocator_MoveOnly_::new_1({ 10 })));
                std_forward_list_MoveOnly__test_allocator_MoveOnly__::new_1(
                    { std_move_iterator_MoveOnly_ptr_::new_1({ ({ begin_26(t0.as_pointer()) }) }) },
                    {
                        std_move_iterator_MoveOnly_ptr_::new_1({
                            (t0.as_pointer() as Ptr<MoveOnly>).to_end()
                        })
                    },
                    { __tmp_2.as_pointer() },
                )
            }));
        let c1: Value<std_forward_list_MoveOnly__test_allocator_MoveOnly__> =
            Rc::new(RefCell::new({
                let __tmp_3: Value<test_allocator_MoveOnly_> =
                    Rc::new(RefCell::new(test_allocator_MoveOnly_::new_1({ 11 })));
                std_forward_list_MoveOnly__test_allocator_MoveOnly__::new_1(
                    { std_move_iterator_MoveOnly_ptr_::new_1({ ({ begin_27(t1.as_pointer()) }) }) },
                    {
                        std_move_iterator_MoveOnly_ptr_::new_1({
                            (t1.as_pointer() as Ptr<MoveOnly>).to_end()
                        })
                    },
                    { __tmp_3.as_pointer() },
                )
            }));
        (*c1.borrow_mut()) = (*c0.borrow()).clone();
        let mut n: i32 = 0;
        let i: Value<std__Fwd_list_const_iterator_MoveOnly_> =
            Rc::new(RefCell::new(({ (*c1.borrow()).cbegin() })));
        'loop_: while (*i.borrow()) != ({ (*c1.borrow()).cend() }) {
            assert!(
                ({
                    let _x: Ptr<MoveOnly> = (*i.borrow()).iter_deref();
                    let _y: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ Some(n) })));
                    operator_eq_6(_x, _y.as_pointer())
                })
            );
            {
                {
                    i.as_pointer().with_mut(|__it| {
                        __it.iter_inc();
                    });
                    i.as_pointer()
                };
                &(n.prefix_inc())
            };
        }
        assert!((n == 10));
        assert!(
            ({
                let _x: Value<test_allocator_MoveOnly_> =
                    Rc::new(RefCell::new(({ (*c1.borrow()).get_allocator() })));
                let _y: Value<test_allocator_MoveOnly_> =
                    Rc::new(RefCell::new(test_allocator_MoveOnly_::new_1({ 11 })));
                operator_eq_4(_x.as_pointer(), _y.as_pointer())
            })
        );
        assert!(({ !({ (*c0.borrow()).empty() }) }));
    }
    {
        let t0: Value<Box<[MoveOnly]>> = Rc::new(RefCell::new(Box::new([
            MoveOnly::new({ Some(10) }),
            MoveOnly::new({ Some(11) }),
            MoveOnly::new({ Some(12) }),
            MoveOnly::new({ Some(13) }),
        ])));
        let t1: Value<Box<[MoveOnly]>> = Rc::new(RefCell::new(Box::new([
            MoveOnly::new({ Some(0) }),
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(2) }),
            MoveOnly::new({ Some(3) }),
            MoveOnly::new({ Some(4) }),
            MoveOnly::new({ Some(5) }),
            MoveOnly::new({ Some(6) }),
            MoveOnly::new({ Some(7) }),
            MoveOnly::new({ Some(8) }),
            MoveOnly::new({ Some(9) }),
        ])));
        let c0: Value<std_forward_list_MoveOnly__test_allocator_MoveOnly__> =
            Rc::new(RefCell::new({
                let __tmp_4: Value<test_allocator_MoveOnly_> =
                    Rc::new(RefCell::new(test_allocator_MoveOnly_::new_1({ 10 })));
                std_forward_list_MoveOnly__test_allocator_MoveOnly__::new_1(
                    { std_move_iterator_MoveOnly_ptr_::new_1({ ({ begin_27(t0.as_pointer()) }) }) },
                    {
                        std_move_iterator_MoveOnly_ptr_::new_1({
                            (t0.as_pointer() as Ptr<MoveOnly>).to_end()
                        })
                    },
                    { __tmp_4.as_pointer() },
                )
            }));
        let c1: Value<std_forward_list_MoveOnly__test_allocator_MoveOnly__> =
            Rc::new(RefCell::new({
                let __tmp_5: Value<test_allocator_MoveOnly_> =
                    Rc::new(RefCell::new(test_allocator_MoveOnly_::new_1({ 10 })));
                std_forward_list_MoveOnly__test_allocator_MoveOnly__::new_1(
                    { std_move_iterator_MoveOnly_ptr_::new_1({ ({ begin_26(t1.as_pointer()) }) }) },
                    {
                        std_move_iterator_MoveOnly_ptr_::new_1({
                            (t1.as_pointer() as Ptr<MoveOnly>).to_end()
                        })
                    },
                    { __tmp_5.as_pointer() },
                )
            }));
        (*c1.borrow_mut()) = (*c0.borrow()).clone();
        let mut n: i32 = 0;
        let i: Value<std__Fwd_list_const_iterator_MoveOnly_> =
            Rc::new(RefCell::new(({ (*c1.borrow()).cbegin() })));
        'loop_: while (*i.borrow()) != ({ (*c1.borrow()).cend() }) {
            assert!(
                ({
                    let _x: Ptr<MoveOnly> = (*i.borrow()).iter_deref();
                    let _y: Value<MoveOnly> =
                        Rc::new(RefCell::new(MoveOnly::new({ Some((10 + n)) })));
                    operator_eq_6(_x, _y.as_pointer())
                })
            );
            {
                {
                    i.as_pointer().with_mut(|__it| {
                        __it.iter_inc();
                    });
                    i.as_pointer()
                };
                &(n.prefix_inc())
            };
        }
        assert!((n == 4));
        assert!(
            ({
                let _x: Value<test_allocator_MoveOnly_> =
                    Rc::new(RefCell::new(({ (*c1.borrow()).get_allocator() })));
                let _y: Value<test_allocator_MoveOnly_> =
                    Rc::new(RefCell::new(test_allocator_MoveOnly_::new_1({ 10 })));
                operator_eq_4(_x.as_pointer(), _y.as_pointer())
            })
        );
        assert!(({ ({ (*c0.borrow()).empty() }) }));
    }
    {
        let t0: Value<Box<[MoveOnly]>> = Rc::new(RefCell::new(Box::new([
            MoveOnly::new({ Some(10) }),
            MoveOnly::new({ Some(11) }),
            MoveOnly::new({ Some(12) }),
            MoveOnly::new({ Some(13) }),
        ])));
        let t1: Value<Box<[MoveOnly]>> = Rc::new(RefCell::new(Box::new([
            MoveOnly::new({ Some(0) }),
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(2) }),
            MoveOnly::new({ Some(3) }),
            MoveOnly::new({ Some(4) }),
            MoveOnly::new({ Some(5) }),
            MoveOnly::new({ Some(6) }),
            MoveOnly::new({ Some(7) }),
            MoveOnly::new({ Some(8) }),
            MoveOnly::new({ Some(9) }),
        ])));
        let c0: Value<std_forward_list_MoveOnly__test_allocator_MoveOnly__> =
            Rc::new(RefCell::new({
                let __tmp_6: Value<test_allocator_MoveOnly_> =
                    Rc::new(RefCell::new(test_allocator_MoveOnly_::new_1({ 10 })));
                std_forward_list_MoveOnly__test_allocator_MoveOnly__::new_1(
                    { std_move_iterator_MoveOnly_ptr_::new_1({ ({ begin_27(t0.as_pointer()) }) }) },
                    {
                        std_move_iterator_MoveOnly_ptr_::new_1({
                            (t0.as_pointer() as Ptr<MoveOnly>).to_end()
                        })
                    },
                    { __tmp_6.as_pointer() },
                )
            }));
        let c1: Value<std_forward_list_MoveOnly__test_allocator_MoveOnly__> =
            Rc::new(RefCell::new({
                let __tmp_7: Value<test_allocator_MoveOnly_> =
                    Rc::new(RefCell::new(test_allocator_MoveOnly_::new_1({ 11 })));
                std_forward_list_MoveOnly__test_allocator_MoveOnly__::new_1(
                    { std_move_iterator_MoveOnly_ptr_::new_1({ ({ begin_26(t1.as_pointer()) }) }) },
                    {
                        std_move_iterator_MoveOnly_ptr_::new_1({
                            (t1.as_pointer() as Ptr<MoveOnly>).to_end()
                        })
                    },
                    { __tmp_7.as_pointer() },
                )
            }));
        (*c1.borrow_mut()) = (*c0.borrow()).clone();
        let mut n: i32 = 0;
        let i: Value<std__Fwd_list_const_iterator_MoveOnly_> =
            Rc::new(RefCell::new(({ (*c1.borrow()).cbegin() })));
        'loop_: while (*i.borrow()) != ({ (*c1.borrow()).cend() }) {
            assert!(
                ({
                    let _x: Ptr<MoveOnly> = (*i.borrow()).iter_deref();
                    let _y: Value<MoveOnly> =
                        Rc::new(RefCell::new(MoveOnly::new({ Some((10 + n)) })));
                    operator_eq_6(_x, _y.as_pointer())
                })
            );
            {
                {
                    i.as_pointer().with_mut(|__it| {
                        __it.iter_inc();
                    });
                    i.as_pointer()
                };
                &(n.prefix_inc())
            };
        }
        assert!((n == 4));
        assert!(
            ({
                let _x: Value<test_allocator_MoveOnly_> =
                    Rc::new(RefCell::new(({ (*c1.borrow()).get_allocator() })));
                let _y: Value<test_allocator_MoveOnly_> =
                    Rc::new(RefCell::new(test_allocator_MoveOnly_::new_1({ 11 })));
                operator_eq_4(_x.as_pointer(), _y.as_pointer())
            })
        );
        assert!(({ !({ (*c0.borrow()).empty() }) }));
    }
    {
        let t0: Value<Box<[MoveOnly]>> = Rc::new(RefCell::new(Box::new([
            MoveOnly::new({ Some(0) }),
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(2) }),
            MoveOnly::new({ Some(3) }),
            MoveOnly::new({ Some(4) }),
            MoveOnly::new({ Some(5) }),
            MoveOnly::new({ Some(6) }),
            MoveOnly::new({ Some(7) }),
            MoveOnly::new({ Some(8) }),
            MoveOnly::new({ Some(9) }),
        ])));
        let t1: Value<Box<[MoveOnly]>> = Rc::new(RefCell::new(Box::new([
            MoveOnly::new({ Some(10) }),
            MoveOnly::new({ Some(11) }),
            MoveOnly::new({ Some(12) }),
            MoveOnly::new({ Some(13) }),
        ])));
        let c0: Value<std_forward_list_MoveOnly__other_allocator_MoveOnly__> =
            Rc::new(RefCell::new({
                let __tmp_8: Value<other_allocator_MoveOnly_> =
                    Rc::new(RefCell::new(other_allocator_MoveOnly_::new_1({ 10 })));
                std_forward_list_MoveOnly__other_allocator_MoveOnly__::new_1(
                    { std_move_iterator_MoveOnly_ptr_::new_1({ ({ begin_26(t0.as_pointer()) }) }) },
                    {
                        std_move_iterator_MoveOnly_ptr_::new_1({
                            (t0.as_pointer() as Ptr<MoveOnly>).to_end()
                        })
                    },
                    { __tmp_8.as_pointer() },
                )
            }));
        let c1: Value<std_forward_list_MoveOnly__other_allocator_MoveOnly__> =
            Rc::new(RefCell::new({
                let __tmp_9: Value<other_allocator_MoveOnly_> =
                    Rc::new(RefCell::new(other_allocator_MoveOnly_::new_1({ 10 })));
                std_forward_list_MoveOnly__other_allocator_MoveOnly__::new_1(
                    { std_move_iterator_MoveOnly_ptr_::new_1({ ({ begin_27(t1.as_pointer()) }) }) },
                    {
                        std_move_iterator_MoveOnly_ptr_::new_1({
                            (t1.as_pointer() as Ptr<MoveOnly>).to_end()
                        })
                    },
                    { __tmp_9.as_pointer() },
                )
            }));
        (*c1.borrow_mut()) = (*c0.borrow()).clone();
        let mut n: i32 = 0;
        let i: Value<std__Fwd_list_const_iterator_MoveOnly_> =
            Rc::new(RefCell::new(({ (*c1.borrow()).cbegin() })));
        'loop_: while (*i.borrow()) != ({ (*c1.borrow()).cend() }) {
            assert!(
                ({
                    let _x: Ptr<MoveOnly> = (*i.borrow()).iter_deref();
                    let _y: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ Some(n) })));
                    operator_eq_6(_x, _y.as_pointer())
                })
            );
            {
                {
                    i.as_pointer().with_mut(|__it| {
                        __it.iter_inc();
                    });
                    i.as_pointer()
                };
                &(n.prefix_inc())
            };
        }
        assert!((n == 10));
        assert!(
            ({
                let _x: Value<other_allocator_MoveOnly_> =
                    Rc::new(RefCell::new(({ (*c1.borrow()).get_allocator() })));
                let _y: Value<other_allocator_MoveOnly_> =
                    Rc::new(RefCell::new(other_allocator_MoveOnly_::new_1({ 10 })));
                operator_eq_5(_x.as_pointer(), _y.as_pointer())
            })
        );
        assert!(({ ({ (*c0.borrow()).empty() }) }));
    }
    {
        let t0: Value<Box<[MoveOnly]>> = Rc::new(RefCell::new(Box::new([
            MoveOnly::new({ Some(0) }),
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(2) }),
            MoveOnly::new({ Some(3) }),
            MoveOnly::new({ Some(4) }),
            MoveOnly::new({ Some(5) }),
            MoveOnly::new({ Some(6) }),
            MoveOnly::new({ Some(7) }),
            MoveOnly::new({ Some(8) }),
            MoveOnly::new({ Some(9) }),
        ])));
        let t1: Value<Box<[MoveOnly]>> = Rc::new(RefCell::new(Box::new([
            MoveOnly::new({ Some(10) }),
            MoveOnly::new({ Some(11) }),
            MoveOnly::new({ Some(12) }),
            MoveOnly::new({ Some(13) }),
        ])));
        let c0: Value<std_forward_list_MoveOnly__other_allocator_MoveOnly__> =
            Rc::new(RefCell::new({
                let __tmp_10: Value<other_allocator_MoveOnly_> =
                    Rc::new(RefCell::new(other_allocator_MoveOnly_::new_1({ 10 })));
                std_forward_list_MoveOnly__other_allocator_MoveOnly__::new_1(
                    { std_move_iterator_MoveOnly_ptr_::new_1({ ({ begin_26(t0.as_pointer()) }) }) },
                    {
                        std_move_iterator_MoveOnly_ptr_::new_1({
                            (t0.as_pointer() as Ptr<MoveOnly>).to_end()
                        })
                    },
                    { __tmp_10.as_pointer() },
                )
            }));
        let c1: Value<std_forward_list_MoveOnly__other_allocator_MoveOnly__> =
            Rc::new(RefCell::new({
                let __tmp_11: Value<other_allocator_MoveOnly_> =
                    Rc::new(RefCell::new(other_allocator_MoveOnly_::new_1({ 11 })));
                std_forward_list_MoveOnly__other_allocator_MoveOnly__::new_1(
                    { std_move_iterator_MoveOnly_ptr_::new_1({ ({ begin_27(t1.as_pointer()) }) }) },
                    {
                        std_move_iterator_MoveOnly_ptr_::new_1({
                            (t1.as_pointer() as Ptr<MoveOnly>).to_end()
                        })
                    },
                    { __tmp_11.as_pointer() },
                )
            }));
        (*c1.borrow_mut()) = (*c0.borrow()).clone();
        let mut n: i32 = 0;
        let i: Value<std__Fwd_list_const_iterator_MoveOnly_> =
            Rc::new(RefCell::new(({ (*c1.borrow()).cbegin() })));
        'loop_: while (*i.borrow()) != ({ (*c1.borrow()).cend() }) {
            assert!(
                ({
                    let _x: Ptr<MoveOnly> = (*i.borrow()).iter_deref();
                    let _y: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ Some(n) })));
                    operator_eq_6(_x, _y.as_pointer())
                })
            );
            {
                {
                    i.as_pointer().with_mut(|__it| {
                        __it.iter_inc();
                    });
                    i.as_pointer()
                };
                &(n.prefix_inc())
            };
        }
        assert!((n == 10));
        assert!(
            ({
                let _x: Value<other_allocator_MoveOnly_> =
                    Rc::new(RefCell::new(({ (*c1.borrow()).get_allocator() })));
                let _y: Value<other_allocator_MoveOnly_> =
                    Rc::new(RefCell::new(other_allocator_MoveOnly_::new_1({ 10 })));
                operator_eq_5(_x.as_pointer(), _y.as_pointer())
            })
        );
        assert!(({ ({ (*c0.borrow()).empty() }) }));
    }
    {
        let t0: Value<Box<[MoveOnly]>> = Rc::new(RefCell::new(Box::new([
            MoveOnly::new({ Some(10) }),
            MoveOnly::new({ Some(11) }),
            MoveOnly::new({ Some(12) }),
            MoveOnly::new({ Some(13) }),
        ])));
        let t1: Value<Box<[MoveOnly]>> = Rc::new(RefCell::new(Box::new([
            MoveOnly::new({ Some(0) }),
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(2) }),
            MoveOnly::new({ Some(3) }),
            MoveOnly::new({ Some(4) }),
            MoveOnly::new({ Some(5) }),
            MoveOnly::new({ Some(6) }),
            MoveOnly::new({ Some(7) }),
            MoveOnly::new({ Some(8) }),
            MoveOnly::new({ Some(9) }),
        ])));
        let c0: Value<std_forward_list_MoveOnly__other_allocator_MoveOnly__> =
            Rc::new(RefCell::new({
                let __tmp_12: Value<other_allocator_MoveOnly_> =
                    Rc::new(RefCell::new(other_allocator_MoveOnly_::new_1({ 10 })));
                std_forward_list_MoveOnly__other_allocator_MoveOnly__::new_1(
                    { std_move_iterator_MoveOnly_ptr_::new_1({ ({ begin_27(t0.as_pointer()) }) }) },
                    {
                        std_move_iterator_MoveOnly_ptr_::new_1({
                            (t0.as_pointer() as Ptr<MoveOnly>).to_end()
                        })
                    },
                    { __tmp_12.as_pointer() },
                )
            }));
        let c1: Value<std_forward_list_MoveOnly__other_allocator_MoveOnly__> =
            Rc::new(RefCell::new({
                let __tmp_13: Value<other_allocator_MoveOnly_> =
                    Rc::new(RefCell::new(other_allocator_MoveOnly_::new_1({ 10 })));
                std_forward_list_MoveOnly__other_allocator_MoveOnly__::new_1(
                    { std_move_iterator_MoveOnly_ptr_::new_1({ ({ begin_26(t1.as_pointer()) }) }) },
                    {
                        std_move_iterator_MoveOnly_ptr_::new_1({
                            (t1.as_pointer() as Ptr<MoveOnly>).to_end()
                        })
                    },
                    { __tmp_13.as_pointer() },
                )
            }));
        (*c1.borrow_mut()) = (*c0.borrow()).clone();
        let mut n: i32 = 0;
        let i: Value<std__Fwd_list_const_iterator_MoveOnly_> =
            Rc::new(RefCell::new(({ (*c1.borrow()).cbegin() })));
        'loop_: while (*i.borrow()) != ({ (*c1.borrow()).cend() }) {
            assert!(
                ({
                    let _x: Ptr<MoveOnly> = (*i.borrow()).iter_deref();
                    let _y: Value<MoveOnly> =
                        Rc::new(RefCell::new(MoveOnly::new({ Some((10 + n)) })));
                    operator_eq_6(_x, _y.as_pointer())
                })
            );
            {
                {
                    i.as_pointer().with_mut(|__it| {
                        __it.iter_inc();
                    });
                    i.as_pointer()
                };
                &(n.prefix_inc())
            };
        }
        assert!((n == 4));
        assert!(
            ({
                let _x: Value<other_allocator_MoveOnly_> =
                    Rc::new(RefCell::new(({ (*c1.borrow()).get_allocator() })));
                let _y: Value<other_allocator_MoveOnly_> =
                    Rc::new(RefCell::new(other_allocator_MoveOnly_::new_1({ 10 })));
                operator_eq_5(_x.as_pointer(), _y.as_pointer())
            })
        );
        assert!(({ ({ (*c0.borrow()).empty() }) }));
    }
    {
        let t0: Value<Box<[MoveOnly]>> = Rc::new(RefCell::new(Box::new([
            MoveOnly::new({ Some(10) }),
            MoveOnly::new({ Some(11) }),
            MoveOnly::new({ Some(12) }),
            MoveOnly::new({ Some(13) }),
        ])));
        let t1: Value<Box<[MoveOnly]>> = Rc::new(RefCell::new(Box::new([
            MoveOnly::new({ Some(0) }),
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(2) }),
            MoveOnly::new({ Some(3) }),
            MoveOnly::new({ Some(4) }),
            MoveOnly::new({ Some(5) }),
            MoveOnly::new({ Some(6) }),
            MoveOnly::new({ Some(7) }),
            MoveOnly::new({ Some(8) }),
            MoveOnly::new({ Some(9) }),
        ])));
        let c0: Value<std_forward_list_MoveOnly__other_allocator_MoveOnly__> =
            Rc::new(RefCell::new({
                let __tmp_14: Value<other_allocator_MoveOnly_> =
                    Rc::new(RefCell::new(other_allocator_MoveOnly_::new_1({ 10 })));
                std_forward_list_MoveOnly__other_allocator_MoveOnly__::new_1(
                    { std_move_iterator_MoveOnly_ptr_::new_1({ ({ begin_27(t0.as_pointer()) }) }) },
                    {
                        std_move_iterator_MoveOnly_ptr_::new_1({
                            (t0.as_pointer() as Ptr<MoveOnly>).to_end()
                        })
                    },
                    { __tmp_14.as_pointer() },
                )
            }));
        let c1: Value<std_forward_list_MoveOnly__other_allocator_MoveOnly__> =
            Rc::new(RefCell::new({
                let __tmp_15: Value<other_allocator_MoveOnly_> =
                    Rc::new(RefCell::new(other_allocator_MoveOnly_::new_1({ 11 })));
                std_forward_list_MoveOnly__other_allocator_MoveOnly__::new_1(
                    { std_move_iterator_MoveOnly_ptr_::new_1({ ({ begin_26(t1.as_pointer()) }) }) },
                    {
                        std_move_iterator_MoveOnly_ptr_::new_1({
                            (t1.as_pointer() as Ptr<MoveOnly>).to_end()
                        })
                    },
                    { __tmp_15.as_pointer() },
                )
            }));
        (*c1.borrow_mut()) = (*c0.borrow()).clone();
        let mut n: i32 = 0;
        let i: Value<std__Fwd_list_const_iterator_MoveOnly_> =
            Rc::new(RefCell::new(({ (*c1.borrow()).cbegin() })));
        'loop_: while (*i.borrow()) != ({ (*c1.borrow()).cend() }) {
            assert!(
                ({
                    let _x: Ptr<MoveOnly> = (*i.borrow()).iter_deref();
                    let _y: Value<MoveOnly> =
                        Rc::new(RefCell::new(MoveOnly::new({ Some((10 + n)) })));
                    operator_eq_6(_x, _y.as_pointer())
                })
            );
            {
                {
                    i.as_pointer().with_mut(|__it| {
                        __it.iter_inc();
                    });
                    i.as_pointer()
                };
                &(n.prefix_inc())
            };
        }
        assert!((n == 4));
        assert!(
            ({
                let _x: Value<other_allocator_MoveOnly_> =
                    Rc::new(RefCell::new(({ (*c1.borrow()).get_allocator() })));
                let _y: Value<other_allocator_MoveOnly_> =
                    Rc::new(RefCell::new(other_allocator_MoveOnly_::new_1({ 10 })));
                operator_eq_5(_x.as_pointer(), _y.as_pointer())
            })
        );
        assert!(({ ({ (*c0.borrow()).empty() }) }));
    }
    {
        let t0: Value<Box<[MoveOnly]>> = Rc::new(RefCell::new(Box::new([
            MoveOnly::new({ Some(0) }),
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(2) }),
            MoveOnly::new({ Some(3) }),
            MoveOnly::new({ Some(4) }),
            MoveOnly::new({ Some(5) }),
            MoveOnly::new({ Some(6) }),
            MoveOnly::new({ Some(7) }),
            MoveOnly::new({ Some(8) }),
            MoveOnly::new({ Some(9) }),
        ])));
        let t1: Value<Box<[MoveOnly]>> = Rc::new(RefCell::new(Box::new([
            MoveOnly::new({ Some(10) }),
            MoveOnly::new({ Some(11) }),
            MoveOnly::new({ Some(12) }),
            MoveOnly::new({ Some(13) }),
        ])));
        let c0: Value<std_forward_list_MoveOnly__min_allocator_MoveOnly__> =
            Rc::new(RefCell::new({
                let __tmp_16: Value<min_allocator_MoveOnly_> =
                    Rc::new(RefCell::new(<min_allocator_MoveOnly_>::default()));
                std_forward_list_MoveOnly__min_allocator_MoveOnly__::new_1(
                    { std_move_iterator_MoveOnly_ptr_::new_1({ ({ begin_26(t0.as_pointer()) }) }) },
                    {
                        std_move_iterator_MoveOnly_ptr_::new_1({
                            (t0.as_pointer() as Ptr<MoveOnly>).to_end()
                        })
                    },
                    { __tmp_16.as_pointer() },
                )
            }));
        let c1: Value<std_forward_list_MoveOnly__min_allocator_MoveOnly__> =
            Rc::new(RefCell::new({
                let __tmp_17: Value<min_allocator_MoveOnly_> =
                    Rc::new(RefCell::new(<min_allocator_MoveOnly_>::default()));
                std_forward_list_MoveOnly__min_allocator_MoveOnly__::new_1(
                    { std_move_iterator_MoveOnly_ptr_::new_1({ ({ begin_27(t1.as_pointer()) }) }) },
                    {
                        std_move_iterator_MoveOnly_ptr_::new_1({
                            (t1.as_pointer() as Ptr<MoveOnly>).to_end()
                        })
                    },
                    { __tmp_17.as_pointer() },
                )
            }));
        (*c1.borrow_mut()) = (*c0.borrow()).clone();
        let mut n: i32 = 0;
        let i : Value<std___fwdlist__Iterator_true__min_pointer_MoveOnly__std_integral_constant_unsigned_long__0___ > = Rc::new(RefCell::new(( { (*c1.borrow()) . cbegin ( ) } )  )) ;
        'loop_: while (*i.borrow()) != ({ (*c1.borrow()).cend() }) {
            assert!(
                ({
                    let _x: Ptr<MoveOnly> = (*i.borrow()).iter_deref();
                    let _y: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ Some(n) })));
                    operator_eq_6(_x, _y.as_pointer())
                })
            );
            {
                {
                    i.as_pointer().with_mut(|__it| {
                        __it.iter_inc();
                    });
                    i.as_pointer()
                };
                &(n.prefix_inc())
            };
        }
        assert!((n == 10));
        assert!(
            ({
                ({
                    let _arg0: min_allocator_MoveOnly_ = ({ (*c1.borrow()).get_allocator() });
                    operator_eq_24(_arg0, <min_allocator_MoveOnly_>::default())
                })
            })
        );
        assert!(({ ({ (*c0.borrow()).empty() }) }));
    }
    {
        let t0: Value<Box<[MoveOnly]>> = Rc::new(RefCell::new(Box::new([
            MoveOnly::new({ Some(10) }),
            MoveOnly::new({ Some(11) }),
            MoveOnly::new({ Some(12) }),
            MoveOnly::new({ Some(13) }),
        ])));
        let t1: Value<Box<[MoveOnly]>> = Rc::new(RefCell::new(Box::new([
            MoveOnly::new({ Some(0) }),
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(2) }),
            MoveOnly::new({ Some(3) }),
            MoveOnly::new({ Some(4) }),
            MoveOnly::new({ Some(5) }),
            MoveOnly::new({ Some(6) }),
            MoveOnly::new({ Some(7) }),
            MoveOnly::new({ Some(8) }),
            MoveOnly::new({ Some(9) }),
        ])));
        let c0: Value<std_forward_list_MoveOnly__min_allocator_MoveOnly__> =
            Rc::new(RefCell::new({
                let __tmp_18: Value<min_allocator_MoveOnly_> =
                    Rc::new(RefCell::new(<min_allocator_MoveOnly_>::default()));
                std_forward_list_MoveOnly__min_allocator_MoveOnly__::new_1(
                    { std_move_iterator_MoveOnly_ptr_::new_1({ ({ begin_27(t0.as_pointer()) }) }) },
                    {
                        std_move_iterator_MoveOnly_ptr_::new_1({
                            (t0.as_pointer() as Ptr<MoveOnly>).to_end()
                        })
                    },
                    { __tmp_18.as_pointer() },
                )
            }));
        let c1: Value<std_forward_list_MoveOnly__min_allocator_MoveOnly__> =
            Rc::new(RefCell::new({
                let __tmp_19: Value<min_allocator_MoveOnly_> =
                    Rc::new(RefCell::new(<min_allocator_MoveOnly_>::default()));
                std_forward_list_MoveOnly__min_allocator_MoveOnly__::new_1(
                    { std_move_iterator_MoveOnly_ptr_::new_1({ ({ begin_26(t1.as_pointer()) }) }) },
                    {
                        std_move_iterator_MoveOnly_ptr_::new_1({
                            (t1.as_pointer() as Ptr<MoveOnly>).to_end()
                        })
                    },
                    { __tmp_19.as_pointer() },
                )
            }));
        (*c1.borrow_mut()) = (*c0.borrow()).clone();
        let mut n: i32 = 0;
        let i : Value<std___fwdlist__Iterator_true__min_pointer_MoveOnly__std_integral_constant_unsigned_long__0___ > = Rc::new(RefCell::new(( { (*c1.borrow()) . cbegin ( ) } )  )) ;
        'loop_: while (*i.borrow()) != ({ (*c1.borrow()).cend() }) {
            assert!(
                ({
                    let _x: Ptr<MoveOnly> = (*i.borrow()).iter_deref();
                    let _y: Value<MoveOnly> =
                        Rc::new(RefCell::new(MoveOnly::new({ Some((10 + n)) })));
                    operator_eq_6(_x, _y.as_pointer())
                })
            );
            {
                {
                    i.as_pointer().with_mut(|__it| {
                        __it.iter_inc();
                    });
                    i.as_pointer()
                };
                &(n.prefix_inc())
            };
        }
        assert!((n == 4));
        assert!(
            ({
                ({
                    let _arg0: min_allocator_MoveOnly_ = ({ (*c1.borrow()).get_allocator() });
                    operator_eq_24(_arg0, <min_allocator_MoveOnly_>::default())
                })
            })
        );
        assert!(({ ({ (*c0.borrow()).empty() }) }));
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
    assert!(({ test_25() }));
    return 0;
}
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__;
pub trait MoveOnlyImpl {
    fn move_assign(&self, x: Ptr<MoveOnly>) -> Ptr<MoveOnly>;
    fn get(&self) -> i32;
    fn operator_add(&self, x: Ptr<MoveOnly>) -> MoveOnly;
    fn operator_mul(&self, x: Ptr<MoveOnly>) -> MoveOnly;
}
impl MoveOnlyImpl for Ptr<MoveOnly> {
    fn move_assign(&self, x: Ptr<MoveOnly>) -> Ptr<MoveOnly> {
        field!((*self), data_).write({ x.with(|__s| __s.data_) });
        field!(x, data_).write(0);
        return (*self).clone();
    }
    fn get(&self) -> i32 {
        return (*self).with(|__s| __s.data_);
    }
    fn operator_add(&self, x: Ptr<MoveOnly>) -> MoveOnly {
        return MoveOnly::new({
            Some(({ (*self).with(|__s| __s.data_) } + { x.with(|__s| __s.data_) }))
        });
    }
    fn operator_mul(&self, x: Ptr<MoveOnly>) -> MoveOnly {
        return MoveOnly::new({
            Some(({ (*self).with(|__s| __s.data_) } * { x.with(|__s| __s.data_) }))
        });
    }
}
pub trait Tag_XImpl {
    fn copy_assign(&self, _a0: Ptr<Tag_X>) -> Ptr<Tag_X>;
    fn move_assign(&self, _a0: Ptr<Tag_X>) -> Ptr<Tag_X>;
}
impl Tag_XImpl for Ptr<Tag_X> {
    fn copy_assign(&self, _a0: Ptr<Tag_X>) -> Ptr<Tag_X> {
        return (*self).clone();
    }
    fn move_assign(&self, _a0: Ptr<Tag_X>) -> Ptr<Tag_X> {
        return (*self).clone();
    }
}
pub trait TrivialMoveOnlyImpl {
    fn move_assign(&self, _a0: Ptr<TrivialMoveOnly>) -> Ptr<TrivialMoveOnly>;
    fn get(&self) -> i32;
    fn operator_add(&self, x: Ptr<TrivialMoveOnly>) -> TrivialMoveOnly;
    fn operator_mul(&self, x: Ptr<TrivialMoveOnly>) -> TrivialMoveOnly;
}
impl TrivialMoveOnlyImpl for Ptr<TrivialMoveOnly> {
    fn move_assign(&self, _a0: Ptr<TrivialMoveOnly>) -> Ptr<TrivialMoveOnly> {
        field!((*self), data_).write({ { (*_a0.upgrade().deref()).data_ } });
        return (*self).clone();
    }
    fn get(&self) -> i32 {
        return (*self).with(|__s| __s.data_);
    }
    fn operator_add(&self, x: Ptr<TrivialMoveOnly>) -> TrivialMoveOnly {
        return TrivialMoveOnly::new({
            Some(({ (*self).with(|__s| __s.data_) } + { x.with(|__s| __s.data_) }))
        });
    }
    fn operator_mul(&self, x: Ptr<TrivialMoveOnly>) -> TrivialMoveOnly {
        return TrivialMoveOnly::new({
            Some(({ (*self).with(|__s| __s.data_) } * { x.with(|__s| __s.data_) }))
        });
    }
}
pub trait min_allocator_MoveOnly_Impl {
    fn allocate(
        &self,
        mut n: usize,
    ) -> min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn deallocate(
        &self,
        mut p: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        unimplemented!()
    }
}
impl min_allocator_MoveOnly_Impl for Ptr<min_allocator_MoveOnly_> {}
pub trait other_allocator_MoveOnly_Impl {
    fn allocate(&self, mut n: usize) -> Ptr<MoveOnly> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<MoveOnly>, mut s: usize) {
        unimplemented!()
    }
    fn select_on_container_copy_construction(&self) -> other_allocator_MoveOnly_ {
        unimplemented!()
    }
    fn get_data(&self) -> i32 {
        unimplemented!()
    }
}
impl other_allocator_MoveOnly_Impl for Ptr<other_allocator_MoveOnly_> {}
pub trait test_allocator_MoveOnly_Impl {
    fn destructor(&self);
    fn address_11(&self, x: Ptr<MoveOnly>) -> Ptr<MoveOnly> {
        unimplemented!()
    }
    fn address_12(&self, x: Ptr<MoveOnly>) -> Ptr<MoveOnly> {
        unimplemented!()
    }
    fn allocate(&self, mut n: u32, mut _a1: AnyPtr) -> Ptr<MoveOnly> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<MoveOnly>, mut s: u32) {
        unimplemented!()
    }
    fn max_size(&self) -> u32 {
        unimplemented!()
    }
    fn destroy(&self, mut p: Ptr<MoveOnly>) {
        unimplemented!()
    }
    fn get_data(&self) -> i32 {
        unimplemented!()
    }
    fn get_id(&self) -> i32 {
        unimplemented!()
    }
}
impl test_allocator_MoveOnly_Impl for Ptr<test_allocator_MoveOnly_> {
    fn destructor(&self) {
        assert!(((*self).with(|__s| __s.data_) != destructed_value_0.with(|rc| *rc.borrow())));
        assert!(((*self).with(|__s| __s.id_) != destructed_value_0.with(|rc| *rc.borrow())));
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), count).with_mut(|__v| {
                __v.prefix_dec();
            });
        }
        field!((*self), data_).write(destructed_value_0.with(|rc| *rc.borrow()));
        field!((*self), id_).write(destructed_value_0.with(|rc| *rc.borrow()));
    }
}
pub trait test_allocator_statisticsImpl {
    fn clear(&self);
    fn clear_ctor_counters(&self);
}
impl test_allocator_statisticsImpl for Ptr<test_allocator_statistics> {
    fn clear(&self) {
        assert!(
            ((*self).with(|__s| __s.count) == 0)
                && (!(Ptr::<i8>::from_string_literal(b"clearing leaking allocator data?"))
                    .is_null())
        );
        field!((*self), count).write(0);
        field!((*self), time_to_throw).write(0);
        field!((*self), alloc_count).write(0);
        field!((*self), allocated_size).write(0);
        field!((*self), construct_count).write(0);
        field!((*self), destroy_count).write(0);
        field!((*self), throw_after).write(2147483647);
        ({ test_allocator_statisticsImpl::clear_ctor_counters(self) });
    }
    fn clear_ctor_counters(&self) {
        field!((*self), copied).write(0);
        field!((*self), moved).write(0);
        field!((*self), converted).write(0);
    }
}
pub trait test_allocator_void_Impl {
    fn destructor(&self);
    fn get_id(&self) -> i32;
    fn get_data(&self) -> i32;
    fn copy_assign(&self, _a0: Ptr<test_allocator_void_>) -> Ptr<test_allocator_void_>;
}
impl test_allocator_void_Impl for Ptr<test_allocator_void_> {
    fn destructor(&self) {
        field!((*self), data_).write(destructed_value_0.with(|rc| *rc.borrow()));
        field!((*self), id_).write(destructed_value_0.with(|rc| *rc.borrow()));
    }
    fn get_id(&self) -> i32 {
        return (*self).with(|__s| __s.id_);
    }
    fn get_data(&self) -> i32 {
        return (*self).with(|__s| __s.data_);
    }
    fn copy_assign(&self, _a0: Ptr<test_allocator_void_>) -> Ptr<test_allocator_void_> {
        field!((*self), data_).write({ _a0.with(|__s| __s.data_) });
        field!((*self), id_).write({ _a0.with(|__s| __s.id_) });
        field!((*self), stats_).write({ _a0.with(|__s| __s.stats_.clone()) });
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = destructed_value_0.with(|_| ());
    let _ = moved_value_1.with(|_| ());
    let _ = outstanding_bytes_20.with(|_| ());
    let _ = alloc_count_21.with(|_| ());
    let _ = dealloc_count_22.with(|_| ());
    let _ = disable_default_constructor_23.with(|_| ());
}
