extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};

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
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    ({
        cpp2rust_assert_fail_0(
            ((error_collate_1.with(|rc| *rc.borrow()) as i32)
                != (error_ctype_2.with(|rc| *rc.borrow()) as i32)),
        )
    });
    ({
        cpp2rust_assert_fail_0(
            ((error_collate_1.with(|rc| *rc.borrow()) as i32)
                != (error_escape_3.with(|rc| *rc.borrow()) as i32)),
        )
    });
    ({
        cpp2rust_assert_fail_0(
            ((error_collate_1.with(|rc| *rc.borrow()) as i32)
                != (error_backref_4.with(|rc| *rc.borrow()) as i32)),
        )
    });
    ({
        cpp2rust_assert_fail_0(
            ((error_collate_1.with(|rc| *rc.borrow()) as i32)
                != (error_brack_5.with(|rc| *rc.borrow()) as i32)),
        )
    });
    ({
        cpp2rust_assert_fail_0(
            ((error_collate_1.with(|rc| *rc.borrow()) as i32)
                != (error_paren_6.with(|rc| *rc.borrow()) as i32)),
        )
    });
    ({
        cpp2rust_assert_fail_0(
            ((error_collate_1.with(|rc| *rc.borrow()) as i32)
                != (error_brace_7.with(|rc| *rc.borrow()) as i32)),
        )
    });
    ({
        cpp2rust_assert_fail_0(
            ((error_collate_1.with(|rc| *rc.borrow()) as i32)
                != (error_badbrace_8.with(|rc| *rc.borrow()) as i32)),
        )
    });
    ({
        cpp2rust_assert_fail_0(
            ((error_collate_1.with(|rc| *rc.borrow()) as i32)
                != (error_range_9.with(|rc| *rc.borrow()) as i32)),
        )
    });
    ({
        cpp2rust_assert_fail_0(
            ((error_collate_1.with(|rc| *rc.borrow()) as i32)
                != (error_space_10.with(|rc| *rc.borrow()) as i32)),
        )
    });
    ({
        cpp2rust_assert_fail_0(
            ((error_collate_1.with(|rc| *rc.borrow()) as i32)
                != (error_badrepeat_11.with(|rc| *rc.borrow()) as i32)),
        )
    });
    ({
        cpp2rust_assert_fail_0(
            ((error_collate_1.with(|rc| *rc.borrow()) as i32)
                != (error_complexity_12.with(|rc| *rc.borrow()) as i32)),
        )
    });
    ({
        cpp2rust_assert_fail_0(
            ((error_collate_1.with(|rc| *rc.borrow()) as i32)
                != (error_stack_13.with(|rc| *rc.borrow()) as i32)),
        )
    });
    ({
        cpp2rust_assert_fail_0(
            ((error_ctype_2.with(|rc| *rc.borrow()) as i32)
                != (error_escape_3.with(|rc| *rc.borrow()) as i32)),
        )
    });
    ({
        cpp2rust_assert_fail_0(
            ((error_ctype_2.with(|rc| *rc.borrow()) as i32)
                != (error_backref_4.with(|rc| *rc.borrow()) as i32)),
        )
    });
    ({
        cpp2rust_assert_fail_0(
            ((error_ctype_2.with(|rc| *rc.borrow()) as i32)
                != (error_brack_5.with(|rc| *rc.borrow()) as i32)),
        )
    });
    ({
        cpp2rust_assert_fail_0(
            ((error_ctype_2.with(|rc| *rc.borrow()) as i32)
                != (error_paren_6.with(|rc| *rc.borrow()) as i32)),
        )
    });
    ({
        cpp2rust_assert_fail_0(
            ((error_ctype_2.with(|rc| *rc.borrow()) as i32)
                != (error_brace_7.with(|rc| *rc.borrow()) as i32)),
        )
    });
    ({
        cpp2rust_assert_fail_0(
            ((error_ctype_2.with(|rc| *rc.borrow()) as i32)
                != (error_badbrace_8.with(|rc| *rc.borrow()) as i32)),
        )
    });
    ({
        cpp2rust_assert_fail_0(
            ((error_ctype_2.with(|rc| *rc.borrow()) as i32)
                != (error_range_9.with(|rc| *rc.borrow()) as i32)),
        )
    });
    ({
        cpp2rust_assert_fail_0(
            ((error_ctype_2.with(|rc| *rc.borrow()) as i32)
                != (error_space_10.with(|rc| *rc.borrow()) as i32)),
        )
    });
    ({
        cpp2rust_assert_fail_0(
            ((error_ctype_2.with(|rc| *rc.borrow()) as i32)
                != (error_badrepeat_11.with(|rc| *rc.borrow()) as i32)),
        )
    });
    ({
        cpp2rust_assert_fail_0(
            ((error_ctype_2.with(|rc| *rc.borrow()) as i32)
                != (error_complexity_12.with(|rc| *rc.borrow()) as i32)),
        )
    });
    ({
        cpp2rust_assert_fail_0(
            ((error_ctype_2.with(|rc| *rc.borrow()) as i32)
                != (error_stack_13.with(|rc| *rc.borrow()) as i32)),
        )
    });
    ({
        cpp2rust_assert_fail_0(
            ((error_escape_3.with(|rc| *rc.borrow()) as i32)
                != (error_backref_4.with(|rc| *rc.borrow()) as i32)),
        )
    });
    ({
        cpp2rust_assert_fail_0(
            ((error_escape_3.with(|rc| *rc.borrow()) as i32)
                != (error_brack_5.with(|rc| *rc.borrow()) as i32)),
        )
    });
    ({
        cpp2rust_assert_fail_0(
            ((error_escape_3.with(|rc| *rc.borrow()) as i32)
                != (error_paren_6.with(|rc| *rc.borrow()) as i32)),
        )
    });
    ({
        cpp2rust_assert_fail_0(
            ((error_escape_3.with(|rc| *rc.borrow()) as i32)
                != (error_brace_7.with(|rc| *rc.borrow()) as i32)),
        )
    });
    ({
        cpp2rust_assert_fail_0(
            ((error_escape_3.with(|rc| *rc.borrow()) as i32)
                != (error_badbrace_8.with(|rc| *rc.borrow()) as i32)),
        )
    });
    ({
        cpp2rust_assert_fail_0(
            ((error_escape_3.with(|rc| *rc.borrow()) as i32)
                != (error_range_9.with(|rc| *rc.borrow()) as i32)),
        )
    });
    ({
        cpp2rust_assert_fail_0(
            ((error_escape_3.with(|rc| *rc.borrow()) as i32)
                != (error_space_10.with(|rc| *rc.borrow()) as i32)),
        )
    });
    ({
        cpp2rust_assert_fail_0(
            ((error_escape_3.with(|rc| *rc.borrow()) as i32)
                != (error_badrepeat_11.with(|rc| *rc.borrow()) as i32)),
        )
    });
    ({
        cpp2rust_assert_fail_0(
            ((error_escape_3.with(|rc| *rc.borrow()) as i32)
                != (error_complexity_12.with(|rc| *rc.borrow()) as i32)),
        )
    });
    ({
        cpp2rust_assert_fail_0(
            ((error_escape_3.with(|rc| *rc.borrow()) as i32)
                != (error_stack_13.with(|rc| *rc.borrow()) as i32)),
        )
    });
    ({
        cpp2rust_assert_fail_0(
            ((error_backref_4.with(|rc| *rc.borrow()) as i32)
                != (error_brack_5.with(|rc| *rc.borrow()) as i32)),
        )
    });
    ({
        cpp2rust_assert_fail_0(
            ((error_backref_4.with(|rc| *rc.borrow()) as i32)
                != (error_paren_6.with(|rc| *rc.borrow()) as i32)),
        )
    });
    ({
        cpp2rust_assert_fail_0(
            ((error_backref_4.with(|rc| *rc.borrow()) as i32)
                != (error_brace_7.with(|rc| *rc.borrow()) as i32)),
        )
    });
    ({
        cpp2rust_assert_fail_0(
            ((error_backref_4.with(|rc| *rc.borrow()) as i32)
                != (error_badbrace_8.with(|rc| *rc.borrow()) as i32)),
        )
    });
    ({
        cpp2rust_assert_fail_0(
            ((error_backref_4.with(|rc| *rc.borrow()) as i32)
                != (error_range_9.with(|rc| *rc.borrow()) as i32)),
        )
    });
    ({
        cpp2rust_assert_fail_0(
            ((error_backref_4.with(|rc| *rc.borrow()) as i32)
                != (error_space_10.with(|rc| *rc.borrow()) as i32)),
        )
    });
    ({
        cpp2rust_assert_fail_0(
            ((error_backref_4.with(|rc| *rc.borrow()) as i32)
                != (error_badrepeat_11.with(|rc| *rc.borrow()) as i32)),
        )
    });
    ({
        cpp2rust_assert_fail_0(
            ((error_backref_4.with(|rc| *rc.borrow()) as i32)
                != (error_complexity_12.with(|rc| *rc.borrow()) as i32)),
        )
    });
    ({
        cpp2rust_assert_fail_0(
            ((error_backref_4.with(|rc| *rc.borrow()) as i32)
                != (error_stack_13.with(|rc| *rc.borrow()) as i32)),
        )
    });
    ({
        cpp2rust_assert_fail_0(
            ((error_brack_5.with(|rc| *rc.borrow()) as i32)
                != (error_paren_6.with(|rc| *rc.borrow()) as i32)),
        )
    });
    ({
        cpp2rust_assert_fail_0(
            ((error_brack_5.with(|rc| *rc.borrow()) as i32)
                != (error_brace_7.with(|rc| *rc.borrow()) as i32)),
        )
    });
    ({
        cpp2rust_assert_fail_0(
            ((error_brack_5.with(|rc| *rc.borrow()) as i32)
                != (error_badbrace_8.with(|rc| *rc.borrow()) as i32)),
        )
    });
    ({
        cpp2rust_assert_fail_0(
            ((error_brack_5.with(|rc| *rc.borrow()) as i32)
                != (error_range_9.with(|rc| *rc.borrow()) as i32)),
        )
    });
    ({
        cpp2rust_assert_fail_0(
            ((error_brack_5.with(|rc| *rc.borrow()) as i32)
                != (error_space_10.with(|rc| *rc.borrow()) as i32)),
        )
    });
    ({
        cpp2rust_assert_fail_0(
            ((error_brack_5.with(|rc| *rc.borrow()) as i32)
                != (error_badrepeat_11.with(|rc| *rc.borrow()) as i32)),
        )
    });
    ({
        cpp2rust_assert_fail_0(
            ((error_brack_5.with(|rc| *rc.borrow()) as i32)
                != (error_complexity_12.with(|rc| *rc.borrow()) as i32)),
        )
    });
    ({
        cpp2rust_assert_fail_0(
            ((error_brack_5.with(|rc| *rc.borrow()) as i32)
                != (error_stack_13.with(|rc| *rc.borrow()) as i32)),
        )
    });
    ({
        cpp2rust_assert_fail_0(
            ((error_paren_6.with(|rc| *rc.borrow()) as i32)
                != (error_brace_7.with(|rc| *rc.borrow()) as i32)),
        )
    });
    ({
        cpp2rust_assert_fail_0(
            ((error_paren_6.with(|rc| *rc.borrow()) as i32)
                != (error_badbrace_8.with(|rc| *rc.borrow()) as i32)),
        )
    });
    ({
        cpp2rust_assert_fail_0(
            ((error_paren_6.with(|rc| *rc.borrow()) as i32)
                != (error_range_9.with(|rc| *rc.borrow()) as i32)),
        )
    });
    ({
        cpp2rust_assert_fail_0(
            ((error_paren_6.with(|rc| *rc.borrow()) as i32)
                != (error_space_10.with(|rc| *rc.borrow()) as i32)),
        )
    });
    ({
        cpp2rust_assert_fail_0(
            ((error_paren_6.with(|rc| *rc.borrow()) as i32)
                != (error_badrepeat_11.with(|rc| *rc.borrow()) as i32)),
        )
    });
    ({
        cpp2rust_assert_fail_0(
            ((error_paren_6.with(|rc| *rc.borrow()) as i32)
                != (error_complexity_12.with(|rc| *rc.borrow()) as i32)),
        )
    });
    ({
        cpp2rust_assert_fail_0(
            ((error_paren_6.with(|rc| *rc.borrow()) as i32)
                != (error_stack_13.with(|rc| *rc.borrow()) as i32)),
        )
    });
    ({
        cpp2rust_assert_fail_0(
            ((error_brace_7.with(|rc| *rc.borrow()) as i32)
                != (error_badbrace_8.with(|rc| *rc.borrow()) as i32)),
        )
    });
    ({
        cpp2rust_assert_fail_0(
            ((error_brace_7.with(|rc| *rc.borrow()) as i32)
                != (error_range_9.with(|rc| *rc.borrow()) as i32)),
        )
    });
    ({
        cpp2rust_assert_fail_0(
            ((error_brace_7.with(|rc| *rc.borrow()) as i32)
                != (error_space_10.with(|rc| *rc.borrow()) as i32)),
        )
    });
    ({
        cpp2rust_assert_fail_0(
            ((error_brace_7.with(|rc| *rc.borrow()) as i32)
                != (error_badrepeat_11.with(|rc| *rc.borrow()) as i32)),
        )
    });
    ({
        cpp2rust_assert_fail_0(
            ((error_brace_7.with(|rc| *rc.borrow()) as i32)
                != (error_complexity_12.with(|rc| *rc.borrow()) as i32)),
        )
    });
    ({
        cpp2rust_assert_fail_0(
            ((error_brace_7.with(|rc| *rc.borrow()) as i32)
                != (error_stack_13.with(|rc| *rc.borrow()) as i32)),
        )
    });
    ({
        cpp2rust_assert_fail_0(
            ((error_badbrace_8.with(|rc| *rc.borrow()) as i32)
                != (error_range_9.with(|rc| *rc.borrow()) as i32)),
        )
    });
    ({
        cpp2rust_assert_fail_0(
            ((error_badbrace_8.with(|rc| *rc.borrow()) as i32)
                != (error_space_10.with(|rc| *rc.borrow()) as i32)),
        )
    });
    ({
        cpp2rust_assert_fail_0(
            ((error_badbrace_8.with(|rc| *rc.borrow()) as i32)
                != (error_badrepeat_11.with(|rc| *rc.borrow()) as i32)),
        )
    });
    ({
        cpp2rust_assert_fail_0(
            ((error_badbrace_8.with(|rc| *rc.borrow()) as i32)
                != (error_complexity_12.with(|rc| *rc.borrow()) as i32)),
        )
    });
    ({
        cpp2rust_assert_fail_0(
            ((error_badbrace_8.with(|rc| *rc.borrow()) as i32)
                != (error_stack_13.with(|rc| *rc.borrow()) as i32)),
        )
    });
    ({
        cpp2rust_assert_fail_0(
            ((error_range_9.with(|rc| *rc.borrow()) as i32)
                != (error_space_10.with(|rc| *rc.borrow()) as i32)),
        )
    });
    ({
        cpp2rust_assert_fail_0(
            ((error_range_9.with(|rc| *rc.borrow()) as i32)
                != (error_badrepeat_11.with(|rc| *rc.borrow()) as i32)),
        )
    });
    ({
        cpp2rust_assert_fail_0(
            ((error_range_9.with(|rc| *rc.borrow()) as i32)
                != (error_complexity_12.with(|rc| *rc.borrow()) as i32)),
        )
    });
    ({
        cpp2rust_assert_fail_0(
            ((error_range_9.with(|rc| *rc.borrow()) as i32)
                != (error_stack_13.with(|rc| *rc.borrow()) as i32)),
        )
    });
    ({
        cpp2rust_assert_fail_0(
            ((error_space_10.with(|rc| *rc.borrow()) as i32)
                != (error_badrepeat_11.with(|rc| *rc.borrow()) as i32)),
        )
    });
    ({
        cpp2rust_assert_fail_0(
            ((error_space_10.with(|rc| *rc.borrow()) as i32)
                != (error_complexity_12.with(|rc| *rc.borrow()) as i32)),
        )
    });
    ({
        cpp2rust_assert_fail_0(
            ((error_space_10.with(|rc| *rc.borrow()) as i32)
                != (error_stack_13.with(|rc| *rc.borrow()) as i32)),
        )
    });
    ({
        cpp2rust_assert_fail_0(
            ((error_badrepeat_11.with(|rc| *rc.borrow()) as i32)
                != (error_complexity_12.with(|rc| *rc.borrow()) as i32)),
        )
    });
    ({
        cpp2rust_assert_fail_0(
            ((error_badrepeat_11.with(|rc| *rc.borrow()) as i32)
                != (error_stack_13.with(|rc| *rc.borrow()) as i32)),
        )
    });
    ({
        cpp2rust_assert_fail_0(
            ((error_complexity_12.with(|rc| *rc.borrow()) as i32)
                != (error_stack_13.with(|rc| *rc.borrow()) as i32)),
        )
    });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
