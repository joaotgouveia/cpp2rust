extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct bare_allocator_void_ {}
impl bare_allocator_void_ {
    pub fn new() -> Self {
        Self {}
    }
}
impl Default for bare_allocator_void_ {
    fn default() -> Self {
        { bare_allocator_void_::new() }
    }
}
impl bare_allocator_Zero_ {
    pub fn new_1(mut _a0: bare_allocator_void_) -> Self {
        Self {}
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct bare_allocator_Zero_ {}
impl bare_allocator_Zero_ {}
impl bare_allocator_One_ {
    pub fn new_1(mut _a0: bare_allocator_void_) -> Self {
        Self {}
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct bare_allocator_One_ {}
impl bare_allocator_One_ {}
impl bare_allocator_Two_ {
    pub fn new_1(mut _a0: bare_allocator_void_) -> Self {
        Self {}
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct bare_allocator_Two_ {}
impl bare_allocator_Two_ {}
impl bare_allocator_Three_ {
    pub fn new_1(mut _a0: bare_allocator_void_) -> Self {
        Self {}
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct bare_allocator_Three_ {}
impl bare_allocator_Three_ {}
impl bare_allocator_A_ {
    pub fn new_1(mut _a0: bare_allocator_void_) -> Self {
        Self {}
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct bare_allocator_A_ {}
impl bare_allocator_A_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub fn outstanding_alloc() -> () {
        unimplemented!();
        return ((alloc_count_1.with(|rc| *rc.borrow()))
            .wrapping_sub(dealloc_count_2.with(|rc| *rc.borrow())));
    }
    pub fn reset() {
        unimplemented!();
        disable_default_constructor_3.with(|rc| *rc.borrow_mut() = false);
        outstanding_bytes_0.with(|rc| *rc.borrow_mut() = 0_());
        alloc_count_1.with(|rc| *rc.borrow_mut() = 0_());
        dealloc_count_2.with(|rc| *rc.borrow_mut() = 0_());
    }
}
thread_local!(
    pub static outstanding_bytes_0: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static alloc_count_1: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static dealloc_count_2: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static disable_default_constructor_3: Value<()> = Rc::new(RefCell::new(false));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: (),
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_operator_hijacker__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<operator_hijacker>,
}
impl min_pointer_operator_hijacker__std_integral_constant_unsigned_long__0__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_A__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<A>,
}
impl min_pointer_A__std_integral_constant_unsigned_long__0__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_void_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_operator_hijacker_ {}
impl min_allocator_A_ {
    pub fn new_1(mut _a0: min_allocator_void_) -> Self {
        Self {}
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_A_ {}
impl min_allocator_A_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct complete_type_allocator_int_ {}
impl complete_type_allocator_int_ {
    pub fn new() -> Self {
        Self {}
    }
}
impl Default for complete_type_allocator_int_ {
    fn default() -> Self {
        { complete_type_allocator_int_::new() }
    }
}
impl complete_type_allocator_Zero_ {
    pub fn new_1(mut _a0: complete_type_allocator_int_) -> Self {
        Self {}
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct complete_type_allocator_Zero_ {}
impl complete_type_allocator_Zero_ {}
impl complete_type_allocator_One_ {
    pub fn new_1(mut _a0: complete_type_allocator_int_) -> Self {
        Self {}
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct complete_type_allocator_One_ {}
impl complete_type_allocator_One_ {}
impl complete_type_allocator_Two_ {
    pub fn new_1(mut _a0: complete_type_allocator_int_) -> Self {
        Self {}
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct complete_type_allocator_Two_ {}
impl complete_type_allocator_Two_ {}
impl complete_type_allocator_Three_ {
    pub fn new_1(mut _a0: complete_type_allocator_int_) -> Self {
        Self {}
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct complete_type_allocator_Three_ {}
impl complete_type_allocator_Three_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct operator_hijacker {}
impl std::cmp::Ord for operator_hijacker {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            if operator_hijackerImpl::operator_lt(
                &Rc::new(RefCell::new(operator_hijacker {})).as_pointer(),
                Rc::new(RefCell::new(operator_hijacker {})).as_pointer(),
            ) {
                std::cmp::Ordering::Less
            } else if operator_hijackerImpl::operator_lt(
                &Rc::new(RefCell::new(operator_hijacker {})).as_pointer(),
                Rc::new(RefCell::new(operator_hijacker {})).as_pointer(),
            ) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for operator_hijacker {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for operator_hijacker {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_hijackerImpl::operator_eq(
                &Rc::new(RefCell::new(operator_hijacker {})).as_pointer(),
                Rc::new(RefCell::new(operator_hijacker {})).as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for operator_hijacker {}
const _: () = assert!(
    true,
    "std::is_trivially_copyable<operator_hijacker>::value &&     //\n                  std::is_copy_constructible<operator_hijacker>::value && //\n                  std::is_move_constructible<operator_hijacker>::value && //\n                  std::is_copy_assignable<operator_hijacker>::value &&    //\n                  std::is_move_assignable<operator_hijacker>::value"
);
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
    pub static destructed_value_4: Value<()> = Rc::new(RefCell::new(-1_()));
);
thread_local!(
    pub static moved_value_5: Value<()> = Rc::new(RefCell::new(2147483647_()));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_alloc_base {}
pub fn operator_eq_6(x: Ptr<test_allocator_void_>, y: Ptr<test_allocator_void_>) -> () {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
pub fn operator_ne_7(x: Ptr<test_allocator_void_>, y: Ptr<test_allocator_void_>) -> () {
    return !({
        let _x: Ptr<test_allocator_void_> = (x).clone();
        let _y: Ptr<test_allocator_void_> = (y).clone();
        operator_eq_6(_x, _y)
    });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_allocator_void__rebind_Zero_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_allocator_void__rebind_One_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_allocator_void__rebind_Two_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_allocator_void__rebind_Three_ {}
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
            operator_eq_6(
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
pub struct test_allocator_A__rebind_A_ {}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, Destructor)]
#[byte_size(16)]
pub struct test_allocator_A_ {
    #[offset(0)]
    data_: (),
    #[offset(4)]
    id_: (),
    #[offset(8)]
    #[byte_size(8)]
    stats_: Ptr<test_allocator_statistics>,
}
impl test_allocator_A_ {
    pub fn new_1(mut data: (), mut stats: Ptr<test_allocator_statistics>) -> Self {
        let __this: Value<test_allocator_A_> = Rc::new(RefCell::new(Self {
            data_: data,
            id_: 0_(),
            stats_: (stats).clone(),
        }));
        let this: Ptr<test_allocator_A_> = __this.as_pointer();
        if !((stats).is_null()) {
            field!(this.with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_inc());
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn copy_from(a: Ptr<test_allocator_A_>) -> Self {
        let __this: Value<test_allocator_A_> = Rc::new(RefCell::new(Self {
            data_: a.with(|__s| __s.data_),
            id_: a.with(|__s| __s.id_),
            stats_: a.with(|__s| __s.stats_.clone()),
        }));
        let this: Ptr<test_allocator_A_> = __this.as_pointer();
        unimplemented!();
        if !((this.with(|__s| __s.stats_.clone())).is_null()) {
            field!(this.with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_inc());
            field!(this.with(|__s| __s.stats_.clone()), copied).with_mut(|__v| __v.prefix_inc());
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn move_from(a: Ptr<test_allocator_A_>) -> Self {
        let __this: Value<test_allocator_A_> = Rc::new(RefCell::new(Self {
            data_: a.with(|__s| __s.data_),
            id_: a.with(|__s| __s.id_),
            stats_: a.with(|__s| __s.stats_.clone()),
        }));
        let this: Ptr<test_allocator_A_> = __this.as_pointer();
        if !((this.with(|__s| __s.stats_.clone())).is_null()) {
            field!(this.with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_inc());
            field!(this.with(|__s| __s.stats_.clone()), moved).with_mut(|__v| __v.prefix_inc());
        }
        unimplemented!();
        field!(a, id_).write(moved_value_5.with(|rc| *rc.borrow()));
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for test_allocator_A_ {
    fn clone(&self) -> Self {
        let __src: Value<test_allocator_A_> = Rc::new(RefCell::new(test_allocator_A_ {
            data_: self.data_.clone(),
            id_: self.id_.clone(),
            stats_: self.stats_.clone(),
        }));
        test_allocator_A_::copy_from(__src.as_pointer())
    }
}
impl Default for test_allocator_A_ {
    fn default() -> Self {
        {
            test_allocator_A_ {
                data_: unimplemented!(),
                id_: 0_(),
                stats_: Ptr::<test_allocator_statistics>::null(),
            }
        }
    }
}
impl test_allocator_Zero_ {
    pub fn new_1(a: Ptr<test_allocator_void_>) -> Self {
        let __this: Value<test_allocator_Zero_> = Rc::new(RefCell::new(Self {
            data_: a.with(|__s| __s.data_),
            id_: a.with(|__s| __s.id_),
            stats_: a.with(|__s| __s.stats_.clone()),
        }));
        let this: Ptr<test_allocator_Zero_> = __this.as_pointer();
        if !((this.with(|__s| __s.stats_.clone())).is_null()) {
            field!(this.with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_inc());
            field!(this.with(|__s| __s.stats_.clone()), converted).with_mut(|__v| __v.prefix_inc());
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, Destructor)]
#[byte_size(16)]
pub struct test_allocator_Zero_ {
    #[offset(0)]
    data_: (),
    #[offset(4)]
    id_: (),
    #[offset(8)]
    #[byte_size(8)]
    stats_: Ptr<test_allocator_statistics>,
}
impl test_allocator_Zero_ {
    pub fn copy_from(a: Ptr<test_allocator_Zero_>) -> Self {
        let __this: Value<test_allocator_Zero_> = Rc::new(RefCell::new(Self {
            data_: a.with(|__s| __s.data_),
            id_: a.with(|__s| __s.id_),
            stats_: a.with(|__s| __s.stats_.clone()),
        }));
        let this: Ptr<test_allocator_Zero_> = __this.as_pointer();
        unimplemented!();
        if !((this.with(|__s| __s.stats_.clone())).is_null()) {
            field!(this.with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_inc());
            field!(this.with(|__s| __s.stats_.clone()), copied).with_mut(|__v| __v.prefix_inc());
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn move_from(a: Ptr<test_allocator_Zero_>) -> Self {
        let __this: Value<test_allocator_Zero_> = Rc::new(RefCell::new(Self {
            data_: a.with(|__s| __s.data_),
            id_: a.with(|__s| __s.id_),
            stats_: a.with(|__s| __s.stats_.clone()),
        }));
        let this: Ptr<test_allocator_Zero_> = __this.as_pointer();
        if !((this.with(|__s| __s.stats_.clone())).is_null()) {
            field!(this.with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_inc());
            field!(this.with(|__s| __s.stats_.clone()), moved).with_mut(|__v| __v.prefix_inc());
        }
        unimplemented!();
        field!(a, id_).write(moved_value_5.with(|rc| *rc.borrow()));
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for test_allocator_Zero_ {
    fn clone(&self) -> Self {
        let __src: Value<test_allocator_Zero_> = Rc::new(RefCell::new(test_allocator_Zero_ {
            data_: self.data_.clone(),
            id_: self.id_.clone(),
            stats_: self.stats_.clone(),
        }));
        test_allocator_Zero_::copy_from(__src.as_pointer())
    }
}
impl Default for test_allocator_Zero_ {
    fn default() -> Self {
        {
            test_allocator_Zero_ {
                data_: unimplemented!(),
                id_: unimplemented!(),
                stats_: Ptr::<test_allocator_statistics>::null(),
            }
        }
    }
}
impl test_allocator_One_ {
    pub fn new_1(a: Ptr<test_allocator_void_>) -> Self {
        let __this: Value<test_allocator_One_> = Rc::new(RefCell::new(Self {
            data_: a.with(|__s| __s.data_),
            id_: a.with(|__s| __s.id_),
            stats_: a.with(|__s| __s.stats_.clone()),
        }));
        let this: Ptr<test_allocator_One_> = __this.as_pointer();
        if !((this.with(|__s| __s.stats_.clone())).is_null()) {
            field!(this.with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_inc());
            field!(this.with(|__s| __s.stats_.clone()), converted).with_mut(|__v| __v.prefix_inc());
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, Destructor)]
#[byte_size(16)]
pub struct test_allocator_One_ {
    #[offset(0)]
    data_: (),
    #[offset(4)]
    id_: (),
    #[offset(8)]
    #[byte_size(8)]
    stats_: Ptr<test_allocator_statistics>,
}
impl test_allocator_One_ {
    pub fn copy_from(a: Ptr<test_allocator_One_>) -> Self {
        let __this: Value<test_allocator_One_> = Rc::new(RefCell::new(Self {
            data_: a.with(|__s| __s.data_),
            id_: a.with(|__s| __s.id_),
            stats_: a.with(|__s| __s.stats_.clone()),
        }));
        let this: Ptr<test_allocator_One_> = __this.as_pointer();
        unimplemented!();
        if !((this.with(|__s| __s.stats_.clone())).is_null()) {
            field!(this.with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_inc());
            field!(this.with(|__s| __s.stats_.clone()), copied).with_mut(|__v| __v.prefix_inc());
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn move_from(a: Ptr<test_allocator_One_>) -> Self {
        let __this: Value<test_allocator_One_> = Rc::new(RefCell::new(Self {
            data_: a.with(|__s| __s.data_),
            id_: a.with(|__s| __s.id_),
            stats_: a.with(|__s| __s.stats_.clone()),
        }));
        let this: Ptr<test_allocator_One_> = __this.as_pointer();
        if !((this.with(|__s| __s.stats_.clone())).is_null()) {
            field!(this.with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_inc());
            field!(this.with(|__s| __s.stats_.clone()), moved).with_mut(|__v| __v.prefix_inc());
        }
        unimplemented!();
        field!(a, id_).write(moved_value_5.with(|rc| *rc.borrow()));
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for test_allocator_One_ {
    fn clone(&self) -> Self {
        let __src: Value<test_allocator_One_> = Rc::new(RefCell::new(test_allocator_One_ {
            data_: self.data_.clone(),
            id_: self.id_.clone(),
            stats_: self.stats_.clone(),
        }));
        test_allocator_One_::copy_from(__src.as_pointer())
    }
}
impl Default for test_allocator_One_ {
    fn default() -> Self {
        {
            test_allocator_One_ {
                data_: unimplemented!(),
                id_: unimplemented!(),
                stats_: Ptr::<test_allocator_statistics>::null(),
            }
        }
    }
}
impl test_allocator_Two_ {
    pub fn new_1(a: Ptr<test_allocator_void_>) -> Self {
        let __this: Value<test_allocator_Two_> = Rc::new(RefCell::new(Self {
            data_: a.with(|__s| __s.data_),
            id_: a.with(|__s| __s.id_),
            stats_: a.with(|__s| __s.stats_.clone()),
        }));
        let this: Ptr<test_allocator_Two_> = __this.as_pointer();
        if !((this.with(|__s| __s.stats_.clone())).is_null()) {
            field!(this.with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_inc());
            field!(this.with(|__s| __s.stats_.clone()), converted).with_mut(|__v| __v.prefix_inc());
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, Destructor)]
#[byte_size(16)]
pub struct test_allocator_Two_ {
    #[offset(0)]
    data_: (),
    #[offset(4)]
    id_: (),
    #[offset(8)]
    #[byte_size(8)]
    stats_: Ptr<test_allocator_statistics>,
}
impl test_allocator_Two_ {
    pub fn copy_from(a: Ptr<test_allocator_Two_>) -> Self {
        let __this: Value<test_allocator_Two_> = Rc::new(RefCell::new(Self {
            data_: a.with(|__s| __s.data_),
            id_: a.with(|__s| __s.id_),
            stats_: a.with(|__s| __s.stats_.clone()),
        }));
        let this: Ptr<test_allocator_Two_> = __this.as_pointer();
        unimplemented!();
        if !((this.with(|__s| __s.stats_.clone())).is_null()) {
            field!(this.with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_inc());
            field!(this.with(|__s| __s.stats_.clone()), copied).with_mut(|__v| __v.prefix_inc());
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn move_from(a: Ptr<test_allocator_Two_>) -> Self {
        let __this: Value<test_allocator_Two_> = Rc::new(RefCell::new(Self {
            data_: a.with(|__s| __s.data_),
            id_: a.with(|__s| __s.id_),
            stats_: a.with(|__s| __s.stats_.clone()),
        }));
        let this: Ptr<test_allocator_Two_> = __this.as_pointer();
        if !((this.with(|__s| __s.stats_.clone())).is_null()) {
            field!(this.with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_inc());
            field!(this.with(|__s| __s.stats_.clone()), moved).with_mut(|__v| __v.prefix_inc());
        }
        unimplemented!();
        field!(a, id_).write(moved_value_5.with(|rc| *rc.borrow()));
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for test_allocator_Two_ {
    fn clone(&self) -> Self {
        let __src: Value<test_allocator_Two_> = Rc::new(RefCell::new(test_allocator_Two_ {
            data_: self.data_.clone(),
            id_: self.id_.clone(),
            stats_: self.stats_.clone(),
        }));
        test_allocator_Two_::copy_from(__src.as_pointer())
    }
}
impl Default for test_allocator_Two_ {
    fn default() -> Self {
        {
            test_allocator_Two_ {
                data_: unimplemented!(),
                id_: unimplemented!(),
                stats_: Ptr::<test_allocator_statistics>::null(),
            }
        }
    }
}
impl test_allocator_Three_ {
    pub fn new_1(a: Ptr<test_allocator_void_>) -> Self {
        let __this: Value<test_allocator_Three_> = Rc::new(RefCell::new(Self {
            data_: a.with(|__s| __s.data_),
            id_: a.with(|__s| __s.id_),
            stats_: a.with(|__s| __s.stats_.clone()),
        }));
        let this: Ptr<test_allocator_Three_> = __this.as_pointer();
        if !((this.with(|__s| __s.stats_.clone())).is_null()) {
            field!(this.with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_inc());
            field!(this.with(|__s| __s.stats_.clone()), converted).with_mut(|__v| __v.prefix_inc());
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, Destructor)]
#[byte_size(16)]
pub struct test_allocator_Three_ {
    #[offset(0)]
    data_: (),
    #[offset(4)]
    id_: (),
    #[offset(8)]
    #[byte_size(8)]
    stats_: Ptr<test_allocator_statistics>,
}
impl test_allocator_Three_ {
    pub fn copy_from(a: Ptr<test_allocator_Three_>) -> Self {
        let __this: Value<test_allocator_Three_> = Rc::new(RefCell::new(Self {
            data_: a.with(|__s| __s.data_),
            id_: a.with(|__s| __s.id_),
            stats_: a.with(|__s| __s.stats_.clone()),
        }));
        let this: Ptr<test_allocator_Three_> = __this.as_pointer();
        unimplemented!();
        if !((this.with(|__s| __s.stats_.clone())).is_null()) {
            field!(this.with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_inc());
            field!(this.with(|__s| __s.stats_.clone()), copied).with_mut(|__v| __v.prefix_inc());
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn move_from(a: Ptr<test_allocator_Three_>) -> Self {
        let __this: Value<test_allocator_Three_> = Rc::new(RefCell::new(Self {
            data_: a.with(|__s| __s.data_),
            id_: a.with(|__s| __s.id_),
            stats_: a.with(|__s| __s.stats_.clone()),
        }));
        let this: Ptr<test_allocator_Three_> = __this.as_pointer();
        if !((this.with(|__s| __s.stats_.clone())).is_null()) {
            field!(this.with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_inc());
            field!(this.with(|__s| __s.stats_.clone()), moved).with_mut(|__v| __v.prefix_inc());
        }
        unimplemented!();
        field!(a, id_).write(moved_value_5.with(|rc| *rc.borrow()));
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for test_allocator_Three_ {
    fn clone(&self) -> Self {
        let __src: Value<test_allocator_Three_> = Rc::new(RefCell::new(test_allocator_Three_ {
            data_: self.data_.clone(),
            id_: self.id_.clone(),
            stats_: self.stats_.clone(),
        }));
        test_allocator_Three_::copy_from(__src.as_pointer())
    }
}
impl Default for test_allocator_Three_ {
    fn default() -> Self {
        {
            test_allocator_Three_ {
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
thread_local!(
    pub static new_count_8: Value<()> = Rc::new(RefCell::new(0_()));
);
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CopyAssign, Destructor, Default)]
#[byte_size(8)]
pub struct A {
    #[offset(0)]
    int_: (),
    #[offset(4)]
    char_: (),
}
impl A {
    pub fn new(mut i: (), mut c: ()) -> Self {
        let __this: A = Self { int_: i, char_: c };
        (*count_9.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
    pub fn copy_from(a: Ptr<A>) -> Self {
        let __this: A = Self {
            int_: a.with(|__s| __s.int_),
            char_: a.with(|__s| __s.char_),
        };
        (*count_9.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
}
impl Clone for A {
    fn clone(&self) -> Self {
        let __src: Value<A> = Rc::new(RefCell::new(A {
            int_: self.int_.clone(),
            char_: self.char_.clone(),
        }));
        A::copy_from(__src.as_pointer())
    }
}
thread_local!(
    pub static count_9: Value<()> = Rc::new(RefCell::new(0_()));
);
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CopyAssign, Destructor)]
#[byte_size(1)]
pub struct Zero {}
impl Zero {
    pub fn new() -> Self {
        let __this: Zero = Self {};
        (*count_10.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
    pub fn copy_from(_a0: Ptr<Zero>) -> Self {
        let __this: Zero = Self {};
        (*count_10.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
}
impl Clone for Zero {
    fn clone(&self) -> Self {
        let __src: Value<Zero> = Rc::new(RefCell::new(Zero {}));
        Zero::copy_from(__src.as_pointer())
    }
}
impl Default for Zero {
    fn default() -> Self {
        { Zero::new() }
    }
}
thread_local!(
    pub static count_10: Value<()> = Rc::new(RefCell::new(0_()));
);
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CopyAssign, Destructor, Default)]
#[byte_size(4)]
pub struct One {
    #[offset(0)]
    pub value: (),
}
impl One {
    pub fn new(mut v: ()) -> Self {
        let __this: One = Self { value: v };
        (*count_11.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
    pub fn copy_from(o: Ptr<One>) -> Self {
        let __this: One = Self {
            value: o.with(|__s| __s.value),
        };
        (*count_11.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
}
impl Clone for One {
    fn clone(&self) -> Self {
        let __src: Value<One> = Rc::new(RefCell::new(One {
            value: self.value.clone(),
        }));
        One::copy_from(__src.as_pointer())
    }
}
thread_local!(
    pub static count_11: Value<()> = Rc::new(RefCell::new(0_()));
);
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CopyAssign, Destructor, Default)]
#[byte_size(4)]
pub struct Two {
    #[offset(0)]
    pub value: (),
}
impl Two {
    pub fn new(mut v: (), mut _a1: ()) -> Self {
        let __this: Two = Self { value: v };
        (*count_12.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
    pub fn copy_from(o: Ptr<Two>) -> Self {
        let __this: Two = Self {
            value: o.with(|__s| __s.value),
        };
        (*count_12.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
}
impl Clone for Two {
    fn clone(&self) -> Self {
        let __src: Value<Two> = Rc::new(RefCell::new(Two {
            value: self.value.clone(),
        }));
        Two::copy_from(__src.as_pointer())
    }
}
thread_local!(
    pub static count_12: Value<()> = Rc::new(RefCell::new(0_()));
);
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CopyAssign, Destructor, Default)]
#[byte_size(4)]
pub struct Three {
    #[offset(0)]
    pub value: (),
}
impl Three {
    pub fn new(mut v: (), mut _a1: (), mut _a2: ()) -> Self {
        let __this: Three = Self { value: v };
        (*count_13.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
    pub fn copy_from(o: Ptr<Three>) -> Self {
        let __this: Three = Self {
            value: o.with(|__s| __s.value),
        };
        (*count_13.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
}
impl Clone for Three {
    fn clone(&self) -> Self {
        let __src: Value<Three> = Rc::new(RefCell::new(Three {
            value: self.value.clone(),
        }));
        Three::copy_from(__src.as_pointer())
    }
}
thread_local!(
    pub static count_13: Value<()> = Rc::new(RefCell::new(0_()));
);
pub fn test_14() {
    let bad: Value<()> = Rc::new(RefCell::new(-1_()));
    {
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    unimplemented!();
    {
        let mut i: () = 42_();
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
    }
    unimplemented!();
    {
        let mut i: () = 42_();
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
    }
    unimplemented!();
    {
        let mut i: () = 42_();
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
    }
    unimplemented!();
}
pub fn test_15() {
    let bad: Value<()> = Rc::new(RefCell::new(-1_()));
    {
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    unimplemented!();
    {
        let mut i: () = 42_();
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
    }
    unimplemented!();
    {
        let mut i: () = 42_();
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
    }
    unimplemented!();
    {
        let mut i: () = 42_();
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
    }
    unimplemented!();
}
pub fn test_16() {
    let bad: Value<()> = Rc::new(RefCell::new(-1_()));
    {
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    unimplemented!();
    {
        let mut i: () = 42_();
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
    }
    unimplemented!();
    {
        let mut i: () = 42_();
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
    }
    unimplemented!();
    {
        let mut i: () = 42_();
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
    }
    unimplemented!();
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
    ({ test_14() });
    ({ test_15() });
    ({ test_16() });
    let alloc_stats: Value<test_allocator_statistics> =
        Rc::new(RefCell::new(<test_allocator_statistics>::default()));
    {
        let mut i: () = 67_();
        let c: Value<()> = Rc::new(RefCell::new(('e' as ())));
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    unimplemented!();
    unimplemented!();
    {
        let mut i: () = 67_();
        let c: Value<()> = Rc::new(RefCell::new(('e' as ())));
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    unimplemented!();
    {
        let mut i: () = 68_();
        let c: Value<()> = Rc::new(RefCell::new(('f' as ())));
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    unimplemented!();
    {
        let p1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let p2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
    }
    return 0_();
}
pub trait AImpl {
    fn destructor(&self);
    fn get_int(&self) -> ();
    fn get_char(&self) -> ();
    fn copy_assign(&self, _a0: Ptr<A>) -> Ptr<A>;
}
impl AImpl for Ptr<A> {
    fn destructor(&self) {
        (*count_9.with(Value::clone).borrow_mut()).prefix_dec();
    }
    fn get_int(&self) -> () {
        return (*self).with(|__s| __s.int_);
    }
    fn get_char(&self) -> () {
        return (*self).with(|__s| __s.char_);
    }
    fn copy_assign(&self, _a0: Ptr<A>) -> Ptr<A> {
        field!((*self), int_).write({ _a0.with(|__s| __s.int_) });
        field!((*self), char_).write({ _a0.with(|__s| __s.char_) });
        return (*self).clone();
    }
}
pub trait OneImpl {
    fn destructor(&self);
    fn copy_assign(&self, _a0: Ptr<One>) -> Ptr<One>;
}
impl OneImpl for Ptr<One> {
    fn destructor(&self) {
        (*count_11.with(Value::clone).borrow_mut()).prefix_dec();
    }
    fn copy_assign(&self, _a0: Ptr<One>) -> Ptr<One> {
        field!((*self), value).write({ _a0.with(|__s| __s.value) });
        return (*self).clone();
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
pub trait ThreeImpl {
    fn destructor(&self);
    fn copy_assign(&self, _a0: Ptr<Three>) -> Ptr<Three>;
}
impl ThreeImpl for Ptr<Three> {
    fn destructor(&self) {
        (*count_13.with(Value::clone).borrow_mut()).prefix_dec();
    }
    fn copy_assign(&self, _a0: Ptr<Three>) -> Ptr<Three> {
        field!((*self), value).write({ _a0.with(|__s| __s.value) });
        return (*self).clone();
    }
}
pub trait TwoImpl {
    fn destructor(&self);
    fn copy_assign(&self, _a0: Ptr<Two>) -> Ptr<Two>;
}
impl TwoImpl for Ptr<Two> {
    fn destructor(&self) {
        (*count_12.with(Value::clone).borrow_mut()).prefix_dec();
    }
    fn copy_assign(&self, _a0: Ptr<Two>) -> Ptr<Two> {
        field!((*self), value).write({ _a0.with(|__s| __s.value) });
        return (*self).clone();
    }
}
pub trait ZeroImpl {
    fn destructor(&self);
    fn copy_assign(&self, _a0: Ptr<Zero>) -> Ptr<Zero>;
}
impl ZeroImpl for Ptr<Zero> {
    fn destructor(&self) {
        (*count_10.with(Value::clone).borrow_mut()).prefix_dec();
    }
    fn copy_assign(&self, _a0: Ptr<Zero>) -> Ptr<Zero> {
        return (*self).clone();
    }
}
pub trait bare_allocator_A_Impl {
    fn allocate(&self, mut n: ()) -> Ptr<A> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<A>, mut _a1: ()) {
        unimplemented!()
    }
}
impl bare_allocator_A_Impl for Ptr<bare_allocator_A_> {}
pub trait bare_allocator_One_Impl {
    fn allocate(&self, mut n: ()) -> Ptr<One> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<One>, mut _a1: ()) {
        unimplemented!()
    }
}
impl bare_allocator_One_Impl for Ptr<bare_allocator_One_> {}
pub trait bare_allocator_Three_Impl {
    fn allocate(&self, mut n: ()) -> Ptr<Three> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<Three>, mut _a1: ()) {
        unimplemented!()
    }
}
impl bare_allocator_Three_Impl for Ptr<bare_allocator_Three_> {}
pub trait bare_allocator_Two_Impl {
    fn allocate(&self, mut n: ()) -> Ptr<Two> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<Two>, mut _a1: ()) {
        unimplemented!()
    }
}
impl bare_allocator_Two_Impl for Ptr<bare_allocator_Two_> {}
pub trait bare_allocator_Zero_Impl {
    fn allocate(&self, mut n: ()) -> Ptr<Zero> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<Zero>, mut _a1: ()) {
        unimplemented!()
    }
}
impl bare_allocator_Zero_Impl for Ptr<bare_allocator_Zero_> {}
pub trait bare_allocator_void_Impl {
    fn allocate(&self, mut n: ()) -> () {
        unimplemented!()
    }
    fn deallocate(&self, mut p: (), mut _a1: ()) {
        unimplemented!()
    }
}
impl bare_allocator_void_Impl for Ptr<bare_allocator_void_> {}
pub trait complete_type_allocator_One_Impl {
    fn allocate(&self, mut n: ()) -> Ptr<One> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<One>, mut n: ()) {
        unimplemented!()
    }
}
impl complete_type_allocator_One_Impl for Ptr<complete_type_allocator_One_> {}
pub trait complete_type_allocator_Three_Impl {
    fn allocate(&self, mut n: ()) -> Ptr<Three> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<Three>, mut n: ()) {
        unimplemented!()
    }
}
impl complete_type_allocator_Three_Impl for Ptr<complete_type_allocator_Three_> {}
pub trait complete_type_allocator_Two_Impl {
    fn allocate(&self, mut n: ()) -> Ptr<Two> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<Two>, mut n: ()) {
        unimplemented!()
    }
}
impl complete_type_allocator_Two_Impl for Ptr<complete_type_allocator_Two_> {}
pub trait complete_type_allocator_Zero_Impl {
    fn allocate(&self, mut n: ()) -> Ptr<Zero> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<Zero>, mut n: ()) {
        unimplemented!()
    }
}
impl complete_type_allocator_Zero_Impl for Ptr<complete_type_allocator_Zero_> {}
pub trait complete_type_allocator_int_Impl {
    fn allocate(&self, mut n: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<()>, mut n: ()) {
        unimplemented!()
    }
}
impl complete_type_allocator_int_Impl for Ptr<complete_type_allocator_int_> {}
pub trait min_allocator_A_Impl {
    fn allocate(&self, mut n: ()) -> min_pointer_A__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn deallocate(
        &self,
        mut p: min_pointer_A__std_integral_constant_unsigned_long__0__,
        mut n: (),
    ) {
        unimplemented!()
    }
}
impl min_allocator_A_Impl for Ptr<min_allocator_A_> {}
pub trait min_allocator_operator_hijacker_Impl {
    fn allocate(
        &self,
        mut n: (),
    ) -> min_pointer_operator_hijacker__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn deallocate(
        &self,
        mut p: min_pointer_operator_hijacker__std_integral_constant_unsigned_long__0__,
        mut n: (),
    ) {
        unimplemented!()
    }
}
impl min_allocator_operator_hijacker_Impl for Ptr<min_allocator_operator_hijacker_> {}
pub trait min_allocator_void_Impl {
    fn allocate(&self, mut n: ()) -> min_pointer_void__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn deallocate(
        &self,
        mut p: min_pointer_void__std_integral_constant_unsigned_long__0__,
        mut n: (),
    ) {
        unimplemented!()
    }
}
impl min_allocator_void_Impl for Ptr<min_allocator_void_> {}
pub trait min_pointer_A__std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<A> {
        unimplemented!()
    }
    fn operator_arrow(&self) -> Ptr<A> {
        unimplemented!()
    }
    fn operator_inc_8(&self) -> Ptr<min_pointer_A__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_inc_9(
        &self,
        mut _a0: (),
    ) -> min_pointer_A__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_dec_10(&self) -> Ptr<min_pointer_A__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_dec_11(
        &self,
        mut _a0: (),
    ) -> min_pointer_A__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_A__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_sub_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_A__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_add(&self, mut n: ()) -> min_pointer_A__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_sub(&self, mut n: ()) -> min_pointer_A__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_index(&self, mut n: ()) -> Ptr<A> {
        unimplemented!()
    }
}
impl min_pointer_A__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_A__std_integral_constant_unsigned_long__0__>
{
}
pub trait min_pointer_operator_hijacker__std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<operator_hijacker> {
        unimplemented!()
    }
    fn operator_arrow(&self) -> Ptr<operator_hijacker> {
        unimplemented!()
    }
    fn operator_inc_8(
        &self,
    ) -> Ptr<min_pointer_operator_hijacker__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_inc_9(
        &self,
        mut _a0: (),
    ) -> min_pointer_operator_hijacker__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_dec_10(
        &self,
    ) -> Ptr<min_pointer_operator_hijacker__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_dec_11(
        &self,
        mut _a0: (),
    ) -> min_pointer_operator_hijacker__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_operator_hijacker__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_sub_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_operator_hijacker__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_add(
        &self,
        mut n: (),
    ) -> min_pointer_operator_hijacker__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_sub(
        &self,
        mut n: (),
    ) -> min_pointer_operator_hijacker__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_index(&self, mut n: ()) -> Ptr<operator_hijacker> {
        unimplemented!()
    }
}
impl min_pointer_operator_hijacker__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_operator_hijacker__std_integral_constant_unsigned_long__0__>
{
}
pub trait min_pointer_void__std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_void__std_integral_constant_unsigned_long__0__>
{
}
pub trait operator_hijackerImpl {
    fn operator_lt(&self, _a0: Ptr<operator_hijacker>) -> ();
    fn operator_eq(&self, _a0: Ptr<operator_hijacker>) -> ();
    fn operator_call(&self) -> ();
}
impl operator_hijackerImpl for Ptr<operator_hijacker> {
    fn operator_lt(&self, _a0: Ptr<operator_hijacker>) -> () {
        return true;
    }
    fn operator_eq(&self, _a0: Ptr<operator_hijacker>) -> () {
        return true;
    }
    fn operator_call(&self) -> () {
        return 42_();
    }
}
pub trait test_allocator_A_Impl {
    fn destructor(&self);
    fn address_11(&self, x: ()) -> Ptr<A> {
        unimplemented!()
    }
    fn address_12(&self, x: ()) -> Ptr<A> {
        unimplemented!()
    }
    fn allocate(&self, mut n: (), mut _a1: ()) -> Ptr<A> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<A>, mut s: ()) {
        unimplemented!()
    }
    fn max_size(&self) -> () {
        unimplemented!()
    }
    fn destroy(&self, p: Ptr<A>);
    fn get_data(&self) -> () {
        unimplemented!()
    }
    fn get_id(&self) -> () {
        unimplemented!()
    }
}
impl test_allocator_A_Impl for Ptr<test_allocator_A_> {
    fn destructor(&self) {
        unimplemented!();
        unimplemented!();
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_dec());
        }
        field!((*self), data_).write(destructed_value_4.with(|rc| *rc.borrow()));
        field!((*self), id_).write(destructed_value_4.with(|rc| *rc.borrow()));
    }
    fn destroy(&self, mut p: Ptr<A>) {
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), destroy_count)
                .with_mut(|__v| __v.prefix_inc());
        }
        ({ AImpl::destructor(&p) });
    }
}
pub trait test_allocator_One_Impl {
    fn destructor(&self);
    fn address_11(&self, x: ()) -> Ptr<One> {
        unimplemented!()
    }
    fn address_12(&self, x: ()) -> Ptr<One> {
        unimplemented!()
    }
    fn allocate(&self, mut n: (), mut _a1: ()) -> Ptr<One> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<One>, mut s: ()) {
        unimplemented!()
    }
    fn max_size(&self) -> () {
        unimplemented!()
    }
    fn destroy(&self, p: Ptr<One>);
    fn get_data(&self) -> () {
        unimplemented!()
    }
    fn get_id(&self) -> () {
        unimplemented!()
    }
    fn construct_16_const_int_ref(&self, p: Ptr<One>, val: Ptr<()>);
}
impl test_allocator_One_Impl for Ptr<test_allocator_One_> {
    fn destructor(&self) {
        unimplemented!();
        unimplemented!();
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_dec());
        }
        field!((*self), data_).write(destructed_value_4.with(|rc| *rc.borrow()));
        field!((*self), id_).write(destructed_value_4.with(|rc| *rc.borrow()));
    }
    fn destroy(&self, mut p: Ptr<One>) {
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), destroy_count)
                .with_mut(|__v| __v.prefix_inc());
        }
        ({ OneImpl::destructor(&p) });
    }
    fn construct_16_const_int_ref(&self, mut p: Ptr<One>, val: Ptr<()>) {
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), construct_count)
                .with_mut(|__v| __v.prefix_inc());
        }
        unimplemented!();
    }
}
pub trait test_allocator_Three_Impl {
    fn destructor(&self);
    fn address_11(&self, x: ()) -> Ptr<Three> {
        unimplemented!()
    }
    fn address_12(&self, x: ()) -> Ptr<Three> {
        unimplemented!()
    }
    fn allocate(&self, mut n: (), mut _a1: ()) -> Ptr<Three> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<Three>, mut s: ()) {
        unimplemented!()
    }
    fn max_size(&self) -> () {
        unimplemented!()
    }
    fn destroy(&self, p: Ptr<Three>);
    fn get_data(&self) -> () {
        unimplemented!()
    }
    fn get_id(&self) -> () {
        unimplemented!()
    }
}
impl test_allocator_Three_Impl for Ptr<test_allocator_Three_> {
    fn destructor(&self) {
        unimplemented!();
        unimplemented!();
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_dec());
        }
        field!((*self), data_).write(destructed_value_4.with(|rc| *rc.borrow()));
        field!((*self), id_).write(destructed_value_4.with(|rc| *rc.borrow()));
    }
    fn destroy(&self, mut p: Ptr<Three>) {
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), destroy_count)
                .with_mut(|__v| __v.prefix_inc());
        }
        ({ ThreeImpl::destructor(&p) });
    }
}
pub trait test_allocator_Two_Impl {
    fn destructor(&self);
    fn address_11(&self, x: ()) -> Ptr<Two> {
        unimplemented!()
    }
    fn address_12(&self, x: ()) -> Ptr<Two> {
        unimplemented!()
    }
    fn allocate(&self, mut n: (), mut _a1: ()) -> Ptr<Two> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<Two>, mut s: ()) {
        unimplemented!()
    }
    fn max_size(&self) -> () {
        unimplemented!()
    }
    fn destroy(&self, p: Ptr<Two>);
    fn get_data(&self) -> () {
        unimplemented!()
    }
    fn get_id(&self) -> () {
        unimplemented!()
    }
}
impl test_allocator_Two_Impl for Ptr<test_allocator_Two_> {
    fn destructor(&self) {
        unimplemented!();
        unimplemented!();
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_dec());
        }
        field!((*self), data_).write(destructed_value_4.with(|rc| *rc.borrow()));
        field!((*self), id_).write(destructed_value_4.with(|rc| *rc.borrow()));
    }
    fn destroy(&self, mut p: Ptr<Two>) {
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), destroy_count)
                .with_mut(|__v| __v.prefix_inc());
        }
        ({ TwoImpl::destructor(&p) });
    }
}
pub trait test_allocator_Zero_Impl {
    fn destructor(&self);
    fn address_11(&self, x: ()) -> Ptr<Zero> {
        unimplemented!()
    }
    fn address_12(&self, x: ()) -> Ptr<Zero> {
        unimplemented!()
    }
    fn allocate(&self, mut n: (), mut _a1: ()) -> Ptr<Zero> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<Zero>, mut s: ()) {
        unimplemented!()
    }
    fn max_size(&self) -> () {
        unimplemented!()
    }
    fn destroy(&self, p: Ptr<Zero>);
    fn get_data(&self) -> () {
        unimplemented!()
    }
    fn get_id(&self) -> () {
        unimplemented!()
    }
}
impl test_allocator_Zero_Impl for Ptr<test_allocator_Zero_> {
    fn destructor(&self) {
        unimplemented!();
        unimplemented!();
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_dec());
        }
        field!((*self), data_).write(destructed_value_4.with(|rc| *rc.borrow()));
        field!((*self), id_).write(destructed_value_4.with(|rc| *rc.borrow()));
    }
    fn destroy(&self, mut p: Ptr<Zero>) {
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), destroy_count)
                .with_mut(|__v| __v.prefix_inc());
        }
        ({ ZeroImpl::destructor(&p) });
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
        field!((*self), data_).write(destructed_value_4.with(|rc| *rc.borrow()));
        field!((*self), id_).write(destructed_value_4.with(|rc| *rc.borrow()));
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
    let _ = outstanding_bytes_0.with(|_| ());
    let _ = alloc_count_1.with(|_| ());
    let _ = dealloc_count_2.with(|_| ());
    let _ = disable_default_constructor_3.with(|_| ());
    let _ = destructed_value_4.with(|_| ());
    let _ = moved_value_5.with(|_| ());
    let _ = new_count_8.with(|_| ());
    let _ = count_9.with(|_| ());
    let _ = count_10.with(|_| ());
    let _ = count_11.with(|_| ());
    let _ = count_12.with(|_| ());
    let _ = count_13.with(|_| ());
}
