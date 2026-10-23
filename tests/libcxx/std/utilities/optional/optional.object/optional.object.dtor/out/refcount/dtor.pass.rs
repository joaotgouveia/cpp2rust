extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct PODType {
    #[offset(0)]
    pub value: (),
    #[offset(4)]
    pub value2: (),
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor, Default)]
#[byte_size(1)]
pub struct X {}
thread_local!(
    pub static dtor_called_0: Value<()> = Rc::new(RefCell::new(false));
);

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
    {
        const _: () = assert!(true, "std::is_trivially_destructible<T>::value");;
        const _: () = assert!(true, "std::is_trivially_destructible<optional<T>>::value");;
    }
    {
        const _: () = assert!(true, "std::is_trivially_destructible<T>::value");;
        const _: () = assert!(true, "std::is_trivially_destructible<optional<T>>::value");;
    }
    {
        const _: () = assert!(true, "std::is_trivially_destructible<T>::value");;
        const _: () = assert!(true, "std::is_trivially_destructible<optional<T>>::value");;
    }
    {
        const _: () = assert!(true, "!std::is_trivially_destructible<T>::value");;
        const _: () = assert!(true, "!std::is_trivially_destructible<optional<T>>::value");;
        {
            let x: Value<X> = Rc::new(RefCell::new(<X>::default()));
            let _dtor_x = ScopedDestructor::new(&x, |__p| __p.destructor());
            let opt: Value<()> = Rc::new(RefCell::new(std_optional_X_::new_1({ x.as_pointer() })));
            unimplemented!();
        }
        unimplemented!();
    }
    return 0_();
}
pub trait XImpl {
    fn destructor(&self);
}
impl XImpl for Ptr<X> {
    fn destructor(&self) {
        dtor_called_0.with(|rc| *rc.borrow_mut() = true);
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = dtor_called_0.with(|_| ());
}
