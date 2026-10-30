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
    let mut vec_: () = unimplemented!();
    let mut i: () = 27_();
    let mut str: () = unimplemented!();
    write!((*unimplemented!()), "{:} a", i,);
    (*unimplemented!()).write_all(
        &([
            (&[vec_[(0_()) as usize] as u8] as &[u8]),
            (&[vec_[(1_()) as usize] as u8] as &[u8]),
            (&[('o' as ()) as u8] as &[u8]),
        ]
        .concat()),
    );
    write!((*unimplemented!()), "{:}\n0x{:x}", str, 27_(),);
    (*unimplemented!()).write_all(
        &([
            (b" a\xc3\xa7ordas?" as &[u8]),
            (&[('\n' as ()) as u8] as &[u8]),
            (b"Sim, 0x" as &[u8]),
        ]
        .concat()),
    );
    write!((*unimplemented!()), "{:x}.\n", i,);
    write!((*unimplemented!()), "Hello, World!\n",);
    (*unimplemented!()).write_all(
        &([
            (&[vec_[(0_()) as usize] as u8] as &[u8]),
            (&[('\n' as ()) as u8] as &[u8]),
            (&[vec_[(1_()) as usize] as u8] as &[u8]),
            (&[('\n' as ()) as u8] as &[u8]),
        ]
        .concat()),
    );
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
