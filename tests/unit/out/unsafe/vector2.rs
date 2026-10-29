extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn fn_0(
    v: *mut std_vector_int__std_allocator_int__,
    mut v3: std_vector_int__std_allocator_int__,
) {
    (unsafe {
        let mut ___x: () = 20_();
        (*v).push_back_58(&mut ___x)
    });
    let mut x: () = unimplemented!();
    let mut v2: std_vector_int__std_allocator_int__ =
        <std_vector_int__std_allocator_int__>::default();
    let mut v4: *mut std_vector_int__std_allocator_int__ =
        (&mut v3 as *mut std_vector_int__std_allocator_int__);
    (unsafe {
        let mut ___x: () = 0_();
        v2.push_back_58(&mut ___x)
    });
    (unsafe {
        let mut ___x: () = 1_();
        v2.push_back_58(&mut ___x)
    });
    (unsafe {
        let mut ___x: () = 3_();
        v2.push_back_58(&mut ___x)
    });
    x = (&mut (*v))[(2_()) as usize];
    v2[(0_()) as usize] = 1_();
    (&mut (*if true {
        (&mut v3 as *mut std_vector_int__std_allocator_int__)
    } else {
        (v)
    }))[(0_()) as usize] = 7_();
    v2 = (*v).clone();
    (&mut (*v4))[(1_()) as usize] = 13_();
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
    (unsafe {
        let mut ___x: () = 20_();
        (*v).push_back_58(&mut ___x)
    });
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut v: std_vector_int__std_allocator_int__ =
        <std_vector_int__std_allocator_int__>::default();
    let mut v2: std_vector_int__std_allocator_int__ =
        <std_vector_int__std_allocator_int__>::default();
    (unsafe {
        let mut ___x: () = 4_();
        v.push_back_58(&mut ___x)
    });
    (unsafe {
        let mut ___x: () = 5_();
        v.push_back_58(&mut ___x)
    });
    (unsafe {
        let mut ___x: () = 6_();
        v.push_back_58(&mut ___x)
    });
    (unsafe {
        let mut ___x: () = 8_();
        v2.push_back_58(&mut ___x)
    });
    (unsafe {
        let mut ___x: () = 9_();
        v2.push_back_58(&mut ___x)
    });
    (unsafe { fn_0(&mut v, v2.clone()) });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
