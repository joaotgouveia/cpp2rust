extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut v1: std_vector_int__std_allocator_int__ =
        <std_vector_int__std_allocator_int__>::default();
    (unsafe {
        let mut ___x: () = 1_();
        v1.push_back_58(&mut ___x)
    });
    (unsafe {
        let mut ___x: () = 2_();
        v1.push_back_58(&mut ___x)
    });
    (unsafe {
        let mut ___x: () = 3_();
        v1.push_back_58(&mut ___x)
    });
    let mut sum: () = 0_();
    'loop_: for x in IterRange::new((unsafe { v1.begin_26() }), (unsafe { v1.end_28() })) {
        let mut x: () = (*x);
        sum += x.prefix_inc();
    }
    'loop_: for x in IterRange::new((unsafe { v1.begin_26() }), (unsafe { v1.end_28() })) {
        let x: () = (*x);
        sum += x;
    }
    'loop_: for x in IterRange::new((unsafe { v1.begin_26() }), (unsafe { v1.end_28() })) {
        let x: *mut () = &mut (*x);
        (*x) += 10_();
    }
    'loop_: for x in IterRange::new((unsafe { v1.begin_26() }), (unsafe { v1.end_28() })) {
        let x: *const () = &(*x);
        sum += (*x);
    }
    let mut v2: std_vector_int_ptr__std_allocator_int_ptr__ =
        <std_vector_int_ptr__std_allocator_int_ptr__>::default();
    (unsafe {
        let mut ___x: *mut () = (&mut v1[(0_()) as usize] as *mut ());
        v2.push_back_58(&mut ___x)
    });
    (unsafe {
        let mut ___x: *mut () = (&mut v1[(1_()) as usize] as *mut ());
        v2.push_back_58(&mut ___x)
    });
    (unsafe {
        let mut ___x: *mut () = (&mut v1[(2_()) as usize] as *mut ());
        v2.push_back_58(&mut ___x)
    });
    'loop_: for p in IterRange::new((unsafe { v2.begin_26() }), (unsafe { v2.end_28() })) {
        let mut p: *mut () = (*p);
        (*p) += 5_();
    }
    'loop_: for p in IterRange::new((unsafe { v2.begin_26() }), (unsafe { v2.end_28() })) {
        let p: *mut () = (*p);
        sum += (*p);
    }
    'loop_: for p in IterRange::new((unsafe { v2.begin_26() }), (unsafe { v2.end_28() })) {
        let mut p: *mut () = (*p);
        (*p) += 5_();
    }
    'loop_: for p in IterRange::new((unsafe { v2.begin_26() }), (unsafe { v2.end_28() })) {
        let mut p: *const () = (*p).cast_const();
        sum += (*p);
    }
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
