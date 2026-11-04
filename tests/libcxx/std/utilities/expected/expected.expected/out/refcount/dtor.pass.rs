extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor, Default)]
#[byte_size(1)]
pub struct NonTrivial {}
impl CppMove for NonTrivial {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
const _: () = assert!(
    true,
    "std::is_trivially_destructible_v<std::expected<int, int>>"
);
const _: () = assert!(
    true,
    "!std::is_trivially_destructible_v<std::expected<NonTrivial, int>>"
);
const _: () = assert!(
    true,
    "!std::is_trivially_destructible_v<std::expected<int, NonTrivial>>"
);
const _: () = assert!(
    true,
    "!std::is_trivially_destructible_v<std::expected<NonTrivial, NonTrivial>>"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Destructor, Default)]
#[cpp_from(bool, new, mut_ptr)]
#[byte_size(8)]
pub struct TrackedDestroy {
    #[offset(0)]
    #[byte_size(8)]
    pub destroyed: Ptr<bool>,
}
impl TrackedDestroy {
    pub fn new(b: Ptr<bool>) -> Self {
        Self {
            destroyed: (b).clone(),
        }
    }
}
impl CppMove for TrackedDestroy {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn test_0() -> bool {
    {
        let valueDestroyed: Value<bool> = Rc::new(RefCell::new(false));
        {
            let e: Value<std_expected_TrackedDestroy__TrackedDestroy_> = Rc::new(RefCell::new(
                std_expected_TrackedDestroy__TrackedDestroy_::new_1(
                    { (*in_place_1.with(Value::clone).borrow()).clone() },
                    { valueDestroyed.as_pointer() },
                ),
            ));
        }
        assert!(({ (*valueDestroyed.borrow()) }));
    }
    {
        let errorDestroyed: Value<bool> = Rc::new(RefCell::new(false));
        {
            let e: Value<std_expected_TrackedDestroy__TrackedDestroy_> = Rc::new(RefCell::new(
                std_expected_TrackedDestroy__TrackedDestroy_::new_2(
                    { (*unexpect_2.with(Value::clone).borrow()).clone() },
                    { errorDestroyed.as_pointer() },
                ),
            ));
        }
        assert!(({ (*errorDestroyed.borrow()) }));
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
    let a: Value<std_expected_std_unique_ptr_int__std_default_delete_int____int_> =
        Rc::new(RefCell::new(Some(Rc::new(RefCell::new(42)))));
    ({ test_0() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub trait NonTrivialImpl {
    fn destructor(&self);
}
impl NonTrivialImpl for Ptr<NonTrivial> {
    fn destructor(&self) {}
}
pub trait TrackedDestroyImpl {
    fn destructor(&self);
}
impl TrackedDestroyImpl for Ptr<TrackedDestroy> {
    fn destructor(&self) {
        (*self).with(|__s| __s.destroyed.clone()).write(true);
    }
}
pub fn __cpp2rust_init_globals() {}
