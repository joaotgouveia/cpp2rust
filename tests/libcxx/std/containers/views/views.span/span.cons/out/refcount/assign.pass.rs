extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn doAssign_0(lhs: (), rhs: ()) -> () {
    let lhs: Value<()> = Rc::new(RefCell::new(lhs));
    let rhs: Value<()> = Rc::new(RefCell::new(rhs));
    const _: () = assert!(true, "");;
    unimplemented!();
    return (unimplemented!() == unimplemented!()) && (unimplemented!() == unimplemented!());
}
pub fn doAssign_1(lhs: (), rhs: ()) -> () {
    let lhs: Value<()> = Rc::new(RefCell::new(lhs));
    let rhs: Value<()> = Rc::new(RefCell::new(rhs));
    const _: () = assert!(true, "");;
    unimplemented!();
    return (unimplemented!() == unimplemented!()) && (unimplemented!() == unimplemented!());
}
pub fn doAssign_2(lhs: (), rhs: ()) -> () {
    let lhs: Value<()> = Rc::new(RefCell::new(lhs));
    let rhs: Value<()> = Rc::new(RefCell::new(rhs));
    const _: () = assert!(true, "");;
    unimplemented!();
    return (unimplemented!() == unimplemented!()) && (unimplemented!() == unimplemented!());
}
pub fn doAssign_3(lhs: (), rhs: ()) -> () {
    let lhs: Value<()> = Rc::new(RefCell::new(lhs));
    let rhs: Value<()> = Rc::new(RefCell::new(rhs));
    const _: () = assert!(true, "");;
    unimplemented!();
    return (unimplemented!() == unimplemented!()) && (unimplemented!() == unimplemented!());
}
pub fn doAssign_4(lhs: (), rhs: ()) -> () {
    let lhs: Value<()> = Rc::new(RefCell::new(lhs));
    let rhs: Value<()> = Rc::new(RefCell::new(rhs));
    const _: () = assert!(true, "");;
    unimplemented!();
    return (unimplemented!() == unimplemented!()) && (unimplemented!() == unimplemented!());
}
pub fn doAssign_5(lhs: (), rhs: ()) -> () {
    let lhs: Value<()> = Rc::new(RefCell::new(lhs));
    let rhs: Value<()> = Rc::new(RefCell::new(rhs));
    const _: () = assert!(true, "");;
    unimplemented!();
    return (unimplemented!() == unimplemented!()) && (unimplemented!() == unimplemented!());
}
thread_local!(
    pub static carr1_6: Value<Box<[()]>> =
        Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_(), 4_()])));
);
thread_local!(
    pub static carr2_7: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([3_(), 4_(), 5_()])));
);
thread_local!(
    pub static carr3_8: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([7_(), 8_()])));
);
thread_local!(
    pub static arr_9: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([5_(), 6_(), 7_(), 9_()])));
);
thread_local!(
    pub static strs_10: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ])));
);

