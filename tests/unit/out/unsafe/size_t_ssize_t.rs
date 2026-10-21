extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn add_sizes_0(mut a: (), mut b: ()) -> () {
    return (a).wrapping_add(b);
}
pub unsafe fn take_ulong_1(mut x: ()) -> () {
    return x;
}
pub unsafe fn sub_signed_2(mut a: (), mut b: ()) -> () {
    return ((a) - (b));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut n: () = (::std::mem::size_of::<()>() as ()).wrapping_add(4_());
    unimplemented!();
    let mut ul: () = 10_();
    let mut sz: () = 20_();
    let mut mixed: () = (sz).wrapping_add(ul);
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut chain: () = (((sz).wrapping_add(ul)).wrapping_add(5_()))
        .wrapping_add((::std::mem::size_of::<()>() as ()));
    unimplemented!();
    let mut acc: () = 100_();
    acc = (acc).wrapping_add((::std::mem::size_of::<()>() as ()));
    acc = (acc).wrapping_mul(2_());
    acc = (acc).wrapping_sub(ul);
    unimplemented!();
    sz = (sz).wrapping_add(1_());
    unimplemented!();
    let mut fr: () =
        (unsafe { add_sizes_0((::std::mem::size_of::<()>() as ()).wrapping_add(sz), ul) });
    unimplemented!();
    let mut fr2: () = (unsafe { take_ulong_1(sz) });
    unimplemented!();
    let mut lo: () = (*unimplemented!());
    let mut hi: () = (*unimplemented!());
    unimplemented!();
    unimplemented!();
    let mut bound: () = (*unimplemented!());
    unimplemented!();
    let mut data: [(); 8] = [unimplemented!(); 8];
    let mut count: () =
        (::std::mem::size_of::<[(); 8]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    let mut i: () = unimplemented!();
    'loop_: while ((i) < (count)) {
        data[(i) as usize] = ((i).wrapping_mul(2_()));
        i.postfix_inc();
    }
    let mut total: () = unimplemented!();
    let mut i: () = unimplemented!();
    'loop_: while ((i) < (count)) {
        total = (total).wrapping_add(data[(i) as usize]);
        i.postfix_inc();
    }
    unimplemented!();
    let mut cond: () = if ((sz) > (ul)) {
        (sz).wrapping_add((::std::mem::size_of::<()>() as ()))
    } else {
        ul
    };
    unimplemented!();
    let mut arr: [(); 4] = [unimplemented!(), 1_(), 2_(), 3_()];
    let mut idx: () = if ((::std::mem::size_of::<()>()) > (2_())) {
        2_()
    } else {
        unimplemented!()
    };
    unimplemented!();
    let mut s1: () = 5_();
    let mut s2: () = 12_();
    let mut sd: () = (unsafe { sub_signed_2(s1, s2) });
    unimplemented!();
    unimplemented!();
    let mut l: () = 3_();
    let mut sm: () = ((s2) + (l));
    unimplemented!();
    unimplemented!();
    let mut smin: () = (*unimplemented!());
    let mut smax: () = (*unimplemented!());
    unimplemented!();
    unimplemented!();
    let mut delta: () = ((sz) - (ul));
    unimplemented!();
    let mut a64: () = 100_();
    let mut b: () = 30_();
    a64 -= b;
    unimplemented!();
    a64 += b;
    unimplemented!();
    let mut c: () = -20_();
    a64 -= c;
    unimplemented!();
    unimplemented!();
    let mx: () = 5_();
    let mut mins: [(); 4] = [
        unimplemented!(),
        (*unimplemented!()),
        (*unimplemented!()),
        mx,
    ];
    unimplemented!();
    unimplemented!();
    let mut pr: () = unimplemented!();
    unimplemented!();
    let sizes: [(); 4] = [1_(), 2_(), 3_(), 4_()];
    let mut sizes_end: *const () = unimplemented!();
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
