extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_0() {
    let ints: Value<()> = Rc::new(RefCell::new(
        ({
            let ___e: Value<()> = Rc::new(RefCell::new(0_()));
            let ___f: Value<()> = Rc::new(RefCell::new(5_()));
            (*iota_1.with(Value::clone).borrow_mut())(___e.as_pointer(), ___f.as_pointer())
        })
        .clone(),
    ));
    let unique_ptr_maker: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {},
        |mut v: ()| -> () {
            return unimplemented!();
        },
        |mut v: ()| -> () {
            return unimplemented!();
        }
    )));
    {
        let consume: Value<FnPtr<Generic>> =
            Rc::new(RefCell::new(lambda!(Generic, {}, |mut _a0: ()| {})));
        const _: () = assert!(
            true,
            "std::indirectly_unary_invocable<decltype(consume), projected_iter>"
        );;
        ({
            (*for_each_2.with(Value::clone).borrow_mut())(
                ints.as_pointer(),
                (*consume.borrow()).copy_from(),
                (*unique_ptr_maker.borrow()).copy_from(),
            )
        });
        ({
            let ___first: () = unimplemented!();
            let ___last: () = unimplemented!();
            (*for_each_2.with(Value::clone).borrow_mut())(
                ___first,
                ___last,
                (*consume.borrow()).copy_from(),
                (*unique_ptr_maker.borrow()).copy_from(),
            )
        });
    }
    {
        const _: () = assert!(
            true,
            "std::indirectly_regular_unary_invocable<decltype([](auto) {{}}), projected_iter>"
        );;
    }
    {
        let unary_pred: Value<FnPtr<Generic>> =
            Rc::new(RefCell::new(lambda!(Generic, {}, |mut _a0: ()| -> () {
                return false;
            })));
        const _: () = assert!(
            true,
            "std::indirect_unary_predicate<decltype(unary_pred), projected_iter>"
        );;
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let binary_pred: Value<FnPtr<Generic>> =
            Rc::new(RefCell::new(lambda!(Generic, {}, |mut _a0: (),
                                                       mut _a1: ()|
             -> () {
                return false;
            })));
        const _: () = assert!(
            true,
            "std::indirect_binary_predicate<decltype(binary_pred), projected_iter, projected_iter>"
        );;
        unimplemented!();
        unimplemented!();
    }
    {
        let rel: Value<FnPtr<Generic>> =
            Rc::new(RefCell::new(lambda!(Generic, {}, |mut _a0: (),
                                                       mut _a1: ()|
             -> () {
                return false;
            })));
        const _: () = assert!(
            true,
            "std::indirect_equivalence_relation<decltype(rel), projected_iter>"
        );;
        let mut out: () = unimplemented!();
        &({
            (*unique_copy_3.with(Value::clone).borrow_mut())(
                ints.as_pointer(),
                unimplemented!(),
                (*rel.borrow()).copy_from(),
                (*unique_ptr_maker.borrow()).copy_from(),
            )
        });
        &({
            let ___first: () = unimplemented!();
            let ___last: () = unimplemented!();
            (*unique_copy_3.with(Value::clone).borrow_mut())(
                ___first,
                ___last,
                unimplemented!(),
                (*rel.borrow()).copy_from(),
                (*unique_ptr_maker.borrow()).copy_from(),
            )
        });
    }
    {
        let rel: Value<FnPtr<Generic>> =
            Rc::new(RefCell::new(lambda!(Generic, {}, |mut x: (),
                                                       mut y: ()|
             -> () {
                return ((unimplemented!().read()) < (unimplemented!().read()));
            })));
        const _: () = assert!(
            true,
            "std::indirect_strict_weak_order<decltype(rel), projected_iter>"
        );;
        unimplemented!();
        unimplemented!();
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    ({ test_0() });
    const _: () = assert!(true, "(test(), true)");;
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
