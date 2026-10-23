extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn operator_eq_0(a: Ptr<test_equal_to_int_>, b: Ptr<test_equal_to_int_>) -> () {
    return ({ a.with(|__s| __s.data_) } == { b.with(|__s| __s.data_) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct test_equal_to_int_ {
    #[offset(0)]
    pub data_: (),
}
impl test_equal_to_int_ {
    pub fn new_1() -> Self {
        Self { data_: 0_() }
    }
    pub fn new_2(mut data: ()) -> Self {
        Self { data_: data }
    }
}
impl std::cmp::PartialEq for test_equal_to_int_ {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_0(
                Rc::new(RefCell::new(test_equal_to_int_ {
                    data_: self.data_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(test_equal_to_int_ {
                    data_: other.data_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for test_equal_to_int_ {}
impl Default for test_equal_to_int_ {
    fn default() -> Self {
        { test_equal_to_int_::new_1() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct test_hash_int_ {
    #[offset(0)]
    data_: (),
}
impl test_hash_int_ {
    pub fn new(data: Option<()>) -> Self {
        let mut data: () = data.unwrap_or_else(|| 0_());
        Self { data_: data }
    }
}
impl std::cmp::PartialEq for test_hash_int_ {
    fn eq(&self, other: &Self) -> bool {
        {
            test_hash_int_Impl::operator_eq(
                &Rc::new(RefCell::new(test_hash_int_ {
                    data_: self.data_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(test_hash_int_ {
                    data_: other.data_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for test_hash_int_ {}
impl Default for test_hash_int_ {
    fn default() -> Self {
        { test_hash_int_::new(None) }
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
    pub static destructed_value_1: Value<()> = Rc::new(RefCell::new(-1_()));
);
thread_local!(
    pub static moved_value_2: Value<()> = Rc::new(RefCell::new(2147483647_()));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_alloc_base {}
pub fn operator_eq_3(x: Ptr<test_allocator_void_>, y: Ptr<test_allocator_void_>) -> () {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
pub fn operator_ne_4(x: Ptr<test_allocator_void_>, y: Ptr<test_allocator_void_>) -> () {
    return !({
        let _x: Ptr<test_allocator_void_> = (x).clone();
        let _y: Ptr<test_allocator_void_> = (y).clone();
        operator_eq_3(_x, _y)
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
            operator_eq_3(
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
pub fn operator_eq_5(
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
    pub fn new_1(mut data: ()) -> Self {
        Self {
            data_: data,
            id_: 0_(),
            stats_: Ptr::<test_allocator_statistics>::null(),
        }
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
            operator_eq_5(
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
                id_: 0_(),
                stats_: Ptr::<test_allocator_statistics>::null(),
            }
        }
    }
}
pub fn operator_eq_6(
    x: Ptr<other_allocator_std_pair_const_int__int__>,
    y: Ptr<other_allocator_std_pair_const_int__int__>,
) -> () {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct other_allocator_std_pair_const_int__int__ {
    #[offset(0)]
    data_: (),
}
impl other_allocator_std_pair_const_int__int__ {
    pub fn new_1(mut i: ()) -> Self {
        Self { data_: i }
    }
}
impl std::cmp::PartialEq for other_allocator_std_pair_const_int__int__ {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_6(
                Rc::new(RefCell::new(other_allocator_std_pair_const_int__int__ {
                    data_: self.data_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(other_allocator_std_pair_const_int__int__ {
                    data_: other.data_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for other_allocator_std_pair_const_int__int__ {}
impl Default for other_allocator_std_pair_const_int__int__ {
    fn default() -> Self {
        {
            other_allocator_std_pair_const_int__int__ {
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
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub fn outstanding_alloc() -> () {
        unimplemented!();
        return ((alloc_count_8.with(|rc| *rc.borrow()))
            .wrapping_sub(dealloc_count_9.with(|rc| *rc.borrow())));
    }
    pub fn reset() {
        unimplemented!();
        disable_default_constructor_10.with(|rc| *rc.borrow_mut() = false);
        outstanding_bytes_7.with(|rc| *rc.borrow_mut() = 0_());
        alloc_count_8.with(|rc| *rc.borrow_mut() = 0_());
        dealloc_count_9.with(|rc| *rc.borrow_mut() = 0_());
    }
}
thread_local!(
    pub static outstanding_bytes_7: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static alloc_count_8: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static dealloc_count_9: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static disable_default_constructor_10: Value<()> = Rc::new(RefCell::new(false));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<()>,
}
impl min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0__ {
    fn new_1(mut p: Ptr<()>) -> Self {
        Self { ptr_: (p).clone() }
    }
    pub fn pointer_to(
        t: Ptr<()>,
    ) -> min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0__ {
        return min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0__ :: new_1 ( { unimplemented!()   } , )    ;
    }
}
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
pub struct min_pointer_const_std_pair_const_int__int___std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<()>,
}
impl min_pointer_const_std_pair_const_int__int___std_integral_constant_unsigned_long__0__ {
    fn new_1(mut p: Ptr<()>) -> Self {
        Self { ptr_: (p).clone() }
    }
    pub fn pointer_to(
        t: Ptr<()>,
    ) -> min_pointer_const_std_pair_const_int__int___std_integral_constant_unsigned_long__0__ {
        return min_pointer_const_std_pair_const_int__int___std_integral_constant_unsigned_long__0__ :: new_1 ( { unimplemented!()   } , )    ;
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_const_void__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: (),
}
impl min_pointer_const_void__std_integral_constant_unsigned_long__0__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { #[offset(0)] #[byte_size(8)] ptr_ : Ptr< min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__  >  , }
impl min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { fn new_1 ( mut p : Ptr< min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__  >  , ) -> Self { Self { ptr_ : (p ).clone() , }  } pub fn new_2 ( mut _a0 : ()  , ) -> Self { Self { ptr_ : Ptr::<min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__>::null()  , }  } }
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_std_pair_const_int__int__ {}
impl min_allocator_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0___ { pub fn new_1 ( mut _a0 : min_allocator_std_pair_const_int__int__  , ) -> Self { Self { }  } }
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0___
{}
impl min_allocator_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0___ { }
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct tracking_allocator_std_pair_const_int__int__ {
    #[offset(0)]
    #[byte_size(8)]
    allocs_: Ptr<()>,
}
impl tracking_allocator_std_pair_const_int__int__ {
    pub fn new(allocs: Ptr<()>) -> Self {
        Self {
            allocs_: (allocs).clone(),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct NoOp {}
pub fn test_alloc_11(
    lhs_alloc: Option<Ptr<()>>,
    rhs_alloc: Option<Ptr<()>>,
    check_alloc_invariant: Option<NoOp>,
) {
    let mut __lhs_alloc_default: Option<Value<()>> = None;
    let lhs_alloc: Ptr<()> = lhs_alloc.unwrap_or_else(|| {
        __lhs_alloc_default
            .insert(Rc::new(RefCell::new(unimplemented!())))
            .as_pointer()
    });
    let mut __rhs_alloc_default: Option<Value<()>> = None;
    let rhs_alloc: Ptr<()> = rhs_alloc.unwrap_or_else(|| {
        __rhs_alloc_default
            .insert(Rc::new(RefCell::new(unimplemented!())))
            .as_pointer()
    });
    let check_alloc_invariant: Value<NoOp> = Rc::new(RefCell::new(
        check_alloc_invariant.unwrap_or_else(|| <NoOp>::default()),
    ));
    {
        {
            let mut arr: [(); 4] = [
                {
                    let __tmp_0: Value<()> = Rc::new(RefCell::new(1_()));
                    let __tmp_1: Value<()> = Rc::new(RefCell::new(1_()));
                    std_pair_const_int__int_::new_1({ __tmp_0.as_pointer() }, {
                        __tmp_1.as_pointer()
                    })
                },
                {
                    let __tmp_2: Value<()> = Rc::new(RefCell::new(2_()));
                    let __tmp_3: Value<()> = Rc::new(RefCell::new(3_()));
                    std_pair_const_int__int_::new_1({ __tmp_2.as_pointer() }, {
                        __tmp_3.as_pointer()
                    })
                },
                {
                    let __tmp_4: Value<()> = Rc::new(RefCell::new(4_()));
                    let __tmp_5: Value<()> = Rc::new(RefCell::new(4_()));
                    std_pair_const_int__int_::new_1({ __tmp_4.as_pointer() }, {
                        __tmp_5.as_pointer()
                    })
                },
                {
                    let __tmp_6: Value<()> = Rc::new(RefCell::new(5_()));
                    let __tmp_7: Value<()> = Rc::new(RefCell::new(2_()));
                    std_pair_const_int__int_::new_1({ __tmp_6.as_pointer() }, {
                        __tmp_7.as_pointer()
                    })
                },
            ];
            let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        ({ NoOpImpl::operator_call(&check_alloc_invariant.as_pointer()) });
        {
            let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        ({ NoOpImpl::operator_call(&check_alloc_invariant.as_pointer()) });
        {
            let mut arr: [(); 4] = [
                {
                    let __tmp_8: Value<()> = Rc::new(RefCell::new(1_()));
                    let __tmp_9: Value<()> = Rc::new(RefCell::new(1_()));
                    std_pair_const_int__int_::new_1({ __tmp_8.as_pointer() }, {
                        __tmp_9.as_pointer()
                    })
                },
                {
                    let __tmp_10: Value<()> = Rc::new(RefCell::new(2_()));
                    let __tmp_11: Value<()> = Rc::new(RefCell::new(3_()));
                    std_pair_const_int__int_::new_1({ __tmp_10.as_pointer() }, {
                        __tmp_11.as_pointer()
                    })
                },
                {
                    let __tmp_12: Value<()> = Rc::new(RefCell::new(4_()));
                    let __tmp_13: Value<()> = Rc::new(RefCell::new(4_()));
                    std_pair_const_int__int_::new_1({ __tmp_12.as_pointer() }, {
                        __tmp_13.as_pointer()
                    })
                },
                {
                    let __tmp_14: Value<()> = Rc::new(RefCell::new(5_()));
                    let __tmp_15: Value<()> = Rc::new(RefCell::new(2_()));
                    std_pair_const_int__int_::new_1({ __tmp_14.as_pointer() }, {
                        __tmp_15.as_pointer()
                    })
                },
            ];
            let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        ({ NoOpImpl::operator_call(&check_alloc_invariant.as_pointer()) });
    }
    {
        {
            let mut arr: [(); 1] = [{
                let __tmp_16: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_17: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_const_int__int_::new_1({ __tmp_16.as_pointer() }, {
                    __tmp_17.as_pointer()
                })
            }];
            let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut arr: [(); 1] = [{
                let __tmp_18: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_19: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_const_int__int_::new_1({ __tmp_18.as_pointer() }, {
                    __tmp_19.as_pointer()
                })
            }];
            let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        {
            let mut arr: [(); 4] = [
                {
                    let __tmp_20: Value<()> = Rc::new(RefCell::new(1_()));
                    let __tmp_21: Value<()> = Rc::new(RefCell::new(1_()));
                    std_pair_const_int__int_::new_1({ __tmp_20.as_pointer() }, {
                        __tmp_21.as_pointer()
                    })
                },
                {
                    let __tmp_22: Value<()> = Rc::new(RefCell::new(2_()));
                    let __tmp_23: Value<()> = Rc::new(RefCell::new(3_()));
                    std_pair_const_int__int_::new_1({ __tmp_22.as_pointer() }, {
                        __tmp_23.as_pointer()
                    })
                },
                {
                    let __tmp_24: Value<()> = Rc::new(RefCell::new(4_()));
                    let __tmp_25: Value<()> = Rc::new(RefCell::new(4_()));
                    std_pair_const_int__int_::new_1({ __tmp_24.as_pointer() }, {
                        __tmp_25.as_pointer()
                    })
                },
                {
                    let __tmp_26: Value<()> = Rc::new(RefCell::new(5_()));
                    let __tmp_27: Value<()> = Rc::new(RefCell::new(2_()));
                    std_pair_const_int__int_::new_1({ __tmp_26.as_pointer() }, {
                        __tmp_27.as_pointer()
                    })
                },
            ];
            let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        ({ NoOpImpl::operator_call(&check_alloc_invariant.as_pointer()) });
        {
            let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        ({ NoOpImpl::operator_call(&check_alloc_invariant.as_pointer()) });
    }
    {
        {
            let mut lhs_arr: [(); 4] = [
                {
                    let __tmp_28: Value<()> = Rc::new(RefCell::new(1_()));
                    let __tmp_29: Value<()> = Rc::new(RefCell::new(1_()));
                    std_pair_const_int__int_::new_1({ __tmp_28.as_pointer() }, {
                        __tmp_29.as_pointer()
                    })
                },
                {
                    let __tmp_30: Value<()> = Rc::new(RefCell::new(2_()));
                    let __tmp_31: Value<()> = Rc::new(RefCell::new(3_()));
                    std_pair_const_int__int_::new_1({ __tmp_30.as_pointer() }, {
                        __tmp_31.as_pointer()
                    })
                },
                {
                    let __tmp_32: Value<()> = Rc::new(RefCell::new(4_()));
                    let __tmp_33: Value<()> = Rc::new(RefCell::new(4_()));
                    std_pair_const_int__int_::new_1({ __tmp_32.as_pointer() }, {
                        __tmp_33.as_pointer()
                    })
                },
                {
                    let __tmp_34: Value<()> = Rc::new(RefCell::new(5_()));
                    let __tmp_35: Value<()> = Rc::new(RefCell::new(2_()));
                    std_pair_const_int__int_::new_1({ __tmp_34.as_pointer() }, {
                        __tmp_35.as_pointer()
                    })
                },
            ];
            let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let mut rhs_arr: [(); 2] = [
                {
                    let __tmp_36: Value<()> = Rc::new(RefCell::new(10_()));
                    let __tmp_37: Value<()> = Rc::new(RefCell::new(4_()));
                    std_pair_const_int__int_::new_1({ __tmp_36.as_pointer() }, {
                        __tmp_37.as_pointer()
                    })
                },
                {
                    let __tmp_38: Value<()> = Rc::new(RefCell::new(13_()));
                    let __tmp_39: Value<()> = Rc::new(RefCell::new(5_()));
                    std_pair_const_int__int_::new_1({ __tmp_38.as_pointer() }, {
                        __tmp_39.as_pointer()
                    })
                },
            ];
            let copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        ({ NoOpImpl::operator_call(&check_alloc_invariant.as_pointer()) });
        {
            let mut lhs_arr: [(); 4] = [
                {
                    let __tmp_40: Value<()> = Rc::new(RefCell::new(1_()));
                    let __tmp_41: Value<()> = Rc::new(RefCell::new(1_()));
                    std_pair_const_int__int_::new_1({ __tmp_40.as_pointer() }, {
                        __tmp_41.as_pointer()
                    })
                },
                {
                    let __tmp_42: Value<()> = Rc::new(RefCell::new(2_()));
                    let __tmp_43: Value<()> = Rc::new(RefCell::new(3_()));
                    std_pair_const_int__int_::new_1({ __tmp_42.as_pointer() }, {
                        __tmp_43.as_pointer()
                    })
                },
                {
                    let __tmp_44: Value<()> = Rc::new(RefCell::new(4_()));
                    let __tmp_45: Value<()> = Rc::new(RefCell::new(4_()));
                    std_pair_const_int__int_::new_1({ __tmp_44.as_pointer() }, {
                        __tmp_45.as_pointer()
                    })
                },
                {
                    let __tmp_46: Value<()> = Rc::new(RefCell::new(5_()));
                    let __tmp_47: Value<()> = Rc::new(RefCell::new(2_()));
                    std_pair_const_int__int_::new_1({ __tmp_46.as_pointer() }, {
                        __tmp_47.as_pointer()
                    })
                },
            ];
            let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let mut rhs_arr: [(); 4] = [
                {
                    let __tmp_48: Value<()> = Rc::new(RefCell::new(10_()));
                    let __tmp_49: Value<()> = Rc::new(RefCell::new(4_()));
                    std_pair_const_int__int_::new_1({ __tmp_48.as_pointer() }, {
                        __tmp_49.as_pointer()
                    })
                },
                {
                    let __tmp_50: Value<()> = Rc::new(RefCell::new(13_()));
                    let __tmp_51: Value<()> = Rc::new(RefCell::new(5_()));
                    std_pair_const_int__int_::new_1({ __tmp_50.as_pointer() }, {
                        __tmp_51.as_pointer()
                    })
                },
                {
                    let __tmp_52: Value<()> = Rc::new(RefCell::new(12_()));
                    let __tmp_53: Value<()> = Rc::new(RefCell::new(324_()));
                    std_pair_const_int__int_::new_1({ __tmp_52.as_pointer() }, {
                        __tmp_53.as_pointer()
                    })
                },
                {
                    let __tmp_54: Value<()> = Rc::new(RefCell::new(0_()));
                    let __tmp_55: Value<()> = Rc::new(RefCell::new(54_()));
                    std_pair_const_int__int_::new_1({ __tmp_54.as_pointer() }, {
                        __tmp_55.as_pointer()
                    })
                },
            ];
            let copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        ({ NoOpImpl::operator_call(&check_alloc_invariant.as_pointer()) });
        {
            let mut lhs_arr: [(); 4] = [
                {
                    let __tmp_56: Value<()> = Rc::new(RefCell::new(1_()));
                    let __tmp_57: Value<()> = Rc::new(RefCell::new(1_()));
                    std_pair_const_int__int_::new_1({ __tmp_56.as_pointer() }, {
                        __tmp_57.as_pointer()
                    })
                },
                {
                    let __tmp_58: Value<()> = Rc::new(RefCell::new(2_()));
                    let __tmp_59: Value<()> = Rc::new(RefCell::new(3_()));
                    std_pair_const_int__int_::new_1({ __tmp_58.as_pointer() }, {
                        __tmp_59.as_pointer()
                    })
                },
                {
                    let __tmp_60: Value<()> = Rc::new(RefCell::new(4_()));
                    let __tmp_61: Value<()> = Rc::new(RefCell::new(4_()));
                    std_pair_const_int__int_::new_1({ __tmp_60.as_pointer() }, {
                        __tmp_61.as_pointer()
                    })
                },
                {
                    let __tmp_62: Value<()> = Rc::new(RefCell::new(5_()));
                    let __tmp_63: Value<()> = Rc::new(RefCell::new(2_()));
                    std_pair_const_int__int_::new_1({ __tmp_62.as_pointer() }, {
                        __tmp_63.as_pointer()
                    })
                },
            ];
            let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let mut rhs_arr: [(); 6] = [
                {
                    let __tmp_64: Value<()> = Rc::new(RefCell::new(10_()));
                    let __tmp_65: Value<()> = Rc::new(RefCell::new(4_()));
                    std_pair_const_int__int_::new_1({ __tmp_64.as_pointer() }, {
                        __tmp_65.as_pointer()
                    })
                },
                {
                    let __tmp_66: Value<()> = Rc::new(RefCell::new(13_()));
                    let __tmp_67: Value<()> = Rc::new(RefCell::new(5_()));
                    std_pair_const_int__int_::new_1({ __tmp_66.as_pointer() }, {
                        __tmp_67.as_pointer()
                    })
                },
                {
                    let __tmp_68: Value<()> = Rc::new(RefCell::new(12_()));
                    let __tmp_69: Value<()> = Rc::new(RefCell::new(324_()));
                    std_pair_const_int__int_::new_1({ __tmp_68.as_pointer() }, {
                        __tmp_69.as_pointer()
                    })
                },
                {
                    let __tmp_70: Value<()> = Rc::new(RefCell::new(0_()));
                    let __tmp_71: Value<()> = Rc::new(RefCell::new(54_()));
                    std_pair_const_int__int_::new_1({ __tmp_70.as_pointer() }, {
                        __tmp_71.as_pointer()
                    })
                },
                {
                    let __tmp_72: Value<()> = Rc::new(RefCell::new(50_()));
                    let __tmp_73: Value<()> = Rc::new(RefCell::new(5_()));
                    std_pair_const_int__int_::new_1({ __tmp_72.as_pointer() }, {
                        __tmp_73.as_pointer()
                    })
                },
                {
                    let __tmp_74: Value<()> = Rc::new(RefCell::new(2_()));
                    let __tmp_75: Value<()> = Rc::new(RefCell::new(5_()));
                    std_pair_const_int__int_::new_1({ __tmp_74.as_pointer() }, {
                        __tmp_75.as_pointer()
                    })
                },
            ];
            let copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        ({ NoOpImpl::operator_call(&check_alloc_invariant.as_pointer()) });
    }
}
pub fn test_alloc_12(
    lhs_alloc: Option<Ptr<min_allocator_std_pair_const_int__int__>>,
    rhs_alloc: Option<Ptr<min_allocator_std_pair_const_int__int__>>,
    check_alloc_invariant: Option<NoOp>,
) {
    let mut __lhs_alloc_default: Option<Value<min_allocator_std_pair_const_int__int__>> = None;
    let lhs_alloc: Ptr<min_allocator_std_pair_const_int__int__> = lhs_alloc.unwrap_or_else(|| {
        __lhs_alloc_default
            .insert(Rc::new(RefCell::new(
                <min_allocator_std_pair_const_int__int__>::default(),
            )))
            .as_pointer()
    });
    let mut __rhs_alloc_default: Option<Value<min_allocator_std_pair_const_int__int__>> = None;
    let rhs_alloc: Ptr<min_allocator_std_pair_const_int__int__> = rhs_alloc.unwrap_or_else(|| {
        __rhs_alloc_default
            .insert(Rc::new(RefCell::new(
                <min_allocator_std_pair_const_int__int__>::default(),
            )))
            .as_pointer()
    });
    let check_alloc_invariant: Value<NoOp> = Rc::new(RefCell::new(
        check_alloc_invariant.unwrap_or_else(|| <NoOp>::default()),
    ));
    {
        {
            let mut arr: [(); 4] = [
                {
                    let __tmp_76: Value<()> = Rc::new(RefCell::new(1_()));
                    let __tmp_77: Value<()> = Rc::new(RefCell::new(1_()));
                    std_pair_const_int__int_::new_1({ __tmp_76.as_pointer() }, {
                        __tmp_77.as_pointer()
                    })
                },
                {
                    let __tmp_78: Value<()> = Rc::new(RefCell::new(2_()));
                    let __tmp_79: Value<()> = Rc::new(RefCell::new(3_()));
                    std_pair_const_int__int_::new_1({ __tmp_78.as_pointer() }, {
                        __tmp_79.as_pointer()
                    })
                },
                {
                    let __tmp_80: Value<()> = Rc::new(RefCell::new(4_()));
                    let __tmp_81: Value<()> = Rc::new(RefCell::new(4_()));
                    std_pair_const_int__int_::new_1({ __tmp_80.as_pointer() }, {
                        __tmp_81.as_pointer()
                    })
                },
                {
                    let __tmp_82: Value<()> = Rc::new(RefCell::new(5_()));
                    let __tmp_83: Value<()> = Rc::new(RefCell::new(2_()));
                    std_pair_const_int__int_::new_1({ __tmp_82.as_pointer() }, {
                        __tmp_83.as_pointer()
                    })
                },
            ];
            let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        ({ NoOpImpl::operator_call(&check_alloc_invariant.as_pointer()) });
        {
            let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        ({ NoOpImpl::operator_call(&check_alloc_invariant.as_pointer()) });
        {
            let mut arr: [(); 4] = [
                {
                    let __tmp_84: Value<()> = Rc::new(RefCell::new(1_()));
                    let __tmp_85: Value<()> = Rc::new(RefCell::new(1_()));
                    std_pair_const_int__int_::new_1({ __tmp_84.as_pointer() }, {
                        __tmp_85.as_pointer()
                    })
                },
                {
                    let __tmp_86: Value<()> = Rc::new(RefCell::new(2_()));
                    let __tmp_87: Value<()> = Rc::new(RefCell::new(3_()));
                    std_pair_const_int__int_::new_1({ __tmp_86.as_pointer() }, {
                        __tmp_87.as_pointer()
                    })
                },
                {
                    let __tmp_88: Value<()> = Rc::new(RefCell::new(4_()));
                    let __tmp_89: Value<()> = Rc::new(RefCell::new(4_()));
                    std_pair_const_int__int_::new_1({ __tmp_88.as_pointer() }, {
                        __tmp_89.as_pointer()
                    })
                },
                {
                    let __tmp_90: Value<()> = Rc::new(RefCell::new(5_()));
                    let __tmp_91: Value<()> = Rc::new(RefCell::new(2_()));
                    std_pair_const_int__int_::new_1({ __tmp_90.as_pointer() }, {
                        __tmp_91.as_pointer()
                    })
                },
            ];
            let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        ({ NoOpImpl::operator_call(&check_alloc_invariant.as_pointer()) });
    }
    {
        {
            let mut arr: [(); 1] = [{
                let __tmp_92: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_93: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_const_int__int_::new_1({ __tmp_92.as_pointer() }, {
                    __tmp_93.as_pointer()
                })
            }];
            let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut arr: [(); 1] = [{
                let __tmp_94: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_95: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_const_int__int_::new_1({ __tmp_94.as_pointer() }, {
                    __tmp_95.as_pointer()
                })
            }];
            let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        {
            let mut arr: [(); 4] = [
                {
                    let __tmp_96: Value<()> = Rc::new(RefCell::new(1_()));
                    let __tmp_97: Value<()> = Rc::new(RefCell::new(1_()));
                    std_pair_const_int__int_::new_1({ __tmp_96.as_pointer() }, {
                        __tmp_97.as_pointer()
                    })
                },
                {
                    let __tmp_98: Value<()> = Rc::new(RefCell::new(2_()));
                    let __tmp_99: Value<()> = Rc::new(RefCell::new(3_()));
                    std_pair_const_int__int_::new_1({ __tmp_98.as_pointer() }, {
                        __tmp_99.as_pointer()
                    })
                },
                {
                    let __tmp_100: Value<()> = Rc::new(RefCell::new(4_()));
                    let __tmp_101: Value<()> = Rc::new(RefCell::new(4_()));
                    std_pair_const_int__int_::new_1({ __tmp_100.as_pointer() }, {
                        __tmp_101.as_pointer()
                    })
                },
                {
                    let __tmp_102: Value<()> = Rc::new(RefCell::new(5_()));
                    let __tmp_103: Value<()> = Rc::new(RefCell::new(2_()));
                    std_pair_const_int__int_::new_1({ __tmp_102.as_pointer() }, {
                        __tmp_103.as_pointer()
                    })
                },
            ];
            let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        ({ NoOpImpl::operator_call(&check_alloc_invariant.as_pointer()) });
        {
            let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        ({ NoOpImpl::operator_call(&check_alloc_invariant.as_pointer()) });
    }
    {
        {
            let mut lhs_arr: [(); 4] = [
                {
                    let __tmp_104: Value<()> = Rc::new(RefCell::new(1_()));
                    let __tmp_105: Value<()> = Rc::new(RefCell::new(1_()));
                    std_pair_const_int__int_::new_1({ __tmp_104.as_pointer() }, {
                        __tmp_105.as_pointer()
                    })
                },
                {
                    let __tmp_106: Value<()> = Rc::new(RefCell::new(2_()));
                    let __tmp_107: Value<()> = Rc::new(RefCell::new(3_()));
                    std_pair_const_int__int_::new_1({ __tmp_106.as_pointer() }, {
                        __tmp_107.as_pointer()
                    })
                },
                {
                    let __tmp_108: Value<()> = Rc::new(RefCell::new(4_()));
                    let __tmp_109: Value<()> = Rc::new(RefCell::new(4_()));
                    std_pair_const_int__int_::new_1({ __tmp_108.as_pointer() }, {
                        __tmp_109.as_pointer()
                    })
                },
                {
                    let __tmp_110: Value<()> = Rc::new(RefCell::new(5_()));
                    let __tmp_111: Value<()> = Rc::new(RefCell::new(2_()));
                    std_pair_const_int__int_::new_1({ __tmp_110.as_pointer() }, {
                        __tmp_111.as_pointer()
                    })
                },
            ];
            let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let mut rhs_arr: [(); 2] = [
                {
                    let __tmp_112: Value<()> = Rc::new(RefCell::new(10_()));
                    let __tmp_113: Value<()> = Rc::new(RefCell::new(4_()));
                    std_pair_const_int__int_::new_1({ __tmp_112.as_pointer() }, {
                        __tmp_113.as_pointer()
                    })
                },
                {
                    let __tmp_114: Value<()> = Rc::new(RefCell::new(13_()));
                    let __tmp_115: Value<()> = Rc::new(RefCell::new(5_()));
                    std_pair_const_int__int_::new_1({ __tmp_114.as_pointer() }, {
                        __tmp_115.as_pointer()
                    })
                },
            ];
            let copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        ({ NoOpImpl::operator_call(&check_alloc_invariant.as_pointer()) });
        {
            let mut lhs_arr: [(); 4] = [
                {
                    let __tmp_116: Value<()> = Rc::new(RefCell::new(1_()));
                    let __tmp_117: Value<()> = Rc::new(RefCell::new(1_()));
                    std_pair_const_int__int_::new_1({ __tmp_116.as_pointer() }, {
                        __tmp_117.as_pointer()
                    })
                },
                {
                    let __tmp_118: Value<()> = Rc::new(RefCell::new(2_()));
                    let __tmp_119: Value<()> = Rc::new(RefCell::new(3_()));
                    std_pair_const_int__int_::new_1({ __tmp_118.as_pointer() }, {
                        __tmp_119.as_pointer()
                    })
                },
                {
                    let __tmp_120: Value<()> = Rc::new(RefCell::new(4_()));
                    let __tmp_121: Value<()> = Rc::new(RefCell::new(4_()));
                    std_pair_const_int__int_::new_1({ __tmp_120.as_pointer() }, {
                        __tmp_121.as_pointer()
                    })
                },
                {
                    let __tmp_122: Value<()> = Rc::new(RefCell::new(5_()));
                    let __tmp_123: Value<()> = Rc::new(RefCell::new(2_()));
                    std_pair_const_int__int_::new_1({ __tmp_122.as_pointer() }, {
                        __tmp_123.as_pointer()
                    })
                },
            ];
            let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let mut rhs_arr: [(); 4] = [
                {
                    let __tmp_124: Value<()> = Rc::new(RefCell::new(10_()));
                    let __tmp_125: Value<()> = Rc::new(RefCell::new(4_()));
                    std_pair_const_int__int_::new_1({ __tmp_124.as_pointer() }, {
                        __tmp_125.as_pointer()
                    })
                },
                {
                    let __tmp_126: Value<()> = Rc::new(RefCell::new(13_()));
                    let __tmp_127: Value<()> = Rc::new(RefCell::new(5_()));
                    std_pair_const_int__int_::new_1({ __tmp_126.as_pointer() }, {
                        __tmp_127.as_pointer()
                    })
                },
                {
                    let __tmp_128: Value<()> = Rc::new(RefCell::new(12_()));
                    let __tmp_129: Value<()> = Rc::new(RefCell::new(324_()));
                    std_pair_const_int__int_::new_1({ __tmp_128.as_pointer() }, {
                        __tmp_129.as_pointer()
                    })
                },
                {
                    let __tmp_130: Value<()> = Rc::new(RefCell::new(0_()));
                    let __tmp_131: Value<()> = Rc::new(RefCell::new(54_()));
                    std_pair_const_int__int_::new_1({ __tmp_130.as_pointer() }, {
                        __tmp_131.as_pointer()
                    })
                },
            ];
            let copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        ({ NoOpImpl::operator_call(&check_alloc_invariant.as_pointer()) });
        {
            let mut lhs_arr: [(); 4] = [
                {
                    let __tmp_132: Value<()> = Rc::new(RefCell::new(1_()));
                    let __tmp_133: Value<()> = Rc::new(RefCell::new(1_()));
                    std_pair_const_int__int_::new_1({ __tmp_132.as_pointer() }, {
                        __tmp_133.as_pointer()
                    })
                },
                {
                    let __tmp_134: Value<()> = Rc::new(RefCell::new(2_()));
                    let __tmp_135: Value<()> = Rc::new(RefCell::new(3_()));
                    std_pair_const_int__int_::new_1({ __tmp_134.as_pointer() }, {
                        __tmp_135.as_pointer()
                    })
                },
                {
                    let __tmp_136: Value<()> = Rc::new(RefCell::new(4_()));
                    let __tmp_137: Value<()> = Rc::new(RefCell::new(4_()));
                    std_pair_const_int__int_::new_1({ __tmp_136.as_pointer() }, {
                        __tmp_137.as_pointer()
                    })
                },
                {
                    let __tmp_138: Value<()> = Rc::new(RefCell::new(5_()));
                    let __tmp_139: Value<()> = Rc::new(RefCell::new(2_()));
                    std_pair_const_int__int_::new_1({ __tmp_138.as_pointer() }, {
                        __tmp_139.as_pointer()
                    })
                },
            ];
            let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let mut rhs_arr: [(); 6] = [
                {
                    let __tmp_140: Value<()> = Rc::new(RefCell::new(10_()));
                    let __tmp_141: Value<()> = Rc::new(RefCell::new(4_()));
                    std_pair_const_int__int_::new_1({ __tmp_140.as_pointer() }, {
                        __tmp_141.as_pointer()
                    })
                },
                {
                    let __tmp_142: Value<()> = Rc::new(RefCell::new(13_()));
                    let __tmp_143: Value<()> = Rc::new(RefCell::new(5_()));
                    std_pair_const_int__int_::new_1({ __tmp_142.as_pointer() }, {
                        __tmp_143.as_pointer()
                    })
                },
                {
                    let __tmp_144: Value<()> = Rc::new(RefCell::new(12_()));
                    let __tmp_145: Value<()> = Rc::new(RefCell::new(324_()));
                    std_pair_const_int__int_::new_1({ __tmp_144.as_pointer() }, {
                        __tmp_145.as_pointer()
                    })
                },
                {
                    let __tmp_146: Value<()> = Rc::new(RefCell::new(0_()));
                    let __tmp_147: Value<()> = Rc::new(RefCell::new(54_()));
                    std_pair_const_int__int_::new_1({ __tmp_146.as_pointer() }, {
                        __tmp_147.as_pointer()
                    })
                },
                {
                    let __tmp_148: Value<()> = Rc::new(RefCell::new(50_()));
                    let __tmp_149: Value<()> = Rc::new(RefCell::new(5_()));
                    std_pair_const_int__int_::new_1({ __tmp_148.as_pointer() }, {
                        __tmp_149.as_pointer()
                    })
                },
                {
                    let __tmp_150: Value<()> = Rc::new(RefCell::new(2_()));
                    let __tmp_151: Value<()> = Rc::new(RefCell::new(5_()));
                    std_pair_const_int__int_::new_1({ __tmp_150.as_pointer() }, {
                        __tmp_151.as_pointer()
                    })
                },
            ];
            let copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        ({ NoOpImpl::operator_call(&check_alloc_invariant.as_pointer()) });
    }
}
pub fn test_alloc_13(
    lhs_alloc: Ptr<tracking_allocator_std_pair_const_int__int__>,
    rhs_alloc: Ptr<tracking_allocator_std_pair_const_int__int__>,
    check_alloc_invariant: AssertEmpty_14,
) {
    let check_alloc_invariant: Value<AssertEmpty_14> = Rc::new(RefCell::new(check_alloc_invariant));
    {
        {
            let mut arr: [(); 4] = [
                {
                    let __tmp_152: Value<()> = Rc::new(RefCell::new(1_()));
                    let __tmp_153: Value<()> = Rc::new(RefCell::new(1_()));
                    std_pair_const_int__int_::new_1({ __tmp_152.as_pointer() }, {
                        __tmp_153.as_pointer()
                    })
                },
                {
                    let __tmp_154: Value<()> = Rc::new(RefCell::new(2_()));
                    let __tmp_155: Value<()> = Rc::new(RefCell::new(3_()));
                    std_pair_const_int__int_::new_1({ __tmp_154.as_pointer() }, {
                        __tmp_155.as_pointer()
                    })
                },
                {
                    let __tmp_156: Value<()> = Rc::new(RefCell::new(4_()));
                    let __tmp_157: Value<()> = Rc::new(RefCell::new(4_()));
                    std_pair_const_int__int_::new_1({ __tmp_156.as_pointer() }, {
                        __tmp_157.as_pointer()
                    })
                },
                {
                    let __tmp_158: Value<()> = Rc::new(RefCell::new(5_()));
                    let __tmp_159: Value<()> = Rc::new(RefCell::new(2_()));
                    std_pair_const_int__int_::new_1({ __tmp_158.as_pointer() }, {
                        __tmp_159.as_pointer()
                    })
                },
            ];
            let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        ({ AssertEmpty_14Impl::operator_call(&check_alloc_invariant.as_pointer()) });
        {
            let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        ({ AssertEmpty_14Impl::operator_call(&check_alloc_invariant.as_pointer()) });
        {
            let mut arr: [(); 4] = [
                {
                    let __tmp_160: Value<()> = Rc::new(RefCell::new(1_()));
                    let __tmp_161: Value<()> = Rc::new(RefCell::new(1_()));
                    std_pair_const_int__int_::new_1({ __tmp_160.as_pointer() }, {
                        __tmp_161.as_pointer()
                    })
                },
                {
                    let __tmp_162: Value<()> = Rc::new(RefCell::new(2_()));
                    let __tmp_163: Value<()> = Rc::new(RefCell::new(3_()));
                    std_pair_const_int__int_::new_1({ __tmp_162.as_pointer() }, {
                        __tmp_163.as_pointer()
                    })
                },
                {
                    let __tmp_164: Value<()> = Rc::new(RefCell::new(4_()));
                    let __tmp_165: Value<()> = Rc::new(RefCell::new(4_()));
                    std_pair_const_int__int_::new_1({ __tmp_164.as_pointer() }, {
                        __tmp_165.as_pointer()
                    })
                },
                {
                    let __tmp_166: Value<()> = Rc::new(RefCell::new(5_()));
                    let __tmp_167: Value<()> = Rc::new(RefCell::new(2_()));
                    std_pair_const_int__int_::new_1({ __tmp_166.as_pointer() }, {
                        __tmp_167.as_pointer()
                    })
                },
            ];
            let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        ({ AssertEmpty_14Impl::operator_call(&check_alloc_invariant.as_pointer()) });
    }
    {
        {
            let mut arr: [(); 1] = [{
                let __tmp_168: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_169: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_const_int__int_::new_1({ __tmp_168.as_pointer() }, {
                    __tmp_169.as_pointer()
                })
            }];
            let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut arr: [(); 1] = [{
                let __tmp_170: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_171: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_const_int__int_::new_1({ __tmp_170.as_pointer() }, {
                    __tmp_171.as_pointer()
                })
            }];
            let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        {
            let mut arr: [(); 4] = [
                {
                    let __tmp_172: Value<()> = Rc::new(RefCell::new(1_()));
                    let __tmp_173: Value<()> = Rc::new(RefCell::new(1_()));
                    std_pair_const_int__int_::new_1({ __tmp_172.as_pointer() }, {
                        __tmp_173.as_pointer()
                    })
                },
                {
                    let __tmp_174: Value<()> = Rc::new(RefCell::new(2_()));
                    let __tmp_175: Value<()> = Rc::new(RefCell::new(3_()));
                    std_pair_const_int__int_::new_1({ __tmp_174.as_pointer() }, {
                        __tmp_175.as_pointer()
                    })
                },
                {
                    let __tmp_176: Value<()> = Rc::new(RefCell::new(4_()));
                    let __tmp_177: Value<()> = Rc::new(RefCell::new(4_()));
                    std_pair_const_int__int_::new_1({ __tmp_176.as_pointer() }, {
                        __tmp_177.as_pointer()
                    })
                },
                {
                    let __tmp_178: Value<()> = Rc::new(RefCell::new(5_()));
                    let __tmp_179: Value<()> = Rc::new(RefCell::new(2_()));
                    std_pair_const_int__int_::new_1({ __tmp_178.as_pointer() }, {
                        __tmp_179.as_pointer()
                    })
                },
            ];
            let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        ({ AssertEmpty_14Impl::operator_call(&check_alloc_invariant.as_pointer()) });
        {
            let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        ({ AssertEmpty_14Impl::operator_call(&check_alloc_invariant.as_pointer()) });
    }
    {
        {
            let mut lhs_arr: [(); 4] = [
                {
                    let __tmp_180: Value<()> = Rc::new(RefCell::new(1_()));
                    let __tmp_181: Value<()> = Rc::new(RefCell::new(1_()));
                    std_pair_const_int__int_::new_1({ __tmp_180.as_pointer() }, {
                        __tmp_181.as_pointer()
                    })
                },
                {
                    let __tmp_182: Value<()> = Rc::new(RefCell::new(2_()));
                    let __tmp_183: Value<()> = Rc::new(RefCell::new(3_()));
                    std_pair_const_int__int_::new_1({ __tmp_182.as_pointer() }, {
                        __tmp_183.as_pointer()
                    })
                },
                {
                    let __tmp_184: Value<()> = Rc::new(RefCell::new(4_()));
                    let __tmp_185: Value<()> = Rc::new(RefCell::new(4_()));
                    std_pair_const_int__int_::new_1({ __tmp_184.as_pointer() }, {
                        __tmp_185.as_pointer()
                    })
                },
                {
                    let __tmp_186: Value<()> = Rc::new(RefCell::new(5_()));
                    let __tmp_187: Value<()> = Rc::new(RefCell::new(2_()));
                    std_pair_const_int__int_::new_1({ __tmp_186.as_pointer() }, {
                        __tmp_187.as_pointer()
                    })
                },
            ];
            let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let mut rhs_arr: [(); 2] = [
                {
                    let __tmp_188: Value<()> = Rc::new(RefCell::new(10_()));
                    let __tmp_189: Value<()> = Rc::new(RefCell::new(4_()));
                    std_pair_const_int__int_::new_1({ __tmp_188.as_pointer() }, {
                        __tmp_189.as_pointer()
                    })
                },
                {
                    let __tmp_190: Value<()> = Rc::new(RefCell::new(13_()));
                    let __tmp_191: Value<()> = Rc::new(RefCell::new(5_()));
                    std_pair_const_int__int_::new_1({ __tmp_190.as_pointer() }, {
                        __tmp_191.as_pointer()
                    })
                },
            ];
            let copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        ({ AssertEmpty_14Impl::operator_call(&check_alloc_invariant.as_pointer()) });
        {
            let mut lhs_arr: [(); 4] = [
                {
                    let __tmp_192: Value<()> = Rc::new(RefCell::new(1_()));
                    let __tmp_193: Value<()> = Rc::new(RefCell::new(1_()));
                    std_pair_const_int__int_::new_1({ __tmp_192.as_pointer() }, {
                        __tmp_193.as_pointer()
                    })
                },
                {
                    let __tmp_194: Value<()> = Rc::new(RefCell::new(2_()));
                    let __tmp_195: Value<()> = Rc::new(RefCell::new(3_()));
                    std_pair_const_int__int_::new_1({ __tmp_194.as_pointer() }, {
                        __tmp_195.as_pointer()
                    })
                },
                {
                    let __tmp_196: Value<()> = Rc::new(RefCell::new(4_()));
                    let __tmp_197: Value<()> = Rc::new(RefCell::new(4_()));
                    std_pair_const_int__int_::new_1({ __tmp_196.as_pointer() }, {
                        __tmp_197.as_pointer()
                    })
                },
                {
                    let __tmp_198: Value<()> = Rc::new(RefCell::new(5_()));
                    let __tmp_199: Value<()> = Rc::new(RefCell::new(2_()));
                    std_pair_const_int__int_::new_1({ __tmp_198.as_pointer() }, {
                        __tmp_199.as_pointer()
                    })
                },
            ];
            let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let mut rhs_arr: [(); 4] = [
                {
                    let __tmp_200: Value<()> = Rc::new(RefCell::new(10_()));
                    let __tmp_201: Value<()> = Rc::new(RefCell::new(4_()));
                    std_pair_const_int__int_::new_1({ __tmp_200.as_pointer() }, {
                        __tmp_201.as_pointer()
                    })
                },
                {
                    let __tmp_202: Value<()> = Rc::new(RefCell::new(13_()));
                    let __tmp_203: Value<()> = Rc::new(RefCell::new(5_()));
                    std_pair_const_int__int_::new_1({ __tmp_202.as_pointer() }, {
                        __tmp_203.as_pointer()
                    })
                },
                {
                    let __tmp_204: Value<()> = Rc::new(RefCell::new(12_()));
                    let __tmp_205: Value<()> = Rc::new(RefCell::new(324_()));
                    std_pair_const_int__int_::new_1({ __tmp_204.as_pointer() }, {
                        __tmp_205.as_pointer()
                    })
                },
                {
                    let __tmp_206: Value<()> = Rc::new(RefCell::new(0_()));
                    let __tmp_207: Value<()> = Rc::new(RefCell::new(54_()));
                    std_pair_const_int__int_::new_1({ __tmp_206.as_pointer() }, {
                        __tmp_207.as_pointer()
                    })
                },
            ];
            let copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        ({ AssertEmpty_14Impl::operator_call(&check_alloc_invariant.as_pointer()) });
        {
            let mut lhs_arr: [(); 4] = [
                {
                    let __tmp_208: Value<()> = Rc::new(RefCell::new(1_()));
                    let __tmp_209: Value<()> = Rc::new(RefCell::new(1_()));
                    std_pair_const_int__int_::new_1({ __tmp_208.as_pointer() }, {
                        __tmp_209.as_pointer()
                    })
                },
                {
                    let __tmp_210: Value<()> = Rc::new(RefCell::new(2_()));
                    let __tmp_211: Value<()> = Rc::new(RefCell::new(3_()));
                    std_pair_const_int__int_::new_1({ __tmp_210.as_pointer() }, {
                        __tmp_211.as_pointer()
                    })
                },
                {
                    let __tmp_212: Value<()> = Rc::new(RefCell::new(4_()));
                    let __tmp_213: Value<()> = Rc::new(RefCell::new(4_()));
                    std_pair_const_int__int_::new_1({ __tmp_212.as_pointer() }, {
                        __tmp_213.as_pointer()
                    })
                },
                {
                    let __tmp_214: Value<()> = Rc::new(RefCell::new(5_()));
                    let __tmp_215: Value<()> = Rc::new(RefCell::new(2_()));
                    std_pair_const_int__int_::new_1({ __tmp_214.as_pointer() }, {
                        __tmp_215.as_pointer()
                    })
                },
            ];
            let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let mut rhs_arr: [(); 6] = [
                {
                    let __tmp_216: Value<()> = Rc::new(RefCell::new(10_()));
                    let __tmp_217: Value<()> = Rc::new(RefCell::new(4_()));
                    std_pair_const_int__int_::new_1({ __tmp_216.as_pointer() }, {
                        __tmp_217.as_pointer()
                    })
                },
                {
                    let __tmp_218: Value<()> = Rc::new(RefCell::new(13_()));
                    let __tmp_219: Value<()> = Rc::new(RefCell::new(5_()));
                    std_pair_const_int__int_::new_1({ __tmp_218.as_pointer() }, {
                        __tmp_219.as_pointer()
                    })
                },
                {
                    let __tmp_220: Value<()> = Rc::new(RefCell::new(12_()));
                    let __tmp_221: Value<()> = Rc::new(RefCell::new(324_()));
                    std_pair_const_int__int_::new_1({ __tmp_220.as_pointer() }, {
                        __tmp_221.as_pointer()
                    })
                },
                {
                    let __tmp_222: Value<()> = Rc::new(RefCell::new(0_()));
                    let __tmp_223: Value<()> = Rc::new(RefCell::new(54_()));
                    std_pair_const_int__int_::new_1({ __tmp_222.as_pointer() }, {
                        __tmp_223.as_pointer()
                    })
                },
                {
                    let __tmp_224: Value<()> = Rc::new(RefCell::new(50_()));
                    let __tmp_225: Value<()> = Rc::new(RefCell::new(5_()));
                    std_pair_const_int__int_::new_1({ __tmp_224.as_pointer() }, {
                        __tmp_225.as_pointer()
                    })
                },
                {
                    let __tmp_226: Value<()> = Rc::new(RefCell::new(2_()));
                    let __tmp_227: Value<()> = Rc::new(RefCell::new(5_()));
                    std_pair_const_int__int_::new_1({ __tmp_226.as_pointer() }, {
                        __tmp_227.as_pointer()
                    })
                },
            ];
            let copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        ({ AssertEmpty_14Impl::operator_call(&check_alloc_invariant.as_pointer()) });
    }
}
pub fn test_15() {
    ({ test_alloc_11(None, None, None) });
    ({ test_alloc_12(None, None, None) });
    {
        let lhs_allocs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let rhs_allocs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({
            let _lhs_alloc: Value<tracking_allocator_std_pair_const_int__int__> =
                Rc::new(RefCell::new(
                    tracking_allocator_std_pair_const_int__int__::new({ lhs_allocs.as_pointer() }),
                ));
            let _rhs_alloc: Value<tracking_allocator_std_pair_const_int__int__> =
                Rc::new(RefCell::new(
                    tracking_allocator_std_pair_const_int__int__::new({ rhs_allocs.as_pointer() }),
                ));
            let _check_alloc_invariant: AssertEmpty_14 =
                AssertEmpty_14::new({ lhs_allocs.as_pointer() }, { rhs_allocs.as_pointer() });
            test_alloc_13(
                _lhs_alloc.as_pointer(),
                _rhs_alloc.as_pointer(),
                _check_alloc_invariant,
            )
        });
    }
    {
        {
            let mut arr: [(); 3] = [
                {
                    let __tmp_228: Value<()> = Rc::new(RefCell::new(1_()));
                    let __tmp_229: Value<()> = Rc::new(RefCell::new(1_()));
                    std_pair_const_int__int_::new_1({ __tmp_228.as_pointer() }, {
                        __tmp_229.as_pointer()
                    })
                },
                {
                    let __tmp_230: Value<()> = Rc::new(RefCell::new(2_()));
                    let __tmp_231: Value<()> = Rc::new(RefCell::new(2_()));
                    std_pair_const_int__int_::new_1({ __tmp_230.as_pointer() }, {
                        __tmp_231.as_pointer()
                    })
                },
                {
                    let __tmp_232: Value<()> = Rc::new(RefCell::new(3_()));
                    let __tmp_233: Value<()> = Rc::new(RefCell::new(3_()));
                    std_pair_const_int__int_::new_1({ __tmp_232.as_pointer() }, {
                        __tmp_233.as_pointer()
                    })
                },
            ];
            let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        {
            let mut arr: [(); 3] = [
                {
                    let __tmp_234: Value<()> = Rc::new(RefCell::new(1_()));
                    let __tmp_235: Value<()> = Rc::new(RefCell::new(1_()));
                    std_pair_const_int__int_::new_1({ __tmp_234.as_pointer() }, {
                        __tmp_235.as_pointer()
                    })
                },
                {
                    let __tmp_236: Value<()> = Rc::new(RefCell::new(2_()));
                    let __tmp_237: Value<()> = Rc::new(RefCell::new(2_()));
                    std_pair_const_int__int_::new_1({ __tmp_236.as_pointer() }, {
                        __tmp_237.as_pointer()
                    })
                },
                {
                    let __tmp_238: Value<()> = Rc::new(RefCell::new(3_()));
                    let __tmp_239: Value<()> = Rc::new(RefCell::new(3_()));
                    std_pair_const_int__int_::new_1({ __tmp_238.as_pointer() }, {
                        __tmp_239.as_pointer()
                    })
                },
            ];
            let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        {
            let mut arr: [(); 3] = [
                {
                    let __tmp_240: Value<()> = Rc::new(RefCell::new(1_()));
                    let __tmp_241: Value<()> = Rc::new(RefCell::new(1_()));
                    std_pair_const_int__int_::new_1({ __tmp_240.as_pointer() }, {
                        __tmp_241.as_pointer()
                    })
                },
                {
                    let __tmp_242: Value<()> = Rc::new(RefCell::new(2_()));
                    let __tmp_243: Value<()> = Rc::new(RefCell::new(2_()));
                    std_pair_const_int__int_::new_1({ __tmp_242.as_pointer() }, {
                        __tmp_243.as_pointer()
                    })
                },
                {
                    let __tmp_244: Value<()> = Rc::new(RefCell::new(3_()));
                    let __tmp_245: Value<()> = Rc::new(RefCell::new(3_()));
                    std_pair_const_int__int_::new_1({ __tmp_244.as_pointer() }, {
                        __tmp_245.as_pointer()
                    })
                },
            ];
            let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            let copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            let copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        {
            {
                let mut arr: [(); 3] = [
                    {
                        let __tmp_246: Value<()> = Rc::new(RefCell::new(1_()));
                        let __tmp_247: Value<()> = Rc::new(RefCell::new(1_()));
                        std_pair_const_int__int_::new_1({ __tmp_246.as_pointer() }, {
                            __tmp_247.as_pointer()
                        })
                    },
                    {
                        let __tmp_248: Value<()> = Rc::new(RefCell::new(2_()));
                        let __tmp_249: Value<()> = Rc::new(RefCell::new(2_()));
                        std_pair_const_int__int_::new_1({ __tmp_248.as_pointer() }, {
                            __tmp_249.as_pointer()
                        })
                    },
                    {
                        let __tmp_250: Value<()> = Rc::new(RefCell::new(3_()));
                        let __tmp_251: Value<()> = Rc::new(RefCell::new(3_()));
                        std_pair_const_int__int_::new_1({ __tmp_250.as_pointer() }, {
                            __tmp_251.as_pointer()
                        })
                    },
                ];
                let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                unimplemented!();
                unimplemented!();
            }
            {
                let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                unimplemented!();
                unimplemented!();
            }
        }
        {
            {
                let mut arr: [(); 3] = [
                    {
                        let __tmp_252: Value<()> = Rc::new(RefCell::new(1_()));
                        let __tmp_253: Value<()> = Rc::new(RefCell::new(1_()));
                        std_pair_const_int__int_::new_1({ __tmp_252.as_pointer() }, {
                            __tmp_253.as_pointer()
                        })
                    },
                    {
                        let __tmp_254: Value<()> = Rc::new(RefCell::new(2_()));
                        let __tmp_255: Value<()> = Rc::new(RefCell::new(2_()));
                        std_pair_const_int__int_::new_1({ __tmp_254.as_pointer() }, {
                            __tmp_255.as_pointer()
                        })
                    },
                    {
                        let __tmp_256: Value<()> = Rc::new(RefCell::new(3_()));
                        let __tmp_257: Value<()> = Rc::new(RefCell::new(3_()));
                        std_pair_const_int__int_::new_1({ __tmp_256.as_pointer() }, {
                            __tmp_257.as_pointer()
                        })
                    },
                ];
                let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                unimplemented!();
                unimplemented!();
            }
            {
                let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct AssertEmpty_14 {
    #[offset(0)]
    #[byte_size(8)]
    lhs_allocs_: Ptr<()>,
    #[offset(8)]
    #[byte_size(8)]
    rhs_allocs_: Ptr<()>,
}
impl AssertEmpty_14 {
    pub fn new(lhs_allocs: Ptr<()>, rhs_allocs: Ptr<()>) -> Self {
        Self {
            lhs_allocs_: (lhs_allocs).clone(),
            rhs_allocs_: (rhs_allocs).clone(),
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
    ({ test_15() });
    return 0_();
}
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__;
pub trait AssertEmpty_14Impl {
    fn operator_call(&self);
}
impl AssertEmpty_14Impl for Ptr<AssertEmpty_14> {
    fn operator_call(&self) {
        unimplemented!();
        unimplemented!();
    }
}
pub trait NoOpImpl {
    fn operator_call(&self);
}
impl NoOpImpl for Ptr<NoOp> {
    fn operator_call(&self) {}
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
pub trait min_allocator_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0___Impl
{
    fn allocate ( &self , n : ()  , ) -> min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  ;
    fn deallocate(
        &self,
        p : min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
        n: (),
    );
}
impl min_allocator_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0___Impl for Ptr<min_allocator_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0___> {
fn allocate ( &self , mut n : ()  , ) -> min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  { return min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ :: new_1 ( { unimplemented!()   } , )    ;
 } fn deallocate ( &self , mut p : min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  , mut n : ()  , ) { unimplemented!()  ;
 } }
pub trait min_allocator_std_pair_const_int__int__Impl {
    fn allocate(
        &self,
        mut n: (),
    ) -> min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn deallocate(
        &self,
        mut p: min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0__,
        mut n: (),
    ) {
        unimplemented!()
    }
}
impl min_allocator_std_pair_const_int__int__Impl for Ptr<min_allocator_std_pair_const_int__int__> {}
pub trait min_pointer_const_std_pair_const_int__int___std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_arrow(&self) -> Ptr<()>;
    fn operator_inc_9(
        &self,
    ) -> Ptr<min_pointer_const_std_pair_const_int__int___std_integral_constant_unsigned_long__0__>
    {
        unimplemented!()
    }
    fn operator_post_inc_10(
        &self,
        mut _a0: (),
    ) -> min_pointer_const_std_pair_const_int__int___std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_dec_11(
        &self,
    ) -> Ptr<min_pointer_const_std_pair_const_int__int___std_integral_constant_unsigned_long__0__>
    {
        unimplemented!()
    }
    fn operator_post_dec_12(
        &self,
        mut _a0: (),
    ) -> min_pointer_const_std_pair_const_int__int___std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_const_std_pair_const_int__int___std_integral_constant_unsigned_long__0__>
    {
        unimplemented!()
    }
    fn operator_sub_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_const_std_pair_const_int__int___std_integral_constant_unsigned_long__0__>
    {
        unimplemented!()
    }
    fn operator_add(
        &self,
        mut n: (),
    ) -> min_pointer_const_std_pair_const_int__int___std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_sub(
        &self,
        mut n: (),
    ) -> min_pointer_const_std_pair_const_int__int___std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_index(&self, mut n: ()) -> Ptr<()> {
        unimplemented!()
    }
}
impl min_pointer_const_std_pair_const_int__int___std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_const_std_pair_const_int__int___std_integral_constant_unsigned_long__0__>
{
    fn operator_arrow(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
}
pub trait min_pointer_const_void__std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
}
impl min_pointer_const_void__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_const_void__std_integral_constant_unsigned_long__0__>
{
}
pub trait min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__Impl
{
    fn to_(&self) -> ();
    fn operator_deref ( &self , ) -> Ptr< min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__  > ;
    fn operator_arrow ( &self , ) -> Ptr< min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__  >{
        unimplemented!()
    }
    fn operator_inc_8 ( &self , ) -> Ptr< min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  > ;
    fn operator_post_inc_9 ( &self , mut _a0 : ()  , ) -> min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }    fn operator_dec_10 ( &self , ) -> Ptr< min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  >{
        unimplemented!()
    }    fn operator_post_dec_11 ( &self , mut _a0 : ()  , ) -> min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }    fn operator_add_assign ( &self , mut n : ()  , ) -> Ptr< min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  >{
        unimplemented!()
    }    fn operator_sub_assign ( &self , mut n : ()  , ) -> Ptr< min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  >{
        unimplemented!()
    }    fn operator_add ( &self , mut n : ()  , ) -> min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }    fn operator_sub ( &self , mut n : ()  , ) -> min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }
    fn operator_index ( &self , n : ()  , ) -> Ptr< min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__  > ;
}
impl min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__Impl for Ptr<min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__> {
fn to_ ( &self , ) -> ()  { return ! ( ( (* self ) .with(|__s| __s . ptr_ .clone()) ).is_null() )   ;
 } fn operator_deref ( &self , ) -> Ptr< min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__  > { return (* self ) .with(|__s| __s . ptr_ .clone())    ;
 } fn operator_inc_8 ( &self , ) -> Ptr< min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  > { field!((* self ) , ptr_) .with_mut(|__v| __v. prefix_inc ()) ;
 return ((* self )  ).clone()  ;
 } fn operator_index ( &self , mut n : ()  , ) -> Ptr< min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__  > { return (* self ) .with(|__s| __s . ptr_ .clone()) . offset ( ( n ) as isize )   ;
 } }
pub trait min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_arrow(&self) -> Ptr<()>;
    fn operator_inc_8(
        &self,
    ) -> Ptr<min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_inc_9(
        &self,
        mut _a0: (),
    ) -> min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_dec_10(
        &self,
    ) -> Ptr<min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_dec_11(
        &self,
        mut _a0: (),
    ) -> min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_sub_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_add(
        &self,
        mut n: (),
    ) -> min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_sub(
        &self,
        mut n: (),
    ) -> min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_index(&self, mut n: ()) -> Ptr<()> {
        unimplemented!()
    }
}
impl min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0__>
{
    fn operator_arrow(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
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
pub trait other_allocator_std_pair_const_int__int__Impl {
    fn allocate(&self, mut n: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<()>, mut s: ()) {
        unimplemented!()
    }
    fn select_on_container_copy_construction(&self) -> other_allocator_std_pair_const_int__int__ {
        unimplemented!()
    }
    fn get_data(&self) -> () {
        unimplemented!()
    }
}
impl other_allocator_std_pair_const_int__int__Impl
    for Ptr<other_allocator_std_pair_const_int__int__>
{
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
    fn get_data(&self) -> () {
        unimplemented!()
    }
    fn get_id(&self) -> () {
        unimplemented!()
    }
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
        field!((*self), data_).write(destructed_value_1.with(|rc| *rc.borrow()));
        field!((*self), id_).write(destructed_value_1.with(|rc| *rc.borrow()));
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
        field!((*self), data_).write(destructed_value_1.with(|rc| *rc.borrow()));
        field!((*self), id_).write(destructed_value_1.with(|rc| *rc.borrow()));
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
pub trait test_equal_to_int_Impl {
    fn operator_call(&self, a: Ptr<()>, b: Ptr<()>) -> ();
}
impl test_equal_to_int_Impl for Ptr<test_equal_to_int_> {
    fn operator_call(&self, a: Ptr<()>, b: Ptr<()>) -> () {
        return ({ (a.read()) } == { (b.read()) });
    }
}
pub trait test_hash_int_Impl {
    fn operator_call(&self, x: Ptr<()>) -> ();
    fn operator_eq(&self, c: Ptr<test_hash_int_>) -> ();
}
impl test_hash_int_Impl for Ptr<test_hash_int_> {
    fn operator_call(&self, x: Ptr<()>) -> () {
        return ({ (unimplemented!() as std___hash_impl_int__void_)((x.read())) });
    }
    fn operator_eq(&self, c: Ptr<test_hash_int_>) -> () {
        return ({ (*self).with(|__s| __s.data_) } == { c.with(|__s| __s.data_) });
    }
}
pub trait tracking_allocator_std_pair_const_int__int__Impl {
    fn allocate(&self, mut n: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn deallocate(&self, mut ptr: Ptr<()>, mut n: ()) {
        unimplemented!()
    }
}
impl tracking_allocator_std_pair_const_int__int__Impl
    for Ptr<tracking_allocator_std_pair_const_int__int__>
{
}
pub fn __cpp2rust_init_globals() {
    let _ = destructed_value_1.with(|_| ());
    let _ = moved_value_2.with(|_| ());
    let _ = outstanding_bytes_7.with(|_| ());
    let _ = alloc_count_8.with(|_| ());
    let _ = dealloc_count_9.with(|_| ());
    let _ = disable_default_constructor_10.with(|_| ());
}
