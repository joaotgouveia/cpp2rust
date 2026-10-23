extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
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
pub struct min_pointer_std_pair_const_int__double___std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<()>,
}
impl min_pointer_std_pair_const_int__double___std_integral_constant_unsigned_long__0__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: (),
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
pub fn operator_eq_4(
    mut x : min_pointer_std___tree_node_std___value_type_int__double___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_std___tree_node_std___value_type_int__double___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__,
) -> () {
    return ({ (x.ptr_).clone() } == { (y.ptr_).clone() });
}
pub fn operator_ne_5(
    mut x : min_pointer_std___tree_node_std___value_type_int__double___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_std___tree_node_std___value_type_int__double___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__,
) -> () {
    return !({
        let _x: min_pointer_std___tree_node_std___value_type_int__double___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__   = x   ;
        operator_eq_4(_x, y)
    });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_std___tree_node_std___value_type_int__double___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ { #[offset(0)] #[byte_size(8)] ptr_ : Ptr< std___tree_node_std___value_type_int__double___min_pointer_void__std_integral_constant_unsigned_long__0___  >  , }
impl min_pointer_std___tree_node_std___value_type_int__double___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ { fn new_1 ( mut p : Ptr< std___tree_node_std___value_type_int__double___min_pointer_void__std_integral_constant_unsigned_long__0___  >  , ) -> Self { Self { ptr_ : (p ).clone() , }  } pub fn new_2 ( mut _a0 : ()  , ) -> Self { Self { ptr_ : Ptr::<std___tree_node_std___value_type_int__double___min_pointer_void__std_integral_constant_unsigned_long__0___>::null()  , }  } pub fn pointer_to ( t : Ptr< std___tree_node_std___value_type_int__double___min_pointer_void__std_integral_constant_unsigned_long__0___  > , ) -> min_pointer_std___tree_node_std___value_type_int__double___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__  { return min_pointer_std___tree_node_std___value_type_int__double___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ :: new_1 ( { unimplemented!()   } , )    ;
 } }
impl std::cmp::PartialEq for min_pointer_std___tree_node_std___value_type_int__double___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ { fn eq(&self, other: &Self) -> bool {  { operator_eq_4(self.clone(), other.clone()) }}}
impl std::cmp::Eq for min_pointer_std___tree_node_std___value_type_int__double___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ {}
pub fn operator_eq_6(
    mut x : min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__,
) -> () {
    return ({ (x.ptr_).clone() } == { (y.ptr_).clone() });
}
pub fn operator_ne_7(
    mut x : min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__,
) -> () {
    return !({
        let _x: min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__   = x   ;
        operator_eq_6(_x, y)
    });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__
{
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0___>,
}
impl min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ { fn new_1 ( mut p : Ptr< std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0___  >  , ) -> Self { Self { ptr_ : (p ).clone() , }  } pub fn new_2 ( mut _a0 : ()  , ) -> Self { Self { ptr_ : Ptr::<std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0___>::null()  , }  } pub fn pointer_to ( t : Ptr< std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0___  > , ) -> min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__  { return min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ :: new_1 ( { unimplemented!()   } , )    ;
 } }
impl std::cmp::PartialEq for min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ { fn eq(&self, other: &Self) -> bool {  { operator_eq_6(self.clone(), other.clone()) }}}
impl std::cmp::Eq for min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ {}
pub fn operator_eq_8(
    mut x : min_pointer_std___tree_end_node_min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_std___tree_end_node_min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__,
) -> () {
    return ({ (x.ptr_).clone() } == { (y.ptr_).clone() });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_std___tree_end_node_min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ { #[offset(0)] #[byte_size(8)] ptr_ : Ptr< std___tree_end_node_min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0___  >  , }
impl min_pointer_std___tree_end_node_min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ { fn new_1 ( mut p : Ptr< std___tree_end_node_min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0___  >  , ) -> Self { Self { ptr_ : (p ).clone() , }  } pub fn new_2 ( mut _a0 : ()  , ) -> Self { Self { ptr_ : Ptr::<std___tree_end_node_min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0___>::null()  , }  } pub fn pointer_to ( t : Ptr< std___tree_end_node_min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0___  > , ) -> min_pointer_std___tree_end_node_min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__  { return min_pointer_std___tree_end_node_min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ :: new_1 ( { unimplemented!()   } , )    ;
 } }
impl std::cmp::PartialEq for min_pointer_std___tree_end_node_min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ { fn eq(&self, other: &Self) -> bool {  { operator_eq_8(self.clone(), other.clone()) }}}
impl std::cmp::Eq for min_pointer_std___tree_end_node_min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_std_pair_const_int__double__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_std___tree_node_std___value_type_int__double___min_pointer_void__std_integral_constant_unsigned_long__0____
{}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0____
{}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct PrivateConstructor {
    #[offset(0)]
    val: (),
}
impl PrivateConstructor {
    pub fn make(mut v: ()) -> PrivateConstructor {
        return PrivateConstructor::new({ v });
    }
    fn new(mut v: ()) -> Self {
        Self { val: v }
    }
}
impl std::cmp::Ord for PrivateConstructor {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            if operator_lt_9(
                Rc::new(RefCell::new(PrivateConstructor {
                    val: self.val.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(PrivateConstructor {
                    val: other.val.clone(),
                }))
                .as_pointer(),
            ) {
                std::cmp::Ordering::Less
            } else if operator_lt_9(
                Rc::new(RefCell::new(PrivateConstructor {
                    val: other.val.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(PrivateConstructor {
                    val: self.val.clone(),
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
impl std::cmp::PartialOrd for PrivateConstructor {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for PrivateConstructor {
    fn eq(&self, other: &Self) -> bool {
        {
            !(operator_lt_9(
                Rc::new(RefCell::new(PrivateConstructor {
                    val: self.val.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(PrivateConstructor {
                    val: other.val.clone(),
                }))
                .as_pointer(),
            )) && !(operator_lt_9(
                Rc::new(RefCell::new(PrivateConstructor {
                    val: other.val.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(PrivateConstructor {
                    val: self.val.clone(),
                }))
                .as_pointer(),
            ))
        }
    }
}
impl std::cmp::Eq for PrivateConstructor {}
pub fn operator_lt_9(lhs: Ptr<PrivateConstructor>, rhs: Ptr<PrivateConstructor>) -> () {
    return ({ ({ PrivateConstructorImpl::get(&lhs) }) } < {
        ({ PrivateConstructorImpl::get(&rhs) })
    });
}
pub fn operator_lt_10(lhs: Ptr<PrivateConstructor>, mut rhs: ()) -> () {
    return ({ ({ PrivateConstructorImpl::get(&lhs) }) } < { rhs });
}
pub fn operator_lt_11(mut lhs: (), rhs: Ptr<PrivateConstructor>) -> () {
    return ({ lhs } < { ({ PrivateConstructorImpl::get(&rhs) }) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct transparent_less {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct transparent_less_not_referenceable {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct transparent_less_nonempty_is_transparent {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct transparent_less_nonempty {
    #[offset(0)]
    #[byte_size(1)]
    pub pad_: transparent_less_nonempty_is_transparent,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct transparent_less_no_type {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct transparent_less_private {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct transparent_less_not_a_type {
    #[offset(0)]
    pub is_transparent: (),
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct C2Int {
    #[offset(0)]
    i_: (),
}
impl C2Int {
    pub fn new_1() -> Self {
        Self { i_: 0_() }
    }
    pub fn new_2(mut i: ()) -> Self {
        Self { i_: i }
    }
}
impl std::cmp::Ord for C2Int {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            if operator_lt_12(
                Rc::new(RefCell::new(C2Int {
                    i_: self.i_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(C2Int {
                    i_: other.i_.clone(),
                }))
                .as_pointer(),
            ) {
                std::cmp::Ordering::Less
            } else if operator_lt_12(
                Rc::new(RefCell::new(C2Int {
                    i_: other.i_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(C2Int {
                    i_: self.i_.clone(),
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
impl std::cmp::PartialOrd for C2Int {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for C2Int {
    fn eq(&self, other: &Self) -> bool {
        {
            !(operator_lt_12(
                Rc::new(RefCell::new(C2Int {
                    i_: self.i_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(C2Int {
                    i_: other.i_.clone(),
                }))
                .as_pointer(),
            )) && !(operator_lt_12(
                Rc::new(RefCell::new(C2Int {
                    i_: other.i_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(C2Int {
                    i_: self.i_.clone(),
                }))
                .as_pointer(),
            ))
        }
    }
}
impl std::cmp::Eq for C2Int {}
impl Default for C2Int {
    fn default() -> Self {
        { C2Int::new_1() }
    }
}
pub fn operator_lt_13(mut rhs: (), lhs: Ptr<C2Int>) -> () {
    return ({ rhs } < { ({ C2IntImpl::get(&lhs) }) });
}
pub fn operator_lt_12(rhs: Ptr<C2Int>, lhs: Ptr<C2Int>) -> () {
    return ({ ({ C2IntImpl::get(&rhs) }) } < { ({ C2IntImpl::get(&lhs) }) });
}
pub fn operator_lt_14(rhs: Ptr<C2Int>, mut lhs: ()) -> () {
    return ({ ({ C2IntImpl::get(&rhs) }) } < { lhs });
}
pub fn test_15() -> () {
    {
        {
            let ar: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
                {
                    let __tmp_0: Value<()> = Rc::new(RefCell::new(5_()));
                    let __tmp_1: Value<()> = Rc::new(RefCell::new(1_()));
                    std_pair_const_int__double_::new_1({ __tmp_0.as_pointer() }, {
                        __tmp_1.as_pointer()
                    })
                },
                {
                    let __tmp_2: Value<()> = Rc::new(RefCell::new(5_()));
                    let __tmp_3: Value<()> = Rc::new(RefCell::new(2_()));
                    std_pair_const_int__double_::new_1({ __tmp_2.as_pointer() }, {
                        __tmp_3.as_pointer()
                    })
                },
                {
                    let __tmp_4: Value<()> = Rc::new(RefCell::new(5_()));
                    let __tmp_5: Value<()> = Rc::new(RefCell::new(3_()));
                    std_pair_const_int__double_::new_1({ __tmp_4.as_pointer() }, {
                        __tmp_5.as_pointer()
                    })
                },
                {
                    let __tmp_6: Value<()> = Rc::new(RefCell::new(7_()));
                    let __tmp_7: Value<()> = Rc::new(RefCell::new(1_()));
                    std_pair_const_int__double_::new_1({ __tmp_6.as_pointer() }, {
                        __tmp_7.as_pointer()
                    })
                },
                {
                    let __tmp_8: Value<()> = Rc::new(RefCell::new(7_()));
                    let __tmp_9: Value<()> = Rc::new(RefCell::new(2_()));
                    std_pair_const_int__double_::new_1({ __tmp_8.as_pointer() }, {
                        __tmp_9.as_pointer()
                    })
                },
                {
                    let __tmp_10: Value<()> = Rc::new(RefCell::new(7_()));
                    let __tmp_11: Value<()> = Rc::new(RefCell::new(3_()));
                    std_pair_const_int__double_::new_1({ __tmp_10.as_pointer() }, {
                        __tmp_11.as_pointer()
                    })
                },
                {
                    let __tmp_12: Value<()> = Rc::new(RefCell::new(9_()));
                    let __tmp_13: Value<()> = Rc::new(RefCell::new(1_()));
                    std_pair_const_int__double_::new_1({ __tmp_12.as_pointer() }, {
                        __tmp_13.as_pointer()
                    })
                },
                {
                    let __tmp_14: Value<()> = Rc::new(RefCell::new(9_()));
                    let __tmp_15: Value<()> = Rc::new(RefCell::new(2_()));
                    std_pair_const_int__double_::new_1({ __tmp_14.as_pointer() }, {
                        __tmp_15.as_pointer()
                    })
                },
                {
                    let __tmp_16: Value<()> = Rc::new(RefCell::new(9_()));
                    let __tmp_17: Value<()> = Rc::new(RefCell::new(3_()));
                    std_pair_const_int__double_::new_1({ __tmp_16.as_pointer() }, {
                        __tmp_17.as_pointer()
                    })
                },
            ])));
            let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let r: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            (*r.borrow_mut()) = unimplemented!();
            unimplemented!();
            unimplemented!();
            (*r.borrow_mut()) = unimplemented!();
            unimplemented!();
            unimplemented!();
            (*r.borrow_mut()) = unimplemented!();
            unimplemented!();
            unimplemented!();
            (*r.borrow_mut()) = unimplemented!();
            unimplemented!();
            unimplemented!();
            (*r.borrow_mut()) = unimplemented!();
            unimplemented!();
            unimplemented!();
            (*r.borrow_mut()) = unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let ar: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
                {
                    let __tmp_18: Value<()> = Rc::new(RefCell::new(5_()));
                    let __tmp_19: Value<()> = Rc::new(RefCell::new(1_()));
                    std_pair_const_int__double_::new_1({ __tmp_18.as_pointer() }, {
                        __tmp_19.as_pointer()
                    })
                },
                {
                    let __tmp_20: Value<()> = Rc::new(RefCell::new(5_()));
                    let __tmp_21: Value<()> = Rc::new(RefCell::new(2_()));
                    std_pair_const_int__double_::new_1({ __tmp_20.as_pointer() }, {
                        __tmp_21.as_pointer()
                    })
                },
                {
                    let __tmp_22: Value<()> = Rc::new(RefCell::new(5_()));
                    let __tmp_23: Value<()> = Rc::new(RefCell::new(3_()));
                    std_pair_const_int__double_::new_1({ __tmp_22.as_pointer() }, {
                        __tmp_23.as_pointer()
                    })
                },
                {
                    let __tmp_24: Value<()> = Rc::new(RefCell::new(7_()));
                    let __tmp_25: Value<()> = Rc::new(RefCell::new(1_()));
                    std_pair_const_int__double_::new_1({ __tmp_24.as_pointer() }, {
                        __tmp_25.as_pointer()
                    })
                },
                {
                    let __tmp_26: Value<()> = Rc::new(RefCell::new(7_()));
                    let __tmp_27: Value<()> = Rc::new(RefCell::new(2_()));
                    std_pair_const_int__double_::new_1({ __tmp_26.as_pointer() }, {
                        __tmp_27.as_pointer()
                    })
                },
                {
                    let __tmp_28: Value<()> = Rc::new(RefCell::new(7_()));
                    let __tmp_29: Value<()> = Rc::new(RefCell::new(3_()));
                    std_pair_const_int__double_::new_1({ __tmp_28.as_pointer() }, {
                        __tmp_29.as_pointer()
                    })
                },
                {
                    let __tmp_30: Value<()> = Rc::new(RefCell::new(9_()));
                    let __tmp_31: Value<()> = Rc::new(RefCell::new(1_()));
                    std_pair_const_int__double_::new_1({ __tmp_30.as_pointer() }, {
                        __tmp_31.as_pointer()
                    })
                },
                {
                    let __tmp_32: Value<()> = Rc::new(RefCell::new(9_()));
                    let __tmp_33: Value<()> = Rc::new(RefCell::new(2_()));
                    std_pair_const_int__double_::new_1({ __tmp_32.as_pointer() }, {
                        __tmp_33.as_pointer()
                    })
                },
                {
                    let __tmp_34: Value<()> = Rc::new(RefCell::new(9_()));
                    let __tmp_35: Value<()> = Rc::new(RefCell::new(3_()));
                    std_pair_const_int__double_::new_1({ __tmp_34.as_pointer() }, {
                        __tmp_35.as_pointer()
                    })
                },
            ])));
            let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let r: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            (*r.borrow_mut()) = unimplemented!();
            unimplemented!();
            unimplemented!();
            (*r.borrow_mut()) = unimplemented!();
            unimplemented!();
            unimplemented!();
            (*r.borrow_mut()) = unimplemented!();
            unimplemented!();
            unimplemented!();
            (*r.borrow_mut()) = unimplemented!();
            unimplemented!();
            unimplemented!();
            (*r.borrow_mut()) = unimplemented!();
            unimplemented!();
            unimplemented!();
            (*r.borrow_mut()) = unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        {
            let ar: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
                {
                    let __tmp_36: Value<()> = Rc::new(RefCell::new(5_()));
                    let __tmp_37: Value<()> = Rc::new(RefCell::new(1_()));
                    std_pair_const_int__double_::new_1({ __tmp_36.as_pointer() }, {
                        __tmp_37.as_pointer()
                    })
                },
                {
                    let __tmp_38: Value<()> = Rc::new(RefCell::new(5_()));
                    let __tmp_39: Value<()> = Rc::new(RefCell::new(2_()));
                    std_pair_const_int__double_::new_1({ __tmp_38.as_pointer() }, {
                        __tmp_39.as_pointer()
                    })
                },
                {
                    let __tmp_40: Value<()> = Rc::new(RefCell::new(5_()));
                    let __tmp_41: Value<()> = Rc::new(RefCell::new(3_()));
                    std_pair_const_int__double_::new_1({ __tmp_40.as_pointer() }, {
                        __tmp_41.as_pointer()
                    })
                },
                {
                    let __tmp_42: Value<()> = Rc::new(RefCell::new(7_()));
                    let __tmp_43: Value<()> = Rc::new(RefCell::new(1_()));
                    std_pair_const_int__double_::new_1({ __tmp_42.as_pointer() }, {
                        __tmp_43.as_pointer()
                    })
                },
                {
                    let __tmp_44: Value<()> = Rc::new(RefCell::new(7_()));
                    let __tmp_45: Value<()> = Rc::new(RefCell::new(2_()));
                    std_pair_const_int__double_::new_1({ __tmp_44.as_pointer() }, {
                        __tmp_45.as_pointer()
                    })
                },
                {
                    let __tmp_46: Value<()> = Rc::new(RefCell::new(7_()));
                    let __tmp_47: Value<()> = Rc::new(RefCell::new(3_()));
                    std_pair_const_int__double_::new_1({ __tmp_46.as_pointer() }, {
                        __tmp_47.as_pointer()
                    })
                },
                {
                    let __tmp_48: Value<()> = Rc::new(RefCell::new(9_()));
                    let __tmp_49: Value<()> = Rc::new(RefCell::new(1_()));
                    std_pair_const_int__double_::new_1({ __tmp_48.as_pointer() }, {
                        __tmp_49.as_pointer()
                    })
                },
                {
                    let __tmp_50: Value<()> = Rc::new(RefCell::new(9_()));
                    let __tmp_51: Value<()> = Rc::new(RefCell::new(2_()));
                    std_pair_const_int__double_::new_1({ __tmp_50.as_pointer() }, {
                        __tmp_51.as_pointer()
                    })
                },
                {
                    let __tmp_52: Value<()> = Rc::new(RefCell::new(9_()));
                    let __tmp_53: Value<()> = Rc::new(RefCell::new(3_()));
                    std_pair_const_int__double_::new_1({ __tmp_52.as_pointer() }, {
                        __tmp_53.as_pointer()
                    })
                },
            ])));
            let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let r: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            (*r.borrow_mut()) = unimplemented!();
            unimplemented!();
            unimplemented!();
            (*r.borrow_mut()) = unimplemented!();
            unimplemented!();
            unimplemented!();
            (*r.borrow_mut()) = unimplemented!();
            unimplemented!();
            unimplemented!();
            (*r.borrow_mut()) = unimplemented!();
            unimplemented!();
            unimplemented!();
            (*r.borrow_mut()) = unimplemented!();
            unimplemented!();
            unimplemented!();
            (*r.borrow_mut()) = unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let ar: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
                {
                    let __tmp_54: Value<()> = Rc::new(RefCell::new(5_()));
                    let __tmp_55: Value<()> = Rc::new(RefCell::new(1_()));
                    std_pair_const_int__double_::new_1({ __tmp_54.as_pointer() }, {
                        __tmp_55.as_pointer()
                    })
                },
                {
                    let __tmp_56: Value<()> = Rc::new(RefCell::new(5_()));
                    let __tmp_57: Value<()> = Rc::new(RefCell::new(2_()));
                    std_pair_const_int__double_::new_1({ __tmp_56.as_pointer() }, {
                        __tmp_57.as_pointer()
                    })
                },
                {
                    let __tmp_58: Value<()> = Rc::new(RefCell::new(5_()));
                    let __tmp_59: Value<()> = Rc::new(RefCell::new(3_()));
                    std_pair_const_int__double_::new_1({ __tmp_58.as_pointer() }, {
                        __tmp_59.as_pointer()
                    })
                },
                {
                    let __tmp_60: Value<()> = Rc::new(RefCell::new(7_()));
                    let __tmp_61: Value<()> = Rc::new(RefCell::new(1_()));
                    std_pair_const_int__double_::new_1({ __tmp_60.as_pointer() }, {
                        __tmp_61.as_pointer()
                    })
                },
                {
                    let __tmp_62: Value<()> = Rc::new(RefCell::new(7_()));
                    let __tmp_63: Value<()> = Rc::new(RefCell::new(2_()));
                    std_pair_const_int__double_::new_1({ __tmp_62.as_pointer() }, {
                        __tmp_63.as_pointer()
                    })
                },
                {
                    let __tmp_64: Value<()> = Rc::new(RefCell::new(7_()));
                    let __tmp_65: Value<()> = Rc::new(RefCell::new(3_()));
                    std_pair_const_int__double_::new_1({ __tmp_64.as_pointer() }, {
                        __tmp_65.as_pointer()
                    })
                },
                {
                    let __tmp_66: Value<()> = Rc::new(RefCell::new(9_()));
                    let __tmp_67: Value<()> = Rc::new(RefCell::new(1_()));
                    std_pair_const_int__double_::new_1({ __tmp_66.as_pointer() }, {
                        __tmp_67.as_pointer()
                    })
                },
                {
                    let __tmp_68: Value<()> = Rc::new(RefCell::new(9_()));
                    let __tmp_69: Value<()> = Rc::new(RefCell::new(2_()));
                    std_pair_const_int__double_::new_1({ __tmp_68.as_pointer() }, {
                        __tmp_69.as_pointer()
                    })
                },
                {
                    let __tmp_70: Value<()> = Rc::new(RefCell::new(9_()));
                    let __tmp_71: Value<()> = Rc::new(RefCell::new(3_()));
                    std_pair_const_int__double_::new_1({ __tmp_70.as_pointer() }, {
                        __tmp_71.as_pointer()
                    })
                },
            ])));
            let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let r: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            (*r.borrow_mut()) = unimplemented!();
            unimplemented!();
            unimplemented!();
            (*r.borrow_mut()) = unimplemented!();
            unimplemented!();
            unimplemented!();
            (*r.borrow_mut()) = unimplemented!();
            unimplemented!();
            unimplemented!();
            (*r.borrow_mut()) = unimplemented!();
            unimplemented!();
            unimplemented!();
            (*r.borrow_mut()) = unimplemented!();
            unimplemented!();
            unimplemented!();
            (*r.borrow_mut()) = unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let ar: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
            {
                let __tmp_72: Value<()> = Rc::new(RefCell::new(5_()));
                let __tmp_73: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_const_int__double_::new_1({ __tmp_72.as_pointer() }, {
                    __tmp_73.as_pointer()
                })
            },
            {
                let __tmp_74: Value<()> = Rc::new(RefCell::new(5_()));
                let __tmp_75: Value<()> = Rc::new(RefCell::new(2_()));
                std_pair_const_int__double_::new_1({ __tmp_74.as_pointer() }, {
                    __tmp_75.as_pointer()
                })
            },
            {
                let __tmp_76: Value<()> = Rc::new(RefCell::new(5_()));
                let __tmp_77: Value<()> = Rc::new(RefCell::new(3_()));
                std_pair_const_int__double_::new_1({ __tmp_76.as_pointer() }, {
                    __tmp_77.as_pointer()
                })
            },
            {
                let __tmp_78: Value<()> = Rc::new(RefCell::new(7_()));
                let __tmp_79: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_const_int__double_::new_1({ __tmp_78.as_pointer() }, {
                    __tmp_79.as_pointer()
                })
            },
            {
                let __tmp_80: Value<()> = Rc::new(RefCell::new(7_()));
                let __tmp_81: Value<()> = Rc::new(RefCell::new(2_()));
                std_pair_const_int__double_::new_1({ __tmp_80.as_pointer() }, {
                    __tmp_81.as_pointer()
                })
            },
            {
                let __tmp_82: Value<()> = Rc::new(RefCell::new(7_()));
                let __tmp_83: Value<()> = Rc::new(RefCell::new(3_()));
                std_pair_const_int__double_::new_1({ __tmp_82.as_pointer() }, {
                    __tmp_83.as_pointer()
                })
            },
            {
                let __tmp_84: Value<()> = Rc::new(RefCell::new(9_()));
                let __tmp_85: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_const_int__double_::new_1({ __tmp_84.as_pointer() }, {
                    __tmp_85.as_pointer()
                })
            },
            {
                let __tmp_86: Value<()> = Rc::new(RefCell::new(9_()));
                let __tmp_87: Value<()> = Rc::new(RefCell::new(2_()));
                std_pair_const_int__double_::new_1({ __tmp_86.as_pointer() }, {
                    __tmp_87.as_pointer()
                })
            },
            {
                let __tmp_88: Value<()> = Rc::new(RefCell::new(9_()));
                let __tmp_89: Value<()> = Rc::new(RefCell::new(3_()));
                std_pair_const_int__double_::new_1({ __tmp_88.as_pointer() }, {
                    __tmp_89.as_pointer()
                })
            },
        ])));
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let r: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        (*r.borrow_mut()) = unimplemented!();
        unimplemented!();
        unimplemented!();
        (*r.borrow_mut()) = unimplemented!();
        unimplemented!();
        unimplemented!();
        (*r.borrow_mut()) = unimplemented!();
        unimplemented!();
        unimplemented!();
        (*r.borrow_mut()) = unimplemented!();
        unimplemented!();
        unimplemented!();
        (*r.borrow_mut()) = unimplemented!();
        unimplemented!();
        unimplemented!();
        (*r.borrow_mut()) = unimplemented!();
        unimplemented!();
        unimplemented!();
        (*r.borrow_mut()) = ({
            let ___k: Value<C2Int> = Rc::new(RefCell::new(C2Int::new_2({ 4_() })));
            (*m.borrow()).equal_range_82_struct_C2Int_struct_std_less_void__0(___k.as_pointer())
        });
        unimplemented!();
        unimplemented!();
        (*r.borrow_mut()) = ({
            let ___k: Value<C2Int> = Rc::new(RefCell::new(C2Int::new_2({ 5_() })));
            (*m.borrow()).equal_range_82_struct_C2Int_struct_std_less_void__0(___k.as_pointer())
        });
        unimplemented!();
        unimplemented!();
        (*r.borrow_mut()) = ({
            let ___k: Value<C2Int> = Rc::new(RefCell::new(C2Int::new_2({ 6_() })));
            (*m.borrow()).equal_range_82_struct_C2Int_struct_std_less_void__0(___k.as_pointer())
        });
        unimplemented!();
        unimplemented!();
        (*r.borrow_mut()) = ({
            let ___k: Value<C2Int> = Rc::new(RefCell::new(C2Int::new_2({ 7_() })));
            (*m.borrow()).equal_range_82_struct_C2Int_struct_std_less_void__0(___k.as_pointer())
        });
        unimplemented!();
        unimplemented!();
        (*r.borrow_mut()) = ({
            let ___k: Value<C2Int> = Rc::new(RefCell::new(C2Int::new_2({ 8_() })));
            (*m.borrow()).equal_range_82_struct_C2Int_struct_std_less_void__0(___k.as_pointer())
        });
        unimplemented!();
        unimplemented!();
        (*r.borrow_mut()) = ({
            let ___k: Value<C2Int> = Rc::new(RefCell::new(C2Int::new_2({ 9_() })));
            (*m.borrow()).equal_range_82_struct_C2Int_struct_std_less_void__0(___k.as_pointer())
        });
        unimplemented!();
        unimplemented!();
        (*r.borrow_mut()) = ({
            let ___k: Value<C2Int> = Rc::new(RefCell::new(C2Int::new_2({ 10_() })));
            (*m.borrow()).equal_range_82_struct_C2Int_struct_std_less_void__0(___k.as_pointer())
        });
        unimplemented!();
        unimplemented!();
    }
    {
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({
            let ___p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            (*m.borrow())
                .insert_41_struct_std_pair_struct_PrivateConstructor__double__0(___p.as_pointer())
        });
        ({
            let ___p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            (*m.borrow())
                .insert_41_struct_std_pair_struct_PrivateConstructor__double__0(___p.as_pointer())
        });
        ({
            let ___p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            (*m.borrow())
                .insert_41_struct_std_pair_struct_PrivateConstructor__double__0(___p.as_pointer())
        });
        ({
            let ___p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            (*m.borrow())
                .insert_41_struct_std_pair_struct_PrivateConstructor__double__0(___p.as_pointer())
        });
        ({
            let ___p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            (*m.borrow())
                .insert_41_struct_std_pair_struct_PrivateConstructor__double__0(___p.as_pointer())
        });
        ({
            let ___p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            (*m.borrow())
                .insert_41_struct_std_pair_struct_PrivateConstructor__double__0(___p.as_pointer())
        });
        ({
            let ___p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            (*m.borrow())
                .insert_41_struct_std_pair_struct_PrivateConstructor__double__0(___p.as_pointer())
        });
        ({
            let ___p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            (*m.borrow())
                .insert_41_struct_std_pair_struct_PrivateConstructor__double__0(___p.as_pointer())
        });
        ({
            let ___p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            (*m.borrow())
                .insert_41_struct_std_pair_struct_PrivateConstructor__double__0(___p.as_pointer())
        });
        let r: Value<()> = Rc::new(RefCell::new(
            ({
                let ___k: Value<()> = Rc::new(RefCell::new(4_()));
                (*m.borrow()).equal_range_82_int_struct_std_less_void__0(___k.as_pointer())
            }),
        ));
        unimplemented!();
        unimplemented!();
        (*r.borrow_mut()) = ({
            let ___k: Value<()> = Rc::new(RefCell::new(5_()));
            (*m.borrow()).equal_range_82_int_struct_std_less_void__0(___k.as_pointer())
        });
        unimplemented!();
        unimplemented!();
        (*r.borrow_mut()) = ({
            let ___k: Value<()> = Rc::new(RefCell::new(6_()));
            (*m.borrow()).equal_range_82_int_struct_std_less_void__0(___k.as_pointer())
        });
        unimplemented!();
        unimplemented!();
        (*r.borrow_mut()) = ({
            let ___k: Value<()> = Rc::new(RefCell::new(7_()));
            (*m.borrow()).equal_range_82_int_struct_std_less_void__0(___k.as_pointer())
        });
        unimplemented!();
        unimplemented!();
        (*r.borrow_mut()) = ({
            let ___k: Value<()> = Rc::new(RefCell::new(8_()));
            (*m.borrow()).equal_range_82_int_struct_std_less_void__0(___k.as_pointer())
        });
        unimplemented!();
        unimplemented!();
        (*r.borrow_mut()) = ({
            let ___k: Value<()> = Rc::new(RefCell::new(9_()));
            (*m.borrow()).equal_range_82_int_struct_std_less_void__0(___k.as_pointer())
        });
        unimplemented!();
        unimplemented!();
        (*r.borrow_mut()) = ({
            let ___k: Value<()> = Rc::new(RefCell::new(10_()));
            (*m.borrow()).equal_range_82_int_struct_std_less_void__0(___k.as_pointer())
        });
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
    ({ test_15() });
    return 0_();
}
pub trait C2IntImpl {
    fn get(&self) -> ();
}
impl C2IntImpl for Ptr<C2Int> {
    fn get(&self) -> () {
        return (*self).with(|__s| __s.i_);
    }
}
pub trait PrivateConstructorImpl {
    fn get(&self) -> ();
}
impl PrivateConstructorImpl for Ptr<PrivateConstructor> {
    fn get(&self) -> () {
        return (*self).with(|__s| __s.val);
    }
}
pub trait min_allocator_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0____Impl
{
    fn allocate ( &self , mut n : ()  , ) -> min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }
    fn deallocate(
        &self,
        mut p : min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__,
        mut n: (),
    ) {
        unimplemented!()
    }
}
impl min_allocator_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0____Impl for Ptr<min_allocator_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0____> {
}
pub trait min_allocator_std___tree_node_std___value_type_int__double___min_pointer_void__std_integral_constant_unsigned_long__0____Impl
{
    fn allocate ( &self , n : ()  , ) -> min_pointer_std___tree_node_std___value_type_int__double___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__  ;
    fn deallocate(
        &self,
        p : min_pointer_std___tree_node_std___value_type_int__double___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__,
        n: (),
    );
}
impl min_allocator_std___tree_node_std___value_type_int__double___min_pointer_void__std_integral_constant_unsigned_long__0____Impl for Ptr<min_allocator_std___tree_node_std___value_type_int__double___min_pointer_void__std_integral_constant_unsigned_long__0____> {
fn allocate ( &self , mut n : ()  , ) -> min_pointer_std___tree_node_std___value_type_int__double___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__  { return min_pointer_std___tree_node_std___value_type_int__double___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ :: new_1 ( { unimplemented!()   } , )    ;
 } fn deallocate ( &self , mut p : min_pointer_std___tree_node_std___value_type_int__double___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__  , mut n : ()  , ) { unimplemented!()  ;
 } }
pub trait min_allocator_std_pair_const_int__double__Impl {
    fn allocate(
        &self,
        mut n: (),
    ) -> min_pointer_std_pair_const_int__double___std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn deallocate(
        &self,
        mut p: min_pointer_std_pair_const_int__double___std_integral_constant_unsigned_long__0__,
        mut n: (),
    ) {
        unimplemented!()
    }
}
impl min_allocator_std_pair_const_int__double__Impl
    for Ptr<min_allocator_std_pair_const_int__double__>
{
}
pub trait min_pointer_std___tree_end_node_min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__Impl
{
    fn to_(&self) -> ();
    fn operator_deref ( &self , ) -> Ptr< std___tree_end_node_min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0___  > ;
    fn operator_arrow ( &self , ) -> Ptr< std___tree_end_node_min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0___  >  ;
    fn operator_inc_8 ( &self , ) -> Ptr< min_pointer_std___tree_end_node_min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__  >{
        unimplemented!()
    }    fn operator_post_inc_9 ( &self , mut _a0 : ()  , ) -> min_pointer_std___tree_end_node_min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }    fn operator_dec_10 ( &self , ) -> Ptr< min_pointer_std___tree_end_node_min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__  >{
        unimplemented!()
    }    fn operator_post_dec_11 ( &self , mut _a0 : ()  , ) -> min_pointer_std___tree_end_node_min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }    fn operator_add_assign ( &self , mut n : ()  , ) -> Ptr< min_pointer_std___tree_end_node_min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__  >{
        unimplemented!()
    }    fn operator_sub_assign ( &self , mut n : ()  , ) -> Ptr< min_pointer_std___tree_end_node_min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__  >{
        unimplemented!()
    }    fn operator_add ( &self , mut n : ()  , ) -> min_pointer_std___tree_end_node_min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }    fn operator_sub ( &self , mut n : ()  , ) -> min_pointer_std___tree_end_node_min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }    fn operator_index ( &self , mut n : ()  , ) -> Ptr< std___tree_end_node_min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0___  >{
        unimplemented!()
    }
}
impl min_pointer_std___tree_end_node_min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__Impl for Ptr<min_pointer_std___tree_end_node_min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__> {
fn to_ ( &self , ) -> ()  { return ! ( ( (* self ) .with(|__s| __s . ptr_ .clone()) ).is_null() )   ;
 } fn operator_deref ( &self , ) -> Ptr< std___tree_end_node_min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0___  > { return (* self ) .with(|__s| __s . ptr_ .clone())    ;
 } fn operator_arrow ( &self , ) -> Ptr< std___tree_end_node_min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0___  >  { return (* self ) .with(|__s| __s . ptr_ .clone())   ;
 } }
pub trait min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__Impl
{
    fn to_(&self) -> ();
    fn operator_deref(
        &self,
    ) -> Ptr<std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0___>;
    fn operator_arrow(
        &self,
    ) -> Ptr<std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0___>;
    fn operator_inc_8 ( &self , ) -> Ptr< min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__  >{
        unimplemented!()
    }    fn operator_post_inc_9 ( &self , mut _a0 : ()  , ) -> min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }    fn operator_dec_10 ( &self , ) -> Ptr< min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__  >{
        unimplemented!()
    }    fn operator_post_dec_11 ( &self , mut _a0 : ()  , ) -> min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }    fn operator_add_assign ( &self , mut n : ()  , ) -> Ptr< min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__  >{
        unimplemented!()
    }    fn operator_sub_assign ( &self , mut n : ()  , ) -> Ptr< min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__  >{
        unimplemented!()
    }    fn operator_add ( &self , mut n : ()  , ) -> min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }    fn operator_sub ( &self , mut n : ()  , ) -> min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }
    fn operator_index(
        &self,
        mut n: (),
    ) -> Ptr<std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0___> {
        unimplemented!()
    }
}
impl min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__Impl for Ptr<min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__> {
fn to_ ( &self , ) -> ()  { return ! ( ( (* self ) .with(|__s| __s . ptr_ .clone()) ).is_null() )   ;
 } fn operator_deref ( &self , ) -> Ptr< std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0___  > { return (* self ) .with(|__s| __s . ptr_ .clone())    ;
 } fn operator_arrow ( &self , ) -> Ptr< std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0___  >  { return (* self ) .with(|__s| __s . ptr_ .clone())   ;
 } }
pub trait min_pointer_std___tree_node_std___value_type_int__double___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__Impl
{
    fn to_(&self) -> ();
    fn operator_deref ( &self , ) -> Ptr< std___tree_node_std___value_type_int__double___min_pointer_void__std_integral_constant_unsigned_long__0___  > ;
    fn operator_arrow ( &self , ) -> Ptr< std___tree_node_std___value_type_int__double___min_pointer_void__std_integral_constant_unsigned_long__0___  >  ;
    fn operator_inc_8 ( &self , ) -> Ptr< min_pointer_std___tree_node_std___value_type_int__double___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__  >{
        unimplemented!()
    }    fn operator_post_inc_9 ( &self , mut _a0 : ()  , ) -> min_pointer_std___tree_node_std___value_type_int__double___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }    fn operator_dec_10 ( &self , ) -> Ptr< min_pointer_std___tree_node_std___value_type_int__double___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__  >{
        unimplemented!()
    }    fn operator_post_dec_11 ( &self , mut _a0 : ()  , ) -> min_pointer_std___tree_node_std___value_type_int__double___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }    fn operator_add_assign ( &self , mut n : ()  , ) -> Ptr< min_pointer_std___tree_node_std___value_type_int__double___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__  >{
        unimplemented!()
    }    fn operator_sub_assign ( &self , mut n : ()  , ) -> Ptr< min_pointer_std___tree_node_std___value_type_int__double___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__  >{
        unimplemented!()
    }    fn operator_add ( &self , mut n : ()  , ) -> min_pointer_std___tree_node_std___value_type_int__double___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }    fn operator_sub ( &self , mut n : ()  , ) -> min_pointer_std___tree_node_std___value_type_int__double___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }    fn operator_index ( &self , mut n : ()  , ) -> Ptr< std___tree_node_std___value_type_int__double___min_pointer_void__std_integral_constant_unsigned_long__0___  >{
        unimplemented!()
    }
}
impl min_pointer_std___tree_node_std___value_type_int__double___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__Impl for Ptr<min_pointer_std___tree_node_std___value_type_int__double___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__> {
fn to_ ( &self , ) -> ()  { return ! ( ( (* self ) .with(|__s| __s . ptr_ .clone()) ).is_null() )   ;
 } fn operator_deref ( &self , ) -> Ptr< std___tree_node_std___value_type_int__double___min_pointer_void__std_integral_constant_unsigned_long__0___  > { return (* self ) .with(|__s| __s . ptr_ .clone())    ;
 } fn operator_arrow ( &self , ) -> Ptr< std___tree_node_std___value_type_int__double___min_pointer_void__std_integral_constant_unsigned_long__0___  >  { return (* self ) .with(|__s| __s . ptr_ .clone())   ;
 } }
pub trait min_pointer_std_pair_const_int__double___std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_arrow(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_inc_8(
        &self,
    ) -> Ptr<min_pointer_std_pair_const_int__double___std_integral_constant_unsigned_long__0__>
    {
        unimplemented!()
    }
    fn operator_post_inc_9(
        &self,
        mut _a0: (),
    ) -> min_pointer_std_pair_const_int__double___std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_dec_10(
        &self,
    ) -> Ptr<min_pointer_std_pair_const_int__double___std_integral_constant_unsigned_long__0__>
    {
        unimplemented!()
    }
    fn operator_post_dec_11(
        &self,
        mut _a0: (),
    ) -> min_pointer_std_pair_const_int__double___std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_std_pair_const_int__double___std_integral_constant_unsigned_long__0__>
    {
        unimplemented!()
    }
    fn operator_sub_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_std_pair_const_int__double___std_integral_constant_unsigned_long__0__>
    {
        unimplemented!()
    }
    fn operator_add(
        &self,
        mut n: (),
    ) -> min_pointer_std_pair_const_int__double___std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_sub(
        &self,
        mut n: (),
    ) -> min_pointer_std_pair_const_int__double___std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_index(&self, mut n: ()) -> Ptr<()> {
        unimplemented!()
    }
}
impl min_pointer_std_pair_const_int__double___std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_std_pair_const_int__double___std_integral_constant_unsigned_long__0__>
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
pub fn __cpp2rust_init_globals() {
    let _ = outstanding_bytes_0.with(|_| ());
    let _ = alloc_count_1.with(|_| ());
    let _ = dealloc_count_2.with(|_| ());
    let _ = disable_default_constructor_3.with(|_| ());
}
