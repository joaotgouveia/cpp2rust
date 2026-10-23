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
pub struct TestAllocator_int_ {}
impl TestAllocator_int_ {
    pub unsafe fn allocate(&mut self, mut n: ()) -> *mut () {
        return Box::leak((0..n).map(|_| unimplemented!()).collect::<Box<[()]>>()).as_mut_ptr();
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut (), mut _a1: ()) {
        {
            let __p = p;
            if !__p.is_null() {
                ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                    __p,
                    libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                        / ::std::mem::size_of::<()>(),
                )))
            }
        };
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestAllocator_double_ {}
impl TestAllocator_double_ {
    pub unsafe fn allocate(&mut self, mut n: ()) -> *mut () {
        return Box::leak((0..n).map(|_| unimplemented!()).collect::<Box<[()]>>()).as_mut_ptr();
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut (), mut _a1: ()) {
        {
            let __p = p;
            if !__p.is_null() {
                ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                    __p,
                    libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                        / ::std::mem::size_of::<()>(),
                )))
            }
        };
    }
}
pub unsafe fn copy_0(mut copy_vector: ()) {}
pub unsafe fn fn_1(v: *mut (), mut v3: ()) {
    unimplemented!();
    let mut x: () = unimplemented!();
    let mut v4: *mut () = (&mut v3 as *mut ());
    let mut v2: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    x = (*unimplemented!());
    (*unimplemented!()) = 1_();
    (*unimplemented!()) = 7_();
    (*unimplemented!()) = 13_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut v1: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut s1: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (*unimplemented!()) = 40_();
    (*unimplemented!()) = 50_();
    unimplemented!();
    unimplemented!();
    let mut v2: () = unimplemented!();
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
    (unsafe { copy_0(unimplemented!()) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut s2: () = unimplemented!();
    let mut v3: () = unimplemented!();
    unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((i) < (100_())) {
        unimplemented!();
        i.prefix_inc();
    }
    let mut v6: () = unimplemented!();
    unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((i) < (s2)) {
        unimplemented!();
        i.prefix_inc();
    }
    let mut p1: *const () = (unimplemented!()).cast_const();
    unimplemented!();
    let mut p2: *mut () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (*p2) = 99_f64;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    p2.prefix_inc();
    (*p2) = 98_();
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
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let ref0: *mut () = unimplemented!();
    (*ref0) = 5_f64;
    unimplemented!();
    let mut x0: () = (*unimplemented!());
    unimplemented!();
    x0 = 6_f64;
    unimplemented!();
    let mut idx: () = 0_();
    unimplemented!();
    unimplemented!();
    let ref1: *mut () = unimplemented!();
    (*ref1) += 1.5_f64;
    unimplemented!();
    let mut x1: () = (*unimplemented!());
    unimplemented!();
    x1 -= 1.5_f64;
    unimplemented!();
    let mut v7: () = unimplemented!();
    let mut v8: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (unsafe { fn_1(&mut v7, unimplemented!()) });
    let mut src: [u32; 3] = [1_u32, 2_u32, 3_u32];
    let mut v9: () = std_vector_unsigned_int__std_allocator_unsigned_int__::new_1(
        { (src.as_mut_ptr() as *mut u32).cast::<()>() },
        { (src.as_mut_ptr().offset((3_()) as isize) as *mut u32).cast::<()>() },
        None,
    );
    unimplemented!();
    unimplemented!();
    let mut v10: () = std_vector_unsigned_long__std_allocator_unsigned_long__::new_1(
        { (src.as_mut_ptr() as *mut u32).cast::<()>() },
        { (src.as_mut_ptr().offset((3_()) as isize) as *mut u32).cast::<()>() },
        None,
    );
    unimplemented!();
    unimplemented!();
    let mut v11: () = std_vector_int__std_allocator_int__::new_1(
        { (src.as_mut_ptr() as *mut u32).cast::<()>() },
        { (src.as_mut_ptr().offset((3_()) as isize) as *mut u32).cast::<()>() },
        None,
    );
    unimplemented!();
    unimplemented!();
    let src1: [u32; 3] = [1_u32, 2_u32, 3_u32];
    let mut v12: () = std_vector_unsigned_int__std_allocator_unsigned_int__::new_2(
        { (src1.as_ptr() as *const u32).cast::<()>() },
        { unimplemented!() },
        None,
    );
    unimplemented!();
    unimplemented!();
    let mut buf: [u8; 5] = [10_u8, 20_u8, 30_u8, 40_u8, 50_u8];
    let mut start: *const u8 = (buf.as_mut_ptr()).cast_const();
    let mut len: () = 5_();
    let mut v13: () = std_vector_unsigned_char__std_allocator_unsigned_char__::new_1(
        { (start as *const u8).cast::<()>() },
        { (start.offset((len) as isize) as *const u8).cast::<()>() },
        None,
    );
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
