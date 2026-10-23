extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub unsafe fn outstanding_alloc() -> () {
        unimplemented!();
        return ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_1))
            .wrapping_sub((*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_2))));
    }
    pub unsafe fn reset() {
        unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut disable_default_constructor_3)) = false;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut outstanding_bytes_0)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_1)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_2)) = 0_();
    }
}
pub static mut outstanding_bytes_0: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut alloc_count_1: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut dealloc_count_2: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut disable_default_constructor_3: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { false });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_std_pair_const_int__double___std_integral_constant_unsigned_long__0__ {
    ptr_: *mut (),
}
impl min_pointer_std_pair_const_int__double___std_integral_constant_unsigned_long__0__ {
    unsafe fn new_1(mut p: *mut ()) -> Self {
        let mut this = Self { ptr_: p };
        this
    }
    pub unsafe fn operator_arrow(&self) -> *mut () {
        return self.ptr_;
    }
    pub unsafe fn pointer_to(
        t: *mut (),
    ) -> min_pointer_std_pair_const_int__double___std_integral_constant_unsigned_long__0__ {
        return min_pointer_std_pair_const_int__double___std_integral_constant_unsigned_long__0__ :: new_1 ( { unimplemented!()  } , )   ;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    ptr_: (),
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_const_std_pair_const_int__double___std_integral_constant_unsigned_long__0__ {
    ptr_: *const (),
}
impl min_pointer_const_std_pair_const_int__double___std_integral_constant_unsigned_long__0__ {
    unsafe fn new_1(mut p: *const ()) -> Self {
        let mut this = Self { ptr_: p };
        this
    }
    pub unsafe fn operator_arrow(&self) -> *const () {
        return self.ptr_;
    }
    pub unsafe fn pointer_to(
        t: *const (),
    ) -> min_pointer_const_std_pair_const_int__double___std_integral_constant_unsigned_long__0__
    {
        return min_pointer_const_std_pair_const_int__double___std_integral_constant_unsigned_long__0__ :: new_1 ( { unimplemented!()  } , )   ;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_const_void__std_integral_constant_unsigned_long__0__ {
    ptr_: (),
}
impl min_pointer_const_void__std_integral_constant_unsigned_long__0__ {}
pub unsafe fn operator_eq_4(
    mut x : min_pointer_std___tree_node_std___value_type_int__double___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_std___tree_node_std___value_type_int__double___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__,
) -> () {
    return ((x.ptr_) == (y.ptr_));
}
pub unsafe fn operator_ne_5(
    mut x : min_pointer_std___tree_node_std___value_type_int__double___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_std___tree_node_std___value_type_int__double___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__,
) -> () {
    return !(unsafe {
        let _x: min_pointer_std___tree_node_std___value_type_int__double___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__  = x ;
        operator_eq_4(_x, y)
    });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_std___tree_node_std___value_type_int__double___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ { ptr_ : *mut std___tree_node_std___value_type_int__double___min_pointer_void__std_integral_constant_unsigned_long__0___ , }
impl min_pointer_std___tree_node_std___value_type_int__double___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ { unsafe fn new_1 ( mut p : *mut std___tree_node_std___value_type_int__double___min_pointer_void__std_integral_constant_unsigned_long__0___ , ) -> Self { let mut this = Self { ptr_ : p , } ;
 this } pub unsafe fn new_2 ( mut _a0 : () , ) -> Self { let mut this = Self { ptr_ : std::ptr::null_mut() , } ;
 this } pub unsafe fn to_ ( &self , ) -> () { return ! ( ( self . ptr_  ).is_null() )  ;
 } pub unsafe fn operator_deref ( &self , ) -> *mut std___tree_node_std___value_type_int__double___min_pointer_void__std_integral_constant_unsigned_long__0___ { return & mut ( * self . ptr_   )  ;
 } pub unsafe fn operator_arrow ( &self , ) -> *mut std___tree_node_std___value_type_int__double___min_pointer_void__std_integral_constant_unsigned_long__0___ { return self . ptr_   ;
 } pub unsafe fn pointer_to ( t : *mut std___tree_node_std___value_type_int__double___min_pointer_void__std_integral_constant_unsigned_long__0___ , ) -> min_pointer_std___tree_node_std___value_type_int__double___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ { return min_pointer_std___tree_node_std___value_type_int__double___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ :: new_1 ( { unimplemented!()  } , )   ;
 } }
impl std::cmp::PartialEq for min_pointer_std___tree_node_std___value_type_int__double___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ { fn eq(&self, other: &Self) -> bool { unsafe { operator_eq_4(self.clone(), other.clone()) }}}
impl std::cmp::Eq for min_pointer_std___tree_node_std___value_type_int__double___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ {}
pub unsafe fn operator_eq_6(
    mut x : min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__,
) -> () {
    return ((x.ptr_) == (y.ptr_));
}
pub unsafe fn operator_ne_7(
    mut x : min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__,
) -> () {
    return !(unsafe {
        let _x: min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__  = x ;
        operator_eq_6(_x, y)
    });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__
{
    ptr_: *mut std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0___,
}
impl min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ { unsafe fn new_1 ( mut p : *mut std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0___ , ) -> Self { let mut this = Self { ptr_ : p , } ;
 this } pub unsafe fn new_2 ( mut _a0 : () , ) -> Self { let mut this = Self { ptr_ : std::ptr::null_mut() , } ;
 this } pub unsafe fn to_ ( &self , ) -> () { return ! ( ( self . ptr_  ).is_null() )  ;
 } pub unsafe fn operator_deref ( &self , ) -> *mut std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0___ { return & mut ( * self . ptr_   )  ;
 } pub unsafe fn operator_arrow ( &self , ) -> *mut std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0___ { return self . ptr_   ;
 } pub unsafe fn pointer_to ( t : *mut std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0___ , ) -> min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ { return min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ :: new_1 ( { unimplemented!()  } , )   ;
 } }
impl std::cmp::PartialEq for min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ { fn eq(&self, other: &Self) -> bool { unsafe { operator_eq_6(self.clone(), other.clone()) }}}
impl std::cmp::Eq for min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ {}
pub unsafe fn operator_eq_8(
    mut x : min_pointer_std___tree_end_node_min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_std___tree_end_node_min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__,
) -> () {
    return ((x.ptr_) == (y.ptr_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_std___tree_end_node_min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ { ptr_ : *mut std___tree_end_node_min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0___ , }
impl min_pointer_std___tree_end_node_min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ { unsafe fn new_1 ( mut p : *mut std___tree_end_node_min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0___ , ) -> Self { let mut this = Self { ptr_ : p , } ;
 this } pub unsafe fn new_2 ( mut _a0 : () , ) -> Self { let mut this = Self { ptr_ : std::ptr::null_mut() , } ;
 this } pub unsafe fn to_ ( &self , ) -> () { return ! ( ( self . ptr_  ).is_null() )  ;
 } pub unsafe fn operator_deref ( &self , ) -> *mut std___tree_end_node_min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0___ { return & mut ( * self . ptr_   )  ;
 } pub unsafe fn operator_arrow ( &self , ) -> *mut std___tree_end_node_min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0___ { return self . ptr_   ;
 } pub unsafe fn pointer_to ( t : *mut std___tree_end_node_min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0___ , ) -> min_pointer_std___tree_end_node_min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ { return min_pointer_std___tree_end_node_min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ :: new_1 ( { unimplemented!()  } , )   ;
 } }
impl std::cmp::PartialEq for min_pointer_std___tree_end_node_min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ { fn eq(&self, other: &Self) -> bool { unsafe { operator_eq_8(self.clone(), other.clone()) }}}
impl std::cmp::Eq for min_pointer_std___tree_end_node_min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_std_pair_const_int__double__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_std___tree_node_std___value_type_int__double___min_pointer_void__std_integral_constant_unsigned_long__0____
{}
impl min_allocator_std___tree_node_std___value_type_int__double___min_pointer_void__std_integral_constant_unsigned_long__0____ { pub unsafe fn allocate ( &mut self , mut n : () , ) -> min_pointer_std___tree_node_std___value_type_int__double___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ { return min_pointer_std___tree_node_std___value_type_int__double___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ :: new_1 ( { unimplemented!()  } , )   ;
 } pub unsafe fn deallocate ( &mut self , mut p : min_pointer_std___tree_node_std___value_type_int__double___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ , mut n : () , ) { unimplemented!()  ;
 } }
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0____
{}
pub unsafe fn test_9() -> () {
    {
        let mut ar: [(); 24] = [
            {
                let mut __tmp_0: () = 1_();
                let mut __tmp_1: () = 1_();
                std_pair_const_int__double_::new_1({ &mut __tmp_0 }, { &mut __tmp_1 })
            },
            {
                let mut __tmp_2: () = 1_();
                let mut __tmp_3: () = 1.5_f64;
                std_pair_const_int__double_::new_2({ &mut __tmp_2 }, { &mut __tmp_3 })
            },
            {
                let mut __tmp_4: () = 1_();
                let mut __tmp_5: () = 2_();
                std_pair_const_int__double_::new_1({ &mut __tmp_4 }, { &mut __tmp_5 })
            },
            {
                let mut __tmp_6: () = 2_();
                let mut __tmp_7: () = 1_();
                std_pair_const_int__double_::new_1({ &mut __tmp_6 }, { &mut __tmp_7 })
            },
            {
                let mut __tmp_8: () = 2_();
                let mut __tmp_9: () = 1.5_f64;
                std_pair_const_int__double_::new_2({ &mut __tmp_8 }, { &mut __tmp_9 })
            },
            {
                let mut __tmp_10: () = 2_();
                let mut __tmp_11: () = 2_();
                std_pair_const_int__double_::new_1({ &mut __tmp_10 }, { &mut __tmp_11 })
            },
            {
                let mut __tmp_12: () = 3_();
                let mut __tmp_13: () = 1_();
                std_pair_const_int__double_::new_1({ &mut __tmp_12 }, { &mut __tmp_13 })
            },
            {
                let mut __tmp_14: () = 3_();
                let mut __tmp_15: () = 1.5_f64;
                std_pair_const_int__double_::new_2({ &mut __tmp_14 }, { &mut __tmp_15 })
            },
            {
                let mut __tmp_16: () = 3_();
                let mut __tmp_17: () = 2_();
                std_pair_const_int__double_::new_1({ &mut __tmp_16 }, { &mut __tmp_17 })
            },
            {
                let mut __tmp_18: () = 4_();
                let mut __tmp_19: () = 1_();
                std_pair_const_int__double_::new_1({ &mut __tmp_18 }, { &mut __tmp_19 })
            },
            {
                let mut __tmp_20: () = 4_();
                let mut __tmp_21: () = 1.5_f64;
                std_pair_const_int__double_::new_2({ &mut __tmp_20 }, { &mut __tmp_21 })
            },
            {
                let mut __tmp_22: () = 4_();
                let mut __tmp_23: () = 2_();
                std_pair_const_int__double_::new_1({ &mut __tmp_22 }, { &mut __tmp_23 })
            },
            {
                let mut __tmp_24: () = 5_();
                let mut __tmp_25: () = 1_();
                std_pair_const_int__double_::new_1({ &mut __tmp_24 }, { &mut __tmp_25 })
            },
            {
                let mut __tmp_26: () = 5_();
                let mut __tmp_27: () = 1.5_f64;
                std_pair_const_int__double_::new_2({ &mut __tmp_26 }, { &mut __tmp_27 })
            },
            {
                let mut __tmp_28: () = 5_();
                let mut __tmp_29: () = 2_();
                std_pair_const_int__double_::new_1({ &mut __tmp_28 }, { &mut __tmp_29 })
            },
            {
                let mut __tmp_30: () = 6_();
                let mut __tmp_31: () = 1_();
                std_pair_const_int__double_::new_1({ &mut __tmp_30 }, { &mut __tmp_31 })
            },
            {
                let mut __tmp_32: () = 6_();
                let mut __tmp_33: () = 1.5_f64;
                std_pair_const_int__double_::new_2({ &mut __tmp_32 }, { &mut __tmp_33 })
            },
            {
                let mut __tmp_34: () = 6_();
                let mut __tmp_35: () = 2_();
                std_pair_const_int__double_::new_1({ &mut __tmp_34 }, { &mut __tmp_35 })
            },
            {
                let mut __tmp_36: () = 7_();
                let mut __tmp_37: () = 1_();
                std_pair_const_int__double_::new_1({ &mut __tmp_36 }, { &mut __tmp_37 })
            },
            {
                let mut __tmp_38: () = 7_();
                let mut __tmp_39: () = 1.5_f64;
                std_pair_const_int__double_::new_2({ &mut __tmp_38 }, { &mut __tmp_39 })
            },
            {
                let mut __tmp_40: () = 7_();
                let mut __tmp_41: () = 2_();
                std_pair_const_int__double_::new_1({ &mut __tmp_40 }, { &mut __tmp_41 })
            },
            {
                let mut __tmp_42: () = 8_();
                let mut __tmp_43: () = 1_();
                std_pair_const_int__double_::new_1({ &mut __tmp_42 }, { &mut __tmp_43 })
            },
            {
                let mut __tmp_44: () = 8_();
                let mut __tmp_45: () = 1.5_f64;
                std_pair_const_int__double_::new_2({ &mut __tmp_44 }, { &mut __tmp_45 })
            },
            {
                let mut __tmp_46: () = 8_();
                let mut __tmp_47: () = 2_();
                std_pair_const_int__double_::new_1({ &mut __tmp_46 }, { &mut __tmp_47 })
            },
        ];
        let mut m: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut i: () = unimplemented!();
        unimplemented!();
        let mut k : () = std___map_const_iterator_std___tree_const_iterator_std___value_type_int__double___std___tree_node_std___value_type_int__double___void_ptr__ptr__long__ :: new_1 ( { i .clone() } , )  ;
        unimplemented!();
        let mut j: () = 1_();
        'loop_: while ((j) <= (8_())) {
            let mut d: () = 1_();
            'loop_: while ((d) <= (2_())) {
                unimplemented!();
                unimplemented!();
                (*i).second = 2.5_f64;
                unimplemented!();
                {
                    d += 0.5_f64;
                    unimplemented!()
                };
            }
            j.prefix_inc();
        }
        unimplemented!();
        let mut j: () = 8_();
        'loop_: while ((j) >= (1_())) {
            let mut d: () = 1_();
            'loop_: while ((d) <= (2_())) {
                unimplemented!();
                unimplemented!();
                unimplemented!();
                (*i).second = d;
                unimplemented!();
                d += 0.5_f64;
            }
            j.prefix_dec();
        }
        unimplemented!();
    }
    {
        let mut ar: [(); 24] = [
            {
                let mut __tmp_48: () = 1_();
                let mut __tmp_49: () = 1_();
                std_pair_const_int__double_::new_1({ &mut __tmp_48 }, { &mut __tmp_49 })
            },
            {
                let mut __tmp_50: () = 1_();
                let mut __tmp_51: () = 1.5_f64;
                std_pair_const_int__double_::new_2({ &mut __tmp_50 }, { &mut __tmp_51 })
            },
            {
                let mut __tmp_52: () = 1_();
                let mut __tmp_53: () = 2_();
                std_pair_const_int__double_::new_1({ &mut __tmp_52 }, { &mut __tmp_53 })
            },
            {
                let mut __tmp_54: () = 2_();
                let mut __tmp_55: () = 1_();
                std_pair_const_int__double_::new_1({ &mut __tmp_54 }, { &mut __tmp_55 })
            },
            {
                let mut __tmp_56: () = 2_();
                let mut __tmp_57: () = 1.5_f64;
                std_pair_const_int__double_::new_2({ &mut __tmp_56 }, { &mut __tmp_57 })
            },
            {
                let mut __tmp_58: () = 2_();
                let mut __tmp_59: () = 2_();
                std_pair_const_int__double_::new_1({ &mut __tmp_58 }, { &mut __tmp_59 })
            },
            {
                let mut __tmp_60: () = 3_();
                let mut __tmp_61: () = 1_();
                std_pair_const_int__double_::new_1({ &mut __tmp_60 }, { &mut __tmp_61 })
            },
            {
                let mut __tmp_62: () = 3_();
                let mut __tmp_63: () = 1.5_f64;
                std_pair_const_int__double_::new_2({ &mut __tmp_62 }, { &mut __tmp_63 })
            },
            {
                let mut __tmp_64: () = 3_();
                let mut __tmp_65: () = 2_();
                std_pair_const_int__double_::new_1({ &mut __tmp_64 }, { &mut __tmp_65 })
            },
            {
                let mut __tmp_66: () = 4_();
                let mut __tmp_67: () = 1_();
                std_pair_const_int__double_::new_1({ &mut __tmp_66 }, { &mut __tmp_67 })
            },
            {
                let mut __tmp_68: () = 4_();
                let mut __tmp_69: () = 1.5_f64;
                std_pair_const_int__double_::new_2({ &mut __tmp_68 }, { &mut __tmp_69 })
            },
            {
                let mut __tmp_70: () = 4_();
                let mut __tmp_71: () = 2_();
                std_pair_const_int__double_::new_1({ &mut __tmp_70 }, { &mut __tmp_71 })
            },
            {
                let mut __tmp_72: () = 5_();
                let mut __tmp_73: () = 1_();
                std_pair_const_int__double_::new_1({ &mut __tmp_72 }, { &mut __tmp_73 })
            },
            {
                let mut __tmp_74: () = 5_();
                let mut __tmp_75: () = 1.5_f64;
                std_pair_const_int__double_::new_2({ &mut __tmp_74 }, { &mut __tmp_75 })
            },
            {
                let mut __tmp_76: () = 5_();
                let mut __tmp_77: () = 2_();
                std_pair_const_int__double_::new_1({ &mut __tmp_76 }, { &mut __tmp_77 })
            },
            {
                let mut __tmp_78: () = 6_();
                let mut __tmp_79: () = 1_();
                std_pair_const_int__double_::new_1({ &mut __tmp_78 }, { &mut __tmp_79 })
            },
            {
                let mut __tmp_80: () = 6_();
                let mut __tmp_81: () = 1.5_f64;
                std_pair_const_int__double_::new_2({ &mut __tmp_80 }, { &mut __tmp_81 })
            },
            {
                let mut __tmp_82: () = 6_();
                let mut __tmp_83: () = 2_();
                std_pair_const_int__double_::new_1({ &mut __tmp_82 }, { &mut __tmp_83 })
            },
            {
                let mut __tmp_84: () = 7_();
                let mut __tmp_85: () = 1_();
                std_pair_const_int__double_::new_1({ &mut __tmp_84 }, { &mut __tmp_85 })
            },
            {
                let mut __tmp_86: () = 7_();
                let mut __tmp_87: () = 1.5_f64;
                std_pair_const_int__double_::new_2({ &mut __tmp_86 }, { &mut __tmp_87 })
            },
            {
                let mut __tmp_88: () = 7_();
                let mut __tmp_89: () = 2_();
                std_pair_const_int__double_::new_1({ &mut __tmp_88 }, { &mut __tmp_89 })
            },
            {
                let mut __tmp_90: () = 8_();
                let mut __tmp_91: () = 1_();
                std_pair_const_int__double_::new_1({ &mut __tmp_90 }, { &mut __tmp_91 })
            },
            {
                let mut __tmp_92: () = 8_();
                let mut __tmp_93: () = 1.5_f64;
                std_pair_const_int__double_::new_2({ &mut __tmp_92 }, { &mut __tmp_93 })
            },
            {
                let mut __tmp_94: () = 8_();
                let mut __tmp_95: () = 2_();
                std_pair_const_int__double_::new_1({ &mut __tmp_94 }, { &mut __tmp_95 })
            },
        ];
        let m: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut i: () = unimplemented!();
        unimplemented!();
        let mut j: () = 1_();
        'loop_: while ((j) <= (8_())) {
            let mut d: () = 1_();
            'loop_: while ((d) <= (2_())) {
                unimplemented!();
                unimplemented!();
                {
                    d += 0.5_f64;
                    unimplemented!()
                };
            }
            j.prefix_inc();
        }
        unimplemented!();
        let mut j: () = 8_();
        'loop_: while ((j) >= (1_())) {
            let mut d: () = 2_();
            'loop_: while ((d) >= (1_())) {
                unimplemented!();
                unimplemented!();
                unimplemented!();
                d -= 0.5_f64;
            }
            j.prefix_dec();
        }
        unimplemented!();
    }
    {
        let mut ar: [(); 24] = [
            {
                let mut __tmp_96: () = 1_();
                let mut __tmp_97: () = 1_();
                std_pair_const_int__double_::new_1({ &mut __tmp_96 }, { &mut __tmp_97 })
            },
            {
                let mut __tmp_98: () = 1_();
                let mut __tmp_99: () = 1.5_f64;
                std_pair_const_int__double_::new_2({ &mut __tmp_98 }, { &mut __tmp_99 })
            },
            {
                let mut __tmp_100: () = 1_();
                let mut __tmp_101: () = 2_();
                std_pair_const_int__double_::new_1({ &mut __tmp_100 }, { &mut __tmp_101 })
            },
            {
                let mut __tmp_102: () = 2_();
                let mut __tmp_103: () = 1_();
                std_pair_const_int__double_::new_1({ &mut __tmp_102 }, { &mut __tmp_103 })
            },
            {
                let mut __tmp_104: () = 2_();
                let mut __tmp_105: () = 1.5_f64;
                std_pair_const_int__double_::new_2({ &mut __tmp_104 }, { &mut __tmp_105 })
            },
            {
                let mut __tmp_106: () = 2_();
                let mut __tmp_107: () = 2_();
                std_pair_const_int__double_::new_1({ &mut __tmp_106 }, { &mut __tmp_107 })
            },
            {
                let mut __tmp_108: () = 3_();
                let mut __tmp_109: () = 1_();
                std_pair_const_int__double_::new_1({ &mut __tmp_108 }, { &mut __tmp_109 })
            },
            {
                let mut __tmp_110: () = 3_();
                let mut __tmp_111: () = 1.5_f64;
                std_pair_const_int__double_::new_2({ &mut __tmp_110 }, { &mut __tmp_111 })
            },
            {
                let mut __tmp_112: () = 3_();
                let mut __tmp_113: () = 2_();
                std_pair_const_int__double_::new_1({ &mut __tmp_112 }, { &mut __tmp_113 })
            },
            {
                let mut __tmp_114: () = 4_();
                let mut __tmp_115: () = 1_();
                std_pair_const_int__double_::new_1({ &mut __tmp_114 }, { &mut __tmp_115 })
            },
            {
                let mut __tmp_116: () = 4_();
                let mut __tmp_117: () = 1.5_f64;
                std_pair_const_int__double_::new_2({ &mut __tmp_116 }, { &mut __tmp_117 })
            },
            {
                let mut __tmp_118: () = 4_();
                let mut __tmp_119: () = 2_();
                std_pair_const_int__double_::new_1({ &mut __tmp_118 }, { &mut __tmp_119 })
            },
            {
                let mut __tmp_120: () = 5_();
                let mut __tmp_121: () = 1_();
                std_pair_const_int__double_::new_1({ &mut __tmp_120 }, { &mut __tmp_121 })
            },
            {
                let mut __tmp_122: () = 5_();
                let mut __tmp_123: () = 1.5_f64;
                std_pair_const_int__double_::new_2({ &mut __tmp_122 }, { &mut __tmp_123 })
            },
            {
                let mut __tmp_124: () = 5_();
                let mut __tmp_125: () = 2_();
                std_pair_const_int__double_::new_1({ &mut __tmp_124 }, { &mut __tmp_125 })
            },
            {
                let mut __tmp_126: () = 6_();
                let mut __tmp_127: () = 1_();
                std_pair_const_int__double_::new_1({ &mut __tmp_126 }, { &mut __tmp_127 })
            },
            {
                let mut __tmp_128: () = 6_();
                let mut __tmp_129: () = 1.5_f64;
                std_pair_const_int__double_::new_2({ &mut __tmp_128 }, { &mut __tmp_129 })
            },
            {
                let mut __tmp_130: () = 6_();
                let mut __tmp_131: () = 2_();
                std_pair_const_int__double_::new_1({ &mut __tmp_130 }, { &mut __tmp_131 })
            },
            {
                let mut __tmp_132: () = 7_();
                let mut __tmp_133: () = 1_();
                std_pair_const_int__double_::new_1({ &mut __tmp_132 }, { &mut __tmp_133 })
            },
            {
                let mut __tmp_134: () = 7_();
                let mut __tmp_135: () = 1.5_f64;
                std_pair_const_int__double_::new_2({ &mut __tmp_134 }, { &mut __tmp_135 })
            },
            {
                let mut __tmp_136: () = 7_();
                let mut __tmp_137: () = 2_();
                std_pair_const_int__double_::new_1({ &mut __tmp_136 }, { &mut __tmp_137 })
            },
            {
                let mut __tmp_138: () = 8_();
                let mut __tmp_139: () = 1_();
                std_pair_const_int__double_::new_1({ &mut __tmp_138 }, { &mut __tmp_139 })
            },
            {
                let mut __tmp_140: () = 8_();
                let mut __tmp_141: () = 1.5_f64;
                std_pair_const_int__double_::new_2({ &mut __tmp_140 }, { &mut __tmp_141 })
            },
            {
                let mut __tmp_142: () = 8_();
                let mut __tmp_143: () = 2_();
                std_pair_const_int__double_::new_1({ &mut __tmp_142 }, { &mut __tmp_143 })
            },
        ];
        let mut m: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut i: () = unimplemented!();
        unimplemented!();
        let mut k : () = std___map_const_iterator_std___tree_const_iterator_std___value_type_int__double___min_pointer_std___tree_node_std___value_type_int__double___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____long__ :: new_1 ( { i .clone() } , )  ;
        unimplemented!();
        let mut j: () = 1_();
        'loop_: while ((j) <= (8_())) {
            let mut d: () = 1_();
            'loop_: while ((d) <= (2_())) {
                unimplemented!();
                unimplemented!();
                ( * ( unsafe { min_pointer_std_pair_const_int__double___std_integral_constant_unsigned_long__0__ :: operator_arrow ( & i  , ) } )   ) . second   = 2.5_f64  ;
                unimplemented!();
                {
                    d += 0.5_f64;
                    unimplemented!()
                };
            }
            j.prefix_inc();
        }
        unimplemented!();
        let mut j: () = 8_();
        'loop_: while ((j) >= (1_())) {
            let mut d: () = 1_();
            'loop_: while ((d) <= (2_())) {
                unimplemented!();
                unimplemented!();
                unimplemented!();
                ( * ( unsafe { min_pointer_std_pair_const_int__double___std_integral_constant_unsigned_long__0__ :: operator_arrow ( & i  , ) } )   ) . second   = d  ;
                unimplemented!();
                d += 0.5_f64;
            }
            j.prefix_dec();
        }
        unimplemented!();
    }
    {
        let mut ar: [(); 24] = [
            {
                let mut __tmp_144: () = 1_();
                let mut __tmp_145: () = 1_();
                std_pair_const_int__double_::new_1({ &mut __tmp_144 }, { &mut __tmp_145 })
            },
            {
                let mut __tmp_146: () = 1_();
                let mut __tmp_147: () = 1.5_f64;
                std_pair_const_int__double_::new_2({ &mut __tmp_146 }, { &mut __tmp_147 })
            },
            {
                let mut __tmp_148: () = 1_();
                let mut __tmp_149: () = 2_();
                std_pair_const_int__double_::new_1({ &mut __tmp_148 }, { &mut __tmp_149 })
            },
            {
                let mut __tmp_150: () = 2_();
                let mut __tmp_151: () = 1_();
                std_pair_const_int__double_::new_1({ &mut __tmp_150 }, { &mut __tmp_151 })
            },
            {
                let mut __tmp_152: () = 2_();
                let mut __tmp_153: () = 1.5_f64;
                std_pair_const_int__double_::new_2({ &mut __tmp_152 }, { &mut __tmp_153 })
            },
            {
                let mut __tmp_154: () = 2_();
                let mut __tmp_155: () = 2_();
                std_pair_const_int__double_::new_1({ &mut __tmp_154 }, { &mut __tmp_155 })
            },
            {
                let mut __tmp_156: () = 3_();
                let mut __tmp_157: () = 1_();
                std_pair_const_int__double_::new_1({ &mut __tmp_156 }, { &mut __tmp_157 })
            },
            {
                let mut __tmp_158: () = 3_();
                let mut __tmp_159: () = 1.5_f64;
                std_pair_const_int__double_::new_2({ &mut __tmp_158 }, { &mut __tmp_159 })
            },
            {
                let mut __tmp_160: () = 3_();
                let mut __tmp_161: () = 2_();
                std_pair_const_int__double_::new_1({ &mut __tmp_160 }, { &mut __tmp_161 })
            },
            {
                let mut __tmp_162: () = 4_();
                let mut __tmp_163: () = 1_();
                std_pair_const_int__double_::new_1({ &mut __tmp_162 }, { &mut __tmp_163 })
            },
            {
                let mut __tmp_164: () = 4_();
                let mut __tmp_165: () = 1.5_f64;
                std_pair_const_int__double_::new_2({ &mut __tmp_164 }, { &mut __tmp_165 })
            },
            {
                let mut __tmp_166: () = 4_();
                let mut __tmp_167: () = 2_();
                std_pair_const_int__double_::new_1({ &mut __tmp_166 }, { &mut __tmp_167 })
            },
            {
                let mut __tmp_168: () = 5_();
                let mut __tmp_169: () = 1_();
                std_pair_const_int__double_::new_1({ &mut __tmp_168 }, { &mut __tmp_169 })
            },
            {
                let mut __tmp_170: () = 5_();
                let mut __tmp_171: () = 1.5_f64;
                std_pair_const_int__double_::new_2({ &mut __tmp_170 }, { &mut __tmp_171 })
            },
            {
                let mut __tmp_172: () = 5_();
                let mut __tmp_173: () = 2_();
                std_pair_const_int__double_::new_1({ &mut __tmp_172 }, { &mut __tmp_173 })
            },
            {
                let mut __tmp_174: () = 6_();
                let mut __tmp_175: () = 1_();
                std_pair_const_int__double_::new_1({ &mut __tmp_174 }, { &mut __tmp_175 })
            },
            {
                let mut __tmp_176: () = 6_();
                let mut __tmp_177: () = 1.5_f64;
                std_pair_const_int__double_::new_2({ &mut __tmp_176 }, { &mut __tmp_177 })
            },
            {
                let mut __tmp_178: () = 6_();
                let mut __tmp_179: () = 2_();
                std_pair_const_int__double_::new_1({ &mut __tmp_178 }, { &mut __tmp_179 })
            },
            {
                let mut __tmp_180: () = 7_();
                let mut __tmp_181: () = 1_();
                std_pair_const_int__double_::new_1({ &mut __tmp_180 }, { &mut __tmp_181 })
            },
            {
                let mut __tmp_182: () = 7_();
                let mut __tmp_183: () = 1.5_f64;
                std_pair_const_int__double_::new_2({ &mut __tmp_182 }, { &mut __tmp_183 })
            },
            {
                let mut __tmp_184: () = 7_();
                let mut __tmp_185: () = 2_();
                std_pair_const_int__double_::new_1({ &mut __tmp_184 }, { &mut __tmp_185 })
            },
            {
                let mut __tmp_186: () = 8_();
                let mut __tmp_187: () = 1_();
                std_pair_const_int__double_::new_1({ &mut __tmp_186 }, { &mut __tmp_187 })
            },
            {
                let mut __tmp_188: () = 8_();
                let mut __tmp_189: () = 1.5_f64;
                std_pair_const_int__double_::new_2({ &mut __tmp_188 }, { &mut __tmp_189 })
            },
            {
                let mut __tmp_190: () = 8_();
                let mut __tmp_191: () = 2_();
                std_pair_const_int__double_::new_1({ &mut __tmp_190 }, { &mut __tmp_191 })
            },
        ];
        let m: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut i: () = unimplemented!();
        unimplemented!();
        let mut j: () = 1_();
        'loop_: while ((j) <= (8_())) {
            let mut d: () = 1_();
            'loop_: while ((d) <= (2_())) {
                unimplemented!();
                unimplemented!();
                {
                    d += 0.5_f64;
                    unimplemented!()
                };
            }
            j.prefix_inc();
        }
        unimplemented!();
        let mut j: () = 8_();
        'loop_: while ((j) >= (1_())) {
            let mut d: () = 2_();
            'loop_: while ((d) >= (1_())) {
                unimplemented!();
                unimplemented!();
                unimplemented!();
                d -= 0.5_f64;
            }
            j.prefix_dec();
        }
        unimplemented!();
    }
    {
        let mut ii1: () = unimplemented!();
        let mut ii2: () = unimplemented!();
        let mut ii4: () = ii1.clone();
        let mut cii: () = unimplemented!();
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
    let mut args: Vec<Vec<u8>> = std::env::args()
        .map(|arg| arg.as_bytes().to_vec())
        .collect();
    args.iter_mut().for_each(|v| v.push(0));
    let mut argv: Vec<*mut libc::c_char> = args
        .iter()
        .map(|arg| arg.as_ptr() as *mut libc::c_char)
        .collect();
    argv.push(::std::ptr::null_mut());
    unsafe {
        __cpp2rust_init_globals();
        ::std::process::exit(main_0((argv.len() - 1) as i32, argv.as_mut_ptr()) as i32)
    }
}
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
    (unsafe { test_9() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_0);
    std::cell::LazyCell::force(&*&raw const alloc_count_1);
    std::cell::LazyCell::force(&*&raw const dealloc_count_2);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_3);
}
