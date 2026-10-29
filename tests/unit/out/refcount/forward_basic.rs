extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type Overload = ();
pub const Overload_kLvalueOverload: Overload = 1;
pub const Overload_kRvalueOverload: Overload = 2;
#[derive(Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, Default)]
#[byte_size(12)]
pub struct Tracked {
    #[offset(0)]
    pub v: (),
    #[offset(4)]
    pub copies: (),
    #[offset(8)]
    pub moves: (),
}
impl Tracked {
    pub fn new(mut v: ()) -> Self {
        Self {
            v: v,
            copies: 0_(),
            moves: 0_(),
        }
    }
    pub fn copy_from(o: Ptr<Tracked>) -> Self {
        Self {
            v: o.with(|__s| __s.v),
            copies: (o.with(|__s| __s.copies) + 1_()),
            moves: o.with(|__s| __s.moves),
        }
    }
    pub fn move_from(o: Ptr<Tracked>) -> Self {
        let __this: Tracked = Self {
            v: o.with(|__s| __s.v),
            copies: o.with(|__s| __s.copies),
            moves: (o.with(|__s| __s.moves) + 1_()),
        };
        field!(o, v).write(0_());
        __this
    }
}
impl Clone for Tracked {
    fn clone(&self) -> Self {
        let __src: Value<Tracked> = Rc::new(RefCell::new(Tracked {
            v: self.v.clone(),
            copies: self.copies.clone(),
            moves: self.moves.clone(),
        }));
        Tracked::copy_from(__src.as_pointer())
    }
}
pub fn chosen_overload_0(_a0: Ptr<Tracked>) -> Overload {
    return Overload_kLvalueOverload;
}
pub fn chosen_overload_1(_a0: Ptr<Tracked>) -> Overload {
    return Overload_kRvalueOverload;
}
impl Holder {
    pub fn new_1(x: Ptr<Tracked>) -> Self {
        Self {
            t: Tracked::copy_from({ (x).clone() }),
        }
    }
}
impl Holder {
    pub fn new_2(x: Ptr<Tracked>) -> Self {
        Self {
            t: Tracked::move_from({ (x).clone() }),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, Default)]
#[byte_size(12)]
pub struct Holder {
    #[offset(0)]
    #[byte_size(12)]
    pub t: Tracked,
}
impl Holder {
    pub fn move_from(_a0: Ptr<Holder>) -> Self {
        Self {
            t: Tracked::move_from({ field_ptr!(_a0, t) }),
        }
    }
}
pub fn forward_once_2(x: Ptr<Tracked>) -> Overload {
    return ({ chosen_overload_0((x).clone()) });
}
pub fn forward_once_3(x: Ptr<Tracked>) -> Overload {
    return ({ chosen_overload_1((x).clone()) });
}
pub fn forward_twice_4(x: Ptr<Tracked>) -> Overload {
    return ({ forward_once_2((x).clone()) });
}
pub fn forward_twice_5(x: Ptr<Tracked>) -> Overload {
    return ({ forward_once_3((x).clone()) });
}
pub fn forward_into_ctor_6(x: Ptr<Tracked>) -> Holder {
    return Holder::new_1({ (x).clone() });
}
pub fn forward_into_ctor_7(x: Ptr<Tracked>) -> Holder {
    return Holder::new_2({ (x).clone() });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let lvalue: Value<Tracked> = Rc::new(RefCell::new(Tracked::new({ 7_() })));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let relayed: Value<Tracked> = Rc::new(RefCell::new(Tracked::new({ 9_() })));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let kept: Value<Tracked> = Rc::new(RefCell::new(Tracked::new({ 11_() })));
    let mut from_lvalue: Holder = ({ forward_into_ctor_6(kept.as_pointer()) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut from_rvalue: Holder = ({
        let _x: Value<Tracked> = Rc::new(RefCell::new(Tracked::new({ 12_() })));
        forward_into_ctor_7(_x.as_pointer())
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
