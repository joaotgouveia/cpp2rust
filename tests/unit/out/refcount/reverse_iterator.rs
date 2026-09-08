extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Default)]
pub struct Foo {
    pub v: Value<i32>,
}
impl Clone for Foo {
    fn clone(&self) -> Self {
        let __this: Value<Foo> = Rc::new(RefCell::new(Self {
            v: Rc::new(RefCell::new((*self.v.borrow()))),
        }));
        let this: Ptr<Foo> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for Foo {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.v.borrow()).to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            v: Rc::new(RefCell::new(<i32>::from_bytes(&buf[0..4]))),
        }
    }
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let a1: Value<Box<[Foo]>> = Rc::new(RefCell::new(Box::new([
        Foo {
            v: Rc::new(RefCell::new(10)),
        },
        Foo {
            v: Rc::new(RefCell::new(20)),
        },
        Foo {
            v: Rc::new(RefCell::new(30)),
        },
        Foo {
            v: Rc::new(RefCell::new(40)),
        },
        Foo {
            v: Rc::new(RefCell::new(50)),
        },
    ])));
    let def: Value<Ptr<Foo>> = Rc::new(RefCell::new(Default::default()));
    let first: Value<Ptr<Foo>> = Rc::new(RefCell::new(
        (a1.as_pointer() as Ptr<Foo>).offset((5) as isize),
    ));
    assert!(((*first.borrow()) == (a1.as_pointer() as Ptr<Foo>).offset((5) as isize)));
    let ref_: Ptr<Foo> = (*first.borrow()).offset(-1);
    assert!((({ FooImpl::get(&ref_,) }) == 50));
    assert!((({ FooImpl::get(&(*first.borrow()).offset(-1),) }) == 50));
    (*first.borrow_mut()).prefix_dec();
    assert!((({ FooImpl::get(&(*first.borrow()).offset(-1),) }) == 40));
    let inc: Value<Ptr<Foo>> = Rc::new(RefCell::new((*first.borrow_mut()).postfix_dec()));
    assert!((({ FooImpl::get(&(*inc.borrow()).offset(-1),) }) == 40));
    assert!((({ FooImpl::get(&(*first.borrow()).offset(-1),) }) == 30));
    let n: Value<isize> = Rc::new(RefCell::new(2_isize));
    let plus: Value<Ptr<Foo>> = Rc::new(RefCell::new(
        (*first.borrow()).offset(Into::<isize>::into(-(*n.borrow()))),
    ));
    assert!((({ FooImpl::get(&(*plus.borrow()).offset(-1),) }) == 10));
    let minus: Value<Ptr<Foo>> = Rc::new(RefCell::new(
        (*plus.borrow()).offset(Into::<isize>::into((*n.borrow()))),
    ));
    assert!((({ FooImpl::get(&(*minus.borrow()).offset(-1),) }) == 30));
    assert!((*minus.borrow()) == (*first.borrow()));
    return 0;
}
pub trait FooImpl {
    fn get(&self) -> i32;
}
impl FooImpl for Ptr<Foo> {
    fn get(&self) -> i32 {
        return (*(*(*self).upgrade().deref()).v.borrow());
    }
}
