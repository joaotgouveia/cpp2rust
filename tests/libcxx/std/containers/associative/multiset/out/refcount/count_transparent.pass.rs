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
pub struct Comp {}
impl Callable2<Ptr<(Value<i32>, Value<i32>)>, Ptr<(Value<i32>, Value<i32>)>, bool> for Comp {
    fn call(&self, a1: Ptr<(Value<i32>, Value<i32>)>, a2: Ptr<(Value<i32>, Value<i32>)>) -> bool {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            CompImpl::operator_call_1(__it, a1, a2)
        })
    }
}
impl Callable2<Ptr<(Value<i32>, Value<i32>)>, i32, bool> for Comp {
    fn call(&self, a1: Ptr<(Value<i32>, Value<i32>)>, a2: i32) -> bool {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            CompImpl::operator_call_2(__it, a1, a2)
        })
    }
}
impl Callable2<i32, Ptr<(Value<i32>, Value<i32>)>, bool> for Comp {
    fn call(&self, a1: i32, a2: Ptr<(Value<i32>, Value<i32>)>) -> bool {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            CompImpl::operator_call_3(__it, a1, a2)
        })
    }
}
impl CppMove for Comp {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn test_0() -> bool {
    let s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut cnt: usize = unimplemented!();
    assert!((cnt == 3_usize));
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
    ({ test_0() });
    return 0;
}
pub trait CompImpl {
    fn operator_call_1(
        &self,
        lhs: Ptr<(Value<i32>, Value<i32>)>,
        rhs: Ptr<(Value<i32>, Value<i32>)>,
    ) -> bool;
    fn operator_call_2(&self, lhs: Ptr<(Value<i32>, Value<i32>)>, rhs: i32) -> bool;
    fn operator_call_3(&self, lhs: i32, rhs: Ptr<(Value<i32>, Value<i32>)>) -> bool;
}
impl CompImpl for Ptr<Comp> {
    fn operator_call_1(
        &self,
        lhs: Ptr<(Value<i32>, Value<i32>)>,
        rhs: Ptr<(Value<i32>, Value<i32>)>,
    ) -> bool {
        return unimplemented!() == std::cmp::Ordering::Less;
    }
    fn operator_call_2(&self, lhs: Ptr<(Value<i32>, Value<i32>)>, mut rhs: i32) -> bool {
        return ({ (*(*lhs.upgrade().deref()).0.borrow()) } < { rhs });
    }
    fn operator_call_3(&self, mut lhs: i32, rhs: Ptr<(Value<i32>, Value<i32>)>) -> bool {
        return ({ lhs } < { (*(*rhs.upgrade().deref()).0.borrow()) });
    }
}
pub fn __cpp2rust_init_globals() {}
