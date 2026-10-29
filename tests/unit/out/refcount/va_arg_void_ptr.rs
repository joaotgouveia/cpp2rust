extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct registry {
    #[offset(0)]
    #[byte_size(8)]
    pub slot: (),
    #[offset(8)]
    pub level: (),
}
pub type field = ();
pub const field_FIELD_SLOT: field = 0;
pub const field_FIELD_LEVEL: field = 1;
pub fn registry_update_0(mut r: Ptr<registry>, field: field, __args: Box<[VaArg]>) -> () {
    let __args: Value<Box<[VaArg]>> = Rc::new(RefCell::new(__args.into()));
    let field: Value<field> = Rc::new(RefCell::new(field));
    let mut result: () = 0_();
    let ap: Value<Ptr<VaArg>> = Rc::new(RefCell::new(Ptr::<VaArg>::default()));
    (*ap.borrow_mut()) = __args.as_pointer();
    'switch: {
        match { ((*field.borrow()) as ()) } {
            __v if __v == (field_FIELD_SLOT as i32) => {
                let __rhs = (*ap.borrow_mut()).arg::<()>();
                field!(r, slot).write(__rhs);
                break 'switch;
            }
            __v if __v == (field_FIELD_LEVEL as i32) => {
                let __rhs = (*ap.borrow_mut()).arg::<()>();
                field!(r, level).write(__rhs);
                break 'switch;
            }
            _ => {
                result = 1_();
                break 'switch;
            }
        }
    };
    return result;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let r: Value<registry> = Rc::new(RefCell::new(registry {
        slot: unimplemented!(),
        level: 0_(),
    }));
    let payload: Value<()> = Rc::new(RefCell::new(7_()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
