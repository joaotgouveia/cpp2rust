extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let bits: Value<std_vector_bool__std_allocator_bool__> = Rc::new(RefCell::new({
        let __tmp_0: Value<bool> = Rc::new(RefCell::new(false));
        std_vector_bool__std_allocator_bool__::new_1({ 2_u64 }, { __tmp_0.as_pointer() }, None)
    }));
    let bit: Value<std__Bit_reference> = Rc::new(RefCell::new(((*bits.borrow())[0_u64]).clone()));
    let assigned: Ptr<std__Bit_reference> = ((*bit.borrow_mut()) = true);
    ({ cpp2rust_assert_fail_0(({ (*assigned.upgrade().deref())() })) });
    ({ cpp2rust_assert_fail_0(({ (*bits.borrow())[0_u64]() })) });
    let cleared: Ptr<std__Bit_reference> = ({ (*bit.borrow()).operator_assign_5(false) });
    ({ cpp2rust_assert_fail_0(!({ (*cleared.upgrade().deref())() })) });
    ({ cpp2rust_assert_fail_0(!({ (*bits.borrow())[0_u64]() })) });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
