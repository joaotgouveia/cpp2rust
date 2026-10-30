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
#[derive(Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, CopyAssign, MoveAssign)]
#[byte_size(4)]
pub struct Tracked {
    #[offset(0)]
    pub v: (),
}
impl Tracked {
    pub fn new_1() -> Self {
        Self { v: 0_() }
    }
    pub fn new_2(mut v: ()) -> Self {
        Self { v: v }
    }
    pub fn copy_from(o: Ptr<Tracked>) -> Self {
        let __this: Tracked = Self {
            v: o.with(|__s| __s.v),
        };
        (*copies_0.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
    pub fn move_from(o: Ptr<Tracked>) -> Self {
        Self {
            v: o.with(|__s| __s.v),
        }
    }
}
impl CppLess for Tracked {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_lt_1(__lhs.clone(), __rhs.clone())
    }
}
impl CppEq for Tracked {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        !(operator_lt_1(__lhs.clone(), __rhs.clone()))
            && !(operator_lt_1(__rhs.clone(), __lhs.clone()))
    }
}
impl Clone for Tracked {
    fn clone(&self) -> Self {
        let __src: Value<Tracked> = Rc::new(RefCell::new(Tracked { v: self.v.clone() }));
        Tracked::copy_from(__src.as_pointer())
    }
}
impl Default for Tracked {
    fn default() -> Self {
        { Tracked::new_1() }
    }
}
pub fn operator_lt_1(a: Ptr<Tracked>, b: Ptr<Tracked>) -> () {
    return ({ a.with(|__s| __s.v) } < { b.with(|__s| __s.v) });
}
pub fn greater_2(a: Ptr<Tracked>, b: Ptr<Tracked>) -> () {
    return ({ a.with(|__s| __s.v) } > { b.with(|__s| __s.v) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(24)]
pub struct Bag {
    #[offset(0)]
    #[byte_size(24)]
    pub items: (),
}
impl Bag {
    pub fn move_from(_a0: Ptr<Bag>) -> Self {
        Self {
            items: unimplemented!(),
        }
    }
}
impl CppLess for Bag {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_lt_3(__lhs.clone(), __rhs.clone())
    }
}
impl CppEq for Bag {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_4(__lhs.clone(), __rhs.clone())
    }
}
pub fn operator_lt_3(a: Ptr<Bag>, b: Ptr<Bag>) -> () {
    return ({ unimplemented!() } < { unimplemented!() });
}
pub fn operator_eq_4(a: Ptr<Bag>, b: Ptr<Bag>) -> () {
    return ({ unimplemented!() } == { unimplemented!() });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let arr: Value<Box<[Tracked]>> = Rc::new(RefCell::new(Box::new([
        Tracked::new_2({ 3_() }),
        Tracked::new_2({ 1_() }),
        Tracked::new_2({ 4_() }),
        Tracked::new_2({ 1_() }),
        Tracked::new_2({ 5_() }),
    ])));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut vec_: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let bags: Value<Box<[Bag]>> = Rc::new(RefCell::new(
        (0..3).map(|_| <Bag>::default()).collect::<Box<[Bag]>>(),
    ));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut bag_vec: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let bag_map: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!().write(2_());
    unimplemented!().write(0_());
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return copies_0.with(|rc| *rc.borrow());
}
pub trait BagImpl {
    fn move_assign(&self, _a0: Ptr<Bag>) -> Ptr<Bag>;
}
impl BagImpl for Ptr<Bag> {
    fn move_assign(&self, _a0: Ptr<Bag>) -> Ptr<Bag> {
        unimplemented!();
        return (*self).clone();
    }
}
pub trait TrackedImpl {
    fn copy_assign(&self, o: Ptr<Tracked>) -> Ptr<Tracked>;
    fn move_assign(&self, o: Ptr<Tracked>) -> Ptr<Tracked>;
}
impl TrackedImpl for Ptr<Tracked> {
    fn copy_assign(&self, o: Ptr<Tracked>) -> Ptr<Tracked> {
        field!((*self), v).write({ o.with(|__s| __s.v) });
        (*copies_0.with(Value::clone).borrow_mut()).prefix_inc();
        return (*self).clone();
    }
    fn move_assign(&self, o: Ptr<Tracked>) -> Ptr<Tracked> {
        field!((*self), v).write({ o.with(|__s| __s.v) });
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = copies_0.with(|_| ());
}
