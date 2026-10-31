extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, Default)]
#[byte_size(1)]
pub struct Dummy {}
impl Dummy {
    pub fn move_from(_a0: Ptr<Dummy>) -> Self {
        Self {}
    }
}
#[derive(Record, ByteRepr, FnPtrArg, Default)]
#[byte_size(1)]
pub struct NotCopyOrMoveConstructible {}
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
        const _: () = assert!(true, "std::is_move_constructible<P1>::value");;
        let p1: Value<(Value<i32>, Value<i16>)> = Rc::new(RefCell::new((
            Rc::new(RefCell::new(3)),
            Rc::new(RefCell::new(4_i16)),
        )));
        let p2: Value<(Value<i32>, Value<i16>)> =
            Rc::new(RefCell::new(std::mem::take(&mut (*p1.borrow_mut()))));
        assert!(((*(*p2.borrow()).0.borrow()) == 3));
        assert!((((*(*p2.borrow()).1.borrow()) as i32) == 4));
    }
    {
        const _: () = assert!(true, "!std::is_copy_constructible<P>::value");;
        const _: () = assert!(true, "std::is_move_constructible<P>::value");;
    }
    {
        {
            const _: () = assert!(true, "std::is_move_constructible<P>::value");;
            let obj: Value<NotCopyOrMoveConstructible> =
                Rc::new(RefCell::new(<NotCopyOrMoveConstructible>::default()));
            let p2: Value<(Value<Ptr<NotCopyOrMoveConstructible>>, Value<i32>)> =
                Rc::new(RefCell::new((
                    Rc::new(RefCell::new((*obj.borrow()))),
                    Rc::new(RefCell::new(3)),
                )));
            let p1: Value<(Value<Ptr<NotCopyOrMoveConstructible>>, Value<i32>)> =
                Rc::new(RefCell::new(std::mem::take(&mut (*p2.borrow_mut()))));
            assert!(({ ((*p1.borrow()).0).clone() } == { (obj.as_pointer()) }));
            assert!(({ ((*p2.borrow()).0).clone() } == { (obj.as_pointer()) }));
        }
        {
            const _: () = assert!(true, "std::is_move_constructible<P>::value");;
            let obj: Value<NotCopyOrMoveConstructible> =
                Rc::new(RefCell::new(<NotCopyOrMoveConstructible>::default()));
            let p2: Value<(Value<Ptr<NotCopyOrMoveConstructible>>, Value<i32>)> =
                Rc::new(RefCell::new((
                    Rc::new(RefCell::new((*obj.borrow()))),
                    Rc::new(RefCell::new(3)),
                )));
            let p1: Value<(Value<Ptr<NotCopyOrMoveConstructible>>, Value<i32>)> =
                Rc::new(RefCell::new(std::mem::take(&mut (*p2.borrow_mut()))));
            assert!(({ ((*p1.borrow()).0).clone() } == { (obj.as_pointer()) }));
            assert!(({ ((*p2.borrow()).0).clone() } == { (obj.as_pointer()) }));
        }
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
