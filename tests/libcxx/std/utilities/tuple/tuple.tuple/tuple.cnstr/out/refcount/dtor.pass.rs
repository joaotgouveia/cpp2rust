extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, Destructor, Default)]
#[byte_size(8)]
pub struct TrackDtor {
    #[offset(0)]
    #[byte_size(8)]
    pub count_: Ptr<()>,
}
impl TrackDtor {
    pub fn new(mut count: Ptr<()>) -> Self {
        Self {
            count_: (count).clone(),
        }
    }
    pub fn move_from(that: Ptr<TrackDtor>) -> Self {
        let __this: TrackDtor = Self {
            count_: that.with(|__s| __s.count_.clone()),
        };
        field!(that, count_).write(Ptr::<()>::null());
        __this
    }
}
const _: () = assert!(true, "!std::is_trivially_destructible<TrackDtor>::value");
const _: () = assert!(true, "std::is_trivially_destructible<std::tuple<>>::value");
const _: () = assert!(
    true,
    "std::is_trivially_destructible<std::tuple<void*>>::value"
);
const _: () = assert!(
    true,
    "std::is_trivially_destructible<std::tuple<int, float>>::value"
);
const _: () = assert!(
    true,
    "!std::is_trivially_destructible<std::tuple<std::string>>::value"
);
const _: () = assert!(
    true,
    "!std::is_trivially_destructible<std::tuple<int, std::string>>::value"
);
pub fn test_0() -> () {
    let count: Value<()> = Rc::new(RefCell::new(0_()));
    {
        let tuple: Value<()> = Rc::new(RefCell::new(TrackDtor::new({ (count.as_pointer()) })));
        unimplemented!();
    }
    unimplemented!();
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
    ({ test_0() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub trait TrackDtorImpl {
    fn destructor(&self);
}
impl TrackDtorImpl for Ptr<TrackDtor> {
    fn destructor(&self) {
        if !((*self).with(|__s| __s.count_.clone())).is_null() {
            (*self)
                .with(|__s| __s.count_.clone())
                .with_mut(|__v| __v.prefix_inc());
        }
    }
}
pub fn __cpp2rust_init_globals() {}
