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
    {
        let p1: Value<std_pair_int__short_> = Rc::new(RefCell::new({
            let __tmp_0: Value<i32> = Rc::new(RefCell::new(3));
            let __tmp_1: Value<i16> = Rc::new(RefCell::new(4_i16));
            std_pair_int__short_::new_1({ __tmp_0.as_pointer() }, { __tmp_1.as_pointer() })
        }));
        let p2: Value<std_pair_int__short_> = Rc::new(RefCell::new(std_pair_int__short_::new_2({
            p1.as_pointer()
        })));
        ({ cpp2rust_assert_fail_0(({ (*p2.borrow()).first } == 3)) });
        ({ cpp2rust_assert_fail_0((({ (*p2.borrow()).second } as i32) == 4)) });
    }
    {
        let p1: Value<std_pair_int__short_> = Rc::new(RefCell::new({
            let __tmp_2: Value<i32> = Rc::new(RefCell::new(3));
            let __tmp_3: Value<i16> = Rc::new(RefCell::new(4_i16));
            std_pair_int__short_::new_1({ __tmp_2.as_pointer() }, { __tmp_3.as_pointer() })
        }));
        let p2: Value<std_pair_int__short_> = Rc::new(RefCell::new((*p1.borrow()).clone()));
        const _: () = assert!(true, "p2.first == 3");;
        const _: () = assert!(true, "p2.second == 4");;
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
