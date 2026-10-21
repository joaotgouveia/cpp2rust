extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let mut __do_while = true;
    'loop_: while __do_while || (unimplemented!() != 0) {
        __do_while = false;
        let mut p: Ptr<()> = unimplemented!().reinterpret_cast::<()>();
        p.write(42_());
        unimplemented!();
        unimplemented!();
        let mut arr: Ptr<()> = unimplemented!().reinterpret_cast::<()>();
        let mut i: () = unimplemented!();
        'loop_: while (((i < 4_()) as ()) != 0) {
            elem!(arr, i).write({ (i * 10_()) });
            i.postfix_inc();
        }
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut grow: Ptr<()> = unimplemented!().reinterpret_cast::<()>();
        elem!(grow, unimplemented!()).write(1_());
        elem!(grow, 1_()).write(2_());
        let __rhs = unimplemented!().reinterpret_cast::<()>();
        grow = __rhs;
        elem!(grow, 2_()).write(3_());
        elem!(grow, 3_()).write(4_());
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut zeros: Ptr<()> = unimplemented!().reinterpret_cast::<()>();
        let mut i: () = unimplemented!();
        'loop_: while (((i < 4_()) as ()) != 0) {
            unimplemented!();
            i.postfix_inc();
        }
        unimplemented!();
    }
    let mut pmalloc: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(libcc2rs::malloc_refcount);
    let mut pfree: FnPtr<fn(())> = FnPtr::<fn(())>::new(libcc2rs::free_refcount);
    let mut prealloc: FnPtr<fn((), ()) -> ()> =
        FnPtr::<fn((), ()) -> ()>::new(libcc2rs::realloc_refcount);
    let mut pcalloc: FnPtr<fn((), ()) -> ()> =
        FnPtr::<fn((), ()) -> ()>::new(libcc2rs::calloc_refcount);
    let mut __do_while = true;
    'loop_: while __do_while || (unimplemented!() != 0) {
        __do_while = false;
        let mut p: Ptr<()> =
            ({ pmalloc.call(::std::mem::size_of::<()>()) }).reinterpret_cast::<()>();
        p.write(42_());
        unimplemented!();
        ({ pfree.call((p).to_any()) });
        let mut arr: Ptr<()> =
            ({ pmalloc.call((4_()).wrapping_mul((::std::mem::size_of::<()>() as ()))) })
                .reinterpret_cast::<()>();
        let mut i: () = unimplemented!();
        'loop_: while (((i < 4_()) as ()) != 0) {
            elem!(arr, i).write({ (i * 10_()) });
            i.postfix_inc();
        }
        unimplemented!();
        unimplemented!();
        ({ pfree.call((arr).to_any()) });
        let mut grow: Ptr<()> =
            ({ pmalloc.call((2_()).wrapping_mul((::std::mem::size_of::<()>() as ()))) })
                .reinterpret_cast::<()>();
        elem!(grow, unimplemented!()).write(1_());
        elem!(grow, 1_()).write(2_());
        let __rhs = ({
            prealloc.call(
                (grow).to_any(),
                (4_()).wrapping_mul((::std::mem::size_of::<()>() as ())),
            )
        })
        .reinterpret_cast::<()>();
        grow = __rhs;
        elem!(grow, 2_()).write(3_());
        elem!(grow, 3_()).write(4_());
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        ({ pfree.call((grow).to_any()) });
        let mut zeros: Ptr<()> =
            ({ pcalloc.call(4_(), ::std::mem::size_of::<()>()) }).reinterpret_cast::<()>();
        let mut i: () = unimplemented!();
        'loop_: while (((i < 4_()) as ()) != 0) {
            unimplemented!();
            i.postfix_inc();
        }
        ({ pfree.call((zeros).to_any()) });
    }
    return unimplemented!();
}
pub fn __cpp2rust_init_globals() {}
