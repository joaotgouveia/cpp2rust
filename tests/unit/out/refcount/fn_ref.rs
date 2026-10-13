extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn square_0(mut x: i32) -> i32 {
    return (x * x);
}
pub fn twice_1(mut x: i32) -> i32 {
    return (2 * x);
}
pub fn call_ref_2(f: FnPtr<fn(i32) -> i32>, mut x: i32) -> i32 {
    return ({ f.call(x) });
}
pub fn call_deduced_3(f: FnPtr<fn(i32) -> i32>, mut x: i32) -> i32 {
    return ({ f.call(x) });
}
pub fn call_forwarded_4(f: FnPtr<fn(i32) -> i32>, mut x: i32) -> i32 {
    return ({ f.call(x) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Holder {
    #[offset(0)]
    #[byte_size(8)]
    pub f: FnPtr<fn(i32) -> i32>,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    (if (({ call_ref_2(FnPtr::<fn(i32) -> i32>::new(square_0), 3) }) == 9) {
        &(0);
    } else {
        ({
            __assert_fail_5(
                Ptr::<i8>::from_string_literal(b"call_ref(square, 3) == 9"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ call_ref_2(FnPtr::<fn(i32) -> i32>::new(twice_1), 3) }) == 6) {
        &(0);
    } else {
        ({
            __assert_fail_5(
                Ptr::<i8>::from_string_literal(b"call_ref(twice, 3) == 6"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let r: FnPtr<fn(i32) -> i32> = FnPtr::<fn(i32) -> i32>::new(square_0);
    (if (({ r.call(4) }) == 16) {
        &(0);
    } else {
        ({
            __assert_fail_5(
                Ptr::<i8>::from_string_literal(b"r(4) == 16"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ call_ref_2((r).clone(), 5) }) == 25) {
        &(0);
    } else {
        ({
            __assert_fail_5(
                Ptr::<i8>::from_string_literal(b"call_ref(r, 5) == 25"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ call_deduced_3(FnPtr::<fn(i32) -> i32>::new(twice_1), 7) }) == 14) {
        &(0);
    } else {
        ({
            __assert_fail_5(
                Ptr::<i8>::from_string_literal(b"call_deduced(twice, 7) == 14"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ call_forwarded_4(FnPtr::<fn(i32) -> i32>::new(square_0), 6) }) == 36) {
        &(0);
    } else {
        ({
            __assert_fail_5(
                Ptr::<i8>::from_string_literal(b"call_forwarded(square, 6) == 36"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let h: Value<Holder> = Rc::new(RefCell::new(Holder {
        f: FnPtr::<fn(i32) -> i32>::new(twice_1),
    }));
    (if (({ HolderImpl::run(&h.as_pointer(), 8) }) == 16) {
        &(0);
    } else {
        ({
            __assert_fail_5(
                Ptr::<i8>::from_string_literal(b"h.run(8) == 16"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    return 0;
}
pub fn __assert_fail_5(
    mut __assertion: Ptr<i8>,
    mut __file: Ptr<i8>,
    mut __line: u32,
    mut __function: Ptr<i8>,
) {
    unimplemented!()
}
pub trait HolderImpl {
    fn run(&self, x: i32) -> i32;
}
impl HolderImpl for Ptr<Holder> {
    fn run(&self, mut x: i32) -> i32 {
        return ({ (*self).with(|__s| __s.f.clone()).call(x) });
    }
}
pub fn __cpp2rust_init_globals() {}
