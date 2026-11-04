extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_0() -> bool {
    assert!(({ !({ is_eq_1(({ (*libcc2rs::strong_ordering_less().upgrade().deref())() }),) }) }));
    assert!(({ ({ is_eq_1(({ (*libcc2rs::strong_ordering_equal().upgrade().deref())() }),) }) }));
    assert!(
        ({ !({ is_eq_1(({ (*libcc2rs::strong_ordering_greater().upgrade().deref())() }),) }) })
    );
    assert!(({ !({ is_eq_1(({ (*less_2.with(Value::clone).borrow())() }),) }) }));
    assert!(({ ({ is_eq_1(({ (*equivalent_3.with(Value::clone).borrow())() }),) }) }));
    assert!(({ !({ is_eq_1(({ (*greater_4.with(Value::clone).borrow())() }),) }) }));
    assert!(({ !({ is_eq_1((*less_5.with(Value::clone).borrow()).clone(),) }) }));
    assert!(({ ({ is_eq_1((*equivalent_6.with(Value::clone).borrow()).clone(),) }) }));
    assert!(({ !({ is_eq_1((*greater_7.with(Value::clone).borrow()).clone(),) }) }));
    assert!(({ !({ is_eq_1((*unordered_8.with(Value::clone).borrow()).clone(),) }) }));
    assert!(({ ({ is_neq_9(({ (*libcc2rs::strong_ordering_less().upgrade().deref())() }),) }) }));
    assert!(({ !({ is_neq_9(({ (*libcc2rs::strong_ordering_equal().upgrade().deref())() }),) }) }));
    assert!(
        ({ ({ is_neq_9(({ (*libcc2rs::strong_ordering_greater().upgrade().deref())() }),) }) })
    );
    assert!(({ ({ is_neq_9(({ (*less_2.with(Value::clone).borrow())() }),) }) }));
    assert!(({ !({ is_neq_9(({ (*equivalent_3.with(Value::clone).borrow())() }),) }) }));
    assert!(({ ({ is_neq_9(({ (*greater_4.with(Value::clone).borrow())() }),) }) }));
    assert!(({ ({ is_neq_9((*less_5.with(Value::clone).borrow()).clone(),) }) }));
    assert!(({ !({ is_neq_9((*equivalent_6.with(Value::clone).borrow()).clone(),) }) }));
    assert!(({ ({ is_neq_9((*greater_7.with(Value::clone).borrow()).clone(),) }) }));
    assert!(({ ({ is_neq_9((*unordered_8.with(Value::clone).borrow()).clone(),) }) }));
    assert!(({ ({ is_lt_10(({ (*libcc2rs::strong_ordering_less().upgrade().deref())() }),) }) }));
    assert!(({ !({ is_lt_10(({ (*libcc2rs::strong_ordering_equal().upgrade().deref())() }),) }) }));
    assert!(
        ({ !({ is_lt_10(({ (*libcc2rs::strong_ordering_greater().upgrade().deref())() }),) }) })
    );
    assert!(({ ({ is_lt_10(({ (*less_2.with(Value::clone).borrow())() }),) }) }));
    assert!(({ !({ is_lt_10(({ (*equivalent_3.with(Value::clone).borrow())() }),) }) }));
    assert!(({ !({ is_lt_10(({ (*greater_4.with(Value::clone).borrow())() }),) }) }));
    assert!(({ ({ is_lt_10((*less_5.with(Value::clone).borrow()).clone(),) }) }));
    assert!(({ !({ is_lt_10((*equivalent_6.with(Value::clone).borrow()).clone(),) }) }));
    assert!(({ !({ is_lt_10((*greater_7.with(Value::clone).borrow()).clone(),) }) }));
    assert!(({ !({ is_lt_10((*unordered_8.with(Value::clone).borrow()).clone(),) }) }));
    assert!(({ ({ is_lteq_11(({ (*libcc2rs::strong_ordering_less().upgrade().deref())() }),) }) }));
    assert!(
        ({ ({ is_lteq_11(({ (*libcc2rs::strong_ordering_equal().upgrade().deref())() }),) }) })
    );
    assert!(
        ({ !({ is_lteq_11(({ (*libcc2rs::strong_ordering_greater().upgrade().deref())() }),) }) })
    );
    assert!(({ ({ is_lteq_11(({ (*less_2.with(Value::clone).borrow())() }),) }) }));
    assert!(({ ({ is_lteq_11(({ (*equivalent_3.with(Value::clone).borrow())() }),) }) }));
    assert!(({ !({ is_lteq_11(({ (*greater_4.with(Value::clone).borrow())() }),) }) }));
    assert!(({ ({ is_lteq_11((*less_5.with(Value::clone).borrow()).clone(),) }) }));
    assert!(({ ({ is_lteq_11((*equivalent_6.with(Value::clone).borrow()).clone(),) }) }));
    assert!(({ !({ is_lteq_11((*greater_7.with(Value::clone).borrow()).clone(),) }) }));
    assert!(({ !({ is_lteq_11((*unordered_8.with(Value::clone).borrow()).clone(),) }) }));
    assert!(({ !({ is_gt_12(({ (*libcc2rs::strong_ordering_less().upgrade().deref())() }),) }) }));
    assert!(({ !({ is_gt_12(({ (*libcc2rs::strong_ordering_equal().upgrade().deref())() }),) }) }));
    assert!(
        ({ ({ is_gt_12(({ (*libcc2rs::strong_ordering_greater().upgrade().deref())() }),) }) })
    );
    assert!(({ !({ is_gt_12(({ (*less_2.with(Value::clone).borrow())() }),) }) }));
    assert!(({ !({ is_gt_12(({ (*equivalent_3.with(Value::clone).borrow())() }),) }) }));
    assert!(({ ({ is_gt_12(({ (*greater_4.with(Value::clone).borrow())() }),) }) }));
    assert!(({ !({ is_gt_12((*less_5.with(Value::clone).borrow()).clone(),) }) }));
    assert!(({ !({ is_gt_12((*equivalent_6.with(Value::clone).borrow()).clone(),) }) }));
    assert!(({ ({ is_gt_12((*greater_7.with(Value::clone).borrow()).clone(),) }) }));
    assert!(({ !({ is_gt_12((*unordered_8.with(Value::clone).borrow()).clone(),) }) }));
    assert!(
        ({ !({ is_gteq_13(({ (*libcc2rs::strong_ordering_less().upgrade().deref())() }),) }) })
    );
    assert!(
        ({ ({ is_gteq_13(({ (*libcc2rs::strong_ordering_equal().upgrade().deref())() }),) }) })
    );
    assert!(
        ({ ({ is_gteq_13(({ (*libcc2rs::strong_ordering_greater().upgrade().deref())() }),) }) })
    );
    assert!(({ !({ is_gteq_13(({ (*less_2.with(Value::clone).borrow())() }),) }) }));
    assert!(({ ({ is_gteq_13(({ (*equivalent_3.with(Value::clone).borrow())() }),) }) }));
    assert!(({ ({ is_gteq_13(({ (*greater_4.with(Value::clone).borrow())() }),) }) }));
    assert!(({ !({ is_gteq_13((*less_5.with(Value::clone).borrow()).clone(),) }) }));
    assert!(({ ({ is_gteq_13((*equivalent_6.with(Value::clone).borrow()).clone(),) }) }));
    assert!(({ ({ is_gteq_13((*greater_7.with(Value::clone).borrow()).clone(),) }) }));
    assert!(({ !({ is_gteq_13((*unordered_8.with(Value::clone).borrow()).clone(),) }) }));
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
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
