extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct S {
    #[offset(0)]
    pub a: i32,
}
impl CppMove for S {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static s_0: Value<Ptr<S>> = Rc::new(RefCell::new(Ptr::<S>::null()));
);
thread_local!(
    pub static file_1: Value<Ptr<_IO_FILE>> = Rc::new(RefCell::new(Ptr::<_IO_FILE>::null()));
);
thread_local!(
    pub static size_2: Value<u64> = Rc::new(RefCell::new(0_u64));
);
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    ({ cpp2rust_assert_fail_3((*s_0.with(Value::clone).borrow()).is_null()) });
    ({ cpp2rust_assert_fail_3((*file_1.with(Value::clone).borrow()).is_null()) });
    ({ cpp2rust_assert_fail_3((size_2.with(|rc| *rc.borrow()) == 0_u64)) });
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = s_0.with(|_| ());
    let _ = file_1.with(|_| ());
    let _ = size_2.with(|_| ());
}
