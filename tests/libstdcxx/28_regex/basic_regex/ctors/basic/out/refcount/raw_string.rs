extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct __gnu_test_regex_expected_fail {}
thread_local!(
    pub static regex_error_internal_0: Value<()> = Rc::new(RefCell::new(((-1_()) as ())));
);
thread_local!(
    pub static regex_error_codes_1: Value<Box<[Ptr<()>]>> = Rc::new(RefCell::new(Box::new([
        Ptr::<()>::from_string_literal(b"error_collate"),
        Ptr::<()>::from_string_literal(b"error_ctype"),
        Ptr::<()>::from_string_literal(b"error_escape"),
        Ptr::<()>::from_string_literal(b"error_backref"),
        Ptr::<()>::from_string_literal(b"error_brack"),
        Ptr::<()>::from_string_literal(b"error_paren"),
        Ptr::<()>::from_string_literal(b"error_brace"),
        Ptr::<()>::from_string_literal(b"error_badbrace"),
        Ptr::<()>::from_string_literal(b"error_range"),
        Ptr::<()>::from_string_literal(b"error_space"),
        Ptr::<()>::from_string_literal(b"error_badrepeat"),
        Ptr::<()>::from_string_literal(b"error_complexity"),
        Ptr::<()>::from_string_literal(b"error_stack"),
    ])));
);
pub fn show_regex_error_codes_2() {
    let mut tab: () = ('\t' as ());
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
pub fn regex_sanity_check_3(__res: Ptr<()>, __f: Option<()>, __error: Option<()>) {
    let mut __f: () = __f.unwrap_or_else(|| unimplemented!());
    let mut __error: () = __error.unwrap_or_else(|| regex_error_internal_0.with(|rc| *rc.borrow()));
    let reo: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut n: () = unimplemented!();
    unimplemented!();
}
pub fn test01_4() {
    let sre0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let sre1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let sre2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let re0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let re1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let re2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    ({ regex_sanity_check_3(sre0.as_pointer(), Some(unimplemented!()), None) });
    ({ regex_sanity_check_3(sre1.as_pointer(), None, None) });
    ({ regex_sanity_check_3(sre2.as_pointer(), None, None) });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    ({ test01_4() });
    return 0_();
}
pub fn __cpp2rust_init_globals() {
    let _ = regex_error_internal_0.with(|_| ());
    let _ = regex_error_codes_1.with(|_| ());
}
