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
pub fn forward_by_decltype_2(x: Ptr<Tracked>) -> Overload {
    return ({ chosen_overload_0((x).clone()) });
}
pub fn forward_by_decltype_3(x: Ptr<Tracked>) -> Overload {
    return ({ chosen_overload_1((x).clone()) });
}
pub fn forward_abbreviated_4(x: Ptr<Tracked>) -> Overload {
    return ({ chosen_overload_0((x).clone()) });
}
pub fn forward_abbreviated_5(x: Ptr<Tracked>) -> Overload {
    return ({ chosen_overload_1((x).clone()) });
}
pub fn forward_abbreviated_pack_6(args_0: Ptr<Tracked>, args_1: Ptr<Tracked>) -> () {
    return ({ (({ chosen_overload_0((args_0).clone()) }) as ()) } + {
        (({ chosen_overload_1((args_1).clone()) }) as ())
    });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let a: Value<Tracked> = Rc::new(RefCell::new(Tracked::new({ 3_() })));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let b: Value<Tracked> = Rc::new(RefCell::new(Tracked::new({ 5_() })));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let c: Value<Tracked> = Rc::new(RefCell::new(Tracked::new({ 7_() })));
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
