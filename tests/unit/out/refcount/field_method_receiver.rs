extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Counter {
    #[offset(0)]
    pub n: (),
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign)]
#[byte_size(40)]
pub struct S {
    #[offset(0)]
    pub tag: (),
    #[offset(4)]
    #[byte_size(4)]
    pub c: Counter,
    #[offset(8)]
    #[byte_size(8)]
    pub arr: Box<[Counter]>,
    #[offset(16)]
    #[byte_size(24)]
    pub v: std_vector_int__std_allocator_int__,
}
impl S {
    pub fn move_from(_a0: Ptr<S>) -> Self {
        Self {
            tag: { (*_a0.upgrade().deref()).tag },
            c: { (*_a0.upgrade().deref()).c.clone() },
            arr: Box::new(std::array::from_fn::<_, 2, _>(|__i: usize| {
                ((*_a0.upgrade().deref()).arr[(__i) as usize]).clone()
            })),
            v: field!(_a0, v).with_mut(std::mem::take),
        }
    }
}
impl Default for S {
    fn default() -> Self {
        {
            S {
                tag: unimplemented!(),
                c: <Counter>::default(),
                arr: (0..2)
                    .map(|_| <Counter>::default())
                    .collect::<Box<[Counter]>>(),
                v: <std_vector_int__std_allocator_int__>::default(),
            }
        }
    }
}
pub fn run_0(mut o: Ptr<S>) {
    ({ CounterImpl::add(&field_ptr!(o, c), 2_()) });
    unimplemented!();
    ({
        CounterImpl::add(
            &(array_field_ptr!(o, arr) as Ptr<Counter>).offset((1_()) as isize),
            5_(),
        )
    });
    unimplemented!();
    ({ CounterImpl::add(&({ CounterImpl::self_(&field_ptr!(o, c)) }), 1_()) });
    unimplemented!();
    unimplemented!();
    ({
        let _other: Ptr<Counter> =
            ((array_field_ptr!(o, arr) as Ptr<Counter>).offset((1_()) as isize));
        CounterImpl::take(
            &(array_field_ptr!(o, arr) as Ptr<Counter>).offset((0_()) as isize),
            _other,
        )
    });
    unimplemented!();
    ({
        let _other: Ptr<Counter> = (field_ptr!(o, c));
        CounterImpl::take(&field_ptr!(o, c), _other)
    });
    unimplemented!();
    ({ SImpl::bump(&o) });
    unimplemented!();
    ({
        let ___x: Value<()> = Rc::new(RefCell::new(({ CounterImpl::get(&field_ptr!(o, c)) })));
        (*o.upgrade().deref()).v.push_back_58(___x.as_pointer())
    });
    unimplemented!();
    unimplemented!();
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let local: Value<S> = Rc::new(RefCell::new(<S>::default()));
    (*local.borrow_mut()).tag = 1_();
    ({ run_0((local.as_pointer())) });
    let mut heap: Ptr<S> = Ptr::alloc(<S>::default());
    field!(heap, tag).write(1_());
    ({ run_0((heap).clone()) });
    heap.delete();
    return 0_();
}
pub trait CounterImpl {
    fn get(&self) -> ();
    fn add(&self, k: ());
    fn self_(&self) -> Ptr<Counter>;
    fn take(&self, other: Ptr<Counter>);
}
impl CounterImpl for Ptr<Counter> {
    fn get(&self) -> () {
        return (*self).with(|__s| __s.n);
    }
    fn add(&self, mut k: ()) {
        {
            let __rhs = k;
            field!((*self), n).with_mut(|__v| *__v = *__v + __rhs)
        };
    }
    fn self_(&self) -> Ptr<Counter> {
        return (*self).clone();
    }
    fn take(&self, mut other: Ptr<Counter>) {
        {
            let __rhs = { other.with(|__s| __s.n) };
            field!((*self), n).with_mut(|__v| *__v = *__v + __rhs)
        };
        field!(other, n).write(0_());
    }
}
pub trait SImpl {
    fn bump(&self);
    fn move_assign(&self, _a0: Ptr<S>) -> Ptr<S>;
}
impl SImpl for Ptr<S> {
    fn bump(&self) {
        ({
            let _k: () = (*self).with(|__s| __s.tag);
            CounterImpl::add(&field_ptr!((*self), c), _k)
        });
    }
    fn move_assign(&self, _a0: Ptr<S>) -> Ptr<S> {
        field!((*self), tag).write({ { (*_a0.upgrade().deref()).tag } });
        field!((*self), c).write({ { (*_a0.upgrade().deref()).c.clone() } });
        unimplemented!();
        field!((*self), v).write({ { (*_a0.upgrade().deref()).v.clone() } });
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
