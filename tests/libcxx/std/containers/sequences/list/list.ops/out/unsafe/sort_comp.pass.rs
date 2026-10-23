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
pub struct min_pointer_int__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut (),
}
impl min_pointer_int__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    ptr_: (),
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
    ptr_: *const (),
}
impl min_pointer_const_int__std_integral_constant_unsigned_long__0__ {}
pub unsafe fn operator_eq_4(
    mut x : min_pointer_std___list_node_int__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_std___list_node_int__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__,
) -> () {
    return ((x.ptr_) == (y.ptr_));
}
pub unsafe fn operator_ne_5(
    mut x : min_pointer_std___list_node_int__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_std___list_node_int__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__,
) -> () {
    return !(unsafe {
        let _x: min_pointer_std___list_node_int__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__  = x ;
        operator_eq_4(_x, y)
    });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_std___list_node_int__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__
{
    ptr_: *mut std___list_node_int__min_pointer_void__std_integral_constant_unsigned_long__0___,
}
impl min_pointer_std___list_node_int__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ { unsafe fn new_1 ( mut p : *mut std___list_node_int__min_pointer_void__std_integral_constant_unsigned_long__0___ , ) -> Self { let mut this = Self { ptr_ : p , } ;
 this } pub unsafe fn new_2 ( mut _a0 : () , ) -> Self { let mut this = Self { ptr_ : std::ptr::null_mut() , } ;
 this } pub unsafe fn operator_deref ( &self , ) -> *mut std___list_node_int__min_pointer_void__std_integral_constant_unsigned_long__0___ { return & mut ( * self . ptr_   )  ;
 } pub unsafe fn operator_arrow ( &self , ) -> *mut std___list_node_int__min_pointer_void__std_integral_constant_unsigned_long__0___ { return self . ptr_   ;
 } pub unsafe fn pointer_to ( t : *mut std___list_node_int__min_pointer_void__std_integral_constant_unsigned_long__0___ , ) -> min_pointer_std___list_node_int__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ { return min_pointer_std___list_node_int__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ :: new_1 ( { unimplemented!()  } , )   ;
 } }
impl std::cmp::PartialEq for min_pointer_std___list_node_int__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ { fn eq(&self, other: &Self) -> bool { unsafe { operator_eq_4(self.clone(), other.clone()) }}}
impl std::cmp::Eq for min_pointer_std___list_node_int__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ {}
pub unsafe fn operator_eq_6(
    mut x : min_pointer_std___list_node_base_int__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_std___list_node_base_int__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__,
) -> () {
    return ((x.ptr_) == (y.ptr_));
}
pub unsafe fn operator_ne_7(
    mut x : min_pointer_std___list_node_base_int__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_std___list_node_base_int__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__,
) -> () {
    return !(unsafe {
        let _x: min_pointer_std___list_node_base_int__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__  = x ;
        operator_eq_6(_x, y)
    });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_std___list_node_base_int__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__
{
    ptr_:
        *mut std___list_node_base_int__min_pointer_void__std_integral_constant_unsigned_long__0___,
}
impl min_pointer_std___list_node_base_int__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ { unsafe fn new_1 ( mut p : *mut std___list_node_base_int__min_pointer_void__std_integral_constant_unsigned_long__0___ , ) -> Self { let mut this = Self { ptr_ : p , } ;
 this } pub unsafe fn new_2 ( mut _a0 : () , ) -> Self { let mut this = Self { ptr_ : std::ptr::null_mut() , } ;
 this } pub unsafe fn operator_arrow ( &self , ) -> *mut std___list_node_base_int__min_pointer_void__std_integral_constant_unsigned_long__0___ { return self . ptr_   ;
 } pub unsafe fn pointer_to ( t : *mut std___list_node_base_int__min_pointer_void__std_integral_constant_unsigned_long__0___ , ) -> min_pointer_std___list_node_base_int__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ { return min_pointer_std___list_node_base_int__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ :: new_1 ( { unimplemented!()  } , )   ;
 } }
impl std::cmp::PartialEq for min_pointer_std___list_node_base_int__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ { fn eq(&self, other: &Self) -> bool { unsafe { operator_eq_6(self.clone(), other.clone()) }}}
impl std::cmp::Eq for min_pointer_std___list_node_base_int__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_int_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_std___list_node_int__min_pointer_void__std_integral_constant_unsigned_long__0____
{}
impl
    min_allocator_std___list_node_int__min_pointer_void__std_integral_constant_unsigned_long__0____
{
    pub unsafe fn allocate ( &mut self , mut n : () , ) -> min_pointer_std___list_node_int__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__{
        return min_pointer_std___list_node_int__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ :: new_1 ( { unimplemented!()  } , )   ;
    }
    pub unsafe fn deallocate(
        &mut self,
        mut p : min_pointer_std___list_node_int__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__,
        mut n: (),
    ) {
        unimplemented!();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_std___list_node_base_int__min_pointer_void__std_integral_constant_unsigned_long__0____
{}
pub static mut randomness_8: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Payload {
    pub val: (),
    pub side: (),
}
impl Payload {
    pub unsafe fn new_1(mut v: ()) -> Self {
        let mut this = Self { val: v, side: 0_() };
        this
    }
    pub unsafe fn new_2(mut v: (), mut s: ()) -> Self {
        let mut this = Self { val: v, side: s };
        this
    }
    pub unsafe fn operator_lt(&self, rhs: *const Payload) -> () {
        return ((self.val) < ((*rhs).val));
    }
}
impl std::cmp::Ord for Payload {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if Payload::operator_lt(self, other as *const Payload) {
                std::cmp::Ordering::Less
            } else if Payload::operator_lt(other, self as *const Payload) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for Payload {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for Payload {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            !(Payload::operator_lt(self, other as *const Payload))
                && !(Payload::operator_lt(other, self as *const Payload))
        }
    }
}
impl std::cmp::Eq for Payload {}
pub unsafe fn greater_9(lhs: *const Payload, rhs: *const Payload) -> () {
    return (((*lhs).val) > ((*rhs).val));
}
pub unsafe fn test_stable_10(mut N: ()) {
    let mut v: () = unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((i) < (N)) {
        unimplemented!();
        i.prefix_inc();
    }
    unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((i) < (N)) {
        (*unimplemented!()).side = i;
        i.prefix_inc();
    }
    let mut c: () = std_list_Payload__std_allocator_Payload__::new_1({ unimplemented!() }, {
        unimplemented!()
    });
    unimplemented!();
    unimplemented!();
    let mut j: () = {
        let mut __tmp_0: () = unimplemented!();
        std___list_const_iterator_Payload__void_ptr_::new_1({ &mut __tmp_0 })
    };
    let mut i: () = 0_();
    'loop_: while ((i) < (N)) {
        unimplemented!();
        {
            i.prefix_inc();
            unimplemented!()
        };
    }
    let mut it: () = {
        let mut __tmp_1: () = unimplemented!();
        std___list_const_iterator_Payload__void_ptr_::new_1({ &mut __tmp_1 })
    };
    'loop_: while unimplemented!() {
        let mut next: () = (unsafe { next_11(it.clone(), None) });
        if (unimplemented!()) && (((*it).val) == ((*next).val)) {
            unimplemented!();
        }
        unimplemented!();
    }
}
pub unsafe fn test_12() -> () {
    {
        let mut a1: [(); 12] = [
            4_(),
            8_(),
            1_(),
            0_(),
            5_(),
            7_(),
            2_(),
            3_(),
            6_(),
            11_(),
            10_(),
            9_(),
        ];
        let mut a2: [(); 12] = [
            11_(),
            10_(),
            9_(),
            8_(),
            7_(),
            6_(),
            5_(),
            4_(),
            3_(),
            2_(),
            1_(),
            0_(),
        ];
        let mut c1: () = std_list_int__std_allocator_int__::new_1({ a1.as_mut_ptr() }, {
            a1.as_mut_ptr().offset(
                ((::std::mem::size_of::<[(); 12]>() as ())
                    .wrapping_div((::std::mem::size_of::<()>() as ()))) as isize,
            )
        });
        unimplemented!();
        unimplemented!();
    }
    {
        {
            let mut a1: [(); 12] = [
                4_(),
                8_(),
                1_(),
                0_(),
                5_(),
                7_(),
                2_(),
                3_(),
                6_(),
                11_(),
                10_(),
                9_(),
            ];
            let mut a2: [(); 12] = [
                11_(),
                10_(),
                9_(),
                8_(),
                7_(),
                6_(),
                5_(),
                4_(),
                3_(),
                2_(),
                1_(),
                0_(),
            ];
            let mut c1: () = std_list_int__min_allocator_int__::new_1({ a1.as_mut_ptr() }, {
                a1.as_mut_ptr().offset(
                    ((::std::mem::size_of::<[(); 12]>() as ())
                        .wrapping_div((::std::mem::size_of::<()>() as ())))
                        as isize,
                )
            });
            unimplemented!();
            unimplemented!();
        }
        let mut i: () = 0_();
        'loop_: while ((i) < (40_())) {
            (unsafe { test_stable_10(i) });
            i.prefix_inc();
        }
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
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_0);
    std::cell::LazyCell::force(&*&raw const alloc_count_1);
    std::cell::LazyCell::force(&*&raw const dealloc_count_2);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_3);
    std::cell::LazyCell::force(&*&raw const randomness_8);
}
