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
pub struct shape_a {
    pub code: u16,
    pub pad: [(); 14],
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct shape_b {
    pub code: u16,
    pub lo: u16,
    pub hi: u32,
    pub fill: [(); 8],
}
#[repr(C)]
#[derive(Copy, Clone, FnPtrArg, VaArg)]
pub union anon_0 {
    pub a: shape_a,
    pub b: shape_b,
    pub raw_: [(); 256],
}
impl Default for anon_0 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Container {
    pub view: anon_0,
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut c: Container = <Container>::default();
    (unsafe {
        let ___s: () = ((&mut c as *mut Container) as ());
        let ___n: () = ::std::mem::size_of::<Container>();
        memset_1(___s, 0_(), ___n)
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut src: [(); 16] = [
        0_(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ];
    src[(0_()) as usize] = 2_();
    src[(2_()) as usize] = 80_();
    src[(3_()) as usize] = 0_();
    src[(4_()) as usize] = 127_();
    src[(5_()) as usize] = 0_();
    src[(6_()) as usize] = 0_();
    src[(7_()) as usize] = 1_();
    let mut len: () = 16_();
    unimplemented!();
    (unsafe {
        memcpy_2(
            ((&mut c.view.raw_ as *mut [(); 256]) as ()),
            (src.as_mut_ptr() as ()),
            len,
        )
    });
    unimplemented!();
    unimplemented!();
    (unsafe {
        let ___s: () = ((&mut c as *mut Container) as ());
        let ___n: () = ::std::mem::size_of::<Container>();
        memset_1(___s, 0_(), ___n)
    });
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
