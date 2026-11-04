extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static destroyed_0: Value<std_set_int__std_less_int___std_allocator_int__> = Rc::new(
        RefCell::new(<std_set_int__std_less_int___std_allocator_int__>::default()),
    );
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor, Default)]
#[byte_size(4)]
pub struct Foo {
    #[offset(0)]
    pub value: i32,
}
impl Foo {
    pub fn new(mut i: i32) -> Self {
        Self { value: i }
    }
}
impl CppMove for Foo {
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
    {
        let list: Value<std_list_Foo__std_allocator_Foo__> =
            Rc::new(RefCell::new(<std_list_Foo__std_allocator_Foo__>::default()));
        ({
            let ___args: Value<i32> = Rc::new(RefCell::new(1));
            (*list.borrow()).emplace_back(___args.as_pointer())
        });
        ({
            let ___args: Value<i32> = Rc::new(RefCell::new(2));
            (*list.borrow()).emplace_back(___args.as_pointer())
        });
        ({
            let ___args: Value<i32> = Rc::new(RefCell::new(3));
            (*list.borrow()).emplace_back(___args.as_pointer())
        });
        assert!(({ ({ (*destroyed_0.with(Value::clone).borrow()).empty() }) }));
    }
    assert!(
        (({
            let ___x: Value<i32> = Rc::new(RefCell::new(1));
            (*destroyed_0.with(Value::clone).borrow()).count(___x.as_pointer())
        }) == 1_u64)
    );
    assert!(
        (({
            let ___x: Value<i32> = Rc::new(RefCell::new(2));
            (*destroyed_0.with(Value::clone).borrow()).count(___x.as_pointer())
        }) == 1_u64)
    );
    assert!(
        (({
            let ___x: Value<i32> = Rc::new(RefCell::new(3));
            (*destroyed_0.with(Value::clone).borrow()).count(___x.as_pointer())
        }) == 1_u64)
    );
    return 0;
}
pub trait FooImpl {
    fn destructor(&self);
}
impl FooImpl for Ptr<Foo> {
    fn destructor(&self) {
        ({ (*destroyed_0.with(Value::clone).borrow()).insert_36(field_ptr!((*self), value)) });
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = destroyed_0.with(|_| ());
}
