extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CopyAssign, Destructor, Default)]
#[byte_size(16)]
pub struct DestroyTracker {
    #[offset(0)]
    #[byte_size(8)]
    pub vec__: Ptr<()>,
    #[offset(8)]
    pub index_: (),
}
impl DestroyTracker {
    pub fn new(vec_: Ptr<()>) -> Self {
        let __this: DestroyTracker = Self {
            vec__: (vec_).clone(),
            index_: unimplemented!(),
        };
        unimplemented!();
        __this
    }
    pub fn copy_from(other: Ptr<DestroyTracker>) -> Self {
        let __this: DestroyTracker = Self {
            vec__: other.with(|__s| __s.vec__.clone()),
            index_: unimplemented!(),
        };
        unimplemented!();
        __this
    }
}
impl Clone for DestroyTracker {
    fn clone(&self) -> Self {
        let __src: Value<DestroyTracker> = Rc::new(RefCell::new(DestroyTracker {
            vec__: self.vec__.clone(),
            index_: self.index_.clone(),
        }));
        DestroyTracker::copy_from(__src.as_pointer())
    }
}
pub fn test_0(operation: FnPtr<fn(Ptr<()>, Ptr<()>)>) {
    let operation: Value<FnPtr<fn(Ptr<()>, Ptr<()>)>> = Rc::new(RefCell::new(operation));
    let all_destroyed: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    {
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut i: () = 0_();
        'loop_: while (i != 100_()) {
            ({ (*operation.borrow()).call(v.as_pointer(), all_destroyed.as_pointer()) });
            i.prefix_inc();
        }
    }
    unimplemented!();
}
pub fn test_1(operation: FnPtr<fn(Ptr<()>, Ptr<()>)>) {
    let operation: Value<FnPtr<fn(Ptr<()>, Ptr<()>)>> = Rc::new(RefCell::new(operation));
    let all_destroyed: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    {
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut i: () = 0_();
        'loop_: while (i != 100_()) {
            ({ (*operation.borrow()).call(v.as_pointer(), all_destroyed.as_pointer()) });
            i.prefix_inc();
        }
    }
    unimplemented!();
}
pub fn test_2(operation: FnPtr<fn(Ptr<()>, Ptr<()>)>) {
    let operation: Value<FnPtr<fn(Ptr<()>, Ptr<()>)>> = Rc::new(RefCell::new(operation));
    let all_destroyed: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    {
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut i: () = 0_();
        'loop_: while (i != 100_()) {
            ({ (*operation.borrow()).call(v.as_pointer(), all_destroyed.as_pointer()) });
            i.prefix_inc();
        }
    }
    unimplemented!();
}
pub fn test_3(operation: FnPtr<fn(Ptr<()>, Ptr<()>)>) {
    let operation: Value<FnPtr<fn(Ptr<()>, Ptr<()>)>> = Rc::new(RefCell::new(operation));
    let all_destroyed: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    {
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut i: () = 0_();
        'loop_: while (i != 100_()) {
            ({ (*operation.borrow()).call(v.as_pointer(), all_destroyed.as_pointer()) });
            i.prefix_inc();
        }
    }
    unimplemented!();
}
pub fn test_4(operation: FnPtr<fn(Ptr<()>, Ptr<()>)>) {
    let operation: Value<FnPtr<fn(Ptr<()>, Ptr<()>)>> = Rc::new(RefCell::new(operation));
    let all_destroyed: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    {
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut i: () = 0_();
        'loop_: while (i != 100_()) {
            ({ (*operation.borrow()).call(v.as_pointer(), all_destroyed.as_pointer()) });
            i.prefix_inc();
        }
    }
    unimplemented!();
}
pub fn test_5(operation: FnPtr<fn(Ptr<()>, Ptr<()>)>) {
    let operation: Value<FnPtr<fn(Ptr<()>, Ptr<()>)>> = Rc::new(RefCell::new(operation));
    let all_destroyed: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    {
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut i: () = 0_();
        'loop_: while (i != 100_()) {
            ({ (*operation.borrow()).call(v.as_pointer(), all_destroyed.as_pointer()) });
            i.prefix_inc();
        }
    }
    unimplemented!();
}
pub fn test_6(operation: FnPtr<fn(Ptr<()>, Ptr<()>)>) {
    let operation: Value<FnPtr<fn(Ptr<()>, Ptr<()>)>> = Rc::new(RefCell::new(operation));
    let all_destroyed: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    {
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut i: () = 0_();
        'loop_: while (i != 100_()) {
            ({ (*operation.borrow()).call(v.as_pointer(), all_destroyed.as_pointer()) });
            i.prefix_inc();
        }
    }
    unimplemented!();
}
pub fn test_7() -> () {
    ({
        test_0(FnPtr::<fn(Ptr<()>, Ptr<()>)>::new(
            |vec_: Ptr<()>, tracker: Ptr<()>| {
                unimplemented!();
            },
        ))
    });
    ({
        test_1(FnPtr::<fn(Ptr<()>, Ptr<()>)>::new(
            |vec_: Ptr<()>, tracker: Ptr<()>| {
                unimplemented!();
            },
        ))
    });
    ({
        test_2(FnPtr::<fn(Ptr<()>, Ptr<()>)>::new(
            |vec_: Ptr<()>, tracker: Ptr<()>| {
                unimplemented!();
            },
        ))
    });
    ({
        test_3(FnPtr::<fn(Ptr<()>, Ptr<()>)>::new(
            |vec_: Ptr<()>, tracker: Ptr<()>| {
                unimplemented!();
            },
        ))
    });
    ({
        test_4(FnPtr::<fn(Ptr<()>, Ptr<()>)>::new(
            |vec_: Ptr<()>, tracker: Ptr<()>| {
                unimplemented!();
            },
        ))
    });
    ({
        test_5(FnPtr::<fn(Ptr<()>, Ptr<()>)>::new(
            |vec_: Ptr<()>, tracker: Ptr<()>| {
                unimplemented!();
            },
        ))
    });
    ({
        test_6(FnPtr::<fn(Ptr<()>, Ptr<()>)>::new(
            |vec_: Ptr<()>, tracker: Ptr<()>| {
                unimplemented!();
            },
        ))
    });
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
    ({ test_7() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub trait DestroyTrackerImpl {
    fn copy_assign(&self, _a0: Ptr<DestroyTracker>) -> Ptr<DestroyTracker>;
    fn destructor(&self);
}
impl DestroyTrackerImpl for Ptr<DestroyTracker> {
    fn copy_assign(&self, _a0: Ptr<DestroyTracker>) -> Ptr<DestroyTracker> {
        return (*self).clone();
    }
    fn destructor(&self) {
        unimplemented!() = true;
    }
}
pub fn __cpp2rust_init_globals() {}
