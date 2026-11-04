extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct TakesTwoInts {
    #[offset(0)]
    pub a_: i32,
    #[offset(4)]
    pub b_: i32,
}
impl TakesTwoInts {
    pub fn new(mut a: i32, mut b: i32) -> Self {
        Self { a_: a, b_: b }
    }
}
impl CppMove for TakesTwoInts {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn test_0() -> bool {
    {
        let sv: Value<std_ranges_single_view_TakesTwoInts_> = Rc::new(RefCell::new({
            let __tmp_0: Value<i32> = Rc::new(RefCell::new(1));
            let __tmp_1: Value<i32> = Rc::new(RefCell::new(2));
            std_ranges_single_view_TakesTwoInts_::new_1(
                { (*in_place_1.with(Value::clone).borrow()).clone() },
                { __tmp_0.as_pointer() },
                { __tmp_1.as_pointer() },
            )
        }));
        assert!(({ (({ (*sv.borrow()).data_11() }).with(|__s| __s.a_) == 1) }));
        assert!(({ (({ (*sv.borrow()).data_11() }).with(|__s| __s.b_) == 2) }));
        assert!((({ std_ranges_single_view_TakesTwoInts_::size() }) == 1_usize));
    }
    {
        let sv: Value<std_ranges_single_view_TakesTwoInts_> = Rc::new(RefCell::new({
            let __tmp_2: Value<i32> = Rc::new(RefCell::new(1));
            let __tmp_3: Value<i32> = Rc::new(RefCell::new(2));
            std_ranges_single_view_TakesTwoInts_::new_1(
                { (*in_place_1.with(Value::clone).borrow()).clone() },
                { __tmp_2.as_pointer() },
                { __tmp_3.as_pointer() },
            )
        }));
        assert!(({ (({ (*sv.borrow()).data_12() }).with(|__s| __s.a_) == 1) }));
        assert!(({ (({ (*sv.borrow()).data_12() }).with(|__s| __s.b_) == 2) }));
        assert!((({ std_ranges_single_view_TakesTwoInts_::size() }) == 1_usize));
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> i32 {
    ({ test_0() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub fn __cpp2rust_init_globals() {}
