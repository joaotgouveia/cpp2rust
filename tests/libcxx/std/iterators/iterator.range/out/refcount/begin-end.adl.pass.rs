extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn begin_0(_a0: Ptr<ArrayHijacker>) -> i32 {
    return 42;
}
pub fn end_1(_a0: Ptr<ArrayHijacker>) -> i32 {
    return 42;
}
pub fn begin_2(_a0: Ptr<ArrayHijacker>) -> i32 {
    return 42;
}
pub fn end_3(_a0: Ptr<ArrayHijacker>) -> i32 {
    return 42;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ArrayHijacker {}
impl CppMove for ArrayHijacker {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn begin_4(_a0: Ptr<ContainerHijacker>) -> i32 {
    return 42;
}
pub fn end_5(_a0: Ptr<ContainerHijacker>) -> i32 {
    return 42;
}
pub fn begin_6(_a0: Ptr<ContainerHijacker>) -> i32 {
    return 42;
}
pub fn end_7(_a0: Ptr<ContainerHijacker>) -> i32 {
    return 42;
}
pub fn cbegin_8(_a0: Ptr<ContainerHijacker>) -> i32 {
    return 42;
}
pub fn cend_9(_a0: Ptr<ContainerHijacker>) -> i32 {
    return 42;
}
pub fn cbegin_10(_a0: Ptr<ContainerHijacker>) -> i32 {
    return 42;
}
pub fn cend_11(_a0: Ptr<ContainerHijacker>) -> i32 {
    return 42;
}
pub fn rbegin_12(_a0: Ptr<ContainerHijacker>) -> i32 {
    return 42;
}
pub fn rend_13(_a0: Ptr<ContainerHijacker>) -> i32 {
    return 42;
}
pub fn rbegin_14(_a0: Ptr<ContainerHijacker>) -> i32 {
    return 42;
}
pub fn rend_15(_a0: Ptr<ContainerHijacker>) -> i32 {
    return 42;
}
pub fn crbegin_16(_a0: Ptr<ContainerHijacker>) -> i32 {
    return 42;
}
pub fn crend_17(_a0: Ptr<ContainerHijacker>) -> i32 {
    return 42;
}
pub fn crbegin_18(_a0: Ptr<ContainerHijacker>) -> i32 {
    return 42;
}
pub fn crend_19(_a0: Ptr<ContainerHijacker>) -> i32 {
    return 42;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct ContainerHijacker {
    #[offset(0)]
    #[byte_size(8)]
    pub a_: Ptr<i32>,
}
impl CppMove for ContainerHijacker {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn test_20() -> bool {
    {
        let a: Value<Box<[ArrayHijacker]>> = Rc::new(RefCell::new(Box::new([
            <ArrayHijacker>::default(),
            <ArrayHijacker>::default(),
            <ArrayHijacker>::default(),
        ])));
        ({ cpp2rust_assert_fail_21((({ begin_0(a.as_pointer()) }) == 42)) });
        ({ cpp2rust_assert_fail_21((({ end_1(a.as_pointer()) }) == 42)) });
        ({
            cpp2rust_assert_fail_21(
                (({ begin_22(a.as_pointer()) }) == (a.as_pointer() as Ptr<ArrayHijacker>)),
            )
        });
        ({
            cpp2rust_assert_fail_21(
                (({ end_23(a.as_pointer()) })
                    == (a.as_pointer() as Ptr<ArrayHijacker>).offset((3) as isize)),
            )
        });
        ({
            cpp2rust_assert_fail_21(
                (({ cbegin_24(a.as_pointer()) }) == (a.as_pointer() as Ptr<ArrayHijacker>)),
            )
        });
        ({
            cpp2rust_assert_fail_21(
                (({ cend_25(a.as_pointer()) })
                    == (a.as_pointer() as Ptr<ArrayHijacker>).offset((3) as isize)),
            )
        });
        ({
            cpp2rust_assert_fail_21(
                (({ ({ rbegin_26(a.as_pointer()) }).base() })
                    == (a.as_pointer() as Ptr<ArrayHijacker>).offset((3) as isize)),
            )
        });
        ({
            cpp2rust_assert_fail_21(
                (({ ({ rend_27(a.as_pointer()) }).base() })
                    == (a.as_pointer() as Ptr<ArrayHijacker>)),
            )
        });
        ({
            cpp2rust_assert_fail_21(
                (({ ({ crbegin_28(a.as_pointer()) }).base() })
                    == (a.as_pointer() as Ptr<ArrayHijacker>).offset((3) as isize)),
            )
        });
        ({
            cpp2rust_assert_fail_21(
                (({ ({ crend_29(a.as_pointer()) }).base() })
                    == (a.as_pointer() as Ptr<ArrayHijacker>)),
            )
        });
    }
    {
        let a: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0_i32, 0_i32, 0_i32])));
        let c: Value<ContainerHijacker> = Rc::new(RefCell::new(ContainerHijacker {
            a_: (a.as_pointer() as Ptr<i32>),
        }));
        ({ cpp2rust_assert_fail_21((({ begin_4(c.as_pointer()) }) == 42)) });
        ({ cpp2rust_assert_fail_21((({ end_5(c.as_pointer()) }) == 42)) });
        ({
            cpp2rust_assert_fail_21(
                (({ begin_30(c.as_pointer()) }) == (a.as_pointer() as Ptr<i32>)),
            )
        });
        ({
            cpp2rust_assert_fail_21(
                (({ end_31(c.as_pointer()) }) == (a.as_pointer() as Ptr<i32>).offset((3) as isize)),
            )
        });
        ({
            cpp2rust_assert_fail_21(
                (({ cbegin_32(c.as_pointer()) }) == (a.as_pointer() as Ptr<i32>)),
            )
        });
        ({
            cpp2rust_assert_fail_21(
                (({ cend_33(c.as_pointer()) })
                    == (a.as_pointer() as Ptr<i32>).offset((3) as isize)),
            )
        });
        ({
            cpp2rust_assert_fail_21(
                (({ rbegin_34(c.as_pointer()) }) == (a.as_pointer() as Ptr<i32>)),
            )
        });
        ({
            cpp2rust_assert_fail_21(
                (({ rend_35(c.as_pointer()) })
                    == (a.as_pointer() as Ptr<i32>).offset((3) as isize)),
            )
        });
        ({
            cpp2rust_assert_fail_21(
                (({ crbegin_36(c.as_pointer()) }) == (a.as_pointer() as Ptr<i32>)),
            )
        });
        ({
            cpp2rust_assert_fail_21(
                (({ crend_37(c.as_pointer()) })
                    == (a.as_pointer() as Ptr<i32>).offset((3) as isize)),
            )
        });
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
    ({ test_20() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub trait ContainerHijackerImpl {
    fn begin(&self) -> Ptr<i32>;
    fn end(&self) -> Ptr<i32>;
    fn rbegin(&self) -> Ptr<i32>;
    fn rend(&self) -> Ptr<i32>;
}
impl ContainerHijackerImpl for Ptr<ContainerHijacker> {
    fn begin(&self) -> Ptr<i32> {
        return (*self).with(|__s| __s.a_.clone());
    }
    fn end(&self) -> Ptr<i32> {
        return (*self).with(|__s| __s.a_.clone()).offset((3) as isize);
    }
    fn rbegin(&self) -> Ptr<i32> {
        return (*self).with(|__s| __s.a_.clone());
    }
    fn rend(&self) -> Ptr<i32> {
        return (*self).with(|__s| __s.a_.clone()).offset((3) as isize);
    }
}
pub fn __cpp2rust_init_globals() {}
