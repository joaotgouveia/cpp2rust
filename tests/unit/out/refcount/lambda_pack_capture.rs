extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn sum_0() -> () {
    return 0_();
}
pub fn sum_1(mut t: (), mut u_1: (), mut u_2: ()) -> () {
    return (t + ({ sum_2(u_1, u_2) }));
}
pub fn sum_2(mut t: (), mut u: ()) -> () {
    return (t + ({ sum_3(u) }));
}
pub fn sum_3(mut t: ()) -> () {
    return (t + ({ sum_0() }));
}
pub fn by_value_4() -> () {
    let f: Value<FnPtr<fn() -> ()>> = Rc::new(RefCell::new(FnPtr::<fn() -> ()>::new(|| -> () {
        {
            return ({ sum_0() });
        }
    })));
    unimplemented!();
    return ({ (*f.borrow()).call() }).clone();
}
pub fn by_value_5(t_0: (), t_1: (), t_2: ()) -> () {
    let t_0: Value<()> = Rc::new(RefCell::new(t_0));
    let t_1: Value<()> = Rc::new(RefCell::new(t_1));
    let t_2: Value<()> = Rc::new(RefCell::new(t_2));
    let f: Value<FnPtr<fn() -> ()>> = Rc::new(RefCell::new(lambda!(
        {
            let t_0: Value<()> = Rc::new(RefCell::new((*t_0.borrow())));
            let t_1: Value<()> = Rc::new(RefCell::new((*t_1.borrow())));
            let t_2: Value<()> = Rc::new(RefCell::new((*t_2.borrow())));
        },
        || -> () {
            return ({ sum_1((*t_0.borrow()), (*t_1.borrow()), (*t_2.borrow())) });
        }
    )));
    {
        (*t_0.borrow_mut()) = 0_();
        {
            (*t_1.borrow_mut()) = 0_();
            (*t_2.borrow_mut()) = 0_()
        }
    };
    return ({ (*f.borrow()).call() }).clone();
}
pub fn by_ref_6(t_0: (), t_1: (), t_2: ()) -> () {
    let t_0: Value<()> = Rc::new(RefCell::new(t_0));
    let t_1: Value<()> = Rc::new(RefCell::new(t_1));
    let t_2: Value<()> = Rc::new(RefCell::new(t_2));
    let f: Value<FnPtr<fn()>> = Rc::new(RefCell::new(lambda!(
        {
            let t_0: Ptr<()> = t_0.as_pointer();
            let t_1: Ptr<()> = t_1.as_pointer();
            let t_2: Ptr<()> = t_2.as_pointer();
        },
        || {
            {
                {
                    t_0.with_mut(|__v| *__v = *__v * 2_())
                };
                {
                    {
                        t_1.with_mut(|__v| *__v = *__v * 2_())
                    };
                    { t_2.with_mut(|__v| *__v = *__v * 2_()) }
                }
            };
        }
    )));
    ({ (*f.borrow()).call() });
    return ({ sum_1((*t_0.borrow()), (*t_1.borrow()), (*t_2.borrow())) });
}
pub fn init_pack_7(mut t_0: (), mut t_1: (), mut t_2: ()) -> () {
    let f: Value<FnPtr<fn() -> ()>> = Rc::new(RefCell::new(lambda!(
        {
            let xs_0: Value<()> = Rc::new(RefCell::new((t_0 + 1_())));
            let xs_1: Value<()> = Rc::new(RefCell::new((t_1 + 1_())));
            let xs_2: Value<()> = Rc::new(RefCell::new((t_2 + 1_())));
        },
        || -> () {
            return ({ sum_1((*xs_0.borrow()), (*xs_1.borrow()), (*xs_2.borrow())) });
        }
    )));
    return ({ (*f.borrow()).call() }).clone();
}
pub fn implicit_8(t_0: (), t_1: ()) -> () {
    let t_0: Value<()> = Rc::new(RefCell::new(t_0));
    let t_1: Value<()> = Rc::new(RefCell::new(t_1));
    return ({
        lambda!(
            {
                let t_0: Value<()> = Rc::new(RefCell::new((*t_0.borrow())));
                let t_1: Value<()> = Rc::new(RefCell::new((*t_1.borrow())));
            },
            || -> () {
                return ({ sum_2((*t_0.borrow()), (*t_1.borrow())) });
            }
        )
        .call()
    });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
