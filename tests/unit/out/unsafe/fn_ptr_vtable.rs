extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Vtable {
    pub create: Option<unsafe fn(()) -> ()>,
    pub get: Option<unsafe fn(()) -> ()>,
    pub destroy: Option<unsafe fn(())>,
}
pub static mut storage_0: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub unsafe fn int_create_1(mut val: ()) -> () {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut storage_0)) = val;
    return ((&raw mut (*std::cell::LazyCell::force_mut(&mut *&raw mut storage_0)) as *mut ())
        as ());
}
pub unsafe fn int_get_2(mut p: ()) -> () {
    return (*(p as *mut ()));
}
pub unsafe fn int_destroy_3(mut p: ()) {
    (*(p as *mut ())) = unimplemented!();
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut vt: Vtable = Vtable {
        create: Some(int_create_1),
        get: Some(int_get_2),
        destroy: Some(int_destroy_3),
    };
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut obj: () = (unsafe { (vt.create).unwrap()(42_()) });
    unimplemented!();
    (unsafe { (vt.destroy).unwrap()(obj) });
    unimplemented!();
    (vt.get) = None;
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const storage_0);
}
