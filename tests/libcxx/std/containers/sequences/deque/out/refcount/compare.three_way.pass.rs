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
    pub static destructed_value_0: Value<()> = Rc::new(RefCell::new(-1_()));
);
thread_local!(
    pub static moved_value_1: Value<()> = Rc::new(RefCell::new(2147483647_()));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_alloc_base {}
pub fn operator_eq_2(x: Ptr<test_allocator_void_>, y: Ptr<test_allocator_void_>) -> () {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
pub fn operator_ne_3(x: Ptr<test_allocator_void_>, y: Ptr<test_allocator_void_>) -> () {
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
impl std::cmp::PartialEq for test_allocator_void_ {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_2(
                Rc::new(RefCell::new(test_allocator_void_ {
                    data_: self.data_.clone(),
                    id_: self.id_.clone(),
                    stats_: self.stats_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(test_allocator_void_ {
                    data_: other.data_.clone(),
                    id_: other.id_.clone(),
                    stats_: other.stats_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for test_allocator_void_ {}
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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_allocator_StrongOrder__rebind_StrongOrder_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_allocator_StrongOrder__rebind_StrongOrder_ptr_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_allocator_StrongOrder__rebind_const_StrongOrder_ptr_ {}
#[derive(Record, ByteRepr, FnPtrArg, Destructor)]
#[byte_size(16)]
pub struct test_allocator_StrongOrder_ {
    #[offset(0)]
    data_: (),
    #[offset(4)]
    id_: (),
    #[offset(8)]
    #[byte_size(8)]
    stats_: Ptr<test_allocator_statistics>,
}
impl test_allocator_StrongOrder_ {}
impl Default for test_allocator_StrongOrder_ {
    fn default() -> Self {
        {
            test_allocator_StrongOrder_ {
                data_: 0_(),
                id_: 0_(),
                stats_: Ptr::<test_allocator_statistics>::null(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_allocator_PartialOrder__rebind_PartialOrder_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_allocator_PartialOrder__rebind_PartialOrder_ptr_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_allocator_PartialOrder__rebind_const_PartialOrder_ptr_ {}
#[derive(Record, ByteRepr, FnPtrArg, Destructor)]
#[byte_size(16)]
pub struct test_allocator_PartialOrder_ {
    #[offset(0)]
    data_: (),
    #[offset(4)]
    id_: (),
    #[offset(8)]
    #[byte_size(8)]
    stats_: Ptr<test_allocator_statistics>,
}
impl test_allocator_PartialOrder_ {}
impl Default for test_allocator_PartialOrder_ {
    fn default() -> Self {
        {
            test_allocator_PartialOrder_ {
                data_: 0_(),
                id_: 0_(),
                stats_: Ptr::<test_allocator_statistics>::null(),
            }
        }
    }
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, Destructor)]
#[byte_size(16)]
pub struct test_allocator_StrongOrder_ptr_ {
    #[offset(0)]
    data_: (),
    #[offset(4)]
    id_: (),
    #[offset(8)]
    #[byte_size(8)]
    stats_: Ptr<test_allocator_statistics>,
}
impl test_allocator_StrongOrder_ptr_ {
    pub fn copy_from(a: Ptr<test_allocator_StrongOrder_ptr_>) -> Self {
        let __this: Value<test_allocator_StrongOrder_ptr_> = Rc::new(RefCell::new(Self {
            data_: a.with(|__s| __s.data_),
            id_: a.with(|__s| __s.id_),
            stats_: a.with(|__s| __s.stats_.clone()),
        }));
        let this: Ptr<test_allocator_StrongOrder_ptr_> = __this.as_pointer();
        unimplemented!();
        if !((this.with(|__s| __s.stats_.clone())).is_null()) {
            field!(this.with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_inc());
            field!(this.with(|__s| __s.stats_.clone()), copied).with_mut(|__v| __v.prefix_inc());
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for test_allocator_StrongOrder_ptr_ {
    fn clone(&self) -> Self {
        let __src: Value<test_allocator_StrongOrder_ptr_> =
            Rc::new(RefCell::new(test_allocator_StrongOrder_ptr_ {
                data_: self.data_.clone(),
                id_: self.id_.clone(),
                stats_: self.stats_.clone(),
            }));
        test_allocator_StrongOrder_ptr_::copy_from(__src.as_pointer())
    }
}
impl Default for test_allocator_StrongOrder_ptr_ {
    fn default() -> Self {
        {
            test_allocator_StrongOrder_ptr_ {
                data_: 0_(),
                id_: 0_(),
                stats_: Ptr::<test_allocator_statistics>::null(),
            }
        }
    }
}
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(16)]
pub struct test_allocator_const_StrongOrder_ptr_ {
    #[offset(0)]
    data_: (),
    #[offset(4)]
    id_: (),
    #[offset(8)]
    #[byte_size(8)]
    stats_: Ptr<test_allocator_statistics>,
}
impl test_allocator_const_StrongOrder_ptr_ {}
impl Default for test_allocator_const_StrongOrder_ptr_ {
    fn default() -> Self {
        {
            test_allocator_const_StrongOrder_ptr_ {
                data_: unimplemented!(),
                id_: unimplemented!(),
                stats_: Ptr::<test_allocator_statistics>::null(),
            }
        }
    }
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, Destructor)]
#[byte_size(16)]
pub struct test_allocator_PartialOrder_ptr_ {
    #[offset(0)]
    data_: (),
    #[offset(4)]
    id_: (),
    #[offset(8)]
    #[byte_size(8)]
    stats_: Ptr<test_allocator_statistics>,
}
impl test_allocator_PartialOrder_ptr_ {
    pub fn copy_from(a: Ptr<test_allocator_PartialOrder_ptr_>) -> Self {
        let __this: Value<test_allocator_PartialOrder_ptr_> = Rc::new(RefCell::new(Self {
            data_: a.with(|__s| __s.data_),
            id_: a.with(|__s| __s.id_),
            stats_: a.with(|__s| __s.stats_.clone()),
        }));
        let this: Ptr<test_allocator_PartialOrder_ptr_> = __this.as_pointer();
        unimplemented!();
        if !((this.with(|__s| __s.stats_.clone())).is_null()) {
            field!(this.with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_inc());
            field!(this.with(|__s| __s.stats_.clone()), copied).with_mut(|__v| __v.prefix_inc());
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for test_allocator_PartialOrder_ptr_ {
    fn clone(&self) -> Self {
        let __src: Value<test_allocator_PartialOrder_ptr_> =
            Rc::new(RefCell::new(test_allocator_PartialOrder_ptr_ {
                data_: self.data_.clone(),
                id_: self.id_.clone(),
                stats_: self.stats_.clone(),
            }));
        test_allocator_PartialOrder_ptr_::copy_from(__src.as_pointer())
    }
}
impl Default for test_allocator_PartialOrder_ptr_ {
    fn default() -> Self {
        {
            test_allocator_PartialOrder_ptr_ {
                data_: 0_(),
                id_: 0_(),
                stats_: Ptr::<test_allocator_statistics>::null(),
            }
        }
    }
}
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(16)]
pub struct test_allocator_const_PartialOrder_ptr_ {
    #[offset(0)]
    data_: (),
    #[offset(4)]
    id_: (),
    #[offset(8)]
    #[byte_size(8)]
    stats_: Ptr<test_allocator_statistics>,
}
impl test_allocator_const_PartialOrder_ptr_ {}
impl Default for test_allocator_const_PartialOrder_ptr_ {
    fn default() -> Self {
        {
            test_allocator_const_PartialOrder_ptr_ {
                data_: unimplemented!(),
                id_: unimplemented!(),
                stats_: Ptr::<test_allocator_statistics>::null(),
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
pub fn testComparisonsComplete_4(
    t1: Ptr<()>,
    t2: Ptr<()>,
    mut isEqual: (),
    mut isLess: (),
    mut isGreater: (),
) -> () {
    unimplemented!();
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
pub fn testComparisonsComplete_5(
    t1: Ptr<()>,
    t2: Ptr<()>,
    mut isEqual: (),
    mut isLess: (),
    mut isGreater: (),
) -> () {
    unimplemented!();
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
pub fn testComparisonsComplete_6(
    t1: Ptr<()>,
    t2: Ptr<()>,
    mut isEqual: (),
    mut isLess: (),
    mut isGreater: (),
) -> () {
    unimplemented!();
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
pub fn testComparisonsComplete_7(
    t1: Ptr<()>,
    t2: Ptr<()>,
    mut isEqual: (),
    mut isLess: (),
    mut isGreater: (),
) -> () {
    unimplemented!();
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
pub fn testComparisonsComplete_8(
    t1: Ptr<()>,
    t2: Ptr<()>,
    mut isEqual: (),
    mut isLess: (),
    mut isGreater: (),
) -> () {
    unimplemented!();
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
pub fn testOrder_9(t1: Ptr<()>, t2: Ptr<()>, order: ()) -> () {
    let order: Value<()> = Rc::new(RefCell::new(order));
    let mut equal: () = unimplemented!();
    let mut less: () = unimplemented!();
    let mut greater: () = unimplemented!();
    return (unimplemented!())
        && ({
            let _t1: Ptr<()> = (t1).clone();
            let _t2: Ptr<()> = (t2).clone();
            let _isEqual: () = equal;
            let _isLess: () = less;
            let _isGreater: () = greater;
            testComparisonsComplete_4(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub fn testOrder_10(t1: Ptr<()>, t2: Ptr<()>, order: ()) -> () {
    let order: Value<()> = Rc::new(RefCell::new(order));
    let mut equal: () = unimplemented!();
    let mut less: () = unimplemented!();
    let mut greater: () = unimplemented!();
    return (unimplemented!())
        && ({
            let _t1: Ptr<()> = (t1).clone();
            let _t2: Ptr<()> = (t2).clone();
            let _isEqual: () = equal;
            let _isLess: () = less;
            let _isGreater: () = greater;
            testComparisonsComplete_5(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub fn testOrder_11(t1: Ptr<()>, t2: Ptr<()>, order: ()) -> () {
    let order: Value<()> = Rc::new(RefCell::new(order));
    let mut equal: () = unimplemented!();
    let mut less: () = unimplemented!();
    let mut greater: () = unimplemented!();
    return (unimplemented!())
        && ({
            let _t1: Ptr<()> = (t1).clone();
            let _t2: Ptr<()> = (t2).clone();
            let _isEqual: () = equal;
            let _isLess: () = less;
            let _isGreater: () = greater;
            testComparisonsComplete_6(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub fn testOrder_12(t1: Ptr<()>, t2: Ptr<()>, order: ()) -> () {
    let order: Value<()> = Rc::new(RefCell::new(order));
    let mut equal: () = unimplemented!();
    let mut less: () = unimplemented!();
    let mut greater: () = unimplemented!();
    return (unimplemented!())
        && ({
            let _t1: Ptr<()> = (t1).clone();
            let _t2: Ptr<()> = (t2).clone();
            let _isEqual: () = equal;
            let _isLess: () = less;
            let _isGreater: () = greater;
            testComparisonsComplete_7(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub fn testOrder_13(t1: Ptr<()>, t2: Ptr<()>, order: ()) -> () {
    let order: Value<()> = Rc::new(RefCell::new(order));
    let mut equal: () = unimplemented!();
    let mut less: () = unimplemented!();
    let mut greater: () = unimplemented!();
    return (unimplemented!())
        && ({
            let _t1: Ptr<()> = (t1).clone();
            let _t2: Ptr<()> = (t2).clone();
            let _isEqual: () = equal;
            let _isLess: () = less;
            let _isGreater: () = greater;
            testComparisonsComplete_8(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub fn operator_lt_14(lhs: Ptr<LessAndEqComp>, rhs: Ptr<LessAndEqComp>) -> () {
    return ({ lhs.with(|__s| __s.value) } < { rhs.with(|__s| __s.value) });
}
pub fn operator_eq_15(lhs: Ptr<LessAndEqComp>, rhs: Ptr<LessAndEqComp>) -> () {
    return ({ lhs.with(|__s| __s.value) } == { rhs.with(|__s| __s.value) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct LessAndEqComp {
    #[offset(0)]
    pub value: (),
}
impl LessAndEqComp {
    pub fn new(mut v: ()) -> Self {
        Self { value: v }
    }
}
impl std::cmp::Ord for LessAndEqComp {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            if operator_lt_14(
                Rc::new(RefCell::new(LessAndEqComp {
                    value: self.value.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(LessAndEqComp {
                    value: other.value.clone(),
                }))
                .as_pointer(),
            ) {
                std::cmp::Ordering::Less
            } else if operator_lt_14(
                Rc::new(RefCell::new(LessAndEqComp {
                    value: other.value.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(LessAndEqComp {
                    value: self.value.clone(),
                }))
                .as_pointer(),
            ) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for LessAndEqComp {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for LessAndEqComp {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_15(
                Rc::new(RefCell::new(LessAndEqComp {
                    value: self.value.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(LessAndEqComp {
                    value: other.value.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for LessAndEqComp {}
pub fn operator_cmp_16(mut _a0: StrongOrder, mut _a1: StrongOrder) -> () {
    {
        let cmp: Value<()> = Rc::new(RefCell::new(std::cmp::Ord::cmp(&(_a0.value), &(_a1.value))));
        if !(unimplemented!()) {
            return unimplemented!();
        }
    }
    return unimplemented!();
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct StrongOrder {
    #[offset(0)]
    pub value: (),
}
impl StrongOrder {
    pub fn new(mut v: ()) -> Self {
        Self { value: v }
    }
}
impl std::cmp::Ord for StrongOrder {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        { operator_cmp_16(self.clone(), other.clone()) }
    }
}
impl std::cmp::PartialOrd for StrongOrder {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for StrongOrder {
    fn eq(&self, other: &Self) -> bool {
        { operator_cmp_16(self.clone(), other.clone()) == std::cmp::Ordering::Equal }
    }
}
impl std::cmp::Eq for StrongOrder {}
pub fn operator_cmp_17(mut _a0: WeakOrder, mut _a1: WeakOrder) -> () {
    {
        let cmp: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        if !(unimplemented!()) {
            return unimplemented!();
        }
    }
    return unimplemented!();
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct WeakOrder {
    #[offset(0)]
    pub value: (),
}
impl WeakOrder {
    pub fn new(mut v: ()) -> Self {
        Self { value: v }
    }
}
impl std::cmp::Ord for WeakOrder {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        { operator_cmp_17(self.clone(), other.clone()) }
    }
}
impl std::cmp::PartialOrd for WeakOrder {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for WeakOrder {
    fn eq(&self, other: &Self) -> bool {
        { operator_cmp_17(self.clone(), other.clone()) == std::cmp::Ordering::Equal }
    }
}
impl std::cmp::Eq for WeakOrder {}
pub fn operator_cmp_18(mut lhs: PartialOrder, mut rhs: PartialOrder) -> () {
    if (lhs.value == unimplemented!()) || (rhs.value == unimplemented!()) {
        return unimplemented!();
    }
    if (lhs.value == unimplemented!()) || (rhs.value == unimplemented!()) {
        return unimplemented!();
    }
    return unimplemented!();
}
pub fn operator_eq_19(mut lhs: PartialOrder, mut rhs: PartialOrder) -> () {
    return unimplemented!();
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct PartialOrder {
    #[offset(0)]
    pub value: (),
}
impl PartialOrder {
    pub fn new(mut v: ()) -> Self {
        Self { value: v }
    }
}
impl std::cmp::Ord for PartialOrder {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        { operator_cmp_18(self.clone(), other.clone()) }
    }
}
impl std::cmp::PartialOrd for PartialOrder {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for PartialOrder {
    fn eq(&self, other: &Self) -> bool {
        { operator_eq_19(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for PartialOrder {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct NonComparable {}
const _: () = assert!(!(false), "!std::equality_comparable<NonComparable>");
const _: () = assert!(!(false), "!HasOperatorEqual<NonComparable>");
const _: () = assert!(!(false), "!HasOperatorGreaterThan<NonComparable>");
const _: () = assert!(!(false), "!HasOperatorGreaterThanEqual<NonComparable>");
const _: () = assert!(!(false), "!HasOperatorLessThan<NonComparable>");
const _: () = assert!(!(false), "!HasOperatorLessThanEqual<NonComparable>");
const _: () = assert!(!(false), "!HasOperatorNotEqual<NonComparable>");
const _: () = assert!(!(false), "!HasOperatorSpaceship<NonComparable>");
pub fn operator_eq_20(_a0: Ptr<EqualityComparable>, _a1: Ptr<EqualityComparable>) -> () {
    return ({ _a0.with(|__s| __s.value_) } == { _a1.with(|__s| __s.value_) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct EqualityComparable {
    #[offset(0)]
    value_: (),
}
impl EqualityComparable {
    pub fn new(mut value: ()) -> Self {
        Self { value_: value }
    }
}
impl std::cmp::PartialEq for EqualityComparable {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_20(
                Rc::new(RefCell::new(EqualityComparable {
                    value_: self.value_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(EqualityComparable {
                    value_: other.value_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for EqualityComparable {}
const _: () = assert!(true, "std::equality_comparable<EqualityComparable>");
const _: () = assert!(true, "HasOperatorEqual<EqualityComparable>");
const _: () = assert!(true, "HasOperatorNotEqual<EqualityComparable>");
pub fn operator_eq_21(_a0: Ptr<ThreeWayComparable>, _a1: Ptr<ThreeWayComparable>) -> () {
    return ({ _a0.with(|__s| __s.value_) } == { _a1.with(|__s| __s.value_) });
}
pub fn operator_cmp_22(_a0: Ptr<ThreeWayComparable>, _a1: Ptr<ThreeWayComparable>) -> () {
    {
        let cmp: Value<()> = Rc::new(RefCell::new(std::cmp::Ord::cmp(
            &(_a0.with(|__s| __s.value_)),
            &(_a1.with(|__s| __s.value_)),
        )));
        if !(unimplemented!()) {
            return unimplemented!();
        }
    }
    return unimplemented!();
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct ThreeWayComparable {
    #[offset(0)]
    value_: (),
}
impl ThreeWayComparable {
    pub fn new(mut value: ()) -> Self {
        Self { value_: value }
    }
}
impl std::cmp::Ord for ThreeWayComparable {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            operator_cmp_22(
                Rc::new(RefCell::new(ThreeWayComparable {
                    value_: self.value_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(ThreeWayComparable {
                    value_: other.value_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::PartialOrd for ThreeWayComparable {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for ThreeWayComparable {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_21(
                Rc::new(RefCell::new(ThreeWayComparable {
                    value_: self.value_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(ThreeWayComparable {
                    value_: other.value_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for ThreeWayComparable {}
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
    pub x: (),
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct ImplicitBool_E2 {
    #[offset(0)]
    pub y: (),
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ImplicitBool {
    #[offset(0)]
    pub val: (),
}
pub fn operator_eq_23(mut lhs: ImplicitBool_E1, mut rhs: ImplicitBool_E2) -> ImplicitBool {
    return ImplicitBool {
        val: (lhs.x == rhs.y),
    };
}
pub fn test_sequence_container_spaceship_with_type_24() {
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {}
}
pub fn test_sequence_container_spaceship_with_type_25() {
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {}
}
pub fn test_sequence_container_spaceship_with_type_26() {
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {}
}
pub fn test_sequence_container_spaceship_with_type_27() {
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_sequence_container_spaceship_with_type_28() {
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {}
}
pub fn test_sequence_container_spaceship_29() -> () {
    const _: () = assert!(true, "std::three_way_comparable<Container<int>>");;
    ({ test_sequence_container_spaceship_with_type_24() });
    ({ test_sequence_container_spaceship_with_type_25() });
    ({ test_sequence_container_spaceship_with_type_26() });
    ({ test_sequence_container_spaceship_with_type_27() });
    ({ test_sequence_container_spaceship_with_type_28() });
    const _: () = assert!(
        !(false),
        "!std::three_way_comparable<Container<NonComparable>>"
    );;
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
    unimplemented!();
    return 0_();
}
pub trait ImplicitBoolImpl {
    fn to__1(&self) -> ();
}
impl ImplicitBoolImpl for Ptr<ImplicitBool> {
    fn to__1(&self) -> () {
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
pub trait test_allocator_PartialOrder_Impl {
    fn destructor(&self);
    fn address_11(&self, x: ()) -> Ptr<PartialOrder> {
        unimplemented!()
    }
    fn address_12(&self, x: ()) -> Ptr<PartialOrder> {
        unimplemented!()
    }
    fn allocate(&self, n: (), _a1: Option<()>) -> Ptr<PartialOrder>;
    fn deallocate(&self, p: Ptr<PartialOrder>, s: ());
    fn max_size(&self) -> () {
        unimplemented!()
    }
    fn destroy(&self, p: Ptr<PartialOrder>);
    fn get_data(&self) -> () {
        unimplemented!()
    }
    fn get_id(&self) -> () {
        unimplemented!()
    }
    fn construct_16_const_struct_PartialOrder_ref(
        &self,
        p: Ptr<PartialOrder>,
        val: Ptr<PartialOrder>,
    );
}
impl test_allocator_PartialOrder_Impl for Ptr<test_allocator_PartialOrder_> {
    fn destructor(&self) {
        unimplemented!();
        unimplemented!();
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_dec());
        }
        field!((*self), data_).write(destructed_value_0.with(|rc| *rc.borrow()));
        field!((*self), id_).write(destructed_value_0.with(|rc| *rc.borrow()));
    }
    fn allocate(&self, mut n: (), _a1: Option<()>) -> Ptr<PartialOrder> {
        let mut _a1: () = _a1.unwrap_or_else(|| unimplemented!());
        unimplemented!();
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            if ({
                (*self)
                    .with(|__s| __s.stats_.clone())
                    .with(|__s| __s.time_to_throw)
            } >= {
                (*self)
                    .with(|__s| __s.stats_.clone())
                    .with(|__s| __s.throw_after)
            }) {
                unimplemented!();
            }
            field!((*self).with(|__s| __s.stats_.clone()), time_to_throw)
                .with_mut(|__v| __v.prefix_inc());
            field!((*self).with(|__s| __s.stats_.clone()), alloc_count)
                .with_mut(|__v| __v.prefix_inc());
            field!((*self).with(|__s| __s.stats_.clone()), allocated_size).write({
                ((*self)
                    .with(|__s| __s.stats_.clone())
                    .with(|__s| __s.allocated_size))
                .wrapping_add(n)
            });
        }
        return unimplemented!();
    }
    fn deallocate(&self, mut p: Ptr<PartialOrder>, mut s: ()) {
        unimplemented!();
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), alloc_count)
                .with_mut(|__v| __v.prefix_dec());
            field!((*self).with(|__s| __s.stats_.clone()), allocated_size).write({
                ((*self)
                    .with(|__s| __s.stats_.clone())
                    .with(|__s| __s.allocated_size))
                .wrapping_sub(s)
            });
        }
        unimplemented!();
    }
    fn destroy(&self, mut p: Ptr<PartialOrder>) {
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), destroy_count)
                .with_mut(|__v| __v.prefix_inc());
        };
    }
    fn construct_16_const_struct_PartialOrder_ref(
        &self,
        mut p: Ptr<PartialOrder>,
        val: Ptr<PartialOrder>,
    ) {
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), construct_count)
                .with_mut(|__v| __v.prefix_inc());
        }
        unimplemented!();
    }
}
pub trait test_allocator_PartialOrder_ptr_Impl {
    fn destructor(&self);
    fn address_11(&self, x: ()) -> Ptr<Ptr<PartialOrder>> {
        unimplemented!()
    }
    fn address_12(&self, x: ()) -> Ptr<Ptr<PartialOrder>> {
        unimplemented!()
    }
    fn allocate(&self, n: (), _a1: Option<()>) -> Ptr<Ptr<PartialOrder>>;
    fn deallocate(&self, p: Ptr<Ptr<PartialOrder>>, s: ());
    fn max_size(&self) -> () {
        unimplemented!()
    }
    fn destroy(&self, p: Ptr<Ptr<PartialOrder>>);
    fn get_data(&self) -> () {
        unimplemented!()
    }
    fn get_id(&self) -> () {
        unimplemented!()
    }
    fn construct_16_struct_PartialOrder_ptr(
        &self,
        p: Ptr<Ptr<PartialOrder>>,
        val: Ptr<Ptr<PartialOrder>>,
    );
    fn construct_16_struct_PartialOrder_ptrref(
        &self,
        p: Ptr<Ptr<PartialOrder>>,
        val: Ptr<Ptr<PartialOrder>>,
    );
}
impl test_allocator_PartialOrder_ptr_Impl for Ptr<test_allocator_PartialOrder_ptr_> {
    fn destructor(&self) {
        unimplemented!();
        unimplemented!();
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_dec());
        }
        field!((*self), data_).write(destructed_value_0.with(|rc| *rc.borrow()));
        field!((*self), id_).write(destructed_value_0.with(|rc| *rc.borrow()));
    }
    fn allocate(&self, mut n: (), _a1: Option<()>) -> Ptr<Ptr<PartialOrder>> {
        let mut _a1: () = _a1.unwrap_or_else(|| unimplemented!());
        unimplemented!();
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            if ({
                (*self)
                    .with(|__s| __s.stats_.clone())
                    .with(|__s| __s.time_to_throw)
            } >= {
                (*self)
                    .with(|__s| __s.stats_.clone())
                    .with(|__s| __s.throw_after)
            }) {
                unimplemented!();
            }
            field!((*self).with(|__s| __s.stats_.clone()), time_to_throw)
                .with_mut(|__v| __v.prefix_inc());
            field!((*self).with(|__s| __s.stats_.clone()), alloc_count)
                .with_mut(|__v| __v.prefix_inc());
            field!((*self).with(|__s| __s.stats_.clone()), allocated_size).write({
                ((*self)
                    .with(|__s| __s.stats_.clone())
                    .with(|__s| __s.allocated_size))
                .wrapping_add(n)
            });
        }
        return unimplemented!();
    }
    fn deallocate(&self, mut p: Ptr<Ptr<PartialOrder>>, mut s: ()) {
        unimplemented!();
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), alloc_count)
                .with_mut(|__v| __v.prefix_dec());
            field!((*self).with(|__s| __s.stats_.clone()), allocated_size).write({
                ((*self)
                    .with(|__s| __s.stats_.clone())
                    .with(|__s| __s.allocated_size))
                .wrapping_sub(s)
            });
        }
        unimplemented!();
    }
    fn destroy(&self, mut p: Ptr<Ptr<PartialOrder>>) {
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), destroy_count)
                .with_mut(|__v| __v.prefix_inc());
        };
    }
    fn construct_16_struct_PartialOrder_ptr(
        &self,
        mut p: Ptr<Ptr<PartialOrder>>,
        val: Ptr<Ptr<PartialOrder>>,
    ) {
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), construct_count)
                .with_mut(|__v| __v.prefix_inc());
        }
        unimplemented!();
    }
    fn construct_16_struct_PartialOrder_ptrref(
        &self,
        mut p: Ptr<Ptr<PartialOrder>>,
        val: Ptr<Ptr<PartialOrder>>,
    ) {
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), construct_count)
                .with_mut(|__v| __v.prefix_inc());
        }
        unimplemented!();
    }
}
pub trait test_allocator_StrongOrder_Impl {
    fn destructor(&self);
    fn address_11(&self, x: ()) -> Ptr<StrongOrder> {
        unimplemented!()
    }
    fn address_12(&self, x: ()) -> Ptr<StrongOrder> {
        unimplemented!()
    }
    fn allocate(&self, n: (), _a1: Option<()>) -> Ptr<StrongOrder>;
    fn deallocate(&self, p: Ptr<StrongOrder>, s: ());
    fn max_size(&self) -> () {
        unimplemented!()
    }
    fn destroy(&self, p: Ptr<StrongOrder>);
    fn get_data(&self) -> () {
        unimplemented!()
    }
    fn get_id(&self) -> () {
        unimplemented!()
    }
    fn construct_16_const_struct_StrongOrder_ref(&self, p: Ptr<StrongOrder>, val: Ptr<StrongOrder>);
}
impl test_allocator_StrongOrder_Impl for Ptr<test_allocator_StrongOrder_> {
    fn destructor(&self) {
        unimplemented!();
        unimplemented!();
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_dec());
        }
        field!((*self), data_).write(destructed_value_0.with(|rc| *rc.borrow()));
        field!((*self), id_).write(destructed_value_0.with(|rc| *rc.borrow()));
    }
    fn allocate(&self, mut n: (), _a1: Option<()>) -> Ptr<StrongOrder> {
        let mut _a1: () = _a1.unwrap_or_else(|| unimplemented!());
        unimplemented!();
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            if ({
                (*self)
                    .with(|__s| __s.stats_.clone())
                    .with(|__s| __s.time_to_throw)
            } >= {
                (*self)
                    .with(|__s| __s.stats_.clone())
                    .with(|__s| __s.throw_after)
            }) {
                unimplemented!();
            }
            field!((*self).with(|__s| __s.stats_.clone()), time_to_throw)
                .with_mut(|__v| __v.prefix_inc());
            field!((*self).with(|__s| __s.stats_.clone()), alloc_count)
                .with_mut(|__v| __v.prefix_inc());
            field!((*self).with(|__s| __s.stats_.clone()), allocated_size).write({
                ((*self)
                    .with(|__s| __s.stats_.clone())
                    .with(|__s| __s.allocated_size))
                .wrapping_add(n)
            });
        }
        return unimplemented!();
    }
    fn deallocate(&self, mut p: Ptr<StrongOrder>, mut s: ()) {
        unimplemented!();
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), alloc_count)
                .with_mut(|__v| __v.prefix_dec());
            field!((*self).with(|__s| __s.stats_.clone()), allocated_size).write({
                ((*self)
                    .with(|__s| __s.stats_.clone())
                    .with(|__s| __s.allocated_size))
                .wrapping_sub(s)
            });
        }
        unimplemented!();
    }
    fn destroy(&self, mut p: Ptr<StrongOrder>) {
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), destroy_count)
                .with_mut(|__v| __v.prefix_inc());
        };
    }
    fn construct_16_const_struct_StrongOrder_ref(
        &self,
        mut p: Ptr<StrongOrder>,
        val: Ptr<StrongOrder>,
    ) {
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), construct_count)
                .with_mut(|__v| __v.prefix_inc());
        }
        unimplemented!();
    }
}
pub trait test_allocator_StrongOrder_ptr_Impl {
    fn destructor(&self);
    fn address_11(&self, x: ()) -> Ptr<Ptr<StrongOrder>> {
        unimplemented!()
    }
    fn address_12(&self, x: ()) -> Ptr<Ptr<StrongOrder>> {
        unimplemented!()
    }
    fn allocate(&self, n: (), _a1: Option<()>) -> Ptr<Ptr<StrongOrder>>;
    fn deallocate(&self, p: Ptr<Ptr<StrongOrder>>, s: ());
    fn max_size(&self) -> () {
        unimplemented!()
    }
    fn destroy(&self, p: Ptr<Ptr<StrongOrder>>);
    fn get_data(&self) -> () {
        unimplemented!()
    }
    fn get_id(&self) -> () {
        unimplemented!()
    }
    fn construct_16_struct_StrongOrder_ptr(
        &self,
        p: Ptr<Ptr<StrongOrder>>,
        val: Ptr<Ptr<StrongOrder>>,
    );
    fn construct_16_struct_StrongOrder_ptrref(
        &self,
        p: Ptr<Ptr<StrongOrder>>,
        val: Ptr<Ptr<StrongOrder>>,
    );
}
impl test_allocator_StrongOrder_ptr_Impl for Ptr<test_allocator_StrongOrder_ptr_> {
    fn destructor(&self) {
        unimplemented!();
        unimplemented!();
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_dec());
        }
        field!((*self), data_).write(destructed_value_0.with(|rc| *rc.borrow()));
        field!((*self), id_).write(destructed_value_0.with(|rc| *rc.borrow()));
    }
    fn allocate(&self, mut n: (), _a1: Option<()>) -> Ptr<Ptr<StrongOrder>> {
        let mut _a1: () = _a1.unwrap_or_else(|| unimplemented!());
        unimplemented!();
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            if ({
                (*self)
                    .with(|__s| __s.stats_.clone())
                    .with(|__s| __s.time_to_throw)
            } >= {
                (*self)
                    .with(|__s| __s.stats_.clone())
                    .with(|__s| __s.throw_after)
            }) {
                unimplemented!();
            }
            field!((*self).with(|__s| __s.stats_.clone()), time_to_throw)
                .with_mut(|__v| __v.prefix_inc());
            field!((*self).with(|__s| __s.stats_.clone()), alloc_count)
                .with_mut(|__v| __v.prefix_inc());
            field!((*self).with(|__s| __s.stats_.clone()), allocated_size).write({
                ((*self)
                    .with(|__s| __s.stats_.clone())
                    .with(|__s| __s.allocated_size))
                .wrapping_add(n)
            });
        }
        return unimplemented!();
    }
    fn deallocate(&self, mut p: Ptr<Ptr<StrongOrder>>, mut s: ()) {
        unimplemented!();
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), alloc_count)
                .with_mut(|__v| __v.prefix_dec());
            field!((*self).with(|__s| __s.stats_.clone()), allocated_size).write({
                ((*self)
                    .with(|__s| __s.stats_.clone())
                    .with(|__s| __s.allocated_size))
                .wrapping_sub(s)
            });
        }
        unimplemented!();
    }
    fn destroy(&self, mut p: Ptr<Ptr<StrongOrder>>) {
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), destroy_count)
                .with_mut(|__v| __v.prefix_inc());
        };
    }
    fn construct_16_struct_StrongOrder_ptr(
        &self,
        mut p: Ptr<Ptr<StrongOrder>>,
        val: Ptr<Ptr<StrongOrder>>,
    ) {
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), construct_count)
                .with_mut(|__v| __v.prefix_inc());
        }
        unimplemented!();
    }
    fn construct_16_struct_StrongOrder_ptrref(
        &self,
        mut p: Ptr<Ptr<StrongOrder>>,
        val: Ptr<Ptr<StrongOrder>>,
    ) {
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), construct_count)
                .with_mut(|__v| __v.prefix_inc());
        }
        unimplemented!();
    }
}
pub trait test_allocator_const_PartialOrder_ptr_Impl {
    fn destructor(&self) {
        unimplemented!()
    }
    fn address_11(&self, x: ()) -> Ptr<Ptr<PartialOrder>> {
        unimplemented!()
    }
    fn address_12(&self, x: ()) -> Ptr<Ptr<PartialOrder>> {
        unimplemented!()
    }
    fn allocate(&self, mut n: (), mut _a1: ()) -> Ptr<Ptr<PartialOrder>> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<Ptr<PartialOrder>>, mut s: ()) {
        unimplemented!()
    }
    fn max_size(&self) -> () {
        unimplemented!()
    }
    fn destroy(&self, mut p: Ptr<Ptr<PartialOrder>>) {
        unimplemented!()
    }
    fn get_data(&self) -> () {
        unimplemented!()
    }
    fn get_id(&self) -> () {
        unimplemented!()
    }
}
impl test_allocator_const_PartialOrder_ptr_Impl for Ptr<test_allocator_const_PartialOrder_ptr_> {}
pub trait test_allocator_const_StrongOrder_ptr_Impl {
    fn destructor(&self) {
        unimplemented!()
    }
    fn address_11(&self, x: ()) -> Ptr<Ptr<StrongOrder>> {
        unimplemented!()
    }
    fn address_12(&self, x: ()) -> Ptr<Ptr<StrongOrder>> {
        unimplemented!()
    }
    fn allocate(&self, mut n: (), mut _a1: ()) -> Ptr<Ptr<StrongOrder>> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<Ptr<StrongOrder>>, mut s: ()) {
        unimplemented!()
    }
    fn max_size(&self) -> () {
        unimplemented!()
    }
    fn destroy(&self, mut p: Ptr<Ptr<StrongOrder>>) {
        unimplemented!()
    }
    fn get_data(&self) -> () {
        unimplemented!()
    }
    fn get_id(&self) -> () {
        unimplemented!()
    }
}
impl test_allocator_const_StrongOrder_ptr_Impl for Ptr<test_allocator_const_StrongOrder_ptr_> {}
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
        field!((*self), data_).write(destructed_value_0.with(|rc| *rc.borrow()));
        field!((*self), id_).write(destructed_value_0.with(|rc| *rc.borrow()));
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
pub fn __cpp2rust_init_globals() {
    let _ = destructed_value_0.with(|_| ());
    let _ = moved_value_1.with(|_| ());
}
