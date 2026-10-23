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
pub struct min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0__ {
    ptr_: *mut (),
}
impl min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0__ {}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {
    pub unsafe fn new_1(
        mut p : min_pointer_std___tree_node_std___value_type_int__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__,
    ) -> Self {
        let mut this = Self {
            ptr_: (p.ptr_ as ()),
        };
        this
    }
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {
    pub unsafe fn new_2(
        mut p : min_pointer_std___tree_end_node_min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__,
    ) -> Self {
        let mut this = Self {
            ptr_: (p.ptr_ as ()),
        };
        this
    }
}
pub unsafe fn operator_eq_4(
    mut x: min_pointer_void__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_void__std_integral_constant_unsigned_long__0__,
) -> () {
    return ((x.ptr_) == (y.ptr_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    ptr_: (),
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
impl std::cmp::PartialEq for min_pointer_void__std_integral_constant_unsigned_long__0__ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_4(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_pointer_void__std_integral_constant_unsigned_long__0__ {}
pub unsafe fn operator_eq_5(
    mut x : min_pointer_std___tree_node_std___value_type_int__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_std___tree_node_std___value_type_int__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__,
) -> () {
    return ((x.ptr_) == (y.ptr_));
}
pub unsafe fn operator_ne_6(
    mut x : min_pointer_std___tree_node_std___value_type_int__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_std___tree_node_std___value_type_int__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__,
) -> () {
    return !(unsafe {
        let _x: min_pointer_std___tree_node_std___value_type_int__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__  = x ;
        operator_eq_5(_x, y)
    });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_std___tree_node_std___value_type_int__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ { ptr_ : *mut std___tree_node_std___value_type_int__int___min_pointer_void__std_integral_constant_unsigned_long__0___ , }
impl min_pointer_std___tree_node_std___value_type_int__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ { unsafe fn new_1 ( mut p : *mut std___tree_node_std___value_type_int__int___min_pointer_void__std_integral_constant_unsigned_long__0___ , ) -> Self { let mut this = Self { ptr_ : p , } ;
 this } pub unsafe fn new_2 ( mut _a0 : () , ) -> Self { let mut this = Self { ptr_ : std::ptr::null_mut() , } ;
 this } pub unsafe fn to_ ( &self , ) -> () { return ! ( ( self . ptr_  ).is_null() )  ;
 } pub unsafe fn operator_deref ( &self , ) -> *mut std___tree_node_std___value_type_int__int___min_pointer_void__std_integral_constant_unsigned_long__0___ { return & mut ( * self . ptr_   )  ;
 } pub unsafe fn operator_arrow ( &self , ) -> *mut std___tree_node_std___value_type_int__int___min_pointer_void__std_integral_constant_unsigned_long__0___ { return self . ptr_   ;
 } pub unsafe fn pointer_to ( t : *mut std___tree_node_std___value_type_int__int___min_pointer_void__std_integral_constant_unsigned_long__0___ , ) -> min_pointer_std___tree_node_std___value_type_int__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ { return min_pointer_std___tree_node_std___value_type_int__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ :: new_1 ( { unimplemented!()  } , )   ;
 } }
impl std::cmp::PartialEq for min_pointer_std___tree_node_std___value_type_int__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ { fn eq(&self, other: &Self) -> bool { unsafe { operator_eq_5(self.clone(), other.clone()) }}}
impl std::cmp::Eq for min_pointer_std___tree_node_std___value_type_int__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ {}
pub unsafe fn operator_eq_7(
    mut x : min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__,
) -> () {
    return ((x.ptr_) == (y.ptr_));
}
pub unsafe fn operator_ne_8(
    mut x : min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__,
) -> () {
    return !(unsafe {
        let _x: min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__  = x ;
        operator_eq_7(_x, y)
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
impl std::cmp::PartialEq for min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ { fn eq(&self, other: &Self) -> bool { unsafe { operator_eq_7(self.clone(), other.clone()) }}}
impl std::cmp::Eq for min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ {}
pub unsafe fn operator_eq_9(
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
impl std::cmp::PartialEq for min_pointer_std___tree_end_node_min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ { fn eq(&self, other: &Self) -> bool { unsafe { operator_eq_9(self.clone(), other.clone()) }}}
impl std::cmp::Eq for min_pointer_std___tree_end_node_min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ {}
impl min_allocator_std_pair_const_int__int__ {
    pub unsafe fn new_1(
        mut _a0 : min_allocator_std___tree_node_std___value_type_int__int___min_pointer_void__std_integral_constant_unsigned_long__0____,
    ) -> Self {
        let mut this = Self {};
        this
    }
}
pub unsafe fn operator_eq_10(
    mut _a0: min_allocator_std_pair_const_int__int__,
    mut _a1: min_allocator_std_pair_const_int__int__,
) -> () {
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_std_pair_const_int__int__ {}
impl std::cmp::PartialEq for min_allocator_std_pair_const_int__int__ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_10(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_allocator_std_pair_const_int__int__ {}
impl min_allocator_std___tree_node_std___value_type_int__int___min_pointer_void__std_integral_constant_unsigned_long__0____ { pub unsafe fn new_1 ( mut _a0 : min_allocator_std_pair_const_int__int__ , ) -> Self { let mut this = Self { } ;
 this } }
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_std___tree_node_std___value_type_int__int___min_pointer_void__std_integral_constant_unsigned_long__0____
{}
impl min_allocator_std___tree_node_std___value_type_int__int___min_pointer_void__std_integral_constant_unsigned_long__0____ { pub unsafe fn allocate ( &mut self , mut n : () , ) -> min_pointer_std___tree_node_std___value_type_int__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ { return min_pointer_std___tree_node_std___value_type_int__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ :: new_1 ( { unimplemented!()  } , )   ;
 } pub unsafe fn deallocate ( &mut self , mut p : min_pointer_std___tree_node_std___value_type_int__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ , mut n : () , ) { unimplemented!()  ;
 } }
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0____
{}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
impl min_allocator_std_pair_const_int__int__ {}
impl min_allocator_std___tree_node_std___value_type_int__int___min_pointer_void__std_integral_constant_unsigned_long__0____ { }
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Counter_base {}
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe, DestructorUnsafe, Default)]
pub struct Counter_int_ {
    data_: (),
}
impl Counter_int_ {
    pub unsafe fn new_1(data: *const ()) -> Self {
        let mut this = Self { data_: (*data) };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut gConstructed_11)).prefix_inc();
        this
    }
    pub unsafe fn copy_from(rhs: *const Counter_int_) -> Self {
        let mut this = Self {
            data_: (*rhs).data_,
        };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut gConstructed_11)).prefix_inc();
        this
    }
    pub unsafe fn copy_assign(&mut self, rhs: *const Counter_int_) -> *mut Counter_int_ {
        self.data_ = (*rhs).data_;
        return &mut (*(self as *mut Counter_int_));
    }
    pub unsafe fn destructor(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut gConstructed_11)).prefix_dec();
    }
    pub unsafe fn operator_eq(&self, x: *const Counter_int_) -> () {
        return ((self.data_) == ((*x).data_));
    }
    pub unsafe fn operator_lt(&self, x: *const Counter_int_) -> () {
        return ((self.data_) < ((*x).data_));
    }
}
impl std::cmp::Ord for Counter_int_ {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if Counter_int_::operator_lt(self, other as *const Counter_int_) {
                std::cmp::Ordering::Less
            } else if Counter_int_::operator_lt(other, self as *const Counter_int_) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for Counter_int_ {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for Counter_int_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { Counter_int_::operator_eq(self, other as *const Counter_int_) }
    }
}
impl std::cmp::Eq for Counter_int_ {}
impl Clone for Counter_int_ {
    fn clone(&self) -> Self {
        unsafe { Counter_int_::copy_from(self as *const Counter_int_) }
    }
}
pub static mut gConstructed_11: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub unsafe fn test_12(c: *mut (), mut first: *mut (), mut last: *mut ()) {
    let mut sz: () = unimplemented!();
    unimplemented!();
    let mut copy: *mut () = first;
    'loop_: while ((copy) != (last)) {
        let mut t: () = unimplemented!();
        unimplemented!();
        sz.prefix_dec();
        {
            unimplemented!();
            (*unimplemented!()) = ((*first) as ());
            unimplemented!();
        }
        unimplemented!();
        unimplemented!();
        copy.prefix_inc();
    }
    unimplemented!();
    let mut copy: *mut () = first;
    'loop_: while ((copy) != (last)) {
        let mut t: () = unimplemented!();
        unimplemented!();
        copy.prefix_inc();
    }
}
pub unsafe fn test_13(c: *mut (), mut first: *mut Counter_int_, mut last: *mut Counter_int_) {
    let mut sz: () = unimplemented!();
    unimplemented!();
    let mut copy: *mut Counter_int_ = first;
    'loop_: while ((copy) != (last)) {
        let mut t: () = unimplemented!();
        unimplemented!();
        sz.prefix_dec();
        {
            unimplemented!();
            (unsafe {
                let _rhs: *const Counter_int_ = &(*first);
                Counter_int_::copy_assign(&mut (*unimplemented!()), _rhs)
            });
            unimplemented!();
        }
        unimplemented!();
        unimplemented!();
        copy.prefix_inc();
    }
    unimplemented!();
    let mut copy: *mut Counter_int_ = first;
    'loop_: while ((copy) != (last)) {
        let mut t: () = unimplemented!();
        unimplemented!();
        copy.prefix_inc();
    }
}
pub unsafe fn test_14(c: *mut (), mut first: *mut (), mut last: *mut ()) {
    let mut sz: () = unimplemented!();
    unimplemented!();
    let mut copy: *mut () = first;
    'loop_: while ((copy) != (last)) {
        let mut t: () = unimplemented!();
        unimplemented!();
        sz.prefix_dec();
        {
            unimplemented!();
            (*unimplemented!()) = ((*first) as ());
            unimplemented!();
        }
        unimplemented!();
        unimplemented!();
        copy.prefix_inc();
    }
    unimplemented!();
    let mut copy: *mut () = first;
    'loop_: while ((copy) != (last)) {
        let mut t: () = unimplemented!();
        unimplemented!();
        copy.prefix_inc();
    }
}
pub unsafe fn test_15() -> () {
    {
        let mut m: () = unimplemented!();
        let mut keys: [(); 6] = [1_(), 2_(), 3_(), 4_(), 5_(), 6_()];
        (unsafe {
            let _first: *mut () = unimplemented!();
            let _last: *mut () = unimplemented!();
            test_12(&mut m, _first, _last)
        });
    }
    {
        let mut m: () = unimplemented!();
        {
            let mut keys: [Counter_int_; 6] = [
                {
                    let mut __tmp_0: () = 1_();
                    Counter_int_::new_1({ &mut __tmp_0 })
                },
                {
                    let mut __tmp_1: () = 2_();
                    Counter_int_::new_1({ &mut __tmp_1 })
                },
                {
                    let mut __tmp_2: () = 3_();
                    Counter_int_::new_1({ &mut __tmp_2 })
                },
                {
                    let mut __tmp_3: () = 4_();
                    Counter_int_::new_1({ &mut __tmp_3 })
                },
                {
                    let mut __tmp_4: () = 5_();
                    Counter_int_::new_1({ &mut __tmp_4 })
                },
                {
                    let mut __tmp_5: () = 6_();
                    Counter_int_::new_1({ &mut __tmp_5 })
                },
            ];
            unimplemented!();
            (unsafe {
                let _first: *mut Counter_int_ = unimplemented!();
                let _last: *mut Counter_int_ = unimplemented!();
                test_13(&mut m, _first, _last)
            });
        }
        unimplemented!();
    }
    {
        let mut m: () = unimplemented!();
        let mut keys: [(); 6] = [1_(), 2_(), 3_(), 4_(), 5_(), 6_()];
        (unsafe {
            let _first: *mut () = unimplemented!();
            let _last: *mut () = unimplemented!();
            test_14(&mut m, _first, _last)
        });
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
    (unsafe { test_15() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_0);
    std::cell::LazyCell::force(&*&raw const alloc_count_1);
    std::cell::LazyCell::force(&*&raw const dealloc_count_2);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_3);
    std::cell::LazyCell::force(&*&raw const gConstructed_11);
}
