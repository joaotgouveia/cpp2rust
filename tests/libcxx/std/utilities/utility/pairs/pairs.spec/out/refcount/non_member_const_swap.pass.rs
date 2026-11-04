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
    ({
        let ___a: Ptr<i32> = field_ptr!(lhs, i);
        let ___b: Ptr<i32> = field_ptr!(rhs, i);
        swap_1(___a, ___b)
    });
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
    true,
    "!NonMemberSwapNoexcept<const std::pair<SwapMayThrow<true>, SwapMayThrow<false>>>"
);
const _: () = assert!(
    true,
    "!NonMemberSwapNoexcept<const std::pair<SwapMayThrow<false>, SwapMayThrow<true>>>"
);
const _: () = assert!(
    true,
    "!NonMemberSwapNoexcept<const std::pair<SwapMayThrow<true>, SwapMayThrow<true>>>"
);
pub fn test_2() -> bool {
    {
        let p1: Value<std_pair_const_ConstSwappable__const_ConstSwappable_> =
            Rc::new(RefCell::new({
                let __tmp_0: Value<ConstSwappable> = Rc::new(RefCell::new(ConstSwappable { i: 0 }));
                let __tmp_1: Value<ConstSwappable> = Rc::new(RefCell::new(ConstSwappable { i: 1 }));
                std_pair_const_ConstSwappable__const_ConstSwappable_::new_1(
                    { __tmp_0.as_pointer() },
                    { __tmp_1.as_pointer() },
                )
            }));
        let p2: Value<std_pair_const_ConstSwappable__const_ConstSwappable_> =
            Rc::new(RefCell::new({
                let __tmp_2: Value<ConstSwappable> = Rc::new(RefCell::new(ConstSwappable { i: 2 }));
                let __tmp_3: Value<ConstSwappable> = Rc::new(RefCell::new(ConstSwappable { i: 3 }));
                std_pair_const_ConstSwappable__const_ConstSwappable_::new_1(
                    { __tmp_2.as_pointer() },
                    { __tmp_3.as_pointer() },
                )
            }));
        ({ swap_3(p1.as_pointer(), p2.as_pointer()) });
        ({ cpp2rust_assert_fail_4(({ (*p1.borrow()).first.i } == 2)) });
        ({ cpp2rust_assert_fail_4(({ (*p1.borrow()).second.i } == 3)) });
        ({ cpp2rust_assert_fail_4(({ (*p2.borrow()).first.i } == 0)) });
        ({ cpp2rust_assert_fail_4(({ (*p2.borrow()).second.i } == 1)) });
    }
    {
        let i1: Value<i32> = Rc::new(RefCell::new(0));
        let i2: Value<i32> = Rc::new(RefCell::new(1));
        let i3: Value<i32> = Rc::new(RefCell::new(2));
        let i4: Value<i32> = Rc::new(RefCell::new(3));
        let p1: Value<std_pair_int_ref__int_ref_> = Rc::new(RefCell::new(
            std_pair_int_ref__int_ref_::new_1({ i1.as_pointer() }, { i2.as_pointer() }),
        ));
        let p2: Value<std_pair_int_ref__int_ref_> = Rc::new(RefCell::new(
            std_pair_int_ref__int_ref_::new_1({ i3.as_pointer() }, { i4.as_pointer() }),
        ));
        ({ swap_5(p1.as_pointer(), p2.as_pointer()) });
        ({ cpp2rust_assert_fail_4((({ (*p1.borrow()).first.clone() }.read()) == 2)) });
        ({ cpp2rust_assert_fail_4((({ (*p1.borrow()).second.clone() }.read()) == 3)) });
        ({ cpp2rust_assert_fail_4((({ (*p2.borrow()).first.clone() }.read()) == 0)) });
        ({ cpp2rust_assert_fail_4((({ (*p2.borrow()).second.clone() }.read()) == 1)) });
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
    ({ test_2() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub fn __cpp2rust_init_globals() {}
