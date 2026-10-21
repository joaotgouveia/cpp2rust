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
pub struct S {
    #[offset(0)]
    pub v: (),
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let a: Value<S> = Rc::new(RefCell::new(S { v: 6_() }));
    let b: Value<S> = Rc::new(RefCell::new(S { v: 4_() }));
    ({ SImpl::operator_add_assign(&a.as_pointer(), b.as_pointer()) });
    unimplemented!();
    ({ SImpl::operator_sub_assign(&a.as_pointer(), b.as_pointer()) });
    unimplemented!();
    ({ SImpl::operator_mul_assign(&a.as_pointer(), b.as_pointer()) });
    unimplemented!();
    ({ SImpl::operator_div_assign(&a.as_pointer(), b.as_pointer()) });
    unimplemented!();
    ({ SImpl::operator_rem_assign(&a.as_pointer(), b.as_pointer()) });
    unimplemented!();
    ({ SImpl::operator_bitor_assign(&a.as_pointer(), b.as_pointer()) });
    unimplemented!();
    ({ SImpl::operator_bitand_assign(&a.as_pointer(), b.as_pointer()) });
    unimplemented!();
    ({ SImpl::operator_bitxor_assign(&a.as_pointer(), b.as_pointer()) });
    unimplemented!();
    ({ SImpl::operator_assign_1(&a.as_pointer(), 3_()) });
    unimplemented!();
    ({ SImpl::operator_shl_assign(&a.as_pointer(), 2_()) });
    unimplemented!();
    ({ SImpl::operator_shr_assign(&a.as_pointer(), 1_()) });
    unimplemented!();
    ({
        let _o: Ptr<S> = b.as_pointer();
        SImpl::operator_add_assign(
            &({ SImpl::operator_add_assign(&a.as_pointer(), b.as_pointer()) }),
            _o,
        )
    });
    unimplemented!();
    let mut c: S = S {
        v: unimplemented!(),
    };
    c = (*({ SImpl::operator_assign_1(&a.as_pointer(), 1_()) })
        .upgrade()
        .deref())
    .clone();
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub trait SImpl {
    fn operator_assign_1(&self, n: ()) -> Ptr<S>;
    fn operator_add_assign(&self, o: Ptr<S>) -> Ptr<S>;
    fn operator_sub_assign(&self, o: Ptr<S>) -> Ptr<S>;
    fn operator_mul_assign(&self, o: Ptr<S>) -> Ptr<S>;
    fn operator_div_assign(&self, o: Ptr<S>) -> Ptr<S>;
    fn operator_rem_assign(&self, o: Ptr<S>) -> Ptr<S>;
    fn operator_bitand_assign(&self, o: Ptr<S>) -> Ptr<S>;
    fn operator_bitor_assign(&self, o: Ptr<S>) -> Ptr<S>;
    fn operator_bitxor_assign(&self, o: Ptr<S>) -> Ptr<S>;
    fn operator_shl_assign(&self, n: ()) -> Ptr<S>;
    fn operator_shr_assign(&self, n: ()) -> Ptr<S>;
}
impl SImpl for Ptr<S> {
    fn operator_assign_1(&self, mut n: ()) -> Ptr<S> {
        field!((*self), v).write(n);
        return (*self).clone();
    }
    fn operator_add_assign(&self, o: Ptr<S>) -> Ptr<S> {
        field!((*self), v).write({ ((*self).with(|__s| __s.v)).wrapping_add(o.with(|__s| __s.v)) });
        return (*self).clone();
    }
    fn operator_sub_assign(&self, o: Ptr<S>) -> Ptr<S> {
        field!((*self), v).write({ ((*self).with(|__s| __s.v)).wrapping_sub(o.with(|__s| __s.v)) });
        return (*self).clone();
    }
    fn operator_mul_assign(&self, o: Ptr<S>) -> Ptr<S> {
        field!((*self), v).write({ ((*self).with(|__s| __s.v)).wrapping_mul(o.with(|__s| __s.v)) });
        return (*self).clone();
    }
    fn operator_div_assign(&self, o: Ptr<S>) -> Ptr<S> {
        field!((*self), v).write({ ((*self).with(|__s| __s.v)).wrapping_div(o.with(|__s| __s.v)) });
        return (*self).clone();
    }
    fn operator_rem_assign(&self, o: Ptr<S>) -> Ptr<S> {
        field!((*self), v).write({ ((*self).with(|__s| __s.v)).wrapping_rem(o.with(|__s| __s.v)) });
        return (*self).clone();
    }
    fn operator_bitand_assign(&self, o: Ptr<S>) -> Ptr<S> {
        {
            let __rhs = { o.with(|__s| __s.v) };
            field!((*self), v).with_mut(|__v| *__v = *__v & __rhs)
        };
        return (*self).clone();
    }
    fn operator_bitor_assign(&self, o: Ptr<S>) -> Ptr<S> {
        {
            let __rhs = { o.with(|__s| __s.v) };
            field!((*self), v).with_mut(|__v| *__v = *__v | __rhs)
        };
        return (*self).clone();
    }
    fn operator_bitxor_assign(&self, o: Ptr<S>) -> Ptr<S> {
        {
            let __rhs = { o.with(|__s| __s.v) };
            field!((*self), v).with_mut(|__v| *__v = *__v ^ __rhs)
        };
        return (*self).clone();
    }
    fn operator_shl_assign(&self, mut n: ()) -> Ptr<S> {
        {
            let __rhs = n;
            field!((*self), v).with_mut(|__v| *__v = *__v << __rhs)
        };
        return (*self).clone();
    }
    fn operator_shr_assign(&self, mut n: ()) -> Ptr<S> {
        {
            let __rhs = n;
            field!((*self), v).with_mut(|__v| *__v = *__v >> __rhs)
        };
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
