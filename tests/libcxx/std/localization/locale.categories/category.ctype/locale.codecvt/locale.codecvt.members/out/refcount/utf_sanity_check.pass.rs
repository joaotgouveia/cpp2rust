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
    let l: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let f32_8: Ptr<()> = ({ use_facet_0(unimplemented!()) });
    let f32_16: Ptr<()> = ({ use_facet_1(l.as_pointer()) });
    let f16_8: Ptr<()> = ({ use_facet_2(unimplemented!()) });
    let mbs32_8: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mbs16_8: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mbs32_16: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let c32p: Value<Ptr<()>> = Rc::new(RefCell::new(Ptr::<()>::null()));
    let mut c16p: Ptr<()> = Ptr::<()>::null();
    let mut c8p: Ptr<()> = Ptr::<()>::null();
    let c_c32p: Value<Ptr<()>> = Rc::new(RefCell::new(Ptr::<()>::null()));
    let mut c_c16p: Ptr<()> = Ptr::<()>::null();
    let mut c_c8p: Ptr<()> = Ptr::<()>::null();
    let c32: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let c16: Value<Box<[()]>> = Rc::new(RefCell::new(
        (0..2).map(|_| unimplemented!()).collect::<Box<[()]>>(),
    ));
    let c16c: Value<Box<[()]>> = Rc::new(RefCell::new(
        (0..4).map(|_| unimplemented!()).collect::<Box<[()]>>(),
    ));
    let c16cp: Value<Ptr<()>> = Rc::new(RefCell::new(Ptr::<()>::null()));
    let c8: Value<Box<[()]>> = Rc::new(RefCell::new(
        (0..4).map(|_| unimplemented!()).collect::<Box<[()]>>(),
    ));
    let f32_8t: Ptr<()> = ({ use_facet_3(unimplemented!()) });
    let f16_8t: Ptr<()> = ({ use_facet_4(unimplemented!()) });
    let mbs32_8t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mbs16_8t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut c8tp: Ptr<()> = Ptr::<()>::null();
    let mut c_c8tp: Ptr<()> = Ptr::<()>::null();
    let c8t: Value<Box<[()]>> = Rc::new(RefCell::new(
        (0..4).map(|_| unimplemented!()).collect::<Box<[()]>>(),
    ));
    let c32x: Value<()> = Rc::new(RefCell::new(0_()));
    'loop_: while ((*c32x.borrow()) < 1114115_()) {
        if ((55296_() <= (*c32x.borrow())) && ((*c32x.borrow()) < 57344_()))
            || ((*c32x.borrow()) >= 1114112_())
        {
            unimplemented!();
            unimplemented!();
            unimplemented!();
        } else {
            unimplemented!();
            unimplemented!();
            if ((*c32x.borrow()) < 65536_()) {
                unimplemented!();
            } else {
                unimplemented!();
            }
            let mut i: () = 0_();
            'loop_: while ({ i } < {
                ((((*c16cp.borrow()).clone() - (c16c.as_pointer() as Ptr<()>)) as ()) / 2_())
            }) {
                (*c16.borrow_mut())[(i) as usize] = {
                    (((*c16c.borrow())[(2_() * i) as usize] << 8_())
                        | (*c16c.borrow())[((2_() * i) + 1_()) as usize])
                };
                i.prefix_inc();
            }
            c_c16p = (c16.as_pointer() as Ptr<()>).offset(
                ((((*c16cp.borrow()).clone() - (c16c.as_pointer() as Ptr<()>)) as ()) / 2_())
                    as isize,
            );
            unimplemented!();
            if ((*c32x.borrow()) < 65536_()) {
                unimplemented!();
            } else {
                unimplemented!();
            }
            if ((*c32x.borrow()) < 128_()) {
                unimplemented!();
            } else if ((*c32x.borrow()) < 2048_()) {
                unimplemented!();
            } else if ((*c32x.borrow()) < 65536_()) {
                unimplemented!();
            } else {
                unimplemented!();
            }
            c_c8p = (c8p).clone();
            unimplemented!();
            if ((*c32x.borrow()) < 128_()) {
                unimplemented!();
            } else if ((*c32x.borrow()) < 2048_()) {
                unimplemented!();
            } else if ((*c32x.borrow()) < 65536_()) {
                unimplemented!();
            } else {
                unimplemented!();
            }
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            if ((*c32x.borrow()) < 128_()) {
                unimplemented!();
            } else if ((*c32x.borrow()) < 2048_()) {
                unimplemented!();
            } else if ((*c32x.borrow()) < 65536_()) {
                unimplemented!();
            } else {
                unimplemented!();
            }
            c_c8p = (c8p).clone();
            unimplemented!();
            if ((*c32x.borrow()) < 128_()) {
                unimplemented!();
            } else if ((*c32x.borrow()) < 2048_()) {
                unimplemented!();
            } else if ((*c32x.borrow()) < 65536_()) {
                unimplemented!();
            } else {
                unimplemented!();
            }
            if ((*c32x.borrow()) < 65536_()) {
                unimplemented!();
            } else {
                unimplemented!();
            }
            let mut i: () = 0_();
            'loop_: while ({ i } < { ((c16p).clone() - (c16.as_pointer() as Ptr<()>)) as () }) {
                (*c16c.borrow_mut())[(2_() * i) as usize] =
                    { ((*c16.borrow())[(i) as usize] >> 8_()) };
                (*c16c.borrow_mut())[((2_() * i) + 1_()) as usize] =
                    { (*c16.borrow())[(i) as usize] };
                i.prefix_inc();
            }
            let c_c16cp: Value<Ptr<()>> =
                Rc::new(RefCell::new((c16c.as_pointer() as Ptr<()>).offset(
                    ((((c16p).clone() - (c16.as_pointer() as Ptr<()>)) as ()) * 2_()) as isize,
                )));
            unimplemented!();
            if ((*c32x.borrow()) < 65536_()) {
                unimplemented!();
            } else {
                unimplemented!();
            }
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            if ((*c32x.borrow()) < 128_()) {
                unimplemented!();
            } else if ((*c32x.borrow()) < 2048_()) {
                unimplemented!();
            } else if ((*c32x.borrow()) < 65536_()) {
                unimplemented!();
            } else {
                unimplemented!();
            }
            c_c8tp = (c8tp).clone();
            unimplemented!();
            if ((*c32x.borrow()) < 128_()) {
                unimplemented!();
            } else if ((*c32x.borrow()) < 2048_()) {
                unimplemented!();
            } else if ((*c32x.borrow()) < 65536_()) {
                unimplemented!();
            } else {
                unimplemented!();
            }
            if ((*c32x.borrow()) < 65536_()) {
                unimplemented!();
            } else {
                unimplemented!();
            }
            c_c16p = (c16p).clone();
            unimplemented!();
            if ((*c32x.borrow()) < 65536_()) {
                unimplemented!();
            } else {
                unimplemented!();
            }
            if ((*c32x.borrow()) < 128_()) {
                unimplemented!();
            } else if ((*c32x.borrow()) < 2048_()) {
                unimplemented!();
            } else if ((*c32x.borrow()) < 65536_()) {
                unimplemented!();
            } else {
                unimplemented!();
            }
            c_c8tp = (c8tp).clone();
            unimplemented!();
            if ((*c32x.borrow()) < 128_()) {
                unimplemented!();
            } else if ((*c32x.borrow()) < 2048_()) {
                unimplemented!();
            } else if ((*c32x.borrow()) < 65536_()) {
                unimplemented!();
            } else {
                unimplemented!();
            }
            unimplemented!();
            unimplemented!();
        }
        (*c32x.borrow_mut()).prefix_inc();
    }
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
