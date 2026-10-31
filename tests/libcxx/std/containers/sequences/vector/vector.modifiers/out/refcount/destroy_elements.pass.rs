extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CppFrom, CopyAssign, Destructor, Default)]
#[cpp_from(Vec<bool>  , new, mut_ptr)]
#[byte_size(16)]
pub struct DestroyTracker {
    #[offset(0)]
    #[byte_size(8)]
    pub vec__: Ptr<Vec<bool>>,
    #[offset(8)]
    pub index_: usize,
}
impl DestroyTracker {
    pub fn new(vec_: Ptr<Vec<bool>>) -> Self {
        let __this: DestroyTracker = Self {
            vec__: (vec_).clone(),
            index_: (&(*vec_.upgrade().deref())).len(),
        };
        unimplemented!();
        __this
    }
    pub fn copy_from(other: Ptr<DestroyTracker>) -> Self {
        let __this: Value<DestroyTracker> = Rc::new(RefCell::new(Self {
            vec__: other.with(|__s| __s.vec__.clone()),
            index_: (&(*this.with(|__s| __s.vec__.clone()).upgrade().deref())).len(),
        }));
        let this: Ptr<DestroyTracker> = __this.as_pointer();
        unimplemented!();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
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
pub fn test_0(operation: FnPtr<fn(Ptr<Vec<DestroyTracker>>, Ptr<Vec<bool>>)>) {
    let operation: Value<FnPtr<fn(Ptr<Vec<DestroyTracker>>, Ptr<Vec<bool>>)>> =
        Rc::new(RefCell::new(operation));
    let all_destroyed: Value<Vec<bool>> = Rc::new(RefCell::new(unimplemented!()));
    {
        let v: Value<Vec<DestroyTracker>> = Rc::new(RefCell::new(Vec::new()));
        let mut i: usize = 0_usize;
        'loop_: while (i != 100_usize) {
            ({ (*operation.borrow()).call(v.as_pointer(), all_destroyed.as_pointer()) });
            i.prefix_inc();
        }
    }
    assert!(unimplemented!());
}
pub fn test_1(operation: FnPtr<fn(Ptr<Vec<DestroyTracker>>, Ptr<Vec<bool>>)>) {
    let operation: Value<FnPtr<fn(Ptr<Vec<DestroyTracker>>, Ptr<Vec<bool>>)>> =
        Rc::new(RefCell::new(operation));
    let all_destroyed: Value<Vec<bool>> = Rc::new(RefCell::new(unimplemented!()));
    {
        let v: Value<Vec<DestroyTracker>> = Rc::new(RefCell::new(Vec::new()));
        let mut i: usize = 0_usize;
        'loop_: while (i != 100_usize) {
            ({ (*operation.borrow()).call(v.as_pointer(), all_destroyed.as_pointer()) });
            i.prefix_inc();
        }
    }
    assert!(unimplemented!());
}
pub fn test_2(operation: FnPtr<fn(Ptr<Vec<DestroyTracker>>, Ptr<Vec<bool>>)>) {
    let operation: Value<FnPtr<fn(Ptr<Vec<DestroyTracker>>, Ptr<Vec<bool>>)>> =
        Rc::new(RefCell::new(operation));
    let all_destroyed: Value<Vec<bool>> = Rc::new(RefCell::new(unimplemented!()));
    {
        let v: Value<Vec<DestroyTracker>> = Rc::new(RefCell::new(Vec::new()));
        let mut i: usize = 0_usize;
        'loop_: while (i != 100_usize) {
            ({ (*operation.borrow()).call(v.as_pointer(), all_destroyed.as_pointer()) });
            i.prefix_inc();
        }
    }
    assert!(unimplemented!());
}
pub fn test_3(operation: FnPtr<fn(Ptr<Vec<DestroyTracker>>, Ptr<Vec<bool>>)>) {
    let operation: Value<FnPtr<fn(Ptr<Vec<DestroyTracker>>, Ptr<Vec<bool>>)>> =
        Rc::new(RefCell::new(operation));
    let all_destroyed: Value<Vec<bool>> = Rc::new(RefCell::new(unimplemented!()));
    {
        let v: Value<Vec<DestroyTracker>> = Rc::new(RefCell::new(Vec::new()));
        let mut i: usize = 0_usize;
        'loop_: while (i != 100_usize) {
            ({ (*operation.borrow()).call(v.as_pointer(), all_destroyed.as_pointer()) });
            i.prefix_inc();
        }
    }
    assert!(unimplemented!());
}
pub fn test_4(operation: FnPtr<fn(Ptr<Vec<DestroyTracker>>, Ptr<Vec<bool>>)>) {
    let operation: Value<FnPtr<fn(Ptr<Vec<DestroyTracker>>, Ptr<Vec<bool>>)>> =
        Rc::new(RefCell::new(operation));
    let all_destroyed: Value<Vec<bool>> = Rc::new(RefCell::new(unimplemented!()));
    {
        let v: Value<Vec<DestroyTracker>> = Rc::new(RefCell::new(Vec::new()));
        let mut i: usize = 0_usize;
        'loop_: while (i != 100_usize) {
            ({ (*operation.borrow()).call(v.as_pointer(), all_destroyed.as_pointer()) });
            i.prefix_inc();
        }
    }
    assert!(unimplemented!());
}
pub fn test_5(operation: FnPtr<fn(Ptr<Vec<DestroyTracker>>, Ptr<Vec<bool>>)>) {
    let operation: Value<FnPtr<fn(Ptr<Vec<DestroyTracker>>, Ptr<Vec<bool>>)>> =
        Rc::new(RefCell::new(operation));
    let all_destroyed: Value<Vec<bool>> = Rc::new(RefCell::new(unimplemented!()));
    {
        let v: Value<Vec<DestroyTracker>> = Rc::new(RefCell::new(Vec::new()));
        let mut i: usize = 0_usize;
        'loop_: while (i != 100_usize) {
            ({ (*operation.borrow()).call(v.as_pointer(), all_destroyed.as_pointer()) });
            i.prefix_inc();
        }
    }
    assert!(unimplemented!());
}
pub fn test_6(operation: FnPtr<fn(Ptr<Vec<DestroyTracker>>, Ptr<Vec<bool>>)>) {
    let operation: Value<FnPtr<fn(Ptr<Vec<DestroyTracker>>, Ptr<Vec<bool>>)>> =
        Rc::new(RefCell::new(operation));
    let all_destroyed: Value<Vec<bool>> = Rc::new(RefCell::new(unimplemented!()));
    {
        let v: Value<Vec<DestroyTracker>> = Rc::new(RefCell::new(Vec::new()));
        let mut i: usize = 0_usize;
        'loop_: while (i != 100_usize) {
            ({ (*operation.borrow()).call(v.as_pointer(), all_destroyed.as_pointer()) });
            i.prefix_inc();
        }
    }
    assert!(unimplemented!());
}
pub fn test_7() -> bool {
    ({
        test_0(FnPtr::<fn(Ptr<Vec<DestroyTracker>>, Ptr<Vec<bool>>)>::new(
            |vec_: Ptr<Vec<DestroyTracker>>, tracker: Ptr<Vec<bool>>| {
                {
                    let __init = DestroyTracker::new({ (tracker).clone() });
                    vec_.with_mut(|__v: &mut Vec<DestroyTracker>| __v.push(__init))
                };
            },
        ))
    });
    ({
        test_1(FnPtr::<fn(Ptr<Vec<DestroyTracker>>, Ptr<Vec<bool>>)>::new(
            |vec_: Ptr<Vec<DestroyTracker>>, tracker: Ptr<Vec<bool>>| {
                {
                    let __a1 = DestroyTracker::new({ (tracker).clone() });
                    vec_.with_mut(|__v: &mut Vec<DestroyTracker>| __v.push(__a1))
                };
            },
        ))
    });
    ({
        test_2(FnPtr::<fn(Ptr<Vec<DestroyTracker>>, Ptr<Vec<bool>>)>::new(
            |vec_: Ptr<Vec<DestroyTracker>>, tracker: Ptr<Vec<bool>>| {
                unimplemented!();
            },
        ))
    });
    ({
        test_3(FnPtr::<fn(Ptr<Vec<DestroyTracker>>, Ptr<Vec<bool>>)>::new(
            |vec_: Ptr<Vec<DestroyTracker>>, tracker: Ptr<Vec<bool>>| {
                {
                    let __it = <RandomAccessIterator<DestroyTracker>>::from(vec_.decay().begin());
                    let __off = __it.index();
                    vec_.with_mut(|__v: &mut Vec<DestroyTracker>| {
                        __v.insert(__off, DestroyTracker::new({ (tracker).clone() }))
                    });
                    __it
                };
            },
        ))
    });
    ({
        test_4(FnPtr::<fn(Ptr<Vec<DestroyTracker>>, Ptr<Vec<bool>>)>::new(
            |vec_: Ptr<Vec<DestroyTracker>>, tracker: Ptr<Vec<bool>>| {
                {
                    let __a0 = ((*vec_.upgrade().deref()).len()).wrapping_add(1_usize) as usize;
                    vec_.with_mut(|__v: &mut Vec<DestroyTracker>| {
                        __v.resize(__a0, DestroyTracker::new({ (tracker).clone() }))
                    })
                };
            },
        ))
    });
    ({
        test_5(FnPtr::<fn(Ptr<Vec<DestroyTracker>>, Ptr<Vec<bool>>)>::new(
            |vec_: Ptr<Vec<DestroyTracker>>, tracker: Ptr<Vec<bool>>| {
                unimplemented!();
            },
        ))
    });
    ({
        test_6(FnPtr::<fn(Ptr<Vec<DestroyTracker>>, Ptr<Vec<bool>>)>::new(
            |vec_: Ptr<Vec<DestroyTracker>>, tracker: Ptr<Vec<bool>>| {
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> i32 {
    ({ test_7() });
    const _: () = assert!(true, "test()");;
    return 0;
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
        {
            ((*self).with(|__s| __s.vec__.clone()))
                .decay()
                .offset((*self).with(|__s| __s.index_) as isize)
                .write(true);
            ((*self).with(|__s| __s.vec__.clone()))
                .decay()
                .offset((*self).with(|__s| __s.index_) as isize)
        };
    }
}
pub fn __cpp2rust_init_globals() {}
