extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor, Default)]
#[byte_size(1)]
pub struct X {}
thread_local!(
    pub static dtor_called_0: Value<()> = Rc::new(RefCell::new(false));
);
pub fn check_reset_1() -> () {
    {
        let opt: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "noexcept(opt.reset()) == true");;
        ({ ((*opt.borrow()) as std___optional_destruct_base_int__true_).reset() });
        unimplemented!();
    }
    {
        let opt: Value<()> = Rc::new(RefCell::new(3_()));
        ({ ((*opt.borrow()) as std___optional_destruct_base_int__true_).reset() });
        unimplemented!();
    }
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
    ({ check_reset_1() });
    const _: () = assert!(true, "check_reset()");;
    {
        let opt: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "noexcept(opt.reset()) == true");;
        unimplemented!();
        ({ ((*opt.borrow()) as std___optional_destruct_base_X__false_).reset() });
        unimplemented!();
        unimplemented!();
    }
    {
        let opt: Value<()> = Rc::new(RefCell::new(<X>::default()));
        dtor_called_0.with(|rc| *rc.borrow_mut() = false);
        ({ ((*opt.borrow()) as std___optional_destruct_base_X__false_).reset() });
        unimplemented!();
        unimplemented!();
        dtor_called_0.with(|rc| *rc.borrow_mut() = false);
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
