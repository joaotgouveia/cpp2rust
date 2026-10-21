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
    let fact: Value<FnPtr<Generic>> =
        Rc::new(RefCell::new(lambda!(Generic, {}, |self_: FnPtr<
            Generic,
        >,
                                                   mut n: ()|
         -> () {
            let self_: Value<FnPtr<Generic>> = Rc::new(RefCell::new(self_));
            if (n <= 1_()) {
                return 1_();
            }
            return (n
                * ({
                    let _self_: FnPtr<Generic> = (*self_.borrow()).copy_from();
                    (*self_.borrow())
                        .spec::<fn(FnPtr<Generic>, ()) -> ()>(0)
                        .call(_self_, (n - 1_()))
                }));
        })));
    unimplemented!();
    let calls: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let fib: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let calls: Ptr<()> = calls.as_pointer();
        },
        |self_: Ptr<FnPtr<Generic>>, mut n: ()| -> () {
            calls.with_mut(|__v| __v.postfix_inc());
            if (n <= 2_()) {
                return 1_();
            }
            return (({
                let _self_: Ptr<FnPtr<Generic>> = (self_).clone();
                let _n: () = (n - 1_());
                (*self_.upgrade().deref())
                    .spec::<fn(Ptr<FnPtr<Generic>>, ()) -> ()>(0)
                    .call(_self_, _n)
            }) + ({
                let _self_: Ptr<FnPtr<Generic>> = (self_).clone();
                let _n: () = (n - 2_());
                (*self_.upgrade().deref())
                    .spec::<fn(Ptr<FnPtr<Generic>>, ()) -> ()>(0)
                    .call(_self_, _n)
            }));
        }
    )));
    unimplemented!();
    unimplemented!();
    let depth: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let count_down: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let depth: Ptr<()> = depth.as_pointer();
        },
        |self_: Ptr<FnPtr<Generic>>, mut n: ()| {
            if (n == unimplemented!()) {
                return;
            }
            depth.with_mut(|__v| __v.postfix_inc());
            ({
                let _self_: Ptr<FnPtr<Generic>> = (self_).clone();
                let _n: () = (n - 1_());
                (*self_.upgrade().deref())
                    .spec::<fn(Ptr<FnPtr<Generic>>, ())>(0)
                    .call(_self_, _n)
            });
        }
    )));
    ({
        let _self_: Ptr<FnPtr<Generic>> = count_down.as_pointer();
        (*count_down.borrow())
            .spec::<fn(Ptr<FnPtr<Generic>>, ())>(0)
            .call(_self_, 4_())
    });
    unimplemented!();
    return unimplemented!();
}
pub fn __cpp2rust_init_globals() {}
