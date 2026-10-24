extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn cmpxchg_weak_loop_0(atomic: Ptr<()>, expected: Ptr<()>, mut desired: ()) -> () {
    let mut i: () = 0_();
    'loop_: while (i < 10_()) {
        if (unimplemented!() == true) {
            return true;
        }
        i.postfix_inc();
    }
    return false;
}
pub fn cmpxchg_weak_loop_1(atomic: Ptr<()>, expected: Ptr<()>, mut desired: ()) -> () {
    let mut i: () = 0_();
    'loop_: while (i < 10_()) {
        if (unimplemented!() == true) {
            return true;
        }
        i.postfix_inc();
    }
    return false;
}
pub fn cmpxchg_weak_loop_2(
    atomic: Ptr<()>,
    expected: Ptr<()>,
    mut desired: (),
    mut success: (),
    mut failure: (),
) -> () {
    let mut i: () = 0_();
    'loop_: while (i < 10_()) {
        if (unimplemented!() == true) {
            return true;
        }
        i.postfix_inc();
    }
    return false;
}
pub fn cmpxchg_weak_loop_3(
    atomic: Ptr<()>,
    expected: Ptr<()>,
    mut desired: (),
    mut success: (),
    mut failure: (),
) -> () {
    let mut i: () = 0_();
    'loop_: while (i < 10_()) {
        if (unimplemented!() == true) {
            return true;
        }
        i.postfix_inc();
    }
    return false;
}
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
        let obj: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        {
            let mut lockfree: () = unimplemented!();
            &(lockfree);
            {
                unimplemented!();
            }
        }
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
        let x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        (*x.borrow_mut()) = true;
        unimplemented!();
        unimplemented!();
        unimplemented!();
        (*x.borrow_mut()) = true;
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        (*x.borrow_mut()) = true;
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let obj: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        {
            let mut lockfree: () = unimplemented!();
            &(lockfree);
            {
                unimplemented!();
            }
        }
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
        let x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        (*x.borrow_mut()) = true;
        unimplemented!();
        unimplemented!();
        unimplemented!();
        (*x.borrow_mut()) = true;
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        (*x.borrow_mut()) = true;
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let obj: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        {
            let mut lockfree: () = unimplemented!();
            &(lockfree);
            {
                unimplemented!();
            }
        }
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
        let x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        (*x.borrow_mut()) = true;
        unimplemented!();
        unimplemented!();
        unimplemented!();
        (*x.borrow_mut()) = true;
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        (*x.borrow_mut()) = true;
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let storage: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_()])));
        let zero: Ptr<()> = Ptr::alloc(Rc::new(RefCell::new(unimplemented!())).as_pointer());
        unimplemented!();
    }
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
