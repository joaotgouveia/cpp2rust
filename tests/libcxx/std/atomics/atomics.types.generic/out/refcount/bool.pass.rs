extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};

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
        let obj: Value<std_atomic_bool_> = Rc::new(RefCell::new(std_atomic_bool_::new_1({ true })));
        assert!(({ ((({ (*obj.borrow()).to_bool_10() }) as i32) == (true as i32)) }));
        {
            let mut lockfree: bool = ({ (*obj.borrow()).is_lock_free_12() });
            &(lockfree);
            {
                assert!(lockfree);
            }
        }
        ({ (*obj.borrow()).store_14(false, None) });
        assert!(({ ((({ (*obj.borrow()).to_bool_10() }) as i32) == (false as i32)) }));
        ({ (*obj.borrow()).store_14(true, Some(memory_order_release_0.with(|rc| *rc.borrow()))) });
        assert!(({ ((({ (*obj.borrow()).to_bool_10() }) as i32) == (true as i32)) }));
        assert!(({ ((({ (*obj.borrow()).load_16(None,) }) as i32) == (true as i32)) }));
        assert!(
            ({
                ((({
                    (*obj.borrow()).load_16(Some(memory_order_acquire_1.with(|rc| *rc.borrow())))
                }) as i32)
                    == (true as i32))
            })
        );
        assert!(({ ((({ (*obj.borrow()).exchange_18(false, None,) }) as i32) == (true as i32)) }));
        assert!(({ ((({ (*obj.borrow()).to_bool_10() }) as i32) == (false as i32)) }));
        assert!(
            ({
                ((({
                    (*obj.borrow())
                        .exchange_18(true, Some(memory_order_relaxed_2.with(|rc| *rc.borrow())))
                }) as i32)
                    == (false as i32))
            })
        );
        assert!(({ ((({ (*obj.borrow()).to_bool_10() }) as i32) == (true as i32)) }));
        let x: Value<bool> = Rc::new(RefCell::new(({ (*obj.borrow()).to_bool_10() })));
        assert!(
            ((({ cmpxchg_weak_loop_3(obj.as_pointer(), x.as_pointer(), false,) }) as i32)
                == (true as i32))
        );
        assert!(({ ((({ (*obj.borrow()).to_bool_10() }) as i32) == (false as i32)) }));
        assert!(({ (((*x.borrow()) as i32) == (true as i32)) }));
        assert!(
            ({
                ((({
                    (*obj.borrow()).compare_exchange_weak_22(
                        x.as_pointer(),
                        true,
                        Some(memory_order_seq_cst_4.with(|rc| *rc.borrow())),
                    )
                }) as i32)
                    == (false as i32))
            })
        );
        assert!(({ ((({ (*obj.borrow()).to_bool_10() }) as i32) == (false as i32)) }));
        assert!(({ (((*x.borrow()) as i32) == (false as i32)) }));
        ({ (*obj.borrow()).store_14(true, None) });
        (*x.borrow_mut()) = true;
        assert!(
            ((({
                let _success: std_memory_order = memory_order_seq_cst_4.with(|rc| *rc.borrow());
                let _failure: std_memory_order = memory_order_seq_cst_4.with(|rc| *rc.borrow());
                cmpxchg_weak_loop_5(obj.as_pointer(), x.as_pointer(), false, _success, _failure)
            }) as i32)
                == (true as i32))
        );
        assert!(({ ((({ (*obj.borrow()).to_bool_10() }) as i32) == (false as i32)) }));
        assert!(({ (((*x.borrow()) as i32) == (true as i32)) }));
        (*x.borrow_mut()) = true;
        ({ (*obj.borrow()).store_14(true, None) });
        assert!(
            ({
                ((({ (*obj.borrow()).compare_exchange_strong_26(x.as_pointer(), false, None) })
                    as i32)
                    == (true as i32))
            })
        );
        assert!(({ ((({ (*obj.borrow()).to_bool_10() }) as i32) == (false as i32)) }));
        assert!(({ (((*x.borrow()) as i32) == (true as i32)) }));
        assert!(
            ({
                ((({
                    (*obj.borrow()).compare_exchange_strong_26(
                        x.as_pointer(),
                        true,
                        Some(memory_order_seq_cst_4.with(|rc| *rc.borrow())),
                    )
                }) as i32)
                    == (false as i32))
            })
        );
        assert!(({ ((({ (*obj.borrow()).to_bool_10() }) as i32) == (false as i32)) }));
        assert!(({ (((*x.borrow()) as i32) == (false as i32)) }));
        (*x.borrow_mut()) = true;
        ({ (*obj.borrow()).store_14(true, None) });
        assert!(
            ({
                ((({
                    let ___m1: std_memory_order = memory_order_seq_cst_4.with(|rc| *rc.borrow());
                    let ___m2: std_memory_order = memory_order_seq_cst_4.with(|rc| *rc.borrow());
                    (*obj.borrow()).compare_exchange_strong_24(x.as_pointer(), false, ___m1, ___m2)
                }) as i32)
                    == (true as i32))
            })
        );
        assert!(({ ((({ (*obj.borrow()).to_bool_10() }) as i32) == (false as i32)) }));
        assert!(({ (((*x.borrow()) as i32) == (true as i32)) }));
        assert!(
            ({
                ((({
                    (*obj.borrow_mut()) = false;
                    (*obj.borrow()).clone()
                }) as i32)
                    == (false as i32))
            })
        );
        assert!(({ ((({ (*obj.borrow()).to_bool_10() }) as i32) == (false as i32)) }));
        assert!(
            ({
                ((({
                    (*obj.borrow_mut()) = true;
                    (*obj.borrow()).clone()
                }) as i32)
                    == (true as i32))
            })
        );
        assert!(({ ((({ (*obj.borrow()).to_bool_10() }) as i32) == (true as i32)) }));
    }
    {
        let obj: Value<std_atomic_bool_> = Rc::new(RefCell::new(std_atomic_bool_::new_1({ true })));
        assert!(({ ((({ (*obj.borrow()).to_bool_9() }) as i32) == (true as i32)) }));
        {
            let mut lockfree: bool = ({ (*obj.borrow()).is_lock_free_11() });
            &(lockfree);
            {
                assert!(lockfree);
            }
        }
        ({ (*obj.borrow()).store_13(false, None) });
        assert!(({ ((({ (*obj.borrow()).to_bool_9() }) as i32) == (false as i32)) }));
        ({ (*obj.borrow()).store_13(true, Some(memory_order_release_0.with(|rc| *rc.borrow()))) });
        assert!(({ ((({ (*obj.borrow()).to_bool_9() }) as i32) == (true as i32)) }));
        assert!(({ ((({ (*obj.borrow()).load_15(None,) }) as i32) == (true as i32)) }));
        assert!(
            ({
                ((({
                    (*obj.borrow()).load_15(Some(memory_order_acquire_1.with(|rc| *rc.borrow())))
                }) as i32)
                    == (true as i32))
            })
        );
        assert!(({ ((({ (*obj.borrow()).exchange_17(false, None,) }) as i32) == (true as i32)) }));
        assert!(({ ((({ (*obj.borrow()).to_bool_9() }) as i32) == (false as i32)) }));
        assert!(
            ({
                ((({
                    (*obj.borrow())
                        .exchange_17(true, Some(memory_order_relaxed_2.with(|rc| *rc.borrow())))
                }) as i32)
                    == (false as i32))
            })
        );
        assert!(({ ((({ (*obj.borrow()).to_bool_9() }) as i32) == (true as i32)) }));
        let x: Value<bool> = Rc::new(RefCell::new(({ (*obj.borrow()).to_bool_9() })));
        assert!(
            ((({ cmpxchg_weak_loop_6(obj.as_pointer(), x.as_pointer(), false,) }) as i32)
                == (true as i32))
        );
        assert!(({ ((({ (*obj.borrow()).to_bool_9() }) as i32) == (false as i32)) }));
        assert!(({ (((*x.borrow()) as i32) == (true as i32)) }));
        assert!(
            ({
                ((({
                    (*obj.borrow()).compare_exchange_weak_21(
                        x.as_pointer(),
                        true,
                        Some(memory_order_seq_cst_4.with(|rc| *rc.borrow())),
                    )
                }) as i32)
                    == (false as i32))
            })
        );
        assert!(({ ((({ (*obj.borrow()).to_bool_9() }) as i32) == (false as i32)) }));
        assert!(({ (((*x.borrow()) as i32) == (false as i32)) }));
        ({ (*obj.borrow()).store_13(true, None) });
        (*x.borrow_mut()) = true;
        assert!(
            ((({
                let _success: std_memory_order = memory_order_seq_cst_4.with(|rc| *rc.borrow());
                let _failure: std_memory_order = memory_order_seq_cst_4.with(|rc| *rc.borrow());
                cmpxchg_weak_loop_7(obj.as_pointer(), x.as_pointer(), false, _success, _failure)
            }) as i32)
                == (true as i32))
        );
        assert!(({ ((({ (*obj.borrow()).to_bool_9() }) as i32) == (false as i32)) }));
        assert!(({ (((*x.borrow()) as i32) == (true as i32)) }));
        (*x.borrow_mut()) = true;
        ({ (*obj.borrow()).store_13(true, None) });
        assert!(
            ({
                ((({ (*obj.borrow()).compare_exchange_strong_25(x.as_pointer(), false, None) })
                    as i32)
                    == (true as i32))
            })
        );
        assert!(({ ((({ (*obj.borrow()).to_bool_9() }) as i32) == (false as i32)) }));
        assert!(({ (((*x.borrow()) as i32) == (true as i32)) }));
        assert!(
            ({
                ((({
                    (*obj.borrow()).compare_exchange_strong_25(
                        x.as_pointer(),
                        true,
                        Some(memory_order_seq_cst_4.with(|rc| *rc.borrow())),
                    )
                }) as i32)
                    == (false as i32))
            })
        );
        assert!(({ ((({ (*obj.borrow()).to_bool_9() }) as i32) == (false as i32)) }));
        assert!(({ (((*x.borrow()) as i32) == (false as i32)) }));
        (*x.borrow_mut()) = true;
        ({ (*obj.borrow()).store_13(true, None) });
        assert!(
            ({
                ((({
                    let ___m1: std_memory_order = memory_order_seq_cst_4.with(|rc| *rc.borrow());
                    let ___m2: std_memory_order = memory_order_seq_cst_4.with(|rc| *rc.borrow());
                    (*obj.borrow()).compare_exchange_strong_23(x.as_pointer(), false, ___m1, ___m2)
                }) as i32)
                    == (true as i32))
            })
        );
        assert!(({ ((({ (*obj.borrow()).to_bool_9() }) as i32) == (false as i32)) }));
        assert!(({ (((*x.borrow()) as i32) == (true as i32)) }));
        assert!(
            ({
                ((({
                    (*obj.borrow_mut()) = false;
                    (*obj.borrow()).clone()
                }) as i32)
                    == (false as i32))
            })
        );
        assert!(({ ((({ (*obj.borrow()).to_bool_9() }) as i32) == (false as i32)) }));
        assert!(
            ({
                ((({
                    (*obj.borrow_mut()) = true;
                    (*obj.borrow()).clone()
                }) as i32)
                    == (true as i32))
            })
        );
        assert!(({ ((({ (*obj.borrow()).to_bool_9() }) as i32) == (true as i32)) }));
    }
    {
        let obj: Value<std_atomic_bool_> = Rc::new(RefCell::new(std_atomic_bool_::new_1({ true })));
        assert!(({ ((({ (*obj.borrow()).to_bool_9() }) as i32) == (true as i32)) }));
        {
            let mut lockfree: bool = ({ (*obj.borrow()).is_lock_free_11() });
            &(lockfree);
            {
                assert!(lockfree);
            }
        }
        ({ (*obj.borrow()).store_13(false, None) });
        assert!(({ ((({ (*obj.borrow()).to_bool_9() }) as i32) == (false as i32)) }));
        ({ (*obj.borrow()).store_13(true, Some(memory_order_release_0.with(|rc| *rc.borrow()))) });
        assert!(({ ((({ (*obj.borrow()).to_bool_9() }) as i32) == (true as i32)) }));
        assert!(({ ((({ (*obj.borrow()).load_15(None,) }) as i32) == (true as i32)) }));
        assert!(
            ({
                ((({
                    (*obj.borrow()).load_15(Some(memory_order_acquire_1.with(|rc| *rc.borrow())))
                }) as i32)
                    == (true as i32))
            })
        );
        assert!(({ ((({ (*obj.borrow()).exchange_17(false, None,) }) as i32) == (true as i32)) }));
        assert!(({ ((({ (*obj.borrow()).to_bool_9() }) as i32) == (false as i32)) }));
        assert!(
            ({
                ((({
                    (*obj.borrow())
                        .exchange_17(true, Some(memory_order_relaxed_2.with(|rc| *rc.borrow())))
                }) as i32)
                    == (false as i32))
            })
        );
        assert!(({ ((({ (*obj.borrow()).to_bool_9() }) as i32) == (true as i32)) }));
        let x: Value<bool> = Rc::new(RefCell::new(({ (*obj.borrow()).to_bool_9() })));
        assert!(
            ((({ cmpxchg_weak_loop_6(obj.as_pointer(), x.as_pointer(), false,) }) as i32)
                == (true as i32))
        );
        assert!(({ ((({ (*obj.borrow()).to_bool_9() }) as i32) == (false as i32)) }));
        assert!(({ (((*x.borrow()) as i32) == (true as i32)) }));
        assert!(
            ({
                ((({
                    (*obj.borrow()).compare_exchange_weak_21(
                        x.as_pointer(),
                        true,
                        Some(memory_order_seq_cst_4.with(|rc| *rc.borrow())),
                    )
                }) as i32)
                    == (false as i32))
            })
        );
        assert!(({ ((({ (*obj.borrow()).to_bool_9() }) as i32) == (false as i32)) }));
        assert!(({ (((*x.borrow()) as i32) == (false as i32)) }));
        ({ (*obj.borrow()).store_13(true, None) });
        (*x.borrow_mut()) = true;
        assert!(
            ((({
                let _success: std_memory_order = memory_order_seq_cst_4.with(|rc| *rc.borrow());
                let _failure: std_memory_order = memory_order_seq_cst_4.with(|rc| *rc.borrow());
                cmpxchg_weak_loop_7(obj.as_pointer(), x.as_pointer(), false, _success, _failure)
            }) as i32)
                == (true as i32))
        );
        assert!(({ ((({ (*obj.borrow()).to_bool_9() }) as i32) == (false as i32)) }));
        assert!(({ (((*x.borrow()) as i32) == (true as i32)) }));
        (*x.borrow_mut()) = true;
        ({ (*obj.borrow()).store_13(true, None) });
        assert!(
            ({
                ((({ (*obj.borrow()).compare_exchange_strong_25(x.as_pointer(), false, None) })
                    as i32)
                    == (true as i32))
            })
        );
        assert!(({ ((({ (*obj.borrow()).to_bool_9() }) as i32) == (false as i32)) }));
        assert!(({ (((*x.borrow()) as i32) == (true as i32)) }));
        assert!(
            ({
                ((({
                    (*obj.borrow()).compare_exchange_strong_25(
                        x.as_pointer(),
                        true,
                        Some(memory_order_seq_cst_4.with(|rc| *rc.borrow())),
                    )
                }) as i32)
                    == (false as i32))
            })
        );
        assert!(({ ((({ (*obj.borrow()).to_bool_9() }) as i32) == (false as i32)) }));
        assert!(({ (((*x.borrow()) as i32) == (false as i32)) }));
        (*x.borrow_mut()) = true;
        ({ (*obj.borrow()).store_13(true, None) });
        assert!(
            ({
                ((({
                    let ___m1: std_memory_order = memory_order_seq_cst_4.with(|rc| *rc.borrow());
                    let ___m2: std_memory_order = memory_order_seq_cst_4.with(|rc| *rc.borrow());
                    (*obj.borrow()).compare_exchange_strong_23(x.as_pointer(), false, ___m1, ___m2)
                }) as i32)
                    == (true as i32))
            })
        );
        assert!(({ ((({ (*obj.borrow()).to_bool_9() }) as i32) == (false as i32)) }));
        assert!(({ (((*x.borrow()) as i32) == (true as i32)) }));
        assert!(
            ({
                ((({
                    (*obj.borrow_mut()) = false;
                    (*obj.borrow()).clone()
                }) as i32)
                    == (false as i32))
            })
        );
        assert!(({ ((({ (*obj.borrow()).to_bool_9() }) as i32) == (false as i32)) }));
        assert!(
            ({
                ((({
                    (*obj.borrow_mut()) = true;
                    (*obj.borrow()).clone()
                }) as i32)
                    == (true as i32))
            })
        );
        assert!(({ ((({ (*obj.borrow()).to_bool_9() }) as i32) == (true as i32)) }));
    }
    {
        let storage: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([1_i8])));
        let zero: Ptr<std_atomic_bool_> = Ptr::alloc(<std_atomic_bool_>::default());
        assert!(({ ((({ (*zero.upgrade().deref()).to_bool_9() }) as i32) == (false as i32)) }));
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
