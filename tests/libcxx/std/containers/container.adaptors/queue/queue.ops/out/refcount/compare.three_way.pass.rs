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
pub struct nasty_list_int_ {
    #[offset(0)]
    #[byte_size(24)]
    pub l_: (),
}
impl nasty_list_int_ {
    pub fn new_1() -> Self {
        Self {
            l_: unimplemented!(),
        }
    }
    pub fn new_2(il: ()) -> Self {
        let il: Value<()> = Rc::new(RefCell::new(il));
        Self {
            l_: unimplemented!(),
        }
    }
}
impl std::cmp::Ord for nasty_list_int_ {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            operator_cmp_0(
                Rc::new(RefCell::new(nasty_list_int_ {
                    l_: self.l_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(nasty_list_int_ {
                    l_: other.l_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::PartialOrd for nasty_list_int_ {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for nasty_list_int_ {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_1(
                Rc::new(RefCell::new(nasty_list_int_ {
                    l_: self.l_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(nasty_list_int_ {
                    l_: other.l_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for nasty_list_int_ {}
impl Default for nasty_list_int_ {
    fn default() -> Self {
        { nasty_list_int_::new_1() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor)]
#[byte_size(24)]
pub struct nasty_list_StrongOrder_ {
    #[offset(0)]
    #[byte_size(24)]
    pub l_: (),
}
impl nasty_list_StrongOrder_ {
    pub fn new_1() -> Self {
        Self {
            l_: unimplemented!(),
        }
    }
    pub fn new_2(il: ()) -> Self {
        let il: Value<()> = Rc::new(RefCell::new(il));
        Self {
            l_: unimplemented!(),
        }
    }
}
impl std::cmp::Ord for nasty_list_StrongOrder_ {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            operator_cmp_2(
                Rc::new(RefCell::new(nasty_list_StrongOrder_ {
                    l_: self.l_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(nasty_list_StrongOrder_ {
                    l_: other.l_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::PartialOrd for nasty_list_StrongOrder_ {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for nasty_list_StrongOrder_ {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_3(
                Rc::new(RefCell::new(nasty_list_StrongOrder_ {
                    l_: self.l_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(nasty_list_StrongOrder_ {
                    l_: other.l_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for nasty_list_StrongOrder_ {}
impl Default for nasty_list_StrongOrder_ {
    fn default() -> Self {
        { nasty_list_StrongOrder_::new_1() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor)]
#[byte_size(24)]
pub struct nasty_list_WeakOrder_ {
    #[offset(0)]
    #[byte_size(24)]
    pub l_: (),
}
impl nasty_list_WeakOrder_ {
    pub fn new_1() -> Self {
        Self {
            l_: unimplemented!(),
        }
    }
    pub fn new_2(il: ()) -> Self {
        let il: Value<()> = Rc::new(RefCell::new(il));
        Self {
            l_: unimplemented!(),
        }
    }
}
impl std::cmp::Ord for nasty_list_WeakOrder_ {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            operator_cmp_4(
                Rc::new(RefCell::new(nasty_list_WeakOrder_ {
                    l_: self.l_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(nasty_list_WeakOrder_ {
                    l_: other.l_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::PartialOrd for nasty_list_WeakOrder_ {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for nasty_list_WeakOrder_ {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_5(
                Rc::new(RefCell::new(nasty_list_WeakOrder_ {
                    l_: self.l_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(nasty_list_WeakOrder_ {
                    l_: other.l_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for nasty_list_WeakOrder_ {}
impl Default for nasty_list_WeakOrder_ {
    fn default() -> Self {
        { nasty_list_WeakOrder_::new_1() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor)]
#[byte_size(24)]
pub struct nasty_list_PartialOrder_ {
    #[offset(0)]
    #[byte_size(24)]
    pub l_: (),
}
impl nasty_list_PartialOrder_ {
    pub fn new_1() -> Self {
        Self {
            l_: unimplemented!(),
        }
    }
    pub fn new_2(il: ()) -> Self {
        let il: Value<()> = Rc::new(RefCell::new(il));
        Self {
            l_: unimplemented!(),
        }
    }
}
impl std::cmp::Ord for nasty_list_PartialOrder_ {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            operator_cmp_6(
                Rc::new(RefCell::new(nasty_list_PartialOrder_ {
                    l_: self.l_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(nasty_list_PartialOrder_ {
                    l_: other.l_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::PartialOrd for nasty_list_PartialOrder_ {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for nasty_list_PartialOrder_ {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_7(
                Rc::new(RefCell::new(nasty_list_PartialOrder_ {
                    l_: self.l_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(nasty_list_PartialOrder_ {
                    l_: other.l_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for nasty_list_PartialOrder_ {}
impl Default for nasty_list_PartialOrder_ {
    fn default() -> Self {
        { nasty_list_PartialOrder_::new_1() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor)]
#[byte_size(24)]
pub struct nasty_list_LessAndEqComp_ {
    #[offset(0)]
    #[byte_size(24)]
    pub l_: (),
}
impl nasty_list_LessAndEqComp_ {
    pub fn new_1() -> Self {
        Self {
            l_: unimplemented!(),
        }
    }
    pub fn new_2(il: ()) -> Self {
        let il: Value<()> = Rc::new(RefCell::new(il));
        Self {
            l_: unimplemented!(),
        }
    }
}
impl std::cmp::Ord for nasty_list_LessAndEqComp_ {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            operator_cmp_8(
                Rc::new(RefCell::new(nasty_list_LessAndEqComp_ {
                    l_: self.l_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(nasty_list_LessAndEqComp_ {
                    l_: other.l_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::PartialOrd for nasty_list_LessAndEqComp_ {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for nasty_list_LessAndEqComp_ {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_9(
                Rc::new(RefCell::new(nasty_list_LessAndEqComp_ {
                    l_: self.l_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(nasty_list_LessAndEqComp_ {
                    l_: other.l_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for nasty_list_LessAndEqComp_ {}
impl Default for nasty_list_LessAndEqComp_ {
    fn default() -> Self {
        { nasty_list_LessAndEqComp_::new_1() }
    }
}
pub fn operator_eq_1(x: Ptr<nasty_list_int_>, y: Ptr<nasty_list_int_>) -> () {
    return unimplemented!();
}
pub fn operator_eq_3(x: Ptr<nasty_list_StrongOrder_>, y: Ptr<nasty_list_StrongOrder_>) -> () {
    return unimplemented!();
}
pub fn operator_eq_5(x: Ptr<nasty_list_WeakOrder_>, y: Ptr<nasty_list_WeakOrder_>) -> () {
    return unimplemented!();
}
pub fn operator_eq_7(x: Ptr<nasty_list_PartialOrder_>, y: Ptr<nasty_list_PartialOrder_>) -> () {
    return unimplemented!();
}
pub fn operator_eq_9(x: Ptr<nasty_list_LessAndEqComp_>, y: Ptr<nasty_list_LessAndEqComp_>) -> () {
    return unimplemented!();
}
pub fn operator_cmp_0(x: Ptr<nasty_list_int_>, y: Ptr<nasty_list_int_>) -> () {
    return unimplemented!();
}
pub fn operator_cmp_2(x: Ptr<nasty_list_StrongOrder_>, y: Ptr<nasty_list_StrongOrder_>) -> () {
    return unimplemented!();
}
pub fn operator_cmp_4(x: Ptr<nasty_list_WeakOrder_>, y: Ptr<nasty_list_WeakOrder_>) -> () {
    return unimplemented!();
}
pub fn operator_cmp_6(x: Ptr<nasty_list_PartialOrder_>, y: Ptr<nasty_list_PartialOrder_>) -> () {
    return unimplemented!();
}
pub fn operator_cmp_8(x: Ptr<nasty_list_LessAndEqComp_>, y: Ptr<nasty_list_LessAndEqComp_>) -> () {
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
        unimplemented!();
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
    pub static destructed_value_10: Value<()> = Rc::new(RefCell::new(-1_()));
);
thread_local!(
    pub static moved_value_11: Value<()> = Rc::new(RefCell::new(2147483647_()));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_alloc_base {}
pub fn operator_eq_12(x: Ptr<test_allocator_void_>, y: Ptr<test_allocator_void_>) -> () {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
pub fn operator_ne_13(x: Ptr<test_allocator_void_>, y: Ptr<test_allocator_void_>) -> () {
    return !({
        let _x: Ptr<test_allocator_void_> = (x).clone();
        let _y: Ptr<test_allocator_void_> = (y).clone();
        operator_eq_12(_x, _y)
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
            operator_eq_12(
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
pub fn testComparisonsComplete_14(
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
        if !(unimplemented!()) {
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
pub fn testComparisonsComplete_15(
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
        if !(unimplemented!()) {
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
pub fn testComparisonsComplete_16(
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
        if !(unimplemented!()) {
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
pub fn testComparisonsComplete_17(
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
        if !(unimplemented!()) {
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
pub fn testComparisonsComplete_18(
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
        if !(unimplemented!()) {
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
pub fn testComparisonsComplete_19(
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
        if !(unimplemented!()) {
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
pub fn testComparisonsComplete_20(
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
        if !(unimplemented!()) {
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
pub fn testComparisonsComplete_21(
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
        if !(unimplemented!()) {
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
pub fn testComparisonsComplete_22(
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
        if !(unimplemented!()) {
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
pub fn testComparisonsComplete_23(
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
        if !(unimplemented!()) {
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
pub fn testComparisonsComplete_24(
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
        if !(unimplemented!()) {
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
pub fn testComparisonsComplete_25(
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
        if !(unimplemented!()) {
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
pub fn testComparisonsComplete_26(
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
        if !(unimplemented!()) {
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
pub fn testComparisonsComplete_27(
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
        if !(unimplemented!()) {
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
pub fn testComparisonsComplete_28(
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
        if !(unimplemented!()) {
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
pub fn testOrder_29(t1: Ptr<()>, t2: Ptr<()>, order: ()) -> () {
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
            testComparisonsComplete_14(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub fn testOrder_30(t1: Ptr<()>, t2: Ptr<()>, order: ()) -> () {
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
            testComparisonsComplete_15(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub fn testOrder_31(t1: Ptr<()>, t2: Ptr<()>, order: ()) -> () {
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
            testComparisonsComplete_16(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub fn testOrder_32(t1: Ptr<()>, t2: Ptr<()>, order: ()) -> () {
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
            testComparisonsComplete_17(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub fn testOrder_33(t1: Ptr<()>, t2: Ptr<()>, order: ()) -> () {
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
            testComparisonsComplete_18(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub fn testOrder_34(t1: Ptr<()>, t2: Ptr<()>, order: ()) -> () {
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
            testComparisonsComplete_19(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub fn testOrder_35(t1: Ptr<()>, t2: Ptr<()>, order: ()) -> () {
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
            testComparisonsComplete_20(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub fn testOrder_36(t1: Ptr<()>, t2: Ptr<()>, order: ()) -> () {
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
            testComparisonsComplete_21(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub fn testOrder_37(t1: Ptr<()>, t2: Ptr<()>, order: ()) -> () {
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
            testComparisonsComplete_22(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub fn testOrder_38(t1: Ptr<()>, t2: Ptr<()>, order: ()) -> () {
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
            testComparisonsComplete_23(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub fn testOrder_39(t1: Ptr<()>, t2: Ptr<()>, order: ()) -> () {
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
            testComparisonsComplete_24(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub fn testOrder_40(t1: Ptr<()>, t2: Ptr<()>, order: ()) -> () {
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
            testComparisonsComplete_25(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub fn testOrder_41(t1: Ptr<()>, t2: Ptr<()>, order: ()) -> () {
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
            testComparisonsComplete_26(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub fn testOrder_42(t1: Ptr<()>, t2: Ptr<()>, order: ()) -> () {
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
            testComparisonsComplete_27(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub fn testOrder_43(t1: Ptr<()>, t2: Ptr<()>, order: ()) -> () {
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
            testComparisonsComplete_28(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub fn operator_lt_44(lhs: Ptr<LessAndEqComp>, rhs: Ptr<LessAndEqComp>) -> () {
    return ({ lhs.with(|__s| __s.value) } < { rhs.with(|__s| __s.value) });
}
pub fn operator_eq_45(lhs: Ptr<LessAndEqComp>, rhs: Ptr<LessAndEqComp>) -> () {
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
            if operator_lt_44(
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
            } else if operator_lt_44(
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
            operator_eq_45(
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
pub fn operator_cmp_46(mut _a0: StrongOrder, mut _a1: StrongOrder) -> () {
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
        { operator_cmp_46(self.clone(), other.clone()) }
    }
}
impl std::cmp::PartialOrd for StrongOrder {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for StrongOrder {
    fn eq(&self, other: &Self) -> bool {
        { operator_cmp_46(self.clone(), other.clone()) == std::cmp::Ordering::Equal }
    }
}
impl std::cmp::Eq for StrongOrder {}
pub fn operator_cmp_47(mut _a0: WeakOrder, mut _a1: WeakOrder) -> () {
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
        { operator_cmp_47(self.clone(), other.clone()) }
    }
}
impl std::cmp::PartialOrd for WeakOrder {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for WeakOrder {
    fn eq(&self, other: &Self) -> bool {
        { operator_cmp_47(self.clone(), other.clone()) == std::cmp::Ordering::Equal }
    }
}
impl std::cmp::Eq for WeakOrder {}
pub fn operator_cmp_48(mut lhs: PartialOrder, mut rhs: PartialOrder) -> () {
    if (lhs.value == ({ std_numeric_limits_int_::min() }))
        || (rhs.value == ({ std_numeric_limits_int_::min() }))
    {
        return unimplemented!();
    }
    if (lhs.value == ({ std_numeric_limits_int_::max() }))
        || (rhs.value == ({ std_numeric_limits_int_::max() }))
    {
        return unimplemented!();
    }
    return unimplemented!();
}
pub fn operator_eq_49(mut lhs: PartialOrder, mut rhs: PartialOrder) -> () {
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
        { operator_cmp_48(self.clone(), other.clone()) }
    }
}
impl std::cmp::PartialOrd for PartialOrder {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for PartialOrder {
    fn eq(&self, other: &Self) -> bool {
        { operator_eq_49(self.clone(), other.clone()) }
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
pub fn operator_eq_50(_a0: Ptr<EqualityComparable>, _a1: Ptr<EqualityComparable>) -> () {
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
            operator_eq_50(
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
pub fn operator_eq_51(_a0: Ptr<ThreeWayComparable>, _a1: Ptr<ThreeWayComparable>) -> () {
    return ({ _a0.with(|__s| __s.value_) } == { _a1.with(|__s| __s.value_) });
}
pub fn operator_cmp_52(_a0: Ptr<ThreeWayComparable>, _a1: Ptr<ThreeWayComparable>) -> () {
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
            operator_cmp_52(
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
            operator_eq_51(
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
pub fn operator_eq_53(mut lhs: ImplicitBool_E1, mut rhs: ImplicitBool_E2) -> ImplicitBool {
    return ImplicitBool {
        val: (lhs.x == rhs.y),
    };
}
pub fn test_sequence_container_adaptor_spaceship_with_type_54() {
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {}
}
pub fn test_sequence_container_adaptor_spaceship_with_type_55() {
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {}
}
pub fn test_sequence_container_adaptor_spaceship_with_type_56() {
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {}
}
pub fn test_sequence_container_adaptor_spaceship_with_type_57() {
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_sequence_container_adaptor_spaceship_with_type_58() {
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {}
}
pub fn test_sequence_container_adaptor_spaceship_with_type_59() {
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {}
}
pub fn test_sequence_container_adaptor_spaceship_with_type_60() {
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {}
}
pub fn test_sequence_container_adaptor_spaceship_with_type_61() {
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {}
}
pub fn test_sequence_container_adaptor_spaceship_with_type_62() {
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_sequence_container_adaptor_spaceship_with_type_63() {
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let l1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ca2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {}
}
pub fn test_sequence_container_adaptor_spaceship_with_type_64() {
    {
        let l1: Value<nasty_list_int_> = Rc::new(RefCell::new(nasty_list_int_::new_1()));
        let _dtor_l1 = ScopedDestructor::new(&l1, |__p| __p.destructor());
        let ca1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<nasty_list_int_> = Rc::new(RefCell::new(nasty_list_int_::new_1()));
        let _dtor_l2 = ScopedDestructor::new(&l2, |__p| __p.destructor());
        let ca2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let l1: Value<nasty_list_int_> =
            Rc::new(RefCell::new(nasty_list_int_::new_2({ unimplemented!() })));
        let _dtor_l1 = ScopedDestructor::new(&l1, |__p| __p.destructor());
        let ca1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<nasty_list_int_> =
            Rc::new(RefCell::new(nasty_list_int_::new_2({ unimplemented!() })));
        let _dtor_l2 = ScopedDestructor::new(&l2, |__p| __p.destructor());
        let ca2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let l1: Value<nasty_list_int_> =
            Rc::new(RefCell::new(nasty_list_int_::new_2({ unimplemented!() })));
        let _dtor_l1 = ScopedDestructor::new(&l1, |__p| __p.destructor());
        let ca1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<nasty_list_int_> =
            Rc::new(RefCell::new(nasty_list_int_::new_2({ unimplemented!() })));
        let _dtor_l2 = ScopedDestructor::new(&l2, |__p| __p.destructor());
        let ca2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let l1: Value<nasty_list_int_> =
            Rc::new(RefCell::new(nasty_list_int_::new_2({ unimplemented!() })));
        let _dtor_l1 = ScopedDestructor::new(&l1, |__p| __p.destructor());
        let ca1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<nasty_list_int_> =
            Rc::new(RefCell::new(nasty_list_int_::new_2({ unimplemented!() })));
        let _dtor_l2 = ScopedDestructor::new(&l2, |__p| __p.destructor());
        let ca2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let l1: Value<nasty_list_int_> =
            Rc::new(RefCell::new(nasty_list_int_::new_2({ unimplemented!() })));
        let _dtor_l1 = ScopedDestructor::new(&l1, |__p| __p.destructor());
        let ca1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<nasty_list_int_> =
            Rc::new(RefCell::new(nasty_list_int_::new_2({ unimplemented!() })));
        let _dtor_l2 = ScopedDestructor::new(&l2, |__p| __p.destructor());
        let ca2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let l1: Value<nasty_list_int_> =
            Rc::new(RefCell::new(nasty_list_int_::new_2({ unimplemented!() })));
        let _dtor_l1 = ScopedDestructor::new(&l1, |__p| __p.destructor());
        let ca1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<nasty_list_int_> =
            Rc::new(RefCell::new(nasty_list_int_::new_2({ unimplemented!() })));
        let _dtor_l2 = ScopedDestructor::new(&l2, |__p| __p.destructor());
        let ca2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {}
}
pub fn test_sequence_container_adaptor_spaceship_with_type_65() {
    {
        let l1: Value<nasty_list_StrongOrder_> =
            Rc::new(RefCell::new(nasty_list_StrongOrder_::new_1()));
        let _dtor_l1 = ScopedDestructor::new(&l1, |__p| __p.destructor());
        let ca1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<nasty_list_StrongOrder_> =
            Rc::new(RefCell::new(nasty_list_StrongOrder_::new_1()));
        let _dtor_l2 = ScopedDestructor::new(&l2, |__p| __p.destructor());
        let ca2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let l1: Value<nasty_list_StrongOrder_> =
            Rc::new(RefCell::new(nasty_list_StrongOrder_::new_2({
                unimplemented!()
            })));
        let _dtor_l1 = ScopedDestructor::new(&l1, |__p| __p.destructor());
        let ca1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<nasty_list_StrongOrder_> =
            Rc::new(RefCell::new(nasty_list_StrongOrder_::new_2({
                unimplemented!()
            })));
        let _dtor_l2 = ScopedDestructor::new(&l2, |__p| __p.destructor());
        let ca2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let l1: Value<nasty_list_StrongOrder_> =
            Rc::new(RefCell::new(nasty_list_StrongOrder_::new_2({
                unimplemented!()
            })));
        let _dtor_l1 = ScopedDestructor::new(&l1, |__p| __p.destructor());
        let ca1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<nasty_list_StrongOrder_> =
            Rc::new(RefCell::new(nasty_list_StrongOrder_::new_2({
                unimplemented!()
            })));
        let _dtor_l2 = ScopedDestructor::new(&l2, |__p| __p.destructor());
        let ca2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let l1: Value<nasty_list_StrongOrder_> =
            Rc::new(RefCell::new(nasty_list_StrongOrder_::new_2({
                unimplemented!()
            })));
        let _dtor_l1 = ScopedDestructor::new(&l1, |__p| __p.destructor());
        let ca1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<nasty_list_StrongOrder_> =
            Rc::new(RefCell::new(nasty_list_StrongOrder_::new_2({
                unimplemented!()
            })));
        let _dtor_l2 = ScopedDestructor::new(&l2, |__p| __p.destructor());
        let ca2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let l1: Value<nasty_list_StrongOrder_> =
            Rc::new(RefCell::new(nasty_list_StrongOrder_::new_2({
                unimplemented!()
            })));
        let _dtor_l1 = ScopedDestructor::new(&l1, |__p| __p.destructor());
        let ca1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<nasty_list_StrongOrder_> =
            Rc::new(RefCell::new(nasty_list_StrongOrder_::new_2({
                unimplemented!()
            })));
        let _dtor_l2 = ScopedDestructor::new(&l2, |__p| __p.destructor());
        let ca2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let l1: Value<nasty_list_StrongOrder_> =
            Rc::new(RefCell::new(nasty_list_StrongOrder_::new_2({
                unimplemented!()
            })));
        let _dtor_l1 = ScopedDestructor::new(&l1, |__p| __p.destructor());
        let ca1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<nasty_list_StrongOrder_> =
            Rc::new(RefCell::new(nasty_list_StrongOrder_::new_2({
                unimplemented!()
            })));
        let _dtor_l2 = ScopedDestructor::new(&l2, |__p| __p.destructor());
        let ca2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {}
}
pub fn test_sequence_container_adaptor_spaceship_with_type_66() {
    {
        let l1: Value<nasty_list_WeakOrder_> =
            Rc::new(RefCell::new(nasty_list_WeakOrder_::new_1()));
        let _dtor_l1 = ScopedDestructor::new(&l1, |__p| __p.destructor());
        let ca1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<nasty_list_WeakOrder_> =
            Rc::new(RefCell::new(nasty_list_WeakOrder_::new_1()));
        let _dtor_l2 = ScopedDestructor::new(&l2, |__p| __p.destructor());
        let ca2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let l1: Value<nasty_list_WeakOrder_> =
            Rc::new(RefCell::new(nasty_list_WeakOrder_::new_2({
                unimplemented!()
            })));
        let _dtor_l1 = ScopedDestructor::new(&l1, |__p| __p.destructor());
        let ca1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<nasty_list_WeakOrder_> =
            Rc::new(RefCell::new(nasty_list_WeakOrder_::new_2({
                unimplemented!()
            })));
        let _dtor_l2 = ScopedDestructor::new(&l2, |__p| __p.destructor());
        let ca2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let l1: Value<nasty_list_WeakOrder_> =
            Rc::new(RefCell::new(nasty_list_WeakOrder_::new_2({
                unimplemented!()
            })));
        let _dtor_l1 = ScopedDestructor::new(&l1, |__p| __p.destructor());
        let ca1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<nasty_list_WeakOrder_> =
            Rc::new(RefCell::new(nasty_list_WeakOrder_::new_2({
                unimplemented!()
            })));
        let _dtor_l2 = ScopedDestructor::new(&l2, |__p| __p.destructor());
        let ca2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let l1: Value<nasty_list_WeakOrder_> =
            Rc::new(RefCell::new(nasty_list_WeakOrder_::new_2({
                unimplemented!()
            })));
        let _dtor_l1 = ScopedDestructor::new(&l1, |__p| __p.destructor());
        let ca1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<nasty_list_WeakOrder_> =
            Rc::new(RefCell::new(nasty_list_WeakOrder_::new_2({
                unimplemented!()
            })));
        let _dtor_l2 = ScopedDestructor::new(&l2, |__p| __p.destructor());
        let ca2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let l1: Value<nasty_list_WeakOrder_> =
            Rc::new(RefCell::new(nasty_list_WeakOrder_::new_2({
                unimplemented!()
            })));
        let _dtor_l1 = ScopedDestructor::new(&l1, |__p| __p.destructor());
        let ca1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<nasty_list_WeakOrder_> =
            Rc::new(RefCell::new(nasty_list_WeakOrder_::new_2({
                unimplemented!()
            })));
        let _dtor_l2 = ScopedDestructor::new(&l2, |__p| __p.destructor());
        let ca2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let l1: Value<nasty_list_WeakOrder_> =
            Rc::new(RefCell::new(nasty_list_WeakOrder_::new_2({
                unimplemented!()
            })));
        let _dtor_l1 = ScopedDestructor::new(&l1, |__p| __p.destructor());
        let ca1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<nasty_list_WeakOrder_> =
            Rc::new(RefCell::new(nasty_list_WeakOrder_::new_2({
                unimplemented!()
            })));
        let _dtor_l2 = ScopedDestructor::new(&l2, |__p| __p.destructor());
        let ca2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {}
}
pub fn test_sequence_container_adaptor_spaceship_with_type_67() {
    {
        let l1: Value<nasty_list_PartialOrder_> =
            Rc::new(RefCell::new(nasty_list_PartialOrder_::new_1()));
        let _dtor_l1 = ScopedDestructor::new(&l1, |__p| __p.destructor());
        let ca1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<nasty_list_PartialOrder_> =
            Rc::new(RefCell::new(nasty_list_PartialOrder_::new_1()));
        let _dtor_l2 = ScopedDestructor::new(&l2, |__p| __p.destructor());
        let ca2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let l1: Value<nasty_list_PartialOrder_> =
            Rc::new(RefCell::new(nasty_list_PartialOrder_::new_2({
                unimplemented!()
            })));
        let _dtor_l1 = ScopedDestructor::new(&l1, |__p| __p.destructor());
        let ca1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<nasty_list_PartialOrder_> =
            Rc::new(RefCell::new(nasty_list_PartialOrder_::new_2({
                unimplemented!()
            })));
        let _dtor_l2 = ScopedDestructor::new(&l2, |__p| __p.destructor());
        let ca2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let l1: Value<nasty_list_PartialOrder_> =
            Rc::new(RefCell::new(nasty_list_PartialOrder_::new_2({
                unimplemented!()
            })));
        let _dtor_l1 = ScopedDestructor::new(&l1, |__p| __p.destructor());
        let ca1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<nasty_list_PartialOrder_> =
            Rc::new(RefCell::new(nasty_list_PartialOrder_::new_2({
                unimplemented!()
            })));
        let _dtor_l2 = ScopedDestructor::new(&l2, |__p| __p.destructor());
        let ca2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let l1: Value<nasty_list_PartialOrder_> =
            Rc::new(RefCell::new(nasty_list_PartialOrder_::new_2({
                unimplemented!()
            })));
        let _dtor_l1 = ScopedDestructor::new(&l1, |__p| __p.destructor());
        let ca1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<nasty_list_PartialOrder_> =
            Rc::new(RefCell::new(nasty_list_PartialOrder_::new_2({
                unimplemented!()
            })));
        let _dtor_l2 = ScopedDestructor::new(&l2, |__p| __p.destructor());
        let ca2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let l1: Value<nasty_list_PartialOrder_> =
            Rc::new(RefCell::new(nasty_list_PartialOrder_::new_2({
                unimplemented!()
            })));
        let _dtor_l1 = ScopedDestructor::new(&l1, |__p| __p.destructor());
        let ca1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<nasty_list_PartialOrder_> =
            Rc::new(RefCell::new(nasty_list_PartialOrder_::new_2({
                unimplemented!()
            })));
        let _dtor_l2 = ScopedDestructor::new(&l2, |__p| __p.destructor());
        let ca2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let l1: Value<nasty_list_PartialOrder_> =
            Rc::new(RefCell::new(nasty_list_PartialOrder_::new_2({
                unimplemented!()
            })));
        let _dtor_l1 = ScopedDestructor::new(&l1, |__p| __p.destructor());
        let ca1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<nasty_list_PartialOrder_> =
            Rc::new(RefCell::new(nasty_list_PartialOrder_::new_2({
                unimplemented!()
            })));
        let _dtor_l2 = ScopedDestructor::new(&l2, |__p| __p.destructor());
        let ca2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let l1: Value<nasty_list_PartialOrder_> =
            Rc::new(RefCell::new(nasty_list_PartialOrder_::new_2({
                unimplemented!()
            })));
        let _dtor_l1 = ScopedDestructor::new(&l1, |__p| __p.destructor());
        let ca1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<nasty_list_PartialOrder_> =
            Rc::new(RefCell::new(nasty_list_PartialOrder_::new_2({
                unimplemented!()
            })));
        let _dtor_l2 = ScopedDestructor::new(&l2, |__p| __p.destructor());
        let ca2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
}
pub fn test_sequence_container_adaptor_spaceship_with_type_68() {
    {
        let l1: Value<nasty_list_LessAndEqComp_> =
            Rc::new(RefCell::new(nasty_list_LessAndEqComp_::new_1()));
        let _dtor_l1 = ScopedDestructor::new(&l1, |__p| __p.destructor());
        let ca1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<nasty_list_LessAndEqComp_> =
            Rc::new(RefCell::new(nasty_list_LessAndEqComp_::new_1()));
        let _dtor_l2 = ScopedDestructor::new(&l2, |__p| __p.destructor());
        let ca2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let l1: Value<nasty_list_LessAndEqComp_> =
            Rc::new(RefCell::new(nasty_list_LessAndEqComp_::new_2({
                unimplemented!()
            })));
        let _dtor_l1 = ScopedDestructor::new(&l1, |__p| __p.destructor());
        let ca1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<nasty_list_LessAndEqComp_> =
            Rc::new(RefCell::new(nasty_list_LessAndEqComp_::new_2({
                unimplemented!()
            })));
        let _dtor_l2 = ScopedDestructor::new(&l2, |__p| __p.destructor());
        let ca2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let l1: Value<nasty_list_LessAndEqComp_> =
            Rc::new(RefCell::new(nasty_list_LessAndEqComp_::new_2({
                unimplemented!()
            })));
        let _dtor_l1 = ScopedDestructor::new(&l1, |__p| __p.destructor());
        let ca1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<nasty_list_LessAndEqComp_> =
            Rc::new(RefCell::new(nasty_list_LessAndEqComp_::new_2({
                unimplemented!()
            })));
        let _dtor_l2 = ScopedDestructor::new(&l2, |__p| __p.destructor());
        let ca2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let l1: Value<nasty_list_LessAndEqComp_> =
            Rc::new(RefCell::new(nasty_list_LessAndEqComp_::new_2({
                unimplemented!()
            })));
        let _dtor_l1 = ScopedDestructor::new(&l1, |__p| __p.destructor());
        let ca1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<nasty_list_LessAndEqComp_> =
            Rc::new(RefCell::new(nasty_list_LessAndEqComp_::new_2({
                unimplemented!()
            })));
        let _dtor_l2 = ScopedDestructor::new(&l2, |__p| __p.destructor());
        let ca2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let l1: Value<nasty_list_LessAndEqComp_> =
            Rc::new(RefCell::new(nasty_list_LessAndEqComp_::new_2({
                unimplemented!()
            })));
        let _dtor_l1 = ScopedDestructor::new(&l1, |__p| __p.destructor());
        let ca1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<nasty_list_LessAndEqComp_> =
            Rc::new(RefCell::new(nasty_list_LessAndEqComp_::new_2({
                unimplemented!()
            })));
        let _dtor_l2 = ScopedDestructor::new(&l2, |__p| __p.destructor());
        let ca2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let l1: Value<nasty_list_LessAndEqComp_> =
            Rc::new(RefCell::new(nasty_list_LessAndEqComp_::new_2({
                unimplemented!()
            })));
        let _dtor_l1 = ScopedDestructor::new(&l1, |__p| __p.destructor());
        let ca1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let l2: Value<nasty_list_LessAndEqComp_> =
            Rc::new(RefCell::new(nasty_list_LessAndEqComp_::new_2({
                unimplemented!()
            })));
        let _dtor_l2 = ScopedDestructor::new(&l2, |__p| __p.destructor());
        let ca2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {}
}
pub fn test_sequence_container_adaptor_spaceship_69() -> () {
    const _: () = assert!(
        !(false),
        "!std::three_way_comparable<ContainerAdaptor<NonComparable>>"
    );;
    const _: () = assert!(
        true,
        "std::three_way_comparable<ContainerAdaptor<int, Container<int>>>"
    );;
    ({ test_sequence_container_adaptor_spaceship_with_type_54() });
    ({ test_sequence_container_adaptor_spaceship_with_type_55() });
    ({ test_sequence_container_adaptor_spaceship_with_type_56() });
    ({ test_sequence_container_adaptor_spaceship_with_type_57() });
    ({ test_sequence_container_adaptor_spaceship_with_type_58() });
    return true;
}
pub fn test_sequence_container_adaptor_spaceship_70() -> () {
    const _: () = assert!(
        !(false),
        "!std::three_way_comparable<ContainerAdaptor<NonComparable>>"
    );;
    const _: () = assert!(
        true,
        "std::three_way_comparable<ContainerAdaptor<int, Container<int>>>"
    );;
    ({ test_sequence_container_adaptor_spaceship_with_type_59() });
    ({ test_sequence_container_adaptor_spaceship_with_type_60() });
    ({ test_sequence_container_adaptor_spaceship_with_type_61() });
    ({ test_sequence_container_adaptor_spaceship_with_type_62() });
    ({ test_sequence_container_adaptor_spaceship_with_type_63() });
    return true;
}
pub fn test_sequence_container_adaptor_spaceship_71() -> () {
    const _: () = assert!(
        !(false),
        "!std::three_way_comparable<ContainerAdaptor<NonComparable>>"
    );;
    const _: () = assert!(
        true,
        "std::three_way_comparable<ContainerAdaptor<int, Container<int>>>"
    );;
    ({ test_sequence_container_adaptor_spaceship_with_type_64() });
    ({ test_sequence_container_adaptor_spaceship_with_type_65() });
    ({ test_sequence_container_adaptor_spaceship_with_type_66() });
    ({ test_sequence_container_adaptor_spaceship_with_type_67() });
    ({ test_sequence_container_adaptor_spaceship_with_type_68() });
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
    unimplemented!();
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
pub trait nasty_list_LessAndEqComp_Impl {
    fn destructor(&self);
    fn operator_assign_9(&self, il: ()) -> Ptr<nasty_list_LessAndEqComp_> {
        unimplemented!()
    }
    fn assign_11(&self, mut n: (), t: Ptr<LessAndEqComp>) {
        unimplemented!()
    }
    fn assign_12(&self, il: ()) {
        unimplemented!()
    }
    fn begin_13(&self) -> () {
        unimplemented!()
    }
    fn begin_14(&self) -> () {
        unimplemented!()
    }
    fn end_15(&self) -> () {
        unimplemented!()
    }
    fn end_16(&self) -> () {
        unimplemented!()
    }
    fn rbegin_17(&self) -> () {
        unimplemented!()
    }
    fn rbegin_18(&self) -> () {
        unimplemented!()
    }
    fn rend_19(&self) -> () {
        unimplemented!()
    }
    fn rend_20(&self) -> () {
        unimplemented!()
    }
    fn cbegin(&self) -> () {
        unimplemented!()
    }
    fn cend(&self) -> () {
        unimplemented!()
    }
    fn crbegin(&self) -> () {
        unimplemented!()
    }
    fn crend(&self) -> () {
        unimplemented!()
    }
    fn front_25(&self) -> () {
        unimplemented!()
    }
    fn front_26(&self) -> () {
        unimplemented!()
    }
    fn back_27(&self) -> () {
        unimplemented!()
    }
    fn back_28(&self) -> () {
        unimplemented!()
    }
    fn size(&self) -> () {
        unimplemented!()
    }
    fn max_size(&self) -> () {
        unimplemented!()
    }
    fn empty(&self) -> () {
        unimplemented!()
    }
    fn push_front_32(&self, x: Ptr<LessAndEqComp>) {
        unimplemented!()
    }
    fn push_back_33(&self, x: Ptr<LessAndEqComp>) {
        unimplemented!()
    }
    fn push_back_34(&self, x: Ptr<LessAndEqComp>) {
        unimplemented!()
    }
    fn push_front_35(&self, x: Ptr<LessAndEqComp>) {
        unimplemented!()
    }
    fn pop_front(&self) {
        unimplemented!()
    }
    fn pop_back(&self) {
        unimplemented!()
    }
    fn insert_41(&self, pos: (), x: Ptr<LessAndEqComp>) -> () {
        unimplemented!()
    }
    fn insert_42(&self, pos: (), x: Ptr<LessAndEqComp>) -> () {
        unimplemented!()
    }
    fn insert_43(&self, pos: (), mut n: (), x: Ptr<LessAndEqComp>) -> () {
        unimplemented!()
    }
    fn insert_45(&self, pos: (), il: ()) -> () {
        unimplemented!()
    }
    fn erase_46(&self, pos: ()) -> () {
        unimplemented!()
    }
    fn erase_47(&self, first: (), last: ()) -> () {
        unimplemented!()
    }
    fn resize_48(&self, mut n: ()) {
        unimplemented!()
    }
    fn resize_49(&self, mut n: (), c: Ptr<LessAndEqComp>) {
        unimplemented!()
    }
    fn swap(&self, nl: Ptr<nasty_list_LessAndEqComp_>) {
        unimplemented!()
    }
    fn clear(&self) {
        unimplemented!()
    }
    fn operator_addr_52(&self) -> Ptr<nasty_list_LessAndEqComp_> {
        unimplemented!()
    }
    fn operator_addr_53(&self) -> Ptr<nasty_list_LessAndEqComp_> {
        unimplemented!()
    }
}
impl nasty_list_LessAndEqComp_Impl for Ptr<nasty_list_LessAndEqComp_> {
    fn destructor(&self) {}
}
pub trait nasty_list_PartialOrder_Impl {
    fn destructor(&self);
    fn operator_assign_9(&self, il: ()) -> Ptr<nasty_list_PartialOrder_> {
        unimplemented!()
    }
    fn assign_11(&self, mut n: (), t: Ptr<PartialOrder>) {
        unimplemented!()
    }
    fn assign_12(&self, il: ()) {
        unimplemented!()
    }
    fn begin_13(&self) -> () {
        unimplemented!()
    }
    fn begin_14(&self) -> () {
        unimplemented!()
    }
    fn end_15(&self) -> () {
        unimplemented!()
    }
    fn end_16(&self) -> () {
        unimplemented!()
    }
    fn rbegin_17(&self) -> () {
        unimplemented!()
    }
    fn rbegin_18(&self) -> () {
        unimplemented!()
    }
    fn rend_19(&self) -> () {
        unimplemented!()
    }
    fn rend_20(&self) -> () {
        unimplemented!()
    }
    fn cbegin(&self) -> () {
        unimplemented!()
    }
    fn cend(&self) -> () {
        unimplemented!()
    }
    fn crbegin(&self) -> () {
        unimplemented!()
    }
    fn crend(&self) -> () {
        unimplemented!()
    }
    fn front_25(&self) -> () {
        unimplemented!()
    }
    fn front_26(&self) -> () {
        unimplemented!()
    }
    fn back_27(&self) -> () {
        unimplemented!()
    }
    fn back_28(&self) -> () {
        unimplemented!()
    }
    fn size(&self) -> () {
        unimplemented!()
    }
    fn max_size(&self) -> () {
        unimplemented!()
    }
    fn empty(&self) -> () {
        unimplemented!()
    }
    fn push_front_32(&self, x: Ptr<PartialOrder>) {
        unimplemented!()
    }
    fn push_back_33(&self, x: Ptr<PartialOrder>) {
        unimplemented!()
    }
    fn push_back_34(&self, x: Ptr<PartialOrder>) {
        unimplemented!()
    }
    fn push_front_35(&self, x: Ptr<PartialOrder>) {
        unimplemented!()
    }
    fn pop_front(&self) {
        unimplemented!()
    }
    fn pop_back(&self) {
        unimplemented!()
    }
    fn insert_41(&self, pos: (), x: Ptr<PartialOrder>) -> () {
        unimplemented!()
    }
    fn insert_42(&self, pos: (), x: Ptr<PartialOrder>) -> () {
        unimplemented!()
    }
    fn insert_43(&self, pos: (), mut n: (), x: Ptr<PartialOrder>) -> () {
        unimplemented!()
    }
    fn insert_45(&self, pos: (), il: ()) -> () {
        unimplemented!()
    }
    fn erase_46(&self, pos: ()) -> () {
        unimplemented!()
    }
    fn erase_47(&self, first: (), last: ()) -> () {
        unimplemented!()
    }
    fn resize_48(&self, mut n: ()) {
        unimplemented!()
    }
    fn resize_49(&self, mut n: (), c: Ptr<PartialOrder>) {
        unimplemented!()
    }
    fn swap(&self, nl: Ptr<nasty_list_PartialOrder_>) {
        unimplemented!()
    }
    fn clear(&self) {
        unimplemented!()
    }
    fn operator_addr_52(&self) -> Ptr<nasty_list_PartialOrder_> {
        unimplemented!()
    }
    fn operator_addr_53(&self) -> Ptr<nasty_list_PartialOrder_> {
        unimplemented!()
    }
}
impl nasty_list_PartialOrder_Impl for Ptr<nasty_list_PartialOrder_> {
    fn destructor(&self) {}
}
pub trait nasty_list_StrongOrder_Impl {
    fn destructor(&self);
    fn operator_assign_9(&self, il: ()) -> Ptr<nasty_list_StrongOrder_> {
        unimplemented!()
    }
    fn assign_11(&self, mut n: (), t: Ptr<StrongOrder>) {
        unimplemented!()
    }
    fn assign_12(&self, il: ()) {
        unimplemented!()
    }
    fn begin_13(&self) -> () {
        unimplemented!()
    }
    fn begin_14(&self) -> () {
        unimplemented!()
    }
    fn end_15(&self) -> () {
        unimplemented!()
    }
    fn end_16(&self) -> () {
        unimplemented!()
    }
    fn rbegin_17(&self) -> () {
        unimplemented!()
    }
    fn rbegin_18(&self) -> () {
        unimplemented!()
    }
    fn rend_19(&self) -> () {
        unimplemented!()
    }
    fn rend_20(&self) -> () {
        unimplemented!()
    }
    fn cbegin(&self) -> () {
        unimplemented!()
    }
    fn cend(&self) -> () {
        unimplemented!()
    }
    fn crbegin(&self) -> () {
        unimplemented!()
    }
    fn crend(&self) -> () {
        unimplemented!()
    }
    fn front_25(&self) -> () {
        unimplemented!()
    }
    fn front_26(&self) -> () {
        unimplemented!()
    }
    fn back_27(&self) -> () {
        unimplemented!()
    }
    fn back_28(&self) -> () {
        unimplemented!()
    }
    fn size(&self) -> () {
        unimplemented!()
    }
    fn max_size(&self) -> () {
        unimplemented!()
    }
    fn empty(&self) -> () {
        unimplemented!()
    }
    fn push_front_32(&self, x: Ptr<StrongOrder>) {
        unimplemented!()
    }
    fn push_back_33(&self, x: Ptr<StrongOrder>) {
        unimplemented!()
    }
    fn push_back_34(&self, x: Ptr<StrongOrder>) {
        unimplemented!()
    }
    fn push_front_35(&self, x: Ptr<StrongOrder>) {
        unimplemented!()
    }
    fn pop_front(&self) {
        unimplemented!()
    }
    fn pop_back(&self) {
        unimplemented!()
    }
    fn insert_41(&self, pos: (), x: Ptr<StrongOrder>) -> () {
        unimplemented!()
    }
    fn insert_42(&self, pos: (), x: Ptr<StrongOrder>) -> () {
        unimplemented!()
    }
    fn insert_43(&self, pos: (), mut n: (), x: Ptr<StrongOrder>) -> () {
        unimplemented!()
    }
    fn insert_45(&self, pos: (), il: ()) -> () {
        unimplemented!()
    }
    fn erase_46(&self, pos: ()) -> () {
        unimplemented!()
    }
    fn erase_47(&self, first: (), last: ()) -> () {
        unimplemented!()
    }
    fn resize_48(&self, mut n: ()) {
        unimplemented!()
    }
    fn resize_49(&self, mut n: (), c: Ptr<StrongOrder>) {
        unimplemented!()
    }
    fn swap(&self, nl: Ptr<nasty_list_StrongOrder_>) {
        unimplemented!()
    }
    fn clear(&self) {
        unimplemented!()
    }
    fn operator_addr_52(&self) -> Ptr<nasty_list_StrongOrder_> {
        unimplemented!()
    }
    fn operator_addr_53(&self) -> Ptr<nasty_list_StrongOrder_> {
        unimplemented!()
    }
}
impl nasty_list_StrongOrder_Impl for Ptr<nasty_list_StrongOrder_> {
    fn destructor(&self) {}
}
pub trait nasty_list_WeakOrder_Impl {
    fn destructor(&self);
    fn operator_assign_9(&self, il: ()) -> Ptr<nasty_list_WeakOrder_> {
        unimplemented!()
    }
    fn assign_11(&self, mut n: (), t: Ptr<WeakOrder>) {
        unimplemented!()
    }
    fn assign_12(&self, il: ()) {
        unimplemented!()
    }
    fn begin_13(&self) -> () {
        unimplemented!()
    }
    fn begin_14(&self) -> () {
        unimplemented!()
    }
    fn end_15(&self) -> () {
        unimplemented!()
    }
    fn end_16(&self) -> () {
        unimplemented!()
    }
    fn rbegin_17(&self) -> () {
        unimplemented!()
    }
    fn rbegin_18(&self) -> () {
        unimplemented!()
    }
    fn rend_19(&self) -> () {
        unimplemented!()
    }
    fn rend_20(&self) -> () {
        unimplemented!()
    }
    fn cbegin(&self) -> () {
        unimplemented!()
    }
    fn cend(&self) -> () {
        unimplemented!()
    }
    fn crbegin(&self) -> () {
        unimplemented!()
    }
    fn crend(&self) -> () {
        unimplemented!()
    }
    fn front_25(&self) -> () {
        unimplemented!()
    }
    fn front_26(&self) -> () {
        unimplemented!()
    }
    fn back_27(&self) -> () {
        unimplemented!()
    }
    fn back_28(&self) -> () {
        unimplemented!()
    }
    fn size(&self) -> () {
        unimplemented!()
    }
    fn max_size(&self) -> () {
        unimplemented!()
    }
    fn empty(&self) -> () {
        unimplemented!()
    }
    fn push_front_32(&self, x: Ptr<WeakOrder>) {
        unimplemented!()
    }
    fn push_back_33(&self, x: Ptr<WeakOrder>) {
        unimplemented!()
    }
    fn push_back_34(&self, x: Ptr<WeakOrder>) {
        unimplemented!()
    }
    fn push_front_35(&self, x: Ptr<WeakOrder>) {
        unimplemented!()
    }
    fn pop_front(&self) {
        unimplemented!()
    }
    fn pop_back(&self) {
        unimplemented!()
    }
    fn insert_41(&self, pos: (), x: Ptr<WeakOrder>) -> () {
        unimplemented!()
    }
    fn insert_42(&self, pos: (), x: Ptr<WeakOrder>) -> () {
        unimplemented!()
    }
    fn insert_43(&self, pos: (), mut n: (), x: Ptr<WeakOrder>) -> () {
        unimplemented!()
    }
    fn insert_45(&self, pos: (), il: ()) -> () {
        unimplemented!()
    }
    fn erase_46(&self, pos: ()) -> () {
        unimplemented!()
    }
    fn erase_47(&self, first: (), last: ()) -> () {
        unimplemented!()
    }
    fn resize_48(&self, mut n: ()) {
        unimplemented!()
    }
    fn resize_49(&self, mut n: (), c: Ptr<WeakOrder>) {
        unimplemented!()
    }
    fn swap(&self, nl: Ptr<nasty_list_WeakOrder_>) {
        unimplemented!()
    }
    fn clear(&self) {
        unimplemented!()
    }
    fn operator_addr_52(&self) -> Ptr<nasty_list_WeakOrder_> {
        unimplemented!()
    }
    fn operator_addr_53(&self) -> Ptr<nasty_list_WeakOrder_> {
        unimplemented!()
    }
}
impl nasty_list_WeakOrder_Impl for Ptr<nasty_list_WeakOrder_> {
    fn destructor(&self) {}
}
pub trait nasty_list_int_Impl {
    fn destructor(&self);
    fn operator_assign_9(&self, il: ()) -> Ptr<nasty_list_int_> {
        unimplemented!()
    }
    fn assign_11(&self, mut n: (), t: Ptr<()>) {
        unimplemented!()
    }
    fn assign_12(&self, il: ()) {
        unimplemented!()
    }
    fn begin_13(&self) -> () {
        unimplemented!()
    }
    fn begin_14(&self) -> () {
        unimplemented!()
    }
    fn end_15(&self) -> () {
        unimplemented!()
    }
    fn end_16(&self) -> () {
        unimplemented!()
    }
    fn rbegin_17(&self) -> () {
        unimplemented!()
    }
    fn rbegin_18(&self) -> () {
        unimplemented!()
    }
    fn rend_19(&self) -> () {
        unimplemented!()
    }
    fn rend_20(&self) -> () {
        unimplemented!()
    }
    fn cbegin(&self) -> () {
        unimplemented!()
    }
    fn cend(&self) -> () {
        unimplemented!()
    }
    fn crbegin(&self) -> () {
        unimplemented!()
    }
    fn crend(&self) -> () {
        unimplemented!()
    }
    fn front_25(&self) -> () {
        unimplemented!()
    }
    fn front_26(&self) -> () {
        unimplemented!()
    }
    fn back_27(&self) -> () {
        unimplemented!()
    }
    fn back_28(&self) -> () {
        unimplemented!()
    }
    fn size(&self) -> () {
        unimplemented!()
    }
    fn max_size(&self) -> () {
        unimplemented!()
    }
    fn empty(&self) -> () {
        unimplemented!()
    }
    fn push_front_32(&self, x: Ptr<()>) {
        unimplemented!()
    }
    fn push_back_33(&self, x: Ptr<()>) {
        unimplemented!()
    }
    fn push_back_34(&self, x: Ptr<()>) {
        unimplemented!()
    }
    fn push_front_35(&self, x: Ptr<()>) {
        unimplemented!()
    }
    fn pop_front(&self) {
        unimplemented!()
    }
    fn pop_back(&self) {
        unimplemented!()
    }
    fn insert_41(&self, pos: (), x: Ptr<()>) -> () {
        unimplemented!()
    }
    fn insert_42(&self, pos: (), x: Ptr<()>) -> () {
        unimplemented!()
    }
    fn insert_43(&self, pos: (), mut n: (), x: Ptr<()>) -> () {
        unimplemented!()
    }
    fn insert_45(&self, pos: (), il: ()) -> () {
        unimplemented!()
    }
    fn erase_46(&self, pos: ()) -> () {
        unimplemented!()
    }
    fn erase_47(&self, first: (), last: ()) -> () {
        unimplemented!()
    }
    fn resize_48(&self, mut n: ()) {
        unimplemented!()
    }
    fn resize_49(&self, mut n: (), c: Ptr<()>) {
        unimplemented!()
    }
    fn swap(&self, nl: Ptr<nasty_list_int_>) {
        unimplemented!()
    }
    fn clear(&self) {
        unimplemented!()
    }
    fn operator_addr_52(&self) -> Ptr<nasty_list_int_> {
        unimplemented!()
    }
    fn operator_addr_53(&self) -> Ptr<nasty_list_int_> {
        unimplemented!()
    }
}
impl nasty_list_int_Impl for Ptr<nasty_list_int_> {
    fn destructor(&self) {}
}
pub trait nasty_mutexImpl {
    fn destructor(&self);
    fn operator_addr(&self) -> Ptr<nasty_mutex>;
    fn copy_assign(&self, _a0: Ptr<nasty_mutex>) -> Ptr<nasty_mutex>;
    fn lock(&self);
    fn try_lock(&self) -> ();
    fn unlock(&self);
    fn lock_shared(&self);
    fn try_lock_shared(&self) -> ();
    fn unlock_shared(&self);
}
impl nasty_mutexImpl for Ptr<nasty_mutex> {
    fn destructor(&self) {}
    fn operator_addr(&self) -> Ptr<nasty_mutex> {
        unimplemented!();
        return Ptr::<nasty_mutex>::null();
    }
    fn copy_assign(&self, _a0: Ptr<nasty_mutex>) -> Ptr<nasty_mutex> {
        unimplemented!();
        return (*self).clone();
    }
    fn lock(&self) {}
    fn try_lock(&self) -> () {
        return true;
    }
    fn unlock(&self) {}
    fn lock_shared(&self) {}
    fn try_lock_shared(&self) -> () {
        return true;
    }
    fn unlock_shared(&self) {}
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
        field!((*self), data_).write(destructed_value_10.with(|rc| *rc.borrow()));
        field!((*self), id_).write(destructed_value_10.with(|rc| *rc.borrow()));
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
    let _ = destructed_value_10.with(|_| ());
    let _ = moved_value_11.with(|_| ());
}
