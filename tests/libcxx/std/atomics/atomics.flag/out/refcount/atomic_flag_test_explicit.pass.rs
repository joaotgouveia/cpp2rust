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
        let f: Value<std_atomic_flag> = Rc::new(RefCell::new(<std_atomic_flag>::default()));
        ({ (*f.borrow()).clear_14(None) });
        assert!(
            ((({
                atomic_flag_test_explicit_0(
                    (f.as_pointer()),
                    memory_order_relaxed_1.with(|rc| *rc.borrow()),
                )
            }) as i32)
                == 0)
        );
        assert!(({ ((({ (*f.borrow()).test_and_set_7(None,) }) as i32) == 0) }));
        assert!(
            ((({
                atomic_flag_test_explicit_0(
                    (f.as_pointer()),
                    memory_order_relaxed_1.with(|rc| *rc.borrow()),
                )
            }) as i32)
                == 1)
        );
    }
    {
        let f: Value<std_atomic_flag> = Rc::new(RefCell::new(<std_atomic_flag>::default()));
        ({ (*f.borrow()).clear_14(None) });
        assert!(
            ((({
                atomic_flag_test_explicit_0(
                    (f.as_pointer()),
                    memory_order_consume_2.with(|rc| *rc.borrow()),
                )
            }) as i32)
                == 0)
        );
        assert!(({ ((({ (*f.borrow()).test_and_set_7(None,) }) as i32) == 0) }));
        assert!(
            ((({
                atomic_flag_test_explicit_0(
                    (f.as_pointer()),
                    memory_order_consume_2.with(|rc| *rc.borrow()),
                )
            }) as i32)
                == 1)
        );
    }
    {
        let f: Value<std_atomic_flag> = Rc::new(RefCell::new(<std_atomic_flag>::default()));
        ({ (*f.borrow()).clear_14(None) });
        assert!(
            ((({
                atomic_flag_test_explicit_0(
                    (f.as_pointer()),
                    memory_order_acquire_3.with(|rc| *rc.borrow()),
                )
            }) as i32)
                == 0)
        );
        assert!(({ ((({ (*f.borrow()).test_and_set_7(None,) }) as i32) == 0) }));
        assert!(
            ((({
                atomic_flag_test_explicit_0(
                    (f.as_pointer()),
                    memory_order_acquire_3.with(|rc| *rc.borrow()),
                )
            }) as i32)
                == 1)
        );
    }
    {
        let f: Value<std_atomic_flag> = Rc::new(RefCell::new(<std_atomic_flag>::default()));
        ({ (*f.borrow()).clear_14(None) });
        assert!(
            ((({
                atomic_flag_test_explicit_0(
                    (f.as_pointer()),
                    memory_order_seq_cst_4.with(|rc| *rc.borrow()),
                )
            }) as i32)
                == 0)
        );
        assert!(({ ((({ (*f.borrow()).test_and_set_7(None,) }) as i32) == 0) }));
        assert!(
            ((({
                atomic_flag_test_explicit_0(
                    (f.as_pointer()),
                    memory_order_seq_cst_4.with(|rc| *rc.borrow()),
                )
            }) as i32)
                == 1)
        );
    }
    {
        let f: Value<std_atomic_flag> = Rc::new(RefCell::new(<std_atomic_flag>::default()));
        ({ (*f.borrow()).clear_15(None) });
        assert!(
            ((({
                atomic_flag_test_explicit_5(
                    (f.as_pointer()),
                    memory_order_relaxed_1.with(|rc| *rc.borrow()),
                )
            }) as i32)
                == 0)
        );
        assert!(({ ((({ (*f.borrow()).test_and_set_8(None,) }) as i32) == 0) }));
        assert!(
            ((({
                atomic_flag_test_explicit_5(
                    (f.as_pointer()),
                    memory_order_relaxed_1.with(|rc| *rc.borrow()),
                )
            }) as i32)
                == 1)
        );
    }
    {
        let f: Value<std_atomic_flag> = Rc::new(RefCell::new(<std_atomic_flag>::default()));
        ({ (*f.borrow()).clear_15(None) });
        assert!(
            ((({
                atomic_flag_test_explicit_5(
                    (f.as_pointer()),
                    memory_order_consume_2.with(|rc| *rc.borrow()),
                )
            }) as i32)
                == 0)
        );
        assert!(({ ((({ (*f.borrow()).test_and_set_8(None,) }) as i32) == 0) }));
        assert!(
            ((({
                atomic_flag_test_explicit_5(
                    (f.as_pointer()),
                    memory_order_consume_2.with(|rc| *rc.borrow()),
                )
            }) as i32)
                == 1)
        );
    }
    {
        let f: Value<std_atomic_flag> = Rc::new(RefCell::new(<std_atomic_flag>::default()));
        ({ (*f.borrow()).clear_15(None) });
        assert!(
            ((({
                atomic_flag_test_explicit_5(
                    (f.as_pointer()),
                    memory_order_acquire_3.with(|rc| *rc.borrow()),
                )
            }) as i32)
                == 0)
        );
        assert!(({ ((({ (*f.borrow()).test_and_set_8(None,) }) as i32) == 0) }));
        assert!(
            ((({
                atomic_flag_test_explicit_5(
                    (f.as_pointer()),
                    memory_order_acquire_3.with(|rc| *rc.borrow()),
                )
            }) as i32)
                == 1)
        );
    }
    {
        let f: Value<std_atomic_flag> = Rc::new(RefCell::new(<std_atomic_flag>::default()));
        ({ (*f.borrow()).clear_15(None) });
        assert!(
            ((({
                atomic_flag_test_explicit_5(
                    (f.as_pointer()),
                    memory_order_seq_cst_4.with(|rc| *rc.borrow()),
                )
            }) as i32)
                == 0)
        );
        assert!(({ ((({ (*f.borrow()).test_and_set_8(None,) }) as i32) == 0) }));
        assert!(
            ((({
                atomic_flag_test_explicit_5(
                    (f.as_pointer()),
                    memory_order_seq_cst_4.with(|rc| *rc.borrow()),
                )
            }) as i32)
                == 1)
        );
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
