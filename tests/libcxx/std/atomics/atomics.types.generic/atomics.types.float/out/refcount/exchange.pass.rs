extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_seq_cst_0(store_op: FnPtr<fn(Ptr<()>, (), ())>, load_op: FnPtr<fn(Ptr<()>) -> ()>) {
    let store_op: Value<FnPtr<fn(Ptr<()>, (), ())>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> ()>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_1(store_op: FnPtr<fn(Ptr<()>, (), ())>, load_op: FnPtr<fn(Ptr<()>) -> ()>) {
    let store_op: Value<FnPtr<fn(Ptr<()>, (), ())>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> ()>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_2(store_op: FnPtr<fn(Ptr<()>, (), ())>, load_op: FnPtr<fn(Ptr<()>) -> ()>) {
    let store_op: Value<FnPtr<fn(Ptr<()>, (), ())>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> ()>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_3(store_op: FnPtr<fn(Ptr<()>, (), ())>, load_op: FnPtr<fn(Ptr<()>) -> ()>) {
    let store_op: Value<FnPtr<fn(Ptr<()>, (), ())>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> ()>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_4(store_op: FnPtr<fn(Ptr<()>, (), ())>, load_op: FnPtr<fn(Ptr<()>) -> ()>) {
    let store_op: Value<FnPtr<fn(Ptr<()>, (), ())>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> ()>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_5(store_op: FnPtr<fn(Ptr<()>, (), ())>, load_op: FnPtr<fn(Ptr<()>) -> ()>) {
    let store_op: Value<FnPtr<fn(Ptr<()>, (), ())>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> ()>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_6(store_op: FnPtr<fn(Ptr<()>, (), ())>, load_op: FnPtr<fn(Ptr<()>) -> ()>) {
    let store_op: Value<FnPtr<fn(Ptr<()>, (), ())>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> ()>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_7(store_op: FnPtr<fn(Ptr<()>, (), ())>, load_op: FnPtr<fn(Ptr<()>) -> ()>) {
    let store_op: Value<FnPtr<fn(Ptr<()>, (), ())>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> ()>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_8(
    store_op: FnPtr<fn(Ptr<()>, (), ())>,
    load_op: FnPtr<fn(Ptr<()>) -> ()>,
) {
    let store_op: Value<FnPtr<fn(Ptr<()>, (), ())>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> ()>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_9(
    store_op: FnPtr<fn(Ptr<()>, (), ())>,
    load_op: FnPtr<fn(Ptr<()>) -> ()>,
) {
    let store_op: Value<FnPtr<fn(Ptr<()>, (), ())>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> ()>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_10(
    store_op: FnPtr<fn(Ptr<()>, (), ())>,
    load_op: FnPtr<fn(Ptr<()>) -> ()>,
) {
    let store_op: Value<FnPtr<fn(Ptr<()>, (), ())>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> ()>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_11(
    store_op: FnPtr<fn(Ptr<()>, (), ())>,
    load_op: FnPtr<fn(Ptr<()>) -> ()>,
) {
    let store_op: Value<FnPtr<fn(Ptr<()>, (), ())>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> ()>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_impl_12() {
    const _: () = assert!(
        true,
        "noexcept(std::declval<MaybeVolatile<std::atomic<T>>&>() = (T(0)))"
    );;
    {
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let exchange: Value<FnPtr<fn(Ptr<()>, (), ())>> =
            Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, (), ())>::new(
                |x: Ptr<()>, mut _a1: (), mut new_val: ()| {
                    unimplemented!();
                },
            )));
        let load: Value<FnPtr<fn(Ptr<()>) -> ()>> = Rc::new(RefCell::new(
            FnPtr::<fn(Ptr<()>) -> ()>::new(|x: Ptr<()>| -> () {
                {
                    return unimplemented!();
                }
            }),
        ));
        ({
            test_acquire_release_8(
                (*exchange.borrow()).copy_from(),
                (*load.borrow()).copy_from(),
            )
        });
    }
    {
        let exchange_no_arg: Value<FnPtr<fn(Ptr<()>, (), ())>> =
            Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, (), ())>::new(
                |x: Ptr<()>, mut _a1: (), mut new_val: ()| {
                    unimplemented!();
                },
            )));
        let exchange_with_order: Value<FnPtr<fn(Ptr<()>, (), ())>> = Rc::new(RefCell::new(
            FnPtr::<fn(Ptr<()>, (), ())>::new(|x: Ptr<()>, mut _a1: (), mut new_val: ()| {
                unimplemented!();
            }),
        ));
        let load: Value<FnPtr<fn(Ptr<()>) -> ()>> = Rc::new(RefCell::new(
            FnPtr::<fn(Ptr<()>) -> ()>::new(|x: Ptr<()>| -> () {
                {
                    return unimplemented!();
                }
            }),
        ));
        ({
            test_seq_cst_0(
                (*exchange_no_arg.borrow()).copy_from(),
                (*load.borrow()).copy_from(),
            )
        });
        ({
            test_seq_cst_1(
                (*exchange_with_order.borrow()).copy_from(),
                (*load.borrow()).copy_from(),
            )
        });
    }
}
pub fn test_impl_13() {
    const _: () = assert!(
        true,
        "noexcept(std::declval<MaybeVolatile<std::atomic<T>>&>() = (T(0)))"
    );;
    {
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let exchange: Value<FnPtr<fn(Ptr<()>, (), ())>> =
            Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, (), ())>::new(
                |x: Ptr<()>, mut _a1: (), mut new_val: ()| {
                    unimplemented!();
                },
            )));
        let load: Value<FnPtr<fn(Ptr<()>) -> ()>> = Rc::new(RefCell::new(
            FnPtr::<fn(Ptr<()>) -> ()>::new(|x: Ptr<()>| -> () {
                {
                    return unimplemented!();
                }
            }),
        ));
        ({
            test_acquire_release_9(
                (*exchange.borrow()).copy_from(),
                (*load.borrow()).copy_from(),
            )
        });
    }
    {
        let exchange_no_arg: Value<FnPtr<fn(Ptr<()>, (), ())>> =
            Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, (), ())>::new(
                |x: Ptr<()>, mut _a1: (), mut new_val: ()| {
                    unimplemented!();
                },
            )));
        let exchange_with_order: Value<FnPtr<fn(Ptr<()>, (), ())>> = Rc::new(RefCell::new(
            FnPtr::<fn(Ptr<()>, (), ())>::new(|x: Ptr<()>, mut _a1: (), mut new_val: ()| {
                unimplemented!();
            }),
        ));
        let load: Value<FnPtr<fn(Ptr<()>) -> ()>> = Rc::new(RefCell::new(
            FnPtr::<fn(Ptr<()>) -> ()>::new(|x: Ptr<()>| -> () {
                {
                    return unimplemented!();
                }
            }),
        ));
        ({
            test_seq_cst_2(
                (*exchange_no_arg.borrow()).copy_from(),
                (*load.borrow()).copy_from(),
            )
        });
        ({
            test_seq_cst_3(
                (*exchange_with_order.borrow()).copy_from(),
                (*load.borrow()).copy_from(),
            )
        });
    }
}
pub fn test_impl_14() {
    const _: () = assert!(
        true,
        "noexcept(std::declval<MaybeVolatile<std::atomic<T>>&>() = (T(0)))"
    );;
    {
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let exchange: Value<FnPtr<fn(Ptr<()>, (), ())>> =
            Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, (), ())>::new(
                |x: Ptr<()>, mut _a1: (), mut new_val: ()| {
                    unimplemented!();
                },
            )));
        let load: Value<FnPtr<fn(Ptr<()>) -> ()>> = Rc::new(RefCell::new(
            FnPtr::<fn(Ptr<()>) -> ()>::new(|x: Ptr<()>| -> () {
                {
                    return unimplemented!();
                }
            }),
        ));
        ({
            test_acquire_release_10(
                (*exchange.borrow()).copy_from(),
                (*load.borrow()).copy_from(),
            )
        });
    }
    {
        let exchange_no_arg: Value<FnPtr<fn(Ptr<()>, (), ())>> =
            Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, (), ())>::new(
                |x: Ptr<()>, mut _a1: (), mut new_val: ()| {
                    unimplemented!();
                },
            )));
        let exchange_with_order: Value<FnPtr<fn(Ptr<()>, (), ())>> = Rc::new(RefCell::new(
            FnPtr::<fn(Ptr<()>, (), ())>::new(|x: Ptr<()>, mut _a1: (), mut new_val: ()| {
                unimplemented!();
            }),
        ));
        let load: Value<FnPtr<fn(Ptr<()>) -> ()>> = Rc::new(RefCell::new(
            FnPtr::<fn(Ptr<()>) -> ()>::new(|x: Ptr<()>| -> () {
                {
                    return unimplemented!();
                }
            }),
        ));
        ({
            test_seq_cst_4(
                (*exchange_no_arg.borrow()).copy_from(),
                (*load.borrow()).copy_from(),
            )
        });
        ({
            test_seq_cst_5(
                (*exchange_with_order.borrow()).copy_from(),
                (*load.borrow()).copy_from(),
            )
        });
    }
}
pub fn test_impl_15() {
    const _: () = assert!(
        true,
        "noexcept(std::declval<MaybeVolatile<std::atomic<T>>&>() = (T(0)))"
    );;
    {
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let exchange: Value<FnPtr<fn(Ptr<()>, (), ())>> =
            Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, (), ())>::new(
                |x: Ptr<()>, mut _a1: (), mut new_val: ()| {
                    unimplemented!();
                },
            )));
        let load: Value<FnPtr<fn(Ptr<()>) -> ()>> = Rc::new(RefCell::new(
            FnPtr::<fn(Ptr<()>) -> ()>::new(|x: Ptr<()>| -> () {
                {
                    return unimplemented!();
                }
            }),
        ));
        ({
            test_acquire_release_11(
                (*exchange.borrow()).copy_from(),
                (*load.borrow()).copy_from(),
            )
        });
    }
    {
        let exchange_no_arg: Value<FnPtr<fn(Ptr<()>, (), ())>> =
            Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, (), ())>::new(
                |x: Ptr<()>, mut _a1: (), mut new_val: ()| {
                    unimplemented!();
                },
            )));
        let exchange_with_order: Value<FnPtr<fn(Ptr<()>, (), ())>> = Rc::new(RefCell::new(
            FnPtr::<fn(Ptr<()>, (), ())>::new(|x: Ptr<()>, mut _a1: (), mut new_val: ()| {
                unimplemented!();
            }),
        ));
        let load: Value<FnPtr<fn(Ptr<()>) -> ()>> = Rc::new(RefCell::new(
            FnPtr::<fn(Ptr<()>) -> ()>::new(|x: Ptr<()>| -> () {
                {
                    return unimplemented!();
                }
            }),
        ));
        ({
            test_seq_cst_6(
                (*exchange_no_arg.borrow()).copy_from(),
                (*load.borrow()).copy_from(),
            )
        });
        ({
            test_seq_cst_7(
                (*exchange_with_order.borrow()).copy_from(),
                (*load.borrow()).copy_from(),
            )
        });
    }
}
pub fn test_16() {
    ({ test_impl_12() });
    {
        ({ test_impl_13() });
    }
}
pub fn test_17() {
    ({ test_impl_14() });
    {
        ({ test_impl_15() });
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
    ({ test_16() });
    ({ test_17() });
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