pub fn main() {
    let size: usize = ::std::env::args().map(|arg| arg.len() + 1).sum();
    let args: Value<Box<[i8]>> = Rc::new(RefCell::new(vec![0; size].into_boxed_slice()));
    let args_ptr: Ptr<i8> = args.as_pointer();
    let mut off = 0;
    let argv: Value<Box<[Ptr<i8>]>> = Rc::new(RefCell::new(
        ::std::env::args()
            .map(|arg| {
                let ptr = args_ptr.offset(off);
                ptr.write_c_bytes(arg.as_bytes());
                off += arg.len() + 1;
                ptr
            })
            .chain(::std::iter::once(Ptr::null()))
            .collect(),
    ));
    __cpp2rust_init_globals();
    ::std::process::exit(main_0(::std::env::args().len() as i32, argv.as_pointer()));
}
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    {
        let mut spans: [(); 13] = [
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
        const _: () = assert!(true, "std::size(spans) == 13");;
        const _: () = assert!(true, "doAssign(spans[0], spans[0])");;
        const _: () = assert!(true, "doAssign(spans[0], spans[1])");;
        const _: () = assert!(true, "doAssign(spans[0], spans[2])");;
        const _: () = assert!(true, "doAssign(spans[0], spans[3])");;
        const _: () = assert!(true, "doAssign(spans[0], spans[4])");;
        const _: () = assert!(true, "doAssign(spans[0], spans[5])");;
        const _: () = assert!(true, "doAssign(spans[0], spans[6])");;
        const _: () = assert!(true, "doAssign(spans[0], spans[7])");;
        const _: () = assert!(true, "doAssign(spans[0], spans[8])");;
        const _: () = assert!(true, "doAssign(spans[0], spans[9])");;
        const _: () = assert!(true, "doAssign(spans[0], spans[10])");;
        const _: () = assert!(true, "doAssign(spans[0], spans[11])");;
        const _: () = assert!(true, "doAssign(spans[0], spans[12])");;
        const _: () = assert!(true, "doAssign(spans[1], spans[1])");;
        const _: () = assert!(true, "doAssign(spans[1], spans[2])");;
        const _: () = assert!(true, "doAssign(spans[1], spans[3])");;
        const _: () = assert!(true, "doAssign(spans[1], spans[4])");;
        const _: () = assert!(true, "doAssign(spans[1], spans[5])");;
        const _: () = assert!(true, "doAssign(spans[1], spans[6])");;
        const _: () = assert!(true, "doAssign(spans[1], spans[7])");;
        const _: () = assert!(true, "doAssign(spans[1], spans[8])");;
        const _: () = assert!(true, "doAssign(spans[1], spans[9])");;
        const _: () = assert!(true, "doAssign(spans[1], spans[10])");;
        const _: () = assert!(true, "doAssign(spans[1], spans[11])");;
        const _: () = assert!(true, "doAssign(spans[1], spans[12])");;
        const _: () = assert!(true, "doAssign(spans[2], spans[2])");;
        const _: () = assert!(true, "doAssign(spans[2], spans[3])");;
        const _: () = assert!(true, "doAssign(spans[2], spans[4])");;
        const _: () = assert!(true, "doAssign(spans[2], spans[5])");;
        const _: () = assert!(true, "doAssign(spans[2], spans[6])");;
        const _: () = assert!(true, "doAssign(spans[2], spans[7])");;
        const _: () = assert!(true, "doAssign(spans[2], spans[8])");;
        const _: () = assert!(true, "doAssign(spans[2], spans[9])");;
        const _: () = assert!(true, "doAssign(spans[2], spans[10])");;
        const _: () = assert!(true, "doAssign(spans[2], spans[11])");;
        const _: () = assert!(true, "doAssign(spans[2], spans[12])");;
        const _: () = assert!(true, "doAssign(spans[3], spans[3])");;
        const _: () = assert!(true, "doAssign(spans[3], spans[4])");;
        const _: () = assert!(true, "doAssign(spans[3], spans[4])");;
        const _: () = assert!(true, "doAssign(spans[3], spans[4])");;
        const _: () = assert!(true, "doAssign(spans[3], spans[4])");;
        const _: () = assert!(true, "doAssign(spans[3], spans[4])");;
        const _: () = assert!(true, "doAssign(spans[3], spans[4])");;
        const _: () = assert!(true, "doAssign(spans[3], spans[4])");;
        const _: () = assert!(true, "doAssign(spans[3], spans[4])");;
        const _: () = assert!(true, "doAssign(spans[3], spans[10])");;
        const _: () = assert!(true, "doAssign(spans[3], spans[11])");;
        const _: () = assert!(true, "doAssign(spans[3], spans[12])");;
        const _: () = assert!(true, "doAssign(spans[4], spans[4])");;
        const _: () = assert!(true, "doAssign(spans[4], spans[5])");;
        const _: () = assert!(true, "doAssign(spans[4], spans[6])");;
        const _: () = assert!(true, "doAssign(spans[4], spans[7])");;
        const _: () = assert!(true, "doAssign(spans[4], spans[8])");;
        const _: () = assert!(true, "doAssign(spans[4], spans[9])");;
        const _: () = assert!(true, "doAssign(spans[4], spans[10])");;
        const _: () = assert!(true, "doAssign(spans[4], spans[11])");;
        const _: () = assert!(true, "doAssign(spans[4], spans[12])");;
        const _: () = assert!(true, "doAssign(spans[5], spans[5])");;
        const _: () = assert!(true, "doAssign(spans[5], spans[6])");;
        const _: () = assert!(true, "doAssign(spans[5], spans[7])");;
        const _: () = assert!(true, "doAssign(spans[5], spans[8])");;
        const _: () = assert!(true, "doAssign(spans[5], spans[9])");;
        const _: () = assert!(true, "doAssign(spans[5], spans[10])");;
        const _: () = assert!(true, "doAssign(spans[5], spans[11])");;
        const _: () = assert!(true, "doAssign(spans[5], spans[12])");;
        const _: () = assert!(true, "doAssign(spans[6], spans[6])");;
        const _: () = assert!(true, "doAssign(spans[6], spans[7])");;
        const _: () = assert!(true, "doAssign(spans[6], spans[8])");;
        const _: () = assert!(true, "doAssign(spans[6], spans[9])");;
        const _: () = assert!(true, "doAssign(spans[6], spans[10])");;
        const _: () = assert!(true, "doAssign(spans[6], spans[11])");;
        const _: () = assert!(true, "doAssign(spans[6], spans[12])");;
        const _: () = assert!(true, "doAssign(spans[7], spans[7])");;
        const _: () = assert!(true, "doAssign(spans[7], spans[8])");;
        const _: () = assert!(true, "doAssign(spans[7], spans[9])");;
        const _: () = assert!(true, "doAssign(spans[7], spans[10])");;
        const _: () = assert!(true, "doAssign(spans[7], spans[11])");;
        const _: () = assert!(true, "doAssign(spans[7], spans[12])");;
        const _: () = assert!(true, "doAssign(spans[8], spans[8])");;
        const _: () = assert!(true, "doAssign(spans[8], spans[9])");;
        const _: () = assert!(true, "doAssign(spans[8], spans[10])");;
        const _: () = assert!(true, "doAssign(spans[8], spans[11])");;
        const _: () = assert!(true, "doAssign(spans[8], spans[12])");;
        const _: () = assert!(true, "doAssign(spans[9], spans[9])");;
        const _: () = assert!(true, "doAssign(spans[9], spans[10])");;
        const _: () = assert!(true, "doAssign(spans[9], spans[11])");;
        const _: () = assert!(true, "doAssign(spans[9], spans[12])");;
        const _: () = assert!(true, "doAssign(spans[10], spans[10])");;
        const _: () = assert!(true, "doAssign(spans[10], spans[11])");;
        const _: () = assert!(true, "doAssign(spans[10], spans[12])");;
        const _: () = assert!(true, "doAssign(spans[11], spans[11])");;
        const _: () = assert!(true, "doAssign(spans[11], spans[12])");;
        const _: () = assert!(true, "doAssign(spans[12], spans[12])");;
    }
    {
        let mut spans: [(); 6] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        const _: () = assert!(true, "std::size(spans) == 6");;
        const _: () = assert!(true, "doAssign(spans[0], spans[0])");;
        const _: () = assert!(true, "doAssign(spans[0], spans[1])");;
        const _: () = assert!(true, "doAssign(spans[0], spans[2])");;
        const _: () = assert!(true, "doAssign(spans[0], spans[3])");;
        const _: () = assert!(true, "doAssign(spans[0], spans[4])");;
        const _: () = assert!(true, "doAssign(spans[0], spans[5])");;
        const _: () = assert!(true, "doAssign(spans[1], spans[1])");;
        const _: () = assert!(true, "doAssign(spans[1], spans[2])");;
        const _: () = assert!(true, "doAssign(spans[1], spans[3])");;
        const _: () = assert!(true, "doAssign(spans[1], spans[4])");;
        const _: () = assert!(true, "doAssign(spans[1], spans[5])");;
        const _: () = assert!(true, "doAssign(spans[2], spans[2])");;
        const _: () = assert!(true, "doAssign(spans[2], spans[3])");;
        const _: () = assert!(true, "doAssign(spans[2], spans[4])");;
        const _: () = assert!(true, "doAssign(spans[2], spans[5])");;
        const _: () = assert!(true, "doAssign(spans[3], spans[3])");;
        const _: () = assert!(true, "doAssign(spans[3], spans[4])");;
        const _: () = assert!(true, "doAssign(spans[3], spans[5])");;
        const _: () = assert!(true, "doAssign(spans[4], spans[4])");;
        const _: () = assert!(true, "doAssign(spans[4], spans[5])");;
        const _: () = assert!(true, "doAssign(spans[5], spans[5])");;
    }
    {
        let mut spans: [(); 5] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        let mut i: () = 0_();
        'loop_: while (i < unimplemented!()) {
            let mut j: () = i;
            'loop_: while (j < unimplemented!()) {
                unimplemented!();
                j.prefix_inc();
            }
            i.prefix_inc();
        }
    }
    {
        let mut spans: [(); 3] = [unimplemented!(), unimplemented!(), unimplemented!()];
        let mut i: () = 0_();
        'loop_: while (i < unimplemented!()) {
            let mut j: () = i;
            'loop_: while (j < unimplemented!()) {
                unimplemented!();
                j.prefix_inc();
            }
            i.prefix_inc();
        }
    }
    {
        let mut spans: [(); 10] = [
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
        let mut i: () = 0_();
        'loop_: while (i < unimplemented!()) {
            let mut j: () = i;
            'loop_: while (j < unimplemented!()) {
                unimplemented!();
                j.prefix_inc();
            }
            i.prefix_inc();
        }
    }
    {
        let mut spans: [(); 3] = [unimplemented!(), unimplemented!(), unimplemented!()];
        let mut i: () = 0_();
        'loop_: while (i < unimplemented!()) {
            let mut j: () = i;
            'loop_: while (j < unimplemented!()) {
                unimplemented!();
                j.prefix_inc();
            }
            i.prefix_inc();
        }
    }
    return 0_();
}
pub fn __cpp2rust_init_globals() {
    let _ = carr1_6.with(|_| ());
    let _ = carr2_7.with(|_| ());
    let _ = carr3_8.with(|_| ());
    let _ = arr_9.with(|_| ());
    let _ = strs_10.with(|_| ());
}
