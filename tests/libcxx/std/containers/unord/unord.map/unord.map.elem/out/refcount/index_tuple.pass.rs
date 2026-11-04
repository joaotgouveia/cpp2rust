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
pub struct my_hash {}
impl Callable1<Ptr<std_tuple_int__int_>, usize> for my_hash {
    fn call(&self, a1: Ptr<std_tuple_int__int_>) -> usize {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            my_hashImpl::operator_call(__it, a1)
        })
    }
}
impl CppMove for my_hash {
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
    let m : Value<std_unordered_map_std_tuple_int__int___unsigned_long__my_hash__std_equal_to_std_tuple_int__int____std_allocator_std_pair_const_std_tuple_int__int___unsigned_long___ > = Rc::new(RefCell::new(<std_unordered_map_std_tuple_int__int___unsigned_long__my_hash__std_equal_to_std_tuple_int__int____std_allocator_std_pair_const_std_tuple_int__int___unsigned_long___  >::default() )) ;
    elem!(
        m.as_pointer(),
        ({
            let ___args_0: Value<i32> = Rc::new(RefCell::new(2));
            let ___args_1: Value<i32> = Rc::new(RefCell::new(3));
            make_tuple_0(___args_0.as_pointer(), ___args_1.as_pointer())
        })
    )
    .write(7_u64);
    return 0;
}
pub trait my_hashImpl {
    fn operator_call(&self, _a0: Ptr<std_tuple_int__int_>) -> usize;
}
impl my_hashImpl for Ptr<my_hash> {
    fn operator_call(&self, _a0: Ptr<std_tuple_int__int_>) -> usize {
        return 0_usize;
    }
}
pub fn __cpp2rust_init_globals() {}
