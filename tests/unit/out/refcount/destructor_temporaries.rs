extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static destroyed_0: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static last_1: Value<()> = Rc::new(RefCell::new(0_()));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor, Default)]
#[byte_size(4)]
pub struct S {
    #[offset(0)]
    pub v: (),
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor, Default)]
#[byte_size(8)]
pub struct Outer {
    #[offset(0)]
    #[byte_size(4)]
    pub inner: S,
    #[offset(4)]
    pub w: (),
}
pub fn make_2(mut v: ()) -> S {
    return S { v: v };
}
pub fn make_outer_3() -> Outer {
    return Outer {
        inner: S { v: 7_() },
        w: 8_(),
    };
}
pub fn use_4(s: Ptr<S>) -> () {
    unimplemented!();
    return s.with(|__s| __s.v);
}
pub fn value_5(s: Ptr<S>) -> () {
    return s.with(|__s| __s.v);
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    ({ make_2(1_()) });
    unimplemented!();
    destroyed_0.with(|rc| *rc.borrow_mut() = 0_());
    &({ make_2(2_()) });
    unimplemented!();
    destroyed_0.with(|rc| *rc.borrow_mut() = 0_());
    let mut x: () = { ({ make_2(3_()) }).v };
    unimplemented!();
    destroyed_0.with(|rc| *rc.borrow_mut() = 0_());
    x = ({ SImpl::get(&Rc::new(RefCell::new(({ make_2(4_()) }))).as_pointer()) });
    unimplemented!();
    destroyed_0.with(|rc| *rc.borrow_mut() = 0_());
    x = { ({ make_outer_3() }).inner.v };
    unimplemented!();
    destroyed_0.with(|rc| *rc.borrow_mut() = 0_());
    x = { S { v: 5_() }.v };
    unimplemented!();
    destroyed_0.with(|rc| *rc.borrow_mut() = 0_());
    x = ({
        let _s: Value<S> = Rc::new(RefCell::new(({ make_2(6_()) })));
        use_4(_s.as_pointer())
    });
    unimplemented!();
    destroyed_0.with(|rc| *rc.borrow_mut() = 0_());
    x = (({
        let _s: Value<S> = Rc::new(RefCell::new(S { v: 1_() }));
        value_5(_s.as_pointer())
    }) + ({
        let _s: Value<S> = Rc::new(RefCell::new(S { v: 2_() }));
        value_5(_s.as_pointer())
    }));
    unimplemented!();
    destroyed_0.with(|rc| *rc.borrow_mut() = 0_());
    let __rhs = {
        (if (x == 3_()) {
            ({ make_2(9_()) })
        } else {
            S { v: 10_() }
        })
        .v
    };
    x = __rhs;
    unimplemented!();
    destroyed_0.with(|rc| *rc.borrow_mut() = 0_());
    {
        let __tmp_0: Value<S> = Rc::new(RefCell::new(({ make_2(11_()) })));
        let r: Ptr<S> = __tmp_0.as_pointer();
        unimplemented!();
    }
    unimplemented!();
    destroyed_0.with(|rc| *rc.borrow_mut() = 0_());
    {
        let s: Value<S> = Rc::new(RefCell::new(S { v: 12_() }));
        let _dtor_s = ScopedDestructor::new(&s, |__p| __p.destructor());
        (*s.borrow_mut()) = ({ make_2(13_()) });
        unimplemented!();
    }
    unimplemented!();
    return 0_();
}
pub trait OuterImpl {
    fn destructor(&self);
}
impl OuterImpl for Ptr<Outer> {
    fn destructor(&self) {
        field_ptr!(self, inner).destructor();
    }
}
pub trait SImpl {
    fn destructor(&self);
    fn get(&self) -> ();
}
impl SImpl for Ptr<S> {
    fn destructor(&self) {
        (*destroyed_0.with(Value::clone).borrow_mut()).postfix_inc();
        last_1.with(|rc| *rc.borrow_mut() = (*self).with(|__s| __s.v));
    }
    fn get(&self) -> () {
        return (*self).with(|__s| __s.v);
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = destroyed_0.with(|_| ());
    let _ = last_1.with(|_| ());
}
