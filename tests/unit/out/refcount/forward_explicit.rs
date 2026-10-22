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
pub const Overload_kIntLvalueOverload: Overload = 3;
pub const Overload_kIntRvalueOverload: Overload = 4;
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
pub fn chosen_overload_2(_a0: Ptr<()>) -> Overload {
    return Overload_kIntLvalueOverload;
}
pub fn chosen_overload_3(_a0: Ptr<()>) -> Overload {
    return Overload_kIntRvalueOverload;
}
pub fn copy_or_move_into_param_4(t: Tracked) -> Tracked {
    let t: Value<Tracked> = Rc::new(RefCell::new(t));
    return Tracked::move_from({ t.as_pointer() });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let a: Value<Tracked> = Rc::new(RefCell::new(Tracked::new({ 5_() })));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let b: Value<Tracked> = Rc::new(RefCell::new(Tracked::new({ 6_() })));
    let mut moved: Tracked =
        ({ copy_or_move_into_param_4(Tracked::move_from({ b.as_pointer() })) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let c: Value<Tracked> = Rc::new(RefCell::new(Tracked::new({ 7_() })));
    let mut copied: Tracked =
        ({ copy_or_move_into_param_4(Tracked::copy_from({ c.as_pointer() })) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let i: Value<()> = Rc::new(RefCell::new(8_()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
