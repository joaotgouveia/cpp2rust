extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn del_at_start_0(mut c: Vec<i32>) {
    let mut first: () = unimplemented!();
    let mut it1: () = first.iter_add(1_isize);
    let mut it2: () = unimplemented!().iter_add(-1_isize);
    unimplemented!();
    let mut it3: () = unimplemented!();
    let mut it4: () = unimplemented!().iter_add(-1_isize);
    assert!(it1 == it3);
    assert!(((*it1.iter_deref()) == (*it3.iter_deref())));
    assert!(((&mut (*it1.iter_deref()) as *mut i32) == (&mut (*it3.iter_deref()) as *mut i32)));
    assert!(it2 == it4);
    assert!(((*it2.iter_deref()) == (*it4.iter_deref())));
    assert!(((&mut (*it2.iter_deref()) as *mut i32) == (&mut (*it4.iter_deref()) as *mut i32)));
}
pub unsafe fn del_at_end_1(mut c: Vec<i32>) {
    let mut first: () = unimplemented!().iter_add(-1_isize);
    let mut it1: () = unimplemented!();
    let mut it2: () = first.iter_add(-1_isize);
    unimplemented!();
    let mut it3: () = unimplemented!();
    let mut it4: () = unimplemented!().iter_add(-1_isize);
    assert!(it1 == it3);
    assert!(((*it1.iter_deref()) == (*it3.iter_deref())));
    assert!(((&mut (*it1.iter_deref()) as *mut i32) == (&mut (*it3.iter_deref()) as *mut i32)));
    assert!(it2 == it4);
    assert!(((*it2.iter_deref()) == (*it4.iter_deref())));
    assert!(((&mut (*it2.iter_deref()) as *mut i32) == (&mut (*it4.iter_deref()) as *mut i32)));
    const _: () = assert!(true, "");;
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
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    let mut queue: Vec<i32> = Vec::new();
    let mut i: i32 = 0;
    'loop_: while ((i) < (20)) {
        queue.push(i);
        i.prefix_inc();
    }
    'loop_: while ((unimplemented!()) > (1_usize)) {
        (unsafe { del_at_start_0(queue.clone()) });
        (unsafe { del_at_end_1(queue.clone()) });
        unimplemented!();
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
