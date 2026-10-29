extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let v: Value<std_vector_int__std_allocator_int__> = Rc::new(RefCell::new(
        std_vector_int__std_allocator_int__::new_1({ unimplemented!() }, None),
    ));
    let f: Value<FnPtr<fn() -> ()>> = Rc::new(RefCell::new(lambda!(
        {
            let v: Value<std_vector_int__std_allocator_int__> =
                Rc::new(RefCell::new((*v.borrow()).clone()));
        },
        || -> () {
            return ({ (*v.borrow()).size() });
        }
    )));
    let g: Value<FnPtr<fn() -> ()>> = Rc::new(RefCell::new((*f.borrow()).copy_from()));
    let h: Value<FnPtr<fn() -> ()>> = Rc::new(RefCell::new((*f.borrow()).move_from()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let p: Value<FnPtr<fn() -> ()>> = Rc::new(RefCell::new(lambda!(
        {
            let u: Value<std_unique_ptr_int__std_default_delete_int__> = Rc::new(RefCell::new(
                ({
                    let ___args: Value<()> = Rc::new(RefCell::new(5_()));
                    make_unique_0(___args.as_pointer())
                }),
            ));
        },
        || -> () {
            return if !({ (*u.borrow()).get() }).is_null() {
                ((*u.borrow()).read())
            } else {
                0_()
            };
        }
    )));
    let q: Value<FnPtr<fn() -> ()>> = Rc::new(RefCell::new((*p.borrow()).move_from()));
    unimplemented!();
    unimplemented!();
    let n: Value<()> = Rc::new(RefCell::new(0_()));
    let inner: Value<FnPtr<fn() -> ()>> = Rc::new(RefCell::new(lambda!(
        {
            let n: Value<()> = Rc::new(RefCell::new((*n.borrow())));
        },
        || -> () {
            return (*n.borrow_mut()).prefix_inc();
        }
    )));
    let outer: Value<FnPtr<fn() -> ()>> = Rc::new(RefCell::new(lambda!(
        {
            let inner: Value<FnPtr<fn() -> ()>> =
                Rc::new(RefCell::new((*inner.borrow()).copy_from()));
        },
        || -> () {
            return ({ (*inner.borrow()).call() }).clone();
        }
    )));
    unimplemented!();
    let outer2: Value<FnPtr<fn() -> ()>> = Rc::new(RefCell::new((*outer.borrow()).copy_from()));
    unimplemented!();
    unimplemented!();
    let outer3: Value<FnPtr<fn() -> ()>> = Rc::new(RefCell::new((*outer.borrow()).move_from()));
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
