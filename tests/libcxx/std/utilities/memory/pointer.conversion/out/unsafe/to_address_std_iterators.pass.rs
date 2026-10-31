extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_container_iterators_0(mut c: Vec<i32>) {
    let cc: *const Vec<i32> = &c;
    assert!(((unimplemented!()) == (c.as_mut_ptr())));
    assert!(((unimplemented!()) == (c.as_mut_ptr().offset((c.len()) as isize))));
    assert!(((unimplemented!()) == ((*cc).as_ptr())));
    assert!(((unimplemented!()) == ((*cc).as_ptr().offset(((*cc).len()) as isize))));
}
pub unsafe fn test_container_iterators_1(mut c: Vec<i32>) {
    let cc: *const Vec<i32> = &c;
    assert!(((c.begin().iter_deref()) == (c.as_mut_ptr())));
    assert!(((c.end().iter_deref()) == (c.as_mut_ptr().offset((c.len()) as isize))));
    assert!((((*cc).begin().iter_deref()) == ((*cc).as_ptr())));
    assert!((((*cc).end().iter_deref()) == ((*cc).as_ptr().offset(((*cc).len()) as isize))));
}
pub unsafe fn test_container_iterators_2(mut c: Vec<libc::c_char>) {
    let cc: *const Vec<libc::c_char> = &c;
    assert!(((UnsafeRandomAccessIterator::Vec(c.as_mut_ptr()).iter_deref()) == (c.as_mut_ptr())));
    assert!(
        ((UnsafeRandomAccessIterator::Vec(c.as_mut_ptr().add(c.len() - 1)).iter_deref())
            == (c.as_mut_ptr().offset((c.len() - 1) as isize)))
    );
    assert!(((unimplemented!().iter_deref()) == ((*cc).as_ptr())));
    assert!(
        ((unimplemented!().iter_deref()) == ((*cc).as_ptr().offset(((*cc).len() - 1) as isize)))
    );
}
pub unsafe fn test_container_iterators_3(mut c: ()) {
    let cc: *const () = &c;
    assert!(((unimplemented!()) == (unimplemented!())));
    assert!(((unimplemented!()) == (unimplemented!().offset((unimplemented!()) as isize))));
    assert!(((unimplemented!()) == (unimplemented!())));
    assert!(((unimplemented!()) == (unimplemented!().offset((unimplemented!()) as isize))));
}
pub unsafe fn test_container_iterators_4(mut c: ()) {
    let cc: *const () = &c;
    assert!(((unimplemented!().iter_deref()) == (unimplemented!())));
    assert!(
        ((unimplemented!().iter_deref()) == (unimplemented!().offset((unimplemented!()) as isize)))
    );
    assert!(((unimplemented!().iter_deref()) == (unimplemented!())));
    assert!(
        ((unimplemented!().iter_deref()) == (unimplemented!().offset((unimplemented!()) as isize)))
    );
}
pub unsafe fn test_valarray_iterators_5() {
    let mut v: () = unimplemented!();
    let mut p: *mut i32 = unimplemented!();
    let mut q: *mut i32 = unimplemented!();
    assert!(((((q as usize - p as usize) / ::std::mem::size_of::<i32>()) as i64) == (100_i64)));
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
    (unsafe { test_container_iterators_0(std::array::from_fn::<_, 3, _>(|_| 0_i32).to_vec()) });
    (unsafe {
        test_container_iterators_1((0..(3_usize) as usize).map(|_| 0_i32).collect::<Vec<_>>())
    });
    (unsafe {
        test_container_iterators_2(
            std::slice::from_raw_parts(c"abc".as_ptr(), libcc2rs::c_str_len(c"abc".as_ptr()) + 1)
                .to_vec(),
        )
    });
    (unsafe { test_container_iterators_3(unimplemented!()) });
    (unsafe { test_container_iterators_4(unimplemented!()) });
    (unsafe { test_valarray_iterators_5() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
