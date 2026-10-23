extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type anon_0 = ();
pub const anon_0_value: anon_0 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_map_int__const_long__std_less_int___std_allocator_std_pair_const_int__const_long_____std_map_int__const_long__std_less_int___std_allocator_std_pair_const_int__const_long____
{}
pub type anon_1 = ();
pub const anon_1_value: anon_1 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_map_int__const_long__std_greater_int___std_allocator_std_pair_const_int__const_long_____std_map_int__const_long__std_greater_int___std_allocator_std_pair_const_int__const_long____
{}
pub type anon_2 = ();
pub const anon_2_value: anon_2 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_map_int__const_long__std_greater_int___test_allocator_std_pair_const_int__const_long_____std_map_int__const_long__std_greater_int___test_allocator_std_pair_const_int__const_long____
{}
pub type anon_3 = ();
pub const anon_3_value: anon_3 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_map_int__long__std_less_int___std_allocator_std_pair_const_int__long_____std_map_int__long__std_less_int___std_allocator_std_pair_const_int__long____
{}
pub type anon_4 = ();
pub const anon_4_value: anon_4 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_map_int__long__std_greater_int___std_allocator_std_pair_const_int__long_____std_map_int__long__std_greater_int___std_allocator_std_pair_const_int__long____
{}
pub type anon_5 = ();
pub const anon_5_value: anon_5 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_map_int__long__std_greater_int___test_allocator_std_pair_const_int__long_____std_map_int__long__std_greater_int___test_allocator_std_pair_const_int__long____
{}
pub type anon_6 = ();
pub const anon_6_value: anon_6 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_map_int__long__std_less_int___test_allocator_std_pair_const_int__long_____std_map_int__long__std_less_int___test_allocator_std_pair_const_int__long____
{}
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
    pub static destructed_value_7: Value<()> = Rc::new(RefCell::new(-1_()));
);
thread_local!(
    pub static moved_value_8: Value<()> = Rc::new(RefCell::new(2147483647_()));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_alloc_base {}
pub fn operator_eq_9(x: Ptr<test_allocator_void_>, y: Ptr<test_allocator_void_>) -> () {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
pub fn operator_ne_10(x: Ptr<test_allocator_void_>, y: Ptr<test_allocator_void_>) -> () {
    return !({
        let _x: Ptr<test_allocator_void_> = (x).clone();
        let _y: Ptr<test_allocator_void_> = (y).clone();
        operator_eq_9(_x, _y)
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
            operator_eq_9(
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
impl test_allocator_std_pair_const_int__const_long__ {
    pub fn new_1(
        a: Ptr<test_allocator_std___tree_node_std___value_type_int__const_long___void_ptr__>,
    ) -> Self {
        let __this: Value<test_allocator_std_pair_const_int__const_long__> =
            Rc::new(RefCell::new(Self {
                data_: a.with(|__s| __s.data_),
                id_: a.with(|__s| __s.id_),
                stats_: a.with(|__s| __s.stats_.clone()),
            }));
        let this: Ptr<test_allocator_std_pair_const_int__const_long__> = __this.as_pointer();
        if !((this.with(|__s| __s.stats_.clone())).is_null()) {
            field!(this.with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_inc());
            field!(this.with(|__s| __s.stats_.clone()), converted).with_mut(|__v| __v.prefix_inc());
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_allocator_std_pair_const_int__const_long___rebind_std___tree_node_std___value_type_int__const_long___void_ptr__
{}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_allocator_std_pair_const_int__const_long___rebind_std_pair_const_int__const_long__
{}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, Destructor)]
#[byte_size(16)]
pub struct test_allocator_std_pair_const_int__const_long__ {
    #[offset(0)]
    data_: (),
    #[offset(4)]
    id_: (),
    #[offset(8)]
    #[byte_size(8)]
    stats_: Ptr<test_allocator_statistics>,
}
impl test_allocator_std_pair_const_int__const_long__ {
    pub fn new_2(mut data: (), mut id: ()) -> Self {
        Self {
            data_: data,
            id_: id,
            stats_: Ptr::<test_allocator_statistics>::null(),
        }
    }
    pub fn copy_from(a: Ptr<test_allocator_std_pair_const_int__const_long__>) -> Self {
        let __this: Value<test_allocator_std_pair_const_int__const_long__> =
            Rc::new(RefCell::new(Self {
                data_: a.with(|__s| __s.data_),
                id_: a.with(|__s| __s.id_),
                stats_: a.with(|__s| __s.stats_.clone()),
            }));
        let this: Ptr<test_allocator_std_pair_const_int__const_long__> = __this.as_pointer();
        unimplemented!();
        if !((this.with(|__s| __s.stats_.clone())).is_null()) {
            field!(this.with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_inc());
            field!(this.with(|__s| __s.stats_.clone()), copied).with_mut(|__v| __v.prefix_inc());
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for test_allocator_std_pair_const_int__const_long__ {
    fn clone(&self) -> Self {
        let __src: Value<test_allocator_std_pair_const_int__const_long__> = Rc::new(RefCell::new(
            test_allocator_std_pair_const_int__const_long__ {
                data_: self.data_.clone(),
                id_: self.id_.clone(),
                stats_: self.stats_.clone(),
            },
        ));
        test_allocator_std_pair_const_int__const_long__::copy_from(__src.as_pointer())
    }
}
impl Default for test_allocator_std_pair_const_int__const_long__ {
    fn default() -> Self {
        {
            test_allocator_std_pair_const_int__const_long__ {
                data_: unimplemented!(),
                id_: unimplemented!(),
                stats_: Ptr::<test_allocator_statistics>::null(),
            }
        }
    }
}
impl test_allocator_std___tree_node_std___value_type_int__const_long___void_ptr__ {
    pub fn new_1(a: Ptr<test_allocator_std_pair_const_int__const_long__>) -> Self {
        let __this: Value<
            test_allocator_std___tree_node_std___value_type_int__const_long___void_ptr__,
        > = Rc::new(RefCell::new(Self {
            data_: a.with(|__s| __s.data_),
            id_: a.with(|__s| __s.id_),
            stats_: a.with(|__s| __s.stats_.clone()),
        }));
        let this: Ptr<
            test_allocator_std___tree_node_std___value_type_int__const_long___void_ptr__,
        > = __this.as_pointer();
        if !((this.with(|__s| __s.stats_.clone())).is_null()) {
            field!(this.with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_inc());
            field!(this.with(|__s| __s.stats_.clone()), converted).with_mut(|__v| __v.prefix_inc());
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_allocator_std___tree_node_std___value_type_int__const_long___void_ptr___rebind_std___tree_node_base_void_ptr__
{}
#[derive(Record, ByteRepr, FnPtrArg, Destructor)]
#[byte_size(16)]
pub struct test_allocator_std___tree_node_std___value_type_int__const_long___void_ptr__ {
    #[offset(0)]
    data_: (),
    #[offset(4)]
    id_: (),
    #[offset(8)]
    #[byte_size(8)]
    stats_: Ptr<test_allocator_statistics>,
}
impl test_allocator_std___tree_node_std___value_type_int__const_long___void_ptr__ {}
impl Default for test_allocator_std___tree_node_std___value_type_int__const_long___void_ptr__ {
    fn default() -> Self {
        {
            test_allocator_std___tree_node_std___value_type_int__const_long___void_ptr__ {
                data_: unimplemented!(),
                id_: unimplemented!(),
                stats_: Ptr::<test_allocator_statistics>::null(),
            }
        }
    }
}
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(16)]
pub struct test_allocator_std___tree_node_base_void_ptr__ {
    #[offset(0)]
    data_: (),
    #[offset(4)]
    id_: (),
    #[offset(8)]
    #[byte_size(8)]
    stats_: Ptr<test_allocator_statistics>,
}
impl test_allocator_std___tree_node_base_void_ptr__ {}
impl Default for test_allocator_std___tree_node_base_void_ptr__ {
    fn default() -> Self {
        {
            test_allocator_std___tree_node_base_void_ptr__ {
                data_: unimplemented!(),
                id_: unimplemented!(),
                stats_: Ptr::<test_allocator_statistics>::null(),
            }
        }
    }
}
impl test_allocator_std_pair_const_int__long__ {
    pub fn new_1(
        a: Ptr<test_allocator_std___tree_node_std___value_type_int__long___void_ptr__>,
    ) -> Self {
        let __this: Value<test_allocator_std_pair_const_int__long__> =
            Rc::new(RefCell::new(Self {
                data_: a.with(|__s| __s.data_),
                id_: a.with(|__s| __s.id_),
                stats_: a.with(|__s| __s.stats_.clone()),
            }));
        let this: Ptr<test_allocator_std_pair_const_int__long__> = __this.as_pointer();
        if !((this.with(|__s| __s.stats_.clone())).is_null()) {
            field!(this.with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_inc());
            field!(this.with(|__s| __s.stats_.clone()), converted).with_mut(|__v| __v.prefix_inc());
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_allocator_std_pair_const_int__long___rebind_std___tree_node_std___value_type_int__long___void_ptr__
{}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_allocator_std_pair_const_int__long___rebind_std_pair_const_int__long__ {}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, Destructor)]
#[byte_size(16)]
pub struct test_allocator_std_pair_const_int__long__ {
    #[offset(0)]
    data_: (),
    #[offset(4)]
    id_: (),
    #[offset(8)]
    #[byte_size(8)]
    stats_: Ptr<test_allocator_statistics>,
}
impl test_allocator_std_pair_const_int__long__ {
    pub fn new_2(mut data: (), mut id: ()) -> Self {
        Self {
            data_: data,
            id_: id,
            stats_: Ptr::<test_allocator_statistics>::null(),
        }
    }
    pub fn copy_from(a: Ptr<test_allocator_std_pair_const_int__long__>) -> Self {
        let __this: Value<test_allocator_std_pair_const_int__long__> =
            Rc::new(RefCell::new(Self {
                data_: a.with(|__s| __s.data_),
                id_: a.with(|__s| __s.id_),
                stats_: a.with(|__s| __s.stats_.clone()),
            }));
        let this: Ptr<test_allocator_std_pair_const_int__long__> = __this.as_pointer();
        unimplemented!();
        if !((this.with(|__s| __s.stats_.clone())).is_null()) {
            field!(this.with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_inc());
            field!(this.with(|__s| __s.stats_.clone()), copied).with_mut(|__v| __v.prefix_inc());
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for test_allocator_std_pair_const_int__long__ {
    fn clone(&self) -> Self {
        let __src: Value<test_allocator_std_pair_const_int__long__> =
            Rc::new(RefCell::new(test_allocator_std_pair_const_int__long__ {
                data_: self.data_.clone(),
                id_: self.id_.clone(),
                stats_: self.stats_.clone(),
            }));
        test_allocator_std_pair_const_int__long__::copy_from(__src.as_pointer())
    }
}
impl Default for test_allocator_std_pair_const_int__long__ {
    fn default() -> Self {
        {
            test_allocator_std_pair_const_int__long__ {
                data_: unimplemented!(),
                id_: unimplemented!(),
                stats_: Ptr::<test_allocator_statistics>::null(),
            }
        }
    }
}
impl test_allocator_std___tree_node_std___value_type_int__long___void_ptr__ {
    pub fn new_1(a: Ptr<test_allocator_std_pair_const_int__long__>) -> Self {
        let __this: Value<test_allocator_std___tree_node_std___value_type_int__long___void_ptr__> =
            Rc::new(RefCell::new(Self {
                data_: a.with(|__s| __s.data_),
                id_: a.with(|__s| __s.id_),
                stats_: a.with(|__s| __s.stats_.clone()),
            }));
        let this: Ptr<test_allocator_std___tree_node_std___value_type_int__long___void_ptr__> =
            __this.as_pointer();
        if !((this.with(|__s| __s.stats_.clone())).is_null()) {
            field!(this.with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_inc());
            field!(this.with(|__s| __s.stats_.clone()), converted).with_mut(|__v| __v.prefix_inc());
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_allocator_std___tree_node_std___value_type_int__long___void_ptr___rebind_std___tree_node_base_void_ptr__
{}
#[derive(Record, ByteRepr, FnPtrArg, Destructor)]
#[byte_size(16)]
pub struct test_allocator_std___tree_node_std___value_type_int__long___void_ptr__ {
    #[offset(0)]
    data_: (),
    #[offset(4)]
    id_: (),
    #[offset(8)]
    #[byte_size(8)]
    stats_: Ptr<test_allocator_statistics>,
}
impl test_allocator_std___tree_node_std___value_type_int__long___void_ptr__ {}
impl Default for test_allocator_std___tree_node_std___value_type_int__long___void_ptr__ {
    fn default() -> Self {
        {
            test_allocator_std___tree_node_std___value_type_int__long___void_ptr__ {
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
pub fn test_11() -> () {
    {
        let mut arr: [(); 5] = [
            {
                let __tmp_0: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_1: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_const_int__const_long_::new({ __tmp_0.as_pointer() }, {
                    __tmp_1.as_pointer()
                })
            },
            {
                let __tmp_2: Value<()> = Rc::new(RefCell::new(2_()));
                let __tmp_3: Value<()> = Rc::new(RefCell::new(2_()));
                std_pair_const_int__const_long_::new({ __tmp_2.as_pointer() }, {
                    __tmp_3.as_pointer()
                })
            },
            {
                let __tmp_4: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_5: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_const_int__const_long_::new({ __tmp_4.as_pointer() }, {
                    __tmp_5.as_pointer()
                })
            },
            {
                let __tmp_6: Value<()> = Rc::new(RefCell::new(2147483647_()));
                let __tmp_7: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_const_int__const_long_::new({ __tmp_6.as_pointer() }, {
                    __tmp_7.as_pointer()
                })
            },
            {
                let __tmp_8: Value<()> = Rc::new(RefCell::new(3_()));
                let __tmp_9: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_const_int__const_long_::new({ __tmp_8.as_pointer() }, {
                    __tmp_9.as_pointer()
                })
            },
        ];
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        let mut expected_m: [(); 4] = [
            {
                let __tmp_10: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_11: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_const_int__const_long_::new({ __tmp_10.as_pointer() }, {
                    __tmp_11.as_pointer()
                })
            },
            {
                let __tmp_12: Value<()> = Rc::new(RefCell::new(2_()));
                let __tmp_13: Value<()> = Rc::new(RefCell::new(2_()));
                std_pair_const_int__const_long_::new({ __tmp_12.as_pointer() }, {
                    __tmp_13.as_pointer()
                })
            },
            {
                let __tmp_14: Value<()> = Rc::new(RefCell::new(3_()));
                let __tmp_15: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_const_int__const_long_::new({ __tmp_14.as_pointer() }, {
                    __tmp_15.as_pointer()
                })
            },
            {
                let __tmp_16: Value<()> = Rc::new(RefCell::new(2147483647_()));
                let __tmp_17: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_const_int__const_long_::new({ __tmp_16.as_pointer() }, {
                    __tmp_17.as_pointer()
                })
            },
        ];
        unimplemented!();
    }
    {
        let mut arr: [(); 5] = [
            {
                let __tmp_18: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_19: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_const_int__const_long_::new({ __tmp_18.as_pointer() }, {
                    __tmp_19.as_pointer()
                })
            },
            {
                let __tmp_20: Value<()> = Rc::new(RefCell::new(2_()));
                let __tmp_21: Value<()> = Rc::new(RefCell::new(2_()));
                std_pair_const_int__const_long_::new({ __tmp_20.as_pointer() }, {
                    __tmp_21.as_pointer()
                })
            },
            {
                let __tmp_22: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_23: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_const_int__const_long_::new({ __tmp_22.as_pointer() }, {
                    __tmp_23.as_pointer()
                })
            },
            {
                let __tmp_24: Value<()> = Rc::new(RefCell::new(2147483647_()));
                let __tmp_25: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_const_int__const_long_::new({ __tmp_24.as_pointer() }, {
                    __tmp_25.as_pointer()
                })
            },
            {
                let __tmp_26: Value<()> = Rc::new(RefCell::new(3_()));
                let __tmp_27: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_const_int__const_long_::new({ __tmp_26.as_pointer() }, {
                    __tmp_27.as_pointer()
                })
            },
        ];
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        let mut expected_m: [(); 4] = [
            {
                let __tmp_28: Value<()> = Rc::new(RefCell::new(2147483647_()));
                let __tmp_29: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_const_int__const_long_::new({ __tmp_28.as_pointer() }, {
                    __tmp_29.as_pointer()
                })
            },
            {
                let __tmp_30: Value<()> = Rc::new(RefCell::new(3_()));
                let __tmp_31: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_const_int__const_long_::new({ __tmp_30.as_pointer() }, {
                    __tmp_31.as_pointer()
                })
            },
            {
                let __tmp_32: Value<()> = Rc::new(RefCell::new(2_()));
                let __tmp_33: Value<()> = Rc::new(RefCell::new(2_()));
                std_pair_const_int__const_long_::new({ __tmp_32.as_pointer() }, {
                    __tmp_33.as_pointer()
                })
            },
            {
                let __tmp_34: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_35: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_const_int__const_long_::new({ __tmp_34.as_pointer() }, {
                    __tmp_35.as_pointer()
                })
            },
        ];
        unimplemented!();
    }
    {
        let mut arr: [(); 5] = [
            {
                let __tmp_36: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_37: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_const_int__const_long_::new({ __tmp_36.as_pointer() }, {
                    __tmp_37.as_pointer()
                })
            },
            {
                let __tmp_38: Value<()> = Rc::new(RefCell::new(2_()));
                let __tmp_39: Value<()> = Rc::new(RefCell::new(2_()));
                std_pair_const_int__const_long_::new({ __tmp_38.as_pointer() }, {
                    __tmp_39.as_pointer()
                })
            },
            {
                let __tmp_40: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_41: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_const_int__const_long_::new({ __tmp_40.as_pointer() }, {
                    __tmp_41.as_pointer()
                })
            },
            {
                let __tmp_42: Value<()> = Rc::new(RefCell::new(2147483647_()));
                let __tmp_43: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_const_int__const_long_::new({ __tmp_42.as_pointer() }, {
                    __tmp_43.as_pointer()
                })
            },
            {
                let __tmp_44: Value<()> = Rc::new(RefCell::new(3_()));
                let __tmp_45: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_const_int__const_long_::new({ __tmp_44.as_pointer() }, {
                    __tmp_45.as_pointer()
                })
            },
        ];
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        let mut expected_m: [(); 4] = [
            {
                let __tmp_46: Value<()> = Rc::new(RefCell::new(2147483647_()));
                let __tmp_47: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_const_int__const_long_::new({ __tmp_46.as_pointer() }, {
                    __tmp_47.as_pointer()
                })
            },
            {
                let __tmp_48: Value<()> = Rc::new(RefCell::new(3_()));
                let __tmp_49: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_const_int__const_long_::new({ __tmp_48.as_pointer() }, {
                    __tmp_49.as_pointer()
                })
            },
            {
                let __tmp_50: Value<()> = Rc::new(RefCell::new(2_()));
                let __tmp_51: Value<()> = Rc::new(RefCell::new(2_()));
                std_pair_const_int__const_long_::new({ __tmp_50.as_pointer() }, {
                    __tmp_51.as_pointer()
                })
            },
            {
                let __tmp_52: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_53: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_const_int__const_long_::new({ __tmp_52.as_pointer() }, {
                    __tmp_53.as_pointer()
                })
            },
        ];
        unimplemented!();
        unimplemented!();
    }
    {
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        let mut expected_m: [(); 4] = [
            {
                let __tmp_54: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_55: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_const_int__long_::new({ __tmp_54.as_pointer() }, { __tmp_55.as_pointer() })
            },
            {
                let __tmp_56: Value<()> = Rc::new(RefCell::new(2_()));
                let __tmp_57: Value<()> = Rc::new(RefCell::new(2_()));
                std_pair_const_int__long_::new({ __tmp_56.as_pointer() }, { __tmp_57.as_pointer() })
            },
            {
                let __tmp_58: Value<()> = Rc::new(RefCell::new(3_()));
                let __tmp_59: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_const_int__long_::new({ __tmp_58.as_pointer() }, { __tmp_59.as_pointer() })
            },
            {
                let __tmp_60: Value<()> = Rc::new(RefCell::new(2147483647_()));
                let __tmp_61: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_const_int__long_::new({ __tmp_60.as_pointer() }, { __tmp_61.as_pointer() })
            },
        ];
        unimplemented!();
    }
    {
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        let mut expected_m: [(); 4] = [
            {
                let __tmp_62: Value<()> = Rc::new(RefCell::new(2147483647_()));
                let __tmp_63: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_const_int__long_::new({ __tmp_62.as_pointer() }, { __tmp_63.as_pointer() })
            },
            {
                let __tmp_64: Value<()> = Rc::new(RefCell::new(3_()));
                let __tmp_65: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_const_int__long_::new({ __tmp_64.as_pointer() }, { __tmp_65.as_pointer() })
            },
            {
                let __tmp_66: Value<()> = Rc::new(RefCell::new(2_()));
                let __tmp_67: Value<()> = Rc::new(RefCell::new(2_()));
                std_pair_const_int__long_::new({ __tmp_66.as_pointer() }, { __tmp_67.as_pointer() })
            },
            {
                let __tmp_68: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_69: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_const_int__long_::new({ __tmp_68.as_pointer() }, { __tmp_69.as_pointer() })
            },
        ];
        unimplemented!();
    }
    {
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        let mut expected_m: [(); 4] = [
            {
                let __tmp_70: Value<()> = Rc::new(RefCell::new(2147483647_()));
                let __tmp_71: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_const_int__long_::new({ __tmp_70.as_pointer() }, { __tmp_71.as_pointer() })
            },
            {
                let __tmp_72: Value<()> = Rc::new(RefCell::new(3_()));
                let __tmp_73: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_const_int__long_::new({ __tmp_72.as_pointer() }, { __tmp_73.as_pointer() })
            },
            {
                let __tmp_74: Value<()> = Rc::new(RefCell::new(2_()));
                let __tmp_75: Value<()> = Rc::new(RefCell::new(2_()));
                std_pair_const_int__long_::new({ __tmp_74.as_pointer() }, { __tmp_75.as_pointer() })
            },
            {
                let __tmp_76: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_77: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_const_int__long_::new({ __tmp_76.as_pointer() }, { __tmp_77.as_pointer() })
            },
        ];
        unimplemented!();
        unimplemented!();
    }
    {
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        let mut expected_m: [(); 4] = [
            {
                let __tmp_78: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_79: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_const_int__long_::new({ __tmp_78.as_pointer() }, { __tmp_79.as_pointer() })
            },
            {
                let __tmp_80: Value<()> = Rc::new(RefCell::new(2_()));
                let __tmp_81: Value<()> = Rc::new(RefCell::new(2_()));
                std_pair_const_int__long_::new({ __tmp_80.as_pointer() }, { __tmp_81.as_pointer() })
            },
            {
                let __tmp_82: Value<()> = Rc::new(RefCell::new(3_()));
                let __tmp_83: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_const_int__long_::new({ __tmp_82.as_pointer() }, { __tmp_83.as_pointer() })
            },
            {
                let __tmp_84: Value<()> = Rc::new(RefCell::new(2147483647_()));
                let __tmp_85: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_const_int__long_::new({ __tmp_84.as_pointer() }, { __tmp_85.as_pointer() })
            },
        ];
        unimplemented!();
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
    ({ test_11() });
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
pub trait test_allocator_std___tree_node_base_void_ptr__Impl {
    fn destructor(&self) {
        unimplemented!()
    }
    fn address_11(&self, x: ()) -> Ptr<std___tree_node_base_void_ptr_> {
        unimplemented!()
    }
    fn address_12(&self, x: ()) -> Ptr<std___tree_node_base_void_ptr_> {
        unimplemented!()
    }
    fn allocate(&self, mut n: (), mut _a1: ()) -> Ptr<std___tree_node_base_void_ptr_> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<std___tree_node_base_void_ptr_>, mut s: ()) {
        unimplemented!()
    }
    fn max_size(&self) -> () {
        unimplemented!()
    }
    fn destroy(&self, mut p: Ptr<std___tree_node_base_void_ptr_>) {
        unimplemented!()
    }
    fn get_data(&self) -> () {
        unimplemented!()
    }
    fn get_id(&self) -> () {
        unimplemented!()
    }
}
impl test_allocator_std___tree_node_base_void_ptr__Impl
    for Ptr<test_allocator_std___tree_node_base_void_ptr__>
{
}
pub trait test_allocator_std___tree_node_std___value_type_int__const_long___void_ptr__Impl {
    fn destructor(&self);
    fn address_11(
        &self,
        x: (),
    ) -> Ptr<std___tree_node_std___value_type_int__const_long___void_ptr_> {
        unimplemented!()
    }
    fn address_12(
        &self,
        x: (),
    ) -> Ptr<std___tree_node_std___value_type_int__const_long___void_ptr_> {
        unimplemented!()
    }
    fn allocate(
        &self,
        n: (),
        _a1: Option<()>,
    ) -> Ptr<std___tree_node_std___value_type_int__const_long___void_ptr_>;
    fn deallocate(
        &self,
        p: Ptr<std___tree_node_std___value_type_int__const_long___void_ptr_>,
        s: (),
    );
    fn max_size(&self) -> () {
        unimplemented!()
    }
    fn destroy(&self, mut p: Ptr<std___tree_node_std___value_type_int__const_long___void_ptr_>) {
        unimplemented!()
    }
    fn get_data(&self) -> () {
        unimplemented!()
    }
    fn get_id(&self) -> () {
        unimplemented!()
    }
}
impl test_allocator_std___tree_node_std___value_type_int__const_long___void_ptr__Impl
    for Ptr<test_allocator_std___tree_node_std___value_type_int__const_long___void_ptr__>
{
    fn destructor(&self) {
        unimplemented!();
        unimplemented!();
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_dec());
        }
        field!((*self), data_).write(destructed_value_7.with(|rc| *rc.borrow()));
        field!((*self), id_).write(destructed_value_7.with(|rc| *rc.borrow()));
    }
    fn allocate(
        &self,
        mut n: (),
        _a1: Option<()>,
    ) -> Ptr<std___tree_node_std___value_type_int__const_long___void_ptr_> {
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
    fn deallocate(
        &self,
        mut p: Ptr<std___tree_node_std___value_type_int__const_long___void_ptr_>,
        mut s: (),
    ) {
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
pub trait test_allocator_std___tree_node_std___value_type_int__long___void_ptr__Impl {
    fn destructor(&self);
    fn address_11(&self, x: ()) -> Ptr<std___tree_node_std___value_type_int__long___void_ptr_> {
        unimplemented!()
    }
    fn address_12(&self, x: ()) -> Ptr<std___tree_node_std___value_type_int__long___void_ptr_> {
        unimplemented!()
    }
    fn allocate(
        &self,
        n: (),
        _a1: Option<()>,
    ) -> Ptr<std___tree_node_std___value_type_int__long___void_ptr_>;
    fn deallocate(&self, p: Ptr<std___tree_node_std___value_type_int__long___void_ptr_>, s: ());
    fn max_size(&self) -> () {
        unimplemented!()
    }
    fn destroy(&self, mut p: Ptr<std___tree_node_std___value_type_int__long___void_ptr_>) {
        unimplemented!()
    }
    fn get_data(&self) -> () {
        unimplemented!()
    }
    fn get_id(&self) -> () {
        unimplemented!()
    }
}
impl test_allocator_std___tree_node_std___value_type_int__long___void_ptr__Impl
    for Ptr<test_allocator_std___tree_node_std___value_type_int__long___void_ptr__>
{
    fn destructor(&self) {
        unimplemented!();
        unimplemented!();
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_dec());
        }
        field!((*self), data_).write(destructed_value_7.with(|rc| *rc.borrow()));
        field!((*self), id_).write(destructed_value_7.with(|rc| *rc.borrow()));
    }
    fn allocate(
        &self,
        mut n: (),
        _a1: Option<()>,
    ) -> Ptr<std___tree_node_std___value_type_int__long___void_ptr_> {
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
    fn deallocate(
        &self,
        mut p: Ptr<std___tree_node_std___value_type_int__long___void_ptr_>,
        mut s: (),
    ) {
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
pub trait test_allocator_std_pair_const_int__const_long__Impl {
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
    fn deallocate(&self, mut p: Ptr<()>, mut s: ()) {
        unimplemented!()
    }
    fn max_size(&self) -> () {
        unimplemented!()
    }
    fn destroy(&self, mut p: Ptr<()>) {
        unimplemented!()
    }
    fn get_data(&self) -> () {
        unimplemented!()
    }
    fn get_id(&self) -> ();
}
impl test_allocator_std_pair_const_int__const_long__Impl
    for Ptr<test_allocator_std_pair_const_int__const_long__>
{
    fn destructor(&self) {
        unimplemented!();
        unimplemented!();
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_dec());
        }
        field!((*self), data_).write(destructed_value_7.with(|rc| *rc.borrow()));
        field!((*self), id_).write(destructed_value_7.with(|rc| *rc.borrow()));
    }
    fn get_id(&self) -> () {
        return (*self).with(|__s| __s.id_);
    }
}
pub trait test_allocator_std_pair_const_int__long__Impl {
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
    fn deallocate(&self, mut p: Ptr<()>, mut s: ()) {
        unimplemented!()
    }
    fn max_size(&self) -> () {
        unimplemented!()
    }
    fn destroy(&self, mut p: Ptr<()>) {
        unimplemented!()
    }
    fn get_data(&self) -> () {
        unimplemented!()
    }
    fn get_id(&self) -> ();
}
impl test_allocator_std_pair_const_int__long__Impl
    for Ptr<test_allocator_std_pair_const_int__long__>
{
    fn destructor(&self) {
        unimplemented!();
        unimplemented!();
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_dec());
        }
        field!((*self), data_).write(destructed_value_7.with(|rc| *rc.borrow()));
        field!((*self), id_).write(destructed_value_7.with(|rc| *rc.borrow()));
    }
    fn get_id(&self) -> () {
        return (*self).with(|__s| __s.id_);
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
        field!((*self), data_).write(destructed_value_7.with(|rc| *rc.borrow()));
        field!((*self), id_).write(destructed_value_7.with(|rc| *rc.borrow()));
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
    let _ = destructed_value_7.with(|_| ());
    let _ = moved_value_8.with(|_| ());
}
