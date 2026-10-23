extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static copies_0: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static moves_1: Value<()> = Rc::new(RefCell::new(0_()));
);
#[derive(Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, Default)]
#[byte_size(4)]
pub struct Elem {
    #[offset(0)]
    pub v: (),
}
impl Elem {
    pub fn new(mut v: ()) -> Self {
        Self { v: v }
    }
    pub fn copy_from(other: Ptr<Elem>) -> Self {
        let __this: Elem = Self {
            v: (other.with(|__s| __s.v) + 100_()),
        };
        (*copies_0.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
    pub fn move_from(other: Ptr<Elem>) -> Self {
        let __this: Elem = Self {
            v: (other.with(|__s| __s.v) + 1000_()),
        };
        field!(other, v).write(-1_());
        (*moves_1.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
}
impl Clone for Elem {
    fn clone(&self) -> Self {
        let __src: Value<Elem> = Rc::new(RefCell::new(Elem { v: self.v.clone() }));
        Elem::copy_from(__src.as_pointer())
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, Default)]
#[byte_size(8)]
pub struct Two {
    #[offset(0)]
    #[byte_size(4)]
    pub a: Elem,
    #[offset(4)]
    #[byte_size(4)]
    pub b: Elem,
}
impl Two {
    pub fn move_from(_a0: Ptr<Two>) -> Self {
        Self {
            a: Elem::move_from({ field_ptr!(_a0, a) }),
            b: Elem::move_from({ field_ptr!(_a0, b) }),
        }
    }
}
pub fn make_two_2() -> Two {
    return Two {
        a: Elem::new({ 1_() }),
        b: Elem::new({ 2_() }),
    };
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let t: Value<Two> = Rc::new(RefCell::new(Two {
        a: Elem::new({ 1_() }),
        b: Elem::new({ 2_() }),
    }));
    let mut base_copies: () = copies_0.with(|rc| *rc.borrow());
    let mut base_moves: () = moves_1.with(|rc| *rc.borrow());
    let __decomp_3: Value<Two> = Rc::new(RefCell::new((*t.borrow()).clone()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let __decomp_4: Ptr<Two> = t.as_pointer();
    unimplemented!();
    unimplemented!();
    field!(field!(__decomp_4, a), v).write(50_());
    unimplemented!();
    let __decomp_5: Value<Two> = Rc::new(RefCell::new(Two::move_from({ unimplemented!() })));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut before_copies: () = copies_0.with(|rc| *rc.borrow());
    let mut before_moves: () = moves_1.with(|rc| *rc.borrow());
    let __decomp_6: Value<Two> = Rc::new(RefCell::new(({ make_two_2() })));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let p: Value<()> = Rc::new(RefCell::new({
        let __tmp_0: Value<Elem> = Rc::new(RefCell::new(Elem::new({ 7_() })));
        let __tmp_1: Value<Elem> = Rc::new(RefCell::new(Elem::new({ 8_() })));
        std_pair_Elem__Elem_::new({ __tmp_0.as_pointer() }, { __tmp_1.as_pointer() })
    }));
    before_copies = copies_0.with(|rc| *rc.borrow());
    let __decomp_7: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let x: Ptr<Elem> = ({ get_8(__decomp_7.as_pointer()) });
    let y: Ptr<Elem> = ({ get_9(__decomp_7.as_pointer()) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {
    let _ = copies_0.with(|_| ());
    let _ = moves_1.with(|_| ());
}
