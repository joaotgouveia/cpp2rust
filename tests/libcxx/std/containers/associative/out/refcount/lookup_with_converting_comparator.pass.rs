extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(i32, new, value)]
#[byte_size(4)]
pub struct S {
    #[offset(0)]
    pub i_: i32,
}
impl S {
    pub fn new(mut i: i32) -> Self {
        Self { i_: i }
    }
}
impl CppLess for S {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        SImpl::operator_lt(__lhs, __rhs.read())
    }
}
impl CppEq for S {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        !(SImpl::operator_lt(__lhs, __rhs.read())) && !(SImpl::operator_lt(__rhs, __lhs.read()))
    }
}
impl CppMove for S {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn test_0(container: Ptr<Map<Ptr<S>, (Value<Ptr<S>>, Value<AnyPtr>)>>) {
    {
        let c: Ptr<Map<Ptr<S>, (Value<Ptr<S>>, Value<AnyPtr>)>> = (container).clone();
        let v: Value<S> = Rc::new(RefCell::new(S::new({ 1 })));
        assert!(
            match (Rc::new(RefCell::new(v.as_pointer())).as_pointer(),) {
                (__tmp_0,) => {
                    c.with(|__m: &Map<Ptr<S>, (Value<Ptr<S>>, Value<AnyPtr>)>| {
                        __m.find(&(__tmp_0).clone())
                    })
                }
            } == c.with(|__m: &Map<Ptr<S>, (Value<Ptr<S>>, Value<AnyPtr>)>| __m.end())
        );
        assert!(
            (({
                let ___x: Value<Ptr<S>> = Rc::new(RefCell::new(v.as_pointer()));
                (*c.upgrade().deref()).count(___x.as_pointer())
            }) == 0_usize)
        );
        assert!(
            ({
                let ___x: Value<Ptr<S>> = Rc::new(RefCell::new(v.as_pointer()));
                (*c.upgrade().deref()).lower_bound_82(___x.as_pointer())
            }) == c.with(|__m: &Map<Ptr<S>, (Value<Ptr<S>>, Value<AnyPtr>)>| __m.end())
        );
        assert!(
            ({
                let ___x: Value<Ptr<S>> = Rc::new(RefCell::new(v.as_pointer()));
                (*c.upgrade().deref()).upper_bound_86(___x.as_pointer())
            }) == c.with(|__m: &Map<Ptr<S>, (Value<Ptr<S>>, Value<AnyPtr>)>| __m.end())
        );
        assert!(
            (*({
                let ___x: Value<Ptr<S>> = Rc::new(RefCell::new(v.as_pointer()));
                (*c.upgrade().deref()).equal_range_90(___x.as_pointer())
            })
            .0
            .borrow())
                == c.with(|__m: &Map<Ptr<S>, (Value<Ptr<S>>, Value<AnyPtr>)>| __m.end())
        );
        assert!(
            (*({
                let ___x: Value<Ptr<S>> = Rc::new(RefCell::new(v.as_pointer()));
                (*c.upgrade().deref()).equal_range_90(___x.as_pointer())
            })
            .1
            .borrow())
                == c.with(|__m: &Map<Ptr<S>, (Value<Ptr<S>>, Value<AnyPtr>)>| __m.end())
        );
        assert!(
            !({
                let ___x: Value<Ptr<S>> = Rc::new(RefCell::new(v.as_pointer()));
                (*c.upgrade().deref()).contains(___x.as_pointer())
            })
        );
    }
    {
        let c: Ptr<Map<Ptr<S>, (Value<Ptr<S>>, Value<AnyPtr>)>> = (container).clone();
        let v: Value<S> = Rc::new(RefCell::new(S::new({ 1 })));
        assert!(
            match (Rc::new(RefCell::new(v.as_pointer())).as_pointer(),) {
                (__tmp_1,) => {
                    c.with(|__m: &Map<Ptr<S>, (Value<Ptr<S>>, Value<AnyPtr>)>| {
                        __m.find(&(__tmp_1).clone())
                    })
                }
            } == c.with(|__m: &Map<Ptr<S>, (Value<Ptr<S>>, Value<AnyPtr>)>| __m.end())
        );
        assert!(
            (({
                let ___x: Value<Ptr<S>> = Rc::new(RefCell::new(v.as_pointer()));
                (*c.upgrade().deref()).count(___x.as_pointer())
            }) == 0_usize)
        );
        assert!(
            ({
                let ___x: Value<Ptr<S>> = Rc::new(RefCell::new(v.as_pointer()));
                (*c.upgrade().deref()).lower_bound_84(___x.as_pointer())
            }) == c.with(|__m: &Map<Ptr<S>, (Value<Ptr<S>>, Value<AnyPtr>)>| __m.end())
        );
        assert!(
            ({
                let ___x: Value<Ptr<S>> = Rc::new(RefCell::new(v.as_pointer()));
                (*c.upgrade().deref()).upper_bound_88(___x.as_pointer())
            }) == c.with(|__m: &Map<Ptr<S>, (Value<Ptr<S>>, Value<AnyPtr>)>| __m.end())
        );
        assert!(
            (*({
                let ___x: Value<Ptr<S>> = Rc::new(RefCell::new(v.as_pointer()));
                (*c.upgrade().deref()).equal_range_92(___x.as_pointer())
            })
            .0
            .borrow())
                == c.with(|__m: &Map<Ptr<S>, (Value<Ptr<S>>, Value<AnyPtr>)>| __m.end())
        );
        assert!(
            (*({
                let ___x: Value<Ptr<S>> = Rc::new(RefCell::new(v.as_pointer()));
                (*c.upgrade().deref()).equal_range_92(___x.as_pointer())
            })
            .1
            .borrow())
                == c.with(|__m: &Map<Ptr<S>, (Value<Ptr<S>>, Value<AnyPtr>)>| __m.end())
        );
        assert!(
            !({
                let ___x: Value<Ptr<S>> = Rc::new(RefCell::new(v.as_pointer()));
                (*c.upgrade().deref()).contains(___x.as_pointer())
            })
        );
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
        let m: Value<Map<Ptr<S>, (Value<Ptr<S>>, Value<AnyPtr>)>> =
            Rc::new(RefCell::new(Map::new()));
        ({ test_0(m.as_pointer()) });
    }
    {
        let m : Value<std_multimap_std_reference_wrapper_S___void_ptr__std_less_S___std_allocator_std_pair_const_std_reference_wrapper_S___void_ptr___ > = Rc::new(RefCell::new(<std_multimap_std_reference_wrapper_S___void_ptr__std_less_S___std_allocator_std_pair_const_std_reference_wrapper_S___void_ptr___  >::default() )) ;
        ({ test_1(m.as_pointer()) });
    }
    {
        let s : Value<std_set_std_reference_wrapper_S___std_less_S___std_allocator_std_reference_wrapper_S___ > = Rc::new(RefCell::new(<std_set_std_reference_wrapper_S___std_less_S___std_allocator_std_reference_wrapper_S___  >::default() )) ;
        ({ test_2(s.as_pointer()) });
    }
    {
        let s : Value<std_multiset_std_reference_wrapper_S___std_less_S___std_allocator_std_reference_wrapper_S___ > = Rc::new(RefCell::new(<std_multiset_std_reference_wrapper_S___std_less_S___std_allocator_std_reference_wrapper_S___  >::default() )) ;
        ({ test_3(s.as_pointer()) });
    }
    return 0;
}
pub trait SImpl {
    fn operator_lt(&self, lhs: S) -> bool;
}
impl SImpl for Ptr<S> {
    fn operator_lt(&self, mut lhs: S) -> bool {
        return (lhs.i_ < (*self).with(|__s| __s.i_));
    }
}
pub fn __cpp2rust_init_globals() {}
