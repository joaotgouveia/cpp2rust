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
pub struct U {}
impl CppMove for U {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static u_0: Value<U> = Rc::new(RefCell::new(<U>::default()));
);
thread_local!(
    pub static cu_1: Ptr<U> = u_0.with(|v| v.as_pointer());
);
const _: () = assert!(
    true,
    "std::is_same_v<decltype(std::forward_like<T>(U{{}})), U&&>"
);
const _: () = assert!(
    true,
    "std::is_same_v<decltype(std::forward_like<T>(CU{{}})), CU&&>"
);
const _: () = assert!(
    true,
    "std::is_same_v<decltype(std::forward_like<T>(u)), U&&>"
);
const _: () = assert!(
    true,
    "std::is_same_v<decltype(std::forward_like<T>(cu)), CU&&>"
);
const _: () = assert!(
    true,
    "std::is_same_v<decltype(std::forward_like<T>(std::move(u))), U&&>"
);
const _: () = assert!(
    true,
    "std::is_same_v<decltype(std::forward_like<T>(std::move(cu))), CU&&>"
);
const _: () = assert!(
    true,
    "std::is_same_v<decltype(std::forward_like<CT>(U{{}})), CU&&>"
);
const _: () = assert!(
    true,
    "std::is_same_v<decltype(std::forward_like<CT>(CU{{}})), CU&&>"
);
const _: () = assert!(
    true,
    "std::is_same_v<decltype(std::forward_like<CT>(u)), CU&&>"
);
const _: () = assert!(
    true,
    "std::is_same_v<decltype(std::forward_like<CT>(cu)), CU&&>"
);
const _: () = assert!(
    true,
    "std::is_same_v<decltype(std::forward_like<CT>(std::move(u))), CU&&>"
);
const _: () = assert!(
    true,
    "std::is_same_v<decltype(std::forward_like<CT>(std::move(cu))), CU&&>"
);
const _: () = assert!(
    true,
    "std::is_same_v<decltype(std::forward_like<T&>(U{{}})), U&>"
);
const _: () = assert!(
    true,
    "std::is_same_v<decltype(std::forward_like<T&>(CU{{}})), CU&>"
);
const _: () = assert!(
    true,
    "std::is_same_v<decltype(std::forward_like<T&>(u)), U&>"
);
const _: () = assert!(
    true,
    "std::is_same_v<decltype(std::forward_like<T&>(cu)), CU&>"
);
const _: () = assert!(
    true,
    "std::is_same_v<decltype(std::forward_like<T&>(std::move(u))), U&>"
);
const _: () = assert!(
    true,
    "std::is_same_v<decltype(std::forward_like<T&>(std::move(cu))), CU&>"
);
const _: () = assert!(
    true,
    "std::is_same_v<decltype(std::forward_like<CT&>(U{{}})), CU&>"
);
const _: () = assert!(
    true,
    "std::is_same_v<decltype(std::forward_like<CT&>(CU{{}})), CU&>"
);
const _: () = assert!(
    true,
    "std::is_same_v<decltype(std::forward_like<CT&>(u)), CU&>"
);
const _: () = assert!(
    true,
    "std::is_same_v<decltype(std::forward_like<CT&>(cu)), CU&>"
);
const _: () = assert!(
    true,
    "std::is_same_v<decltype(std::forward_like<CT&>(std::move(u))), CU&>"
);
const _: () = assert!(
    true,
    "std::is_same_v<decltype(std::forward_like<CT&>(std::move(cu))), CU&>"
);
const _: () = assert!(
    true,
    "std::is_same_v<decltype(std::forward_like<T&&>(U{{}})), U&&>"
);
const _: () = assert!(
    true,
    "std::is_same_v<decltype(std::forward_like<T&&>(CU{{}})), CU&&>"
);
const _: () = assert!(
    true,
    "std::is_same_v<decltype(std::forward_like<T&&>(u)), U&&>"
);
const _: () = assert!(
    true,
    "std::is_same_v<decltype(std::forward_like<T&&>(cu)), CU&&>"
);
const _: () = assert!(
    true,
    "std::is_same_v<decltype(std::forward_like<T&&>(std::move(u))), U&&>"
);
const _: () = assert!(
    true,
    "std::is_same_v<decltype(std::forward_like<T&&>(std::move(cu))), CU&&>"
);
const _: () = assert!(
    true,
    "std::is_same_v<decltype(std::forward_like<CT&&>(U{{}})), CU&&>"
);
const _: () = assert!(
    true,
    "std::is_same_v<decltype(std::forward_like<CT&&>(CU{{}})), CU&&>"
);
const _: () = assert!(
    true,
    "std::is_same_v<decltype(std::forward_like<CT&&>(u)), CU&&>"
);
const _: () = assert!(
    true,
    "std::is_same_v<decltype(std::forward_like<CT&&>(cu)), CU&&>"
);
const _: () = assert!(
    true,
    "std::is_same_v<decltype(std::forward_like<CT&&>(std::move(u))), CU&&>"
);
const _: () = assert!(
    true,
    "std::is_same_v<decltype(std::forward_like<CT&&>(std::move(cu))), CU&&>"
);
const _: () = assert!(true, "noexcept(std::forward_like<T>(u))");
const _: () = assert!(
    true,
    "std::is_same_v<decltype(std::forward_like<U&>(u)), U&>"
);
const _: () = assert!(
    true,
    "std::is_same_v<decltype(std::forward_like<CU&>(cu)), CU&>"
);
const _: () = assert!(
    true,
    "std::is_same_v<decltype(std::forward_like<U&&>(std::move(u))), U&&>"
);
const _: () = assert!(
    true,
    "std::is_same_v<decltype(std::forward_like<CU&&>(std::move(cu))), CU&&>"
);
#[derive(Record, ByteRepr, FnPtrArg, Default)]
#[byte_size(1)]
pub struct NoCtorCopyMove {}
const _: () = assert!(
    true,
    "std::is_same_v<decltype(std::forward_like<CT&&>(std::declval<NoCtorCopyMove>())), const NoCtorCopyMove&&>"
);
const _: () = assert!(
    true,
    "std::is_same_v<decltype(std::forward_like<CT&>(std::declval<NoCtorCopyMove>())), const NoCtorCopyMove&>"
);
const _: () = assert!(
    true,
    "std::is_same_v<decltype(std::forward_like<T&&>(std::declval<NoCtorCopyMove>())), NoCtorCopyMove&&>"
);
const _: () = assert!(
    true,
    "std::is_same_v<decltype(std::forward_like<T&>(std::declval<NoCtorCopyMove>())), NoCtorCopyMove&>"
);
const _: () = assert!(
    true,
    "noexcept(std::forward_like<T>(std::declval<NoCtorCopyMove>()))"
);
pub fn test_2() -> bool {
    {
        let val: Value<i32> = Rc::new(RefCell::new(1729));
        let result: Ptr<i32> = val.as_pointer();
        const _: () = assert!(true, "std::is_same_v<decltype(result), const int&>");;
        ({ cpp2rust_assert_fail_3(({ (result).clone() } == { (val.as_pointer()) })) });
    }
    {
        let val: Value<i32> = Rc::new(RefCell::new(1729));
        let result: Ptr<i32> = val.as_pointer();
        const _: () = assert!(true, "std::is_same_v<decltype(result), int&>");;
        ({ cpp2rust_assert_fail_3(({ (result).clone() } == { (val.as_pointer()) })) });
    }
    {
        let val: Value<i32> = Rc::new(RefCell::new(1729));
        let result: Ptr<i32> = val.as_pointer();
        const _: () = assert!(true, "std::is_same_v<decltype(result), const int&&>");;
        ({ cpp2rust_assert_fail_3(({ (result).clone() } == { (val.as_pointer()) })) });
    }
    {
        let val: Value<i32> = Rc::new(RefCell::new(1729));
        let result: Ptr<i32> = val.as_pointer();
        const _: () = assert!(true, "std::is_same_v<decltype(result), int&&>");;
        ({ cpp2rust_assert_fail_3(({ (result).clone() } == { (val.as_pointer()) })) });
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
pub fn __cpp2rust_init_globals() {
    let _ = u_0.with(|_| ());
    let _ = cu_1.with(|_| ());
}
