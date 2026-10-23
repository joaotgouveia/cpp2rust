extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_0() -> () {
    let a: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_()])));
    let b: Value<Box<[Ptr<()>]>> = Rc::new(RefCell::new(Box::new([
        ((a.as_pointer() as Ptr<()>).offset(2_())),
        ((a.as_pointer() as Ptr<()>).offset(0_())),
        ((a.as_pointer() as Ptr<()>).offset(1_())),
    ])));
    let c : Value<std_ranges_subrange_const_int_ptrconst_ptr__const_int_ptrconst_ptr__std_ranges_subrange_kind_sized_ > = Rc::new(RefCell::new(std_ranges_subrange_const_int_ptrconst_ptr__const_int_ptrconst_ptr__std_ranges_subrange_kind_sized_ :: new_1 ( { b .as_pointer()  } , )  )) ;
    unimplemented!();
    unimplemented!();
    let d: Value<std_ranges_subrange_int_ptrptr__int_ptrptr__std_ranges_subrange_kind_sized_> =
        Rc::new(RefCell::new(
            std_ranges_subrange_int_ptrptr__int_ptrptr__std_ranges_subrange_kind_sized_::new_1({
                b.as_pointer()
            }),
        ));
    unimplemented!();
    unimplemented!();
    let e : Value<std_ranges_subrange_const_int_ptrconst_ptr__const_int_ptrconst_ptr__std_ranges_subrange_kind_sized_ > = Rc::new(RefCell::new(std_ranges_subrange_const_int_ptrconst_ptr__const_int_ptrconst_ptr__std_ranges_subrange_kind_sized_ :: new_2 ( { d .as_pointer()  } , )  )) ;
    unimplemented!();
    unimplemented!();
    return true;
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
    ({ test_0() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
