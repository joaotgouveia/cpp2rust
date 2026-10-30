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
pub struct Pair {
    pub x1: *mut (),
    pub x2: *mut (),
}
pub unsafe fn mkPair_0(x1: *mut (), x2: *mut ()) -> Pair {
    return Pair { x1: x1, x2: x2 };
}
pub unsafe fn fill_1(
    arr: *mut std_unique_ptr_int_ptrarrarr__std_default_delete_int_ptrarrarr__,
    n1: *mut (),
) {
    let mut n2: () = (*n1);
    let mut pair: Pair = (unsafe {
        let _x1: *mut () = n1;
        let _x2: *mut () = &mut n2;
        mkPair_0(_x1, _x2)
    });
    (&(*arr))[(0_()) as usize] = (pair.x1);
    (&(*arr))[(1_()) as usize] = (pair.x2);
}
pub unsafe fn any_2(
    arr: *mut std_unique_ptr_int_ptrarrarr__std_default_delete_int_ptrarrarr__,
    n1: *mut (),
) -> () {
    let mut out: () = false;
    let mut i: () = 0_();
    'loop_: while ((i) < (*n1)) {
        out = (out) || ((*(&(*arr))[(i) as usize]) == (0_()));
        i.prefix_inc();
    }
    return out;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
<<<<<<< HEAD
unsafe fn main_0() -> i32 {
    let mut n: i32 = 2;
    let mut arr: Option<Box<[*mut i32]>> = Some(
        (0..(n as usize))
            .map(|_| std::ptr::null_mut())
            .collect::<Box<[_]>>(),
    );
=======
unsafe fn main_0() -> () {
    let mut n: () = 2_();
    let mut arr: std_unique_ptr_int_ptrarrarr__std_default_delete_int_ptrarrarr__ =
        (unsafe { make_unique_3(n) });
>>>>>>> lucian/libcxx-libstdcxx-tests
    (unsafe { fill_1(&mut arr, &mut n) });
    return (unsafe { any_2(&mut arr, &mut n) });
}
pub unsafe fn __cpp2rust_init_globals() {}
