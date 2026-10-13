extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn apply_0(x: i32, fn_: FnPtr<fn(i32) -> i32>) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    let fn_: Value<FnPtr<fn(i32) -> i32>> = Rc::new(RefCell::new(fn_));
    return ({ (*fn_.borrow()).call((*x.borrow())) });
}
pub fn call_nttp_1(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    return ({ (FnPtr::<fn(i32) -> i32>::new(lambda_2::__invoke))((*x.borrow())) });
}
pub fn call_nttp_1(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    return ({ (FnPtr::<fn(i32) -> i32>::new(lambda_3::__invoke))((*x.borrow())) });
}
pub fn call_nttp_4(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    return ({ lambda_5::operator_call((*x.borrow())) });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let fresh: Value<FnPtr<fn(i32) -> i32>> = Rc::new(RefCell::new(
        ({ (<lambda_6>::default()).to_free_function() }),
    ));
    assert!((({ (*fresh.borrow()).call(5,) }) == -5_i32));
    let twice: Value<lambda_7> = Rc::new(RefCell::new((<lambda_7>::default())));
    let named: Value<FnPtr<fn(i32) -> i32>> =
        Rc::new(RefCell::new(({ (*twice.borrow()).to_free_function() })));
    assert!((({ (*named.borrow()).call(5,) }) == 10));
    assert!((({ apply_0(5, ({ (*twice.borrow()).to_free_function() }),) }) == 10));
    (*named.borrow_mut()) = (*fresh.borrow()).clone();
    assert!((({ (*named.borrow()).call(3,) }) == -3_i32));
    let p: Value<FnPtr<fn(i32) -> i32>> = Rc::new(RefCell::new(
        ({ (<lambda_8>::default()).to_free_function() }),
    ));
    assert!((({ (*p.borrow()).call(2,) }) == 6));
    assert!((({ call_nttp_1(5,) }) == 4));
    assert!((({ call_nttp_1(5,) }) == 50));
    assert!((({ call_nttp_4(5,) }) == 105));
    let sum: Value<lambda_9> = Rc::new(RefCell::new((<lambda_9>::default())));
    assert!((({ lambda_9::operator_call(2, &[(5).into(), (5).into(),]) }) == 10));
    assert!((({ lambda_9::operator_call(3, &[(1).into(), (2).into(), (3).into(),]) }) == 6));
    return 0;
}
#[derive(Clone, ByteRepr, Default)]
pub struct lambda_6 {}
impl lambda_6 {
    pub fn operator_call(x: i32) -> i32 {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        return -(*x.borrow());
    }
}
impl Callable1<i32, i32> for lambda_6 {
    fn call(&self, a1: i32) -> i32 {
        { lambda_6::operator_call(a1) }
    }
}
impl lambda_6 {
    pub fn to_free_function(&self) -> FnPtr<fn(i32) -> i32> {
        FnPtr::new(lambda_6::operator_call)
    }
}
#[derive(Clone, ByteRepr, Default)]
pub struct lambda_7 {}
impl lambda_7 {
    pub fn operator_call(x: i32) -> i32 {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        return ((*x.borrow()) * 2);
    }
}
impl Callable1<i32, i32> for lambda_7 {
    fn call(&self, a1: i32) -> i32 {
        { lambda_7::operator_call(a1) }
    }
}
impl lambda_7 {
    pub fn to_free_function(&self) -> FnPtr<fn(i32) -> i32> {
        FnPtr::new(lambda_7::operator_call)
    }
}
#[derive(Clone, ByteRepr, Default)]
pub struct lambda_8 {}
impl lambda_8 {
    pub fn operator_call(x: i32) -> i32 {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        return ((*x.borrow()) * 3);
    }
}
impl Callable1<i32, i32> for lambda_8 {
    fn call(&self, a1: i32) -> i32 {
        { lambda_8::operator_call(a1) }
    }
}
impl lambda_8 {
    pub fn to_free_function(&self) -> FnPtr<fn(i32) -> i32> {
        FnPtr::new(lambda_8::operator_call)
    }
}
#[derive(Clone, ByteRepr, Default)]
pub struct lambda_9 {}
impl lambda_9 {
    pub fn operator_call(n: i32, __args: &[VaArg]) -> i32 {
        let n: Value<i32> = Rc::new(RefCell::new(n));
        let ap: Value<VaList> = Rc::new(RefCell::new(VaList::default()));
        (*ap.borrow_mut()) = VaList::new(__args);
        let s: Value<i32> = Rc::new(RefCell::new(0));
        let i: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*i.borrow()) < (*n.borrow())) {
            (*s.borrow_mut()) += (*ap.borrow_mut()).arg::<i32>();
            (*i.borrow_mut()).postfix_inc();
        }
        return (*s.borrow());
    }
}
impl Callable1<i32, i32> for lambda_9 {
    fn call(&self, a1: i32) -> i32 {
        { lambda_9::operator_call(a1) }
    }
}
impl lambda_9 {
    pub fn to_free_function(&self) -> FnPtr<fn(i32) -> i32> {
        FnPtr::new(lambda_9::operator_call)
    }
}
pub fn __cpp2rust_init_globals() {}
