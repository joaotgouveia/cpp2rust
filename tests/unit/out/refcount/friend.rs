extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn get_0(v: Ptr<V>) -> i32 {
    return v.with(|__s| __s.x);
}
pub fn operator_eq_1(a: Ptr<V>, b: Ptr<V>) -> bool {
    return ({ a.with(|__s| __s.x) } == { b.with(|__s| __s.x) });
}
pub fn scaled_2(v: Ptr<V>, mut k: i32) -> i32 {
    return ({ v.with(|__s| __s.x) } * { k });
}
pub fn scaled_3(v: Ptr<V>, mut k: f64) -> f64 {
    return ({ (v.with(|__s| __s.x) as f64) } * { k });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct V {
    #[offset(0)]
    pub x: i32,
}
impl std::cmp::PartialEq for V {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_1(
                Rc::new(RefCell::new(V { x: self.x.clone() })).as_pointer(),
                Rc::new(RefCell::new(V { x: other.x.clone() })).as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for V {}
pub fn get_5(w: Ptr<W_4>) -> i32 {
    return w.with(|__s| __s.x);
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct W_4 {
    #[offset(0)]
    pub x: i32,
}
pub fn get_7(w: Ptr<W_6>) -> i64 {
    return w.with(|__s| __s.x);
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct W_6 {
    #[offset(0)]
    pub x: i64,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct D {
    #[offset(0)]
    pub x: i32,
}
pub fn declared_then_defined_8(d: Ptr<D>) -> i32 {
    return (d.with(|__s| __s.x) + 1);
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let a: Value<V> = Rc::new(RefCell::new(V { x: 3 }));
    let b: Value<V> = Rc::new(RefCell::new(V { x: 3 }));
    let c: Value<V> = Rc::new(RefCell::new(V { x: 4 }));
    (if (({ get_0(a.as_pointer()) }) == 3) {
        &(0);
    } else {
        ({
            __assert_fail_9(
                Ptr::<i8>::from_string_literal(b"get(a) == 3"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ({
        let _a: Ptr<V> = a.as_pointer();
        operator_eq_1(_a, b.as_pointer())
    }) {
        &(0);
    } else {
        ({
            __assert_fail_9(
                Ptr::<i8>::from_string_literal(b"a == b"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if !({
        let _a: Ptr<V> = a.as_pointer();
        operator_eq_1(_a, c.as_pointer())
    }) {
        &(0);
    } else {
        ({
            __assert_fail_9(
                Ptr::<i8>::from_string_literal(b"!(a == c)"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ scaled_2(c.as_pointer(), 2) }) == 8) {
        &(0);
    } else {
        ({
            __assert_fail_9(
                Ptr::<i8>::from_string_literal(b"scaled(c, 2) == 8"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ scaled_3(c.as_pointer(), 1.5E+0) }) == 6.0E+0) {
        &(0);
    } else {
        ({
            __assert_fail_9(
                Ptr::<i8>::from_string_literal(b"scaled(c, 1.5) == 6.0"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let wi: Value<W_4> = Rc::new(RefCell::new(W_4 { x: 5 }));
    let wl: Value<W_6> = Rc::new(RefCell::new(W_6 { x: 6_i64 }));
    (if (({ get_5(wi.as_pointer()) }) == 5) {
        &(0);
    } else {
        ({
            __assert_fail_9(
                Ptr::<i8>::from_string_literal(b"get(wi) == 5"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ get_7(wl.as_pointer()) }) == 6_i64) {
        &(0);
    } else {
        ({
            __assert_fail_9(
                Ptr::<i8>::from_string_literal(b"get(wl) == 6"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let d: Value<D> = Rc::new(RefCell::new(D { x: 7 }));
    (if (({ declared_then_defined_8(d.as_pointer()) }) == 8) {
        &(0);
    } else {
        ({
            __assert_fail_9(
                Ptr::<i8>::from_string_literal(b"declared_then_defined(d) == 8"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    return 0;
}
pub fn __assert_fail_9(
    mut __assertion: Ptr<i8>,
    mut __file: Ptr<i8>,
    mut __line: u32,
    mut __function: Ptr<i8>,
) {
    unimplemented!()
}
pub fn __cpp2rust_init_globals() {}
