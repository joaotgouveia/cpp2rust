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
    let mut arr: [(); 8] = [10_(), 11_(), 12_(), 13_(), 14_(), 15_(), 16_(), 17_()];
    let mut p: *mut () = arr.as_mut_ptr();
    let mut q: *mut () = p.offset((1_()) as isize);
    unimplemented!();
    let mut r: *mut () = p.offset((3_()) as isize);
    unimplemented!();
    let mut s: *mut () = r.offset(-((2_()) as isize));
    unimplemented!();
    let mut diff: () = (((r as usize - p as usize) / ::std::mem::size_of::<()>()) as () as ());
    unimplemented!();
    let mut idx: () = (((r as usize - p as usize) / ::std::mem::size_of::<()>()) as ());
    unimplemented!();
    let mut q2: *mut () = p;
    q2.prefix_inc();
    unimplemented!();
    q2.postfix_inc();
    unimplemented!();
    q2.prefix_dec();
    unimplemented!();
    q2.postfix_dec();
    unimplemented!();
    unimplemented!();
    let mut q3: *mut () = p;
    q3 = (q3).wrapping_add((4_() as ()) as usize);
    unimplemented!();
    q3 = (q3).wrapping_sub((2_() as ()) as usize);
    unimplemented!();
    let mut step: () = 2_();
    let mut q4: *mut () = p.offset((step) as isize);
    unimplemented!();
    let mut v: () = (*p.offset((3_()) as isize));
    unimplemented!();
    let mut v2: () = (*(p.offset((4_()) as isize)));
    unimplemented!();
    (*(p.offset((5_()) as isize))) = 99_();
    unimplemented!();
    unimplemented!();
    let mut end: *mut () = p.offset((8_()) as isize);
    let mut sum: () = 0_();
    let mut it: *mut () = p;
    'loop_: while ((it) != (end)) {
        sum += (*it);
        it.prefix_inc();
    }
    unimplemented!();
    let mut bytes: [(); 8] = [0_(), 1_(), 2_(), 3_(), 4_(), 5_(), 6_(), 7_()];
    let mut bp: *mut () = bytes.as_mut_ptr();
    let mut bq: *mut () = bp.offset((4_()) as isize);
    unimplemented!();
    let mut bdiff: () = (((bq as usize - bp as usize) / ::std::mem::size_of::<()>()) as () as ());
    unimplemented!();
    let mut cp: *const () = (arr.as_mut_ptr()).cast_const();
    let mut cq: *const () = cp.offset((2_()) as isize);
    unimplemented!();
    let mut cdiff: () = (((cq as usize - cp as usize) / ::std::mem::size_of::<()>()) as () as ());
    unimplemented!();
    let mut n: () = 3_();
    let mut q5: *mut () = arr.as_mut_ptr().offset((n) as isize);
    unimplemented!();
    let mut q6: *mut () = (&mut arr[(n) as usize] as *mut ());
    unimplemented!();
    let mut matrix: [[(); 4]; 3] = [
        [0_(), 1_(), 2_(), 3_()],
        [4_(), 5_(), 6_(), 7_()],
        [8_(), 9_(), 10_(), 11_()],
    ];
    let mut row1: *mut () = (&mut matrix[(1_()) as usize][(0_()) as usize] as *mut ());
    unimplemented!();
    let mut back: *mut () = end.offset(-((1_()) as isize));
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
