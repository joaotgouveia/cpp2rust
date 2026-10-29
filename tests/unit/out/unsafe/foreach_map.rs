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
    let mut m: std_map_int__double__std_less_int___std_allocator_std_pair_const_int__double___ =
        <std_map_int__double__std_less_int___std_allocator_std_pair_const_int__double___>::default(
        );
    let mut i: () = 0_();
    let mut k: () = 100_();
    'loop_: while ((i) < (100_())) {
        m[(i) as usize] = ((k) / (2_f64));
        {
            i.prefix_inc();
            k.prefix_dec()
        };
    }
    let mut sum: () = 0_();
    'loop_: for i in IterRange::new((unsafe { m.begin_18() }), (unsafe { m.end_20() })) {
        let i: *const std_pair_const_int__double_ = &(*i);
        sum += (*i).second;
    }
    'loop_: for i in IterRange::new((unsafe { m.begin_18() }), (unsafe { m.end_20() })) {
        let mut i: std_pair_const_int__double_ = (*i);
        sum += i.first;
    }
    unimplemented!();
    'loop_: for i in IterRange::new((unsafe { m.begin_18() }), (unsafe { m.end_20() })) {
        let mut i: std_pair_const_int__double_ = (*i);
        i.second = -1_();
        unimplemented!();
    }
    'loop_: for i in IterRange::new((unsafe { m.begin_18() }), (unsafe { m.end_20() })) {
        let i: *const std_pair_const_int__double_ = &(*i);
        unimplemented!();
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
