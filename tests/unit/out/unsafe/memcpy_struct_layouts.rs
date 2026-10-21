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
pub struct packed {
    pub a: (),
    pub b: (),
    pub c: (),
    pub d: (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct reordered {
    pub a: (),
    pub b: (),
    pub c: (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct tail {
    pub a: (),
    pub b: (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct nested {
    pub t: tail,
    pub c: (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct array {
    pub name: [(); 3],
    pub x: (),
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut buf: [(); 64] = [unimplemented!(); 64];
    let mut p: [packed; 2] = [
        packed {
            a: 1_(),
            b: 2_(),
            c: 3_(),
            d: 4_(),
        },
        packed {
            a: 5_(),
            b: 6_(),
            c: 7_(),
            d: 8_(),
        },
    ];
    (unsafe {
        let ___src: () = (p.as_mut_ptr() as ());
        let ___n: () = ::std::mem::size_of::<[packed; 2]>();
        memcpy_0((buf.as_mut_ptr() as ()), ___src, ___n)
    });
    unimplemented!();
    let mut p2: [packed; 2] = [<packed>::default(); 2];
    (unsafe {
        let ___dest: () = (p2.as_mut_ptr() as ());
        let ___n: () = ::std::mem::size_of::<[packed; 2]>();
        memcpy_0(___dest, (buf.as_mut_ptr() as ()), ___n)
    });
    unimplemented!();
    let mut r: [reordered; 2] = [
        reordered {
            a: 1_(),
            b: 2_(),
            c: 3_(),
        },
        reordered {
            a: 4_(),
            b: 5_(),
            c: 6_(),
        },
    ];
    (unsafe {
        let ___src: () = (r.as_mut_ptr() as ());
        let ___n: () = ::std::mem::size_of::<[reordered; 2]>();
        memcpy_0((buf.as_mut_ptr() as ()), ___src, ___n)
    });
    unimplemented!();
    let mut r2: [reordered; 2] = [<reordered>::default(); 2];
    (unsafe {
        let ___dest: () = (r2.as_mut_ptr() as ());
        let ___n: () = ::std::mem::size_of::<[reordered; 2]>();
        memcpy_0(___dest, (buf.as_mut_ptr() as ()), ___n)
    });
    unimplemented!();
    let mut n: [nested; 2] = [
        nested {
            t: tail {
                a: 1_(),
                b: 2.5_f64,
            },
            c: 3_(),
        },
        nested {
            t: tail {
                a: 4_(),
                b: 5.5_f64,
            },
            c: 6_(),
        },
    ];
    (unsafe {
        let ___src: () = (n.as_mut_ptr() as ());
        let ___n: () = ::std::mem::size_of::<[nested; 2]>();
        memcpy_0((buf.as_mut_ptr() as ()), ___src, ___n)
    });
    unimplemented!();
    let mut n2: [nested; 2] = [<nested>::default(); 2];
    (unsafe {
        let ___dest: () = (n2.as_mut_ptr() as ());
        let ___n: () = ::std::mem::size_of::<[nested; 2]>();
        memcpy_0(___dest, (buf.as_mut_ptr() as ()), ___n)
    });
    unimplemented!();
    let mut a: [array; 2] = [
        array {
            name: std::mem::transmute(*b"ab\0"),
            x: 1_(),
        },
        array {
            name: std::mem::transmute(*b"cd\0"),
            x: 2_(),
        },
    ];
    (unsafe {
        let ___src: () = (a.as_mut_ptr() as ());
        let ___n: () = ::std::mem::size_of::<[array; 2]>();
        memcpy_0((buf.as_mut_ptr() as ()), ___src, ___n)
    });
    unimplemented!();
    let mut a2: [array; 2] = [<array>::default(); 2];
    (unsafe {
        let ___dest: () = (a2.as_mut_ptr() as ());
        let ___n: () = ::std::mem::size_of::<[array; 2]>();
        memcpy_0(___dest, (buf.as_mut_ptr() as ()), ___n)
    });
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
