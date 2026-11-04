extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct Tracker {
    #[offset(0)]
    pub move_constructs: usize,
}
impl CppMove for Tracker {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for Tracker {
    fn default() -> Self {
        {
            Tracker {
                move_constructs: 0_usize,
            }
        }
    }
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, Default)]
#[byte_size(8)]
pub struct Inner {
    #[offset(0)]
    #[byte_size(8)]
    pub tracker_: Ptr<Tracker>,
}
impl Inner {
    pub fn new(mut tracker: Ptr<Tracker>) -> Self {
        Self {
            tracker_: (tracker).clone(),
        }
    }
    pub fn copy_from(rhs: Ptr<Inner>) -> Self {
        let __this: Value<Inner> = Rc::new(RefCell::new(Self {
            tracker_: rhs.with(|__s| __s.tracker_.clone()),
        }));
        let this: Ptr<Inner> = __this.as_pointer();
        field!(this.with(|__s| __s.tracker_.clone()), move_constructs).write({
            (this
                .with(|__s| __s.tracker_.clone())
                .with(|__s| __s.move_constructs))
            .wrapping_add(1_usize)
        });
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn move_from(rhs: Ptr<Inner>) -> Self {
        let __this: Value<Inner> = Rc::new(RefCell::new(Self {
            tracker_: rhs.with(|__s| __s.tracker_.clone()),
        }));
        let this: Ptr<Inner> = __this.as_pointer();
        field!(this.with(|__s| __s.tracker_.clone()), move_constructs).write({
            (this
                .with(|__s| __s.tracker_.clone())
                .with(|__s| __s.move_constructs))
            .wrapping_add(1_usize)
        });
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for Inner {
    fn clone(&self) -> Self {
        let __src: Value<Inner> = Rc::new(RefCell::new(Inner {
            tracker_: self.tracker_.clone(),
        }));
        Inner::copy_from(__src.as_pointer())
    }
}
impl CppMove for Inner {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, Default)]
#[byte_size(8)]
pub struct NotTriviallyMovable {
    #[offset(0)]
    #[byte_size(8)]
    pub inner_: Inner,
}
impl NotTriviallyMovable {
    pub fn new(mut tracker: Ptr<Tracker>) -> Self {
        Self {
            inner_: Inner::new({ (tracker).clone() }),
        }
    }
    pub fn move_from(other: Ptr<NotTriviallyMovable>) -> Self {
        Self {
            inner_: Inner::move_from({ field_ptr!(other, inner_) }),
        }
    }
}
impl CppMove for NotTriviallyMovable {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
const _: () = assert!(
    true,
    "!std::is_trivially_copyable<NotTriviallyMovable>::value"
);
const _: () = assert!(true, "");
pub fn tests_0() -> bool {
    let track: Value<Tracker> = Rc::new(RefCell::new(<Tracker>::default()));
    let v: Value<Vec<NotTriviallyMovable>> = Rc::new(RefCell::new(Vec::new()));
    if 5_usize as usize > (*v.borrow()).capacity() as usize {
        let len_0 = (*v.borrow()).len();
        (*v.borrow_mut()).reserve_exact(5_usize as usize - len_0 as usize);
    };
    let mut capacity: usize = (*v.borrow()).capacity();
    'loop_: while ((*v.borrow()).len() < (*v.borrow()).capacity()) {
        {
            let __init = NotTriviallyMovable::new({ (track.as_pointer()) });
            v.as_pointer()
                .with_mut(|__v: &mut Vec<NotTriviallyMovable>| __v.push(__init));
            v.as_pointer().decay().to_last()
        };
    }
    assert!(({ ({ (*track.borrow()).move_constructs } == 0_usize) }));
    assert!(({ ((*v.borrow()).capacity() == capacity) }));
    assert!(({ ((*v.borrow()).len() == capacity) }));
    {
        let __init = NotTriviallyMovable::new({ (track.as_pointer()) });
        v.as_pointer()
            .with_mut(|__v: &mut Vec<NotTriviallyMovable>| __v.push(__init));
        v.as_pointer().decay().to_last()
    };
    assert!(({ ({ (*track.borrow()).move_constructs } == capacity) }));
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
    ({ tests_0() });
    const _: () = assert!(true, "tests()");;
    return 0;
}
pub fn __cpp2rust_init_globals() {}
