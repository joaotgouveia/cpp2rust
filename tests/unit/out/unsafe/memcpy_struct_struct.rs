extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Entry {
    pub bits: u8,
    pub value: u16,
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut table: [Entry; 8] = [
        Entry {
            bits: 1_u8,
            value: 4369_u16,
        },
        Entry {
            bits: 2_u8,
            value: 8738_u16,
        },
        Entry {
            bits: 3_u8,
            value: 13107_u16,
        },
        Entry {
            bits: 4_u8,
            value: 17476_u16,
        },
        Entry {
            bits: 0_u8,
            value: 0_u16,
        },
        Entry {
            bits: 0_u8,
            value: 0_u16,
        },
        Entry {
            bits: 0_u8,
            value: 0_u16,
        },
        Entry {
            bits: 0_u8,
            value: 0_u16,
        },
    ];
    let mut table_size: () = 4_();
    (unsafe {
        let ___dest: () = ((&mut table[(table_size) as usize] as *mut Entry) as ());
        let ___src: () = ((&mut table[(0_()) as usize] as *mut Entry) as ());
        let ___n: () = (table_size).wrapping_mul((::std::mem::size_of::<Entry>() as ()));
        memcpy_0(___dest, ___src, ___n)
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
