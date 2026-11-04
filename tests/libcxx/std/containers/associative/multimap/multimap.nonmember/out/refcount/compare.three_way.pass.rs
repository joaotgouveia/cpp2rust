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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_allocator_std_pair_const_int__int___rebind_std_pair_const_int__int__ {}
impl CppMove for test_allocator_std_pair_const_int__int___rebind_std_pair_const_int__int__ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, Destructor)]
#[byte_size(16)]
pub struct test_allocator_std_pair_const_int__int__ {
    #[offset(0)]
    data_: i32,
    #[offset(4)]
    id_: i32,
    #[offset(8)]
    #[byte_size(8)]
    stats_: Ptr<test_allocator_statistics>,
}
impl test_allocator_std_pair_const_int__int__ {
    pub fn copy_from(a: Ptr<test_allocator_std_pair_const_int__int__>) -> Self {
        let __this: Value<test_allocator_std_pair_const_int__int__> = Rc::new(RefCell::new(Self {
            data_: a.with(|__s| __s.data_),
            id_: a.with(|__s| __s.id_),
            stats_: a.with(|__s| __s.stats_.clone()),
        }));
        let this: Ptr<test_allocator_std_pair_const_int__int__> = __this.as_pointer();
        assert!(
            (({ a.with(|__s| __s.data_) } != { destructed_value_0.with(|rc| *rc.borrow()) })
                && ({ a.with(|__s| __s.id_) } != { destructed_value_0.with(|rc| *rc.borrow()) }))
                && (!(Ptr::<i8>::from_string_literal(b"copying from destroyed allocator"))
                    .is_null())
        );
        if !((this.with(|__s| __s.stats_.clone())).is_null()) {
            field!(this.with(|__s| __s.stats_.clone()), count).with_mut(|__v| {
                __v.prefix_inc();
            });
            field!(this.with(|__s| __s.stats_.clone()), copied).with_mut(|__v| {
                __v.prefix_inc();
            });
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for test_allocator_std_pair_const_int__int__ {
    fn clone(&self) -> Self {
        let __src: Value<test_allocator_std_pair_const_int__int__> =
            Rc::new(RefCell::new(test_allocator_std_pair_const_int__int__ {
                data_: self.data_.clone(),
                id_: self.id_.clone(),
                stats_: self.stats_.clone(),
            }));
        test_allocator_std_pair_const_int__int__::copy_from(__src.as_pointer())
    }
}
impl CppMove for test_allocator_std_pair_const_int__int__ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for test_allocator_std_pair_const_int__int__ {
    fn default() -> Self {
        {
            test_allocator_std_pair_const_int__int__ {
                data_: 0,
                id_: 0,
                stats_: Ptr::<test_allocator_statistics>::null(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_allocator_std_pair_const_int__StrongOrder___rebind_std_pair_const_int__StrongOrder__
{}
impl CppMove
    for test_allocator_std_pair_const_int__StrongOrder___rebind_std_pair_const_int__StrongOrder__
{
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, Destructor)]
#[byte_size(16)]
pub struct test_allocator_std_pair_const_int__StrongOrder__ {
    #[offset(0)]
    data_: i32,
    #[offset(4)]
    id_: i32,
    #[offset(8)]
    #[byte_size(8)]
    stats_: Ptr<test_allocator_statistics>,
}
impl test_allocator_std_pair_const_int__StrongOrder__ {
    pub fn copy_from(a: Ptr<test_allocator_std_pair_const_int__StrongOrder__>) -> Self {
        let __this: Value<test_allocator_std_pair_const_int__StrongOrder__> =
            Rc::new(RefCell::new(Self {
                data_: a.with(|__s| __s.data_),
                id_: a.with(|__s| __s.id_),
                stats_: a.with(|__s| __s.stats_.clone()),
            }));
        let this: Ptr<test_allocator_std_pair_const_int__StrongOrder__> = __this.as_pointer();
        assert!(
            (({ a.with(|__s| __s.data_) } != { destructed_value_0.with(|rc| *rc.borrow()) })
                && ({ a.with(|__s| __s.id_) } != { destructed_value_0.with(|rc| *rc.borrow()) }))
                && (!(Ptr::<i8>::from_string_literal(b"copying from destroyed allocator"))
                    .is_null())
        );
        if !((this.with(|__s| __s.stats_.clone())).is_null()) {
            field!(this.with(|__s| __s.stats_.clone()), count).with_mut(|__v| {
                __v.prefix_inc();
            });
            field!(this.with(|__s| __s.stats_.clone()), copied).with_mut(|__v| {
                __v.prefix_inc();
            });
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for test_allocator_std_pair_const_int__StrongOrder__ {
    fn clone(&self) -> Self {
        let __src: Value<test_allocator_std_pair_const_int__StrongOrder__> = Rc::new(RefCell::new(
            test_allocator_std_pair_const_int__StrongOrder__ {
                data_: self.data_.clone(),
                id_: self.id_.clone(),
                stats_: self.stats_.clone(),
            },
        ));
        test_allocator_std_pair_const_int__StrongOrder__::copy_from(__src.as_pointer())
    }
}
impl CppMove for test_allocator_std_pair_const_int__StrongOrder__ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for test_allocator_std_pair_const_int__StrongOrder__ {
    fn default() -> Self {
        {
            test_allocator_std_pair_const_int__StrongOrder__ {
                data_: 0,
                id_: 0,
                stats_: Ptr::<test_allocator_statistics>::null(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_allocator_std_pair_const_int__WeakOrder___rebind_std_pair_const_int__WeakOrder__ {
}
impl CppMove
    for test_allocator_std_pair_const_int__WeakOrder___rebind_std_pair_const_int__WeakOrder__
{
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, Destructor)]
#[byte_size(16)]
pub struct test_allocator_std_pair_const_int__WeakOrder__ {
    #[offset(0)]
    data_: i32,
    #[offset(4)]
    id_: i32,
    #[offset(8)]
    #[byte_size(8)]
    stats_: Ptr<test_allocator_statistics>,
}
impl test_allocator_std_pair_const_int__WeakOrder__ {
    pub fn copy_from(a: Ptr<test_allocator_std_pair_const_int__WeakOrder__>) -> Self {
        let __this: Value<test_allocator_std_pair_const_int__WeakOrder__> =
            Rc::new(RefCell::new(Self {
                data_: a.with(|__s| __s.data_),
                id_: a.with(|__s| __s.id_),
                stats_: a.with(|__s| __s.stats_.clone()),
            }));
        let this: Ptr<test_allocator_std_pair_const_int__WeakOrder__> = __this.as_pointer();
        assert!(
            (({ a.with(|__s| __s.data_) } != { destructed_value_0.with(|rc| *rc.borrow()) })
                && ({ a.with(|__s| __s.id_) } != { destructed_value_0.with(|rc| *rc.borrow()) }))
                && (!(Ptr::<i8>::from_string_literal(b"copying from destroyed allocator"))
                    .is_null())
        );
        if !((this.with(|__s| __s.stats_.clone())).is_null()) {
            field!(this.with(|__s| __s.stats_.clone()), count).with_mut(|__v| {
                __v.prefix_inc();
            });
            field!(this.with(|__s| __s.stats_.clone()), copied).with_mut(|__v| {
                __v.prefix_inc();
            });
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for test_allocator_std_pair_const_int__WeakOrder__ {
    fn clone(&self) -> Self {
        let __src: Value<test_allocator_std_pair_const_int__WeakOrder__> = Rc::new(RefCell::new(
            test_allocator_std_pair_const_int__WeakOrder__ {
                data_: self.data_.clone(),
                id_: self.id_.clone(),
                stats_: self.stats_.clone(),
            },
        ));
        test_allocator_std_pair_const_int__WeakOrder__::copy_from(__src.as_pointer())
    }
}
impl CppMove for test_allocator_std_pair_const_int__WeakOrder__ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for test_allocator_std_pair_const_int__WeakOrder__ {
    fn default() -> Self {
        {
            test_allocator_std_pair_const_int__WeakOrder__ {
                data_: 0,
                id_: 0,
                stats_: Ptr::<test_allocator_statistics>::null(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_allocator_std_pair_const_int__PartialOrder___rebind_std_pair_const_int__PartialOrder__
{}
impl CppMove
    for test_allocator_std_pair_const_int__PartialOrder___rebind_std_pair_const_int__PartialOrder__
{
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, Destructor)]
#[byte_size(16)]
pub struct test_allocator_std_pair_const_int__PartialOrder__ {
    #[offset(0)]
    data_: i32,
    #[offset(4)]
    id_: i32,
    #[offset(8)]
    #[byte_size(8)]
    stats_: Ptr<test_allocator_statistics>,
}
impl test_allocator_std_pair_const_int__PartialOrder__ {
    pub fn copy_from(a: Ptr<test_allocator_std_pair_const_int__PartialOrder__>) -> Self {
        let __this: Value<test_allocator_std_pair_const_int__PartialOrder__> =
            Rc::new(RefCell::new(Self {
                data_: a.with(|__s| __s.data_),
                id_: a.with(|__s| __s.id_),
                stats_: a.with(|__s| __s.stats_.clone()),
            }));
        let this: Ptr<test_allocator_std_pair_const_int__PartialOrder__> = __this.as_pointer();
        assert!(
            (({ a.with(|__s| __s.data_) } != { destructed_value_0.with(|rc| *rc.borrow()) })
                && ({ a.with(|__s| __s.id_) } != { destructed_value_0.with(|rc| *rc.borrow()) }))
                && (!(Ptr::<i8>::from_string_literal(b"copying from destroyed allocator"))
                    .is_null())
        );
        if !((this.with(|__s| __s.stats_.clone())).is_null()) {
            field!(this.with(|__s| __s.stats_.clone()), count).with_mut(|__v| {
                __v.prefix_inc();
            });
            field!(this.with(|__s| __s.stats_.clone()), copied).with_mut(|__v| {
                __v.prefix_inc();
            });
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for test_allocator_std_pair_const_int__PartialOrder__ {
    fn clone(&self) -> Self {
        let __src: Value<test_allocator_std_pair_const_int__PartialOrder__> = Rc::new(
            RefCell::new(test_allocator_std_pair_const_int__PartialOrder__ {
                data_: self.data_.clone(),
                id_: self.id_.clone(),
                stats_: self.stats_.clone(),
            }),
        );
        test_allocator_std_pair_const_int__PartialOrder__::copy_from(__src.as_pointer())
    }
}
impl CppMove for test_allocator_std_pair_const_int__PartialOrder__ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for test_allocator_std_pair_const_int__PartialOrder__ {
    fn default() -> Self {
        {
            test_allocator_std_pair_const_int__PartialOrder__ {
                data_: 0,
                id_: 0,
                stats_: Ptr::<test_allocator_statistics>::null(),
            }
        }
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
pub fn testComparisonsComplete_4(
    t1: Ptr<()>,
    t2: Ptr<()>,
    mut isEqual: bool,
    mut isLess: bool,
    mut isGreater: bool,
) -> bool {
    assert!(
        ((((if isEqual { 1 } else { 0 }) + (if isLess { 1 } else { 0 }))
            + (if isGreater { 1 } else { 0 }))
            <= 1)
            && (!(Ptr::<i8>::from_string_literal(
                b"at most one of isEqual, isLess, and isGreater can be true"
            ))
            .is_null())
    );
    if isEqual {
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (!(unimplemented!())) {
            return false;
        }
        if (!(unimplemented!())) {
            return false;
        }
        if (unimplemented!() == std::cmp::Ordering::Less) {
            return false;
        }
        if (unimplemented!() == std::cmp::Ordering::Less) {
            return false;
        }
        if !(unimplemented!() != std::cmp::Ordering::Greater) {
            return false;
        }
        if !(unimplemented!() != std::cmp::Ordering::Greater) {
            return false;
        }
        if (unimplemented!() == std::cmp::Ordering::Greater) {
            return false;
        }
        if (unimplemented!() == std::cmp::Ordering::Greater) {
            return false;
        }
        if !(unimplemented!() != std::cmp::Ordering::Less) {
            return false;
        }
        if !(unimplemented!() != std::cmp::Ordering::Less) {
            return false;
        }
    } else if isLess {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(unimplemented!() == std::cmp::Ordering::Less) {
            return false;
        }
        if (unimplemented!() == std::cmp::Ordering::Less) {
            return false;
        }
        if !(unimplemented!() != std::cmp::Ordering::Greater) {
            return false;
        }
        if (unimplemented!() != std::cmp::Ordering::Greater) {
            return false;
        }
        if (unimplemented!() == std::cmp::Ordering::Greater) {
            return false;
        }
        if !(unimplemented!() == std::cmp::Ordering::Greater) {
            return false;
        }
        if (unimplemented!() != std::cmp::Ordering::Less) {
            return false;
        }
        if !(unimplemented!() != std::cmp::Ordering::Less) {
            return false;
        }
    } else if isGreater {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if (unimplemented!() == std::cmp::Ordering::Less) {
            return false;
        }
        if !(unimplemented!() == std::cmp::Ordering::Less) {
            return false;
        }
        if (unimplemented!() != std::cmp::Ordering::Greater) {
            return false;
        }
        if !(unimplemented!() != std::cmp::Ordering::Greater) {
            return false;
        }
        if !(unimplemented!() == std::cmp::Ordering::Greater) {
            return false;
        }
        if (unimplemented!() == std::cmp::Ordering::Greater) {
            return false;
        }
        if !(unimplemented!() != std::cmp::Ordering::Less) {
            return false;
        }
        if (unimplemented!() != std::cmp::Ordering::Less) {
            return false;
        }
    } else {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if (unimplemented!() == std::cmp::Ordering::Less) {
            return false;
        }
        if (unimplemented!() == std::cmp::Ordering::Less) {
            return false;
        }
        if (unimplemented!() != std::cmp::Ordering::Greater) {
            return false;
        }
        if (unimplemented!() != std::cmp::Ordering::Greater) {
            return false;
        }
        if (unimplemented!() == std::cmp::Ordering::Greater) {
            return false;
        }
        if (unimplemented!() == std::cmp::Ordering::Greater) {
            return false;
        }
        if (unimplemented!() != std::cmp::Ordering::Less) {
            return false;
        }
        if (unimplemented!() != std::cmp::Ordering::Less) {
            return false;
        }
    }
    return true;
}
pub fn testComparisonsComplete_5(
    t1: Ptr<()>,
    t2: Ptr<()>,
    mut isEqual: bool,
    mut isLess: bool,
    mut isGreater: bool,
) -> bool {
    assert!(
        ((((if isEqual { 1 } else { 0 }) + (if isLess { 1 } else { 0 }))
            + (if isGreater { 1 } else { 0 }))
            <= 1)
            && (!(Ptr::<i8>::from_string_literal(
                b"at most one of isEqual, isLess, and isGreater can be true"
            ))
            .is_null())
    );
    if isEqual {
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (!(unimplemented!())) {
            return false;
        }
        if (!(unimplemented!())) {
            return false;
        }
        if (unimplemented!() == std::cmp::Ordering::Less) {
            return false;
        }
        if (unimplemented!() == std::cmp::Ordering::Less) {
            return false;
        }
        if !(unimplemented!() != std::cmp::Ordering::Greater) {
            return false;
        }
        if !(unimplemented!() != std::cmp::Ordering::Greater) {
            return false;
        }
        if (unimplemented!() == std::cmp::Ordering::Greater) {
            return false;
        }
        if (unimplemented!() == std::cmp::Ordering::Greater) {
            return false;
        }
        if !(unimplemented!() != std::cmp::Ordering::Less) {
            return false;
        }
        if !(unimplemented!() != std::cmp::Ordering::Less) {
            return false;
        }
    } else if isLess {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(unimplemented!() == std::cmp::Ordering::Less) {
            return false;
        }
        if (unimplemented!() == std::cmp::Ordering::Less) {
            return false;
        }
        if !(unimplemented!() != std::cmp::Ordering::Greater) {
            return false;
        }
        if (unimplemented!() != std::cmp::Ordering::Greater) {
            return false;
        }
        if (unimplemented!() == std::cmp::Ordering::Greater) {
            return false;
        }
        if !(unimplemented!() == std::cmp::Ordering::Greater) {
            return false;
        }
        if (unimplemented!() != std::cmp::Ordering::Less) {
            return false;
        }
        if !(unimplemented!() != std::cmp::Ordering::Less) {
            return false;
        }
    } else if isGreater {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if (unimplemented!() == std::cmp::Ordering::Less) {
            return false;
        }
        if !(unimplemented!() == std::cmp::Ordering::Less) {
            return false;
        }
        if (unimplemented!() != std::cmp::Ordering::Greater) {
            return false;
        }
        if !(unimplemented!() != std::cmp::Ordering::Greater) {
            return false;
        }
        if !(unimplemented!() == std::cmp::Ordering::Greater) {
            return false;
        }
        if (unimplemented!() == std::cmp::Ordering::Greater) {
            return false;
        }
        if !(unimplemented!() != std::cmp::Ordering::Less) {
            return false;
        }
        if (unimplemented!() != std::cmp::Ordering::Less) {
            return false;
        }
    } else {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if (unimplemented!() == std::cmp::Ordering::Less) {
            return false;
        }
        if (unimplemented!() == std::cmp::Ordering::Less) {
            return false;
        }
        if (unimplemented!() != std::cmp::Ordering::Greater) {
            return false;
        }
        if (unimplemented!() != std::cmp::Ordering::Greater) {
            return false;
        }
        if (unimplemented!() == std::cmp::Ordering::Greater) {
            return false;
        }
        if (unimplemented!() == std::cmp::Ordering::Greater) {
            return false;
        }
        if (unimplemented!() != std::cmp::Ordering::Less) {
            return false;
        }
        if (unimplemented!() != std::cmp::Ordering::Less) {
            return false;
        }
    }
    return true;
}
pub fn testComparisonsComplete_6(
    t1: Ptr<()>,
    t2: Ptr<()>,
    mut isEqual: bool,
    mut isLess: bool,
    mut isGreater: bool,
) -> bool {
    assert!(
        ((((if isEqual { 1 } else { 0 }) + (if isLess { 1 } else { 0 }))
            + (if isGreater { 1 } else { 0 }))
            <= 1)
            && (!(Ptr::<i8>::from_string_literal(
                b"at most one of isEqual, isLess, and isGreater can be true"
            ))
            .is_null())
    );
    if isEqual {
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (!(unimplemented!())) {
            return false;
        }
        if (!(unimplemented!())) {
            return false;
        }
        if (unimplemented!() == std::cmp::Ordering::Less) {
            return false;
        }
        if (unimplemented!() == std::cmp::Ordering::Less) {
            return false;
        }
        if !(unimplemented!() != std::cmp::Ordering::Greater) {
            return false;
        }
        if !(unimplemented!() != std::cmp::Ordering::Greater) {
            return false;
        }
        if (unimplemented!() == std::cmp::Ordering::Greater) {
            return false;
        }
        if (unimplemented!() == std::cmp::Ordering::Greater) {
            return false;
        }
        if !(unimplemented!() != std::cmp::Ordering::Less) {
            return false;
        }
        if !(unimplemented!() != std::cmp::Ordering::Less) {
            return false;
        }
    } else if isLess {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(unimplemented!() == std::cmp::Ordering::Less) {
            return false;
        }
        if (unimplemented!() == std::cmp::Ordering::Less) {
            return false;
        }
        if !(unimplemented!() != std::cmp::Ordering::Greater) {
            return false;
        }
        if (unimplemented!() != std::cmp::Ordering::Greater) {
            return false;
        }
        if (unimplemented!() == std::cmp::Ordering::Greater) {
            return false;
        }
        if !(unimplemented!() == std::cmp::Ordering::Greater) {
            return false;
        }
        if (unimplemented!() != std::cmp::Ordering::Less) {
            return false;
        }
        if !(unimplemented!() != std::cmp::Ordering::Less) {
            return false;
        }
    } else if isGreater {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if (unimplemented!() == std::cmp::Ordering::Less) {
            return false;
        }
        if !(unimplemented!() == std::cmp::Ordering::Less) {
            return false;
        }
        if (unimplemented!() != std::cmp::Ordering::Greater) {
            return false;
        }
        if !(unimplemented!() != std::cmp::Ordering::Greater) {
            return false;
        }
        if !(unimplemented!() == std::cmp::Ordering::Greater) {
            return false;
        }
        if (unimplemented!() == std::cmp::Ordering::Greater) {
            return false;
        }
        if !(unimplemented!() != std::cmp::Ordering::Less) {
            return false;
        }
        if (unimplemented!() != std::cmp::Ordering::Less) {
            return false;
        }
    } else {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if (unimplemented!() == std::cmp::Ordering::Less) {
            return false;
        }
        if (unimplemented!() == std::cmp::Ordering::Less) {
            return false;
        }
        if (unimplemented!() != std::cmp::Ordering::Greater) {
            return false;
        }
        if (unimplemented!() != std::cmp::Ordering::Greater) {
            return false;
        }
        if (unimplemented!() == std::cmp::Ordering::Greater) {
            return false;
        }
        if (unimplemented!() == std::cmp::Ordering::Greater) {
            return false;
        }
        if (unimplemented!() != std::cmp::Ordering::Less) {
            return false;
        }
        if (unimplemented!() != std::cmp::Ordering::Less) {
            return false;
        }
    }
    return true;
}
pub fn testComparisonsComplete_7(
    t1: Ptr<()>,
    t2: Ptr<()>,
    mut isEqual: bool,
    mut isLess: bool,
    mut isGreater: bool,
) -> bool {
    assert!(
        ((((if isEqual { 1 } else { 0 }) + (if isLess { 1 } else { 0 }))
            + (if isGreater { 1 } else { 0 }))
            <= 1)
            && (!(Ptr::<i8>::from_string_literal(
                b"at most one of isEqual, isLess, and isGreater can be true"
            ))
            .is_null())
    );
    if isEqual {
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (!(unimplemented!())) {
            return false;
        }
        if (!(unimplemented!())) {
            return false;
        }
        if (unimplemented!() == std::cmp::Ordering::Less) {
            return false;
        }
        if (unimplemented!() == std::cmp::Ordering::Less) {
            return false;
        }
        if !(unimplemented!() != std::cmp::Ordering::Greater) {
            return false;
        }
        if !(unimplemented!() != std::cmp::Ordering::Greater) {
            return false;
        }
        if (unimplemented!() == std::cmp::Ordering::Greater) {
            return false;
        }
        if (unimplemented!() == std::cmp::Ordering::Greater) {
            return false;
        }
        if !(unimplemented!() != std::cmp::Ordering::Less) {
            return false;
        }
        if !(unimplemented!() != std::cmp::Ordering::Less) {
            return false;
        }
    } else if isLess {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(unimplemented!() == std::cmp::Ordering::Less) {
            return false;
        }
        if (unimplemented!() == std::cmp::Ordering::Less) {
            return false;
        }
        if !(unimplemented!() != std::cmp::Ordering::Greater) {
            return false;
        }
        if (unimplemented!() != std::cmp::Ordering::Greater) {
            return false;
        }
        if (unimplemented!() == std::cmp::Ordering::Greater) {
            return false;
        }
        if !(unimplemented!() == std::cmp::Ordering::Greater) {
            return false;
        }
        if (unimplemented!() != std::cmp::Ordering::Less) {
            return false;
        }
        if !(unimplemented!() != std::cmp::Ordering::Less) {
            return false;
        }
    } else if isGreater {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if (unimplemented!() == std::cmp::Ordering::Less) {
            return false;
        }
        if !(unimplemented!() == std::cmp::Ordering::Less) {
            return false;
        }
        if (unimplemented!() != std::cmp::Ordering::Greater) {
            return false;
        }
        if !(unimplemented!() != std::cmp::Ordering::Greater) {
            return false;
        }
        if !(unimplemented!() == std::cmp::Ordering::Greater) {
            return false;
        }
        if (unimplemented!() == std::cmp::Ordering::Greater) {
            return false;
        }
        if !(unimplemented!() != std::cmp::Ordering::Less) {
            return false;
        }
        if (unimplemented!() != std::cmp::Ordering::Less) {
            return false;
        }
    } else {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if (unimplemented!() == std::cmp::Ordering::Less) {
            return false;
        }
        if (unimplemented!() == std::cmp::Ordering::Less) {
            return false;
        }
        if (unimplemented!() != std::cmp::Ordering::Greater) {
            return false;
        }
        if (unimplemented!() != std::cmp::Ordering::Greater) {
            return false;
        }
        if (unimplemented!() == std::cmp::Ordering::Greater) {
            return false;
        }
        if (unimplemented!() == std::cmp::Ordering::Greater) {
            return false;
        }
        if (unimplemented!() != std::cmp::Ordering::Less) {
            return false;
        }
        if (unimplemented!() != std::cmp::Ordering::Less) {
            return false;
        }
    }
    return true;
}
pub fn testComparisonsComplete_8(
    t1: Ptr<()>,
    t2: Ptr<()>,
    mut isEqual: bool,
    mut isLess: bool,
    mut isGreater: bool,
) -> bool {
    assert!(
        ((((if isEqual { 1 } else { 0 }) + (if isLess { 1 } else { 0 }))
            + (if isGreater { 1 } else { 0 }))
            <= 1)
            && (!(Ptr::<i8>::from_string_literal(
                b"at most one of isEqual, isLess, and isGreater can be true"
            ))
            .is_null())
    );
    if isEqual {
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (!(unimplemented!())) {
            return false;
        }
        if (!(unimplemented!())) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
    } else if isLess {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
    } else if isGreater {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
    } else {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
    }
    return true;
}
pub fn testComparisonsComplete_9(
    t1: Ptr<()>,
    t2: Ptr<()>,
    mut isEqual: bool,
    mut isLess: bool,
    mut isGreater: bool,
) -> bool {
    assert!(
        ((((if isEqual { 1 } else { 0 }) + (if isLess { 1 } else { 0 }))
            + (if isGreater { 1 } else { 0 }))
            <= 1)
            && (!(Ptr::<i8>::from_string_literal(
                b"at most one of isEqual, isLess, and isGreater can be true"
            ))
            .is_null())
    );
    if isEqual {
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (!(unimplemented!())) {
            return false;
        }
        if (!(unimplemented!())) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
    } else if isLess {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
    } else if isGreater {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
    } else {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
    }
    return true;
}
pub fn testComparisonsComplete_10(
    t1: Ptr<()>,
    t2: Ptr<()>,
    mut isEqual: bool,
    mut isLess: bool,
    mut isGreater: bool,
) -> bool {
    assert!(
        ((((if isEqual { 1 } else { 0 }) + (if isLess { 1 } else { 0 }))
            + (if isGreater { 1 } else { 0 }))
            <= 1)
            && (!(Ptr::<i8>::from_string_literal(
                b"at most one of isEqual, isLess, and isGreater can be true"
            ))
            .is_null())
    );
    if isEqual {
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (!(unimplemented!())) {
            return false;
        }
        if (!(unimplemented!())) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
    } else if isLess {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
    } else if isGreater {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
    } else {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
    }
    return true;
}
pub fn testComparisonsComplete_11(
    t1: Ptr<()>,
    t2: Ptr<()>,
    mut isEqual: bool,
    mut isLess: bool,
    mut isGreater: bool,
) -> bool {
    assert!(
        ((((if isEqual { 1 } else { 0 }) + (if isLess { 1 } else { 0 }))
            + (if isGreater { 1 } else { 0 }))
            <= 1)
            && (!(Ptr::<i8>::from_string_literal(
                b"at most one of isEqual, isLess, and isGreater can be true"
            ))
            .is_null())
    );
    if isEqual {
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (!(unimplemented!())) {
            return false;
        }
        if (!(unimplemented!())) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
    } else if isLess {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
    } else if isGreater {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
    } else {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
    }
    return true;
}
pub fn testComparisonsComplete_12(
    t1: Ptr<()>,
    t2: Ptr<()>,
    mut isEqual: bool,
    mut isLess: bool,
    mut isGreater: bool,
) -> bool {
    assert!(
        ((((if isEqual { 1 } else { 0 }) + (if isLess { 1 } else { 0 }))
            + (if isGreater { 1 } else { 0 }))
            <= 1)
            && (!(Ptr::<i8>::from_string_literal(
                b"at most one of isEqual, isLess, and isGreater can be true"
            ))
            .is_null())
    );
    if isEqual {
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (!(unimplemented!())) {
            return false;
        }
        if (!(unimplemented!())) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
    } else if isLess {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
    } else if isGreater {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
    } else {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
    }
    return true;
}
pub fn testOrder_13(t1: Ptr<()>, t2: Ptr<()>, order: std::cmp::Ordering) -> bool {
    let order: Value<std::cmp::Ordering> = Rc::new(RefCell::new(order));
    let mut equal: bool = ({ (*order.borrow()) })
        == ({ (*libcc2rs::strong_ordering_equivalent().upgrade().deref()) });
    let mut less: bool =
        ({ (*order.borrow()) }) == ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) });
    let mut greater: bool =
        ({ (*order.borrow()) }) == ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) });
    return (unimplemented!() == ({ (*order.borrow()) }))
        && ({
            let _t1: Ptr<()> = (t1).clone();
            let _t2: Ptr<()> = (t2).clone();
            let _isEqual: bool = equal;
            let _isLess: bool = less;
            let _isGreater: bool = greater;
            testComparisonsComplete_4(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub fn testOrder_14(t1: Ptr<()>, t2: Ptr<()>, order: std::cmp::Ordering) -> bool {
    let order: Value<std::cmp::Ordering> = Rc::new(RefCell::new(order));
    let mut equal: bool = ({ (*order.borrow()) })
        == ({ (*libcc2rs::strong_ordering_equivalent().upgrade().deref()) });
    let mut less: bool =
        ({ (*order.borrow()) }) == ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) });
    let mut greater: bool =
        ({ (*order.borrow()) }) == ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) });
    return (unimplemented!() == ({ (*order.borrow()) }))
        && ({
            let _t1: Ptr<()> = (t1).clone();
            let _t2: Ptr<()> = (t2).clone();
            let _isEqual: bool = equal;
            let _isLess: bool = less;
            let _isGreater: bool = greater;
            testComparisonsComplete_5(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub fn testOrder_15(t1: Ptr<()>, t2: Ptr<()>, order: std::cmp::Ordering) -> bool {
    let order: Value<std::cmp::Ordering> = Rc::new(RefCell::new(order));
    let mut equal: bool = ({ (*order.borrow()) })
        == ({ (*libcc2rs::strong_ordering_equivalent().upgrade().deref()) });
    let mut less: bool =
        ({ (*order.borrow()) }) == ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) });
    let mut greater: bool =
        ({ (*order.borrow()) }) == ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) });
    return (unimplemented!() == ({ (*order.borrow()) }))
        && ({
            let _t1: Ptr<()> = (t1).clone();
            let _t2: Ptr<()> = (t2).clone();
            let _isEqual: bool = equal;
            let _isLess: bool = less;
            let _isGreater: bool = greater;
            testComparisonsComplete_6(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub fn testOrder_16(t1: Ptr<()>, t2: Ptr<()>, order: std::cmp::Ordering) -> bool {
    let order: Value<std::cmp::Ordering> = Rc::new(RefCell::new(order));
    let mut equal: bool = ({ (*order.borrow()) })
        == ({ (*libcc2rs::strong_ordering_equivalent().upgrade().deref()) });
    let mut less: bool =
        ({ (*order.borrow()) }) == ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) });
    let mut greater: bool =
        ({ (*order.borrow()) }) == ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) });
    return (unimplemented!() == ({ (*order.borrow()) }))
        && ({
            let _t1: Ptr<()> = (t1).clone();
            let _t2: Ptr<()> = (t2).clone();
            let _isEqual: bool = equal;
            let _isLess: bool = less;
            let _isGreater: bool = greater;
            testComparisonsComplete_7(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub fn testOrder_17(t1: Ptr<()>, t2: Ptr<()>, order: ()) -> bool {
    let order: Value<()> = Rc::new(RefCell::new(order));
    let mut equal: bool = unimplemented!();
    let mut less: bool = unimplemented!();
    let mut greater: bool = unimplemented!();
    return (unimplemented!())
        && ({
            let _t1: Ptr<()> = (t1).clone();
            let _t2: Ptr<()> = (t2).clone();
            let _isEqual: bool = equal;
            let _isLess: bool = less;
            let _isGreater: bool = greater;
            testComparisonsComplete_8(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub fn testOrder_18(t1: Ptr<()>, t2: Ptr<()>, order: ()) -> bool {
    let order: Value<()> = Rc::new(RefCell::new(order));
    let mut equal: bool = unimplemented!();
    let mut less: bool = unimplemented!();
    let mut greater: bool = unimplemented!();
    return (unimplemented!())
        && ({
            let _t1: Ptr<()> = (t1).clone();
            let _t2: Ptr<()> = (t2).clone();
            let _isEqual: bool = equal;
            let _isLess: bool = less;
            let _isGreater: bool = greater;
            testComparisonsComplete_9(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub fn testOrder_19(t1: Ptr<()>, t2: Ptr<()>, order: ()) -> bool {
    let order: Value<()> = Rc::new(RefCell::new(order));
    let mut equal: bool = unimplemented!();
    let mut less: bool = unimplemented!();
    let mut greater: bool = unimplemented!();
    return (unimplemented!())
        && ({
            let _t1: Ptr<()> = (t1).clone();
            let _t2: Ptr<()> = (t2).clone();
            let _isEqual: bool = equal;
            let _isLess: bool = less;
            let _isGreater: bool = greater;
            testComparisonsComplete_10(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub fn testOrder_20(t1: Ptr<()>, t2: Ptr<()>, order: ()) -> bool {
    let order: Value<()> = Rc::new(RefCell::new(order));
    let mut equal: bool = unimplemented!();
    let mut less: bool = unimplemented!();
    let mut greater: bool = unimplemented!();
    return (unimplemented!())
        && ({
            let _t1: Ptr<()> = (t1).clone();
            let _t2: Ptr<()> = (t2).clone();
            let _isEqual: bool = equal;
            let _isLess: bool = less;
            let _isGreater: bool = greater;
            testComparisonsComplete_11(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub fn testOrder_21(t1: Ptr<()>, t2: Ptr<()>, order: ()) -> bool {
    let order: Value<()> = Rc::new(RefCell::new(order));
    let mut equal: bool = unimplemented!();
    let mut less: bool = unimplemented!();
    let mut greater: bool = unimplemented!();
    return (unimplemented!())
        && ({
            let _t1: Ptr<()> = (t1).clone();
            let _t2: Ptr<()> = (t2).clone();
            let _isEqual: bool = equal;
            let _isLess: bool = less;
            let _isGreater: bool = greater;
            testComparisonsComplete_12(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub fn operator_lt_22(lhs: Ptr<LessAndEqComp>, rhs: Ptr<LessAndEqComp>) -> bool {
    return ({ lhs.with(|__s| __s.value) } < { rhs.with(|__s| __s.value) });
}
pub fn operator_eq_23(lhs: Ptr<LessAndEqComp>, rhs: Ptr<LessAndEqComp>) -> bool {
    return ({ lhs.with(|__s| __s.value) } == { rhs.with(|__s| __s.value) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(i32, new, value)]
#[byte_size(4)]
pub struct LessAndEqComp {
    #[offset(0)]
    pub value: i32,
}
impl LessAndEqComp {
    pub fn new(mut v: i32) -> Self {
        Self { value: v }
    }
}
impl CppLess for LessAndEqComp {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_lt_22(__lhs.clone(), __rhs.clone())
    }
}
impl CppEq for LessAndEqComp {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_23(__lhs.clone(), __rhs.clone())
    }
}
impl CppMove for LessAndEqComp {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn operator_cmp_24(mut _a0: StrongOrder, mut _a1: StrongOrder) -> std::cmp::Ordering {
    {
        let cmp: Value<std::cmp::Ordering> =
            Rc::new(RefCell::new(std::cmp::Ord::cmp(&(_a0.value), &(_a1.value))));
        if !(({ (*cmp.borrow()) }) == std::cmp::Ordering::Equal) {
            return ({ (*cmp.borrow()) });
        }
    }
    return ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(i32, new, value)]
#[byte_size(4)]
pub struct StrongOrder {
    #[offset(0)]
    pub value: i32,
}
impl StrongOrder {
    pub fn new(mut v: i32) -> Self {
        Self { value: v }
    }
}
impl CppLess for StrongOrder {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_cmp_24(__lhs.read(), __rhs.read()) == std::cmp::Ordering::Less
    }
}
impl CppEq for StrongOrder {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_cmp_24(__lhs.read(), __rhs.read()) == std::cmp::Ordering::Equal
    }
}
impl CppMove for StrongOrder {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn operator_cmp_25(mut _a0: WeakOrder, mut _a1: WeakOrder) -> () {
    {
        let cmp: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        if !(unimplemented!()) {
            return unimplemented!();
        }
    }
    return unimplemented!();
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(i32, new, value)]
#[byte_size(4)]
pub struct WeakOrder {
    #[offset(0)]
    pub value: i32,
}
impl WeakOrder {
    pub fn new(mut v: i32) -> Self {
        Self { value: v }
    }
}
impl CppLess for WeakOrder {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_cmp_25(__lhs.read(), __rhs.read()) == std::cmp::Ordering::Less
    }
}
impl CppEq for WeakOrder {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_cmp_25(__lhs.read(), __rhs.read()) == std::cmp::Ordering::Equal
    }
}
impl CppMove for WeakOrder {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn operator_cmp_26(mut lhs: PartialOrder, mut rhs: PartialOrder) -> () {
    if (lhs.value == i32::MIN) || (rhs.value == i32::MIN) {
        return unimplemented!();
    }
    if (lhs.value == i32::MAX) || (rhs.value == i32::MAX) {
        return unimplemented!();
    }
    return unimplemented!();
}
pub fn operator_eq_27(mut lhs: PartialOrder, mut rhs: PartialOrder) -> bool {
    return unimplemented!();
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(i32, new, value)]
#[byte_size(4)]
pub struct PartialOrder {
    #[offset(0)]
    pub value: i32,
}
impl PartialOrder {
    pub fn new(mut v: i32) -> Self {
        Self { value: v }
    }
}
impl CppLess for PartialOrder {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_cmp_26(__lhs.read(), __rhs.read()) == std::cmp::Ordering::Less
    }
}
impl CppEq for PartialOrder {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_27(__lhs.read(), __rhs.read())
    }
}
impl CppMove for PartialOrder {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct NonComparable {}
impl CppMove for NonComparable {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
const _: () = assert!(!(false), "!std::equality_comparable<NonComparable>");
const _: () = assert!(!(false), "!HasOperatorEqual<NonComparable>");
const _: () = assert!(!(false), "!HasOperatorGreaterThan<NonComparable>");
const _: () = assert!(!(false), "!HasOperatorGreaterThanEqual<NonComparable>");
const _: () = assert!(!(false), "!HasOperatorLessThan<NonComparable>");
const _: () = assert!(!(false), "!HasOperatorLessThanEqual<NonComparable>");
const _: () = assert!(!(false), "!HasOperatorNotEqual<NonComparable>");
const _: () = assert!(!(false), "!HasOperatorSpaceship<NonComparable>");
pub fn operator_eq_28(_a0: Ptr<EqualityComparable>, _a1: Ptr<EqualityComparable>) -> bool {
    return ({ _a0.with(|__s| __s.value_) } == { _a1.with(|__s| __s.value_) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(i32, new, value)]
#[byte_size(4)]
pub struct EqualityComparable {
    #[offset(0)]
    value_: i32,
}
impl EqualityComparable {
    pub fn new(mut value: i32) -> Self {
        Self { value_: value }
    }
}
impl CppEq for EqualityComparable {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_28(__lhs.clone(), __rhs.clone())
    }
}
impl CppMove for EqualityComparable {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
const _: () = assert!(true, "std::equality_comparable<EqualityComparable>");
const _: () = assert!(true, "HasOperatorEqual<EqualityComparable>");
const _: () = assert!(true, "HasOperatorNotEqual<EqualityComparable>");
pub fn operator_eq_29(_a0: Ptr<ThreeWayComparable>, _a1: Ptr<ThreeWayComparable>) -> bool {
    return ({ _a0.with(|__s| __s.value_) } == { _a1.with(|__s| __s.value_) });
}
pub fn operator_cmp_30(
    _a0: Ptr<ThreeWayComparable>,
    _a1: Ptr<ThreeWayComparable>,
) -> std::cmp::Ordering {
    {
        let cmp: Value<std::cmp::Ordering> = Rc::new(RefCell::new(std::cmp::Ord::cmp(
            &(_a0.with(|__s| __s.value_)),
            &(_a1.with(|__s| __s.value_)),
        )));
        if !(({ (*cmp.borrow()) }) == std::cmp::Ordering::Equal) {
            return ({ (*cmp.borrow()) });
        }
    }
    return ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(i32, new, value)]
#[byte_size(4)]
pub struct ThreeWayComparable {
    #[offset(0)]
    value_: i32,
}
impl ThreeWayComparable {
    pub fn new(mut value: i32) -> Self {
        Self { value_: value }
    }
}
impl CppLess for ThreeWayComparable {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_cmp_30(__lhs.clone(), __rhs.clone()) == std::cmp::Ordering::Less
    }
}
impl CppEq for ThreeWayComparable {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_29(__lhs.clone(), __rhs.clone())
    }
}
impl CppMove for ThreeWayComparable {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
const _: () = assert!(true, "std::equality_comparable<ThreeWayComparable>");
const _: () = assert!(true, "std::three_way_comparable<ThreeWayComparable>");
const _: () = assert!(true, "HasOperatorEqual<ThreeWayComparable>");
const _: () = assert!(true, "HasOperatorGreaterThan<ThreeWayComparable>");
const _: () = assert!(true, "HasOperatorGreaterThanEqual<ThreeWayComparable>");
const _: () = assert!(true, "HasOperatorLessThan<ThreeWayComparable>");
const _: () = assert!(true, "HasOperatorLessThanEqual<ThreeWayComparable>");
const _: () = assert!(true, "HasOperatorNotEqual<ThreeWayComparable>");
const _: () = assert!(true, "HasOperatorSpaceship<ThreeWayComparable>");
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct ImplicitBool_E1 {
    #[offset(0)]
    pub x: i32,
}
impl CppMove for ImplicitBool_E1 {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct ImplicitBool_E2 {
    #[offset(0)]
    pub y: i32,
}
impl CppMove for ImplicitBool_E2 {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_into(bool, to_bool_1)]
#[byte_size(1)]
pub struct ImplicitBool {
    #[offset(0)]
    pub val: bool,
}
impl CppMove for ImplicitBool {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn operator_eq_31(mut lhs: ImplicitBool_E1, mut rhs: ImplicitBool_E2) -> ImplicitBool {
    return ImplicitBool {
        val: (lhs.x == rhs.y),
    };
}
pub fn test_ordered_map_container_spaceship_with_type_32(comp: ()) {
    let comp: Value<()> = Rc::new(RefCell::new(comp));
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            ({
                testOrder_13(
                    l1.as_pointer(),
                    l2.as_pointer(),
                    ({ (*libcc2rs::strong_ordering_equivalent().upgrade().deref()) }),
                )
            })
        );
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            ({
                testOrder_13(
                    l1.as_pointer(),
                    l2.as_pointer(),
                    ({ (*libcc2rs::strong_ordering_equivalent().upgrade().deref()) }),
                )
            })
        );
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            ({
                testOrder_13(
                    l1.as_pointer(),
                    l2.as_pointer(),
                    ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
                )
            })
        );
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            ({
                testOrder_13(
                    l1.as_pointer(),
                    l2.as_pointer(),
                    ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
                )
            })
        );
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            ({
                testOrder_13(
                    l1.as_pointer(),
                    l2.as_pointer(),
                    ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
                )
            })
        );
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            ({
                testOrder_13(
                    l1.as_pointer(),
                    l2.as_pointer(),
                    ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
                )
            })
        );
    }
    {}
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            ({
                testOrder_13(
                    l1.as_pointer(),
                    l2.as_pointer(),
                    ({ (*libcc2rs::strong_ordering_equivalent().upgrade().deref()) }),
                )
            })
        );
        let l3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l4: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            ({
                testOrder_13(
                    l3.as_pointer(),
                    l4.as_pointer(),
                    ({ (*libcc2rs::strong_ordering_equivalent().upgrade().deref()) }),
                )
            })
        );
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            ({
                testOrder_13(
                    l1.as_pointer(),
                    l2.as_pointer(),
                    ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
                )
            })
        );
        let l3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l4: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            ({
                testOrder_13(
                    l3.as_pointer(),
                    l4.as_pointer(),
                    ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
                )
            })
        );
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            ({
                testOrder_13(
                    l1.as_pointer(),
                    l2.as_pointer(),
                    ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
                )
            })
        );
        let l3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l4: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            ({
                testOrder_13(
                    l3.as_pointer(),
                    l4.as_pointer(),
                    ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
                )
            })
        );
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            ({
                testOrder_13(
                    l1.as_pointer(),
                    l2.as_pointer(),
                    ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
                )
            })
        );
        let l3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l4: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            ({
                testOrder_13(
                    l3.as_pointer(),
                    l4.as_pointer(),
                    ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
                )
            })
        );
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            ({
                testOrder_13(
                    l1.as_pointer(),
                    l2.as_pointer(),
                    ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
                )
            })
        );
        let l3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l4: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            ({
                testOrder_13(
                    l3.as_pointer(),
                    l4.as_pointer(),
                    ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
                )
            })
        );
    }
    {}
}
pub fn test_ordered_map_container_spaceship_with_type_33(comp: ()) {
    let comp: Value<()> = Rc::new(RefCell::new(comp));
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            ({
                testOrder_14(
                    l1.as_pointer(),
                    l2.as_pointer(),
                    ({ (*libcc2rs::strong_ordering_equivalent().upgrade().deref()) }),
                )
            })
        );
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            ({
                testOrder_14(
                    l1.as_pointer(),
                    l2.as_pointer(),
                    ({ (*libcc2rs::strong_ordering_equivalent().upgrade().deref()) }),
                )
            })
        );
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            ({
                testOrder_14(
                    l1.as_pointer(),
                    l2.as_pointer(),
                    ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
                )
            })
        );
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            ({
                testOrder_14(
                    l1.as_pointer(),
                    l2.as_pointer(),
                    ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
                )
            })
        );
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            ({
                testOrder_14(
                    l1.as_pointer(),
                    l2.as_pointer(),
                    ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
                )
            })
        );
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            ({
                testOrder_14(
                    l1.as_pointer(),
                    l2.as_pointer(),
                    ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
                )
            })
        );
    }
    {}
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            ({
                testOrder_14(
                    l1.as_pointer(),
                    l2.as_pointer(),
                    ({ (*libcc2rs::strong_ordering_equivalent().upgrade().deref()) }),
                )
            })
        );
        let l3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l4: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            ({
                testOrder_14(
                    l3.as_pointer(),
                    l4.as_pointer(),
                    ({ (*libcc2rs::strong_ordering_equivalent().upgrade().deref()) }),
                )
            })
        );
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            ({
                testOrder_14(
                    l1.as_pointer(),
                    l2.as_pointer(),
                    ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
                )
            })
        );
        let l3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l4: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            ({
                testOrder_14(
                    l3.as_pointer(),
                    l4.as_pointer(),
                    ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
                )
            })
        );
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            ({
                testOrder_14(
                    l1.as_pointer(),
                    l2.as_pointer(),
                    ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
                )
            })
        );
        let l3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l4: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            ({
                testOrder_14(
                    l3.as_pointer(),
                    l4.as_pointer(),
                    ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
                )
            })
        );
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            ({
                testOrder_14(
                    l1.as_pointer(),
                    l2.as_pointer(),
                    ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
                )
            })
        );
        let l3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l4: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            ({
                testOrder_14(
                    l3.as_pointer(),
                    l4.as_pointer(),
                    ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
                )
            })
        );
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            ({
                testOrder_14(
                    l1.as_pointer(),
                    l2.as_pointer(),
                    ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
                )
            })
        );
        let l3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l4: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            ({
                testOrder_14(
                    l3.as_pointer(),
                    l4.as_pointer(),
                    ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
                )
            })
        );
    }
    {}
}
pub fn test_ordered_map_container_spaceship_with_type_34(comp: ()) {
    let comp: Value<()> = Rc::new(RefCell::new(comp));
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            ({
                testOrder_15(
                    l1.as_pointer(),
                    l2.as_pointer(),
                    ({ (*libcc2rs::strong_ordering_equivalent().upgrade().deref()) }),
                )
            })
        );
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            ({
                testOrder_15(
                    l1.as_pointer(),
                    l2.as_pointer(),
                    ({ (*libcc2rs::strong_ordering_equivalent().upgrade().deref()) }),
                )
            })
        );
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            ({
                testOrder_15(
                    l1.as_pointer(),
                    l2.as_pointer(),
                    ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
                )
            })
        );
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            ({
                testOrder_15(
                    l1.as_pointer(),
                    l2.as_pointer(),
                    ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
                )
            })
        );
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            ({
                testOrder_15(
                    l1.as_pointer(),
                    l2.as_pointer(),
                    ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
                )
            })
        );
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            ({
                testOrder_15(
                    l1.as_pointer(),
                    l2.as_pointer(),
                    ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
                )
            })
        );
    }
    {}
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            ({
                testOrder_15(
                    l1.as_pointer(),
                    l2.as_pointer(),
                    ({ (*libcc2rs::strong_ordering_equivalent().upgrade().deref()) }),
                )
            })
        );
        let l3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l4: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            ({
                testOrder_15(
                    l3.as_pointer(),
                    l4.as_pointer(),
                    ({ (*libcc2rs::strong_ordering_equivalent().upgrade().deref()) }),
                )
            })
        );
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            ({
                testOrder_15(
                    l1.as_pointer(),
                    l2.as_pointer(),
                    ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
                )
            })
        );
        let l3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l4: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            ({
                testOrder_15(
                    l3.as_pointer(),
                    l4.as_pointer(),
                    ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
                )
            })
        );
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            ({
                testOrder_15(
                    l1.as_pointer(),
                    l2.as_pointer(),
                    ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
                )
            })
        );
        let l3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l4: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            ({
                testOrder_15(
                    l3.as_pointer(),
                    l4.as_pointer(),
                    ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
                )
            })
        );
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            ({
                testOrder_15(
                    l1.as_pointer(),
                    l2.as_pointer(),
                    ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
                )
            })
        );
        let l3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l4: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            ({
                testOrder_15(
                    l3.as_pointer(),
                    l4.as_pointer(),
                    ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
                )
            })
        );
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            ({
                testOrder_15(
                    l1.as_pointer(),
                    l2.as_pointer(),
                    ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
                )
            })
        );
        let l3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l4: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            ({
                testOrder_15(
                    l3.as_pointer(),
                    l4.as_pointer(),
                    ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
                )
            })
        );
    }
    {}
}
pub fn test_ordered_map_container_spaceship_with_type_35(comp: ()) {
    let comp: Value<()> = Rc::new(RefCell::new(comp));
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            ({
                testOrder_16(
                    l1.as_pointer(),
                    l2.as_pointer(),
                    ({ (*libcc2rs::strong_ordering_equivalent().upgrade().deref()) }),
                )
            })
        );
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            ({
                testOrder_16(
                    l1.as_pointer(),
                    l2.as_pointer(),
                    ({ (*libcc2rs::strong_ordering_equivalent().upgrade().deref()) }),
                )
            })
        );
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            ({
                testOrder_16(
                    l1.as_pointer(),
                    l2.as_pointer(),
                    ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
                )
            })
        );
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            ({
                testOrder_16(
                    l1.as_pointer(),
                    l2.as_pointer(),
                    ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
                )
            })
        );
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            ({
                testOrder_16(
                    l1.as_pointer(),
                    l2.as_pointer(),
                    ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
                )
            })
        );
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            ({
                testOrder_16(
                    l1.as_pointer(),
                    l2.as_pointer(),
                    ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
                )
            })
        );
    }
    {}
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            ({
                testOrder_16(
                    l1.as_pointer(),
                    l2.as_pointer(),
                    ({ (*libcc2rs::strong_ordering_equivalent().upgrade().deref()) }),
                )
            })
        );
        let l3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l4: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            ({
                testOrder_16(
                    l3.as_pointer(),
                    l4.as_pointer(),
                    ({ (*libcc2rs::strong_ordering_equivalent().upgrade().deref()) }),
                )
            })
        );
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            ({
                testOrder_16(
                    l1.as_pointer(),
                    l2.as_pointer(),
                    ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
                )
            })
        );
        let l3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l4: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            ({
                testOrder_16(
                    l3.as_pointer(),
                    l4.as_pointer(),
                    ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
                )
            })
        );
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            ({
                testOrder_16(
                    l1.as_pointer(),
                    l2.as_pointer(),
                    ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
                )
            })
        );
        let l3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l4: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            ({
                testOrder_16(
                    l3.as_pointer(),
                    l4.as_pointer(),
                    ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
                )
            })
        );
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            ({
                testOrder_16(
                    l1.as_pointer(),
                    l2.as_pointer(),
                    ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
                )
            })
        );
        let l3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l4: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            ({
                testOrder_16(
                    l3.as_pointer(),
                    l4.as_pointer(),
                    ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) }),
                )
            })
        );
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            ({
                testOrder_16(
                    l1.as_pointer(),
                    l2.as_pointer(),
                    ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
                )
            })
        );
        let l3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l4: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            ({
                testOrder_16(
                    l3.as_pointer(),
                    l4.as_pointer(),
                    ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) }),
                )
            })
        );
    }
    {}
}
pub fn test_ordered_map_container_spaceship_with_type_36(comp: ()) {
    let comp: Value<()> = Rc::new(RefCell::new(comp));
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(({ testOrder_17(l1.as_pointer(), l2.as_pointer(), unimplemented!(),) }));
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(({ testOrder_17(l1.as_pointer(), l2.as_pointer(), unimplemented!(),) }));
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(({ testOrder_17(l1.as_pointer(), l2.as_pointer(), unimplemented!(),) }));
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(({ testOrder_17(l1.as_pointer(), l2.as_pointer(), unimplemented!(),) }));
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(({ testOrder_17(l1.as_pointer(), l2.as_pointer(), unimplemented!(),) }));
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(({ testOrder_17(l1.as_pointer(), l2.as_pointer(), unimplemented!(),) }));
    }
    {}
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(({ testOrder_17(l1.as_pointer(), l2.as_pointer(), unimplemented!(),) }));
        let l3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l4: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(({ testOrder_17(l3.as_pointer(), l4.as_pointer(), unimplemented!(),) }));
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(({ testOrder_17(l1.as_pointer(), l2.as_pointer(), unimplemented!(),) }));
        let l3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l4: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(({ testOrder_17(l3.as_pointer(), l4.as_pointer(), unimplemented!(),) }));
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(({ testOrder_17(l1.as_pointer(), l2.as_pointer(), unimplemented!(),) }));
        let l3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l4: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(({ testOrder_17(l3.as_pointer(), l4.as_pointer(), unimplemented!(),) }));
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(({ testOrder_17(l1.as_pointer(), l2.as_pointer(), unimplemented!(),) }));
        let l3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l4: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(({ testOrder_17(l3.as_pointer(), l4.as_pointer(), unimplemented!(),) }));
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(({ testOrder_17(l1.as_pointer(), l2.as_pointer(), unimplemented!(),) }));
        let l3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l4: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(({ testOrder_17(l3.as_pointer(), l4.as_pointer(), unimplemented!(),) }));
    }
    {}
}
pub fn test_ordered_map_container_spaceship_with_type_37(comp: ()) {
    let comp: Value<()> = Rc::new(RefCell::new(comp));
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(({ testOrder_18(l1.as_pointer(), l2.as_pointer(), unimplemented!(),) }));
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(({ testOrder_18(l1.as_pointer(), l2.as_pointer(), unimplemented!(),) }));
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(({ testOrder_18(l1.as_pointer(), l2.as_pointer(), unimplemented!(),) }));
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(({ testOrder_18(l1.as_pointer(), l2.as_pointer(), unimplemented!(),) }));
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(({ testOrder_18(l1.as_pointer(), l2.as_pointer(), unimplemented!(),) }));
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(({ testOrder_18(l1.as_pointer(), l2.as_pointer(), unimplemented!(),) }));
    }
    {}
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(({ testOrder_18(l1.as_pointer(), l2.as_pointer(), unimplemented!(),) }));
        let l3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l4: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(({ testOrder_18(l3.as_pointer(), l4.as_pointer(), unimplemented!(),) }));
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(({ testOrder_18(l1.as_pointer(), l2.as_pointer(), unimplemented!(),) }));
        let l3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l4: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(({ testOrder_18(l3.as_pointer(), l4.as_pointer(), unimplemented!(),) }));
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(({ testOrder_18(l1.as_pointer(), l2.as_pointer(), unimplemented!(),) }));
        let l3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l4: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(({ testOrder_18(l3.as_pointer(), l4.as_pointer(), unimplemented!(),) }));
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(({ testOrder_18(l1.as_pointer(), l2.as_pointer(), unimplemented!(),) }));
        let l3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l4: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(({ testOrder_18(l3.as_pointer(), l4.as_pointer(), unimplemented!(),) }));
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(({ testOrder_18(l1.as_pointer(), l2.as_pointer(), unimplemented!(),) }));
        let l3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l4: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(({ testOrder_18(l3.as_pointer(), l4.as_pointer(), unimplemented!(),) }));
    }
    {}
}
pub fn test_ordered_map_container_spaceship_with_type_38(comp: ()) {
    let comp: Value<()> = Rc::new(RefCell::new(comp));
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(({ testOrder_19(l1.as_pointer(), l2.as_pointer(), unimplemented!(),) }));
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(({ testOrder_19(l1.as_pointer(), l2.as_pointer(), unimplemented!(),) }));
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(({ testOrder_19(l1.as_pointer(), l2.as_pointer(), unimplemented!(),) }));
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(({ testOrder_19(l1.as_pointer(), l2.as_pointer(), unimplemented!(),) }));
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(({ testOrder_19(l1.as_pointer(), l2.as_pointer(), unimplemented!(),) }));
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(({ testOrder_19(l1.as_pointer(), l2.as_pointer(), unimplemented!(),) }));
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(({ testOrder_19(l1.as_pointer(), l2.as_pointer(), unimplemented!(),) }));
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(({ testOrder_19(l1.as_pointer(), l2.as_pointer(), unimplemented!(),) }));
        let l3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l4: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(({ testOrder_19(l3.as_pointer(), l4.as_pointer(), unimplemented!(),) }));
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(({ testOrder_19(l1.as_pointer(), l2.as_pointer(), unimplemented!(),) }));
        let l3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l4: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(({ testOrder_19(l3.as_pointer(), l4.as_pointer(), unimplemented!(),) }));
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(({ testOrder_19(l1.as_pointer(), l2.as_pointer(), unimplemented!(),) }));
        let l3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l4: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(({ testOrder_19(l3.as_pointer(), l4.as_pointer(), unimplemented!(),) }));
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(({ testOrder_19(l1.as_pointer(), l2.as_pointer(), unimplemented!(),) }));
        let l3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l4: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(({ testOrder_19(l3.as_pointer(), l4.as_pointer(), unimplemented!(),) }));
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(({ testOrder_19(l1.as_pointer(), l2.as_pointer(), unimplemented!(),) }));
        let l3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l4: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(({ testOrder_19(l3.as_pointer(), l4.as_pointer(), unimplemented!(),) }));
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(({ testOrder_19(l1.as_pointer(), l2.as_pointer(), unimplemented!(),) }));
        let l3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l4: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(({ testOrder_19(l3.as_pointer(), l4.as_pointer(), unimplemented!(),) }));
    }
}
pub fn test_ordered_map_container_spaceship_with_type_39(comp: ()) {
    let comp: Value<()> = Rc::new(RefCell::new(comp));
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(({ testOrder_20(l1.as_pointer(), l2.as_pointer(), unimplemented!(),) }));
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(({ testOrder_20(l1.as_pointer(), l2.as_pointer(), unimplemented!(),) }));
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(({ testOrder_20(l1.as_pointer(), l2.as_pointer(), unimplemented!(),) }));
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(({ testOrder_20(l1.as_pointer(), l2.as_pointer(), unimplemented!(),) }));
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(({ testOrder_20(l1.as_pointer(), l2.as_pointer(), unimplemented!(),) }));
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(({ testOrder_20(l1.as_pointer(), l2.as_pointer(), unimplemented!(),) }));
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(({ testOrder_20(l1.as_pointer(), l2.as_pointer(), unimplemented!(),) }));
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(({ testOrder_20(l1.as_pointer(), l2.as_pointer(), unimplemented!(),) }));
        let l3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l4: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(({ testOrder_20(l3.as_pointer(), l4.as_pointer(), unimplemented!(),) }));
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(({ testOrder_20(l1.as_pointer(), l2.as_pointer(), unimplemented!(),) }));
        let l3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l4: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(({ testOrder_20(l3.as_pointer(), l4.as_pointer(), unimplemented!(),) }));
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(({ testOrder_20(l1.as_pointer(), l2.as_pointer(), unimplemented!(),) }));
        let l3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l4: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(({ testOrder_20(l3.as_pointer(), l4.as_pointer(), unimplemented!(),) }));
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(({ testOrder_20(l1.as_pointer(), l2.as_pointer(), unimplemented!(),) }));
        let l3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l4: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(({ testOrder_20(l3.as_pointer(), l4.as_pointer(), unimplemented!(),) }));
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(({ testOrder_20(l1.as_pointer(), l2.as_pointer(), unimplemented!(),) }));
        let l3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l4: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(({ testOrder_20(l3.as_pointer(), l4.as_pointer(), unimplemented!(),) }));
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(({ testOrder_20(l1.as_pointer(), l2.as_pointer(), unimplemented!(),) }));
        let l3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l4: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(({ testOrder_20(l3.as_pointer(), l4.as_pointer(), unimplemented!(),) }));
    }
}
pub fn test_ordered_map_container_spaceship_with_type_40(comp: ()) {
    let comp: Value<()> = Rc::new(RefCell::new(comp));
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(({ testOrder_21(l1.as_pointer(), l2.as_pointer(), unimplemented!(),) }));
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(({ testOrder_21(l1.as_pointer(), l2.as_pointer(), unimplemented!(),) }));
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(({ testOrder_21(l1.as_pointer(), l2.as_pointer(), unimplemented!(),) }));
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(({ testOrder_21(l1.as_pointer(), l2.as_pointer(), unimplemented!(),) }));
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(({ testOrder_21(l1.as_pointer(), l2.as_pointer(), unimplemented!(),) }));
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(({ testOrder_21(l1.as_pointer(), l2.as_pointer(), unimplemented!(),) }));
    }
    {}
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(({ testOrder_21(l1.as_pointer(), l2.as_pointer(), unimplemented!(),) }));
        let l3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l4: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(({ testOrder_21(l3.as_pointer(), l4.as_pointer(), unimplemented!(),) }));
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(({ testOrder_21(l1.as_pointer(), l2.as_pointer(), unimplemented!(),) }));
        let l3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l4: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(({ testOrder_21(l3.as_pointer(), l4.as_pointer(), unimplemented!(),) }));
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(({ testOrder_21(l1.as_pointer(), l2.as_pointer(), unimplemented!(),) }));
        let l3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l4: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(({ testOrder_21(l3.as_pointer(), l4.as_pointer(), unimplemented!(),) }));
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(({ testOrder_21(l1.as_pointer(), l2.as_pointer(), unimplemented!(),) }));
        let l3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l4: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(({ testOrder_21(l3.as_pointer(), l4.as_pointer(), unimplemented!(),) }));
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(({ testOrder_21(l1.as_pointer(), l2.as_pointer(), unimplemented!(),) }));
        let l3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l4: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(({ testOrder_21(l3.as_pointer(), l4.as_pointer(), unimplemented!(),) }));
    }
    {}
}
pub fn test_ordered_map_container_spaceship_41() -> bool {
    const _: () = assert!(
        !(false),
        "!std::three_way_comparable<Container<int, NonComparable>>"
    );;
    const _: () = assert!(true, "std::three_way_comparable<Container<int, int>>");;
    ({ test_ordered_map_container_spaceship_with_type_32(unimplemented!()) });
    ({ test_ordered_map_container_spaceship_with_type_33(unimplemented!()) });
    ({ test_ordered_map_container_spaceship_with_type_34(unimplemented!()) });
    ({ test_ordered_map_container_spaceship_with_type_35(unimplemented!()) });
    ({ test_ordered_map_container_spaceship_with_type_36(unimplemented!()) });
    ({ test_ordered_map_container_spaceship_with_type_37(unimplemented!()) });
    ({ test_ordered_map_container_spaceship_with_type_38(unimplemented!()) });
    ({ test_ordered_map_container_spaceship_with_type_39(unimplemented!()) });
    ({ test_ordered_map_container_spaceship_with_type_40(unimplemented!()) });
    return true;
}
pub fn test_42() -> bool {
    assert!(({ test_ordered_map_container_spaceship_41() }));
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
    ({ test_42() });
    return 0;
}
pub trait ImplicitBoolImpl {
    fn to_bool_1(&self) -> bool;
}
impl ImplicitBoolImpl for Ptr<ImplicitBool> {
    fn to_bool_1(&self) -> bool {
        return (*self).with(|__s| __s.val);
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
pub trait test_allocator_std_pair_const_int__PartialOrder__Impl {
    fn destructor(&self);
    fn address_11(&self, x: ()) -> Ptr<(Value<i32>, Value<PartialOrder>)> {
        unimplemented!()
    }
    fn address_12(&self, x: ()) -> Ptr<(Value<i32>, Value<PartialOrder>)> {
        unimplemented!()
    }
    fn allocate(&self, mut n: u32, mut _a1: AnyPtr) -> Ptr<(Value<i32>, Value<PartialOrder>)> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<(Value<i32>, Value<PartialOrder>)>, mut s: u32) {
        unimplemented!()
    }
    fn max_size(&self) -> u32 {
        unimplemented!()
    }
    fn destroy(&self, mut p: Ptr<(Value<i32>, Value<PartialOrder>)>) {
        unimplemented!()
    }
    fn get_data(&self) -> i32 {
        unimplemented!()
    }
    fn get_id(&self) -> i32 {
        unimplemented!()
    }
}
impl test_allocator_std_pair_const_int__PartialOrder__Impl
    for Ptr<test_allocator_std_pair_const_int__PartialOrder__>
{
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
pub trait test_allocator_std_pair_const_int__StrongOrder__Impl {
    fn destructor(&self);
    fn address_11(&self, x: ()) -> Ptr<(Value<i32>, Value<StrongOrder>)> {
        unimplemented!()
    }
    fn address_12(&self, x: ()) -> Ptr<(Value<i32>, Value<StrongOrder>)> {
        unimplemented!()
    }
    fn allocate(&self, mut n: u32, mut _a1: AnyPtr) -> Ptr<(Value<i32>, Value<StrongOrder>)> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<(Value<i32>, Value<StrongOrder>)>, mut s: u32) {
        unimplemented!()
    }
    fn max_size(&self) -> u32 {
        unimplemented!()
    }
    fn destroy(&self, mut p: Ptr<(Value<i32>, Value<StrongOrder>)>) {
        unimplemented!()
    }
    fn get_data(&self) -> i32 {
        unimplemented!()
    }
    fn get_id(&self) -> i32 {
        unimplemented!()
    }
}
impl test_allocator_std_pair_const_int__StrongOrder__Impl
    for Ptr<test_allocator_std_pair_const_int__StrongOrder__>
{
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
pub trait test_allocator_std_pair_const_int__WeakOrder__Impl {
    fn destructor(&self);
    fn address_11(&self, x: ()) -> Ptr<(Value<i32>, Value<WeakOrder>)> {
        unimplemented!()
    }
    fn address_12(&self, x: ()) -> Ptr<(Value<i32>, Value<WeakOrder>)> {
        unimplemented!()
    }
    fn allocate(&self, mut n: u32, mut _a1: AnyPtr) -> Ptr<(Value<i32>, Value<WeakOrder>)> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<(Value<i32>, Value<WeakOrder>)>, mut s: u32) {
        unimplemented!()
    }
    fn max_size(&self) -> u32 {
        unimplemented!()
    }
    fn destroy(&self, mut p: Ptr<(Value<i32>, Value<WeakOrder>)>) {
        unimplemented!()
    }
    fn get_data(&self) -> i32 {
        unimplemented!()
    }
    fn get_id(&self) -> i32 {
        unimplemented!()
    }
}
impl test_allocator_std_pair_const_int__WeakOrder__Impl
    for Ptr<test_allocator_std_pair_const_int__WeakOrder__>
{
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
pub trait test_allocator_std_pair_const_int__int__Impl {
    fn destructor(&self);
    fn address_11(&self, x: ()) -> Ptr<(Value<i32>, Value<i32>)> {
        unimplemented!()
    }
    fn address_12(&self, x: ()) -> Ptr<(Value<i32>, Value<i32>)> {
        unimplemented!()
    }
    fn allocate(&self, mut n: u32, mut _a1: AnyPtr) -> Ptr<(Value<i32>, Value<i32>)> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<(Value<i32>, Value<i32>)>, mut s: u32) {
        unimplemented!()
    }
    fn max_size(&self) -> u32 {
        unimplemented!()
    }
    fn destroy(&self, mut p: Ptr<(Value<i32>, Value<i32>)>) {
        unimplemented!()
    }
    fn get_data(&self) -> i32 {
        unimplemented!()
    }
    fn get_id(&self) -> i32 {
        unimplemented!()
    }
}
impl test_allocator_std_pair_const_int__int__Impl
    for Ptr<test_allocator_std_pair_const_int__int__>
{
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
}
