extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct ConstSwappable {
    #[offset(0)]
    pub i: (),
}
pub fn swap_0(lhs: Ptr<ConstSwappable>, rhs: Ptr<ConstSwappable>) {
    ({
        let ___x: Ptr<()> = field_ptr!(lhs, i);
        let ___y: Ptr<()> = field_ptr!(rhs, i);
        swap_1(___x, ___y)
    });
}
pub fn test_2() -> () {
    {
        let t0: Value<()> = Rc::new(RefCell::new(ConstSwappable { i: 0_() }));
        let t1: Value<()> = Rc::new(RefCell::new(ConstSwappable { i: 1_() }));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let t0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let t1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let t0: Value<()> = Rc::new(RefCell::new({
            let __tmp_0: Value<ConstSwappable> = Rc::new(RefCell::new(ConstSwappable { i: 0_() }));
            let __tmp_1: Value<ConstSwappable> = Rc::new(RefCell::new(ConstSwappable { i: 1_() }));
            let __tmp_2: Value<ConstSwappable> = Rc::new(RefCell::new(ConstSwappable { i: 2_() }));
            std_tuple_ConstSwappable__const_ConstSwappable__const_ConstSwappable_::new(
                { __tmp_0.as_pointer() },
                { __tmp_1.as_pointer() },
                { __tmp_2.as_pointer() },
            )
        }));
        let t1: Value<()> = Rc::new(RefCell::new({
            let __tmp_3: Value<ConstSwappable> = Rc::new(RefCell::new(ConstSwappable { i: 3_() }));
            let __tmp_4: Value<ConstSwappable> = Rc::new(RefCell::new(ConstSwappable { i: 4_() }));
            let __tmp_5: Value<ConstSwappable> = Rc::new(RefCell::new(ConstSwappable { i: 5_() }));
            std_tuple_ConstSwappable__const_ConstSwappable__const_ConstSwappable_::new(
                { __tmp_3.as_pointer() },
                { __tmp_4.as_pointer() },
                { __tmp_5.as_pointer() },
            )
        }));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    ({ test_2() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
