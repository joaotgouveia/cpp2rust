extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Empty {}
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
        let t: Value<()> = Rc::new(RefCell::new(3_()));
        unimplemented!();
    }
    {
        let t: Value<()> = Rc::new(RefCell::new({
            let __tmp_0: Value<()> = Rc::new(RefCell::new(5_()));
            std_tuple_std_string__int_::new({ Ptr::<()>::from_string_literal(b"high") }, {
                __tmp_0.as_pointer()
            })
        }));
        unimplemented!();
        unimplemented!();
    }
    {
        let t: Value<()> = Rc::new(RefCell::new({
            let __tmp_1: Value<()> = Rc::new(RefCell::new(2.718_f64));
            let __tmp_2: Value<()> = Rc::new(RefCell::new(5_()));
            std_tuple_double__int_::new({ __tmp_1.as_pointer() }, { __tmp_2.as_pointer() })
        }));
        const _: () = assert!(true, "std::get<0>(t) == 2.718");;
        const _: () = assert!(true, "std::get<1>(t) == 5");;
    }
    {
        let t: Value<()> = Rc::new(RefCell::new(<Empty>::default()));
        let mut e: Empty = (*({ get_0(t.as_pointer()) }).upgrade().deref()).clone();
        (&(e));
    }
    {
        let d: Value<()> = Rc::new(RefCell::new(1.5_f64));
        let t: Value<()> = Rc::new(RefCell::new({
            let __tmp_3: Value<()> = Rc::new(RefCell::new(5_()));
            std_tuple_double_ref__std_string__int_::new(
                { d.as_pointer() },
                { Ptr::<()>::from_string_literal(b"high") },
                { __tmp_3.as_pointer() },
            )
        }));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        ({ get_1(t.as_pointer()) }).write(2.5_f64);
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
