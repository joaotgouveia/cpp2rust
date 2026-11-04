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
    let d: Value<std_deque_int__std_allocator_int__> = Rc::new(RefCell::new(({ make_0(5) })));
    let q: Value<std_stack_int__std_deque_int__std_allocator_int___> = Rc::new(RefCell::new(
        std_stack_int__std_deque_int__std_allocator_int___::new_1({ d.as_pointer() }),
    ));
    ({ cpp2rust_assert_fail_1((({ (*q.borrow()).size() }) == 5_u64)) });
    let mut i: u64 = 0_u64;
    'loop_: while (i < ({ (*d.borrow()).size() })) {
        ({
            cpp2rust_assert_fail_1(
                ((({ (*q.borrow()).top_15() }).read())
                    == (elem!(
                        d.as_pointer(),
                        (({ (*d.borrow()).size() }).wrapping_sub(i)).wrapping_sub(1_u64)
                    )
                    .read())),
            )
        });
        ({ (*q.borrow()).pop() });
        i.prefix_inc();
    }
    const _: () = assert!(true, "!test_convertible<Q, const Container&>()");;
    return 0;
}
pub fn __cpp2rust_init_globals() {}
