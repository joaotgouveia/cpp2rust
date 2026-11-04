extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(72)]
pub struct A {
    #[offset(0)]
    pub data: i32,
    #[offset(8)]
    #[byte_size(48)]
    pub m: std_multiset_A__std_less_A___std_allocator_A__,
    #[offset(56)]
    #[byte_size(8)]
    pub it: BidirectionalIterator<A>,
    #[offset(64)]
    #[byte_size(8)]
    pub cit: BidirectionalIterator<A>,
}
impl A {
    pub fn move_from(_a0: Ptr<A>) -> Self {
        Self {
            data: { (*_a0.upgrade().deref()).data },
            m: field!(_a0, m).with_mut(std::mem::take),
            it: field!(_a0, it).with_mut(std::mem::take),
            cit: field!(_a0, cit).with_mut(std::mem::take),
        }
    }
}
impl CppLess for A {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_lt_0(__lhs.clone(), __rhs.clone())
    }
}
impl CppEq for A {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_1(__lhs.clone(), __rhs.clone())
    }
}
impl CppMove for A {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
pub fn operator_eq_1(L: Ptr<A>, R: Ptr<A>) -> bool {
    return ({ (L).clone() } == { (R).clone() });
}
pub fn operator_lt_0(L: Ptr<A>, R: Ptr<A>) -> bool {
    return ({ L.with(|__s| __s.data) } < { R.with(|__s| __s.data) });
}
pub fn test_2() -> bool {
    let mut a: A = <A>::default();
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
    return 0;
}
pub trait AImpl {
    fn move_assign(&self, _a0: Ptr<A>) -> Ptr<A>;
}
impl AImpl for Ptr<A> {
    fn move_assign(&self, _a0: Ptr<A>) -> Ptr<A> {
        field!((*self), data).write({ { (*_a0.upgrade().deref()).data } });
        field!((*self), m).write({ { (*_a0.upgrade().deref()).m.clone() } });
        {
            let __a1 = ({ { (*_a0.upgrade().deref()).it.clone() } });
            field_ptr!((*self), it).with_mut(|__it| *__it = __a1);
            field_ptr!((*self), it)
        };
        {
            let __a1 = ({ { (*_a0.upgrade().deref()).cit.clone() } });
            field_ptr!((*self), cit).with_mut(|__it| *__it = __a1);
            field_ptr!((*self), cit)
        };
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
