extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(24)]
pub struct Sorted__lambda_at_lambda_decltype_type_cpp_22_18__ {
    #[offset(0)]
    #[byte_size(16)]
    pub items: Box<[()]>,
    #[offset(16)]
    pub size: (),
    #[offset(20)]
    #[byte_size(1)]
    pub cmp: FnPtr<fn((), ()) -> ()>,
}
impl Default for Sorted__lambda_at_lambda_decltype_type_cpp_22_18__ {
    fn default() -> Self {
        {
            Sorted__lambda_at_lambda_decltype_type_cpp_22_18__ {
                items: (0..4).map(|_| unimplemented!()).collect::<Box<[()]>>(),
                size: unimplemented!(),
                cmp: FnPtr::<fn((), ()) -> ()>::new(|mut a: (), mut b: ()| -> () {
                    {
                        return (a > b);
                    }
                }),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(24)]
pub struct Sorted__lambda_at_lambda_decltype_type_cpp_19_23__ {
    #[offset(0)]
    #[byte_size(16)]
    pub items: Box<[()]>,
    #[offset(16)]
    pub size: (),
    #[offset(20)]
    #[byte_size(1)]
    pub cmp: FnPtr<fn((), ()) -> ()>,
}
impl Default for Sorted__lambda_at_lambda_decltype_type_cpp_19_23__ {
    fn default() -> Self {
        {
            Sorted__lambda_at_lambda_decltype_type_cpp_19_23__ {
                items: (0..4).map(|_| unimplemented!()).collect::<Box<[()]>>(),
                size: unimplemented!(),
                cmp: FnPtr::<fn((), ()) -> ()>::new(|mut a: (), mut b: ()| -> () {
                    {
                        return (a < b);
                    }
                }),
            }
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let greater: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(
        FnPtr::<fn((), ()) -> ()>::new(|mut a: (), mut b: ()| -> () {
            {
                return (a > b);
            }
        }),
    ));
    let desc: Value<Sorted__lambda_at_lambda_decltype_type_cpp_22_18__> = Rc::new(RefCell::new(
        <Sorted__lambda_at_lambda_decltype_type_cpp_22_18__>::default(),
    ));
    ({ Sorted__lambda_at_lambda_decltype_type_cpp_22_18__Impl::insert(&desc.as_pointer(), 2_()) });
    ({ Sorted__lambda_at_lambda_decltype_type_cpp_22_18__Impl::insert(&desc.as_pointer(), 7_()) });
    ({ Sorted__lambda_at_lambda_decltype_type_cpp_22_18__Impl::insert(&desc.as_pointer(), 4_()) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let asc: Value<Sorted__lambda_at_lambda_decltype_type_cpp_19_23__> = Rc::new(RefCell::new(
        <Sorted__lambda_at_lambda_decltype_type_cpp_19_23__>::default(),
    ));
    ({ Sorted__lambda_at_lambda_decltype_type_cpp_19_23__Impl::insert(&asc.as_pointer(), 2_()) });
    ({ Sorted__lambda_at_lambda_decltype_type_cpp_19_23__Impl::insert(&asc.as_pointer(), 7_()) });
    ({ Sorted__lambda_at_lambda_decltype_type_cpp_19_23__Impl::insert(&asc.as_pointer(), 4_()) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let fresh: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(
        FnPtr::<fn((), ()) -> ()>::new(|mut a: (), mut b: ()| -> () {
            {
                return (a > b);
            }
        }),
    ));
    unimplemented!();
    unimplemented!();
    let assigned: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(
        FnPtr::<fn((), ()) -> ()>::new(|mut a: (), mut b: ()| -> () {
            {
                return (a > b);
            }
        }),
    ));
    (*assigned.borrow_mut()) = (*greater.borrow()).clone();
    unimplemented!();
    let less: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(
        FnPtr::<fn((), ()) -> ()>::new(|mut a: (), mut b: ()| -> () {
            {
                return (a < b);
            }
        }),
    ));
    unimplemented!();
    return unimplemented!();
}
pub trait Sorted__lambda_at_lambda_decltype_type_cpp_19_23__Impl {
    fn insert(&self, v: ());
}
impl Sorted__lambda_at_lambda_decltype_type_cpp_19_23__Impl
    for Ptr<Sorted__lambda_at_lambda_decltype_type_cpp_19_23__>
{
    fn insert(&self, mut v: ()) {
        let mut i: () = (*self).with(|__s| __s.size);
        'loop_: while (i > unimplemented!())
            && ({
                let _b: () = (array_elem!((*self), items, (i - 1_())).read());
                (*(*self).upgrade().deref()).cmp.call(v, _b)
            })
        {
            array_elem!((*self), items, i)
                .write({ (array_elem!((*self), items, (i - 1_())).read()) });
            i.postfix_dec();
        }
        array_elem!((*self), items, i).write(v);
        field!((*self), size).with_mut(|__v| __v.postfix_inc());
    }
}
pub trait Sorted__lambda_at_lambda_decltype_type_cpp_22_18__Impl {
    fn insert(&self, v: ());
}
impl Sorted__lambda_at_lambda_decltype_type_cpp_22_18__Impl
    for Ptr<Sorted__lambda_at_lambda_decltype_type_cpp_22_18__>
{
    fn insert(&self, mut v: ()) {
        let mut i: () = (*self).with(|__s| __s.size);
        'loop_: while (i > unimplemented!())
            && ({
                let _b: () = (array_elem!((*self), items, (i - 1_())).read());
                (*(*self).upgrade().deref()).cmp.call(v, _b)
            })
        {
            array_elem!((*self), items, i)
                .write({ (array_elem!((*self), items, (i - 1_())).read()) });
            i.postfix_dec();
        }
        array_elem!((*self), items, i).write(v);
        field!((*self), size).with_mut(|__v| __v.postfix_inc());
    }
}
pub fn __cpp2rust_init_globals() {}
