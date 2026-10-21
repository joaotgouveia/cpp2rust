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
pub struct record {
    pub code: (),
    pub lo: (),
    pub hi: (),
    pub pad: [(); 8],
}
#[repr(C)]
#[derive(Copy, Clone, FnPtrArg, VaArg)]
pub union anon_0 {
    pub h: record,
    pub raw_: [(); 128],
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
pub unsafe fn fill_1(mut out: (), mut cap: ()) {
    let mut src: [(); 16] = [
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
        unimplemented!(),
    ];
    src[(unimplemented!()) as usize] = 2_();
    src[(1_()) as usize] = unimplemented!();
    src[(2_()) as usize] = unimplemented!();
    src[(3_()) as usize] = 80_();
    src[(4_()) as usize] = 127_();
    src[(5_()) as usize] = unimplemented!();
    src[(6_()) as usize] = unimplemented!();
    src[(7_()) as usize] = 1_();
    let mut n: () = if ((((::std::mem::size_of::<[(); 16]>()) < (cap)) as ()) != 0) {
        ::std::mem::size_of::<[(); 16]>()
    } else {
        cap
    };
    (unsafe { memcpy_2(out, (src.as_mut_ptr() as ()), n) });
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
        memset_3(___s, unimplemented!(), ___n)
    });
    (unsafe {
        let _out: () = ((&mut c.view as *mut anon_0) as ());
        let _cap: () = ::std::mem::size_of::<anon_0>();
        fill_1(_out, _cap)
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
