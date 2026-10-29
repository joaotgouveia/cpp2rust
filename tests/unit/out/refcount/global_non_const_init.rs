extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn next_0() -> () {
    return (*counter_1.with(Value::clone).borrow_mut()).prefix_inc();
}
thread_local!(
    static counter_1: Value<()> = Rc::new(RefCell::new(0_()));
);
pub fn marker_2(mut tag: ()) -> () {
    return ((tag << 3_()) | 2_());
}
thread_local!(
    pub static signature_3: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        ({ marker_2(1_()) }),
        4_(),
        ('B' as ()),
    ])));
);
thread_local!(
    pub static single_4: Value<()> = Rc::new(RefCell::new(({ marker_2(2_()) })));
);
thread_local!(
    pub static from_call_5: Value<()> = Rc::new(RefCell::new(({ next_0() })));
);
thread_local!(
    pub static depends_on_call_6: Value<()> =
        Rc::new(RefCell::new((from_call_5.with(|rc| *rc.borrow()) + 1_())));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct Ctor {
    #[offset(0)]
    pub v: (),
}
impl Ctor {
    pub fn new_1() -> Self {
        Self { v: ({ next_0() }) }
    }
    pub fn new_2(mut x: ()) -> Self {
        Self { v: x }
    }
}
impl Default for Ctor {
    fn default() -> Self {
        { Ctor::new_1() }
    }
}
thread_local!(
    pub static default_ctor_7: Value<Ctor> = Rc::new(RefCell::new(Ctor::new_1()));
);
thread_local!(
    pub static arg_ctor_8: Value<Ctor> = Rc::new(RefCell::new(Ctor::new_2({ 7_() })));
);
thread_local!(
    pub static str_9: Value<std_basic_string_char__std_char_traits_char___std_allocator_char__> =
        Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abc") },
                None,
            ),
        ));
);
thread_local!(
    pub static inline_member_11: Value<Ctor> = Rc::new(RefCell::new(Ctor::new_2({ 5_() })));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Holder {}
thread_local!(
    pub static member_10: Value<()> = Rc::new(RefCell::new(({ next_0() })));
);
pub fn local_static_12() -> () {
    return (once_13.with(|rc| *rc.borrow()) + { (*local_ctor_14.with(Value::clone).borrow()).v });
}
thread_local!(
    static once_13: Value<()> = Rc::new(RefCell::new(({ next_0() })));
);
thread_local!(
    static local_ctor_14: Value<Ctor> = Rc::new(RefCell::new(Ctor::new_2({ 3_() })));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct Singleton {
    #[offset(0)]
    pub hits: (),
}
impl Singleton {
    pub fn new() -> Self {
        Self { hits: 0_() }
    }
    pub fn instance() -> Ptr<Singleton> {
        return s_15.with(|v| v.as_pointer());
    }
}
impl Default for Singleton {
    fn default() -> Self {
        { Singleton::new() }
    }
}
thread_local!(
    static s_15: Value<Singleton> = Rc::new(RefCell::new(Singleton::new()));
);
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
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    field!(({ Singleton::instance() }), hits).with_mut(|__v| __v.postfix_inc());
    field!(({ Singleton::instance() }), hits).with_mut(|__v| __v.postfix_inc());
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {
    let _ = signature_3.with(|_| ());
    let _ = single_4.with(|_| ());
    let _ = from_call_5.with(|_| ());
    let _ = depends_on_call_6.with(|_| ());
    let _ = default_ctor_7.with(|_| ());
    let _ = arg_ctor_8.with(|_| ());
    let _ = str_9.with(|_| ());
    let _ = inline_member_11.with(|_| ());
    let _ = member_10.with(|_| ());
}
