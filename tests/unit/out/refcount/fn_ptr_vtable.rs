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
    pub create: FnPtr<fn(i32) -> AnyPtr>,
    #[offset(8)]
    #[byte_size(8)]
    pub get: FnPtr<fn(AnyPtr) -> i32>,
    #[offset(16)]
    #[byte_size(8)]
    pub destroy: FnPtr<fn(AnyPtr)>,
}
thread_local!(
    pub static storage_0: Value<i32> = Rc::new(RefCell::new(0_i32));
);
pub fn int_create_1(mut val: i32) -> AnyPtr {
    storage_0.with(|rc| *rc.borrow_mut() = val);
    return ((storage_0.with(|v| v.as_pointer())) as Ptr<i32>).to_any();
}
pub fn int_get_2(mut p: AnyPtr) -> i32 {
    return (p.reinterpret_cast::<i32>().read());
}
pub fn int_destroy_3(mut p: AnyPtr) {
    p.reinterpret_cast::<i32>().write(0);
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut vt: Vtable = Vtable {
        create: FnPtr::<fn(i32) -> AnyPtr>::new(int_create_1),
        get: FnPtr::<fn(AnyPtr) -> i32>::new(int_get_2),
        destroy: FnPtr::<fn(AnyPtr)>::new(int_destroy_3),
    };
    (if !((vt.create).is_null()) {
        &(0);
    } else {
        ({
            __assert_fail_4(
                Ptr::<i8>::from_string_literal(b"vt.create != nullptr"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if !((vt.get).is_null()) {
        &(0);
    } else {
        ({
            __assert_fail_4(
                Ptr::<i8>::from_string_literal(b"vt.get != nullptr"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if !((vt.destroy).is_null()) {
        &(0);
    } else {
        ({
            __assert_fail_4(
                Ptr::<i8>::from_string_literal(b"vt.destroy != nullptr"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut obj: AnyPtr = ({ vt.create.call(42) });
    (if (({ vt.get.call((obj).clone()) }) == 42) {
        &(0);
    } else {
        ({
            __assert_fail_4(
                Ptr::<i8>::from_string_literal(b"vt.get(obj) == 42"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    ({ vt.destroy.call((obj).clone()) });
    (if (storage_0.with(|rc| *rc.borrow()) == 0) {
        &(0);
    } else {
        ({
            __assert_fail_4(
                Ptr::<i8>::from_string_literal(b"storage == 0"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    vt.get = FnPtr::<fn(AnyPtr) -> i32>::null();
    (if (vt.get).is_null() {
        &(0);
    } else {
        ({
            __assert_fail_4(
                Ptr::<i8>::from_string_literal(b"vt.get == nullptr"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    return 0;
}
pub fn __assert_fail_4(
    mut __assertion: Ptr<i8>,
    mut __file: Ptr<i8>,
    mut __line: u32,
    mut __function: Ptr<i8>,
) {
    unimplemented!()
}
pub fn __cpp2rust_init_globals() {
    let _ = storage_0.with(|_| ());
}
