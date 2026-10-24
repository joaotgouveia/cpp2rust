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
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_std_pair_const_int__double__ {}
pub unsafe fn test_4() -> () {
    {
        let mut ar: [(); 24] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        let mut m: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut i: () = unimplemented!();
        unimplemented!();
        let mut k : () = std___map_const_iterator_std___tree_const_iterator_std___value_type_int__double___std___tree_node_std___value_type_int__double___void_ptr__ptr__long__ :: new_1 ( { i .clone() } , )  ;
        unimplemented!();
        let mut j: () = 1_();
        'loop_: while ((j) <= (unimplemented!())) {
            unimplemented!();
            unimplemented!();
            (*unimplemented!()).second = 2.5_f64;
            unimplemented!();
            {
                j.prefix_inc();
                unimplemented!()
            };
        }
        unimplemented!();
        let mut j: () = unimplemented!();
        'loop_: while ((j) >= (1_())) {
            unimplemented!();
            unimplemented!();
            unimplemented!();
            (*unimplemented!()).second = 1_();
            unimplemented!();
            j.prefix_dec();
        }
        unimplemented!();
    }
    {
        let mut ar: [(); 24] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        let m: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut i: () = unimplemented!();
        unimplemented!();
        let mut j: () = 1_();
        'loop_: while ((j) <= (unimplemented!())) {
            unimplemented!();
            unimplemented!();
            {
                j.prefix_inc();
                unimplemented!()
            };
        }
        unimplemented!();
        let mut j: () = unimplemented!();
        'loop_: while ((j) >= (1_())) {
            unimplemented!();
            unimplemented!();
            unimplemented!();
            j.prefix_dec();
        }
        unimplemented!();
    }
    {
        let mut ar: [(); 24] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        let mut m: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut i: () = unimplemented!();
        unimplemented!();
        let mut k : () = std___map_const_iterator_std___tree_const_iterator_std___value_type_int__double___min_pointer_std___tree_node_std___value_type_int__double___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____long__ :: new_1 ( { i .clone() } , )  ;
        unimplemented!();
        let mut j: () = 1_();
        'loop_: while ((j) <= (unimplemented!())) {
            unimplemented!();
            unimplemented!();
            ( * ( unsafe { min_pointer_std_pair_const_int__double___std_integral_constant_unsigned_long__0__ :: operator_arrow ( & unimplemented!()   , ) } )   ) . second   = 2.5_f64  ;
            unimplemented!();
            {
                j.prefix_inc();
                unimplemented!()
            };
        }
        unimplemented!();
        let mut j: () = unimplemented!();
        'loop_: while ((j) >= (1_())) {
            unimplemented!();
            unimplemented!();
            unimplemented!();
            ( * ( unsafe { min_pointer_std_pair_const_int__double___std_integral_constant_unsigned_long__0__ :: operator_arrow ( & unimplemented!()   , ) } )   ) . second   = 1_()  ;
            unimplemented!();
            j.prefix_dec();
        }
        unimplemented!();
    }
    {
        let mut ar: [(); 24] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        let m: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut i: () = unimplemented!();
        unimplemented!();
        let mut j: () = 1_();
        'loop_: while ((j) <= (unimplemented!())) {
            unimplemented!();
            unimplemented!();
            {
                j.prefix_inc();
                unimplemented!()
            };
        }
        unimplemented!();
        let mut j: () = unimplemented!();
        'loop_: while ((j) >= (1_())) {
            unimplemented!();
            unimplemented!();
            unimplemented!();
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
    (unsafe { test_4() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_0);
    std::cell::LazyCell::force(&*&raw const alloc_count_1);
    std::cell::LazyCell::force(&*&raw const dealloc_count_2);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_3);
}
