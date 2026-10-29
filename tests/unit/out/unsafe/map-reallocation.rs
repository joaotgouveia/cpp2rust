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
    let N: () = 10000_();
    let sentinel: () = ((N) / (2_()));
    let mut m: std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int___ =
        <std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int___>::default();
    m[(sentinel) as usize] = sentinel;
    let mut it: std__Rb_tree_iterator_std_pair_const_int__int__ = (unsafe { m.find_71(&sentinel) });
    let mut p: *mut () = (&mut (*it).second as *mut ());
    unimplemented!();
    unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((i) < (sentinel)) {
        m[(i) as usize] = i;
        i.prefix_inc();
    }
    let mut i: () = ((sentinel) + (1_()));
    'loop_: while ((i) <= (N)) {
        m[(i) as usize] = i;
        i.prefix_inc();
    }
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (*it).second = 57005_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut prev: () = -1_();
    'loop_: for pair in IterRange::new((unsafe { m.begin_18() }), (unsafe { m.end_20() })) {
        let pair: *mut std_pair_const_int__int_ = &mut (*pair);
        unimplemented!();
        prev = (*pair).first;
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
