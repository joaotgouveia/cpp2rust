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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    {
        let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let mut a: [(); 8] = [
            {
                let __tmp_0: Value<()> = Rc::new(RefCell::new(10_()));
                std_pair_int__std_string_::new_1({ __tmp_0.as_pointer() }, {
                    Ptr::<()>::from_string_literal(b"ten")
                })
            },
            {
                let __tmp_1: Value<()> = Rc::new(RefCell::new(20_()));
                std_pair_int__std_string_::new_2({ __tmp_1.as_pointer() }, {
                    Ptr::<()>::from_string_literal(b"twenty")
                })
            },
            {
                let __tmp_2: Value<()> = Rc::new(RefCell::new(30_()));
                std_pair_int__std_string_::new_2({ __tmp_2.as_pointer() }, {
                    Ptr::<()>::from_string_literal(b"thirty")
                })
            },
            {
                let __tmp_3: Value<()> = Rc::new(RefCell::new(40_()));
                std_pair_int__std_string_::new_3({ __tmp_3.as_pointer() }, {
                    Ptr::<()>::from_string_literal(b"forty")
                })
            },
            {
                let __tmp_4: Value<()> = Rc::new(RefCell::new(50_()));
                std_pair_int__std_string_::new_3({ __tmp_4.as_pointer() }, {
                    Ptr::<()>::from_string_literal(b"fifty")
                })
            },
            {
                let __tmp_5: Value<()> = Rc::new(RefCell::new(60_()));
                std_pair_int__std_string_::new_3({ __tmp_5.as_pointer() }, {
                    Ptr::<()>::from_string_literal(b"sixty")
                })
            },
            {
                let __tmp_6: Value<()> = Rc::new(RefCell::new(70_()));
                std_pair_int__std_string_::new_4({ __tmp_6.as_pointer() }, {
                    Ptr::<()>::from_string_literal(b"seventy")
                })
            },
            {
                let __tmp_7: Value<()> = Rc::new(RefCell::new(80_()));
                std_pair_int__std_string_::new_2({ __tmp_7.as_pointer() }, {
                    Ptr::<()>::from_string_literal(b"eighty")
                })
            },
        ];
        let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
