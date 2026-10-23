extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn where_0(_a0: Ptr<()>) {}
pub fn test_1() {
    const _: () = assert!(true, "(BR::icase == std::regex_constants::icase)");;
    const _: () = assert!(true, "(BR::nosubs == std::regex_constants::nosubs)");;
    const _: () = assert!(true, "(BR::optimize == std::regex_constants::optimize)");;
    const _: () = assert!(true, "(BR::collate == std::regex_constants::collate)");;
    const _: () = assert!(true, "(BR::ECMAScript == std::regex_constants::ECMAScript)");;
    const _: () = assert!(true, "(BR::basic == std::regex_constants::basic)");;
    const _: () = assert!(true, "(BR::extended == std::regex_constants::extended)");;
    const _: () = assert!(true, "(BR::awk == std::regex_constants::awk)");;
    const _: () = assert!(true, "(BR::grep == std::regex_constants::grep)");;
    const _: () = assert!(true, "(BR::egrep == std::regex_constants::egrep)");;
    ({ where_0(icase_2.with(|v| v.as_pointer())) });
    ({ where_0(nosubs_3.with(|v| v.as_pointer())) });
    ({ where_0(optimize_4.with(|v| v.as_pointer())) });
    ({ where_0(collate_5.with(|v| v.as_pointer())) });
    ({ where_0(ECMAScript_6.with(|v| v.as_pointer())) });
    ({ where_0(basic_7.with(|v| v.as_pointer())) });
    ({ where_0(extended_8.with(|v| v.as_pointer())) });
    ({ where_0(awk_9.with(|v| v.as_pointer())) });
    ({ where_0(grep_10.with(|v| v.as_pointer())) });
    ({ where_0(egrep_11.with(|v| v.as_pointer())) });
}
pub fn test_12() {
    const _: () = assert!(true, "(BR::icase == std::regex_constants::icase)");;
    const _: () = assert!(true, "(BR::nosubs == std::regex_constants::nosubs)");;
    const _: () = assert!(true, "(BR::optimize == std::regex_constants::optimize)");;
    const _: () = assert!(true, "(BR::collate == std::regex_constants::collate)");;
    const _: () = assert!(true, "(BR::ECMAScript == std::regex_constants::ECMAScript)");;
    const _: () = assert!(true, "(BR::basic == std::regex_constants::basic)");;
    const _: () = assert!(true, "(BR::extended == std::regex_constants::extended)");;
    const _: () = assert!(true, "(BR::awk == std::regex_constants::awk)");;
    const _: () = assert!(true, "(BR::grep == std::regex_constants::grep)");;
    const _: () = assert!(true, "(BR::egrep == std::regex_constants::egrep)");;
    ({ where_0(icase_13.with(|v| v.as_pointer())) });
    ({ where_0(nosubs_14.with(|v| v.as_pointer())) });
    ({ where_0(optimize_15.with(|v| v.as_pointer())) });
    ({ where_0(collate_16.with(|v| v.as_pointer())) });
    ({ where_0(ECMAScript_17.with(|v| v.as_pointer())) });
    ({ where_0(basic_18.with(|v| v.as_pointer())) });
    ({ where_0(extended_19.with(|v| v.as_pointer())) });
    ({ where_0(awk_20.with(|v| v.as_pointer())) });
    ({ where_0(grep_21.with(|v| v.as_pointer())) });
    ({ where_0(egrep_22.with(|v| v.as_pointer())) });
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
    ({ test_1() });
    ({ test_12() });
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
