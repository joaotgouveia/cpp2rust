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
pub fn swap_0(lhs: Ptr<ConstSwappable>, rhs: Ptr<ConstSwappable>) {
    unimplemented!();
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct ConstSwappable {
    #[offset(0)]
    pub i: (),
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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct SwapMayThrow_true_ {}
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
pub fn test_1() -> () {
    {
        let p1: Value<()> = Rc::new(RefCell::new({
            let __tmp_0: Value<ConstSwappable> = Rc::new(RefCell::new(ConstSwappable { i: 0_() }));
            let __tmp_1: Value<ConstSwappable> = Rc::new(RefCell::new(ConstSwappable { i: 1_() }));
            std_pair_const_ConstSwappable__const_ConstSwappable_::new({ __tmp_0.as_pointer() }, {
                __tmp_1.as_pointer()
            })
        }));
        let p2: Value<()> = Rc::new(RefCell::new({
            let __tmp_2: Value<ConstSwappable> = Rc::new(RefCell::new(ConstSwappable { i: 2_() }));
            let __tmp_3: Value<ConstSwappable> = Rc::new(RefCell::new(ConstSwappable { i: 3_() }));
            std_pair_const_ConstSwappable__const_ConstSwappable_::new({ __tmp_2.as_pointer() }, {
                __tmp_3.as_pointer()
            })
        }));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let i1: Value<()> = Rc::new(RefCell::new(0_()));
        let i2: Value<()> = Rc::new(RefCell::new(1_()));
        let i3: Value<()> = Rc::new(RefCell::new(2_()));
        let i4: Value<()> = Rc::new(RefCell::new(3_()));
        let p1: Value<()> = Rc::new(RefCell::new(std_pair_int_ref__int_ref_::new_1(
            { i1.as_pointer() },
            { i2.as_pointer() },
        )));
        let p2: Value<()> = Rc::new(RefCell::new(std_pair_int_ref__int_ref_::new_1(
            { i3.as_pointer() },
            { i4.as_pointer() },
        )));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
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
    ({ test_1() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
