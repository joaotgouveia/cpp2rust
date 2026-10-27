extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_0(mut bad: (), mut args_1: (), mut args_2: (), mut args_3: (), mut args_4: ()) {
    let set: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let vals: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([args_1, args_2, args_3, args_4])));
    'loop_: for v in IterRange::new(
        (vals.as_pointer() as Ptr<()>),
        (vals.as_pointer() as Ptr<()>).offset((4_()) as isize),
    ) {
        let v: Ptr<()> = (v).clone();
        unimplemented!();
    }
    'loop_: for v in IterRange::new(
        (vals.as_pointer() as Ptr<()>),
        (vals.as_pointer() as Ptr<()>).offset((4_()) as isize),
    ) {
        let v: Ptr<()> = (v).clone();
        unimplemented!();
    }
    unimplemented!();
}
pub fn test_1(mut bad: (), mut args_1: (), mut args_2: (), mut args_3: (), mut args_4: ()) {
    let set: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let vals: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([args_1, args_2, args_3, args_4])));
    'loop_: for v in IterRange::new(
        (vals.as_pointer() as Ptr<()>),
        (vals.as_pointer() as Ptr<()>).offset((4_()) as isize),
    ) {
        let v: Ptr<()> = (v).clone();
        unimplemented!();
    }
    'loop_: for v in IterRange::new(
        (vals.as_pointer() as Ptr<()>),
        (vals.as_pointer() as Ptr<()>).offset((4_()) as isize),
    ) {
        let v: Ptr<()> = (v).clone();
        unimplemented!();
    }
    unimplemented!();
}
pub fn test_2(mut bad: (), mut args_1: (), mut args_2: (), mut args_3: (), mut args_4: ()) {
    let set: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let vals: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([args_1, args_2, args_3, args_4])));
    'loop_: for v in IterRange::new(
        (vals.as_pointer() as Ptr<()>),
        (vals.as_pointer() as Ptr<()>).offset((4_()) as isize),
    ) {
        let v: Ptr<()> = (v).clone();
        unimplemented!();
    }
    'loop_: for v in IterRange::new(
        (vals.as_pointer() as Ptr<()>),
        (vals.as_pointer() as Ptr<()>).offset((4_()) as isize),
    ) {
        let v: Ptr<()> = (v).clone();
        unimplemented!();
    }
    unimplemented!();
}
pub fn test_3(mut bad: (), mut args_1: (), mut args_2: (), mut args_3: (), mut args_4: ()) {
    let set: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let vals: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([args_1, args_2, args_3, args_4])));
    'loop_: for v in IterRange::new(
        (vals.as_pointer() as Ptr<()>),
        (vals.as_pointer() as Ptr<()>).offset((4_()) as isize),
    ) {
        let v: Ptr<()> = (v).clone();
        unimplemented!();
    }
    'loop_: for v in IterRange::new(
        (vals.as_pointer() as Ptr<()>),
        (vals.as_pointer() as Ptr<()>).offset((4_()) as isize),
    ) {
        let v: Ptr<()> = (v).clone();
        unimplemented!();
    }
    unimplemented!();
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(24)]
pub struct E {
    #[offset(0)]
    pub a: (),
    #[offset(8)]
    pub b: (),
    #[offset(16)]
    pub c: (),
}
impl Default for E {
    fn default() -> Self {
        {
            E {
                a: 1_(),
                b: 1_(),
                c: 1_(),
            }
        }
    }
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
        ({ test_0(14_(), 10_(), 11_(), 12_(), 13_()) });
        ({
            test_1(
                ('e' as ()),
                ('a' as ()),
                ('b' as ()),
                ('c' as ()),
                ('d' as ()),
            )
        });
    }
    {
        ({ test_2(14_(), 10_(), 11_(), 12_(), 13_()) });
        ({
            test_3(
                ('e' as ()),
                ('a' as ()),
                ('b' as ()),
                ('c' as ()),
                ('d' as ()),
            )
        });
    }
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
