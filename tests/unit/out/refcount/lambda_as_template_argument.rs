extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn apply_int_0(fn_: FnPtr<fn(()) -> ()>, mut x: ()) -> () {
    let fn_: Value<FnPtr<fn(()) -> ()>> = Rc::new(RefCell::new(fn_));
    return ({ (*fn_.borrow()).call(x) });
}
pub fn apply_int_1(fn_: FnPtr<Generic>, mut x: ()) -> () {
    let fn_: Value<FnPtr<Generic>> = Rc::new(RefCell::new(fn_));
    return ({ (*fn_.borrow()).spec::<fn(()) -> ()>(0).call(x) });
}
pub fn apply_int_2(fn_: FnPtr<Generic>, mut x: ()) -> () {
    let fn_: Value<FnPtr<Generic>> = Rc::new(RefCell::new(fn_));
    return ({ (*fn_.borrow()).spec::<fn(()) -> ()>(0).call(x) });
}
pub fn apply_int_3(fn_: FnPtr<Generic>, mut x: ()) -> () {
    let fn_: Value<FnPtr<Generic>> = Rc::new(RefCell::new(fn_));
    return ({ (*fn_.borrow()).spec::<fn(()) -> ()>(0).call(x) });
}
pub fn apply_double_4(fn_: FnPtr<Generic>, mut x: ()) -> () {
    let fn_: Value<FnPtr<Generic>> = Rc::new(RefCell::new(fn_));
    return ({ (*fn_.borrow()).spec::<fn(()) -> ()>(1).call(x) });
}
pub fn apply_double_5(fn_: FnPtr<Generic>, mut x: ()) -> () {
    let fn_: Value<FnPtr<Generic>> = Rc::new(RefCell::new(fn_));
    return ({ (*fn_.borrow()).spec::<fn(()) -> ()>(1).call(x) });
}
pub fn apply_twice_6(fn_: FnPtr<fn(()) -> ()>, mut x: ()) -> () {
    let fn_: Value<FnPtr<fn(()) -> ()>> = Rc::new(RefCell::new(fn_));
    return ({
        let _x: () = ({ (*fn_.borrow()).call(x) });
        (*fn_.borrow()).call(_x)
    })
    .clone();
}
pub fn none_match_7(pred: FnPtr<fn(()) -> ()>, mut a: (), mut b: ()) -> () {
    let pred: Value<FnPtr<fn(()) -> ()>> = Rc::new(RefCell::new(pred));
    let neg_pred: Value<FnPtr<fn(()) -> ()>> = Rc::new(RefCell::new(lambda!(
        {
            let pred: Ptr<FnPtr<fn(()) -> ()>> = pred.as_pointer();
        },
        |mut x: ()| -> () {
            return !({ (*pred.upgrade().deref()).call(x) });
        }
    )));
    return ({ (*neg_pred.borrow()).call(a) }) && ({ (*neg_pred.borrow()).call(b) });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let factor: Value<()> = Rc::new(RefCell::new(3_()));
    let scale: Value<FnPtr<fn(()) -> ()>> = Rc::new(RefCell::new(lambda!(
        {
            let factor: Value<()> = Rc::new(RefCell::new((*factor.borrow())));
        },
        |mut x: ()| -> () {
            return (x * (*factor.borrow()));
        }
    )));
    unimplemented!();
    unimplemented!();
    let generic_scale: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let factor: Value<()> = Rc::new(RefCell::new((*factor.borrow())));
        },
        |mut x: ()| -> () {
            return (x * (*factor.borrow()));
        },
        |mut x: ()| -> () {
            return (x * (*factor.borrow()));
        }
    )));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let offset: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let factor: Value<()> = Rc::new(RefCell::new((*factor.borrow())));
        },
        |mut x: ()| -> () {
            return (x + (*factor.borrow()));
        },
        |mut x: ()| -> () {
            return (x + (*factor.borrow()));
        }
    )));
    unimplemented!();
    unimplemented!();
    let is_even: Value<FnPtr<fn(()) -> ()>> = Rc::new(RefCell::new(FnPtr::<fn(()) -> ()>::new(
        |mut x: ()| -> () {
            {
                return ((x % 2_()) == 0_());
            }
        },
    )));
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
