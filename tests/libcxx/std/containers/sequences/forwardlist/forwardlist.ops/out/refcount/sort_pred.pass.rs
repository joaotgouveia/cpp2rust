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
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub fn outstanding_alloc() -> usize {
        assert!((alloc_count_1.with(|rc| *rc.borrow()) >= dealloc_count_2.with(|rc| *rc.borrow())));
        return ((alloc_count_1.with(|rc| *rc.borrow()))
            .wrapping_sub(dealloc_count_2.with(|rc| *rc.borrow())));
    }
    pub fn reset() {
        assert!((({ malloc_allocator_base::outstanding_alloc() }) == 0_usize));
        disable_default_constructor_3.with(|rc| *rc.borrow_mut() = false);
        outstanding_bytes_0.with(|rc| *rc.borrow_mut() = 0_usize);
        alloc_count_1.with(|rc| *rc.borrow_mut() = 0_usize);
        dealloc_count_2.with(|rc| *rc.borrow_mut() = 0_usize);
    }
}
impl CppMove for malloc_allocator_base {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static outstanding_bytes_0: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static alloc_count_1: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static dealloc_count_2: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static disable_default_constructor_3: Value<bool> = Rc::new(RefCell::new(false));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_int_ {}
impl CppMove for min_allocator_int_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static randomness_4 : Value<std_mersenne_twister_engine_unsigned_long__32__624__397__31__2567483615__11__4294967295__7__2636928640__15__4022730752__18__1812433253_ > = Rc::new(RefCell::new(<std_mersenne_twister_engine_unsigned_long__32__624__397__31__2567483615__11__4294967295__7__2636928640__15__4022730752__18__1812433253_  >::default() )) ;
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(i32, new_1, value)]
#[byte_size(8)]
pub struct Payload {
    #[offset(0)]
    pub val: i32,
    #[offset(4)]
    pub side: i32,
}
impl Payload {
    pub fn new_1(mut v: i32) -> Self {
        Self { val: v, side: 0 }
    }
    pub fn new_2(mut v: i32, mut s: i32) -> Self {
        Self { val: v, side: s }
    }
}
impl CppLess for Payload {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        PayloadImpl::operator_lt(__lhs, __rhs.clone())
    }
}
impl CppEq for Payload {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        !(PayloadImpl::operator_lt(__lhs, __rhs.clone()))
            && !(PayloadImpl::operator_lt(__rhs, __lhs.clone()))
    }
}
impl CppMove for Payload {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn greater_5(lhs: Ptr<Payload>, rhs: Ptr<Payload>) -> bool {
    return ({ lhs.with(|__s| __s.val) } > { rhs.with(|__s| __s.val) });
}
pub fn test_stable_6(mut N: i32) {
    let v: Value<Vec<Payload>> = Rc::new(RefCell::new(Vec::new()));
    let mut i: i32 = 0;
    'loop_: while (i < N) {
        {
            let __a1 = Payload::new_1({ (i / 2) });
            (*v.borrow_mut()).push(__a1)
        };
        i.prefix_inc();
    }
    ({
        let ___first: RandomAccessIterator<Payload> = CppRange::begin(&v.decay());
        let ___last: RandomAccessIterator<Payload> = CppRange::end(&v.decay());
        shuffle_7(___first, ___last, randomness_4.with(|v| v.as_pointer()))
    });
    let mut i: i32 = 0;
    'loop_: while (i < N) {
        let __rhs = i;
        field!(v.decay().offset((i as usize) as isize), side).write(__rhs);
        i.prefix_inc();
    }
    let c: Value<std_forward_list_Payload__std_allocator_Payload__> = Rc::new(RefCell::new(
        std_forward_list_Payload__std_allocator_Payload__::new_1(
            { CppRange::begin(&v.decay()) },
            { CppRange::end(&v.decay()) },
            None,
        ),
    ));
    ({
        (*c.borrow()).sort(FnPtr::<fn(Ptr<Payload>, Ptr<Payload>) -> bool>::new(
            greater_5,
        ))
    });
    assert!(
        ({
            (({
                let ___first: std__Fwd_list_iterator_Payload_ = ({ (*c.borrow()).begin_25() });
                let ___last: std__Fwd_list_iterator_Payload_ = ({ (*c.borrow()).end_27() });
                distance_8(___first, ___last)
            }) == (N as isize))
        })
    );
    let j: Value<std__Fwd_list_const_iterator_Payload_> = Rc::new(RefCell::new(
        <std__Fwd_list_const_iterator_Payload_>::from(({ (*c.borrow()).begin_25() })),
    ));
    let mut i: i32 = 0;
    'loop_: while (i < N) {
        assert!(({ ((*j.borrow()).iter_deref().with(|__s| __s.val) == (((N - 1) - i) / 2)) }));
        {
            i.prefix_inc();
            {
                j.as_pointer().with_mut(|__it| {
                    __it.iter_inc();
                });
                j.as_pointer()
            }
        };
    }
    let it: Value<std__Fwd_list_const_iterator_Payload_> = Rc::new(RefCell::new(
        <std__Fwd_list_const_iterator_Payload_>::from(({ (*c.borrow()).begin_25() })),
    ));
    'loop_: while (*it.borrow())
        != <std__Fwd_list_const_iterator_Payload_>::from(({ (*c.borrow()).end_27() }))
    {
        let next: Value<std__Fwd_list_const_iterator_Payload_> = Rc::new(RefCell::new(
            libcc2rs::next_refcount(({ (*it.borrow()).clone() }), 1_isize),
        ));
        if ((*next.borrow())
            != <std__Fwd_list_const_iterator_Payload_>::from(({ (*c.borrow()).end_27() })))
            && ((*it.borrow()).iter_deref().with(|__s| __s.val)
                == (*next.borrow()).iter_deref().with(|__s| __s.val))
        {
            assert!(
                ({
                    ((*it.borrow()).iter_deref().with(|__s| __s.side)
                        < (*next.borrow()).iter_deref().with(|__s| __s.side))
                })
            );
        }
        {
            it.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            it.as_pointer()
        };
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
    let mut i: i32 = 0;
    'loop_: while (i < 40) {
        ({ test_9(i) });
        i.prefix_inc();
    }
    let mut i: i32 = 0;
    'loop_: while (i < 40) {
        ({ test_10(i) });
        i.prefix_inc();
    }
    let mut i: i32 = 0;
    'loop_: while (i < 40) {
        ({ test_stable_6(i) });
        i.prefix_inc();
    }
    return 0;
}
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct min_pointer_int__std_integral_constant_unsigned_long__0__;
pub trait PayloadImpl {
    fn operator_lt(&self, rhs: Ptr<Payload>) -> bool;
}
impl PayloadImpl for Ptr<Payload> {
    fn operator_lt(&self, rhs: Ptr<Payload>) -> bool {
        return ({ (*self).with(|__s| __s.val) } < { rhs.with(|__s| __s.val) });
    }
}
pub trait min_allocator_int_Impl {
    fn allocate(&self, mut n: usize) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn deallocate(
        &self,
        mut p: min_pointer_int__std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        unimplemented!()
    }
}
impl min_allocator_int_Impl for Ptr<min_allocator_int_> {}
pub fn __cpp2rust_init_globals() {
    let _ = outstanding_bytes_0.with(|_| ());
    let _ = alloc_count_1.with(|_| ());
    let _ = dealloc_count_2.with(|_| ());
    let _ = disable_default_constructor_3.with(|_| ());
    let _ = randomness_4.with(|_| ());
}
