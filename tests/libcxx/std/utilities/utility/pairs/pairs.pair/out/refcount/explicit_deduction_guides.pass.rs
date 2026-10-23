extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type anon_0 = ();
pub const anon_0_value: anon_0 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_pair_char_ptr__void__ptr______std_pair_char_ptr__void__ptr_____
{}
pub type anon_1 = ();
pub const anon_1_value: anon_1 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_pair_std_reference_wrapper_int___void__ptr______std_pair_std_reference_wrapper_int___void__ptr_____
{}
pub fn dummy_2() {}
pub fn test_decay_3() {
    let arr: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([unimplemented!()])));
    let pr: Value<()> = Rc::new(RefCell::new({
        let __tmp_0: Value<Ptr<()>> = Rc::new(RefCell::new((arr.as_pointer() as Ptr<()>)));
        let __tmp_1: Value<FnPtr<fn()>> = Rc::new(RefCell::new(FnPtr::<fn()>::new(dummy_2)));
        std_pair_char_ptr__void__ptr____::new({ __tmp_0.as_pointer() }, { __tmp_1.as_pointer() })
    }));
    const _: () = assert!(true, "");;
    unimplemented!();
}
pub fn test_unwrap_4() {
    let n: Value<()> = Rc::new(RefCell::new(0_()));
    let pr: Value<()> = Rc::new(RefCell::new({
        let __tmp_2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        std_pair_std_reference_wrapper_int___void__ptr____::new({ __tmp_2.as_pointer() }, {
            FnPtr::<fn()>::new(dummy_2)
        })
    }));
    const _: () = assert!(true, "");;
    const _: () = assert!(
        true,
        "!std::is_same_v<decltype(pr), decltype(std::make_pair(std::ref(n), dummy))>"
    );;
    unimplemented!();
    unimplemented!();
}
pub fn test_5() -> () {
    ({ test_decay_3() });
    {
        ({ test_unwrap_4() });
    }
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
    ({ test_5() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
