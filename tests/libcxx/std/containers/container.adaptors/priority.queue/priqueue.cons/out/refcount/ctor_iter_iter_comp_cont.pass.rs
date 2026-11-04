extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_0() -> bool {
    let a: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([3, 5, 2, 0, 6, 8, 1])));
    let mut n: i32 = (((::std::mem::size_of::<[i32; 7]>() as u64)
        .wrapping_div((::std::mem::size_of::<i32>() as u64))) as i32);
    let v: Value<std_vector_int__std_allocator_int__> =
        Rc::new(RefCell::new(std_vector_int__std_allocator_int__::new_1(
            { (a.as_pointer() as Ptr<i32>) },
            { (a.as_pointer() as Ptr<i32>).offset((n / 2) as isize) },
            None,
        )));
    let q: Value<std_priority_queue_int__std_vector_int__std_allocator_int____std_less_int__> =
        Rc::new(RefCell::new({
            let __tmp_0: Value<std_less_int_> = Rc::new(RefCell::new(<std_less_int_>::default()));
            std_priority_queue_int__std_vector_int__std_allocator_int____std_less_int__::new_1(
                { (a.as_pointer() as Ptr<i32>).offset((n / 2) as isize) },
                { (a.as_pointer() as Ptr<i32>).offset((n) as isize) },
                { __tmp_0.as_pointer() },
                { v.as_pointer() },
            )
        }));
    ({ cpp2rust_assert_fail_1((({ (*q.borrow()).size() }) == (n as u64))) });
    ({ cpp2rust_assert_fail_1(((({ (*q.borrow()).top() }).read()) == 8)) });
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
    ({ cpp2rust_assert_fail_1(({ test_0() })) });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
