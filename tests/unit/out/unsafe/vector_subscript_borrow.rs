extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut next_0: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
pub unsafe fn bump_1(mut v: *mut Vec<i32>, mut x: i32) -> i32 {
    (*(&mut (*v)).as_mut_ptr().add(0_usize as usize)) += x;
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut next_0)).postfix_inc();
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut v: Vec<i32> = vec![1; 4_usize as usize];
    let mut items: Vec<u32> = (0..(3_usize) as usize).map(|_| 0_u32).collect::<Vec<_>>();
    let mut i: u32 = 0_u32;
    'loop_: while ((i as usize) < (items.len())) {
        (*(&mut items).as_mut_ptr().add((i as usize) as usize)) = (i).wrapping_mul(2_u32);
        i.prefix_inc();
    }
    (*(&mut items).as_mut_ptr().add(1_usize as usize)) =
        (*(&mut items).as_mut_ptr().add(1_usize as usize)).wrapping_add(5_u32);
    (*(&mut items).as_mut_ptr().add(2_usize as usize)).postfix_inc();
    assert!(
        (((*(&mut items).as_mut_ptr().add(0_usize as usize)) == (0_u32))
            && ((*(&mut items).as_mut_ptr().add(1_usize as usize)) == (7_u32)))
            && ((*(&mut items).as_mut_ptr().add(2_usize as usize)) == (5_u32))
    );
    (*(&mut v)
        .as_mut_ptr()
        .add(((*(&mut v).as_mut_ptr().add(0_usize as usize)) as usize) as usize)) = 3;
    (*(&mut v)
        .as_mut_ptr()
        .add(((*(&mut v).as_mut_ptr().add(1_usize as usize)) as usize) as usize)) =
        ((*(&mut v).as_mut_ptr().add(1_usize as usize)) + (4));
    assert!(
        ((*(&mut v).as_mut_ptr().add(1_usize as usize)) == (3))
            && ((*(&mut v).as_mut_ptr().add(3_usize as usize)) == (7))
    );
    let mut p: *mut i32 = (&mut (*(&mut v).as_mut_ptr().add(0_usize as usize)) as *mut i32);
    (*(&mut v).as_mut_ptr().add(((*p) as usize) as usize)) = 9;
    assert!(((*(&mut v).as_mut_ptr().add(1_usize as usize)) == (9)));
    (*(&mut v)
        .as_mut_ptr()
        .add(((unsafe { bump_1((&mut v as *mut Vec<i32>), 10) }) as usize) as usize)) = 2;
    assert!(((*(&mut v).as_mut_ptr().add(0_usize as usize)) == (2)));
    assert!(
        ((unsafe {
            let _v: *mut Vec<i32> = (&mut v as *mut Vec<i32>);
            let _x: i32 = (*(&mut v).as_mut_ptr().add(2_usize as usize));
            bump_1(_v, _x)
        }) == (1))
            && ((*(&mut v).as_mut_ptr().add(0_usize as usize)) == (3))
    );
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const next_0);
}
