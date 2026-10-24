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
pub fn operator_eq_4(x: Ptr<test_allocator_int_>, y: Ptr<test_allocator_int_>) -> () {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_allocator_int__rebind_int_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_allocator_int__rebind_int_ptr_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_allocator_int__rebind_const_int_ptr_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_allocator_int__rebind_unsigned_int_ {}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, Destructor)]
#[byte_size(16)]
pub struct test_allocator_int_ {
    #[offset(0)]
    data_: (),
    #[offset(4)]
    id_: (),
    #[offset(8)]
    #[byte_size(8)]
    stats_: Ptr<test_allocator_statistics>,
}
impl test_allocator_int_ {
    pub fn new_1(mut data: (), mut id: (), mut stats: Ptr<test_allocator_statistics>) -> Self {
        let __this: Value<test_allocator_int_> = Rc::new(RefCell::new(Self {
            data_: data,
            id_: id,
            stats_: (stats).clone(),
        }));
        let this: Ptr<test_allocator_int_> = __this.as_pointer();
        if !((this.with(|__s| __s.stats_.clone())).is_null()) {
            field!(this.with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_inc());
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn copy_from(a: Ptr<test_allocator_int_>) -> Self {
        let __this: Value<test_allocator_int_> = Rc::new(RefCell::new(Self {
            data_: a.with(|__s| __s.data_),
            id_: a.with(|__s| __s.id_),
            stats_: a.with(|__s| __s.stats_.clone()),
        }));
        let this: Ptr<test_allocator_int_> = __this.as_pointer();
        unimplemented!();
        if !((this.with(|__s| __s.stats_.clone())).is_null()) {
            field!(this.with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_inc());
            field!(this.with(|__s| __s.stats_.clone()), copied).with_mut(|__v| __v.prefix_inc());
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn move_from(a: Ptr<test_allocator_int_>) -> Self {
        let __this: Value<test_allocator_int_> = Rc::new(RefCell::new(Self {
            data_: a.with(|__s| __s.data_),
            id_: a.with(|__s| __s.id_),
            stats_: a.with(|__s| __s.stats_.clone()),
        }));
        let this: Ptr<test_allocator_int_> = __this.as_pointer();
        if !((this.with(|__s| __s.stats_.clone())).is_null()) {
            field!(this.with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_inc());
            field!(this.with(|__s| __s.stats_.clone()), moved).with_mut(|__v| __v.prefix_inc());
        }
        unimplemented!();
        field!(a, id_).write(moved_value_1.with(|rc| *rc.borrow()));
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl std::cmp::PartialEq for test_allocator_int_ {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_4(
                Rc::new(RefCell::new(test_allocator_int_ {
                    data_: self.data_.clone(),
                    id_: self.id_.clone(),
                    stats_: self.stats_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(test_allocator_int_ {
                    data_: other.data_.clone(),
                    id_: other.id_.clone(),
                    stats_: other.stats_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for test_allocator_int_ {}
impl Clone for test_allocator_int_ {
    fn clone(&self) -> Self {
        let __src: Value<test_allocator_int_> = Rc::new(RefCell::new(test_allocator_int_ {
            data_: self.data_.clone(),
            id_: self.id_.clone(),
            stats_: self.stats_.clone(),
        }));
        test_allocator_int_::copy_from(__src.as_pointer())
    }
}
impl Default for test_allocator_int_ {
    fn default() -> Self {
        {
            test_allocator_int_ {
                data_: unimplemented!(),
                id_: unimplemented!(),
                stats_: Ptr::<test_allocator_statistics>::null(),
            }
        }
    }
}
impl test_allocator_bool_ {
    pub fn new_1(a: Ptr<test_allocator_unsigned_int_>) -> Self {
        let __this: Value<test_allocator_bool_> = Rc::new(RefCell::new(Self {
            data_: a.with(|__s| __s.data_),
            id_: a.with(|__s| __s.id_),
            stats_: a.with(|__s| __s.stats_.clone()),
        }));
        let this: Ptr<test_allocator_bool_> = __this.as_pointer();
        if !((this.with(|__s| __s.stats_.clone())).is_null()) {
            field!(this.with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_inc());
            field!(this.with(|__s| __s.stats_.clone()), converted).with_mut(|__v| __v.prefix_inc());
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
pub fn operator_eq_5(x: Ptr<test_allocator_bool_>, y: Ptr<test_allocator_bool_>) -> () {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_allocator_bool__rebind_unsigned_int_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_allocator_bool__rebind_bool_ {}
#[derive(Record, ByteRepr, FnPtrArg, Destructor)]
#[byte_size(16)]
pub struct test_allocator_bool_ {
    #[offset(0)]
    data_: (),
    #[offset(4)]
    id_: (),
    #[offset(8)]
    #[byte_size(8)]
    stats_: Ptr<test_allocator_statistics>,
}
impl test_allocator_bool_ {
    pub fn new_2(mut data: (), mut id: (), mut stats: Ptr<test_allocator_statistics>) -> Self {
        let __this: Value<test_allocator_bool_> = Rc::new(RefCell::new(Self {
            data_: data,
            id_: id,
            stats_: (stats).clone(),
        }));
        let this: Ptr<test_allocator_bool_> = __this.as_pointer();
        if !((this.with(|__s| __s.stats_.clone())).is_null()) {
            field!(this.with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_inc());
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl std::cmp::PartialEq for test_allocator_bool_ {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_5(
                Rc::new(RefCell::new(test_allocator_bool_ {
                    data_: self.data_.clone(),
                    id_: self.id_.clone(),
                    stats_: self.stats_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(test_allocator_bool_ {
                    data_: other.data_.clone(),
                    id_: other.id_.clone(),
                    stats_: other.stats_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for test_allocator_bool_ {}
impl Default for test_allocator_bool_ {
    fn default() -> Self {
        {
            test_allocator_bool_ {
                data_: unimplemented!(),
                id_: unimplemented!(),
                stats_: Ptr::<test_allocator_statistics>::null(),
            }
        }
    }
}
pub fn operator_eq_6(
    x: Ptr<test_allocator_std_pair_const_int__int__>,
    y: Ptr<test_allocator_std_pair_const_int__int__>,
) -> () {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_allocator_std_pair_const_int__int___rebind_std_pair_const_int__int__ {}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, Destructor)]
#[byte_size(16)]
pub struct test_allocator_std_pair_const_int__int__ {
    #[offset(0)]
    data_: (),
    #[offset(4)]
    id_: (),
    #[offset(8)]
    #[byte_size(8)]
    stats_: Ptr<test_allocator_statistics>,
}
impl test_allocator_std_pair_const_int__int__ {
    pub fn new_1(mut data: (), mut id: (), mut stats: Ptr<test_allocator_statistics>) -> Self {
        let __this: Value<test_allocator_std_pair_const_int__int__> = Rc::new(RefCell::new(Self {
            data_: data,
            id_: id,
            stats_: (stats).clone(),
        }));
        let this: Ptr<test_allocator_std_pair_const_int__int__> = __this.as_pointer();
        if !((this.with(|__s| __s.stats_.clone())).is_null()) {
            field!(this.with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_inc());
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn copy_from(a: Ptr<test_allocator_std_pair_const_int__int__>) -> Self {
        let __this: Value<test_allocator_std_pair_const_int__int__> = Rc::new(RefCell::new(Self {
            data_: a.with(|__s| __s.data_),
            id_: a.with(|__s| __s.id_),
            stats_: a.with(|__s| __s.stats_.clone()),
        }));
        let this: Ptr<test_allocator_std_pair_const_int__int__> = __this.as_pointer();
        unimplemented!();
        if !((this.with(|__s| __s.stats_.clone())).is_null()) {
            field!(this.with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_inc());
            field!(this.with(|__s| __s.stats_.clone()), copied).with_mut(|__v| __v.prefix_inc());
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl std::cmp::PartialEq for test_allocator_std_pair_const_int__int__ {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_6(
                Rc::new(RefCell::new(test_allocator_std_pair_const_int__int__ {
                    data_: self.data_.clone(),
                    id_: self.id_.clone(),
                    stats_: self.stats_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(test_allocator_std_pair_const_int__int__ {
                    data_: other.data_.clone(),
                    id_: other.id_.clone(),
                    stats_: other.stats_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for test_allocator_std_pair_const_int__int__ {}
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
impl Default for test_allocator_std_pair_const_int__int__ {
    fn default() -> Self {
        {
            test_allocator_std_pair_const_int__int__ {
                data_: unimplemented!(),
                id_: unimplemented!(),
                stats_: Ptr::<test_allocator_statistics>::null(),
            }
        }
    }
}
impl test_allocator_unsigned_int_ {
    pub fn new_1(a: Ptr<test_allocator_bool_>) -> Self {
        let __this: Value<test_allocator_unsigned_int_> = Rc::new(RefCell::new(Self {
            data_: a.with(|__s| __s.data_),
            id_: a.with(|__s| __s.id_),
            stats_: a.with(|__s| __s.stats_.clone()),
        }));
        let this: Ptr<test_allocator_unsigned_int_> = __this.as_pointer();
        if !((this.with(|__s| __s.stats_.clone())).is_null()) {
            field!(this.with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_inc());
            field!(this.with(|__s| __s.stats_.clone()), converted).with_mut(|__v| __v.prefix_inc());
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, Destructor)]
#[byte_size(16)]
pub struct test_allocator_unsigned_int_ {
    #[offset(0)]
    data_: (),
    #[offset(4)]
    id_: (),
    #[offset(8)]
    #[byte_size(8)]
    stats_: Ptr<test_allocator_statistics>,
}
impl test_allocator_unsigned_int_ {
    pub fn move_from(a: Ptr<test_allocator_unsigned_int_>) -> Self {
        let __this: Value<test_allocator_unsigned_int_> = Rc::new(RefCell::new(Self {
            data_: a.with(|__s| __s.data_),
            id_: a.with(|__s| __s.id_),
            stats_: a.with(|__s| __s.stats_.clone()),
        }));
        let this: Ptr<test_allocator_unsigned_int_> = __this.as_pointer();
        if !((this.with(|__s| __s.stats_.clone())).is_null()) {
            field!(this.with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_inc());
            field!(this.with(|__s| __s.stats_.clone()), moved).with_mut(|__v| __v.prefix_inc());
        }
        unimplemented!();
        field!(a, id_).write(moved_value_1.with(|rc| *rc.borrow()));
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for test_allocator_unsigned_int_ {
    fn default() -> Self {
        {
            test_allocator_unsigned_int_ {
                data_: unimplemented!(),
                id_: unimplemented!(),
                stats_: Ptr::<test_allocator_statistics>::null(),
            }
        }
    }
}
impl test_allocator_int_ptr_ {
    pub fn new_1(a: Ptr<test_allocator_int_>) -> Self {
        let __this: Value<test_allocator_int_ptr_> = Rc::new(RefCell::new(Self {
            data_: a.with(|__s| __s.data_),
            id_: a.with(|__s| __s.id_),
            stats_: a.with(|__s| __s.stats_.clone()),
        }));
        let this: Ptr<test_allocator_int_ptr_> = __this.as_pointer();
        if !((this.with(|__s| __s.stats_.clone())).is_null()) {
            field!(this.with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_inc());
            field!(this.with(|__s| __s.stats_.clone()), converted).with_mut(|__v| __v.prefix_inc());
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, Destructor)]
#[byte_size(16)]
pub struct test_allocator_int_ptr_ {
    #[offset(0)]
    data_: (),
    #[offset(4)]
    id_: (),
    #[offset(8)]
    #[byte_size(8)]
    stats_: Ptr<test_allocator_statistics>,
}
impl test_allocator_int_ptr_ {
    pub fn copy_from(a: Ptr<test_allocator_int_ptr_>) -> Self {
        let __this: Value<test_allocator_int_ptr_> = Rc::new(RefCell::new(Self {
            data_: a.with(|__s| __s.data_),
            id_: a.with(|__s| __s.id_),
            stats_: a.with(|__s| __s.stats_.clone()),
        }));
        let this: Ptr<test_allocator_int_ptr_> = __this.as_pointer();
        unimplemented!();
        if !((this.with(|__s| __s.stats_.clone())).is_null()) {
            field!(this.with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_inc());
            field!(this.with(|__s| __s.stats_.clone()), copied).with_mut(|__v| __v.prefix_inc());
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn move_from(a: Ptr<test_allocator_int_ptr_>) -> Self {
        let __this: Value<test_allocator_int_ptr_> = Rc::new(RefCell::new(Self {
            data_: a.with(|__s| __s.data_),
            id_: a.with(|__s| __s.id_),
            stats_: a.with(|__s| __s.stats_.clone()),
        }));
        let this: Ptr<test_allocator_int_ptr_> = __this.as_pointer();
        if !((this.with(|__s| __s.stats_.clone())).is_null()) {
            field!(this.with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_inc());
            field!(this.with(|__s| __s.stats_.clone()), moved).with_mut(|__v| __v.prefix_inc());
        }
        unimplemented!();
        field!(a, id_).write(moved_value_1.with(|rc| *rc.borrow()));
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for test_allocator_int_ptr_ {
    fn clone(&self) -> Self {
        let __src: Value<test_allocator_int_ptr_> =
            Rc::new(RefCell::new(test_allocator_int_ptr_ {
                data_: self.data_.clone(),
                id_: self.id_.clone(),
                stats_: self.stats_.clone(),
            }));
        test_allocator_int_ptr_::copy_from(__src.as_pointer())
    }
}
impl Default for test_allocator_int_ptr_ {
    fn default() -> Self {
        {
            test_allocator_int_ptr_ {
                data_: unimplemented!(),
                id_: unimplemented!(),
                stats_: Ptr::<test_allocator_statistics>::null(),
            }
        }
    }
}
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(16)]
pub struct test_allocator_const_int_ptr_ {
    #[offset(0)]
    data_: (),
    #[offset(4)]
    id_: (),
    #[offset(8)]
    #[byte_size(8)]
    stats_: Ptr<test_allocator_statistics>,
}
impl test_allocator_const_int_ptr_ {}
impl Default for test_allocator_const_int_ptr_ {
    fn default() -> Self {
        {
            test_allocator_const_int_ptr_ {
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
pub fn test_7(expected_num_allocs: Option<()>) {
    let mut expected_num_allocs: () = expected_num_allocs.unwrap_or_else(|| 1_());
    let alloc_stats: Value<test_allocator_statistics> =
        Rc::new(RefCell::new(<test_allocator_statistics>::default()));
    {
        ({ test_allocator_statisticsImpl::clear(&alloc_stats.as_pointer()) });
        let mut v: () = unimplemented!();
        unimplemented!();
        let mut num_stored_allocs: () = { (*alloc_stats.borrow()).count };
        {
            let __tmp_0: Value<test_allocator_int_> = Rc::new(RefCell::new(unimplemented!()));
            let a: Ptr<test_allocator_int_> = __tmp_0.as_pointer();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        unimplemented!();
        ({ test_allocator_statisticsImpl::clear_ctor_counters(&alloc_stats.as_pointer()) });
        let mut v2: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        {
            let __tmp_1: Value<test_allocator_int_> = Rc::new(RefCell::new(unimplemented!()));
            let a1: Ptr<test_allocator_int_> = __tmp_1.as_pointer();
            unimplemented!();
            unimplemented!();
            let __tmp_2: Value<test_allocator_int_> = Rc::new(RefCell::new(unimplemented!()));
            let a2: Ptr<test_allocator_int_> = __tmp_2.as_pointer();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub fn test_8(expected_num_allocs: Option<()>) {
    let mut expected_num_allocs: () = expected_num_allocs.unwrap_or_else(|| 1_());
    let alloc_stats: Value<test_allocator_statistics> =
        Rc::new(RefCell::new(<test_allocator_statistics>::default()));
    {
        ({ test_allocator_statisticsImpl::clear(&alloc_stats.as_pointer()) });
        let mut v: () = unimplemented!();
        unimplemented!();
        let mut num_stored_allocs: () = { (*alloc_stats.borrow()).count };
        {
            let __tmp_3: Value<test_allocator_bool_> = Rc::new(RefCell::new(unimplemented!()));
            let a: Ptr<test_allocator_bool_> = __tmp_3.as_pointer();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        unimplemented!();
        ({ test_allocator_statisticsImpl::clear_ctor_counters(&alloc_stats.as_pointer()) });
        let mut v2: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        {
            let __tmp_4: Value<test_allocator_bool_> = Rc::new(RefCell::new(unimplemented!()));
            let a1: Ptr<test_allocator_bool_> = __tmp_4.as_pointer();
            unimplemented!();
            unimplemented!();
            let __tmp_5: Value<test_allocator_bool_> = Rc::new(RefCell::new(unimplemented!()));
            let a2: Ptr<test_allocator_bool_> = __tmp_5.as_pointer();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub fn test_9(expected_num_allocs: Option<()>) {
    let mut expected_num_allocs: () = expected_num_allocs.unwrap_or_else(|| 1_());
    let alloc_stats: Value<test_allocator_statistics> =
        Rc::new(RefCell::new(<test_allocator_statistics>::default()));
    {
        ({ test_allocator_statisticsImpl::clear(&alloc_stats.as_pointer()) });
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        let mut num_stored_allocs: () = { (*alloc_stats.borrow()).count };
        {
            let __tmp_6: Value<test_allocator_int_> = Rc::new(RefCell::new(unimplemented!()));
            let a: Ptr<test_allocator_int_> = __tmp_6.as_pointer();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        unimplemented!();
        ({ test_allocator_statisticsImpl::clear_ctor_counters(&alloc_stats.as_pointer()) });
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        {
            let __tmp_7: Value<test_allocator_int_> = Rc::new(RefCell::new(unimplemented!()));
            let a1: Ptr<test_allocator_int_> = __tmp_7.as_pointer();
            unimplemented!();
            unimplemented!();
            let __tmp_8: Value<test_allocator_int_> = Rc::new(RefCell::new(unimplemented!()));
            let a2: Ptr<test_allocator_int_> = __tmp_8.as_pointer();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub fn test_10(expected_num_allocs: Option<()>) {
    let mut expected_num_allocs: () = expected_num_allocs.unwrap_or_else(|| 1_());
    let alloc_stats: Value<test_allocator_statistics> =
        Rc::new(RefCell::new(<test_allocator_statistics>::default()));
    {
        ({ test_allocator_statisticsImpl::clear(&alloc_stats.as_pointer()) });
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        let mut num_stored_allocs: () = { (*alloc_stats.borrow()).count };
        {
            let __tmp_9: Value<test_allocator_int_> = Rc::new(RefCell::new(unimplemented!()));
            let a: Ptr<test_allocator_int_> = __tmp_9.as_pointer();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        unimplemented!();
        ({ test_allocator_statisticsImpl::clear_ctor_counters(&alloc_stats.as_pointer()) });
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        {
            let __tmp_10: Value<test_allocator_int_> = Rc::new(RefCell::new(unimplemented!()));
            let a1: Ptr<test_allocator_int_> = __tmp_10.as_pointer();
            unimplemented!();
            unimplemented!();
            let __tmp_11: Value<test_allocator_int_> = Rc::new(RefCell::new(unimplemented!()));
            let a2: Ptr<test_allocator_int_> = __tmp_11.as_pointer();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub fn test_11(mut expected_num_allocs: ()) {
    let alloc_stats: Value<test_allocator_statistics> =
        Rc::new(RefCell::new(<test_allocator_statistics>::default()));
    {
        ({ test_allocator_statisticsImpl::clear(&alloc_stats.as_pointer()) });
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        let mut num_stored_allocs: () = { (*alloc_stats.borrow()).count };
        {
            let __tmp_12: Value<test_allocator_int_> = Rc::new(RefCell::new(unimplemented!()));
            let a: Ptr<test_allocator_int_> = __tmp_12.as_pointer();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        unimplemented!();
        ({ test_allocator_statisticsImpl::clear_ctor_counters(&alloc_stats.as_pointer()) });
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        {
            let __tmp_13: Value<test_allocator_int_> = Rc::new(RefCell::new(unimplemented!()));
            let a1: Ptr<test_allocator_int_> = __tmp_13.as_pointer();
            unimplemented!();
            unimplemented!();
            let __tmp_14: Value<test_allocator_int_> = Rc::new(RefCell::new(unimplemented!()));
            let a2: Ptr<test_allocator_int_> = __tmp_14.as_pointer();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub fn test_12(expected_num_allocs: Option<()>) {
    let mut expected_num_allocs: () = expected_num_allocs.unwrap_or_else(|| 1_());
    let alloc_stats: Value<test_allocator_statistics> =
        Rc::new(RefCell::new(<test_allocator_statistics>::default()));
    {
        ({ test_allocator_statisticsImpl::clear(&alloc_stats.as_pointer()) });
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        let mut num_stored_allocs: () = { (*alloc_stats.borrow()).count };
        {
            let __tmp_15: Value<test_allocator_int_> = Rc::new(RefCell::new(unimplemented!()));
            let a: Ptr<test_allocator_int_> = __tmp_15.as_pointer();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        unimplemented!();
        ({ test_allocator_statisticsImpl::clear_ctor_counters(&alloc_stats.as_pointer()) });
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        {
            let __tmp_16: Value<test_allocator_int_> = Rc::new(RefCell::new(unimplemented!()));
            let a1: Ptr<test_allocator_int_> = __tmp_16.as_pointer();
            unimplemented!();
            unimplemented!();
            let __tmp_17: Value<test_allocator_int_> = Rc::new(RefCell::new(unimplemented!()));
            let a2: Ptr<test_allocator_int_> = __tmp_17.as_pointer();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub fn test_13(expected_num_allocs: Option<()>) {
    let mut expected_num_allocs: () = expected_num_allocs.unwrap_or_else(|| 1_());
    let alloc_stats: Value<test_allocator_statistics> =
        Rc::new(RefCell::new(<test_allocator_statistics>::default()));
    {
        ({ test_allocator_statisticsImpl::clear(&alloc_stats.as_pointer()) });
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        let mut num_stored_allocs: () = { (*alloc_stats.borrow()).count };
        {
            let __tmp_18: Value<test_allocator_int_> = Rc::new(RefCell::new(unimplemented!()));
            let a: Ptr<test_allocator_int_> = __tmp_18.as_pointer();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        unimplemented!();
        ({ test_allocator_statisticsImpl::clear_ctor_counters(&alloc_stats.as_pointer()) });
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        {
            let __tmp_19: Value<test_allocator_int_> = Rc::new(RefCell::new(unimplemented!()));
            let a1: Ptr<test_allocator_int_> = __tmp_19.as_pointer();
            unimplemented!();
            unimplemented!();
            let __tmp_20: Value<test_allocator_int_> = Rc::new(RefCell::new(unimplemented!()));
            let a2: Ptr<test_allocator_int_> = __tmp_20.as_pointer();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub fn test_14(expected_num_allocs: Option<()>) {
    let mut expected_num_allocs: () = expected_num_allocs.unwrap_or_else(|| 1_());
    let alloc_stats: Value<test_allocator_statistics> =
        Rc::new(RefCell::new(<test_allocator_statistics>::default()));
    {
        ({ test_allocator_statisticsImpl::clear(&alloc_stats.as_pointer()) });
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        let mut num_stored_allocs: () = { (*alloc_stats.borrow()).count };
        {
            let __tmp_21: Value<test_allocator_std_pair_const_int__int__> =
                Rc::new(RefCell::new(unimplemented!()));
            let a: Ptr<test_allocator_std_pair_const_int__int__> = __tmp_21.as_pointer();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        unimplemented!();
        ({ test_allocator_statisticsImpl::clear_ctor_counters(&alloc_stats.as_pointer()) });
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        {
            let __tmp_22: Value<test_allocator_std_pair_const_int__int__> =
                Rc::new(RefCell::new(unimplemented!()));
            let a1: Ptr<test_allocator_std_pair_const_int__int__> = __tmp_22.as_pointer();
            unimplemented!();
            unimplemented!();
            let __tmp_23: Value<test_allocator_std_pair_const_int__int__> =
                Rc::new(RefCell::new(unimplemented!()));
            let a2: Ptr<test_allocator_std_pair_const_int__int__> = __tmp_23.as_pointer();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub fn test_15(expected_num_allocs: Option<()>) {
    let mut expected_num_allocs: () = expected_num_allocs.unwrap_or_else(|| 1_());
    let alloc_stats: Value<test_allocator_statistics> =
        Rc::new(RefCell::new(<test_allocator_statistics>::default()));
    {
        ({ test_allocator_statisticsImpl::clear(&alloc_stats.as_pointer()) });
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        let mut num_stored_allocs: () = { (*alloc_stats.borrow()).count };
        {
            let __tmp_24: Value<test_allocator_std_pair_const_int__int__> =
                Rc::new(RefCell::new(unimplemented!()));
            let a: Ptr<test_allocator_std_pair_const_int__int__> = __tmp_24.as_pointer();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        unimplemented!();
        ({ test_allocator_statisticsImpl::clear_ctor_counters(&alloc_stats.as_pointer()) });
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        {
            let __tmp_25: Value<test_allocator_std_pair_const_int__int__> =
                Rc::new(RefCell::new(unimplemented!()));
            let a1: Ptr<test_allocator_std_pair_const_int__int__> = __tmp_25.as_pointer();
            unimplemented!();
            unimplemented!();
            let __tmp_26: Value<test_allocator_std_pair_const_int__int__> =
                Rc::new(RefCell::new(unimplemented!()));
            let a2: Ptr<test_allocator_std_pair_const_int__int__> = __tmp_26.as_pointer();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub fn test_16(mut expected_num_allocs: ()) {
    let alloc_stats: Value<test_allocator_statistics> =
        Rc::new(RefCell::new(<test_allocator_statistics>::default()));
    {
        ({ test_allocator_statisticsImpl::clear(&alloc_stats.as_pointer()) });
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        let mut num_stored_allocs: () = { (*alloc_stats.borrow()).count };
        {
            let __tmp_27: Value<test_allocator_int_> = Rc::new(RefCell::new(unimplemented!()));
            let a: Ptr<test_allocator_int_> = __tmp_27.as_pointer();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        unimplemented!();
        ({ test_allocator_statisticsImpl::clear_ctor_counters(&alloc_stats.as_pointer()) });
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        {
            let __tmp_28: Value<test_allocator_int_> = Rc::new(RefCell::new(unimplemented!()));
            let a1: Ptr<test_allocator_int_> = __tmp_28.as_pointer();
            unimplemented!();
            unimplemented!();
            let __tmp_29: Value<test_allocator_int_> = Rc::new(RefCell::new(unimplemented!()));
            let a2: Ptr<test_allocator_int_> = __tmp_29.as_pointer();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub fn test_17(mut expected_num_allocs: ()) {
    let alloc_stats: Value<test_allocator_statistics> =
        Rc::new(RefCell::new(<test_allocator_statistics>::default()));
    {
        ({ test_allocator_statisticsImpl::clear(&alloc_stats.as_pointer()) });
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        let mut num_stored_allocs: () = { (*alloc_stats.borrow()).count };
        {
            let __tmp_30: Value<test_allocator_int_> = Rc::new(RefCell::new(unimplemented!()));
            let a: Ptr<test_allocator_int_> = __tmp_30.as_pointer();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        unimplemented!();
        ({ test_allocator_statisticsImpl::clear_ctor_counters(&alloc_stats.as_pointer()) });
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        {
            let __tmp_31: Value<test_allocator_int_> = Rc::new(RefCell::new(unimplemented!()));
            let a1: Ptr<test_allocator_int_> = __tmp_31.as_pointer();
            unimplemented!();
            unimplemented!();
            let __tmp_32: Value<test_allocator_int_> = Rc::new(RefCell::new(unimplemented!()));
            let a2: Ptr<test_allocator_int_> = __tmp_32.as_pointer();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub fn test_18(mut expected_num_allocs: ()) {
    let alloc_stats: Value<test_allocator_statistics> =
        Rc::new(RefCell::new(<test_allocator_statistics>::default()));
    {
        ({ test_allocator_statisticsImpl::clear(&alloc_stats.as_pointer()) });
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        let mut num_stored_allocs: () = { (*alloc_stats.borrow()).count };
        {
            let __tmp_33: Value<test_allocator_std_pair_const_int__int__> =
                Rc::new(RefCell::new(unimplemented!()));
            let a: Ptr<test_allocator_std_pair_const_int__int__> = __tmp_33.as_pointer();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        unimplemented!();
        ({ test_allocator_statisticsImpl::clear_ctor_counters(&alloc_stats.as_pointer()) });
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        {
            let __tmp_34: Value<test_allocator_std_pair_const_int__int__> =
                Rc::new(RefCell::new(unimplemented!()));
            let a1: Ptr<test_allocator_std_pair_const_int__int__> = __tmp_34.as_pointer();
            unimplemented!();
            unimplemented!();
            let __tmp_35: Value<test_allocator_std_pair_const_int__int__> =
                Rc::new(RefCell::new(unimplemented!()));
            let a2: Ptr<test_allocator_std_pair_const_int__int__> = __tmp_35.as_pointer();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub fn test_19(mut expected_num_allocs: ()) {
    let alloc_stats: Value<test_allocator_statistics> =
        Rc::new(RefCell::new(<test_allocator_statistics>::default()));
    {
        ({ test_allocator_statisticsImpl::clear(&alloc_stats.as_pointer()) });
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        let mut num_stored_allocs: () = { (*alloc_stats.borrow()).count };
        {
            let __tmp_36: Value<test_allocator_std_pair_const_int__int__> =
                Rc::new(RefCell::new(unimplemented!()));
            let a: Ptr<test_allocator_std_pair_const_int__int__> = __tmp_36.as_pointer();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        unimplemented!();
        ({ test_allocator_statisticsImpl::clear_ctor_counters(&alloc_stats.as_pointer()) });
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        {
            let __tmp_37: Value<test_allocator_std_pair_const_int__int__> =
                Rc::new(RefCell::new(unimplemented!()));
            let a1: Ptr<test_allocator_std_pair_const_int__int__> = __tmp_37.as_pointer();
            unimplemented!();
            unimplemented!();
            let __tmp_38: Value<test_allocator_std_pair_const_int__int__> =
                Rc::new(RefCell::new(unimplemented!()));
            let a2: Ptr<test_allocator_std_pair_const_int__int__> = __tmp_38.as_pointer();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    {
        ({ test_7(None) });
        ({ test_8(None) });
        ({ test_9(None) });
        ({ test_10(None) });
        let mut stored_allocators: () = 2_();
        ({ test_11(stored_allocators) });
    }
    {
        ({ test_12(None) });
        ({ test_13(None) });
        ({ test_14(None) });
        ({ test_15(None) });
    }
    {
        let mut stored_allocators: () = 2_();
        ({ test_16(stored_allocators) });
        ({ test_17(stored_allocators) });
        ({ test_18(stored_allocators) });
        ({ test_19(stored_allocators) });
    }
    return 0_();
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
pub trait test_allocator_bool_Impl {
    fn destructor(&self);
    fn address_11(&self, x: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn address_12(&self, x: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn allocate(&self, mut n: (), mut _a1: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<()>, mut s: ()) {
        unimplemented!()
    }
    fn max_size(&self) -> () {
        unimplemented!()
    }
    fn destroy(&self, mut p: Ptr<()>) {
        unimplemented!()
    }
    fn get_data(&self) -> ();
    fn get_id(&self) -> ();
}
impl test_allocator_bool_Impl for Ptr<test_allocator_bool_> {
    fn destructor(&self) {
        unimplemented!();
        unimplemented!();
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_dec());
        }
        field!((*self), data_).write(destructed_value_0.with(|rc| *rc.borrow()));
        field!((*self), id_).write(destructed_value_0.with(|rc| *rc.borrow()));
    }
    fn get_data(&self) -> () {
        return (*self).with(|__s| __s.data_);
    }
    fn get_id(&self) -> () {
        return (*self).with(|__s| __s.id_);
    }
}
pub trait test_allocator_const_int_ptr_Impl {
    fn destructor(&self) {
        unimplemented!()
    }
    fn address_11(&self, x: ()) -> Ptr<Ptr<()>> {
        unimplemented!()
    }
    fn address_12(&self, x: ()) -> Ptr<Ptr<()>> {
        unimplemented!()
    }
    fn allocate(&self, mut n: (), mut _a1: ()) -> Ptr<Ptr<()>> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<Ptr<()>>, mut s: ()) {
        unimplemented!()
    }
    fn max_size(&self) -> () {
        unimplemented!()
    }
    fn destroy(&self, mut p: Ptr<Ptr<()>>) {
        unimplemented!()
    }
    fn get_data(&self) -> () {
        unimplemented!()
    }
    fn get_id(&self) -> () {
        unimplemented!()
    }
}
impl test_allocator_const_int_ptr_Impl for Ptr<test_allocator_const_int_ptr_> {}
pub trait test_allocator_int_Impl {
    fn destructor(&self);
    fn address_11(&self, x: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn address_12(&self, x: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn allocate(&self, mut n: (), _a1: Option<()>) -> Ptr<()> {
        unimplemented!()
    }
    fn deallocate(&self, p: Ptr<()>, s: ());
    fn max_size(&self) -> () {
        unimplemented!()
    }
    fn destroy(&self, p: Ptr<()>);
    fn get_data(&self) -> ();
    fn get_id(&self) -> ();
}
impl test_allocator_int_Impl for Ptr<test_allocator_int_> {
    fn destructor(&self) {
        unimplemented!();
        unimplemented!();
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_dec());
        }
        field!((*self), data_).write(destructed_value_0.with(|rc| *rc.borrow()));
        field!((*self), id_).write(destructed_value_0.with(|rc| *rc.borrow()));
    }
    fn deallocate(&self, mut p: Ptr<()>, mut s: ()) {
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
    fn destroy(&self, mut p: Ptr<()>) {
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), destroy_count)
                .with_mut(|__v| __v.prefix_inc());
        };
    }
    fn get_data(&self) -> () {
        return (*self).with(|__s| __s.data_);
    }
    fn get_id(&self) -> () {
        return (*self).with(|__s| __s.id_);
    }
}
pub trait test_allocator_int_ptr_Impl {
    fn destructor(&self);
    fn address_11(&self, x: ()) -> Ptr<Ptr<()>> {
        unimplemented!()
    }
    fn address_12(&self, x: ()) -> Ptr<Ptr<()>> {
        unimplemented!()
    }
    fn allocate(&self, mut n: (), mut _a1: ()) -> Ptr<Ptr<()>> {
        unimplemented!()
    }
    fn deallocate(&self, p: Ptr<Ptr<()>>, s: ());
    fn max_size(&self) -> () {
        unimplemented!()
    }
    fn destroy(&self, p: Ptr<Ptr<()>>);
    fn get_data(&self) -> () {
        unimplemented!()
    }
    fn get_id(&self) -> () {
        unimplemented!()
    }
}
impl test_allocator_int_ptr_Impl for Ptr<test_allocator_int_ptr_> {
    fn destructor(&self) {
        unimplemented!();
        unimplemented!();
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_dec());
        }
        field!((*self), data_).write(destructed_value_0.with(|rc| *rc.borrow()));
        field!((*self), id_).write(destructed_value_0.with(|rc| *rc.borrow()));
    }
    fn deallocate(&self, mut p: Ptr<Ptr<()>>, mut s: ()) {
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
    fn destroy(&self, mut p: Ptr<Ptr<()>>) {
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), destroy_count)
                .with_mut(|__v| __v.prefix_inc());
        };
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
pub trait test_allocator_std_pair_const_int__int__Impl {
    fn destructor(&self);
    fn address_11(&self, x: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn address_12(&self, x: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn allocate(&self, mut n: (), mut _a1: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<()>, mut s: ()) {
        unimplemented!()
    }
    fn max_size(&self) -> () {
        unimplemented!()
    }
    fn destroy(&self, mut p: Ptr<()>) {
        unimplemented!()
    }
    fn get_data(&self) -> ();
    fn get_id(&self) -> ();
}
impl test_allocator_std_pair_const_int__int__Impl
    for Ptr<test_allocator_std_pair_const_int__int__>
{
    fn destructor(&self) {
        unimplemented!();
        unimplemented!();
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_dec());
        }
        field!((*self), data_).write(destructed_value_0.with(|rc| *rc.borrow()));
        field!((*self), id_).write(destructed_value_0.with(|rc| *rc.borrow()));
    }
    fn get_data(&self) -> () {
        return (*self).with(|__s| __s.data_);
    }
    fn get_id(&self) -> () {
        return (*self).with(|__s| __s.id_);
    }
}
pub trait test_allocator_unsigned_int_Impl {
    fn destructor(&self);
    fn address_11(&self, x: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn address_12(&self, x: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn allocate(&self, mut n: (), mut _a1: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn deallocate(&self, p: Ptr<()>, s: ());
    fn max_size(&self) -> () {
        unimplemented!()
    }
    fn destroy(&self, mut p: Ptr<()>) {
        unimplemented!()
    }
    fn get_data(&self) -> () {
        unimplemented!()
    }
    fn get_id(&self) -> () {
        unimplemented!()
    }
}
impl test_allocator_unsigned_int_Impl for Ptr<test_allocator_unsigned_int_> {
    fn destructor(&self) {
        unimplemented!();
        unimplemented!();
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_dec());
        }
        field!((*self), data_).write(destructed_value_0.with(|rc| *rc.borrow()));
        field!((*self), id_).write(destructed_value_0.with(|rc| *rc.borrow()));
    }
    fn deallocate(&self, mut p: Ptr<()>, mut s: ()) {
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
