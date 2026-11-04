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
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "std::is_swappable_v<T>");;
    }
    {
        let t1: Value<Box<[Box<[f64]>]>> = Rc::new(RefCell::new(Box::new([
            Box::new(std::array::from_fn::<_, 29, _>(|_| Default::default())),
            Box::new(std::array::from_fn::<_, 29, _>(|_| Default::default())),
            Box::new(std::array::from_fn::<_, 29, _>(|_| Default::default())),
            Box::new(std::array::from_fn::<_, 29, _>(|_| Default::default())),
            Box::new(std::array::from_fn::<_, 29, _>(|_| Default::default())),
            Box::new(std::array::from_fn::<_, 29, _>(|_| Default::default())),
            Box::new(std::array::from_fn::<_, 29, _>(|_| Default::default())),
            Box::new(std::array::from_fn::<_, 29, _>(|_| Default::default())),
            Box::new(std::array::from_fn::<_, 29, _>(|_| Default::default())),
            Box::new(std::array::from_fn::<_, 29, _>(|_| Default::default())),
            Box::new(std::array::from_fn::<_, 29, _>(|_| Default::default())),
            Box::new(std::array::from_fn::<_, 29, _>(|_| Default::default())),
            Box::new(std::array::from_fn::<_, 29, _>(|_| Default::default())),
            Box::new(std::array::from_fn::<_, 29, _>(|_| Default::default())),
            Box::new(std::array::from_fn::<_, 29, _>(|_| Default::default())),
            Box::new(std::array::from_fn::<_, 29, _>(|_| Default::default())),
            Box::new(std::array::from_fn::<_, 29, _>(|_| Default::default())),
        ])));
        let t2: Value<Box<[Box<[f64]>]>> = Rc::new(RefCell::new(Box::new([
            Box::new(std::array::from_fn::<_, 29, _>(|_| Default::default())),
            Box::new(std::array::from_fn::<_, 29, _>(|_| Default::default())),
            Box::new(std::array::from_fn::<_, 29, _>(|_| Default::default())),
            Box::new(std::array::from_fn::<_, 29, _>(|_| Default::default())),
            Box::new(std::array::from_fn::<_, 29, _>(|_| Default::default())),
            Box::new(std::array::from_fn::<_, 29, _>(|_| Default::default())),
            Box::new(std::array::from_fn::<_, 29, _>(|_| Default::default())),
            Box::new(std::array::from_fn::<_, 29, _>(|_| Default::default())),
            Box::new(std::array::from_fn::<_, 29, _>(|_| Default::default())),
            Box::new(std::array::from_fn::<_, 29, _>(|_| Default::default())),
            Box::new(std::array::from_fn::<_, 29, _>(|_| Default::default())),
            Box::new(std::array::from_fn::<_, 29, _>(|_| Default::default())),
            Box::new(std::array::from_fn::<_, 29, _>(|_| Default::default())),
            Box::new(std::array::from_fn::<_, 29, _>(|_| Default::default())),
            Box::new(std::array::from_fn::<_, 29, _>(|_| Default::default())),
            Box::new(std::array::from_fn::<_, 29, _>(|_| Default::default())),
            Box::new(std::array::from_fn::<_, 29, _>(|_| Default::default())),
        ])));
        ({
            iter_swap_0(
                (t1.as_pointer() as Ptr<Box<[f64]>>),
                (t2.as_pointer() as Ptr<Box<[f64]>>),
            )
        });
        ({
            let ___first1: Ptr<Box<[f64]>> = (t1.as_pointer() as Ptr<Box<[f64]>>);
            let ___last1: Ptr<Box<[f64]>> =
                (t1.as_pointer() as Ptr<Box<[f64]>>).offset((17) as isize);
            swap_ranges_1(___first1, ___last1, (t2.as_pointer() as Ptr<Box<[f64]>>))
        });
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
