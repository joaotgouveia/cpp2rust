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
    let mut arr: [(); 3] = [1_(), 2_(), 3_()];
    {
        let mut p: *mut () = (&mut arr[(0_()) as usize] as *mut ());
        p = (p).wrapping_add((1_u8 as ()) as usize);
        unimplemented!();
    }
    {
        let mut p: *mut () = (&mut arr[(0_()) as usize] as *mut ());
        p = (p).wrapping_add((1_i8 as ()) as usize);
        unimplemented!();
    }
    {
        let mut p: *mut () = (&mut arr[(0_()) as usize] as *mut ());
        p = (p).wrapping_add((1_u16 as ()) as usize);
        unimplemented!();
    }
    {
        let mut p: *mut () = (&mut arr[(0_()) as usize] as *mut ());
        p = (p).wrapping_add((1_i16 as ()) as usize);
        unimplemented!();
    }
    {
        let mut p: *mut () = (&mut arr[(0_()) as usize] as *mut ());
        p = (p).wrapping_add((1_u32 as u32) as usize);
        unimplemented!();
    }
    {
        let mut p: *mut () = (&mut arr[(0_()) as usize] as *mut ());
        p = (p).wrapping_add(((1_() as i32) as i32) as usize);
        unimplemented!();
    }
    {
        let mut p: *mut () = (&mut arr[(0_()) as usize] as *mut ());
        p = (p).wrapping_add((1_u64 as u64) as usize);
        unimplemented!();
    }
    {
        let mut p: *mut () = (&mut arr[(0_()) as usize] as *mut ());
        p = (p).wrapping_add((1_i64 as i64) as usize);
        unimplemented!();
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
