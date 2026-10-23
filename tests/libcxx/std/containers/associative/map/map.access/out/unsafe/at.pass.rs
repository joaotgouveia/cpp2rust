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
impl min_pointer_std_pair_const_int__double___std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    ptr_: (),
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
pub unsafe fn operator_eq_4(
    mut x : min_pointer_std___tree_node_std___value_type_int__double___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_std___tree_node_std___value_type_int__double___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__,
) -> () {
    return ((x.ptr_) == (y.ptr_));
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
pub unsafe fn operator_eq_5(
    mut x : min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__,
) -> () {
    return ((x.ptr_) == (y.ptr_));
}
pub unsafe fn operator_ne_6(
    mut x : min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__,
) -> () {
    return !(unsafe {
        let _x: min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__  = x ;
        operator_eq_5(_x, y)
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
impl std::cmp::PartialEq for min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ { fn eq(&self, other: &Self) -> bool { unsafe { operator_eq_5(self.clone(), other.clone()) }}}
impl std::cmp::Eq for min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ {}
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
pub unsafe fn test_7() -> () {
    {
        let mut ar: [(); 7] = [
            {
                let mut __tmp_0: () = 1_();
                let mut __tmp_1: () = 1.5_f64;
                std_pair_const_int__double_::new_1({ &mut __tmp_0 }, { &mut __tmp_1 })
            },
            {
                let mut __tmp_2: () = 2_();
                let mut __tmp_3: () = 2.5_f64;
                std_pair_const_int__double_::new_1({ &mut __tmp_2 }, { &mut __tmp_3 })
            },
            {
                let mut __tmp_4: () = 3_();
                let mut __tmp_5: () = 3.5_f64;
                std_pair_const_int__double_::new_1({ &mut __tmp_4 }, { &mut __tmp_5 })
            },
            {
                let mut __tmp_6: () = 4_();
                let mut __tmp_7: () = 4.5_f64;
                std_pair_const_int__double_::new_1({ &mut __tmp_6 }, { &mut __tmp_7 })
            },
            {
                let mut __tmp_8: () = 5_();
                let mut __tmp_9: () = 5.5_f64;
                std_pair_const_int__double_::new_1({ &mut __tmp_8 }, { &mut __tmp_9 })
            },
            {
                let mut __tmp_10: () = 7_();
                let mut __tmp_11: () = 7.5_f64;
                std_pair_const_int__double_::new_1({ &mut __tmp_10 }, { &mut __tmp_11 })
            },
            {
                let mut __tmp_12: () = 8_();
                let mut __tmp_13: () = 8.5_f64;
                std_pair_const_int__double_::new_1({ &mut __tmp_12 }, { &mut __tmp_13 })
            },
        ];
        let mut m: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        (*unimplemented!()) = -1.5_f64;
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut ar: [(); 7] = [
            {
                let mut __tmp_14: () = 1_();
                let mut __tmp_15: () = 1.5_f64;
                std_pair_const_int__double_::new_1({ &mut __tmp_14 }, { &mut __tmp_15 })
            },
            {
                let mut __tmp_16: () = 2_();
                let mut __tmp_17: () = 2.5_f64;
                std_pair_const_int__double_::new_1({ &mut __tmp_16 }, { &mut __tmp_17 })
            },
            {
                let mut __tmp_18: () = 3_();
                let mut __tmp_19: () = 3.5_f64;
                std_pair_const_int__double_::new_1({ &mut __tmp_18 }, { &mut __tmp_19 })
            },
            {
                let mut __tmp_20: () = 4_();
                let mut __tmp_21: () = 4.5_f64;
                std_pair_const_int__double_::new_1({ &mut __tmp_20 }, { &mut __tmp_21 })
            },
            {
                let mut __tmp_22: () = 5_();
                let mut __tmp_23: () = 5.5_f64;
                std_pair_const_int__double_::new_1({ &mut __tmp_22 }, { &mut __tmp_23 })
            },
            {
                let mut __tmp_24: () = 7_();
                let mut __tmp_25: () = 7.5_f64;
                std_pair_const_int__double_::new_1({ &mut __tmp_24 }, { &mut __tmp_25 })
            },
            {
                let mut __tmp_26: () = 8_();
                let mut __tmp_27: () = 8.5_f64;
                std_pair_const_int__double_::new_1({ &mut __tmp_26 }, { &mut __tmp_27 })
            },
        ];
        let m: () = unimplemented!();
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
    {
        let mut ar: [(); 7] = [
            {
                let mut __tmp_28: () = 1_();
                let mut __tmp_29: () = 1.5_f64;
                std_pair_const_int__double_::new_1({ &mut __tmp_28 }, { &mut __tmp_29 })
            },
            {
                let mut __tmp_30: () = 2_();
                let mut __tmp_31: () = 2.5_f64;
                std_pair_const_int__double_::new_1({ &mut __tmp_30 }, { &mut __tmp_31 })
            },
            {
                let mut __tmp_32: () = 3_();
                let mut __tmp_33: () = 3.5_f64;
                std_pair_const_int__double_::new_1({ &mut __tmp_32 }, { &mut __tmp_33 })
            },
            {
                let mut __tmp_34: () = 4_();
                let mut __tmp_35: () = 4.5_f64;
                std_pair_const_int__double_::new_1({ &mut __tmp_34 }, { &mut __tmp_35 })
            },
            {
                let mut __tmp_36: () = 5_();
                let mut __tmp_37: () = 5.5_f64;
                std_pair_const_int__double_::new_1({ &mut __tmp_36 }, { &mut __tmp_37 })
            },
            {
                let mut __tmp_38: () = 7_();
                let mut __tmp_39: () = 7.5_f64;
                std_pair_const_int__double_::new_1({ &mut __tmp_38 }, { &mut __tmp_39 })
            },
            {
                let mut __tmp_40: () = 8_();
                let mut __tmp_41: () = 8.5_f64;
                std_pair_const_int__double_::new_1({ &mut __tmp_40 }, { &mut __tmp_41 })
            },
        ];
        let mut m: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        (*unimplemented!()) = -1.5_f64;
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut ar: [(); 7] = [
            {
                let mut __tmp_42: () = 1_();
                let mut __tmp_43: () = 1.5_f64;
                std_pair_const_int__double_::new_1({ &mut __tmp_42 }, { &mut __tmp_43 })
            },
            {
                let mut __tmp_44: () = 2_();
                let mut __tmp_45: () = 2.5_f64;
                std_pair_const_int__double_::new_1({ &mut __tmp_44 }, { &mut __tmp_45 })
            },
            {
                let mut __tmp_46: () = 3_();
                let mut __tmp_47: () = 3.5_f64;
                std_pair_const_int__double_::new_1({ &mut __tmp_46 }, { &mut __tmp_47 })
            },
            {
                let mut __tmp_48: () = 4_();
                let mut __tmp_49: () = 4.5_f64;
                std_pair_const_int__double_::new_1({ &mut __tmp_48 }, { &mut __tmp_49 })
            },
            {
                let mut __tmp_50: () = 5_();
                let mut __tmp_51: () = 5.5_f64;
                std_pair_const_int__double_::new_1({ &mut __tmp_50 }, { &mut __tmp_51 })
            },
            {
                let mut __tmp_52: () = 7_();
                let mut __tmp_53: () = 7.5_f64;
                std_pair_const_int__double_::new_1({ &mut __tmp_52 }, { &mut __tmp_53 })
            },
            {
                let mut __tmp_54: () = 8_();
                let mut __tmp_55: () = 8.5_f64;
                std_pair_const_int__double_::new_1({ &mut __tmp_54 }, { &mut __tmp_55 })
            },
        ];
        let m: () = unimplemented!();
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
    (unsafe { test_7() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_0);
    std::cell::LazyCell::force(&*&raw const alloc_count_1);
    std::cell::LazyCell::force(&*&raw const dealloc_count_2);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_3);
}
