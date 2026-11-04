extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn tests_0() -> bool {
    let a: Value<Box<[bool]>> = Rc::new(RefCell::new(Box::new([false, true, false, true])));
    let mut an: Ptr<bool> = (a.as_pointer() as Ptr<bool>).offset(
        ((::std::mem::size_of::<[bool; 4]>() as u64)
            .wrapping_div((::std::mem::size_of::<bool>() as u64))) as isize,
    );
    let v: Value<std_vector_bool__std_allocator_bool__> =
        Rc::new(RefCell::new(std_vector_bool__std_allocator_bool__::new_1(
            { (a.as_pointer() as Ptr<bool>) },
            { (an).clone() },
            None,
        )));
    let r1: Value<std__Bit_reference> = Rc::new(RefCell::new(((*v.borrow())[0_u64]).clone()));
    let r2: Value<std__Bit_reference> = Rc::new(RefCell::new(((*v.borrow())[3_u64]).clone()));
    const _: () = assert!(true, "(noexcept(v.swap(r1, r2)))");;
    ({ cpp2rust_assert_fail_1(!({ (*r1.borrow())() })) });
    ({ cpp2rust_assert_fail_1(({ (*r2.borrow())() })) });
    ({
        std_vector_bool__std_allocator_bool__::swap_51(
            (*r1.borrow()).clone(),
            (*r2.borrow()).clone(),
        )
    });
    ({ cpp2rust_assert_fail_1(({ (*r1.borrow())() })) });
    ({ cpp2rust_assert_fail_1(!({ (*r2.borrow())() })) });
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
    ({ tests_0() });
    const _: () = assert!(true, "tests()");;
    return 0;
}
pub fn __cpp2rust_init_globals() {}
