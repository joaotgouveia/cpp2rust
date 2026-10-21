extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn apply_0(mut x: (), mut fn_: FnPtr<fn(()) -> ()>) -> () {
    return ({ fn_.call(x) });
}
pub fn call_nttp_1(mut x: ()) -> () {
    return ({
        (FnPtr::<fn(()) -> ()>::new(|mut x: ()| -> () {
            {
                return (x - 1_());
            }
        }))
        .call(x)
    });
}
pub fn call_nttp_2(mut x: ()) -> () {
    return ({
        (FnPtr::<fn(()) -> ()>::new(|mut x: ()| -> () {
            {
                return (x * 10_());
            }
        }))
        .call(x)
    });
}
pub fn call_nttp_3(mut x: ()) -> () {
    return ({
        FnPtr::<fn(()) -> ()>::new(|mut x: ()| -> () {
            {
                return (x + 100_());
            }
        })
        .call(x)
    });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let mut fresh: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut x: ()| -> () {
        {
            return -x;
        }
    });
    unimplemented!();
    let twice: Value<FnPtr<fn(()) -> ()>> = Rc::new(RefCell::new(FnPtr::<fn(()) -> ()>::new(
        |mut x: ()| -> () {
            {
                return (x * 2_());
            }
        },
    )));
    let mut named: FnPtr<fn(()) -> ()> = (*twice.borrow()).clone();
    unimplemented!();
    unimplemented!();
    named = (fresh).clone();
    unimplemented!();
    let mut p: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut x: ()| -> () {
        {
            return (x * 3_());
        }
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let sum: Value<FnPtr<fn((), Box<[VaArg]>) -> ()>> =
        Rc::new(RefCell::new(FnPtr::<fn((), Box<[VaArg]>) -> ()>::new(
            |n: (), __args: Box<[VaArg]>| -> () {
                {
                    let __args: Value<Box<[VaArg]>> = Rc::new(RefCell::new(__args.into()));
                    let n: Value<()> = Rc::new(RefCell::new(n));
                    let ap: Value<Ptr<VaArg>> = Rc::new(RefCell::new(Ptr::<VaArg>::default()));
                    (*ap.borrow_mut()) = __args.as_pointer();
                    let s: Value<()> = Rc::new(RefCell::new(0_()));
                    let mut i: () = 0_();
                    'loop_: while (i < (*n.borrow())) {
                        (*s.borrow_mut()) += (*ap.borrow_mut()).arg::<()>();
                        i.postfix_inc();
                    }
                    return (*s.borrow());
                }
            },
        )));
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
