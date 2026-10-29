extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type Width_enum = ();
pub const Width_enum_W_64: Width_enum = 0;
pub const Width_enum_W_32: Width_enum = 1;
pub const Width_enum_W_16: Width_enum = 2;
#[derive(VaArg, FnPtrArg, ByteRepr, Clone)]
#[byte_size(8)]
pub struct anon_0 {
    #[offset(0)]
    #[byte_size(8)]
    __bytes: Box<[u8]>,
}
impl anon_0 {
    pub fn text(this: Ptr<Self>) -> Ptr<Ptr<()>> {
        this.reinterpret_cast()
    }
    pub fn handle(this: Ptr<Self>) -> Ptr<()> {
        this.reinterpret_cast()
    }
    pub fn signed_n(this: Ptr<Self>) -> Ptr<()> {
        this.reinterpret_cast()
    }
    pub fn f(this: Ptr<Self>) -> Ptr<()> {
        this.reinterpret_cast()
    }
}
impl Default for anon_0 {
    fn default() -> Self {
        anon_0 {
            __bytes: Box::from([0u8; 8]),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct Sink {
    #[offset(0)]
    pub width: Width_enum,
    #[offset(8)]
    #[byte_size(8)]
    pub out: anon_0,
}
pub fn write_count_1(mut s: Ptr<Sink>, mut count: ()) {
    'switch: {
        match { (s.with(|__s| __s.width) as ()) } {
            __v if __v == (Width_enum_W_64 as i32) => {
                (anon_0::handle(field_ptr!(s, out)).read())
                    .reinterpret_cast::<()>()
                    .write(count);
                break 'switch;
            }
            __v if __v == (Width_enum_W_32 as i32) => {
                (anon_0::handle(field_ptr!(s, out)).read())
                    .reinterpret_cast::<()>()
                    .write((count as ()));
                break 'switch;
            }
            __v if __v == (Width_enum_W_16 as i32) => {
                (anon_0::handle(field_ptr!(s, out)).read())
                    .reinterpret_cast::<()>()
                    .write((count as ()));
                break 'switch;
            }
            _ => {}
        }
    };
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let buf64: Value<()> = Rc::new(RefCell::new(0_()));
    let buf32: Value<()> = Rc::new(RefCell::new(0_()));
    let buf16: Value<()> = Rc::new(RefCell::new(0_()));
    let s: Value<Sink> = <Value<Sink>>::default();
    (*s.borrow_mut()).width = Width_enum_W_64;
    anon_0::handle(field_ptr!(s.as_pointer(), out))
        .write(((buf64.as_pointer()) as Ptr<()>).to_any());
    ({ write_count_1((s.as_pointer()), 1234605616436508552_()) });
    unimplemented!();
    (*s.borrow_mut()).width = Width_enum_W_32;
    anon_0::handle(field_ptr!(s.as_pointer(), out))
        .write(((buf32.as_pointer()) as Ptr<()>).to_any());
    ({ write_count_1((s.as_pointer()), 305419896_()) });
    unimplemented!();
    (*s.borrow_mut()).width = Width_enum_W_16;
    anon_0::handle(field_ptr!(s.as_pointer(), out))
        .write(((buf16.as_pointer()) as Ptr<()>).to_any());
    ({ write_count_1((s.as_pointer()), 4660_()) });
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
