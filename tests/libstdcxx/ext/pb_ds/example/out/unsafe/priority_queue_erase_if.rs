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
    let mut p : __gnu_pbds_priority_queue_int__std_less_int_____gnu_pbds_pairing_heap_tag__std_allocator_char__ = <__gnu_pbds_priority_queue_int__std_less_int_____gnu_pbds_pairing_heap_tag__std_allocator_char__ >::default() ;
    let mut i: () = 0_();
    'loop_: while ((i) < (1000_())) {
        (unsafe {
            (p as __gnu_pbds_detail_pairing_heap_int__std_less_int___std_allocator_char__).push(&i)
        });
        i.prefix_inc();
    }
    unimplemented!();
    (unsafe {
        (p as __gnu_pbds_detail_pairing_heap_int__std_less_int___std_allocator_char__)
            .erase_if(unimplemented!())
    });
    unimplemented!();
    0
}
pub unsafe fn __cpp2rust_init_globals() {}
