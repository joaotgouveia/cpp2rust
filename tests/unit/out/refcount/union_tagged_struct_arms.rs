extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type Choice_enum = ();
pub const Choice_enum_C_LIST: Choice_enum = 1;
pub const Choice_enum_C_LETTERS: Choice_enum = 2;
pub const Choice_enum_C_INTEGERS: Choice_enum = 3;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(24)]
pub struct anon_1 {
    #[offset(0)]
    #[byte_size(8)]
    pub items: Ptr<Ptr<()>>,
    #[offset(8)]
    pub count: (),
    #[offset(16)]
    pub cursor: (),
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct anon_2 {
    #[offset(0)]
    pub lo: (),
    #[offset(4)]
    pub hi: (),
    #[offset(8)]
    pub curr: (),
    #[offset(12)]
    pub step: (),
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(40)]
pub struct anon_3 {
    #[offset(0)]
    pub lo: (),
    #[offset(8)]
    pub hi: (),
    #[offset(16)]
    pub curr: (),
    #[offset(24)]
    pub step: (),
    #[offset(32)]
    pub width: (),
}
#[derive(VaArg, FnPtrArg, ByteRepr, Clone)]
#[byte_size(40)]
pub struct anon_0 {
    #[offset(0)]
    #[byte_size(40)]
    __bytes: Box<[u8]>,
}
impl anon_0 {
    pub fn list(this: Ptr<Self>) -> Ptr<anon_1> {
        this.reinterpret_cast()
    }
    pub fn letters(this: Ptr<Self>) -> Ptr<anon_2> {
        this.reinterpret_cast()
    }
    pub fn integers(this: Ptr<Self>) -> Ptr<anon_3> {
        this.reinterpret_cast()
    }
}
impl Default for anon_0 {
    fn default() -> Self {
        anon_0 {
            __bytes: Box::from([0u8; 40]),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(48)]
pub struct Branch {
    #[offset(0)]
    pub choice: Choice_enum,
    #[offset(4)]
    pub index: (),
    #[offset(8)]
    #[byte_size(40)]
    pub v: anon_0,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let p_list: Value<Branch> = <Value<Branch>>::default();
    (*p_list.borrow_mut()).choice = Choice_enum_C_LIST;
    (*p_list.borrow_mut()).index = 0_();
    field!(anon_0::list(field_ptr!(p_list.as_pointer(), v)), items)
        .write((items_4.with(|v| v.as_pointer()) as Ptr<Ptr<()>>));
    field!(anon_0::list(field_ptr!(p_list.as_pointer(), v)), count).write(3_());
    field!(anon_0::list(field_ptr!(p_list.as_pointer(), v)), cursor).write(1_());
    unimplemented!();
    unimplemented!();
    let p_letters: Value<Branch> = <Value<Branch>>::default();
    (*p_letters.borrow_mut()).choice = Choice_enum_C_LETTERS;
    (*p_letters.borrow_mut()).index = 1_();
    field!(anon_0::letters(field_ptr!(p_letters.as_pointer(), v)), lo).write(('a' as ()));
    field!(anon_0::letters(field_ptr!(p_letters.as_pointer(), v)), hi).write(('z' as ()));
    field!(anon_0::letters(field_ptr!(p_letters.as_pointer(), v)), curr).write(('m' as ()));
    field!(anon_0::letters(field_ptr!(p_letters.as_pointer(), v)), step).write(1_());
    unimplemented!();
    let p_integers: Value<Branch> = <Value<Branch>>::default();
    (*p_integers.borrow_mut()).choice = Choice_enum_C_INTEGERS;
    (*p_integers.borrow_mut()).index = 2_();
    field!(anon_0::integers(field_ptr!(p_integers.as_pointer(), v)), lo).write(1_());
    field!(anon_0::integers(field_ptr!(p_integers.as_pointer(), v)), hi).write(100_());
    field!(
        anon_0::integers(field_ptr!(p_integers.as_pointer(), v)),
        curr
    )
    .write(1_());
    field!(
        anon_0::integers(field_ptr!(p_integers.as_pointer(), v)),
        step
    )
    .write(1_());
    field!(
        anon_0::integers(field_ptr!(p_integers.as_pointer(), v)),
        width
    )
    .write(3_());
    unimplemented!();
    unimplemented!();
    return 0_();
}
thread_local!(
    static items_4: Value<Box<[Ptr<()>]>> = Rc::new(RefCell::new(Box::new([
        Ptr::<()>::from_string_literal(b"a"),
        Ptr::<()>::from_string_literal(b"b"),
        Ptr::<()>::from_string_literal(b"c"),
    ])));
);
pub fn __cpp2rust_init_globals() {}
