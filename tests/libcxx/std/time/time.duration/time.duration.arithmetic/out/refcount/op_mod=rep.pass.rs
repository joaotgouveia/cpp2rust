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
pub struct NotARep {}
pub fn operator_rem_assign_0(d: (), mut _a1: NotARep) -> () {
    let d: Value<()> = Rc::new(RefCell::new(d));
    return (*d.borrow_mut()).clone();
}
pub fn test_constexpr_1() -> () {
    let s: Value<()> = Rc::new(RefCell::new({
        let __tmp_0: Value<()> = Rc::new(RefCell::new(11_()));
        std_chrono_duration_long_long__std_ratio_1__1__::new_1({ __tmp_0.as_pointer() })
    }));
    unimplemented!();
    return (unimplemented!() == 2_());
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    {
        let us: Value<()> = Rc::new(RefCell::new({
            let __tmp_1: Value<()> = Rc::new(RefCell::new(11_()));
            std_chrono_duration_long_long__std_ratio_1__1000000__::new_1({ __tmp_1.as_pointer() })
        }));
        unimplemented!();
        unimplemented!();
    }
    const _: () = assert!(true, "test_constexpr()");;
    {
        let d: Value<()> = Rc::new(RefCell::new({
            let __tmp_2: Value<()> = Rc::new(RefCell::new(5_()));
            std_chrono_duration_long_long__std_ratio_1__1__::new_1({ __tmp_2.as_pointer() })
        }));
        let mut n: NotARep = <NotARep>::default();
        ({
            let _d: () = (*d.borrow()).clone();
            operator_rem_assign_0(_d, (n).clone())
        });
        unimplemented!();
    }
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
