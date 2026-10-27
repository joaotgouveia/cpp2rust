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
    ({ where_0(unimplemented!()) });
    ({ where_0(unimplemented!()) });
    ({ where_0(unimplemented!()) });
    ({ where_0(unimplemented!()) });
    ({ where_0(unimplemented!()) });
    ({ where_0(unimplemented!()) });
    ({ where_0(unimplemented!()) });
    ({ where_0(unimplemented!()) });
    ({ where_0(unimplemented!()) });
    ({ where_0(unimplemented!()) });
}
pub fn test_2() {
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
    ({ where_0(unimplemented!()) });
    ({ where_0(unimplemented!()) });
    ({ where_0(unimplemented!()) });
    ({ where_0(unimplemented!()) });
    ({ where_0(unimplemented!()) });
    ({ where_0(unimplemented!()) });
    ({ where_0(unimplemented!()) });
    ({ where_0(unimplemented!()) });
    ({ where_0(unimplemented!()) });
    ({ where_0(unimplemented!()) });
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
    ({ test_2() });
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
