extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor)]
#[byte_size(1)]
pub struct Guard__lambda_at_lambda_as_field_cpp_34_13__ {
    #[offset(0)]
    #[byte_size(1)]
    pub f: FnPtr<fn()>,
}
impl Default for Guard__lambda_at_lambda_as_field_cpp_34_13__ {
    fn default() -> Self {
        {
            Guard__lambda_at_lambda_as_field_cpp_34_13__ {
                f: FnPtr::<fn()>::new(|| {
                    (*count_0.with(Value::clone).borrow_mut()).postfix_inc();
                }),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor, Default)]
#[byte_size(8)]
pub struct Guard__lambda_at_lambda_as_field_cpp_45_13__ {
    #[offset(0)]
    #[byte_size(8)]
    pub f: FnPtr<fn()>,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct Holder__lambda_at_lambda_as_field_cpp_39_12__ {
    #[offset(0)]
    #[byte_size(1)]
    pub f: FnPtr<fn(()) -> ()>,
    #[offset(4)]
    pub calls: (),
}
impl Default for Holder__lambda_at_lambda_as_field_cpp_39_12__ {
    fn default() -> Self {
        {
            Holder__lambda_at_lambda_as_field_cpp_39_12__ {
                f: FnPtr::<fn(()) -> ()>::new(|mut x: ()| -> () {
                    {
                        return (x + 1_());
                    }
                }),
                calls: unimplemented!(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Holder__lambda_at_lambda_as_field_cpp_51_12__ {
    #[offset(0)]
    #[byte_size(4)]
    pub f: FnPtr<fn(()) -> ()>,
    #[offset(4)]
    pub calls: (),
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor)]
#[byte_size(8)]
pub struct Moving__lambda_at_lambda_as_field_cpp_64_12__ {
    #[offset(0)]
    #[byte_size(1)]
    pub f: FnPtr<fn(()) -> ()>,
    #[offset(4)]
    pub moves: (),
}
impl Moving__lambda_at_lambda_as_field_cpp_64_12__ {
    pub fn new(f: FnPtr<fn(()) -> ()>) -> Self {
        let f: Value<FnPtr<fn(()) -> ()>> = Rc::new(RefCell::new(f));
        Self {
            f: (*f.borrow()).copy_from(),
            moves: 0_(),
        }
    }
    pub fn move_from(o: Ptr<Moving__lambda_at_lambda_as_field_cpp_64_12__>) -> Self {
        Self {
            f: (*unimplemented!().upgrade().deref()).move_from(),
            moves: (o.with(|__s| __s.moves) + 1_()),
        }
    }
}
impl Default for Moving__lambda_at_lambda_as_field_cpp_64_12__ {
    fn default() -> Self {
        {
            Moving__lambda_at_lambda_as_field_cpp_64_12__ {
                f: FnPtr::<fn(()) -> ()>::new(|mut x: ()| -> () {
                    {
                        return (x * 2_());
                    }
                }),
                moves: 0_(),
            }
        }
    }
}
pub fn wrap_1(fn_: FnPtr<fn(()) -> ()>) -> FnPtr<fn(()) -> ()> {
    let fn_: Value<FnPtr<fn(()) -> ()>> = Rc::new(RefCell::new(fn_));
    return lambda!(
        {
            let fn_: Value<FnPtr<fn(()) -> ()>> =
                Rc::new(RefCell::new((*fn_.borrow()).copy_from()));
        },
        |mut x: ()| -> () {
            return (({ (*fn_.borrow()).call(x) }) + 1_());
        }
    );
}
pub fn wrap_2(fn_: FnPtr<fn(()) -> ()>) -> FnPtr<fn(()) -> ()> {
    let fn_: Value<FnPtr<fn(()) -> ()>> = Rc::new(RefCell::new(fn_));
    return lambda!(
        {
            let fn_: Value<FnPtr<fn(()) -> ()>> =
                Rc::new(RefCell::new((*fn_.borrow()).copy_from()));
        },
        |mut x: ()| -> () {
            return (({ (*fn_.borrow()).call(x) }) + 1_());
        }
    );
}
thread_local!(
    pub static count_0: Value<()> = Rc::new(RefCell::new(0_()));
);
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    {
        let g: Value<Guard__lambda_at_lambda_as_field_cpp_34_13__> =
            Rc::new(RefCell::new(Guard__lambda_at_lambda_as_field_cpp_34_13__ {
                f: FnPtr::<fn()>::new(|| {
                    (*count_0.with(Value::clone).borrow_mut()).postfix_inc();
                }),
            }));
        let _dtor_g = ScopedDestructor::new(&g, |__p| __p.destructor());
        unimplemented!();
    }
    unimplemented!();
    let n: Value<Holder__lambda_at_lambda_as_field_cpp_39_12__> = Rc::new(RefCell::new(
        Holder__lambda_at_lambda_as_field_cpp_39_12__ {
            f: FnPtr::<fn(()) -> ()>::new(|mut x: ()| -> () {
                {
                    return (x + 1_());
                }
            }),
            calls: 0_(),
        },
    ));
    unimplemented!();
    unimplemented!();
    let cleaned: Value<()> = Rc::new(RefCell::new(0_()));
    {
        let g: Value<Guard__lambda_at_lambda_as_field_cpp_45_13__> =
            Rc::new(RefCell::new(Guard__lambda_at_lambda_as_field_cpp_45_13__ {
                f: lambda!(
                    {
                        let cleaned: Ptr<()> = cleaned.as_pointer();
                    },
                    || {
                        cleaned.with_mut(|__v| __v.postfix_inc());
                    }
                ),
            }));
        let _dtor_g = ScopedDestructor::new(&g, |__p| __p.destructor());
        unimplemented!();
    }
    unimplemented!();
    let factor: Value<()> = Rc::new(RefCell::new(3_()));
    let h: Value<Holder__lambda_at_lambda_as_field_cpp_51_12__> = Rc::new(RefCell::new(
        Holder__lambda_at_lambda_as_field_cpp_51_12__ {
            f: lambda!(
                {
                    let factor: Value<()> = Rc::new(RefCell::new((*factor.borrow())));
                },
                |mut x: ()| -> () {
                    return (x * (*factor.borrow()));
                }
            ),
            calls: 0_(),
        },
    ));
    (*factor.borrow_mut()) = 100_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let w: Value<FnPtr<fn(()) -> ()>> = Rc::new(RefCell::new(
        ({
            wrap_1(lambda!(
                {
                    let factor: Value<()> = Rc::new(RefCell::new((*factor.borrow())));
                },
                |mut x: ()| -> () {
                    return (x * (*factor.borrow()));
                }
            ))
        }),
    ));
    (*factor.borrow_mut()) = 7_();
    unimplemented!();
    let ww: Value<FnPtr<fn(()) -> ()>> =
        Rc::new(RefCell::new(({ wrap_2((*w.borrow()).copy_from()) })));
    unimplemented!();
    let mut m: Moving__lambda_at_lambda_as_field_cpp_64_12__ =
        Moving__lambda_at_lambda_as_field_cpp_64_12__::new({
            FnPtr::<fn(()) -> ()>::new(|mut x: ()| -> () {
                {
                    return (x * 2_());
                }
            })
        });
    let m2: Value<Moving__lambda_at_lambda_as_field_cpp_64_12__> = Rc::new(RefCell::new(
        Moving__lambda_at_lambda_as_field_cpp_64_12__::move_from({ unimplemented!() }),
    ));
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub trait Guard__lambda_at_lambda_as_field_cpp_34_13__Impl {
    fn destructor(&self);
}
impl Guard__lambda_at_lambda_as_field_cpp_34_13__Impl
    for Ptr<Guard__lambda_at_lambda_as_field_cpp_34_13__>
{
    fn destructor(&self) {
        ({ (*(*self).upgrade().deref()).f.call() });
    }
}
pub trait Guard__lambda_at_lambda_as_field_cpp_45_13__Impl {
    fn destructor(&self);
}
impl Guard__lambda_at_lambda_as_field_cpp_45_13__Impl
    for Ptr<Guard__lambda_at_lambda_as_field_cpp_45_13__>
{
    fn destructor(&self) {
        ({ (*(*self).upgrade().deref()).f.call() });
    }
}
pub trait Holder__lambda_at_lambda_as_field_cpp_39_12__Impl {
    fn call(&self, x: ()) -> ();
}
impl Holder__lambda_at_lambda_as_field_cpp_39_12__Impl
    for Ptr<Holder__lambda_at_lambda_as_field_cpp_39_12__>
{
    fn call(&self, mut x: ()) -> () {
        field!((*self), calls).with_mut(|__v| __v.postfix_inc());
        return ({ (*(*self).upgrade().deref()).f.call(x) });
    }
}
pub trait Holder__lambda_at_lambda_as_field_cpp_51_12__Impl {
    fn call(&self, x: ()) -> ();
}
impl Holder__lambda_at_lambda_as_field_cpp_51_12__Impl
    for Ptr<Holder__lambda_at_lambda_as_field_cpp_51_12__>
{
    fn call(&self, mut x: ()) -> () {
        field!((*self), calls).with_mut(|__v| __v.postfix_inc());
        return ({ (*(*self).upgrade().deref()).f.call(x) });
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = count_0.with(|_| ());
}
