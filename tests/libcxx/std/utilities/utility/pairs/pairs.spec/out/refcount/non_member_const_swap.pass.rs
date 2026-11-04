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
pub struct NonConstSwappable {}
impl CppMove for NonConstSwappable {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn swap_0(lhs: Ptr<ConstSwappable>, rhs: Ptr<ConstSwappable>) {
    cpp_swap!(field_ptr!(lhs, i), field_ptr!(rhs, i));
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct ConstSwappable {
    #[offset(0)]
    pub i: i32,
}
impl CppMove for ConstSwappable {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
const _: () = assert!(
    true,
    "std::is_swappable_v<const std::pair<ConstSwappable, ConstSwappable>>"
);
const _: () = assert!(
    true,
    "!std::is_swappable_v<const std::pair<NonConstSwappable, ConstSwappable>>"
);
const _: () = assert!(
    true,
    "!std::is_swappable_v<const std::pair<ConstSwappable, NonConstSwappable>>"
);
const _: () = assert!(
    true,
    "!std::is_swappable_v<const std::pair<NonConstSwappable, NonConstSwappable>>"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct SwapMayThrow_false_ {}
impl CppMove for SwapMayThrow_false_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct SwapMayThrow_true_ {}
impl CppMove for SwapMayThrow_true_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
const _: () = assert!(
    true,
    "NonMemberSwapNoexcept<const std::pair<SwapMayThrow<false>, SwapMayThrow<false>>>"
);
const _: () = assert!(
    !(false),
    "!NonMemberSwapNoexcept<const std::pair<SwapMayThrow<true>, SwapMayThrow<false>>>"
);
const _: () = assert!(
    !(false),
    "!NonMemberSwapNoexcept<const std::pair<SwapMayThrow<false>, SwapMayThrow<true>>>"
);
const _: () = assert!(
    !(false),
    "!NonMemberSwapNoexcept<const std::pair<SwapMayThrow<true>, SwapMayThrow<true>>>"
);
pub fn test_1() -> bool {
    {
        let p1: Value<(Value<ConstSwappable>, Value<ConstSwappable>)> = Rc::new(RefCell::new((
            Rc::new(RefCell::new(ConstSwappable { i: 0 })),
            Rc::new(RefCell::new(ConstSwappable { i: 1 })),
        )));
        let p2: Value<(Value<ConstSwappable>, Value<ConstSwappable>)> = Rc::new(RefCell::new((
            Rc::new(RefCell::new(ConstSwappable { i: 2 })),
            Rc::new(RefCell::new(ConstSwappable { i: 3 })),
        )));
        cpp_swap!(p1.as_pointer(), p2.as_pointer());
        assert!(({ ({ (*(*p1.borrow()).0.borrow()).i } == 2) }));
        assert!(({ ({ (*(*p1.borrow()).1.borrow()).i } == 3) }));
        assert!(({ ({ (*(*p2.borrow()).0.borrow()).i } == 0) }));
        assert!(({ ({ (*(*p2.borrow()).1.borrow()).i } == 1) }));
    }
    {
        let i1: Value<i32> = Rc::new(RefCell::new(0));
        let i2: Value<i32> = Rc::new(RefCell::new(1));
        let i3: Value<i32> = Rc::new(RefCell::new(2));
        let i4: Value<i32> = Rc::new(RefCell::new(3));
        let p1: Value<(Value<Ptr<i32>>, Value<Ptr<i32>>)> = Rc::new(RefCell::new(
            std_pair_int_ref__int_ref_::new_1({ i1.as_pointer() }, { i2.as_pointer() }),
        ));
        let p2: Value<(Value<Ptr<i32>>, Value<Ptr<i32>>)> = Rc::new(RefCell::new(
            std_pair_int_ref__int_ref_::new_1({ i3.as_pointer() }, { i4.as_pointer() }),
        ));
        cpp_swap!(p1.as_pointer(), p2.as_pointer());
        assert!(({ (((*p1.borrow()).0.borrow().clone().read()) == 2) }));
        assert!(({ (((*p1.borrow()).1.borrow().clone().read()) == 3) }));
        assert!(({ (((*p2.borrow()).0.borrow().clone().read()) == 0) }));
        assert!(({ (((*p2.borrow()).1.borrow().clone().read()) == 1) }));
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
