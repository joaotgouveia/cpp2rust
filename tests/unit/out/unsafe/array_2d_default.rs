extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn fill_row_0(mut row: *mut (), mut c: ()) {
    (*row.offset((0_()) as isize)) = c;
    (*row.offset((1_()) as isize)) = ('\0' as ());
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut grid: [[(); 6]; 3] = [[unimplemented!(); 6]; 3];
    let mut i: () = 0_();
    'loop_: while ((((i) < (3_())) as ()) != 0) {
        (unsafe {
            let _row: *mut () = grid[(i) as usize].as_mut_ptr();
            let _c: () = ((('a' as ()) + (i)) as ());
            fill_row_0(_row, _c)
        });
        i.postfix_inc();
    }
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    grid[(2_()) as usize][(5_()) as usize] = ('z' as ());
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
