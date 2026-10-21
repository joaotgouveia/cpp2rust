extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static counter_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
);
thread_local!(
    pub static inc_1: Value<FnPtr<fn(()) -> ()>> = Rc::new(RefCell::new(
        FnPtr::<fn(()) -> ()>::new(|mut x: ()| -> () {
            {
                return (x + 1_());
            }
        }),
    ));
);
thread_local!(
    pub static bump_2: Value<FnPtr<fn() -> ()>> =
        Rc::new(RefCell::new(FnPtr::<fn() -> ()>::new(|| -> () {
            {
                (*counter_0.with(Value::clone).borrow_mut()).postfix_inc();
                return counter_0.with(|rc| *rc.borrow());
            }
        })));
);
pub fn apply_3(f: FnPtr<fn(()) -> ()>, mut x: ()) -> () {
    let f: Value<FnPtr<fn(()) -> ()>> = Rc::new(RefCell::new(f));
    return ({ (*f.borrow()).call(x) });
}
pub fn by_ref_4(v: Ptr<()>) -> () {
    return ((v.read()) * 2_());
}
thread_local!(
    pub static twice_next_5: Value<FnPtr<fn(()) -> ()>> =
        Rc::new(RefCell::new(FnPtr::<fn(()) -> ()>::new(
            |mut x: ()| -> () {
                {
                    return ({
                        let _v: Value<()> = Rc::new(RefCell::new((x + 1_())));
                        by_ref_4(_v.as_pointer())
                    });
                }
            },
        )));
);
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    unimplemented!();
    ({ (*bump_2.with(Value::clone).borrow()).call() });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let copy: Value<FnPtr<fn(()) -> ()>> = Rc::new(RefCell::new(
        (*inc_1.with(Value::clone).borrow()).copy_from(),
    ));
    unimplemented!();
    let mut fp: FnPtr<fn(()) -> ()> = (*inc_1.with(Value::clone).borrow()).clone();
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub fn __cpp2rust_init_globals() {
    let _ = counter_0.with(|_| ());
    let _ = inc_1.with(|_| ());
    let _ = bump_2.with(|_| ());
    let _ = twice_next_5.with(|_| ());
}
