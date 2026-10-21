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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
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
    pub v: (),
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
                v: unimplemented!(),
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
            &(array_field_ptr!(o, arr) as Ptr<Counter>).offset((unimplemented!()) as isize),
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
    unimplemented!();
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
    return unimplemented!();
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
        field!(other, n).write(unimplemented!());
    }
}
pub trait SImpl {
    fn bump(&self);
}
impl SImpl for Ptr<S> {
    fn bump(&self) {
        ({
            let _k: () = (*self).with(|__s| __s.tag);
            CounterImpl::add(&field_ptr!((*self), c), _k)
        });
    }
}
pub fn __cpp2rust_init_globals() {}
