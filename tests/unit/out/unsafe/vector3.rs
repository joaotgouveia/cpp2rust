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
    let mut v : std_vector_std_vector_int__std_allocator_int____std_allocator_std_vector_int__std_allocator_int____ = <std_vector_std_vector_int__std_allocator_int____std_allocator_std_vector_int__std_allocator_int____ >::default() ;
    (unsafe { v.resize_40(2_()) });
    (unsafe { v[(0_()) as usize].resize_40(2_()) });
    (unsafe { v[(1_()) as usize].resize_40(1_()) });
    v[(0_()) as usize][(0_()) as usize] = 1_();
    v[(0_()) as usize][(1_()) as usize] = 5_();
    v[(1_()) as usize][(0_()) as usize] = 6_();
    'loop_: for v2 in IterRange::new((unsafe { v.begin_26() }), (unsafe { v.end_28() })) {
        let v2: *mut std_vector_int__std_allocator_int__ = &mut (*v2);
        'loop_: for i in IterRange::new((unsafe { (*v2).begin_26() }), (unsafe { (*v2).end_28() }))
        {
            let i: *mut () = &mut (*i);
            (*i).prefix_inc();
        }
    }
    'loop_: for v2 in IterRange::new((unsafe { v.begin_26() }), (unsafe { v.end_28() })) {
        let v2: *mut std_vector_int__std_allocator_int__ = &mut (*v2);
        'loop_: for i in IterRange::new((unsafe { (*v2).begin_26() }), (unsafe { (*v2).end_28() }))
        {
            let mut i: () = (*i);
            unimplemented!();
        }
    }
    'loop_: for v2 in IterRange::new((unsafe { v.begin_26() }), (unsafe { v.end_28() })) {
        let mut v2: std_vector_int__std_allocator_int__ = (*v2).clone();
        'loop_: for i in IterRange::new((unsafe { v2.begin_26() }), (unsafe { v2.end_28() })) {
            let mut i: () = (*i);
            unimplemented!();
        }
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
