extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct A {}
impl CppMove for A {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
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
    let s: Value<std_shared_ptr_A_> = Rc::new(RefCell::new(std_shared_ptr_A_::new_1({
        Ptr::alloc(<A>::default())
    })));
    let w: Value<std_weak_ptr_A_> =
        Rc::new(RefCell::new(std_weak_ptr_A_::new_1({ s.as_pointer() })));
    const _: () = assert!(true, "");;
    assert!(({ !({ ((*w.borrow()) as std___weak_ptr_A____gnu_cxx__S_atomic_).expired() }) }));
    assert!(
        ({
            (({ ((*w.borrow()) as std___weak_ptr_A____gnu_cxx__S_atomic_).use_count() }) == 1_i64)
        })
    );
    assert!(
        (({ (({ (*w.borrow()).lock() }) as std___shared_ptr_A____gnu_cxx__S_atomic_).get() })
            == ({ ((*s.borrow()) as std___shared_ptr_A____gnu_cxx__S_atomic_).get() }))
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
