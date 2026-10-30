extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn operator_eq_0(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
pub fn operator_ne_1(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } != { y.with(|__s| __s.data_) });
}
pub fn operator_lt_2(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } < { y.with(|__s| __s.data_) });
}
pub fn operator_le_3(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } <= { y.with(|__s| __s.data_) });
}
pub fn operator_gt_4(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } > { y.with(|__s| __s.data_) });
}
pub fn operator_ge_5(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } >= { y.with(|__s| __s.data_) });
}
pub fn operator_cmp_6(_a0: Ptr<MoveOnly>, _a1: Ptr<MoveOnly>) -> () {
    {
        let cmp: Value<()> = Rc::new(RefCell::new(std::cmp::Ord::cmp(
            &(_a0.with(|__s| __s.data_)),
            &(_a1.with(|__s| __s.data_)),
        )));
        if !(unimplemented!()) {
            return unimplemented!();
        }
    }
    return unimplemented!();
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign)]
#[byte_size(4)]
pub struct MoveOnly {
    #[offset(0)]
    data_: (),
}
impl MoveOnly {
    pub fn new(data: Option<()>) -> Self {
        let mut data: () = data.unwrap_or_else(|| 1_());
        Self { data_: data }
    }
    pub fn move_from(x: Ptr<MoveOnly>) -> Self {
        let __this: MoveOnly = Self {
            data_: x.with(|__s| __s.data_),
        };
        field!(x, data_).write(0_());
        __this
    }
}
impl CppLess for MoveOnly {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_lt_2(__lhs.clone(), __rhs.clone())
    }
}
impl CppEq for MoveOnly {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_0(__lhs.clone(), __rhs.clone())
    }
}
impl Default for MoveOnly {
    fn default() -> Self {
        { MoveOnly::new(None) }
    }
}
pub fn operator_eq_7(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
pub fn operator_ne_8(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } != { y.with(|__s| __s.data_) });
}
pub fn operator_lt_9(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } < { y.with(|__s| __s.data_) });
}
pub fn operator_le_10(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } <= { y.with(|__s| __s.data_) });
}
pub fn operator_gt_11(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } > { y.with(|__s| __s.data_) });
}
pub fn operator_ge_12(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } >= { y.with(|__s| __s.data_) });
}
pub fn operator_cmp_13(_a0: Ptr<TrivialMoveOnly>, _a1: Ptr<TrivialMoveOnly>) -> () {
    {
        let cmp: Value<()> = Rc::new(RefCell::new(std::cmp::Ord::cmp(
            &(_a0.with(|__s| __s.data_)),
            &(_a1.with(|__s| __s.data_)),
        )));
        if !(unimplemented!()) {
            return unimplemented!();
        }
    }
    return unimplemented!();
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign)]
#[byte_size(4)]
pub struct TrivialMoveOnly {
    #[offset(0)]
    data_: (),
}
impl TrivialMoveOnly {
    pub fn new(data: Option<()>) -> Self {
        let mut data: () = data.unwrap_or_else(|| 1_());
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
        operator_lt_9(__lhs.clone(), __rhs.clone())
    }
}
impl CppEq for TrivialMoveOnly {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_7(__lhs.clone(), __rhs.clone())
    }
}
impl Default for TrivialMoveOnly {
    fn default() -> Self {
        { TrivialMoveOnly::new(None) }
    }
}
pub fn operator_eq_14(a: Ptr<test_less_MoveOnly_>, b: Ptr<test_less_MoveOnly_>) -> () {
    return ({ a.with(|__s| __s.data_) } == { b.with(|__s| __s.data_) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct test_less_MoveOnly_ {
    #[offset(0)]
    pub data_: (),
}
impl test_less_MoveOnly_ {
    pub fn new_1(mut data: ()) -> Self {
        Self { data_: data }
    }
}
impl CppEq for test_less_MoveOnly_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_14(__lhs.clone(), __rhs.clone())
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(40)]
pub struct test_allocator_statistics {
    #[offset(0)]
    pub time_to_throw: (),
    #[offset(4)]
    pub throw_after: (),
    #[offset(8)]
    pub count: (),
    #[offset(12)]
    pub alloc_count: (),
    #[offset(16)]
    pub allocated_size: (),
    #[offset(20)]
    pub construct_count: (),
    #[offset(24)]
    pub destroy_count: (),
    #[offset(28)]
    pub copied: (),
    #[offset(32)]
    pub moved: (),
    #[offset(36)]
    pub converted: (),
}
impl Default for test_allocator_statistics {
    fn default() -> Self {
        {
            test_allocator_statistics {
                time_to_throw: 0_(),
                throw_after: 2147483647_(),
                count: 0_(),
                alloc_count: 0_(),
                allocated_size: 0_(),
                construct_count: 0_(),
                destroy_count: 0_(),
                copied: 0_(),
                moved: 0_(),
                converted: 0_(),
            }
        }
    }
}
thread_local!(
    pub static destructed_value_15: Value<()> = Rc::new(RefCell::new(-1_()));
);
thread_local!(
    pub static moved_value_16: Value<()> = Rc::new(RefCell::new(2147483647_()));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_alloc_base {}
pub fn operator_eq_17(x: Ptr<test_allocator_void_>, y: Ptr<test_allocator_void_>) -> () {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
pub fn operator_ne_18(x: Ptr<test_allocator_void_>, y: Ptr<test_allocator_void_>) -> () {
    return !({
        let _x: Ptr<test_allocator_void_> = (x).clone();
        let _y: Ptr<test_allocator_void_> = (y).clone();
        operator_eq_17(_x, _y)
    });
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CopyAssign, Destructor)]
#[byte_size(16)]
pub struct test_allocator_void_ {
    #[offset(0)]
    data_: (),
    #[offset(4)]
    id_: (),
    #[offset(8)]
    #[byte_size(8)]
    stats_: Ptr<test_allocator_statistics>,
}
impl test_allocator_void_ {
    pub fn new_1(mut stats: Ptr<test_allocator_statistics>) -> Self {
        Self {
            data_: 0_(),
            id_: 0_(),
            stats_: (stats).clone(),
        }
    }
    pub fn new_2(mut data: ()) -> Self {
        Self {
            data_: data,
            id_: 0_(),
            stats_: Ptr::<test_allocator_statistics>::null(),
        }
    }
    pub fn new_3(mut data: (), mut stats: Ptr<test_allocator_statistics>) -> Self {
        Self {
            data_: data,
            id_: 0_(),
            stats_: (stats).clone(),
        }
    }
    pub fn new_4(mut data: (), mut id: ()) -> Self {
        Self {
            data_: data,
            id_: id,
            stats_: Ptr::<test_allocator_statistics>::null(),
        }
    }
    pub fn new_5(mut data: (), mut id: (), mut stats: Ptr<test_allocator_statistics>) -> Self {
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
        operator_eq_17(__lhs.clone(), __rhs.clone())
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
impl Default for test_allocator_void_ {
    fn default() -> Self {
        {
            test_allocator_void_ {
                data_: 0_(),
                id_: 0_(),
                stats_: Ptr::<test_allocator_statistics>::null(),
            }
        }
    }
}
pub fn operator_eq_19(x: Ptr<test_allocator_MoveOnly_>, y: Ptr<test_allocator_MoveOnly_>) -> () {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_allocator_MoveOnly__rebind_MoveOnly_ {}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, Destructor)]
#[byte_size(16)]
pub struct test_allocator_MoveOnly_ {
    #[offset(0)]
    data_: (),
    #[offset(4)]
    id_: (),
    #[offset(8)]
    #[byte_size(8)]
    stats_: Ptr<test_allocator_statistics>,
}
impl test_allocator_MoveOnly_ {
    pub fn new_1(mut data: ()) -> Self {
        Self {
            data_: data,
            id_: 0_(),
            stats_: Ptr::<test_allocator_statistics>::null(),
        }
    }
    pub fn copy_from(a: Ptr<test_allocator_MoveOnly_>) -> Self {
        let __this: Value<test_allocator_MoveOnly_> = Rc::new(RefCell::new(Self {
            data_: a.with(|__s| __s.data_),
            id_: a.with(|__s| __s.id_),
            stats_: a.with(|__s| __s.stats_.clone()),
        }));
        let this: Ptr<test_allocator_MoveOnly_> = __this.as_pointer();
        unimplemented!();
        if !((this.with(|__s| __s.stats_.clone())).is_null()) {
            field!(this.with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_inc());
            field!(this.with(|__s| __s.stats_.clone()), copied).with_mut(|__v| __v.prefix_inc());
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl CppEq for test_allocator_MoveOnly_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_19(__lhs.clone(), __rhs.clone())
    }
}
impl Clone for test_allocator_MoveOnly_ {
    fn clone(&self) -> Self {
        let __src: Value<test_allocator_MoveOnly_> =
            Rc::new(RefCell::new(test_allocator_MoveOnly_ {
                data_: self.data_.clone(),
                id_: self.id_.clone(),
                stats_: self.stats_.clone(),
            }));
        test_allocator_MoveOnly_::copy_from(__src.as_pointer())
    }
}
impl Default for test_allocator_MoveOnly_ {
    fn default() -> Self {
        {
            test_allocator_MoveOnly_ {
                data_: unimplemented!(),
                id_: 0_(),
                stats_: Ptr::<test_allocator_statistics>::null(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_allocator_Counter_int___rebind_Counter_int__ {}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, Destructor)]
#[byte_size(16)]
pub struct test_allocator_Counter_int__ {
    #[offset(0)]
    data_: (),
    #[offset(4)]
    id_: (),
    #[offset(8)]
    #[byte_size(8)]
    stats_: Ptr<test_allocator_statistics>,
}
impl test_allocator_Counter_int__ {
    pub fn new_1(mut data: ()) -> Self {
        Self {
            data_: data,
            id_: 0_(),
            stats_: Ptr::<test_allocator_statistics>::null(),
        }
    }
    pub fn copy_from(a: Ptr<test_allocator_Counter_int__>) -> Self {
        let __this: Value<test_allocator_Counter_int__> = Rc::new(RefCell::new(Self {
            data_: a.with(|__s| __s.data_),
            id_: a.with(|__s| __s.id_),
            stats_: a.with(|__s| __s.stats_.clone()),
        }));
        let this: Ptr<test_allocator_Counter_int__> = __this.as_pointer();
        unimplemented!();
        if !((this.with(|__s| __s.stats_.clone())).is_null()) {
            field!(this.with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_inc());
            field!(this.with(|__s| __s.stats_.clone()), copied).with_mut(|__v| __v.prefix_inc());
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for test_allocator_Counter_int__ {
    fn clone(&self) -> Self {
        let __src: Value<test_allocator_Counter_int__> =
            Rc::new(RefCell::new(test_allocator_Counter_int__ {
                data_: self.data_.clone(),
                id_: self.id_.clone(),
                stats_: self.stats_.clone(),
            }));
        test_allocator_Counter_int__::copy_from(__src.as_pointer())
    }
}
impl Default for test_allocator_Counter_int__ {
    fn default() -> Self {
        {
            test_allocator_Counter_int__ {
                data_: 0_(),
                id_: 0_(),
                stats_: Ptr::<test_allocator_statistics>::null(),
            }
        }
    }
}
pub fn operator_eq_20(x: Ptr<other_allocator_MoveOnly_>, y: Ptr<other_allocator_MoveOnly_>) -> () {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct other_allocator_MoveOnly_ {
    #[offset(0)]
    data_: (),
}
impl other_allocator_MoveOnly_ {
    pub fn new_1(mut i: ()) -> Self {
        Self { data_: i }
    }
}
impl CppEq for other_allocator_MoveOnly_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_20(__lhs.clone(), __rhs.clone())
    }
}
impl Default for other_allocator_MoveOnly_ {
    fn default() -> Self {
        {
            other_allocator_MoveOnly_ {
                data_: unimplemented!(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Ctor_Tag {}
#[derive(Record, ByteRepr, FnPtrArg, CopyAssign, MoveAssign, Default)]
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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Counter_base {}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, Destructor, Default)]
#[byte_size(4)]
pub struct Counter_int_ {
    #[offset(0)]
    data_: (),
}
impl Counter_int_ {
    pub fn new_1(data: Ptr<()>) -> Self {
        let __this: Counter_int_ = Self {
            data_: (data.read()),
        };
        (*gConstructed_21.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
    pub fn copy_from(rhs: Ptr<Counter_int_>) -> Self {
        let __this: Counter_int_ = Self {
            data_: rhs.with(|__s| __s.data_),
        };
        (*gConstructed_21.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
}
impl CppLess for Counter_int_ {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        Counter_int_Impl::operator_lt(__lhs, __rhs.clone())
    }
}
impl CppEq for Counter_int_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        Counter_int_Impl::operator_eq(__lhs, __rhs.clone())
    }
}
impl Clone for Counter_int_ {
    fn clone(&self) -> Self {
        let __src: Value<Counter_int_> = Rc::new(RefCell::new(Counter_int_ {
            data_: self.data_.clone(),
        }));
        Counter_int_::copy_from(__src.as_pointer())
    }
}
thread_local!(
    pub static gConstructed_21: Value<()> = Rc::new(RefCell::new(0_()));
);
pub fn test_22() -> () {
    {
        let a1: Value<Box<[MoveOnly]>> = Rc::new(RefCell::new(Box::new([
            MoveOnly::new({ Some(1_()) }),
            MoveOnly::new({ Some(1_()) }),
            MoveOnly::new({ Some(1_()) }),
            MoveOnly::new({ Some(2_()) }),
            MoveOnly::new({ Some(2_()) }),
            MoveOnly::new({ Some(2_()) }),
            MoveOnly::new({ Some(3_()) }),
            MoveOnly::new({ Some(3_()) }),
            MoveOnly::new({ Some(3_()) }),
        ])));
        let m1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let a2: Value<Box<[MoveOnly]>> = Rc::new(RefCell::new(Box::new([
            MoveOnly::new({ Some(1_()) }),
            MoveOnly::new({ Some(1_()) }),
            MoveOnly::new({ Some(1_()) }),
            MoveOnly::new({ Some(2_()) }),
            MoveOnly::new({ Some(2_()) }),
            MoveOnly::new({ Some(2_()) }),
            MoveOnly::new({ Some(3_()) }),
            MoveOnly::new({ Some(3_()) }),
            MoveOnly::new({ Some(3_()) }),
        ])));
        let m2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let m3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        const _: () = assert!(true, "");;
    }
    {
        let a1: Value<Box<[MoveOnly]>> = Rc::new(RefCell::new(Box::new([
            MoveOnly::new({ Some(1_()) }),
            MoveOnly::new({ Some(1_()) }),
            MoveOnly::new({ Some(1_()) }),
            MoveOnly::new({ Some(2_()) }),
            MoveOnly::new({ Some(2_()) }),
            MoveOnly::new({ Some(2_()) }),
            MoveOnly::new({ Some(3_()) }),
            MoveOnly::new({ Some(3_()) }),
            MoveOnly::new({ Some(3_()) }),
        ])));
        let m1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let a2: Value<Box<[MoveOnly]>> = Rc::new(RefCell::new(Box::new([
            MoveOnly::new({ Some(1_()) }),
            MoveOnly::new({ Some(1_()) }),
            MoveOnly::new({ Some(1_()) }),
            MoveOnly::new({ Some(2_()) }),
            MoveOnly::new({ Some(2_()) }),
            MoveOnly::new({ Some(2_()) }),
            MoveOnly::new({ Some(3_()) }),
            MoveOnly::new({ Some(3_()) }),
            MoveOnly::new({ Some(3_()) }),
        ])));
        let m2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let m3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        const _: () = assert!(true, "");;
    }
    {
        let a1: Value<Box<[MoveOnly]>> = Rc::new(RefCell::new(Box::new([
            MoveOnly::new({ Some(1_()) }),
            MoveOnly::new({ Some(1_()) }),
            MoveOnly::new({ Some(1_()) }),
            MoveOnly::new({ Some(2_()) }),
            MoveOnly::new({ Some(2_()) }),
            MoveOnly::new({ Some(2_()) }),
            MoveOnly::new({ Some(3_()) }),
            MoveOnly::new({ Some(3_()) }),
            MoveOnly::new({ Some(3_()) }),
        ])));
        let m1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let a2: Value<Box<[MoveOnly]>> = Rc::new(RefCell::new(Box::new([
            MoveOnly::new({ Some(1_()) }),
            MoveOnly::new({ Some(1_()) }),
            MoveOnly::new({ Some(1_()) }),
            MoveOnly::new({ Some(2_()) }),
            MoveOnly::new({ Some(2_()) }),
            MoveOnly::new({ Some(2_()) }),
            MoveOnly::new({ Some(3_()) }),
            MoveOnly::new({ Some(3_()) }),
            MoveOnly::new({ Some(3_()) }),
        ])));
        let m2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let m3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        const _: () = assert!(true, "");;
    }
    {
        gConstructed_21.with(|rc| *rc.borrow_mut() = 0_());
        {
            let a1: Value<Box<[Counter_int_]>> = Rc::new(RefCell::new(Box::new([
                {
                    let __tmp_0: Value<()> = Rc::new(RefCell::new(1_()));
                    Counter_int_::new_1({ __tmp_0.as_pointer() })
                },
                {
                    let __tmp_1: Value<()> = Rc::new(RefCell::new(1_()));
                    Counter_int_::new_1({ __tmp_1.as_pointer() })
                },
                {
                    let __tmp_2: Value<()> = Rc::new(RefCell::new(1_()));
                    Counter_int_::new_1({ __tmp_2.as_pointer() })
                },
                {
                    let __tmp_3: Value<()> = Rc::new(RefCell::new(2_()));
                    Counter_int_::new_1({ __tmp_3.as_pointer() })
                },
                {
                    let __tmp_4: Value<()> = Rc::new(RefCell::new(2_()));
                    Counter_int_::new_1({ __tmp_4.as_pointer() })
                },
                {
                    let __tmp_5: Value<()> = Rc::new(RefCell::new(2_()));
                    Counter_int_::new_1({ __tmp_5.as_pointer() })
                },
                {
                    let __tmp_6: Value<()> = Rc::new(RefCell::new(3_()));
                    Counter_int_::new_1({ __tmp_6.as_pointer() })
                },
                {
                    let __tmp_7: Value<()> = Rc::new(RefCell::new(3_()));
                    Counter_int_::new_1({ __tmp_7.as_pointer() })
                },
                {
                    let __tmp_8: Value<()> = Rc::new(RefCell::new(3_()));
                    Counter_int_::new_1({ __tmp_8.as_pointer() })
                },
            ])));
            let mut num: () = (36usize as ()).wrapping_div((4usize as ()));
            unimplemented!();
            let m1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            let m2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            let m3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            const _: () = assert!(true, "");;
            unimplemented!();
            unimplemented!();
            {
                let m4: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                unimplemented!();
                unimplemented!();
                unimplemented!();
                const _: () = assert!(true, "");;
            }
            unimplemented!();
            unimplemented!();
        }
        unimplemented!();
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    ({ test_22() });
    return 0_();
}
pub trait Counter_int_Impl {
    fn copy_assign(&self, rhs: Ptr<Counter_int_>) -> Ptr<Counter_int_> {
        unimplemented!()
    }
    fn move_assign(&self, rhs: Ptr<Counter_int_>) -> Ptr<Counter_int_> {
        unimplemented!()
    }
    fn destructor(&self);
    fn get(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_eq(&self, x: Ptr<Counter_int_>) -> ();
    fn operator_lt(&self, x: Ptr<Counter_int_>) -> ();
}
impl Counter_int_Impl for Ptr<Counter_int_> {
    fn destructor(&self) {
        (*gConstructed_21.with(Value::clone).borrow_mut()).prefix_dec();
    }
    fn operator_eq(&self, x: Ptr<Counter_int_>) -> () {
        return ({ (*self).with(|__s| __s.data_) } == { x.with(|__s| __s.data_) });
    }
    fn operator_lt(&self, x: Ptr<Counter_int_>) -> () {
        return ({ (*self).with(|__s| __s.data_) } < { x.with(|__s| __s.data_) });
    }
}
pub trait MoveOnlyImpl {
    fn move_assign(&self, x: Ptr<MoveOnly>) -> Ptr<MoveOnly>;
    fn get(&self) -> ();
    fn operator_add(&self, x: Ptr<MoveOnly>) -> MoveOnly;
    fn operator_mul(&self, x: Ptr<MoveOnly>) -> MoveOnly;
}
impl MoveOnlyImpl for Ptr<MoveOnly> {
    fn move_assign(&self, x: Ptr<MoveOnly>) -> Ptr<MoveOnly> {
        field!((*self), data_).write({ x.with(|__s| __s.data_) });
        field!(x, data_).write(0_());
        return (*self).clone();
    }
    fn get(&self) -> () {
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
    fn get(&self) -> ();
    fn operator_add(&self, x: Ptr<TrivialMoveOnly>) -> TrivialMoveOnly;
    fn operator_mul(&self, x: Ptr<TrivialMoveOnly>) -> TrivialMoveOnly;
}
impl TrivialMoveOnlyImpl for Ptr<TrivialMoveOnly> {
    fn move_assign(&self, _a0: Ptr<TrivialMoveOnly>) -> Ptr<TrivialMoveOnly> {
        field!((*self), data_).write({ { (*_a0.upgrade().deref()).data_ } });
        return (*self).clone();
    }
    fn get(&self) -> () {
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
pub trait other_allocator_MoveOnly_Impl {
    fn allocate(&self, mut n: ()) -> Ptr<MoveOnly> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<MoveOnly>, mut s: ()) {
        unimplemented!()
    }
    fn select_on_container_copy_construction(&self) -> other_allocator_MoveOnly_ {
        unimplemented!()
    }
    fn get_data(&self) -> () {
        unimplemented!()
    }
}
impl other_allocator_MoveOnly_Impl for Ptr<other_allocator_MoveOnly_> {}
pub trait test_allocator_Counter_int__Impl {
    fn destructor(&self);
    fn address_11(&self, x: ()) -> Ptr<Counter_int_> {
        unimplemented!()
    }
    fn address_12(&self, x: ()) -> Ptr<Counter_int_> {
        unimplemented!()
    }
    fn allocate(&self, mut n: (), mut _a1: ()) -> Ptr<Counter_int_> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<Counter_int_>, mut s: ()) {
        unimplemented!()
    }
    fn max_size(&self) -> () {
        unimplemented!()
    }
    fn destroy(&self, mut p: Ptr<Counter_int_>) {
        unimplemented!()
    }
    fn get_data(&self) -> () {
        unimplemented!()
    }
    fn get_id(&self) -> () {
        unimplemented!()
    }
}
impl test_allocator_Counter_int__Impl for Ptr<test_allocator_Counter_int__> {
    fn destructor(&self) {
        unimplemented!();
        unimplemented!();
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_dec());
        }
        field!((*self), data_).write(destructed_value_15.with(|rc| *rc.borrow()));
        field!((*self), id_).write(destructed_value_15.with(|rc| *rc.borrow()));
    }
}
pub trait test_allocator_MoveOnly_Impl {
    fn destructor(&self);
    fn address_11(&self, x: ()) -> Ptr<MoveOnly> {
        unimplemented!()
    }
    fn address_12(&self, x: ()) -> Ptr<MoveOnly> {
        unimplemented!()
    }
    fn allocate(&self, mut n: (), mut _a1: ()) -> Ptr<MoveOnly> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<MoveOnly>, mut s: ()) {
        unimplemented!()
    }
    fn max_size(&self) -> () {
        unimplemented!()
    }
    fn destroy(&self, mut p: Ptr<MoveOnly>) {
        unimplemented!()
    }
    fn get_data(&self) -> () {
        unimplemented!()
    }
    fn get_id(&self) -> () {
        unimplemented!()
    }
}
impl test_allocator_MoveOnly_Impl for Ptr<test_allocator_MoveOnly_> {
    fn destructor(&self) {
        unimplemented!();
        unimplemented!();
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_dec());
        }
        field!((*self), data_).write(destructed_value_15.with(|rc| *rc.borrow()));
        field!((*self), id_).write(destructed_value_15.with(|rc| *rc.borrow()));
    }
}
pub trait test_allocator_statisticsImpl {
    fn clear(&self);
    fn clear_ctor_counters(&self);
}
impl test_allocator_statisticsImpl for Ptr<test_allocator_statistics> {
    fn clear(&self) {
        unimplemented!();
        field!((*self), count).write(0_());
        field!((*self), time_to_throw).write(0_());
        field!((*self), alloc_count).write(0_());
        field!((*self), allocated_size).write(0_());
        field!((*self), construct_count).write(0_());
        field!((*self), destroy_count).write(0_());
        field!((*self), throw_after).write(2147483647_());
        ({ test_allocator_statisticsImpl::clear_ctor_counters(self) });
    }
    fn clear_ctor_counters(&self) {
        field!((*self), copied).write(0_());
        field!((*self), moved).write(0_());
        field!((*self), converted).write(0_());
    }
}
pub trait test_allocator_void_Impl {
    fn destructor(&self);
    fn get_id(&self) -> ();
    fn get_data(&self) -> ();
    fn copy_assign(&self, _a0: Ptr<test_allocator_void_>) -> Ptr<test_allocator_void_>;
}
impl test_allocator_void_Impl for Ptr<test_allocator_void_> {
    fn destructor(&self) {
        field!((*self), data_).write(destructed_value_15.with(|rc| *rc.borrow()));
        field!((*self), id_).write(destructed_value_15.with(|rc| *rc.borrow()));
    }
    fn get_id(&self) -> () {
        return (*self).with(|__s| __s.id_);
    }
    fn get_data(&self) -> () {
        return (*self).with(|__s| __s.data_);
    }
    fn copy_assign(&self, _a0: Ptr<test_allocator_void_>) -> Ptr<test_allocator_void_> {
        field!((*self), data_).write({ _a0.with(|__s| __s.data_) });
        field!((*self), id_).write({ _a0.with(|__s| __s.id_) });
        field!((*self), stats_).write({ _a0.with(|__s| __s.stats_.clone()) });
        return (*self).clone();
    }
}
pub trait test_less_MoveOnly_Impl {
    fn operator_call(&self, a: Ptr<MoveOnly>, b: Ptr<MoveOnly>) -> ();
}
impl test_less_MoveOnly_Impl for Ptr<test_less_MoveOnly_> {
    fn operator_call(&self, a: Ptr<MoveOnly>, b: Ptr<MoveOnly>) -> () {
        return ({
            let _x: Ptr<MoveOnly> = (a).clone();
            let _y: Ptr<MoveOnly> = (b).clone();
            operator_lt_2(_x, _y)
        });
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = destructed_value_15.with(|_| ());
    let _ = moved_value_16.with(|_| ());
    let _ = gConstructed_21.with(|_| ());
}
