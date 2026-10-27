extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_0(mut bad: (), mut args_1: (), mut args_2: (), mut args_3: (), mut args_4: ()) {
    let mut set: () = unimplemented!();
    let mut vals: [(); 4] = [args_1, args_2, args_3, args_4];
    'loop_: for v in IterRange::new(vals.as_mut_ptr(), vals.as_mut_ptr().offset((4_()) as isize)) {
        let v: *mut () = &mut (*v);
        unimplemented!();
    }
    'loop_: for v in IterRange::new(vals.as_mut_ptr(), vals.as_mut_ptr().offset((4_()) as isize)) {
        let v: *mut () = &mut (*v);
        unimplemented!();
    }
    unimplemented!();
}
pub unsafe fn test_1(mut bad: (), mut args_1: (), mut args_2: (), mut args_3: (), mut args_4: ()) {
    let mut set: () = unimplemented!();
    let mut vals: [(); 4] = [args_1, args_2, args_3, args_4];
    'loop_: for v in IterRange::new(vals.as_mut_ptr(), vals.as_mut_ptr().offset((4_()) as isize)) {
        let v: *mut () = &mut (*v);
        unimplemented!();
    }
    'loop_: for v in IterRange::new(vals.as_mut_ptr(), vals.as_mut_ptr().offset((4_()) as isize)) {
        let v: *mut () = &mut (*v);
        unimplemented!();
    }
    unimplemented!();
}
pub unsafe fn test_2(mut bad: (), mut args_1: (), mut args_2: (), mut args_3: (), mut args_4: ()) {
    let mut set: () = unimplemented!();
    let mut vals: [(); 4] = [args_1, args_2, args_3, args_4];
    'loop_: for v in IterRange::new(vals.as_mut_ptr(), vals.as_mut_ptr().offset((4_()) as isize)) {
        let v: *mut () = &mut (*v);
        unimplemented!();
    }
    'loop_: for v in IterRange::new(vals.as_mut_ptr(), vals.as_mut_ptr().offset((4_()) as isize)) {
        let v: *mut () = &mut (*v);
        unimplemented!();
    }
    unimplemented!();
}
pub unsafe fn test_3(mut bad: (), mut args_1: (), mut args_2: (), mut args_3: (), mut args_4: ()) {
    let mut set: () = unimplemented!();
    let mut vals: [(); 4] = [args_1, args_2, args_3, args_4];
    'loop_: for v in IterRange::new(vals.as_mut_ptr(), vals.as_mut_ptr().offset((4_()) as isize)) {
        let v: *mut () = &mut (*v);
        unimplemented!();
    }
    'loop_: for v in IterRange::new(vals.as_mut_ptr(), vals.as_mut_ptr().offset((4_()) as isize)) {
        let v: *mut () = &mut (*v);
        unimplemented!();
    }
    unimplemented!();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct E {
    pub a: (),
    pub b: (),
    pub c: (),
}
impl Default for E {
    fn default() -> Self {
        unsafe {
            E {
                a: 1_(),
                b: 1_(),
                c: 1_(),
            }
        }
    }
}
pub fn main() {
    let mut args: Vec<Vec<u8>> = std::env::args()
        .map(|arg| arg.as_bytes().to_vec())
        .collect();
    args.iter_mut().for_each(|v| v.push(0));
    let mut argv: Vec<*mut libc::c_char> = args
        .iter()
        .map(|arg| arg.as_ptr() as *mut libc::c_char)
        .collect();
    argv.push(::std::ptr::null_mut());
    unsafe {
        __cpp2rust_init_globals();
        ::std::process::exit(main_0((argv.len() - 1) as i32, argv.as_mut_ptr()) as i32)
    }
}
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
    {
        (unsafe { test_0(14_(), 10_(), 11_(), 12_(), 13_()) });
        (unsafe {
            test_1(
                ('e' as ()),
                ('a' as ()),
                ('b' as ()),
                ('c' as ()),
                ('d' as ()),
            )
        });
    }
    {
        (unsafe { test_2(14_(), 10_(), 11_(), 12_(), 13_()) });
        (unsafe {
            test_3(
                ('e' as ()),
                ('a' as ()),
                ('b' as ()),
                ('c' as ()),
                ('d' as ()),
            )
        });
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
