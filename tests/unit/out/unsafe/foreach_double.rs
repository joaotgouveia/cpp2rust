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
    let mut v: std_vector_int__std_allocator_int__ =
        <std_vector_int__std_allocator_int__>::default();
    (unsafe {
        let mut ___x: () = 1_();
        v.push_back_58(&mut ___x)
    });
    (unsafe {
        let mut ___x: () = 2_();
        v.push_back_58(&mut ___x)
    });
    (unsafe {
        let mut ___x: () = 3_();
        v.push_back_58(&mut ___x)
    });
    let mut square: () = 0_();
    'loop_: for e1 in IterRange::new((unsafe { v.begin_26() }), (unsafe { v.end_28() })) {
        let mut e1: () = (*e1);
        'loop_: for e2 in IterRange::new((unsafe { v.begin_26() }), (unsafe { v.end_28() })) {
            let mut e2: () = (*e2);
            square += ((e1) * (e2));
        }
    }
    'loop_: for e1 in IterRange::new((unsafe { v.begin_26() }), (unsafe { v.end_28() })) {
        let e1: *mut () = &mut (*e1);
        'loop_: for e2 in IterRange::new((unsafe { v.begin_26() }), (unsafe { v.end_28() })) {
            let e2: *const () = &(*e2);
            square += ((*e1) * (*e2));
        }
    }
    'loop_: for e1 in IterRange::new((unsafe { v.begin_26() }), (unsafe { v.end_28() })) {
        let e1: *const () = &(*e1);
        'loop_: for e2 in IterRange::new((unsafe { v.begin_26() }), (unsafe { v.end_28() })) {
            let e2: *mut () = &mut (*e2);
            square += ((*e1) * (*e2));
        }
    }
    'loop_: for e1 in IterRange::new((unsafe { v.begin_26() }), (unsafe { v.end_28() })) {
        let e1: *mut () = &mut (*e1);
        'loop_: for e2 in IterRange::new((unsafe { v.begin_26() }), (unsafe { v.end_28() })) {
            let e2: *mut () = &mut (*e2);
            square += ((*e1) * (*e2));
        }
    }
    let mut m : std_vector_std_vector_int__std_allocator_int____std_allocator_std_vector_int__std_allocator_int____ = <std_vector_std_vector_int__std_allocator_int____std_allocator_std_vector_int__std_allocator_int____ >::default() ;
    let mut v1: std_vector_int__std_allocator_int__ =
        <std_vector_int__std_allocator_int__>::default();
    (unsafe { m.push_back_58(&mut v1) });
    let mut v2: std_vector_int__std_allocator_int__ =
        <std_vector_int__std_allocator_int__>::default();
    (unsafe { m.push_back_58(&mut v2) });
    let mut v3: std_vector_int__std_allocator_int__ =
        <std_vector_int__std_allocator_int__>::default();
    (unsafe { m.push_back_58(&mut v3) });
    'loop_: for row in IterRange::new((unsafe { m.begin_26() }), (unsafe { m.end_28() })) {
        let row: *mut std_vector_int__std_allocator_int__ = &mut (*row);
        'loop_: for col in
            IterRange::new((unsafe { (*row).begin_26() }), (unsafe { (*row).end_28() }))
        {
            let col: *mut () = &mut (*col);
            square += (*col);
        }
    }
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
