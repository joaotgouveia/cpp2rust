extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn extract_0(t: Ptr<()>, _a1: ()) -> () {
    let _a1: Value<()> = Rc::new(RefCell::new(_a1));
    return unimplemented!();
}
pub fn extract_1(t: Ptr<()>, _a1: ()) -> () {
    let _a1: Value<()> = Rc::new(RefCell::new(_a1));
    return unimplemented!();
}
pub fn extract_2(t: Ptr<()>, _a1: ()) -> () {
    let _a1: Value<()> = Rc::new(RefCell::new(_a1));
    return unimplemented!();
}
pub fn extract_3(t: Ptr<()>, _a1: ()) -> () {
    let _a1: Value<()> = Rc::new(RefCell::new(_a1));
    return unimplemented!();
}
pub fn extract_4(t: Ptr<()>, _a1: ()) -> () {
    let _a1: Value<()> = Rc::new(RefCell::new(_a1));
    return unimplemented!();
}
pub fn extract_5(t: Ptr<()>, _a1: ()) -> () {
    let _a1: Value<()> = Rc::new(RefCell::new(_a1));
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
    const _: () = assert!(true, "std::is_same<int3::value_type, int>::value");;
    const _: () = assert!(true, "int3::size () == 3");;
    const _: () = assert!(true, "std::is_same<size7::value_type, std::size_t>::value");;
    const _: () = assert!(true, "size7::size () == 7");;
    const _: () = assert!(true, "std::is_same<size4::value_type, std::size_t>::value");;
    const _: () = assert!(true, "size4::size () == 4");;
    const _: () = assert!(true, "std::is_same<size2::value_type, std::size_t>::value");;
    const _: () = assert!(true, "size2::size () == 2");;
    const _: () = assert!(true, "std::is_same<intmix::value_type, int>::value");;
    const _: () = assert!(true, "intmix::size () == 4");;
    const _: () = assert!(
        true,
        "std::is_same<sizemix::value_type, std::size_t>::value"
    );;
    const _: () = assert!(true, "sizemix::size () == 5");;
    let tup: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let t3: Value<()> = Rc::new(RefCell::new(
        ({ extract_0(tup.as_pointer(), unimplemented!()) }),
    ));
    const _: () = assert!(
        true,
        "std::tuple_size<decltype(t3)>::value == int3::size ()"
    );;
    unimplemented!();
    let t7: Value<()> = Rc::new(RefCell::new(
        ({ extract_1(tup.as_pointer(), unimplemented!()) }),
    ));
    const _: () = assert!(
        true,
        "std::tuple_size<decltype(t7)>::value == size7::size ()"
    );;
    unimplemented!();
    let t4: Value<()> = Rc::new(RefCell::new(
        ({ extract_2(tup.as_pointer(), unimplemented!()) }),
    ));
    const _: () = assert!(
        true,
        "std::tuple_size<decltype(t4)>::value == size4::size ()"
    );;
    unimplemented!();
    let t2: Value<()> = Rc::new(RefCell::new(
        ({ extract_3(tup.as_pointer(), unimplemented!()) }),
    ));
    const _: () = assert!(
        true,
        "std::tuple_size<decltype(t2)>::value == size2::size ()"
    );;
    unimplemented!();
    let tintmix: Value<()> = Rc::new(RefCell::new(
        ({ extract_4(tup.as_pointer(), unimplemented!()) }),
    ));
    const _: () = assert!(
        true,
        "std::tuple_size<decltype(tintmix)>::value == intmix::size ()"
    );;
    unimplemented!();
    let tsizemix: Value<()> = Rc::new(RefCell::new(
        ({ extract_5(tup.as_pointer(), unimplemented!()) }),
    ));
    const _: () = assert!(
        true,
        "std::tuple_size<decltype(tsizemix)>::value == sizemix::size ()"
    );;
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
