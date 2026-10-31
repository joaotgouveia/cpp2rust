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
        let s1: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
            ('1' as i8),
            ('2' as i8),
            ('3' as i8),
            0_i8,
        ])));
        let sm1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*((*sm1.borrow()) as (Value<*const i8>, Value<*const i8>))
            .0
            .borrow_mut()) = (s1.as_pointer() as Ptr<i8>);
        (*((*sm1.borrow()) as (Value<*const i8>, Value<*const i8>))
            .1
            .borrow_mut()) = (s1.as_pointer() as Ptr<i8>).offset((3) as isize);
        (*sm1.borrow_mut()).matched = true;
        let sm2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let s2: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
            ('c' as i8),
            ('a' as i8),
            ('t' as i8),
            0_i8,
        ])));
        (*((*sm2.borrow()) as (Value<*const i8>, Value<*const i8>))
            .0
            .borrow_mut()) = (s2.as_pointer() as Ptr<i8>);
        (*((*sm2.borrow()) as (Value<*const i8>, Value<*const i8>))
            .1
            .borrow_mut()) = (s2.as_pointer() as Ptr<i8>).offset((3) as isize);
        (*sm2.borrow_mut()).matched = false;
        unimplemented!();
        assert!(
            ({
                (*((*sm1.borrow()) as (Value<*const i8>, Value<*const i8>))
                    .0
                    .borrow())
                .clone()
            } == { (s2.as_pointer() as Ptr::<i8>) })
        );
        assert!(
            ({
                (*((*sm1.borrow()) as (Value<*const i8>, Value<*const i8>))
                    .1
                    .borrow())
                .clone()
            } == { (s2.as_pointer() as Ptr::<i8>).offset((3) as isize) })
        );
        assert!(!({ (*sm1.borrow()).matched }));
        assert!(
            ({
                (*((*sm2.borrow()) as (Value<*const i8>, Value<*const i8>))
                    .0
                    .borrow())
                .clone()
            } == { (s1.as_pointer() as Ptr::<i8>) })
        );
        assert!(
            ({
                (*((*sm2.borrow()) as (Value<*const i8>, Value<*const i8>))
                    .1
                    .borrow())
                .clone()
            } == { (s1.as_pointer() as Ptr::<i8>).offset((3) as isize) })
        );
        assert!({ (*sm2.borrow()).matched });
        const _: () = assert!(true, "");;
    }
    {
        let s1: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([
            (49 as i32),
            (50 as i32),
            (51 as i32),
            0_i32,
        ])));
        let sm1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*((*sm1.borrow()) as (Value<*const i32>, Value<*const i32>))
            .0
            .borrow_mut()) = (s1.as_pointer() as Ptr<i32>);
        (*((*sm1.borrow()) as (Value<*const i32>, Value<*const i32>))
            .1
            .borrow_mut()) = (s1.as_pointer() as Ptr<i32>).offset((3) as isize);
        (*sm1.borrow_mut()).matched = true;
        let sm2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let s2: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([
            (99 as i32),
            (97 as i32),
            (116 as i32),
            0_i32,
        ])));
        (*((*sm2.borrow()) as (Value<*const i32>, Value<*const i32>))
            .0
            .borrow_mut()) = (s2.as_pointer() as Ptr<i32>);
        (*((*sm2.borrow()) as (Value<*const i32>, Value<*const i32>))
            .1
            .borrow_mut()) = (s2.as_pointer() as Ptr<i32>).offset((3) as isize);
        (*sm2.borrow_mut()).matched = false;
        unimplemented!();
        assert!(
            ({
                (*((*sm1.borrow()) as (Value<*const i32>, Value<*const i32>))
                    .0
                    .borrow())
                .clone()
            } == { (s2.as_pointer() as Ptr::<i32>) })
        );
        assert!(
            ({
                (*((*sm1.borrow()) as (Value<*const i32>, Value<*const i32>))
                    .1
                    .borrow())
                .clone()
            } == { (s2.as_pointer() as Ptr::<i32>).offset((3) as isize) })
        );
        assert!(!({ (*sm1.borrow()).matched }));
        assert!(
            ({
                (*((*sm2.borrow()) as (Value<*const i32>, Value<*const i32>))
                    .0
                    .borrow())
                .clone()
            } == { (s1.as_pointer() as Ptr::<i32>) })
        );
        assert!(
            ({
                (*((*sm2.borrow()) as (Value<*const i32>, Value<*const i32>))
                    .1
                    .borrow())
                .clone()
            } == { (s1.as_pointer() as Ptr::<i32>).offset((3) as isize) })
        );
        assert!({ (*sm2.borrow()).matched });
        const _: () = assert!(true, "");;
    }
    0
}
pub fn __cpp2rust_init_globals() {}
