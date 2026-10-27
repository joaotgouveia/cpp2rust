extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn is_skipws_0(mut is: Ptr<()>) -> () {
    return (({ unimplemented!() } & { unimplemented!() }) != 0_());
}
pub fn is_skipws_1(mut is: Ptr<()>) -> () {
    return (({ unimplemented!() } & { unimplemented!() }) != 0_());
}
pub fn round_trip_2(mut p: Ptr<()>) {
    let ss: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut skippingws: () = ({ is_skipws_0((ss.as_pointer())) });
    let sv: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    let mut s: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn round_trip_ws_3(mut p: Ptr<()>) {
    let ss: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    let mut skippingws: () = ({ is_skipws_0((ss.as_pointer())) });
    let sv: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    let mut s: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn round_trip_d_4(mut p: Ptr<()>, mut delim: ()) {
    let ss: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let sv: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    let mut s: () = unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn round_trip_e_5(mut p: Ptr<()>, mut escape: ()) {
    let ss: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let sv: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    let mut s: () = unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn quote_6(mut p: Ptr<()>, delim: Option<()>, escape: Option<()>) -> () {
    let mut delim: () = delim.unwrap_or_else(|| ('\"' as ()));
    let mut escape: () = escape.unwrap_or_else(|| ('\\' as ()));
    let ss: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    let mut s: () = unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub fn unquote_7(mut p: Ptr<()>, delim: Option<()>, escape: Option<()>) -> () {
    let mut delim: () = delim.unwrap_or_else(|| ('\"' as ()));
    let mut escape: () = escape.unwrap_or_else(|| ('\\' as ()));
    let ss: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    let mut s: () = unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub fn round_trip_8(mut p: Ptr<()>) {
    let ss: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut skippingws: () = ({ is_skipws_1((ss.as_pointer())) });
    let sv: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    let mut s: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn round_trip_ws_9(mut p: Ptr<()>) {
    let ss: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    let mut skippingws: () = ({ is_skipws_1((ss.as_pointer())) });
    let sv: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    let mut s: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn round_trip_d_10(mut p: Ptr<()>, mut delim: ()) {
    let ss: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let sv: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    let mut s: () = unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn round_trip_e_11(mut p: Ptr<()>, mut escape: ()) {
    let ss: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let sv: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    let mut s: () = unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn quote_12(mut p: Ptr<()>, delim: Option<()>, escape: Option<()>) -> () {
    let mut delim: () = delim.unwrap_or_else(|| ('\"' as ()));
    let mut escape: () = escape.unwrap_or_else(|| ('\\' as ()));
    let ss: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let sv: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    let mut s: () = unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub fn unquote_13(mut p: Ptr<()>, delim: Option<()>, escape: Option<()>) -> () {
    let mut delim: () = delim.unwrap_or_else(|| ('\"' as ()));
    let mut escape: () = escape.unwrap_or_else(|| ('\\' as ()));
    let ss: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let sv: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    let mut s: () = unimplemented!();
    unimplemented!();
    return unimplemented!();
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
    ({ round_trip_2(Ptr::<()>::from_string_literal(b"")) });
    ({ round_trip_ws_3(Ptr::<()>::from_string_literal(b"")) });
    ({ round_trip_d_4(Ptr::<()>::from_string_literal(b""), ('q' as ())) });
    ({ round_trip_e_5(Ptr::<()>::from_string_literal(b""), ('q' as ())) });
    ({ round_trip_8(Ptr::<()>::from_string_literal(&[0 as ()])) });
    ({ round_trip_ws_9(Ptr::<()>::from_string_literal(&[0 as ()])) });
    ({ round_trip_d_10(Ptr::<()>::from_string_literal(&[0 as ()]), ('q' as ())) });
    ({ round_trip_e_11(Ptr::<()>::from_string_literal(&[0 as ()]), ('q' as ())) });
    ({ round_trip_2(Ptr::<()>::from_string_literal(b"Hi")) });
    ({ round_trip_ws_3(Ptr::<()>::from_string_literal(b"Hi")) });
    ({ round_trip_d_4(Ptr::<()>::from_string_literal(b"Hi"), ('!' as ())) });
    ({ round_trip_e_5(Ptr::<()>::from_string_literal(b"Hi"), ('!' as ())) });
    unimplemented!();
    unimplemented!();
    ({
        round_trip_8(Ptr::<()>::from_string_literal(&[
            72 as (), 105 as (), 0 as (),
        ]))
    });
    ({
        round_trip_ws_9(Ptr::<()>::from_string_literal(&[
            72 as (), 105 as (), 0 as (),
        ]))
    });
    ({
        round_trip_d_10(
            Ptr::<()>::from_string_literal(&[72 as (), 105 as (), 0 as ()]),
            ('!' as ()),
        )
    });
    ({
        round_trip_e_11(
            Ptr::<()>::from_string_literal(&[72 as (), 105 as (), 0 as ()]),
            ('!' as ()),
        )
    });
    unimplemented!();
    unimplemented!();
    ({ round_trip_2(Ptr::<()>::from_string_literal(b"Hi Mom")) });
    ({ round_trip_ws_3(Ptr::<()>::from_string_literal(b"Hi Mom")) });
    ({
        round_trip_8(Ptr::<()>::from_string_literal(&[
            72 as (), 105 as (), 32 as (), 77 as (), 111 as (), 109 as (), 0 as (),
        ]))
    });
    ({
        round_trip_ws_9(Ptr::<()>::from_string_literal(&[
            72 as (), 105 as (), 32 as (), 77 as (), 111 as (), 109 as (), 0 as (),
        ]))
    });
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
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
