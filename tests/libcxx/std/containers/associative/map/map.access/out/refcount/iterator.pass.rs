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
impl min_pointer_std_pair_const_int__double___std_integral_constant_unsigned_long__0__ {
    fn new_1(mut p: Ptr<()>) -> Self {
        Self { ptr_: (p).clone() }
    }
    pub fn pointer_to(
        t: Ptr<()>,
    ) -> min_pointer_std_pair_const_int__double___std_integral_constant_unsigned_long__0__ {
        return min_pointer_std_pair_const_int__double___std_integral_constant_unsigned_long__0__ :: new_1 ( { unimplemented!()   } , )    ;
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
pub struct min_pointer_const_std_pair_const_int__double___std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<()>,
}
impl min_pointer_const_std_pair_const_int__double___std_integral_constant_unsigned_long__0__ {
    fn new_1(mut p: Ptr<()>) -> Self {
        Self { ptr_: (p).clone() }
    }
    pub fn pointer_to(
        t: Ptr<()>,
    ) -> min_pointer_const_std_pair_const_int__double___std_integral_constant_unsigned_long__0__
    {
        return min_pointer_const_std_pair_const_int__double___std_integral_constant_unsigned_long__0__ :: new_1 ( { unimplemented!()   } , )    ;
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
pub fn operator_eq_4(
    mut x : min_pointer_std___tree_node_std___value_type_int__double___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_std___tree_node_std___value_type_int__double___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__,
) -> () {
    return ({ (x.ptr_).clone() } == { (y.ptr_).clone() });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_std___tree_node_std___value_type_int__double___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ { #[offset(0)] #[byte_size(8)] ptr_ : Ptr< std___tree_node_std___value_type_int__double___min_pointer_void__std_integral_constant_unsigned_long__0___  >  , }
impl min_pointer_std___tree_node_std___value_type_int__double___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ { fn new_1 ( mut p : Ptr< std___tree_node_std___value_type_int__double___min_pointer_void__std_integral_constant_unsigned_long__0___  >  , ) -> Self { Self { ptr_ : (p ).clone() , }  } pub fn new_2 ( mut _a0 : ()  , ) -> Self { Self { ptr_ : Ptr::<std___tree_node_std___value_type_int__double___min_pointer_void__std_integral_constant_unsigned_long__0___>::null()  , }  } pub fn pointer_to ( t : Ptr< std___tree_node_std___value_type_int__double___min_pointer_void__std_integral_constant_unsigned_long__0___  > , ) -> min_pointer_std___tree_node_std___value_type_int__double___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__  { return min_pointer_std___tree_node_std___value_type_int__double___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ :: new_1 ( { unimplemented!()   } , )    ;
 } }
impl std::cmp::PartialEq for min_pointer_std___tree_node_std___value_type_int__double___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ { fn eq(&self, other: &Self) -> bool {  { operator_eq_4(self.clone(), other.clone()) }}}
impl std::cmp::Eq for min_pointer_std___tree_node_std___value_type_int__double___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ {}
pub fn operator_eq_5(
    mut x : min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__,
) -> () {
    return ({ (x.ptr_).clone() } == { (y.ptr_).clone() });
}
pub fn operator_ne_6(
    mut x : min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__,
) -> () {
    return !({
        let _x: min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__   = x   ;
        operator_eq_5(_x, y)
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
impl std::cmp::PartialEq for min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ { fn eq(&self, other: &Self) -> bool {  { operator_eq_5(self.clone(), other.clone()) }}}
impl std::cmp::Eq for min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ {}
pub fn operator_eq_7(
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
impl std::cmp::PartialEq for min_pointer_std___tree_end_node_min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ { fn eq(&self, other: &Self) -> bool {  { operator_eq_7(self.clone(), other.clone()) }}}
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
pub fn test_8() -> () {
    {
        let ar: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
            {
                let __tmp_0: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_1: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_const_int__double_::new_1({ __tmp_0.as_pointer() }, {
                    __tmp_1.as_pointer()
                })
            },
            {
                let __tmp_2: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_3: Value<()> = Rc::new(RefCell::new(1.5_f64));
                std_pair_const_int__double_::new_2({ __tmp_2.as_pointer() }, {
                    __tmp_3.as_pointer()
                })
            },
            {
                let __tmp_4: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_5: Value<()> = Rc::new(RefCell::new(2_()));
                std_pair_const_int__double_::new_1({ __tmp_4.as_pointer() }, {
                    __tmp_5.as_pointer()
                })
            },
            {
                let __tmp_6: Value<()> = Rc::new(RefCell::new(2_()));
                let __tmp_7: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_const_int__double_::new_1({ __tmp_6.as_pointer() }, {
                    __tmp_7.as_pointer()
                })
            },
            {
                let __tmp_8: Value<()> = Rc::new(RefCell::new(2_()));
                let __tmp_9: Value<()> = Rc::new(RefCell::new(1.5_f64));
                std_pair_const_int__double_::new_2({ __tmp_8.as_pointer() }, {
                    __tmp_9.as_pointer()
                })
            },
            {
                let __tmp_10: Value<()> = Rc::new(RefCell::new(2_()));
                let __tmp_11: Value<()> = Rc::new(RefCell::new(2_()));
                std_pair_const_int__double_::new_1({ __tmp_10.as_pointer() }, {
                    __tmp_11.as_pointer()
                })
            },
            {
                let __tmp_12: Value<()> = Rc::new(RefCell::new(3_()));
                let __tmp_13: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_const_int__double_::new_1({ __tmp_12.as_pointer() }, {
                    __tmp_13.as_pointer()
                })
            },
            {
                let __tmp_14: Value<()> = Rc::new(RefCell::new(3_()));
                let __tmp_15: Value<()> = Rc::new(RefCell::new(1.5_f64));
                std_pair_const_int__double_::new_2({ __tmp_14.as_pointer() }, {
                    __tmp_15.as_pointer()
                })
            },
            {
                let __tmp_16: Value<()> = Rc::new(RefCell::new(3_()));
                let __tmp_17: Value<()> = Rc::new(RefCell::new(2_()));
                std_pair_const_int__double_::new_1({ __tmp_16.as_pointer() }, {
                    __tmp_17.as_pointer()
                })
            },
            {
                let __tmp_18: Value<()> = Rc::new(RefCell::new(4_()));
                let __tmp_19: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_const_int__double_::new_1({ __tmp_18.as_pointer() }, {
                    __tmp_19.as_pointer()
                })
            },
            {
                let __tmp_20: Value<()> = Rc::new(RefCell::new(4_()));
                let __tmp_21: Value<()> = Rc::new(RefCell::new(1.5_f64));
                std_pair_const_int__double_::new_2({ __tmp_20.as_pointer() }, {
                    __tmp_21.as_pointer()
                })
            },
            {
                let __tmp_22: Value<()> = Rc::new(RefCell::new(4_()));
                let __tmp_23: Value<()> = Rc::new(RefCell::new(2_()));
                std_pair_const_int__double_::new_1({ __tmp_22.as_pointer() }, {
                    __tmp_23.as_pointer()
                })
            },
            {
                let __tmp_24: Value<()> = Rc::new(RefCell::new(5_()));
                let __tmp_25: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_const_int__double_::new_1({ __tmp_24.as_pointer() }, {
                    __tmp_25.as_pointer()
                })
            },
            {
                let __tmp_26: Value<()> = Rc::new(RefCell::new(5_()));
                let __tmp_27: Value<()> = Rc::new(RefCell::new(1.5_f64));
                std_pair_const_int__double_::new_2({ __tmp_26.as_pointer() }, {
                    __tmp_27.as_pointer()
                })
            },
            {
                let __tmp_28: Value<()> = Rc::new(RefCell::new(5_()));
                let __tmp_29: Value<()> = Rc::new(RefCell::new(2_()));
                std_pair_const_int__double_::new_1({ __tmp_28.as_pointer() }, {
                    __tmp_29.as_pointer()
                })
            },
            {
                let __tmp_30: Value<()> = Rc::new(RefCell::new(6_()));
                let __tmp_31: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_const_int__double_::new_1({ __tmp_30.as_pointer() }, {
                    __tmp_31.as_pointer()
                })
            },
            {
                let __tmp_32: Value<()> = Rc::new(RefCell::new(6_()));
                let __tmp_33: Value<()> = Rc::new(RefCell::new(1.5_f64));
                std_pair_const_int__double_::new_2({ __tmp_32.as_pointer() }, {
                    __tmp_33.as_pointer()
                })
            },
            {
                let __tmp_34: Value<()> = Rc::new(RefCell::new(6_()));
                let __tmp_35: Value<()> = Rc::new(RefCell::new(2_()));
                std_pair_const_int__double_::new_1({ __tmp_34.as_pointer() }, {
                    __tmp_35.as_pointer()
                })
            },
            {
                let __tmp_36: Value<()> = Rc::new(RefCell::new(7_()));
                let __tmp_37: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_const_int__double_::new_1({ __tmp_36.as_pointer() }, {
                    __tmp_37.as_pointer()
                })
            },
            {
                let __tmp_38: Value<()> = Rc::new(RefCell::new(7_()));
                let __tmp_39: Value<()> = Rc::new(RefCell::new(1.5_f64));
                std_pair_const_int__double_::new_2({ __tmp_38.as_pointer() }, {
                    __tmp_39.as_pointer()
                })
            },
            {
                let __tmp_40: Value<()> = Rc::new(RefCell::new(7_()));
                let __tmp_41: Value<()> = Rc::new(RefCell::new(2_()));
                std_pair_const_int__double_::new_1({ __tmp_40.as_pointer() }, {
                    __tmp_41.as_pointer()
                })
            },
            {
                let __tmp_42: Value<()> = Rc::new(RefCell::new(8_()));
                let __tmp_43: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_const_int__double_::new_1({ __tmp_42.as_pointer() }, {
                    __tmp_43.as_pointer()
                })
            },
            {
                let __tmp_44: Value<()> = Rc::new(RefCell::new(8_()));
                let __tmp_45: Value<()> = Rc::new(RefCell::new(1.5_f64));
                std_pair_const_int__double_::new_2({ __tmp_44.as_pointer() }, {
                    __tmp_45.as_pointer()
                })
            },
            {
                let __tmp_46: Value<()> = Rc::new(RefCell::new(8_()));
                let __tmp_47: Value<()> = Rc::new(RefCell::new(2_()));
                std_pair_const_int__double_::new_1({ __tmp_46.as_pointer() }, {
                    __tmp_47.as_pointer()
                })
            },
        ])));
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        let i: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        let k : Value<() > = Rc::new(RefCell::new(std___map_const_iterator_std___tree_const_iterator_std___value_type_int__double___std___tree_node_std___value_type_int__double___void_ptr__ptr__long__ :: new_1 ( { ((*i.borrow()) ).clone()  } , )  )) ;
        unimplemented!();
        let mut j: () = 1_();
        'loop_: while (j <= unimplemented!()) {
            unimplemented!();
            unimplemented!();
            field!((*i.borrow_mut()), second).write(2.5_f64);
            unimplemented!();
            {
                j.prefix_inc();
                unimplemented!()
            };
        }
        unimplemented!();
        let mut j: () = unimplemented!();
        'loop_: while (j >= 1_()) {
            unimplemented!();
            unimplemented!();
            unimplemented!();
            field!((*i.borrow_mut()), second).write(1_());
            unimplemented!();
            j.prefix_dec();
        }
        unimplemented!();
    }
    {
        let ar: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
            {
                let __tmp_48: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_49: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_const_int__double_::new_1({ __tmp_48.as_pointer() }, {
                    __tmp_49.as_pointer()
                })
            },
            {
                let __tmp_50: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_51: Value<()> = Rc::new(RefCell::new(1.5_f64));
                std_pair_const_int__double_::new_2({ __tmp_50.as_pointer() }, {
                    __tmp_51.as_pointer()
                })
            },
            {
                let __tmp_52: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_53: Value<()> = Rc::new(RefCell::new(2_()));
                std_pair_const_int__double_::new_1({ __tmp_52.as_pointer() }, {
                    __tmp_53.as_pointer()
                })
            },
            {
                let __tmp_54: Value<()> = Rc::new(RefCell::new(2_()));
                let __tmp_55: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_const_int__double_::new_1({ __tmp_54.as_pointer() }, {
                    __tmp_55.as_pointer()
                })
            },
            {
                let __tmp_56: Value<()> = Rc::new(RefCell::new(2_()));
                let __tmp_57: Value<()> = Rc::new(RefCell::new(1.5_f64));
                std_pair_const_int__double_::new_2({ __tmp_56.as_pointer() }, {
                    __tmp_57.as_pointer()
                })
            },
            {
                let __tmp_58: Value<()> = Rc::new(RefCell::new(2_()));
                let __tmp_59: Value<()> = Rc::new(RefCell::new(2_()));
                std_pair_const_int__double_::new_1({ __tmp_58.as_pointer() }, {
                    __tmp_59.as_pointer()
                })
            },
            {
                let __tmp_60: Value<()> = Rc::new(RefCell::new(3_()));
                let __tmp_61: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_const_int__double_::new_1({ __tmp_60.as_pointer() }, {
                    __tmp_61.as_pointer()
                })
            },
            {
                let __tmp_62: Value<()> = Rc::new(RefCell::new(3_()));
                let __tmp_63: Value<()> = Rc::new(RefCell::new(1.5_f64));
                std_pair_const_int__double_::new_2({ __tmp_62.as_pointer() }, {
                    __tmp_63.as_pointer()
                })
            },
            {
                let __tmp_64: Value<()> = Rc::new(RefCell::new(3_()));
                let __tmp_65: Value<()> = Rc::new(RefCell::new(2_()));
                std_pair_const_int__double_::new_1({ __tmp_64.as_pointer() }, {
                    __tmp_65.as_pointer()
                })
            },
            {
                let __tmp_66: Value<()> = Rc::new(RefCell::new(4_()));
                let __tmp_67: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_const_int__double_::new_1({ __tmp_66.as_pointer() }, {
                    __tmp_67.as_pointer()
                })
            },
            {
                let __tmp_68: Value<()> = Rc::new(RefCell::new(4_()));
                let __tmp_69: Value<()> = Rc::new(RefCell::new(1.5_f64));
                std_pair_const_int__double_::new_2({ __tmp_68.as_pointer() }, {
                    __tmp_69.as_pointer()
                })
            },
            {
                let __tmp_70: Value<()> = Rc::new(RefCell::new(4_()));
                let __tmp_71: Value<()> = Rc::new(RefCell::new(2_()));
                std_pair_const_int__double_::new_1({ __tmp_70.as_pointer() }, {
                    __tmp_71.as_pointer()
                })
            },
            {
                let __tmp_72: Value<()> = Rc::new(RefCell::new(5_()));
                let __tmp_73: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_const_int__double_::new_1({ __tmp_72.as_pointer() }, {
                    __tmp_73.as_pointer()
                })
            },
            {
                let __tmp_74: Value<()> = Rc::new(RefCell::new(5_()));
                let __tmp_75: Value<()> = Rc::new(RefCell::new(1.5_f64));
                std_pair_const_int__double_::new_2({ __tmp_74.as_pointer() }, {
                    __tmp_75.as_pointer()
                })
            },
            {
                let __tmp_76: Value<()> = Rc::new(RefCell::new(5_()));
                let __tmp_77: Value<()> = Rc::new(RefCell::new(2_()));
                std_pair_const_int__double_::new_1({ __tmp_76.as_pointer() }, {
                    __tmp_77.as_pointer()
                })
            },
            {
                let __tmp_78: Value<()> = Rc::new(RefCell::new(6_()));
                let __tmp_79: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_const_int__double_::new_1({ __tmp_78.as_pointer() }, {
                    __tmp_79.as_pointer()
                })
            },
            {
                let __tmp_80: Value<()> = Rc::new(RefCell::new(6_()));
                let __tmp_81: Value<()> = Rc::new(RefCell::new(1.5_f64));
                std_pair_const_int__double_::new_2({ __tmp_80.as_pointer() }, {
                    __tmp_81.as_pointer()
                })
            },
            {
                let __tmp_82: Value<()> = Rc::new(RefCell::new(6_()));
                let __tmp_83: Value<()> = Rc::new(RefCell::new(2_()));
                std_pair_const_int__double_::new_1({ __tmp_82.as_pointer() }, {
                    __tmp_83.as_pointer()
                })
            },
            {
                let __tmp_84: Value<()> = Rc::new(RefCell::new(7_()));
                let __tmp_85: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_const_int__double_::new_1({ __tmp_84.as_pointer() }, {
                    __tmp_85.as_pointer()
                })
            },
            {
                let __tmp_86: Value<()> = Rc::new(RefCell::new(7_()));
                let __tmp_87: Value<()> = Rc::new(RefCell::new(1.5_f64));
                std_pair_const_int__double_::new_2({ __tmp_86.as_pointer() }, {
                    __tmp_87.as_pointer()
                })
            },
            {
                let __tmp_88: Value<()> = Rc::new(RefCell::new(7_()));
                let __tmp_89: Value<()> = Rc::new(RefCell::new(2_()));
                std_pair_const_int__double_::new_1({ __tmp_88.as_pointer() }, {
                    __tmp_89.as_pointer()
                })
            },
            {
                let __tmp_90: Value<()> = Rc::new(RefCell::new(8_()));
                let __tmp_91: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_const_int__double_::new_1({ __tmp_90.as_pointer() }, {
                    __tmp_91.as_pointer()
                })
            },
            {
                let __tmp_92: Value<()> = Rc::new(RefCell::new(8_()));
                let __tmp_93: Value<()> = Rc::new(RefCell::new(1.5_f64));
                std_pair_const_int__double_::new_2({ __tmp_92.as_pointer() }, {
                    __tmp_93.as_pointer()
                })
            },
            {
                let __tmp_94: Value<()> = Rc::new(RefCell::new(8_()));
                let __tmp_95: Value<()> = Rc::new(RefCell::new(2_()));
                std_pair_const_int__double_::new_1({ __tmp_94.as_pointer() }, {
                    __tmp_95.as_pointer()
                })
            },
        ])));
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let i: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        let mut j: () = 1_();
        'loop_: while (j <= unimplemented!()) {
            unimplemented!();
            unimplemented!();
            {
                j.prefix_inc();
                unimplemented!()
            };
        }
        unimplemented!();
        let mut j: () = unimplemented!();
        'loop_: while (j >= 1_()) {
            unimplemented!();
            unimplemented!();
            unimplemented!();
            j.prefix_dec();
        }
        unimplemented!();
    }
    {
        let ar: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
            {
                let __tmp_96: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_97: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_const_int__double_::new_1({ __tmp_96.as_pointer() }, {
                    __tmp_97.as_pointer()
                })
            },
            {
                let __tmp_98: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_99: Value<()> = Rc::new(RefCell::new(1.5_f64));
                std_pair_const_int__double_::new_2({ __tmp_98.as_pointer() }, {
                    __tmp_99.as_pointer()
                })
            },
            {
                let __tmp_100: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_101: Value<()> = Rc::new(RefCell::new(2_()));
                std_pair_const_int__double_::new_1({ __tmp_100.as_pointer() }, {
                    __tmp_101.as_pointer()
                })
            },
            {
                let __tmp_102: Value<()> = Rc::new(RefCell::new(2_()));
                let __tmp_103: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_const_int__double_::new_1({ __tmp_102.as_pointer() }, {
                    __tmp_103.as_pointer()
                })
            },
            {
                let __tmp_104: Value<()> = Rc::new(RefCell::new(2_()));
                let __tmp_105: Value<()> = Rc::new(RefCell::new(1.5_f64));
                std_pair_const_int__double_::new_2({ __tmp_104.as_pointer() }, {
                    __tmp_105.as_pointer()
                })
            },
            {
                let __tmp_106: Value<()> = Rc::new(RefCell::new(2_()));
                let __tmp_107: Value<()> = Rc::new(RefCell::new(2_()));
                std_pair_const_int__double_::new_1({ __tmp_106.as_pointer() }, {
                    __tmp_107.as_pointer()
                })
            },
            {
                let __tmp_108: Value<()> = Rc::new(RefCell::new(3_()));
                let __tmp_109: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_const_int__double_::new_1({ __tmp_108.as_pointer() }, {
                    __tmp_109.as_pointer()
                })
            },
            {
                let __tmp_110: Value<()> = Rc::new(RefCell::new(3_()));
                let __tmp_111: Value<()> = Rc::new(RefCell::new(1.5_f64));
                std_pair_const_int__double_::new_2({ __tmp_110.as_pointer() }, {
                    __tmp_111.as_pointer()
                })
            },
            {
                let __tmp_112: Value<()> = Rc::new(RefCell::new(3_()));
                let __tmp_113: Value<()> = Rc::new(RefCell::new(2_()));
                std_pair_const_int__double_::new_1({ __tmp_112.as_pointer() }, {
                    __tmp_113.as_pointer()
                })
            },
            {
                let __tmp_114: Value<()> = Rc::new(RefCell::new(4_()));
                let __tmp_115: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_const_int__double_::new_1({ __tmp_114.as_pointer() }, {
                    __tmp_115.as_pointer()
                })
            },
            {
                let __tmp_116: Value<()> = Rc::new(RefCell::new(4_()));
                let __tmp_117: Value<()> = Rc::new(RefCell::new(1.5_f64));
                std_pair_const_int__double_::new_2({ __tmp_116.as_pointer() }, {
                    __tmp_117.as_pointer()
                })
            },
            {
                let __tmp_118: Value<()> = Rc::new(RefCell::new(4_()));
                let __tmp_119: Value<()> = Rc::new(RefCell::new(2_()));
                std_pair_const_int__double_::new_1({ __tmp_118.as_pointer() }, {
                    __tmp_119.as_pointer()
                })
            },
            {
                let __tmp_120: Value<()> = Rc::new(RefCell::new(5_()));
                let __tmp_121: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_const_int__double_::new_1({ __tmp_120.as_pointer() }, {
                    __tmp_121.as_pointer()
                })
            },
            {
                let __tmp_122: Value<()> = Rc::new(RefCell::new(5_()));
                let __tmp_123: Value<()> = Rc::new(RefCell::new(1.5_f64));
                std_pair_const_int__double_::new_2({ __tmp_122.as_pointer() }, {
                    __tmp_123.as_pointer()
                })
            },
            {
                let __tmp_124: Value<()> = Rc::new(RefCell::new(5_()));
                let __tmp_125: Value<()> = Rc::new(RefCell::new(2_()));
                std_pair_const_int__double_::new_1({ __tmp_124.as_pointer() }, {
                    __tmp_125.as_pointer()
                })
            },
            {
                let __tmp_126: Value<()> = Rc::new(RefCell::new(6_()));
                let __tmp_127: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_const_int__double_::new_1({ __tmp_126.as_pointer() }, {
                    __tmp_127.as_pointer()
                })
            },
            {
                let __tmp_128: Value<()> = Rc::new(RefCell::new(6_()));
                let __tmp_129: Value<()> = Rc::new(RefCell::new(1.5_f64));
                std_pair_const_int__double_::new_2({ __tmp_128.as_pointer() }, {
                    __tmp_129.as_pointer()
                })
            },
            {
                let __tmp_130: Value<()> = Rc::new(RefCell::new(6_()));
                let __tmp_131: Value<()> = Rc::new(RefCell::new(2_()));
                std_pair_const_int__double_::new_1({ __tmp_130.as_pointer() }, {
                    __tmp_131.as_pointer()
                })
            },
            {
                let __tmp_132: Value<()> = Rc::new(RefCell::new(7_()));
                let __tmp_133: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_const_int__double_::new_1({ __tmp_132.as_pointer() }, {
                    __tmp_133.as_pointer()
                })
            },
            {
                let __tmp_134: Value<()> = Rc::new(RefCell::new(7_()));
                let __tmp_135: Value<()> = Rc::new(RefCell::new(1.5_f64));
                std_pair_const_int__double_::new_2({ __tmp_134.as_pointer() }, {
                    __tmp_135.as_pointer()
                })
            },
            {
                let __tmp_136: Value<()> = Rc::new(RefCell::new(7_()));
                let __tmp_137: Value<()> = Rc::new(RefCell::new(2_()));
                std_pair_const_int__double_::new_1({ __tmp_136.as_pointer() }, {
                    __tmp_137.as_pointer()
                })
            },
            {
                let __tmp_138: Value<()> = Rc::new(RefCell::new(8_()));
                let __tmp_139: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_const_int__double_::new_1({ __tmp_138.as_pointer() }, {
                    __tmp_139.as_pointer()
                })
            },
            {
                let __tmp_140: Value<()> = Rc::new(RefCell::new(8_()));
                let __tmp_141: Value<()> = Rc::new(RefCell::new(1.5_f64));
                std_pair_const_int__double_::new_2({ __tmp_140.as_pointer() }, {
                    __tmp_141.as_pointer()
                })
            },
            {
                let __tmp_142: Value<()> = Rc::new(RefCell::new(8_()));
                let __tmp_143: Value<()> = Rc::new(RefCell::new(2_()));
                std_pair_const_int__double_::new_1({ __tmp_142.as_pointer() }, {
                    __tmp_143.as_pointer()
                })
            },
        ])));
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        let i: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        let k : Value<() > = Rc::new(RefCell::new(std___map_const_iterator_std___tree_const_iterator_std___value_type_int__double___min_pointer_std___tree_node_std___value_type_int__double___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____long__ :: new_1 ( { ((*i.borrow()) ).clone()  } , )  )) ;
        unimplemented!();
        let mut j: () = 1_();
        'loop_: while (j <= unimplemented!()) {
            unimplemented!();
            unimplemented!();
            field!(( { min_pointer_std_pair_const_int__double___std_integral_constant_unsigned_long__0__Impl :: operator_arrow ( &Rc::new(RefCell::new(( * (*i.borrow()) .upgrade().deref() ) )).as_pointer() , ) } )  , second) .write( 2.5_f64  ) ;
            unimplemented!();
            {
                j.prefix_inc();
                unimplemented!()
            };
        }
        unimplemented!();
        let mut j: () = unimplemented!();
        'loop_: while (j >= 1_()) {
            unimplemented!();
            unimplemented!();
            unimplemented!();
            field!(( { min_pointer_std_pair_const_int__double___std_integral_constant_unsigned_long__0__Impl :: operator_arrow ( &Rc::new(RefCell::new(( * (*i.borrow()) .upgrade().deref() ) )).as_pointer() , ) } )  , second) .write( 1_()  ) ;
            unimplemented!();
            j.prefix_dec();
        }
        unimplemented!();
    }
    {
        let ar: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
            {
                let __tmp_144: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_145: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_const_int__double_::new_1({ __tmp_144.as_pointer() }, {
                    __tmp_145.as_pointer()
                })
            },
            {
                let __tmp_146: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_147: Value<()> = Rc::new(RefCell::new(1.5_f64));
                std_pair_const_int__double_::new_2({ __tmp_146.as_pointer() }, {
                    __tmp_147.as_pointer()
                })
            },
            {
                let __tmp_148: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_149: Value<()> = Rc::new(RefCell::new(2_()));
                std_pair_const_int__double_::new_1({ __tmp_148.as_pointer() }, {
                    __tmp_149.as_pointer()
                })
            },
            {
                let __tmp_150: Value<()> = Rc::new(RefCell::new(2_()));
                let __tmp_151: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_const_int__double_::new_1({ __tmp_150.as_pointer() }, {
                    __tmp_151.as_pointer()
                })
            },
            {
                let __tmp_152: Value<()> = Rc::new(RefCell::new(2_()));
                let __tmp_153: Value<()> = Rc::new(RefCell::new(1.5_f64));
                std_pair_const_int__double_::new_2({ __tmp_152.as_pointer() }, {
                    __tmp_153.as_pointer()
                })
            },
            {
                let __tmp_154: Value<()> = Rc::new(RefCell::new(2_()));
                let __tmp_155: Value<()> = Rc::new(RefCell::new(2_()));
                std_pair_const_int__double_::new_1({ __tmp_154.as_pointer() }, {
                    __tmp_155.as_pointer()
                })
            },
            {
                let __tmp_156: Value<()> = Rc::new(RefCell::new(3_()));
                let __tmp_157: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_const_int__double_::new_1({ __tmp_156.as_pointer() }, {
                    __tmp_157.as_pointer()
                })
            },
            {
                let __tmp_158: Value<()> = Rc::new(RefCell::new(3_()));
                let __tmp_159: Value<()> = Rc::new(RefCell::new(1.5_f64));
                std_pair_const_int__double_::new_2({ __tmp_158.as_pointer() }, {
                    __tmp_159.as_pointer()
                })
            },
            {
                let __tmp_160: Value<()> = Rc::new(RefCell::new(3_()));
                let __tmp_161: Value<()> = Rc::new(RefCell::new(2_()));
                std_pair_const_int__double_::new_1({ __tmp_160.as_pointer() }, {
                    __tmp_161.as_pointer()
                })
            },
            {
                let __tmp_162: Value<()> = Rc::new(RefCell::new(4_()));
                let __tmp_163: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_const_int__double_::new_1({ __tmp_162.as_pointer() }, {
                    __tmp_163.as_pointer()
                })
            },
            {
                let __tmp_164: Value<()> = Rc::new(RefCell::new(4_()));
                let __tmp_165: Value<()> = Rc::new(RefCell::new(1.5_f64));
                std_pair_const_int__double_::new_2({ __tmp_164.as_pointer() }, {
                    __tmp_165.as_pointer()
                })
            },
            {
                let __tmp_166: Value<()> = Rc::new(RefCell::new(4_()));
                let __tmp_167: Value<()> = Rc::new(RefCell::new(2_()));
                std_pair_const_int__double_::new_1({ __tmp_166.as_pointer() }, {
                    __tmp_167.as_pointer()
                })
            },
            {
                let __tmp_168: Value<()> = Rc::new(RefCell::new(5_()));
                let __tmp_169: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_const_int__double_::new_1({ __tmp_168.as_pointer() }, {
                    __tmp_169.as_pointer()
                })
            },
            {
                let __tmp_170: Value<()> = Rc::new(RefCell::new(5_()));
                let __tmp_171: Value<()> = Rc::new(RefCell::new(1.5_f64));
                std_pair_const_int__double_::new_2({ __tmp_170.as_pointer() }, {
                    __tmp_171.as_pointer()
                })
            },
            {
                let __tmp_172: Value<()> = Rc::new(RefCell::new(5_()));
                let __tmp_173: Value<()> = Rc::new(RefCell::new(2_()));
                std_pair_const_int__double_::new_1({ __tmp_172.as_pointer() }, {
                    __tmp_173.as_pointer()
                })
            },
            {
                let __tmp_174: Value<()> = Rc::new(RefCell::new(6_()));
                let __tmp_175: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_const_int__double_::new_1({ __tmp_174.as_pointer() }, {
                    __tmp_175.as_pointer()
                })
            },
            {
                let __tmp_176: Value<()> = Rc::new(RefCell::new(6_()));
                let __tmp_177: Value<()> = Rc::new(RefCell::new(1.5_f64));
                std_pair_const_int__double_::new_2({ __tmp_176.as_pointer() }, {
                    __tmp_177.as_pointer()
                })
            },
            {
                let __tmp_178: Value<()> = Rc::new(RefCell::new(6_()));
                let __tmp_179: Value<()> = Rc::new(RefCell::new(2_()));
                std_pair_const_int__double_::new_1({ __tmp_178.as_pointer() }, {
                    __tmp_179.as_pointer()
                })
            },
            {
                let __tmp_180: Value<()> = Rc::new(RefCell::new(7_()));
                let __tmp_181: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_const_int__double_::new_1({ __tmp_180.as_pointer() }, {
                    __tmp_181.as_pointer()
                })
            },
            {
                let __tmp_182: Value<()> = Rc::new(RefCell::new(7_()));
                let __tmp_183: Value<()> = Rc::new(RefCell::new(1.5_f64));
                std_pair_const_int__double_::new_2({ __tmp_182.as_pointer() }, {
                    __tmp_183.as_pointer()
                })
            },
            {
                let __tmp_184: Value<()> = Rc::new(RefCell::new(7_()));
                let __tmp_185: Value<()> = Rc::new(RefCell::new(2_()));
                std_pair_const_int__double_::new_1({ __tmp_184.as_pointer() }, {
                    __tmp_185.as_pointer()
                })
            },
            {
                let __tmp_186: Value<()> = Rc::new(RefCell::new(8_()));
                let __tmp_187: Value<()> = Rc::new(RefCell::new(1_()));
                std_pair_const_int__double_::new_1({ __tmp_186.as_pointer() }, {
                    __tmp_187.as_pointer()
                })
            },
            {
                let __tmp_188: Value<()> = Rc::new(RefCell::new(8_()));
                let __tmp_189: Value<()> = Rc::new(RefCell::new(1.5_f64));
                std_pair_const_int__double_::new_2({ __tmp_188.as_pointer() }, {
                    __tmp_189.as_pointer()
                })
            },
            {
                let __tmp_190: Value<()> = Rc::new(RefCell::new(8_()));
                let __tmp_191: Value<()> = Rc::new(RefCell::new(2_()));
                std_pair_const_int__double_::new_1({ __tmp_190.as_pointer() }, {
                    __tmp_191.as_pointer()
                })
            },
        ])));
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let i: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        let mut j: () = 1_();
        'loop_: while (j <= unimplemented!()) {
            unimplemented!();
            unimplemented!();
            {
                j.prefix_inc();
                unimplemented!()
            };
        }
        unimplemented!();
        let mut j: () = unimplemented!();
        'loop_: while (j >= 1_()) {
            unimplemented!();
            unimplemented!();
            unimplemented!();
            j.prefix_dec();
        }
        unimplemented!();
    }
    {
        let ii1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ii2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ii4: Value<()> = Rc::new(RefCell::new((*ii1.borrow()).clone()));
        let cii: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
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
    ({ test_8() });
    return 0_();
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
pub trait min_pointer_const_std_pair_const_int__double___std_integral_constant_unsigned_long__0__Impl
{
    fn to_(&self) -> () {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_arrow(&self) -> Ptr<()>;
    fn operator_inc_9(
        &self,
    ) -> Ptr<min_pointer_const_std_pair_const_int__double___std_integral_constant_unsigned_long__0__>
    {
        unimplemented!()
    }
    fn operator_post_inc_10(
        &self,
        mut _a0: (),
    ) -> min_pointer_const_std_pair_const_int__double___std_integral_constant_unsigned_long__0__
    {
        unimplemented!()
    }
    fn operator_dec_11(
        &self,
    ) -> Ptr<min_pointer_const_std_pair_const_int__double___std_integral_constant_unsigned_long__0__>
    {
        unimplemented!()
    }
    fn operator_post_dec_12(
        &self,
        mut _a0: (),
    ) -> min_pointer_const_std_pair_const_int__double___std_integral_constant_unsigned_long__0__
    {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_const_std_pair_const_int__double___std_integral_constant_unsigned_long__0__>
    {
        unimplemented!()
    }
    fn operator_sub_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_const_std_pair_const_int__double___std_integral_constant_unsigned_long__0__>
    {
        unimplemented!()
    }
    fn operator_add(
        &self,
        mut n: (),
    ) -> min_pointer_const_std_pair_const_int__double___std_integral_constant_unsigned_long__0__
    {
        unimplemented!()
    }
    fn operator_sub(
        &self,
        mut n: (),
    ) -> min_pointer_const_std_pair_const_int__double___std_integral_constant_unsigned_long__0__
    {
        unimplemented!()
    }
    fn operator_index(&self, mut n: ()) -> Ptr<()> {
        unimplemented!()
    }
}
impl min_pointer_const_std_pair_const_int__double___std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_const_std_pair_const_int__double___std_integral_constant_unsigned_long__0__>
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
    fn operator_arrow(&self) -> Ptr<()>;
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
pub fn __cpp2rust_init_globals() {
    let _ = outstanding_bytes_0.with(|_| ());
    let _ = alloc_count_1.with(|_| ());
    let _ = dealloc_count_2.with(|_| ());
    let _ = disable_default_constructor_3.with(|_| ());
}
