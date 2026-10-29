extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn my_alternative_fread_0(mut p: Ptr<()>, mut n: (), mut m: (), mut f: ()) -> () {
    return 22_();
}
pub fn my_alternative_fwrite_1(mut p: Ptr<()>, mut n: (), mut m: (), mut f: ()) -> () {
    return 33_();
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let mut fn1: FnPtr<fn((), (), (), ()) -> ()> =
        FnPtr::<fn((), (), (), ()) -> ()>::new(libcc2rs::fread_refcount);
    unimplemented!();
    unimplemented!();
    let mut fn2: FnPtr<fn(Ptr<()>, (), (), ()) -> ()> = FnPtr::<fn((), (), (), ()) -> ()>::new(
        libcc2rs::fread_refcount,
    )
    .cast::<fn(Ptr<()>, (), (), ()) -> ()>();
    unimplemented!();
    let mut f3: FnPtr<fn((), (), (), ()) -> ()> =
        FnPtr::<fn(Ptr<()>, (), (), ()) -> ()>::new(my_alternative_fread_0)
            .cast::<fn((), (), (), ()) -> ()>();
    unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (0_() != 0) {
        __do_while = false;
        let mut stream: () = unimplemented!();
        unimplemented!();
        let buf: Value<Box<[()]>> = Rc::new(RefCell::new(
            (0..16).map(|_| unimplemented!()).collect::<Box<[()]>>(),
        ));
        unimplemented!();
        let mut n: () = unimplemented!();
        unimplemented!();
        let mut i: () = 0_();
        'loop_: while (i < 10_()) {
            unimplemented!();
            i.prefix_inc();
        }
        let mut i: () = 10_();
        'loop_: while (i < 16_()) {
            unimplemented!();
            i.prefix_inc();
        }
        unimplemented!();
    }
    let mut __do_while = true;
    'loop_: while __do_while || (0_() != 0) {
        __do_while = false;
        let mut stream: () = unimplemented!();
        unimplemented!();
        let buf: Value<Box<[()]>> = Rc::new(RefCell::new(
            (0..16).map(|_| unimplemented!()).collect::<Box<[()]>>(),
        ));
        unimplemented!();
        let mut n: () = ({
            (fn1).call(
                ((buf.as_pointer() as Ptr<()>) as Ptr<()>).to_any(),
                1_(),
                10_(),
                (stream).clone(),
            )
        });
        unimplemented!();
        let mut i: () = 0_();
        'loop_: while (i < 10_()) {
            unimplemented!();
            i.prefix_inc();
        }
        let mut i: () = 10_();
        'loop_: while (i < 16_()) {
            unimplemented!();
            i.prefix_inc();
        }
        unimplemented!();
    }
    let mut gn1: FnPtr<fn((), (), (), ()) -> ()> =
        FnPtr::<fn((), (), (), ()) -> ()>::new(libcc2rs::fwrite_refcount);
    unimplemented!();
    unimplemented!();
    let mut gn2: FnPtr<fn(Ptr<()>, (), (), ()) -> ()> = FnPtr::<fn((), (), (), ()) -> ()>::new(
        libcc2rs::fwrite_refcount,
    )
    .cast::<fn(Ptr<()>, (), (), ()) -> ()>();
    unimplemented!();
    let mut g3: FnPtr<fn((), (), (), ()) -> ()> = FnPtr::<fn(Ptr<()>, (), (), ()) -> ()>::new(
        my_alternative_fwrite_1,
    )
    .cast::<fn((), (), (), ()) -> ()>();
    unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (0_() != 0) {
        __do_while = false;
        let mut stream: () = unimplemented!();
        unimplemented!();
        let buf: Value<Box<[()]>> = Rc::new(RefCell::new(
            (0..10).map(|_| unimplemented!()).collect::<Box<[()]>>(),
        ));
        unimplemented!();
        let mut n: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    let mut __do_while = true;
    'loop_: while __do_while || (0_() != 0) {
        __do_while = false;
        let mut stream: () = unimplemented!();
        unimplemented!();
        let buf: Value<Box<[()]>> = Rc::new(RefCell::new(
            (0..10).map(|_| unimplemented!()).collect::<Box<[()]>>(),
        ));
        unimplemented!();
        let mut n: () = ({
            (gn1).call(
                ((buf.as_pointer() as Ptr<()>) as Ptr<()>).to_any(),
                1_(),
                10_(),
                (stream).clone(),
            )
        });
        unimplemented!();
        unimplemented!();
    }
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
