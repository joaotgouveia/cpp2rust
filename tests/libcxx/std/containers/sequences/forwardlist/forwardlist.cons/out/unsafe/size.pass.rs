extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn operator_eq_1(x: *const DefaultOnly, y: *const DefaultOnly) -> () {
    return (((*x).data_) == ((*y).data_));
}
pub unsafe fn operator_lt_2(x: *const DefaultOnly, y: *const DefaultOnly) -> () {
    return (((*x).data_) < ((*y).data_));
}
#[repr(C)]
#[derive(FnPtrArg, DestructorUnsafe)]
pub struct DefaultOnly {
    data_: (),
}
impl DefaultOnly {
    pub unsafe fn new() -> Self {
        let mut this = Self { data_: -1_() };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_0)).prefix_inc();
        this
    }
    pub unsafe fn destructor(&mut self) {
        self.data_ = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_0)).prefix_dec();
    }
}
impl std::cmp::Ord for DefaultOnly {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if operator_lt_2(self as *const DefaultOnly, other as *const DefaultOnly) {
                std::cmp::Ordering::Less
            } else if operator_lt_2(other as *const DefaultOnly, self as *const DefaultOnly) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for DefaultOnly {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for DefaultOnly {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_1(self as *const DefaultOnly, other as *const DefaultOnly) }
    }
}
impl std::cmp::Eq for DefaultOnly {}
impl Default for DefaultOnly {
    fn default() -> Self {
        unsafe { DefaultOnly::new() }
    }
}
pub static mut count_0: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub unsafe fn outstanding_alloc() -> () {
        unimplemented!();
        return ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_4))
            .wrapping_sub((*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_5))));
    }
    pub unsafe fn reset() {
        unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut disable_default_constructor_6)) = false;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut outstanding_bytes_3)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_4)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_5)) = 0_();
    }
}
pub static mut outstanding_bytes_3: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut alloc_count_4: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut dealloc_count_5: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut disable_default_constructor_6: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { false });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut DefaultOnly,
}
impl min_pointer_DefaultOnly__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    ptr_: (),
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
pub unsafe fn operator_eq_7(
    mut _a0: min_allocator_DefaultOnly_,
    mut _a1: min_allocator_DefaultOnly_,
) -> () {
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_DefaultOnly_ {}
impl std::cmp::PartialEq for min_allocator_DefaultOnly_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_7(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_allocator_DefaultOnly_ {}
pub unsafe fn check_allocator_8(mut n: (), alloc: Option<*const ()>) {
    let mut __alloc_default: Option<()> = None;
    let mut alloc: *const () =
        alloc.unwrap_or_else(|| unsafe { __alloc_default.insert(unimplemented!()) as *const () });
    let mut d: () = unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn check_allocator_9(mut n: (), alloc: Option<*const min_allocator_DefaultOnly_>) {
    let mut __alloc_default: Option<min_allocator_DefaultOnly_> = None;
    let mut alloc: *const min_allocator_DefaultOnly_ = alloc.unwrap_or_else(|| unsafe {
        __alloc_default.insert(<min_allocator_DefaultOnly_>::default())
            as *const min_allocator_DefaultOnly_
    });
    let mut d: () = unimplemented!();
    unimplemented!();
    unimplemented!();
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
        const _: () = assert!(true, "(std::is_constructible<C, std::size_t>::value)");;
        const _: () = assert!(true, "(!std::is_convertible<std::size_t, C>::value)");;
    }
    {
        let mut N: () = 10_();
        let mut c: () = unimplemented!();
        let mut n: () = 0_();
        let mut i : () = std___forward_list_const_iterator_std___forward_list_node_DefaultOnly__void_ptr__ptr_ :: new_1 ( { unimplemented!()  } , )  ;
        let mut e : () = std___forward_list_const_iterator_std___forward_list_node_DefaultOnly__void_ptr__ptr_ :: new_1 ( { unimplemented!()  } , )  ;
        'loop_: while unimplemented!() {
            unimplemented!();
            {
                unimplemented!();
                n.prefix_inc()
            };
        }
        unimplemented!();
    }
    {
        let mut N: () = 10_();
        let mut c: () = unimplemented!();
        let mut n: () = 0_();
        let mut i : () = std___forward_list_const_iterator_std___forward_list_node_DefaultOnly__void_ptr__ptr_ :: new_1 ( { unimplemented!()  } , )  ;
        let mut e : () = std___forward_list_const_iterator_std___forward_list_node_DefaultOnly__void_ptr__ptr_ :: new_1 ( { unimplemented!()  } , )  ;
        'loop_: while unimplemented!() {
            unimplemented!();
            {
                unimplemented!();
                n.prefix_inc()
            };
        }
        unimplemented!();
        (unsafe { check_allocator_8(0_(), None) });
        (unsafe { check_allocator_8(3_(), None) });
    }
    {
        let mut N: () = 10_();
        let mut c: () = unimplemented!();
        let mut n: () = 0_();
        let mut i : () = std___forward_list_const_iterator_min_pointer_std___forward_list_node_DefaultOnly__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0___ :: new_1 ( { unimplemented!()  } , )  ;
        let mut e : () = std___forward_list_const_iterator_min_pointer_std___forward_list_node_DefaultOnly__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0___ :: new_1 ( { unimplemented!()  } , )  ;
        'loop_: while unimplemented!() {
            unimplemented!();
            {
                unimplemented!();
                n.prefix_inc()
            };
        }
        unimplemented!();
        (unsafe { check_allocator_9(0_(), None) });
        (unsafe { check_allocator_9(3_(), None) });
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const count_0);
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_3);
    std::cell::LazyCell::force(&*&raw const alloc_count_4);
    std::cell::LazyCell::force(&*&raw const dealloc_count_5);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_6);
}
