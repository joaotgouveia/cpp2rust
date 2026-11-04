extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_0() -> bool {
    let s: Value<std_set_int__std_less_int___std_allocator_int__> = Rc::new(RefCell::new(
        <std_set_int__std_less_int___std_allocator_int__>::default(),
    ));
    let p1: Value<std_pair_std__Rb_tree_const_iterator_int___bool_> = Rc::new(RefCell::new(
        ({
            let ___x: Value<i32> = Rc::new(RefCell::new(1));
            (*s.borrow()).insert_37(___x.as_pointer())
        }),
    ));
    let p2: Value<std_pair_std__Rb_tree_const_iterator_int___bool_> = Rc::new(RefCell::new(
        ({
            let ___x: Value<i32> = Rc::new(RefCell::new(2));
            (*s.borrow()).insert_37(___x.as_pointer())
        }),
    ));
    let cs: Ptr<std_set_int__std_less_int___std_allocator_int__> = s.as_pointer();
    ({
        cpp2rust_assert_fail_1(
            ({
                let ___x: Ptr<i32> = field_ptr!(p1.as_pointer(), first);
                let ___y: Ptr<i32> = field_ptr!(p2.as_pointer(), first);
                ({ (*cs.upgrade().deref()).key_comp() })(___x, ___y)
            }),
        )
    });
    ({
        cpp2rust_assert_fail_1(
            !({
                let ___x: Ptr<i32> = field_ptr!(p2.as_pointer(), first);
                let ___y: Ptr<i32> = field_ptr!(p1.as_pointer(), first);
                ({ (*cs.upgrade().deref()).key_comp() })(___x, ___y)
            }),
        )
    });
    ({
        cpp2rust_assert_fail_1(
            ({
                let ___x: Ptr<i32> = field_ptr!(p1.as_pointer(), first);
                let ___y: Ptr<i32> = field_ptr!(p2.as_pointer(), first);
                ({ (*cs.upgrade().deref()).value_comp() })(___x, ___y)
            }),
        )
    });
    ({
        cpp2rust_assert_fail_1(
            !({
                let ___x: Ptr<i32> = field_ptr!(p2.as_pointer(), first);
                let ___y: Ptr<i32> = field_ptr!(p1.as_pointer(), first);
                ({ (*cs.upgrade().deref()).value_comp() })(___x, ___y)
            }),
        )
    });
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
    return 0;
}
pub fn __cpp2rust_init_globals() {}
