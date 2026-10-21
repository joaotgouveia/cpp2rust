extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn get_0(s: Ptr<()>) -> Ptr<()> {
    let s: Value<Ptr<()>> = Rc::new(RefCell::new(s));
    return (*s.borrow()).clone();
}
pub fn get_1(s: Ptr<()>) -> Ptr<()> {
    let s: Value<Ptr<()>> = Rc::new(RefCell::new(s));
    return (*s.borrow()).clone();
}
pub fn get_2(s: Ptr<()>) -> Ptr<()> {
    let s: Value<Ptr<()>> = Rc::new(RefCell::new(s));
    return (*s.borrow()).clone();
}
pub fn get_3(s: Ptr<()>) -> Ptr<()> {
    let s: Value<Ptr<()>> = Rc::new(RefCell::new(s));
    return (*s.borrow()).clone();
}
pub fn get_4(s: Ptr<()>) -> Ptr<()> {
    let s: Value<Ptr<()>> = Rc::new(RefCell::new(s));
    return (*s.borrow()).clone();
}
pub fn second_5(s: Ptr<()>) -> () {
    return (elem!((s), 1_()).read());
}
pub fn second_6(s: Ptr<()>) -> () {
    return (elem!((s), 1_()).read());
}
pub fn second_7(s: Ptr<()>) -> () {
    return (elem!((s), 1_()).read());
}
pub fn second_8(s: Ptr<()>) -> () {
    return (elem!((s), 1_()).read());
}
pub fn second_9(s: Ptr<()>) -> () {
    return (elem!((s), 1_()).read());
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let mut c: Ptr<()> = Ptr::<()>::from_string_literal(b"A");
    let mut w: Ptr<()> = Ptr::<()>::from_string_literal(&[65 as (), 258 as (), 0 as ()]);
    let mut b: Ptr<()> = Ptr::<()>::from_string_literal(&[196 as (), 130 as (), 0 as ()]);
    let mut s: Ptr<()> = Ptr::<()>::from_string_literal(&[65 as (), 258 as (), 0 as ()]);
    let mut l: Ptr<()> = Ptr::<()>::from_string_literal(&[65 as (), 258 as (), 0 as ()]);
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut nw: () = ((::std::mem::size_of::<[(); 4]>() as ())
        .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
        .wrapping_sub(1_());
    let mut nb: () = ((::std::mem::size_of::<[(); 4]>() as ())
        .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
        .wrapping_sub(1_());
    let mut ns: () = ((::std::mem::size_of::<[(); 4]>() as ())
        .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
        .wrapping_sub(1_());
    let mut nl: () = ((::std::mem::size_of::<[(); 4]>() as ())
        .wrapping_div((::std::mem::size_of::<()>() as ())) as ())
        .wrapping_sub(1_());
    unimplemented!();
    let mut pw: [(); 4] = [258 as (), 0 as (), 0 as (), 0 as ()];
    let mut ps: [(); 4] = [258 as (), 0 as (), 0 as (), 0 as ()];
    unimplemented!();
    unimplemented!();
    let mut ew: [(); 2] = [0 as (), 0 as ()];
    unimplemented!();
    let mut wc: () = (258 as ());
    let mut bc: () = (65 as ());
    let mut sc: () = (258 as ());
    let mut lc: () = (258 as ());
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut wa: [(); 3] = [(65 as ()), (258 as ()), (0 as ())];
    unimplemented!();
    wa[(unimplemented!()) as usize] = (66 as ());
    unimplemented!();
    return unimplemented!();
}
pub fn __cpp2rust_init_globals() {}
