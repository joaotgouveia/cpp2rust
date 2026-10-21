extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(24)]
pub struct Vtable {
    #[offset(0)]
    #[byte_size(8)]
    pub create: FnPtr<fn(()) -> ()>,
    #[offset(8)]
    #[byte_size(8)]
    pub get: FnPtr<fn(()) -> ()>,
    #[offset(16)]
    #[byte_size(8)]
    pub destroy: FnPtr<fn(())>,
}
thread_local!(
    pub static storage_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
);
pub fn int_create_1(mut val: ()) -> () {
    storage_0.with(|rc| *rc.borrow_mut() = val);
    return ((storage_0.with(|v| v.as_pointer())) as Ptr<()>).to_any();
}
pub fn int_get_2(mut p: ()) -> () {
    return (p.reinterpret_cast::<()>().read());
}
pub fn int_destroy_3(mut p: ()) {
    p.reinterpret_cast::<()>().write(0_());
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let mut vt: Vtable = Vtable {
        create: FnPtr::<fn(()) -> ()>::new(int_create_1),
        get: FnPtr::<fn(()) -> ()>::new(int_get_2),
        destroy: FnPtr::<fn(())>::new(int_destroy_3),
    };
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut obj: () = ({ vt.create.call(42_()) });
    unimplemented!();
    ({ vt.destroy.call((obj).clone()) });
    unimplemented!();
    vt.get = FnPtr::<fn(()) -> ()>::null();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {
    let _ = storage_0.with(|_| ());
}
