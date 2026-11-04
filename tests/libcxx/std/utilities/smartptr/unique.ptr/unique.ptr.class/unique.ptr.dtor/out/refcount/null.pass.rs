extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(4)]
pub struct Deleter {
    #[offset(0)]
    state_: i32,
}
impl Deleter {
    pub fn new() -> Self {
        Self { state_: 0 }
    }
}
impl Default for Deleter {
    fn default() -> Self {
        { Deleter::new() }
    }
}
pub fn test_basic_0() {
    let d: Value<Deleter> = Rc::new(RefCell::new(Deleter::new()));
    assert!((({ DeleterImpl::state(&d.as_pointer(),) }) == 0));
    {
        let p: Value<Option<Value<i32>>> = Rc::new(RefCell::new(
            std_unique_ptr_int__Deleter_ref_::new_1({ Ptr::null() }, { d.as_pointer() }),
        ));
        assert!(({ ((*p.borrow()).as_pointer()).is_null() }));
        assert!(({ (({ (*p.borrow()).get_deleter_17() }) == (d.as_pointer())) }));
    }
    assert!((({ DeleterImpl::state(&d.as_pointer(),) }) == 0));
}
pub fn test_basic_1() {
    let d: Value<Deleter> = Rc::new(RefCell::new(Deleter::new()));
    assert!((({ DeleterImpl::state(&d.as_pointer(),) }) == 0));
    {
        let p: Value<Option<Value<Box<[i32]>>>> = Rc::new(RefCell::new(
            std_unique_ptr_intarrarr__Deleter_ref_::new_1({ Default::default() }, {
                d.as_pointer()
            }),
        ));
        assert!(({ ((*p.borrow()).as_pointer()).is_null() }));
        assert!(({ (({ (*p.borrow()).get_deleter_15() }) == (d.as_pointer())) }));
    }
    assert!((({ DeleterImpl::state(&d.as_pointer(),) }) == 0));
}
pub fn test_2() -> bool {
    ({ test_basic_0() });
    ({ test_basic_1() });
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
    ({ test_2() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub trait DeleterImpl {
    fn copy_assign(&self, _a0: Ptr<Deleter>) -> Ptr<Deleter> {
        unimplemented!()
    }
    fn state(&self) -> i32;
    fn operator_call(&self, _a0: AnyPtr);
}
impl DeleterImpl for Ptr<Deleter> {
    fn state(&self) -> i32 {
        return (*self).with(|__s| __s.state_);
    }
    fn operator_call(&self, mut _a0: AnyPtr) {
        field!((*self), state_).with_mut(|__v| {
            __v.prefix_inc();
        });
    }
}
pub fn __cpp2rust_init_globals() {}
