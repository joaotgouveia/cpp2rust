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
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_const_std_pair_const_int__int___std_integral_constant_unsigned_long__0__ {
    ptr_: *const (),
}
impl min_pointer_const_std_pair_const_int__int___std_integral_constant_unsigned_long__0__ {
    unsafe fn new_1(mut p: *const ()) -> Self {
        let mut this = Self { ptr_: p };
        this
    }
    pub unsafe fn operator_arrow(&self) -> *const () {
        return self.ptr_;
    }
    pub unsafe fn pointer_to(
        t: *const (),
    ) -> min_pointer_const_std_pair_const_int__int___std_integral_constant_unsigned_long__0__ {
        return min_pointer_const_std_pair_const_int__int___std_integral_constant_unsigned_long__0__ :: new_1 ( { unimplemented!()  } , )   ;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_const_void__std_integral_constant_unsigned_long__0__ {
    ptr_: (),
}
impl min_pointer_const_void__std_integral_constant_unsigned_long__0__ {}
pub unsafe fn operator_eq_5(
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
        unsafe { operator_eq_5(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_allocator_std_pair_const_int__int__ {}
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
        (*std::cell::LazyCell::force_mut(&mut *&raw mut gConstructed_6)).prefix_inc();
        this
    }
    pub unsafe fn copy_from(rhs: *const Counter_int_) -> Self {
        let mut this = Self {
            data_: (*rhs).data_,
        };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut gConstructed_6)).prefix_inc();
        this
    }
    pub unsafe fn copy_assign(&mut self, rhs: *const Counter_int_) -> *mut Counter_int_ {
        self.data_ = (*rhs).data_;
        return &mut (*(self as *mut Counter_int_));
    }
    pub unsafe fn destructor(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut gConstructed_6)).prefix_dec();
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
pub static mut gConstructed_6: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub unsafe fn test_7(c: *mut ()) -> () {
    let mut sz: () = unimplemented!();
    let mut some_key: () = (*unimplemented!()).first;
    let mut first: () = unimplemented!();
    'loop_: while unimplemented!() {
        let mut key_value: () = (*unimplemented!()).first;
        let mut t: () = unimplemented!();
        sz.prefix_dec();
        {
            unimplemented!();
            (*unimplemented!()) = (some_key as ());
            unimplemented!();
        }
        unimplemented!();
        unimplemented!();
    }
    unimplemented!();
    return true;
}
pub unsafe fn test_8(c: *mut ()) -> () {
    let mut sz: () = unimplemented!();
    let mut some_key : () = ( * ( unsafe { min_pointer_const_std_pair_const_int__int___std_integral_constant_unsigned_long__0__ :: operator_arrow ( & unimplemented!()   , ) } )   ) . first  ;
    let mut first: () = unimplemented!();
    'loop_: while unimplemented!() {
        let mut key_value : () = ( * ( unsafe { min_pointer_const_std_pair_const_int__int___std_integral_constant_unsigned_long__0__ :: operator_arrow ( & unimplemented!()   , ) } )   ) . first  ;
        let mut t: () = unimplemented!();
        sz.prefix_dec();
        {
            unimplemented!();
            (*unimplemented!()) = (some_key as ());
            unimplemented!();
        }
        unimplemented!();
        unimplemented!();
    }
    unimplemented!();
    return true;
}
pub unsafe fn test_9(c: *mut ()) -> () {
    let mut sz: () = unimplemented!();
    let mut some_key: Counter_int_ = Counter_int_::copy_from({ &(*unimplemented!()).first });
    let _dtor_some_key = ScopedDestructorUnsafe::new(&raw mut some_key, Counter_int_::destructor);
    let mut first: () = unimplemented!();
    'loop_: while unimplemented!() {
        let mut key_value: Counter_int_ = Counter_int_::copy_from({ &(*unimplemented!()).first });
        let _dtor_key_value =
            ScopedDestructorUnsafe::new(&raw mut key_value, Counter_int_::destructor);
        let mut t: () = unimplemented!();
        sz.prefix_dec();
        {
            unimplemented!();
            (unsafe { Counter_int_::copy_assign(&mut (*unimplemented!()), &some_key) });
            unimplemented!();
        }
        unimplemented!();
        unimplemented!();
    }
    unimplemented!();
    return true;
}
pub unsafe fn test_10() -> () {
    {
        let mut m: () = unimplemented!();
        (unsafe { test_7(&mut m) });
    }
    {
        let mut m: () = unimplemented!();
        (unsafe { test_8(&mut m) });
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
    {
        let mut m: () = unimplemented!();
        unimplemented!();
        (unsafe { test_9(&mut m) });
        unimplemented!();
    }
    (unsafe { test_10() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_0);
    std::cell::LazyCell::force(&*&raw const alloc_count_1);
    std::cell::LazyCell::force(&*&raw const dealloc_count_2);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_3);
    std::cell::LazyCell::force(&*&raw const gConstructed_6);
}
