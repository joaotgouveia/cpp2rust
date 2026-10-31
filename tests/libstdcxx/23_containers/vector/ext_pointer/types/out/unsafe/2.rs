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
pub struct N_X {}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut v: Vec<N_X> = (0..(5_usize) as usize)
        .map(|_| <N_X>::default())
        .collect::<Vec<_>>();
    let w: Vec<N_X> = (0..(1_usize) as usize)
        .map(|_| <N_X>::default())
        .collect::<Vec<_>>();
    &((&mut v).as_mut_ptr().add(0_usize as usize));
    &((&w).as_ptr().add(0_usize as usize));
    &(v.len());
    &(v.capacity());
    {
        let __a0 = 1_usize as usize;
        v.resize_with(__a0, || <N_X>::default())
    };
    {
        let __it = <()>::from(v.begin());
        let pos = __it.iter_distance(&UnsafeRandomAccessIterator::Vec(v.as_mut_ptr())) as usize;
        v.insert(pos, <N_X>::default());
        __it
    };
    {
        let __off = <()>::from(v.begin())
            .iter_distance(&UnsafeRandomAccessIterator::Vec(v.as_mut_ptr()))
            as usize;
        v.splice(__off..__off, std::iter::repeat_n(<N_X>::default(), 1_usize));
        UnsafeRandomAccessIterator::Vec(v.as_mut_ptr().add(__off))
    };
    {
        let __off = <()>::from(v.begin())
            .iter_distance(&UnsafeRandomAccessIterator::Vec(v.as_mut_ptr()))
            as usize;
        let mut __tmp = Vec::new();
        let mut __it = w.begin();
        while __it != w.end() {
            __tmp.push(<N_X as CppFromUnsafe<N_X>>::cpp_from(__it.iter_deref()));
            __it.iter_inc();
        }
        v.splice(__off..__off, __tmp);
        UnsafeRandomAccessIterator::Vec(v.as_mut_ptr().add(__off))
    };
    v = w.clone();
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
