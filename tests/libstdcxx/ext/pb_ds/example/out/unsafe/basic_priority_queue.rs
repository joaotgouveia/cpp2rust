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
    {
        unimplemented!();
        let mut c : __gnu_pbds_priority_queue_int__std_less_int_____gnu_pbds_pairing_heap_tag__std_allocator_char__ = <__gnu_pbds_priority_queue_int__std_less_int_____gnu_pbds_pairing_heap_tag__std_allocator_char__ >::default() ;
        (unsafe { some_op_sequence_0(&mut c) });
    }
    {
        unimplemented!();
        let mut c : __gnu_pbds_priority_queue_int__std_less_int_____gnu_pbds_binomial_heap_tag__std_allocator_char__ = <__gnu_pbds_priority_queue_int__std_less_int_____gnu_pbds_binomial_heap_tag__std_allocator_char__ >::default() ;
        (unsafe { some_op_sequence_1(&mut c) });
    }
    {
        unimplemented!();
        let mut c : __gnu_pbds_priority_queue_int__std_less_int_____gnu_pbds_rc_binomial_heap_tag__std_allocator_char__ = <__gnu_pbds_priority_queue_int__std_less_int_____gnu_pbds_rc_binomial_heap_tag__std_allocator_char__ >::default() ;
        (unsafe { some_op_sequence_2(&mut c) });
    }
    {
        unimplemented!();
        let mut c : __gnu_pbds_priority_queue_int__std_less_int_____gnu_pbds_binary_heap_tag__std_allocator_char__ = <__gnu_pbds_priority_queue_int__std_less_int_____gnu_pbds_binary_heap_tag__std_allocator_char__ >::default() ;
        (unsafe { some_op_sequence_3(&mut c) });
    }
    {
        unimplemented!();
        let mut c : __gnu_pbds_priority_queue_int__std_less_int_____gnu_pbds_thin_heap_tag__std_allocator_char__ = <__gnu_pbds_priority_queue_int__std_less_int_____gnu_pbds_thin_heap_tag__std_allocator_char__ >::default() ;
        (unsafe { some_op_sequence_4(&mut c) });
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
