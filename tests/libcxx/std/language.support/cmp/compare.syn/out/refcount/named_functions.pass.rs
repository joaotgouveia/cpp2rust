extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_0() -> bool {
    ({ cpp2rust_assert_fail_1(!({ is_eq_2(({ (*less_3.with(Value::clone).borrow())() })) })) });
    ({ cpp2rust_assert_fail_1(({ is_eq_2(({ (*equal_4.with(Value::clone).borrow())() })) })) });
    ({ cpp2rust_assert_fail_1(!({ is_eq_2(({ (*greater_5.with(Value::clone).borrow())() })) })) });
    ({ cpp2rust_assert_fail_1(!({ is_eq_2(({ (*less_6.with(Value::clone).borrow())() })) })) });
    ({
        cpp2rust_assert_fail_1(({ is_eq_2(({ (*equivalent_7.with(Value::clone).borrow())() })) }))
    });
    ({ cpp2rust_assert_fail_1(!({ is_eq_2(({ (*greater_8.with(Value::clone).borrow())() })) })) });
    ({ cpp2rust_assert_fail_1(!({ is_eq_2((*less_9.with(Value::clone).borrow()).clone()) })) });
    ({
        cpp2rust_assert_fail_1(({ is_eq_2((*equivalent_10.with(Value::clone).borrow()).clone()) }))
    });
    ({ cpp2rust_assert_fail_1(!({ is_eq_2((*greater_11.with(Value::clone).borrow()).clone()) })) });
    ({
        cpp2rust_assert_fail_1(!({ is_eq_2((*unordered_12.with(Value::clone).borrow()).clone()) }))
    });
    ({ cpp2rust_assert_fail_1(({ is_neq_13(({ (*less_3.with(Value::clone).borrow())() })) })) });
    ({ cpp2rust_assert_fail_1(!({ is_neq_13(({ (*equal_4.with(Value::clone).borrow())() })) })) });
    ({ cpp2rust_assert_fail_1(({ is_neq_13(({ (*greater_5.with(Value::clone).borrow())() })) })) });
    ({ cpp2rust_assert_fail_1(({ is_neq_13(({ (*less_6.with(Value::clone).borrow())() })) })) });
    ({
        cpp2rust_assert_fail_1(
            !({ is_neq_13(({ (*equivalent_7.with(Value::clone).borrow())() })) }),
        )
    });
    ({ cpp2rust_assert_fail_1(({ is_neq_13(({ (*greater_8.with(Value::clone).borrow())() })) })) });
    ({ cpp2rust_assert_fail_1(({ is_neq_13((*less_9.with(Value::clone).borrow()).clone()) })) });
    ({
        cpp2rust_assert_fail_1(
            !({ is_neq_13((*equivalent_10.with(Value::clone).borrow()).clone()) }),
        )
    });
    ({
        cpp2rust_assert_fail_1(({ is_neq_13((*greater_11.with(Value::clone).borrow()).clone()) }))
    });
    ({
        cpp2rust_assert_fail_1(({ is_neq_13((*unordered_12.with(Value::clone).borrow()).clone()) }))
    });
    ({ cpp2rust_assert_fail_1(({ is_lt_14(({ (*less_3.with(Value::clone).borrow())() })) })) });
    ({ cpp2rust_assert_fail_1(!({ is_lt_14(({ (*equal_4.with(Value::clone).borrow())() })) })) });
    ({ cpp2rust_assert_fail_1(!({ is_lt_14(({ (*greater_5.with(Value::clone).borrow())() })) })) });
    ({ cpp2rust_assert_fail_1(({ is_lt_14(({ (*less_6.with(Value::clone).borrow())() })) })) });
    ({
        cpp2rust_assert_fail_1(!({ is_lt_14(({ (*equivalent_7.with(Value::clone).borrow())() })) }))
    });
    ({ cpp2rust_assert_fail_1(!({ is_lt_14(({ (*greater_8.with(Value::clone).borrow())() })) })) });
    ({ cpp2rust_assert_fail_1(({ is_lt_14((*less_9.with(Value::clone).borrow()).clone()) })) });
    ({
        cpp2rust_assert_fail_1(
            !({ is_lt_14((*equivalent_10.with(Value::clone).borrow()).clone()) }),
        )
    });
    ({
        cpp2rust_assert_fail_1(!({ is_lt_14((*greater_11.with(Value::clone).borrow()).clone()) }))
    });
    ({
        cpp2rust_assert_fail_1(!({ is_lt_14((*unordered_12.with(Value::clone).borrow()).clone()) }))
    });
    ({ cpp2rust_assert_fail_1(({ is_lteq_15(({ (*less_3.with(Value::clone).borrow())() })) })) });
    ({ cpp2rust_assert_fail_1(({ is_lteq_15(({ (*equal_4.with(Value::clone).borrow())() })) })) });
    ({
        cpp2rust_assert_fail_1(!({ is_lteq_15(({ (*greater_5.with(Value::clone).borrow())() })) }))
    });
    ({ cpp2rust_assert_fail_1(({ is_lteq_15(({ (*less_6.with(Value::clone).borrow())() })) })) });
    ({
        cpp2rust_assert_fail_1(
            ({ is_lteq_15(({ (*equivalent_7.with(Value::clone).borrow())() })) }),
        )
    });
    ({
        cpp2rust_assert_fail_1(!({ is_lteq_15(({ (*greater_8.with(Value::clone).borrow())() })) }))
    });
    ({ cpp2rust_assert_fail_1(({ is_lteq_15((*less_9.with(Value::clone).borrow()).clone()) })) });
    ({
        cpp2rust_assert_fail_1(
            ({ is_lteq_15((*equivalent_10.with(Value::clone).borrow()).clone()) }),
        )
    });
    ({
        cpp2rust_assert_fail_1(!({ is_lteq_15((*greater_11.with(Value::clone).borrow()).clone()) }))
    });
    ({
        cpp2rust_assert_fail_1(
            !({ is_lteq_15((*unordered_12.with(Value::clone).borrow()).clone()) }),
        )
    });
    ({ cpp2rust_assert_fail_1(!({ is_gt_16(({ (*less_3.with(Value::clone).borrow())() })) })) });
    ({ cpp2rust_assert_fail_1(!({ is_gt_16(({ (*equal_4.with(Value::clone).borrow())() })) })) });
    ({ cpp2rust_assert_fail_1(({ is_gt_16(({ (*greater_5.with(Value::clone).borrow())() })) })) });
    ({ cpp2rust_assert_fail_1(!({ is_gt_16(({ (*less_6.with(Value::clone).borrow())() })) })) });
    ({
        cpp2rust_assert_fail_1(!({ is_gt_16(({ (*equivalent_7.with(Value::clone).borrow())() })) }))
    });
    ({ cpp2rust_assert_fail_1(({ is_gt_16(({ (*greater_8.with(Value::clone).borrow())() })) })) });
    ({ cpp2rust_assert_fail_1(!({ is_gt_16((*less_9.with(Value::clone).borrow()).clone()) })) });
    ({
        cpp2rust_assert_fail_1(
            !({ is_gt_16((*equivalent_10.with(Value::clone).borrow()).clone()) }),
        )
    });
    ({ cpp2rust_assert_fail_1(({ is_gt_16((*greater_11.with(Value::clone).borrow()).clone()) })) });
    ({
        cpp2rust_assert_fail_1(!({ is_gt_16((*unordered_12.with(Value::clone).borrow()).clone()) }))
    });
    ({ cpp2rust_assert_fail_1(!({ is_gteq_17(({ (*less_3.with(Value::clone).borrow())() })) })) });
    ({ cpp2rust_assert_fail_1(({ is_gteq_17(({ (*equal_4.with(Value::clone).borrow())() })) })) });
    ({
        cpp2rust_assert_fail_1(({ is_gteq_17(({ (*greater_5.with(Value::clone).borrow())() })) }))
    });
    ({ cpp2rust_assert_fail_1(!({ is_gteq_17(({ (*less_6.with(Value::clone).borrow())() })) })) });
    ({
        cpp2rust_assert_fail_1(
            ({ is_gteq_17(({ (*equivalent_7.with(Value::clone).borrow())() })) }),
        )
    });
    ({
        cpp2rust_assert_fail_1(({ is_gteq_17(({ (*greater_8.with(Value::clone).borrow())() })) }))
    });
    ({ cpp2rust_assert_fail_1(!({ is_gteq_17((*less_9.with(Value::clone).borrow()).clone()) })) });
    ({
        cpp2rust_assert_fail_1(
            ({ is_gteq_17((*equivalent_10.with(Value::clone).borrow()).clone()) }),
        )
    });
    ({
        cpp2rust_assert_fail_1(({ is_gteq_17((*greater_11.with(Value::clone).borrow()).clone()) }))
    });
    ({
        cpp2rust_assert_fail_1(
            !({ is_gteq_17((*unordered_12.with(Value::clone).borrow()).clone()) }),
        )
    });
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
