extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_container_iterators_0(c: ()) {
    let c: Value<()> = Rc::new(RefCell::new(c));
    let cc: Ptr<()> = c.as_pointer();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_container_iterators_1(c: ()) {
    let c: Value<()> = Rc::new(RefCell::new(c));
    let cc: Ptr<()> = c.as_pointer();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_container_iterators_2(c: ()) {
    let c: Value<()> = Rc::new(RefCell::new(c));
    let cc: Ptr<()> = c.as_pointer();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_container_iterators_3(c: ()) {
    let c: Value<()> = Rc::new(RefCell::new(c));
    let cc: Ptr<()> = c.as_pointer();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_container_iterators_4(c: ()) {
    let c: Value<()> = Rc::new(RefCell::new(c));
    let cc: Ptr<()> = c.as_pointer();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_valarray_iterators_5() {
    let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut p: Ptr<()> = unimplemented!();
    let mut q: Ptr<()> = unimplemented!();
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
    ({ test_container_iterators_0(unimplemented!()) });
    ({ test_container_iterators_1(unimplemented!()) });
    ({
        test_container_iterators_2(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { Ptr::<()>::from_string_literal(b"abc") },
                None,
            ),
        )
    });
    ({ test_container_iterators_3(unimplemented!()) });
    ({ test_container_iterators_4(unimplemented!()) });
    ({ test_valarray_iterators_5() });
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
