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
    assert!(
        ((error_collate_0.with(|rc| *rc.borrow()) as i32)
            != (error_ctype_1.with(|rc| *rc.borrow()) as i32))
    );
    assert!(
        ((error_collate_0.with(|rc| *rc.borrow()) as i32)
            != (error_escape_2.with(|rc| *rc.borrow()) as i32))
    );
    assert!(
        ((error_collate_0.with(|rc| *rc.borrow()) as i32)
            != (error_backref_3.with(|rc| *rc.borrow()) as i32))
    );
    assert!(
        ((error_collate_0.with(|rc| *rc.borrow()) as i32)
            != (error_brack_4.with(|rc| *rc.borrow()) as i32))
    );
    assert!(
        ((error_collate_0.with(|rc| *rc.borrow()) as i32)
            != (error_paren_5.with(|rc| *rc.borrow()) as i32))
    );
    assert!(
        ((error_collate_0.with(|rc| *rc.borrow()) as i32)
            != (error_brace_6.with(|rc| *rc.borrow()) as i32))
    );
    assert!(
        ((error_collate_0.with(|rc| *rc.borrow()) as i32)
            != (error_badbrace_7.with(|rc| *rc.borrow()) as i32))
    );
    assert!(
        ((error_collate_0.with(|rc| *rc.borrow()) as i32)
            != (error_range_8.with(|rc| *rc.borrow()) as i32))
    );
    assert!(
        ((error_collate_0.with(|rc| *rc.borrow()) as i32)
            != (error_space_9.with(|rc| *rc.borrow()) as i32))
    );
    assert!(
        ((error_collate_0.with(|rc| *rc.borrow()) as i32)
            != (error_badrepeat_10.with(|rc| *rc.borrow()) as i32))
    );
    assert!(
        ((error_collate_0.with(|rc| *rc.borrow()) as i32)
            != (error_complexity_11.with(|rc| *rc.borrow()) as i32))
    );
    assert!(
        ((error_collate_0.with(|rc| *rc.borrow()) as i32)
            != (error_stack_12.with(|rc| *rc.borrow()) as i32))
    );
    assert!(
        ((error_ctype_1.with(|rc| *rc.borrow()) as i32)
            != (error_escape_2.with(|rc| *rc.borrow()) as i32))
    );
    assert!(
        ((error_ctype_1.with(|rc| *rc.borrow()) as i32)
            != (error_backref_3.with(|rc| *rc.borrow()) as i32))
    );
    assert!(
        ((error_ctype_1.with(|rc| *rc.borrow()) as i32)
            != (error_brack_4.with(|rc| *rc.borrow()) as i32))
    );
    assert!(
        ((error_ctype_1.with(|rc| *rc.borrow()) as i32)
            != (error_paren_5.with(|rc| *rc.borrow()) as i32))
    );
    assert!(
        ((error_ctype_1.with(|rc| *rc.borrow()) as i32)
            != (error_brace_6.with(|rc| *rc.borrow()) as i32))
    );
    assert!(
        ((error_ctype_1.with(|rc| *rc.borrow()) as i32)
            != (error_badbrace_7.with(|rc| *rc.borrow()) as i32))
    );
    assert!(
        ((error_ctype_1.with(|rc| *rc.borrow()) as i32)
            != (error_range_8.with(|rc| *rc.borrow()) as i32))
    );
    assert!(
        ((error_ctype_1.with(|rc| *rc.borrow()) as i32)
            != (error_space_9.with(|rc| *rc.borrow()) as i32))
    );
    assert!(
        ((error_ctype_1.with(|rc| *rc.borrow()) as i32)
            != (error_badrepeat_10.with(|rc| *rc.borrow()) as i32))
    );
    assert!(
        ((error_ctype_1.with(|rc| *rc.borrow()) as i32)
            != (error_complexity_11.with(|rc| *rc.borrow()) as i32))
    );
    assert!(
        ((error_ctype_1.with(|rc| *rc.borrow()) as i32)
            != (error_stack_12.with(|rc| *rc.borrow()) as i32))
    );
    assert!(
        ((error_escape_2.with(|rc| *rc.borrow()) as i32)
            != (error_backref_3.with(|rc| *rc.borrow()) as i32))
    );
    assert!(
        ((error_escape_2.with(|rc| *rc.borrow()) as i32)
            != (error_brack_4.with(|rc| *rc.borrow()) as i32))
    );
    assert!(
        ((error_escape_2.with(|rc| *rc.borrow()) as i32)
            != (error_paren_5.with(|rc| *rc.borrow()) as i32))
    );
    assert!(
        ((error_escape_2.with(|rc| *rc.borrow()) as i32)
            != (error_brace_6.with(|rc| *rc.borrow()) as i32))
    );
    assert!(
        ((error_escape_2.with(|rc| *rc.borrow()) as i32)
            != (error_badbrace_7.with(|rc| *rc.borrow()) as i32))
    );
    assert!(
        ((error_escape_2.with(|rc| *rc.borrow()) as i32)
            != (error_range_8.with(|rc| *rc.borrow()) as i32))
    );
    assert!(
        ((error_escape_2.with(|rc| *rc.borrow()) as i32)
            != (error_space_9.with(|rc| *rc.borrow()) as i32))
    );
    assert!(
        ((error_escape_2.with(|rc| *rc.borrow()) as i32)
            != (error_badrepeat_10.with(|rc| *rc.borrow()) as i32))
    );
    assert!(
        ((error_escape_2.with(|rc| *rc.borrow()) as i32)
            != (error_complexity_11.with(|rc| *rc.borrow()) as i32))
    );
    assert!(
        ((error_escape_2.with(|rc| *rc.borrow()) as i32)
            != (error_stack_12.with(|rc| *rc.borrow()) as i32))
    );
    assert!(
        ((error_backref_3.with(|rc| *rc.borrow()) as i32)
            != (error_brack_4.with(|rc| *rc.borrow()) as i32))
    );
    assert!(
        ((error_backref_3.with(|rc| *rc.borrow()) as i32)
            != (error_paren_5.with(|rc| *rc.borrow()) as i32))
    );
    assert!(
        ((error_backref_3.with(|rc| *rc.borrow()) as i32)
            != (error_brace_6.with(|rc| *rc.borrow()) as i32))
    );
    assert!(
        ((error_backref_3.with(|rc| *rc.borrow()) as i32)
            != (error_badbrace_7.with(|rc| *rc.borrow()) as i32))
    );
    assert!(
        ((error_backref_3.with(|rc| *rc.borrow()) as i32)
            != (error_range_8.with(|rc| *rc.borrow()) as i32))
    );
    assert!(
        ((error_backref_3.with(|rc| *rc.borrow()) as i32)
            != (error_space_9.with(|rc| *rc.borrow()) as i32))
    );
    assert!(
        ((error_backref_3.with(|rc| *rc.borrow()) as i32)
            != (error_badrepeat_10.with(|rc| *rc.borrow()) as i32))
    );
    assert!(
        ((error_backref_3.with(|rc| *rc.borrow()) as i32)
            != (error_complexity_11.with(|rc| *rc.borrow()) as i32))
    );
    assert!(
        ((error_backref_3.with(|rc| *rc.borrow()) as i32)
            != (error_stack_12.with(|rc| *rc.borrow()) as i32))
    );
    assert!(
        ((error_brack_4.with(|rc| *rc.borrow()) as i32)
            != (error_paren_5.with(|rc| *rc.borrow()) as i32))
    );
    assert!(
        ((error_brack_4.with(|rc| *rc.borrow()) as i32)
            != (error_brace_6.with(|rc| *rc.borrow()) as i32))
    );
    assert!(
        ((error_brack_4.with(|rc| *rc.borrow()) as i32)
            != (error_badbrace_7.with(|rc| *rc.borrow()) as i32))
    );
    assert!(
        ((error_brack_4.with(|rc| *rc.borrow()) as i32)
            != (error_range_8.with(|rc| *rc.borrow()) as i32))
    );
    assert!(
        ((error_brack_4.with(|rc| *rc.borrow()) as i32)
            != (error_space_9.with(|rc| *rc.borrow()) as i32))
    );
    assert!(
        ((error_brack_4.with(|rc| *rc.borrow()) as i32)
            != (error_badrepeat_10.with(|rc| *rc.borrow()) as i32))
    );
    assert!(
        ((error_brack_4.with(|rc| *rc.borrow()) as i32)
            != (error_complexity_11.with(|rc| *rc.borrow()) as i32))
    );
    assert!(
        ((error_brack_4.with(|rc| *rc.borrow()) as i32)
            != (error_stack_12.with(|rc| *rc.borrow()) as i32))
    );
    assert!(
        ((error_paren_5.with(|rc| *rc.borrow()) as i32)
            != (error_brace_6.with(|rc| *rc.borrow()) as i32))
    );
    assert!(
        ((error_paren_5.with(|rc| *rc.borrow()) as i32)
            != (error_badbrace_7.with(|rc| *rc.borrow()) as i32))
    );
    assert!(
        ((error_paren_5.with(|rc| *rc.borrow()) as i32)
            != (error_range_8.with(|rc| *rc.borrow()) as i32))
    );
    assert!(
        ((error_paren_5.with(|rc| *rc.borrow()) as i32)
            != (error_space_9.with(|rc| *rc.borrow()) as i32))
    );
    assert!(
        ((error_paren_5.with(|rc| *rc.borrow()) as i32)
            != (error_badrepeat_10.with(|rc| *rc.borrow()) as i32))
    );
    assert!(
        ((error_paren_5.with(|rc| *rc.borrow()) as i32)
            != (error_complexity_11.with(|rc| *rc.borrow()) as i32))
    );
    assert!(
        ((error_paren_5.with(|rc| *rc.borrow()) as i32)
            != (error_stack_12.with(|rc| *rc.borrow()) as i32))
    );
    assert!(
        ((error_brace_6.with(|rc| *rc.borrow()) as i32)
            != (error_badbrace_7.with(|rc| *rc.borrow()) as i32))
    );
    assert!(
        ((error_brace_6.with(|rc| *rc.borrow()) as i32)
            != (error_range_8.with(|rc| *rc.borrow()) as i32))
    );
    assert!(
        ((error_brace_6.with(|rc| *rc.borrow()) as i32)
            != (error_space_9.with(|rc| *rc.borrow()) as i32))
    );
    assert!(
        ((error_brace_6.with(|rc| *rc.borrow()) as i32)
            != (error_badrepeat_10.with(|rc| *rc.borrow()) as i32))
    );
    assert!(
        ((error_brace_6.with(|rc| *rc.borrow()) as i32)
            != (error_complexity_11.with(|rc| *rc.borrow()) as i32))
    );
    assert!(
        ((error_brace_6.with(|rc| *rc.borrow()) as i32)
            != (error_stack_12.with(|rc| *rc.borrow()) as i32))
    );
    assert!(
        ((error_badbrace_7.with(|rc| *rc.borrow()) as i32)
            != (error_range_8.with(|rc| *rc.borrow()) as i32))
    );
    assert!(
        ((error_badbrace_7.with(|rc| *rc.borrow()) as i32)
            != (error_space_9.with(|rc| *rc.borrow()) as i32))
    );
    assert!(
        ((error_badbrace_7.with(|rc| *rc.borrow()) as i32)
            != (error_badrepeat_10.with(|rc| *rc.borrow()) as i32))
    );
    assert!(
        ((error_badbrace_7.with(|rc| *rc.borrow()) as i32)
            != (error_complexity_11.with(|rc| *rc.borrow()) as i32))
    );
    assert!(
        ((error_badbrace_7.with(|rc| *rc.borrow()) as i32)
            != (error_stack_12.with(|rc| *rc.borrow()) as i32))
    );
    assert!(
        ((error_range_8.with(|rc| *rc.borrow()) as i32)
            != (error_space_9.with(|rc| *rc.borrow()) as i32))
    );
    assert!(
        ((error_range_8.with(|rc| *rc.borrow()) as i32)
            != (error_badrepeat_10.with(|rc| *rc.borrow()) as i32))
    );
    assert!(
        ((error_range_8.with(|rc| *rc.borrow()) as i32)
            != (error_complexity_11.with(|rc| *rc.borrow()) as i32))
    );
    assert!(
        ((error_range_8.with(|rc| *rc.borrow()) as i32)
            != (error_stack_12.with(|rc| *rc.borrow()) as i32))
    );
    assert!(
        ((error_space_9.with(|rc| *rc.borrow()) as i32)
            != (error_badrepeat_10.with(|rc| *rc.borrow()) as i32))
    );
    assert!(
        ((error_space_9.with(|rc| *rc.borrow()) as i32)
            != (error_complexity_11.with(|rc| *rc.borrow()) as i32))
    );
    assert!(
        ((error_space_9.with(|rc| *rc.borrow()) as i32)
            != (error_stack_12.with(|rc| *rc.borrow()) as i32))
    );
    assert!(
        ((error_badrepeat_10.with(|rc| *rc.borrow()) as i32)
            != (error_complexity_11.with(|rc| *rc.borrow()) as i32))
    );
    assert!(
        ((error_badrepeat_10.with(|rc| *rc.borrow()) as i32)
            != (error_stack_12.with(|rc| *rc.borrow()) as i32))
    );
    assert!(
        ((error_complexity_11.with(|rc| *rc.borrow()) as i32)
            != (error_stack_12.with(|rc| *rc.borrow()) as i32))
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
