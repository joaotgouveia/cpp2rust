extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct S {
    #[offset(0)]
    pub a_: i32,
    #[offset(8)]
    #[byte_size(8)]
    pub self__: Ptr<S>,
}
impl S {
    pub fn new_1(mut a: i32) -> Self {
        Self {
            a_: a,
            self__: Ptr::<S>::null(),
        }
    }
    pub fn new_2(mut a: i32, mut other: Ptr<S>) -> Self {
        let __this: Value<S> = Rc::new(RefCell::new(Self {
            a_: a,
            self__: Ptr::<S>::null(),
        }));
        let this: Ptr<S> = __this.as_pointer();
        if (this == other) {
            field!(this, self__).write(Ptr::<S>::null());
        } else {
            field!(this, self__).write((other).clone());
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn new_3(mut a: i32, mut other: Ptr<S>) -> Self {
        let __this: Value<S> = Rc::new(RefCell::new(Self {
            a_: a,
            self__: Ptr::<S>::null(),
        }));
        let this: Ptr<S> = __this.as_pointer();
        if (this == other) {
            field!(this, self__).write(Ptr::<S>::null());
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
pub fn bump_0(mut p: Ptr<S>) {
    field!(p, a_).with_mut(|__v| __v.postfix_inc());
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct D {
    #[offset(0)]
    pub a_: i32,
}
impl D {
    pub fn new(mut a: i32) -> Self {
        let __this: Value<D> = Rc::new(RefCell::new(Self { a_: a }));
        let this: Ptr<D> = __this.as_pointer();
        {
            field!(this, a_).with_mut(|__v| *__v = *__v * 2)
        };
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let s: Value<S> = Rc::new(RefCell::new(S::new_1({ 1 })));
    let ref_: Ptr<S> = ({ SImpl::returns_this_reference(&s.as_pointer()) });
    field!(ref_, a_).with_mut(|__v| __v.postfix_inc());
    (if ({ (*s.borrow()).a_ } == 2) {
        &(0);
    } else {
        ({
            __assert_fail_1(
                Ptr::<i8>::from_string_literal(b"s.a_ == 2"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut ptr: Ptr<S> = ({ SImpl::returns_this_pointer(&s.as_pointer()) });
    field!(ptr, a_).with_mut(|__v| __v.postfix_inc());
    (if ({ (*s.borrow()).a_ } == 3) {
        &(0);
    } else {
        ({
            __assert_fail_1(
                Ptr::<i8>::from_string_literal(b"s.a_ == 3"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    ({ SImpl::inc(&({ SImpl::inc(&({ SImpl::inc(&s.as_pointer()) })) })) });
    (if ({ (*s.borrow()).a_ } == 6) {
        &(0);
    } else {
        ({
            __assert_fail_1(
                Ptr::<i8>::from_string_literal(b"s.a_ == 6"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    ({ SImpl::set_from_this(&s.as_pointer()) });
    (if ({ (*s.borrow()).a_ } == 7) {
        &(0);
    } else {
        ({
            __assert_fail_1(
                Ptr::<i8>::from_string_literal(b"s.a_ == 7"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ SImpl::twice(&s.as_pointer()) }) == 14) {
        &(0);
    } else {
        ({
            __assert_fail_1(
                Ptr::<i8>::from_string_literal(b"s.twice() == 14"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    ({ SImpl::link(&s.as_pointer()) });
    (if ({ { (*s.borrow()).self__.clone() } } == { (s.as_pointer()) }) {
        &(0);
    } else {
        ({
            __assert_fail_1(
                Ptr::<i8>::from_string_literal(b"s.self_ == &s"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    field!({ (*s.borrow()).self__.clone() }, a_).with_mut(|__v| __v.postfix_inc());
    (if ({ (*s.borrow()).a_ } == 8) {
        &(0);
    } else {
        ({
            __assert_fail_1(
                Ptr::<i8>::from_string_literal(b"s.a_ == 8"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    ({ SImpl::bump_me(&s.as_pointer()) });
    (if ({ (*s.borrow()).a_ } == 9) {
        &(0);
    } else {
        ({
            __assert_fail_1(
                Ptr::<i8>::from_string_literal(b"s.a_ == 9"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut d: D = D::new({ 3 });
    (if (d.a_ == 6) {
        &(0);
    } else {
        ({
            __assert_fail_1(
                Ptr::<i8>::from_string_literal(b"d.a_ == 6"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let cr: Ptr<S> = ({ SImpl::cref(&s.as_pointer()) });
    (if (cr.with(|__s| __s.a_) == 9) {
        &(0);
    } else {
        ({
            __assert_fail_1(
                Ptr::<i8>::from_string_literal(b"cr.a_ == 9"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let t: Value<S> = Rc::new(RefCell::new(S::new_1({ 0 })));
    (if ({
        let _o: Ptr<S> = (s.as_pointer());
        SImpl::is(&s.as_pointer(), _o)
    }) {
        &(0);
    } else {
        ({
            __assert_fail_1(
                Ptr::<i8>::from_string_literal(b"s.is(&s)"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if !({ SImpl::is(&s.as_pointer(), (t.as_pointer())) }) {
        &(0);
    } else {
        ({
            __assert_fail_1(
                Ptr::<i8>::from_string_literal(b"!s.is(&t)"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut p: Ptr<S> = Ptr::alloc(S::new_1({ 1 }));
    let mut q: Ptr<S> = ({ SImpl::returns_this_pointer(&p) });
    field!(q, a_).with_mut(|__v| __v.postfix_inc());
    (if (p.with(|__s| __s.a_) == 2) {
        &(0);
    } else {
        ({
            __assert_fail_1(
                Ptr::<i8>::from_string_literal(b"p->a_ == 2"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    p.delete();
    let mut h: Ptr<S> = Ptr::alloc(S::new_1({ 5 }));
    ({ SImpl::destroy(&h) });
    ({ SImpl::reset(&s.as_pointer()) });
    (if ({ (*s.borrow()).a_ } == 0) {
        &(0);
    } else {
        ({
            __assert_fail_1(
                Ptr::<i8>::from_string_literal(b"s.a_ == 0"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ({ (*s.borrow()).self__.clone() }).is_null() {
        &(0);
    } else {
        ({
            __assert_fail_1(
                Ptr::<i8>::from_string_literal(b"s.self_ == nullptr"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ((({
        let _other: Ptr<S> = (s.as_pointer());
        SImpl::copy_if_different(&s.as_pointer(), _other)
    }) as i32)
        == (false as i32))
    {
        &(0);
    } else {
        ({
            __assert_fail_1(
                Ptr::<i8>::from_string_literal(b"s.copy_if_different(&s) == false"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ((({
        let _other: Ptr<S> = (s.as_pointer());
        SImpl::copy_if_different_const(&s.as_pointer(), _other)
    }) as i32)
        == (false as i32))
    {
        &(0);
    } else {
        ({
            __assert_fail_1(
                Ptr::<i8>::from_string_literal(b"s.copy_if_different_const(&s) == false"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let other: Value<S> = Rc::new(RefCell::new(S::new_1({ 22 })));
    (if ((({ SImpl::copy_if_different(&s.as_pointer(), (other.as_pointer())) }) as i32)
        == (true as i32))
    {
        &(0);
    } else {
        ({
            __assert_fail_1(
                Ptr::<i8>::from_string_literal(b"s.copy_if_different(&other) == true"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ({ (*s.borrow()).a_ } == { (*other.borrow()).a_ }) {
        &(0);
    } else {
        ({
            __assert_fail_1(
                Ptr::<i8>::from_string_literal(b"s.a_ == other.a_"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ({ { (*s.borrow()).self__.clone() } } == { { (*other.borrow()).self__.clone() } }) {
        &(0);
    } else {
        ({
            __assert_fail_1(
                Ptr::<i8>::from_string_literal(b"s.self_ == other.self_"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut u: S = S::new_2({ 1 }, { (s.as_pointer()) });
    (if ({ (u.self__).clone() } == { (s.as_pointer()) }) {
        &(0);
    } else {
        ({
            __assert_fail_1(
                Ptr::<i8>::from_string_literal(b"u.self_ == &s"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let s_const: Value<S> = Rc::new(RefCell::new(S::new_1({ 100 })));
    let mut u1: S = S::new_3({ 1 }, { (s_const.as_pointer()) });
    (if (u1.self__).is_null() {
        &(0);
    } else {
        ({
            __assert_fail_1(
                Ptr::<i8>::from_string_literal(b"u1.self_ == nullptr"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    return 0;
}
pub fn __assert_fail_1(
    mut __assertion: Ptr<i8>,
    mut __file: Ptr<i8>,
    mut __line: u32,
    mut __function: Ptr<i8>,
) {
    unimplemented!()
}
pub trait SImpl {
    fn returns_this_reference(&self) -> Ptr<S>;
    fn returns_this_pointer(&self) -> Ptr<S>;
    fn inc(&self) -> Ptr<S>;
    fn set_from_this(&self);
    fn get(&self) -> i32;
    fn twice(&self) -> i32;
    fn link(&self);
    fn bump_me(&self);
    fn cref(&self) -> Ptr<S>;
    fn is(&self, o: Ptr<S>) -> bool;
    fn destroy(&self);
    fn reset(&self);
    fn copy_if_different_const(&self, other: Ptr<S>) -> bool;
    fn copy_if_different(&self, other: Ptr<S>) -> bool;
}
impl SImpl for Ptr<S> {
    fn returns_this_reference(&self) -> Ptr<S> {
        return (*self).clone();
    }
    fn returns_this_pointer(&self) -> Ptr<S> {
        return (*self).clone();
    }
    fn inc(&self) -> Ptr<S> {
        field!((*self), a_).with_mut(|__v| __v.postfix_inc());
        return (*self).clone();
    }
    fn set_from_this(&self) {
        field!((*self), a_).write({ ((*self).with(|__s| __s.a_) + 1) });
    }
    fn get(&self) -> i32 {
        return (*self).with(|__s| __s.a_);
    }
    fn twice(&self) -> i32 {
        return (({ SImpl::get(self) }) * 2);
    }
    fn link(&self) {
        field!((*self), self__).write((*self).clone());
    }
    fn bump_me(&self) {
        ({ bump_0((*self).clone()) });
    }
    fn cref(&self) -> Ptr<S> {
        return (*self).clone();
    }
    fn is(&self, mut o: Ptr<S>) -> bool {
        return (o == (*self));
    }
    fn destroy(&self) {
        (*self).delete();
    }
    fn reset(&self) {
        (*self).write(S::new_1({ 0 }));
    }
    fn copy_if_different_const(&self, mut other: Ptr<S>) -> bool {
        if ((*self) == other) {
            return false;
        }
        field!((*self), a_).write({ other.with(|__s| __s.a_) });
        field!((*self), self__).write({ other.with(|__s| __s.self__.clone()) });
        return true;
    }
    fn copy_if_different(&self, mut other: Ptr<S>) -> bool {
        if ((*self) == other) {
            return false;
        }
        field!((*self), a_).write({ other.with(|__s| __s.a_) });
        field!((*self), self__).write({ other.with(|__s| __s.self__.clone()) });
        return true;
    }
}
pub fn __cpp2rust_init_globals() {}
