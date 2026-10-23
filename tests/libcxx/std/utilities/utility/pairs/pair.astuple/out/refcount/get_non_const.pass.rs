extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(12)]
pub struct S {
    #[offset(0)]
    #[byte_size(8)]
    pub a: (),
    #[offset(8)]
    pub k: (),
}
impl S {
    pub fn new() -> Self {
        let __this: Value<S> = Rc::new(RefCell::new(Self {
            a: {
                let __tmp_0: Value<()> = Rc::new(RefCell::new(1_()));
                let __tmp_1: Value<()> = Rc::new(RefCell::new(2_()));
                std_pair_int__int_::new({ __tmp_0.as_pointer() }, { __tmp_1.as_pointer() })
            },
            k: (({ get_0(field_ptr!(this, a)) }).read()),
        }));
        let this: Ptr<S> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for S {
    fn default() -> Self {
        { S::new() }
    }
}
pub fn getP_1() -> () {
    return {
        let __tmp_2: Value<()> = Rc::new(RefCell::new(3_()));
        let __tmp_3: Value<()> = Rc::new(RefCell::new(4_()));
        std_pair_int__int_::new({ __tmp_2.as_pointer() }, { __tmp_3.as_pointer() })
    };
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
        let p: Value<()> = Rc::new(RefCell::new({
            let __tmp_4: Value<()> = Rc::new(RefCell::new(3_()));
            let __tmp_5: Value<()> = Rc::new(RefCell::new(4_()));
            std_pair_int__short_::new({ __tmp_4.as_pointer() }, { __tmp_5.as_pointer() })
        }));
        unimplemented!();
        unimplemented!();
        ({ get_2(p.as_pointer()) }).write(5_());
        ({ get_3(p.as_pointer()) }).write(6_());
        unimplemented!();
        unimplemented!();
    }
    {
        const _: () = assert!(true, "S().k == 1");;
        const _: () = assert!(true, "std::get<1>(getP()) == 4");;
    }
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
