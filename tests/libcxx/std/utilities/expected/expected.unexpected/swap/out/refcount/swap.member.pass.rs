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
pub struct NoexceptSwap {}
impl CppMove for NoexceptSwap {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct MayThrowSwap {}
impl CppMove for MayThrowSwap {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
const _: () = assert!(true, "MemberSwapNoexcept<std::unexpected<NoexceptSwap>>");
const _: () = assert!(
    !(false),
    "!MemberSwapNoexcept<std::unexpected<MayThrowSwap>>"
);
pub fn swap_0(x: Ptr<ADLSwap>, y: Ptr<ADLSwap>) {
    cpp_swap!(field_ptr!(x, i), field_ptr!(y, i));
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(i32, new, value)]
#[byte_size(4)]
pub struct ADLSwap {
    #[offset(0)]
    pub i: i32,
}
impl ADLSwap {
    pub fn new(mut ii: i32) -> Self {
        Self { i: ii }
    }
}
impl CppMove for ADLSwap {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn test_1() -> bool {
    {
        let unex1: Value<std_unexpected_int_> = Rc::new(RefCell::new({
            let __tmp_0: Value<i32> = Rc::new(RefCell::new(5));
            std_unexpected_int_::new({ __tmp_0.as_pointer() })
        }));
        let unex2: Value<std_unexpected_int_> = Rc::new(RefCell::new({
            let __tmp_1: Value<i32> = Rc::new(RefCell::new(6));
            std_unexpected_int_::new({ __tmp_1.as_pointer() })
        }));
        ({ (*unex1.borrow()).swap(unex2.as_pointer()) });
        assert!(({ ((({ (*unex1.borrow()).error_9() }).read()) == 6) }));
        assert!(({ ((({ (*unex2.borrow()).error_9() }).read()) == 5) }));
    }
    {
        let unex1: Value<std_unexpected_ADLSwap_> = Rc::new(RefCell::new({
            let __tmp_2: Value<i32> = Rc::new(RefCell::new(5));
            std_unexpected_ADLSwap_::new({ __tmp_2.as_pointer() })
        }));
        let unex2: Value<std_unexpected_ADLSwap_> = Rc::new(RefCell::new({
            let __tmp_3: Value<i32> = Rc::new(RefCell::new(6));
            std_unexpected_ADLSwap_::new({ __tmp_3.as_pointer() })
        }));
        ({ (*unex1.borrow()).swap(unex2.as_pointer()) });
        assert!(({ (({ (*unex1.borrow()).error_9() }).with(|__s| __s.i) == 6) }));
        assert!(({ (({ (*unex2.borrow()).error_9() }).with(|__s| __s.i) == 5) }));
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
    ({ test_1() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub fn __cpp2rust_init_globals() {}
