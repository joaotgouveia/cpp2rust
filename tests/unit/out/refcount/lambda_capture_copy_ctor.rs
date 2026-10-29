extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Record, ByteRepr, VaArg, FnPtrArg, MoveCtor)]
#[byte_size(8)]
pub struct Counted {
    #[offset(0)]
    pub copies: (),
    #[offset(4)]
    pub moves: (),
}
impl Counted {
    pub fn new() -> Self {
        Self {
            copies: 0_(),
            moves: 0_(),
        }
    }
    pub fn copy_from(o: Ptr<Counted>) -> Self {
        Self {
            copies: (o.with(|__s| __s.copies) + 1_()),
            moves: o.with(|__s| __s.moves),
        }
    }
    pub fn move_from(o: Ptr<Counted>) -> Self {
        Self {
            copies: o.with(|__s| __s.copies),
            moves: (o.with(|__s| __s.moves) + 1_()),
        }
    }
}
impl Clone for Counted {
    fn clone(&self) -> Self {
        let __src: Value<Counted> = Rc::new(RefCell::new(Counted {
            copies: self.copies.clone(),
            moves: self.moves.clone(),
        }));
        Counted::copy_from(__src.as_pointer())
    }
}
impl Default for Counted {
    fn default() -> Self {
        { Counted::new() }
    }
}
thread_local!(
    pub static drops_0: Value<()> = Rc::new(RefCell::new(0_()));
);
#[derive(Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, Destructor)]
#[byte_size(1)]
pub struct Dropped {}
impl Dropped {
    pub fn new() -> Self {
        Self {}
    }
    pub fn copy_from(_a0: Ptr<Dropped>) -> Self {
        Self {}
    }
    pub fn move_from(_a0: Ptr<Dropped>) -> Self {
        Self {}
    }
}
impl Clone for Dropped {
    fn clone(&self) -> Self {
        let __src: Value<Dropped> = Rc::new(RefCell::new(Dropped {}));
        Dropped::copy_from(__src.as_pointer())
    }
}
impl Default for Dropped {
    fn default() -> Self {
        { Dropped::new() }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let c: Value<Counted> = Rc::new(RefCell::new(Counted::new()));
    let f: Value<FnPtr<fn() -> ()>> = Rc::new(RefCell::new(lambda!(
        {
            let c: Value<Counted> = Rc::new(RefCell::new(Counted::copy_from({ c.as_pointer() })));
        },
        || -> () {
            return (({ (*c.borrow()).copies } * 10_()) + { (*c.borrow()).moves });
        }
    )));
    unimplemented!();
    let g: Value<FnPtr<fn() -> ()>> = Rc::new(RefCell::new((*f.borrow()).copy_from()));
    unimplemented!();
    unimplemented!();
    let h: Value<FnPtr<fn() -> ()>> = Rc::new(RefCell::new((*f.borrow()).move_from()));
    unimplemented!();
    let mut returned: () = {
        ({
            lambda!(
                {
                    let c: Value<Counted> =
                        Rc::new(RefCell::new(Counted::copy_from({ c.as_pointer() })));
                },
                || -> Counted {
                    return Counted::copy_from({ c.as_pointer() });
                }
            )
            .call()
        })
        .copies
    };
    unimplemented!();
    let arr: Value<Box<[Counted]>> = Rc::new(RefCell::new(Box::new(
        std::array::from_fn::<_, 2, _>(|_| Counted::new()),
    )));
    let a: Value<FnPtr<fn() -> ()>> = Rc::new(RefCell::new(lambda!(
        {
            let arr: Value<Box<[Counted]>> = Rc::new(RefCell::new(Box::new(
                std::array::from_fn::<_, 2, _>(|__i: usize| {
                    Counted::copy_from({ (arr.as_pointer() as Ptr<Counted>).offset(__i) })
                }),
            )));
        },
        || -> () {
            return ({ (*arr.borrow())[(0_()) as usize].copies } + {
                (*arr.borrow())[(1_()) as usize].copies
            });
        }
    )));
    unimplemented!();
    let a2: Value<FnPtr<fn() -> ()>> = Rc::new(RefCell::new((*a.borrow()).copy_from()));
    unimplemented!();
    let a3: Value<FnPtr<fn() -> ()>> = Rc::new(RefCell::new((*a.borrow()).move_from()));
    unimplemented!();
    {
        let m: Value<FnPtr<fn()>> = Rc::new(RefCell::new(lambda!(
            {
                let d: Value<Dropped> = Rc::new(RefCell::new(Dropped::new()));
            },
            || {}
        )));
        let _dtor_m = ScopedDestructor::new(&m, |__p| __p.with(|__f| __f.destroy()));
        let m2: Value<FnPtr<fn()>> = Rc::new(RefCell::new((*m.borrow()).move_from()));
        let _dtor_m2 = ScopedDestructor::new(&m2, |__p| __p.with(|__f| __f.destroy()));
    }
    unimplemented!();
    {
        let k: Value<FnPtr<fn()>> = Rc::new(RefCell::new(lambda!(
            {
                let d: Value<Dropped> = Rc::new(RefCell::new(Dropped::new()));
            },
            || {}
        )));
        let _dtor_k = ScopedDestructor::new(&k, |__p| __p.with(|__f| __f.destroy()));
        let k2: Value<FnPtr<fn()>> = Rc::new(RefCell::new((*k.borrow()).copy_from()));
        let _dtor_k2 = ScopedDestructor::new(&k2, |__p| __p.with(|__f| __f.destroy()));
    }
    unimplemented!();
    {
        let inner: Value<FnPtr<fn()>> = Rc::new(RefCell::new(lambda!(
            {
                let d: Value<Dropped> = Rc::new(RefCell::new(Dropped::new()));
            },
            || {}
        )));
        let _dtor_inner = ScopedDestructor::new(&inner, |__p| __p.with(|__f| __f.destroy()));
        let outer: Value<FnPtr<fn()>> = Rc::new(RefCell::new(lambda!(
            {
                let inner: Value<FnPtr<fn()>> =
                    Rc::new(RefCell::new((*inner.borrow()).copy_from()));
            },
            || {}
        )));
        let _dtor_outer = ScopedDestructor::new(&outer, |__p| __p.with(|__f| __f.destroy()));
    }
    unimplemented!();
    return 0_();
}
pub trait DroppedImpl {
    fn destructor(&self);
}
impl DroppedImpl for Ptr<Dropped> {
    fn destructor(&self) {
        (*drops_0.with(Value::clone).borrow_mut()).postfix_inc();
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = drops_0.with(|_| ());
}
