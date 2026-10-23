extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn hash_identity_0(val: ()) -> () {
    let val: Value<()> = Rc::new(RefCell::new(val));
    return (*val.borrow());
}
pub fn hash_identity_1(val: ()) -> () {
    let val: Value<()> = Rc::new(RefCell::new(val));
    return (*val.borrow());
}
pub fn hash_neg_2(mut val: ()) -> () {
    return (({ std_numeric_limits_unsigned_long_::max() }) as ()).wrapping_sub(val);
}
pub fn hash_neg_3(mut val: ()) -> () {
    return (({ std_numeric_limits_bool_::max() }) - val);
}
pub fn hash_scale_4(mut val: ()) -> () {
    return ((val << 1_()) as ());
}
pub fn hash_scale_5(mut val: ()) -> () {
    return (val << 1_());
}
pub fn hash_even_6(mut val: ()) -> () {
    return if ((val & 1_()) != 0) { 1_() } else { 0_() };
}
pub fn hash_even_7(mut val: ()) -> () {
    return if ((val & 1_()) != 0) { 1_() } else { 0_() };
}
pub fn hash_same_8(mut _a0: ()) -> () {
    return 1_();
}
pub fn hash_same_9(mut _a0: ()) -> () {
    return 1_();
}
pub fn hash_same_10(mut _a0: Ptr<()>) -> () {
    return 1_();
}
pub fn hash_identity_11(mut val: Ptr<()>) -> () {
    return (val.read());
}
pub fn hash_neg_12(mut val: Ptr<()>) -> () {
    return ({ ({ std_numeric_limits_char_::max() }) } - { (val.read()) });
}
pub fn hash_scale_13(mut val: Ptr<()>) -> () {
    return ((val.read()) << 1_());
}
pub fn hash_even_14(mut val: Ptr<()>) -> () {
    return if (((val.read()) & 1_()) != 0) {
        1_()
    } else {
        0_()
    };
}
pub fn test_15(vals: Ptr<()>) {
    let c1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let c2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let c3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let c4: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let c5: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
    unimplemented!();
}
pub fn test_16(vals: Ptr<()>) {
    let c1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let c2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let c3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let c4: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let c5: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
    unimplemented!();
}
pub fn test_17(vals: Ptr<Ptr<()>>) {
    let c1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let c2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let c3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let c4: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let c5: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
    unimplemented!();
}
pub fn main() {
    let size: usize = ::std::env::args().map(|arg| arg.len() + 1).sum();
    let args: Value<Box<[i8]>> = Rc::new(RefCell::new(vec![0; size].into_boxed_slice()));
    let args_ptr: Ptr<i8> = args.as_pointer();
    let mut off = 0;
    let argv: Value<Box<[Ptr<i8>]>> = Rc::new(RefCell::new(
        ::std::env::args()
            .map(|arg| {
                let ptr = args_ptr.offset(off);
                ptr.write_c_bytes(arg.as_bytes());
                off += arg.len() + 1;
                ptr
            })
            .chain(::std::iter::once(Ptr::null()))
            .collect(),
    ));
    __cpp2rust_init_globals();
    ::std::process::exit(main_0(::std::env::args().len() as i32, argv.as_pointer()));
}
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    {
        let vals: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
            1_(),
            2_(),
            2_(),
            3_(),
            3_(),
            3_(),
            4_(),
            4_(),
            4_(),
            4_(),
            5_(),
            5_(),
            5_(),
            5_(),
            5_(),
            6_(),
            6_(),
            6_(),
            6_(),
            6_(),
            6_(),
            7_(),
            7_(),
            7_(),
            7_(),
            7_(),
            7_(),
            7_(),
            8_(),
            8_(),
            8_(),
            8_(),
            8_(),
            8_(),
            8_(),
            8_(),
            9_(),
            9_(),
            9_(),
            9_(),
            9_(),
            9_(),
            9_(),
            9_(),
            9_(),
            10_(),
        ])));
        ({ test_15(vals.as_pointer()) });
    }
    {
        let vals: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([true, false])));
        ({ test_16(vals.as_pointer()) });
    }
    {
        let vals: Value<Box<[Ptr<()>]>> = Rc::new(RefCell::new(Box::new([
            Ptr::<()>::from_string_literal(b"a").reinterpret_cast::<()>(),
            Ptr::<()>::from_string_literal(b"b").reinterpret_cast::<()>(),
            Ptr::<()>::from_string_literal(b"cde").reinterpret_cast::<()>(),
        ])));
        ({ test_17(vals.as_pointer()) });
    }
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
