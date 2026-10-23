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
    let mut even_p : __gnu_pbds_priority_queue_int__std_less_int_____gnu_pbds_pairing_heap_tag__std_allocator_char__ = <__gnu_pbds_priority_queue_int__std_less_int_____gnu_pbds_pairing_heap_tag__std_allocator_char__ >::default() ;
    let mut odd_p : __gnu_pbds_priority_queue_int__std_less_int_____gnu_pbds_pairing_heap_tag__std_allocator_char__ = <__gnu_pbds_priority_queue_int__std_less_int_____gnu_pbds_pairing_heap_tag__std_allocator_char__ >::default() ;
    let mut i: () = 0_();
    'loop_: while ((i) < (10_())) {
        (unsafe {
            let mut _r_val: () = (2_()).wrapping_mul(i);
            (even_p as __gnu_pbds_detail_pairing_heap_int__std_less_int___std_allocator_char__)
                .push(&mut _r_val)
        });
        (unsafe {
            let mut _r_val: () = ((2_()).wrapping_mul(i)).wrapping_add(1_());
            (odd_p as __gnu_pbds_detail_pairing_heap_int__std_less_int___std_allocator_char__)
                .push(&mut _r_val)
        });
        i.prefix_inc();
    }
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut it: () = unimplemented!();
    unimplemented!();
    'loop_: while unimplemented!() {
        unimplemented!();
        unimplemented!();
    }
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    'loop_: while unimplemented!() {
        unimplemented!();
        unimplemented!();
    }
    (unsafe {
        (even_p as __gnu_pbds_detail_pairing_heap_int__std_less_int___std_allocator_char__)
            .join(&mut odd_p)
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    'loop_: while unimplemented!() {
        unimplemented!();
        unimplemented!();
    }
    unimplemented!();
    unimplemented!();
    unimplemented!();
    'loop_: while unimplemented!() {
        unimplemented!();
        unimplemented!();
    }
    (unsafe {
        (even_p as __gnu_pbds_detail_pairing_heap_int__std_less_int___std_allocator_char__)
            .split(unimplemented!(), &mut odd_p)
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    'loop_: while unimplemented!() {
        unimplemented!();
        unimplemented!();
    }
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    'loop_: while unimplemented!() {
        unimplemented!();
        unimplemented!();
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
