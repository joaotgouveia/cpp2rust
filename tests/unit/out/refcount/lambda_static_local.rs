extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static a_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
);
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let next: Value<FnPtr<fn() -> ()>> =
        Rc::new(RefCell::new(FnPtr::<fn() -> ()>::new(|| -> () {
            {
                return (*n_1.with(Value::clone).borrow_mut()).prefix_inc();
            }
        })));
    unimplemented!();
    unimplemented!();
    let copy: Value<FnPtr<fn() -> ()>> = Rc::new(RefCell::new((*next.borrow()).copy_from()));
    unimplemented!();
    unimplemented!();
    let fresh: Value<FnPtr<fn() -> ()>> =
        Rc::new(RefCell::new(FnPtr::<fn() -> ()>::new(|| -> () {
            {
                return (*n_1.with(Value::clone).borrow_mut()).prefix_inc();
            }
        })));
    unimplemented!();
    unimplemented!();
    let per_type: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {},
        |mut x: ()| -> () {
            (*calls_2.with(Value::clone).borrow_mut()).postfix_inc();
            return calls_2.with(|rc| *rc.borrow());
        },
        |mut x: ()| -> () {
            (*calls_3.with(Value::clone).borrow_mut()).postfix_inc();
            return calls_3.with(|rc| *rc.borrow());
        }
    )));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let read: Value<FnPtr<fn(()) -> ()>> = Rc::new(RefCell::new(FnPtr::<fn(()) -> ()>::new(
        |mut _a0: ()| -> () {
            {
                return a_0.with(|rc| *rc.borrow());
            }
        },
    )));
    unimplemented!();
    a_0.with(|rc| *rc.borrow_mut() = 1_());
    unimplemented!();
    let local: Value<FnPtr<fn() -> ()>> =
        Rc::new(RefCell::new(FnPtr::<fn() -> ()>::new(|| -> () {
            {
                let mut p: P_4 = P_4 { x: 2_(), y: 3_() };
                return (p.x * p.y);
            }
        })));
    unimplemented!();
    return unimplemented!();
}
thread_local!(
    static n_1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
);
thread_local!(
    static calls_2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
);
thread_local!(
    static calls_3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct P_4 {
    #[offset(0)]
    pub x: (),
    #[offset(4)]
    pub y: (),
}
pub fn __cpp2rust_init_globals() {
    let _ = a_0.with(|_| ());
}
