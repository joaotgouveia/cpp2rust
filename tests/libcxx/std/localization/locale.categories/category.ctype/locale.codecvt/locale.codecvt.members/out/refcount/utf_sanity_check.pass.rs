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
    let l: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let f32_8: Ptr<()> = unimplemented!();
    let f32_16: Ptr<()> = unimplemented!();
    let f16_8: Ptr<()> = unimplemented!();
    let mbs32_8: Value<::libc::mbstate_t> = Rc::new(RefCell::new(unsafe { std::mem::zeroed() }));
    let mbs16_8: Value<::libc::mbstate_t> = Rc::new(RefCell::new(unsafe { std::mem::zeroed() }));
    let mbs32_16: Value<::libc::mbstate_t> = Rc::new(RefCell::new(unsafe { std::mem::zeroed() }));
    let mut c32p: Ptr<()> = Ptr::<()>::null();
    let mut c16p: Ptr<()> = Ptr::<()>::null();
    let mut c8p: Ptr<()> = Ptr::<()>::null();
    let mut c_c32p: Ptr<()> = Ptr::<()>::null();
    let mut c_c16p: Ptr<()> = Ptr::<()>::null();
    let mut c_c8p: Ptr<()> = Ptr::<()>::null();
    let c32: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let c16: Value<Box<[()]>> = Rc::new(RefCell::new(
        (0..2).map(|_| unimplemented!()).collect::<Box<[()]>>(),
    ));
    let c16c: Value<Box<[i8]>> = Rc::new(RefCell::new((0..4).map(|_| 0_i8).collect::<Box<[i8]>>()));
    let mut c16cp: Ptr<i8> = Ptr::<i8>::null();
    let c8: Value<Box<[()]>> = Rc::new(RefCell::new(
        (0..4).map(|_| unimplemented!()).collect::<Box<[()]>>(),
    ));
    let f32_8t: Ptr<()> = unimplemented!();
    let f16_8t: Ptr<()> = unimplemented!();
    let mbs32_8t: Value<::libc::mbstate_t> = Rc::new(RefCell::new(unsafe { std::mem::zeroed() }));
    let mbs16_8t: Value<::libc::mbstate_t> = Rc::new(RefCell::new(unsafe { std::mem::zeroed() }));
    let mut c8tp: Ptr<()> = Ptr::<()>::null();
    let mut c_c8tp: Ptr<()> = Ptr::<()>::null();
    let c8t: Value<Box<[()]>> = Rc::new(RefCell::new(
        (0..4).map(|_| unimplemented!()).collect::<Box<[()]>>(),
    ));
    let c32x: Value<()> = Rc::new(RefCell::new(0_()));
    'loop_: while (((*c32x.borrow()) as u32) < 1114115_u32) {
        if ((55296_u32 <= ((*c32x.borrow()) as u32)) && (((*c32x.borrow()) as u32) < 57344_u32))
            || (((*c32x.borrow()) as u32) >= 1114112_u32)
        {
            assert!(((unimplemented!() as i32) == (unimplemented!() as i32)));
            assert!(((unimplemented!() as i32) == (unimplemented!() as i32)));
            assert!(((unimplemented!() as i32) == (unimplemented!() as i32)));
        } else {
            assert!(((unimplemented!() as i32) == (unimplemented!() as i32)));
            assert!((((c_c32p).clone() - (c32x.as_pointer())) as i64 == 1_i64));
            if (((*c32x.borrow()) as u32) < 65536_u32) {
                assert!((((c16cp).clone() - (c16c.as_pointer() as Ptr::<i8>)) as i64 == 2_i64));
            } else {
                assert!((((c16cp).clone() - (c16c.as_pointer() as Ptr::<i8>)) as i64 == 4_i64));
            }
            let mut i: i32 = 0;
            'loop_: while ({ (i as i64) } < {
                ((((c16cp).clone() - (c16c.as_pointer() as Ptr<i8>)) as i64) / 2_i64)
            }) {
                (*c16.borrow_mut())[(i) as usize] = {
                    (((((((*c16c.borrow())[(2 * i) as usize] as u8) as i32) << 8)
                        | (((*c16c.borrow())[((2 * i) + 1) as usize] as u8) as i32))
                        as u16) as ())
                };
                i.prefix_inc();
            }
            c_c16p = (c16.as_pointer() as Ptr<()>).offset(
                ((((c16cp).clone() - (c16c.as_pointer() as Ptr<i8>)) as i64) / 2_i64) as isize,
            );
            assert!(((unimplemented!() as i32) == (unimplemented!() as i32)));
            if (((*c32x.borrow()) as u32) < 65536_u32) {
                assert!((((c_c16p).clone() - (c16.as_pointer() as Ptr::<()>)) as i64 == 1_i64));
            } else {
                assert!((((c_c16p).clone() - (c16.as_pointer() as Ptr::<()>)) as i64 == 2_i64));
            }
            if (((*c32x.borrow()) as u32) < 128_u32) {
                assert!((((c8p).clone() - (c8.as_pointer() as Ptr::<()>)) as i64 == 1_i64));
            } else if (((*c32x.borrow()) as u32) < 2048_u32) {
                assert!((((c8p).clone() - (c8.as_pointer() as Ptr::<()>)) as i64 == 2_i64));
            } else if (((*c32x.borrow()) as u32) < 65536_u32) {
                assert!((((c8p).clone() - (c8.as_pointer() as Ptr::<()>)) as i64 == 3_i64));
            } else {
                assert!((((c8p).clone() - (c8.as_pointer() as Ptr::<()>)) as i64 == 4_i64));
            }
            c_c8p = (c8p).clone();
            assert!(((unimplemented!() as i32) == (unimplemented!() as i32)));
            if (((*c32x.borrow()) as u32) < 128_u32) {
                assert!((((c_c8p).clone() - (c8.as_pointer() as Ptr::<()>)) as i64 == 1_i64));
            } else if (((*c32x.borrow()) as u32) < 2048_u32) {
                assert!((((c_c8p).clone() - (c8.as_pointer() as Ptr::<()>)) as i64 == 2_i64));
            } else if (((*c32x.borrow()) as u32) < 65536_u32) {
                assert!((((c_c8p).clone() - (c8.as_pointer() as Ptr::<()>)) as i64 == 3_i64));
            } else {
                assert!((((c_c8p).clone() - (c8.as_pointer() as Ptr::<()>)) as i64 == 4_i64));
            }
            assert!((((c32p).clone() - (c32.as_pointer())) as i64 == 1_i64));
            assert!((((*c32.borrow()) as u32) == ((*c32x.borrow()) as u32)));
            assert!(((unimplemented!() as i32) == (unimplemented!() as i32)));
            assert!((((c_c32p).clone() - (c32x.as_pointer())) as i64 == 1_i64));
            if (((*c32x.borrow()) as u32) < 128_u32) {
                assert!((((c8p).clone() - (c8.as_pointer() as Ptr::<()>)) as i64 == 1_i64));
            } else if (((*c32x.borrow()) as u32) < 2048_u32) {
                assert!((((c8p).clone() - (c8.as_pointer() as Ptr::<()>)) as i64 == 2_i64));
            } else if (((*c32x.borrow()) as u32) < 65536_u32) {
                assert!((((c8p).clone() - (c8.as_pointer() as Ptr::<()>)) as i64 == 3_i64));
            } else {
                assert!((((c8p).clone() - (c8.as_pointer() as Ptr::<()>)) as i64 == 4_i64));
            }
            c_c8p = (c8p).clone();
            assert!(((unimplemented!() as i32) == (unimplemented!() as i32)));
            if (((*c32x.borrow()) as u32) < 128_u32) {
                assert!((((c_c8p).clone() - (c8.as_pointer() as Ptr::<()>)) as i64 == 1_i64));
            } else if (((*c32x.borrow()) as u32) < 2048_u32) {
                assert!((((c_c8p).clone() - (c8.as_pointer() as Ptr::<()>)) as i64 == 2_i64));
            } else if (((*c32x.borrow()) as u32) < 65536_u32) {
                assert!((((c_c8p).clone() - (c8.as_pointer() as Ptr::<()>)) as i64 == 3_i64));
            } else {
                assert!((((c_c8p).clone() - (c8.as_pointer() as Ptr::<()>)) as i64 == 4_i64));
            }
            if (((*c32x.borrow()) as u32) < 65536_u32) {
                assert!((((c16p).clone() - (c16.as_pointer() as Ptr::<()>)) as i64 == 1_i64));
            } else {
                assert!((((c16p).clone() - (c16.as_pointer() as Ptr::<()>)) as i64 == 2_i64));
            }
            let mut i: i32 = 0;
            'loop_: while ({ (i as i64) } < {
                ((c16p).clone() - (c16.as_pointer() as Ptr<()>)) as i64
            }) {
                (*c16c.borrow_mut())[(2 * i) as usize] =
                    { ((((*c16.borrow())[(i) as usize] as i32) >> 8) as i8) };
                (*c16c.borrow_mut())[((2 * i) + 1) as usize] =
                    { ((*c16.borrow())[(i) as usize] as i8) };
                i.prefix_inc();
            }
            let mut c_c16cp: Ptr<i8> = (c16c.as_pointer() as Ptr<i8>).offset(
                ((((c16p).clone() - (c16.as_pointer() as Ptr<()>)) as i64) * 2_i64) as isize,
            );
            assert!(((unimplemented!() as i32) == (unimplemented!() as i32)));
            if (((*c32x.borrow()) as u32) < 65536_u32) {
                assert!((((c_c16cp).clone() - (c16c.as_pointer() as Ptr::<i8>)) as i64 == 2_i64));
            } else {
                assert!((((c_c16cp).clone() - (c16c.as_pointer() as Ptr::<i8>)) as i64 == 4_i64));
            }
            assert!((((c32p).clone() - (c32.as_pointer())) as i64 == 1_i64));
            assert!((((*c32.borrow()) as u32) == ((*c32x.borrow()) as u32)));
            assert!(((unimplemented!() as i32) == (unimplemented!() as i32)));
            assert!((((c_c32p).clone() - (c32x.as_pointer())) as i64 == 1_i64));
            if (((*c32x.borrow()) as u32) < 128_u32) {
                assert!((((c8tp).clone() - (c8t.as_pointer() as Ptr::<()>)) as i64 == 1_i64));
            } else if (((*c32x.borrow()) as u32) < 2048_u32) {
                assert!((((c8tp).clone() - (c8t.as_pointer() as Ptr::<()>)) as i64 == 2_i64));
            } else if (((*c32x.borrow()) as u32) < 65536_u32) {
                assert!((((c8tp).clone() - (c8t.as_pointer() as Ptr::<()>)) as i64 == 3_i64));
            } else {
                assert!((((c8tp).clone() - (c8t.as_pointer() as Ptr::<()>)) as i64 == 4_i64));
            }
            c_c8tp = (c8tp).clone();
            assert!(((unimplemented!() as i32) == (unimplemented!() as i32)));
            if (((*c32x.borrow()) as u32) < 128_u32) {
                assert!((((c_c8tp).clone() - (c8t.as_pointer() as Ptr::<()>)) as i64 == 1_i64));
            } else if (((*c32x.borrow()) as u32) < 2048_u32) {
                assert!((((c_c8tp).clone() - (c8t.as_pointer() as Ptr::<()>)) as i64 == 2_i64));
            } else if (((*c32x.borrow()) as u32) < 65536_u32) {
                assert!((((c_c8tp).clone() - (c8t.as_pointer() as Ptr::<()>)) as i64 == 3_i64));
            } else {
                assert!((((c_c8tp).clone() - (c8t.as_pointer() as Ptr::<()>)) as i64 == 4_i64));
            }
            if (((*c32x.borrow()) as u32) < 65536_u32) {
                assert!((((c16p).clone() - (c16.as_pointer() as Ptr::<()>)) as i64 == 1_i64));
            } else {
                assert!((((c16p).clone() - (c16.as_pointer() as Ptr::<()>)) as i64 == 2_i64));
            }
            c_c16p = (c16p).clone();
            assert!(((unimplemented!() as i32) == (unimplemented!() as i32)));
            if (((*c32x.borrow()) as u32) < 65536_u32) {
                assert!((((c_c16p).clone() - (c16.as_pointer() as Ptr::<()>)) as i64 == 1_i64));
            } else {
                assert!((((c_c16p).clone() - (c16.as_pointer() as Ptr::<()>)) as i64 == 2_i64));
            }
            if (((*c32x.borrow()) as u32) < 128_u32) {
                assert!((((c8tp).clone() - (c8t.as_pointer() as Ptr::<()>)) as i64 == 1_i64));
            } else if (((*c32x.borrow()) as u32) < 2048_u32) {
                assert!((((c8tp).clone() - (c8t.as_pointer() as Ptr::<()>)) as i64 == 2_i64));
            } else if (((*c32x.borrow()) as u32) < 65536_u32) {
                assert!((((c8tp).clone() - (c8t.as_pointer() as Ptr::<()>)) as i64 == 3_i64));
            } else {
                assert!((((c8tp).clone() - (c8t.as_pointer() as Ptr::<()>)) as i64 == 4_i64));
            }
            c_c8tp = (c8tp).clone();
            assert!(((unimplemented!() as i32) == (unimplemented!() as i32)));
            if (((*c32x.borrow()) as u32) < 128_u32) {
                assert!((((c_c8tp).clone() - (c8t.as_pointer() as Ptr::<()>)) as i64 == 1_i64));
            } else if (((*c32x.borrow()) as u32) < 2048_u32) {
                assert!((((c_c8tp).clone() - (c8t.as_pointer() as Ptr::<()>)) as i64 == 2_i64));
            } else if (((*c32x.borrow()) as u32) < 65536_u32) {
                assert!((((c_c8tp).clone() - (c8t.as_pointer() as Ptr::<()>)) as i64 == 3_i64));
            } else {
                assert!((((c_c8tp).clone() - (c8t.as_pointer() as Ptr::<()>)) as i64 == 4_i64));
            }
            assert!((((c32p).clone() - (c32.as_pointer())) as i64 == 1_i64));
            assert!((((*c32.borrow()) as u32) == ((*c32x.borrow()) as u32)));
        }
        (*c32x.borrow_mut()).prefix_inc();
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
