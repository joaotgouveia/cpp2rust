extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn cmpxchg_weak_loop_0(atomic: Ptr<()>, expected: Ptr<()>, mut desired: ()) -> bool {
    let mut i: i32 = 0;
    'loop_: while (i < 10) {
        if ((unimplemented!() as i32) == (true as i32)) {
            return true;
        }
        i.postfix_inc();
    }
    return false;
}
pub fn cmpxchg_weak_loop_1(atomic: Ptr<()>, expected: Ptr<()>, mut desired: ()) -> bool {
    let mut i: i32 = 0;
    'loop_: while (i < 10) {
        if ((unimplemented!() as i32) == (true as i32)) {
            return true;
        }
        i.postfix_inc();
    }
    return false;
}
pub fn cmpxchg_weak_loop_2(
    atomic: Ptr<()>,
    expected: Ptr<()>,
    mut desired: (),
    mut success: (),
    mut failure: (),
) -> bool {
    let mut i: i32 = 0;
    'loop_: while (i < 10) {
        if ((unimplemented!() as i32) == (true as i32)) {
            return true;
        }
        i.postfix_inc();
    }
    return false;
}
pub fn cmpxchg_weak_loop_3(
    atomic: Ptr<()>,
    expected: Ptr<()>,
    mut desired: (),
    mut success: (),
    mut failure: (),
) -> bool {
    let mut i: i32 = 0;
    'loop_: while (i < 10) {
        if ((unimplemented!() as i32) == (true as i32)) {
            return true;
        }
        i.postfix_inc();
    }
    return false;
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> i32 {
    {
        let obj: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(((unimplemented!() as i32) == (true as i32)));
        {
            let mut lockfree: bool = unimplemented!();
            &(lockfree);
            {
                assert!(lockfree);
            }
        }
        unimplemented!();
        assert!(((unimplemented!() as i32) == (false as i32)));
        unimplemented!();
        assert!(((unimplemented!() as i32) == (true as i32)));
        assert!(((unimplemented!() as i32) == (true as i32)));
        assert!(((unimplemented!() as i32) == (true as i32)));
        assert!(((unimplemented!() as i32) == (true as i32)));
        assert!(((unimplemented!() as i32) == (false as i32)));
        assert!(((unimplemented!() as i32) == (false as i32)));
        assert!(((unimplemented!() as i32) == (true as i32)));
        let x: Value<bool> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            ((({
                cmpxchg_weak_loop_0(
                    obj.as_pointer(),
                    (x.as_pointer()).reinterpret_cast::<()>(),
                    (false as ()),
                )
            }) as i32)
                == (true as i32))
        );
        assert!(((unimplemented!() as i32) == (false as i32)));
        assert!((((*x.borrow()) as i32) == (true as i32)));
        assert!(((unimplemented!() as i32) == (false as i32)));
        assert!(((unimplemented!() as i32) == (false as i32)));
        assert!((((*x.borrow()) as i32) == (false as i32)));
        unimplemented!();
        (*x.borrow_mut()) = true;
        assert!(
            ((({
                let _success: () = unimplemented!();
                let _failure: () = unimplemented!();
                cmpxchg_weak_loop_2(
                    obj.as_pointer(),
                    (x.as_pointer()).reinterpret_cast::<()>(),
                    (false as ()),
                    _success,
                    _failure,
                )
            }) as i32)
                == (true as i32))
        );
        assert!(((unimplemented!() as i32) == (false as i32)));
        assert!((((*x.borrow()) as i32) == (true as i32)));
        (*x.borrow_mut()) = true;
        unimplemented!();
        assert!(((unimplemented!() as i32) == (true as i32)));
        assert!(((unimplemented!() as i32) == (false as i32)));
        assert!((((*x.borrow()) as i32) == (true as i32)));
        assert!(((unimplemented!() as i32) == (false as i32)));
        assert!(((unimplemented!() as i32) == (false as i32)));
        assert!((((*x.borrow()) as i32) == (false as i32)));
        (*x.borrow_mut()) = true;
        unimplemented!();
        assert!(((unimplemented!() as i32) == (true as i32)));
        assert!(((unimplemented!() as i32) == (false as i32)));
        assert!((((*x.borrow()) as i32) == (true as i32)));
        assert!((((unimplemented!()) as i32) == (false as i32)));
        assert!(((unimplemented!() as i32) == (false as i32)));
        assert!((((unimplemented!()) as i32) == (true as i32)));
        assert!(((unimplemented!() as i32) == (true as i32)));
    }
    {
        let obj: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(((unimplemented!() as i32) == (true as i32)));
        {
            let mut lockfree: bool = unimplemented!();
            &(lockfree);
            {
                assert!(lockfree);
            }
        }
        unimplemented!();
        assert!(((unimplemented!() as i32) == (false as i32)));
        unimplemented!();
        assert!(((unimplemented!() as i32) == (true as i32)));
        assert!(((unimplemented!() as i32) == (true as i32)));
        assert!(((unimplemented!() as i32) == (true as i32)));
        assert!(((unimplemented!() as i32) == (true as i32)));
        assert!(((unimplemented!() as i32) == (false as i32)));
        assert!(((unimplemented!() as i32) == (false as i32)));
        assert!(((unimplemented!() as i32) == (true as i32)));
        let x: Value<bool> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            ((({
                cmpxchg_weak_loop_1(
                    obj.as_pointer(),
                    (x.as_pointer()).reinterpret_cast::<()>(),
                    (false as ()),
                )
            }) as i32)
                == (true as i32))
        );
        assert!(((unimplemented!() as i32) == (false as i32)));
        assert!((((*x.borrow()) as i32) == (true as i32)));
        assert!(((unimplemented!() as i32) == (false as i32)));
        assert!(((unimplemented!() as i32) == (false as i32)));
        assert!((((*x.borrow()) as i32) == (false as i32)));
        unimplemented!();
        (*x.borrow_mut()) = true;
        assert!(
            ((({
                let _success: () = unimplemented!();
                let _failure: () = unimplemented!();
                cmpxchg_weak_loop_3(
                    obj.as_pointer(),
                    (x.as_pointer()).reinterpret_cast::<()>(),
                    (false as ()),
                    _success,
                    _failure,
                )
            }) as i32)
                == (true as i32))
        );
        assert!(((unimplemented!() as i32) == (false as i32)));
        assert!((((*x.borrow()) as i32) == (true as i32)));
        (*x.borrow_mut()) = true;
        unimplemented!();
        assert!(((unimplemented!() as i32) == (true as i32)));
        assert!(((unimplemented!() as i32) == (false as i32)));
        assert!((((*x.borrow()) as i32) == (true as i32)));
        assert!(((unimplemented!() as i32) == (false as i32)));
        assert!(((unimplemented!() as i32) == (false as i32)));
        assert!((((*x.borrow()) as i32) == (false as i32)));
        (*x.borrow_mut()) = true;
        unimplemented!();
        assert!(((unimplemented!() as i32) == (true as i32)));
        assert!(((unimplemented!() as i32) == (false as i32)));
        assert!((((*x.borrow()) as i32) == (true as i32)));
        assert!((((unimplemented!()) as i32) == (false as i32)));
        assert!(((unimplemented!() as i32) == (false as i32)));
        assert!((((unimplemented!()) as i32) == (true as i32)));
        assert!(((unimplemented!() as i32) == (true as i32)));
    }
    {
        let obj: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(((unimplemented!() as i32) == (true as i32)));
        {
            let mut lockfree: bool = unimplemented!();
            &(lockfree);
            {
                assert!(lockfree);
            }
        }
        unimplemented!();
        assert!(((unimplemented!() as i32) == (false as i32)));
        unimplemented!();
        assert!(((unimplemented!() as i32) == (true as i32)));
        assert!(((unimplemented!() as i32) == (true as i32)));
        assert!(((unimplemented!() as i32) == (true as i32)));
        assert!(((unimplemented!() as i32) == (true as i32)));
        assert!(((unimplemented!() as i32) == (false as i32)));
        assert!(((unimplemented!() as i32) == (false as i32)));
        assert!(((unimplemented!() as i32) == (true as i32)));
        let x: Value<bool> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            ((({
                cmpxchg_weak_loop_1(
                    obj.as_pointer(),
                    (x.as_pointer()).reinterpret_cast::<()>(),
                    (false as ()),
                )
            }) as i32)
                == (true as i32))
        );
        assert!(((unimplemented!() as i32) == (false as i32)));
        assert!((((*x.borrow()) as i32) == (true as i32)));
        assert!(((unimplemented!() as i32) == (false as i32)));
        assert!(((unimplemented!() as i32) == (false as i32)));
        assert!((((*x.borrow()) as i32) == (false as i32)));
        unimplemented!();
        (*x.borrow_mut()) = true;
        assert!(
            ((({
                let _success: () = unimplemented!();
                let _failure: () = unimplemented!();
                cmpxchg_weak_loop_3(
                    obj.as_pointer(),
                    (x.as_pointer()).reinterpret_cast::<()>(),
                    (false as ()),
                    _success,
                    _failure,
                )
            }) as i32)
                == (true as i32))
        );
        assert!(((unimplemented!() as i32) == (false as i32)));
        assert!((((*x.borrow()) as i32) == (true as i32)));
        (*x.borrow_mut()) = true;
        unimplemented!();
        assert!(((unimplemented!() as i32) == (true as i32)));
        assert!(((unimplemented!() as i32) == (false as i32)));
        assert!((((*x.borrow()) as i32) == (true as i32)));
        assert!(((unimplemented!() as i32) == (false as i32)));
        assert!(((unimplemented!() as i32) == (false as i32)));
        assert!((((*x.borrow()) as i32) == (false as i32)));
        (*x.borrow_mut()) = true;
        unimplemented!();
        assert!(((unimplemented!() as i32) == (true as i32)));
        assert!(((unimplemented!() as i32) == (false as i32)));
        assert!((((*x.borrow()) as i32) == (true as i32)));
        assert!((((unimplemented!()) as i32) == (false as i32)));
        assert!(((unimplemented!() as i32) == (false as i32)));
        assert!((((unimplemented!()) as i32) == (true as i32)));
        assert!(((unimplemented!() as i32) == (true as i32)));
    }
    {
        let storage: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([1_i8])));
        let zero: Ptr<()> = Ptr::alloc(Rc::new(RefCell::new(unimplemented!())).as_pointer());
        assert!(((unimplemented!() as i32) == (false as i32)));
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
