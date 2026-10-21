extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static g_cursor_0: Value<Ptr<Ptr<()>>> = Rc::new(RefCell::new(Ptr::<Ptr<()>>::null()));
);
pub fn advance_1() -> () {
    (*g_cursor_0.with(Value::clone).borrow()).with_mut(|__v| __v.prefix_inc());
    return 10_();
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct S {
    #[offset(0)]
    #[byte_size(8)]
    pub ptr: Ptr<()>,
}
pub fn by_ref_2(r: Ptr<Ptr<()>>) {
    let __rhs = ({ advance_1() });
    {
        elem!((r.read()), 1_()).with_mut(|__v| *__v = *__v + __rhs)
    };
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let a: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        1_(),
        2_(),
        3_(),
        4_(),
        5_(),
        6_(),
        7_(),
        8_(),
    ])));
    let q: Value<Ptr<()>> = Rc::new(RefCell::new((a.as_pointer() as Ptr<()>)));
    g_cursor_0.with(|rc| *rc.borrow_mut() = (q.as_pointer()));
    let __rhs = ({ advance_1() });
    elem!((*q.borrow()), 1_()).write(__rhs);
    unimplemented!();
    let __rhs = ({ advance_1() });
    {
        elem!((*q.borrow()), 1_()).with_mut(|__v| *__v = *__v + __rhs)
    };
    unimplemented!();
    let mut pq: Ptr<Ptr<()>> = (q.as_pointer());
    let __rhs = ({ advance_1() });
    {
        elem!((pq.read()), 1_()).with_mut(|__v| *__v = *__v + __rhs)
    };
    unimplemented!();
    let __rhs = ({ advance_1() });
    elem!((pq.read()), 1_()).write(__rhs);
    unimplemented!();
    ({ by_ref_2(q.as_pointer()) });
    unimplemented!();
    let s: Value<S> = Rc::new(RefCell::new(S {
        ptr: (a.as_pointer() as Ptr<()>),
    }));
    g_cursor_0.with(|rc| *rc.borrow_mut() = (field_ptr!(s.as_pointer(), ptr)));
    let __rhs = ({ advance_1() });
    {
        elem!({ (*s.borrow()).ptr.clone() }, 1_()).with_mut(|__v| *__v = *__v + __rhs)
    };
    unimplemented!();
    let mut sp: Ptr<S> = (s.as_pointer());
    let __rhs = ({ advance_1() });
    {
        elem!(sp.with(|__s| __s.ptr.clone()), 1_()).with_mut(|__v| *__v = *__v + __rhs)
    };
    unimplemented!();
    let mut b: Ptr<()> = (a.as_pointer() as Ptr<()>).reinterpret_cast::<()>();
    elem!(b, unimplemented!()).write(7_());
    {
        elem!(b, 4_()).with_mut(|__v| *__v = *__v + 1_())
    };
    unimplemented!();
    return unimplemented!();
}
pub fn __cpp2rust_init_globals() {
    let _ = g_cursor_0.with(|_| ());
}
