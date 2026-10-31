extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_node_container_0(conv: FnPtr<fn(i32) -> i32>) {
    let conv: Value<FnPtr<fn(i32) -> i32>> = Rc::new(RefCell::new(conv));
    let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut i: i32 = 0;
    'loop_: while (i != 10) {
        unimplemented!();
        i.prefix_inc();
    }
    {
        let mut i: i32 = 0;
        'loop_: while (i != 10) {
            let invoke_count: Value<i32> = Rc::new(RefCell::new(i));
            unimplemented!();
            assert!(((*invoke_count.borrow()) == 10));
            i.prefix_inc();
        }
    }
    {
        let mut i: i32 = 0;
        'loop_: while (i != 10) {
            let invoke_count: Value<i32> = Rc::new(RefCell::new(0));
            unimplemented!();
            assert!(((*invoke_count.borrow()) == (10 - i)));
            i.prefix_inc();
        }
    }
    {
        {
            let invoke_count: Value<i32> = Rc::new(RefCell::new(0));
            unimplemented!();
            assert!(((*invoke_count.borrow()) == 0));
        }
        {
            let invoke_count: Value<i32> = Rc::new(RefCell::new(0));
            unimplemented!();
            assert!(((*invoke_count.borrow()) == 0));
        }
    }
    {
        let invoke_count: Value<i32> = Rc::new(RefCell::new(0));
        unimplemented!();
        assert!(((*invoke_count.borrow()) == 1));
    }
}
pub fn test_node_container_1(conv: FnPtr<fn(i32) -> i32>) {
    let conv: Value<FnPtr<fn(i32) -> i32>> = Rc::new(RefCell::new(conv));
    let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut i: i32 = 0;
    'loop_: while (i != 10) {
        unimplemented!();
        i.prefix_inc();
    }
    {
        let mut i: i32 = 0;
        'loop_: while (i != 10) {
            let invoke_count: Value<i32> = Rc::new(RefCell::new(i));
            unimplemented!();
            assert!(((*invoke_count.borrow()) == 10));
            i.prefix_inc();
        }
    }
    {
        let mut i: i32 = 0;
        'loop_: while (i != 10) {
            let invoke_count: Value<i32> = Rc::new(RefCell::new(0));
            unimplemented!();
            assert!(((*invoke_count.borrow()) == (10 - i)));
            i.prefix_inc();
        }
    }
    {
        {
            let invoke_count: Value<i32> = Rc::new(RefCell::new(0));
            unimplemented!();
            assert!(((*invoke_count.borrow()) == 0));
        }
        {
            let invoke_count: Value<i32> = Rc::new(RefCell::new(0));
            unimplemented!();
            assert!(((*invoke_count.borrow()) == 0));
        }
    }
    {
        let invoke_count: Value<i32> = Rc::new(RefCell::new(0));
        unimplemented!();
        assert!(((*invoke_count.borrow()) == 1));
    }
}
pub fn test_node_container_2(conv: FnPtr<fn(i32) -> (Value<i32>, Value<i32>)>) {
    let conv: Value<FnPtr<fn(i32) -> (Value<i32>, Value<i32>)>> = Rc::new(RefCell::new(conv));
    let c: Value<Map<i32, (Value<i32>, Value<i32>)>> = Rc::new(RefCell::new(Map::new()));
    let mut i: i32 = 0;
    'loop_: while (i != 10) {
        unimplemented!();
        i.prefix_inc();
    }
    {
        let mut i: i32 = 0;
        'loop_: while (i != 10) {
            let invoke_count: Value<i32> = Rc::new(RefCell::new(i));
            unimplemented!();
            assert!(((*invoke_count.borrow()) == 10));
            i.prefix_inc();
        }
    }
    {
        let mut i: i32 = 0;
        'loop_: while (i != 10) {
            let invoke_count: Value<i32> = Rc::new(RefCell::new(0));
            unimplemented!();
            assert!(((*invoke_count.borrow()) == (10 - i)));
            i.prefix_inc();
        }
    }
    {
        {
            let invoke_count: Value<i32> = Rc::new(RefCell::new(0));
            unimplemented!();
            assert!(((*invoke_count.borrow()) == 0));
        }
        {
            let invoke_count: Value<i32> = Rc::new(RefCell::new(0));
            unimplemented!();
            assert!(((*invoke_count.borrow()) == 0));
        }
    }
    {
        let invoke_count: Value<i32> = Rc::new(RefCell::new(0));
        unimplemented!();
        assert!(((*invoke_count.borrow()) == 1));
    }
}
pub fn test_node_container_3(conv: FnPtr<fn(i32) -> (Value<i32>, Value<i32>)>) {
    let conv: Value<FnPtr<fn(i32) -> (Value<i32>, Value<i32>)>> = Rc::new(RefCell::new(conv));
    let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut i: i32 = 0;
    'loop_: while (i != 10) {
        unimplemented!();
        i.prefix_inc();
    }
    {
        let mut i: i32 = 0;
        'loop_: while (i != 10) {
            let invoke_count: Value<i32> = Rc::new(RefCell::new(i));
            unimplemented!();
            assert!(((*invoke_count.borrow()) == 10));
            i.prefix_inc();
        }
    }
    {
        let mut i: i32 = 0;
        'loop_: while (i != 10) {
            let invoke_count: Value<i32> = Rc::new(RefCell::new(0));
            unimplemented!();
            assert!(((*invoke_count.borrow()) == (10 - i)));
            i.prefix_inc();
        }
    }
    {
        {
            let invoke_count: Value<i32> = Rc::new(RefCell::new(0));
            unimplemented!();
            assert!(((*invoke_count.borrow()) == 0));
        }
        {
            let invoke_count: Value<i32> = Rc::new(RefCell::new(0));
            unimplemented!();
            assert!(((*invoke_count.borrow()) == 0));
        }
    }
    {
        let invoke_count: Value<i32> = Rc::new(RefCell::new(0));
        unimplemented!();
        assert!(((*invoke_count.borrow()) == 1));
    }
}
pub fn test_4() -> bool {
    ({
        test_node_container_0(FnPtr::<fn(i32) -> i32>::new(|i: i32| -> i32 {
            {
                let i: Value<i32> = Rc::new(RefCell::new(i));
                return (*i.borrow());
            }
        }))
    });
    ({
        test_node_container_1(FnPtr::<fn(i32) -> i32>::new(|i: i32| -> i32 {
            {
                let i: Value<i32> = Rc::new(RefCell::new(i));
                return (*i.borrow());
            }
        }))
    });
    ({
        test_node_container_2(FnPtr::<fn(i32) -> (Value<i32>, Value<i32>)>::new(
            |i: i32| -> (Value<i32>, Value<i32>) {
                {
                    let i: Value<i32> = Rc::new(RefCell::new(i));
                    return (
                        Rc::new(RefCell::new((*i.borrow()))),
                        Rc::new(RefCell::new((*i.borrow()))),
                    );
                }
            },
        ))
    });
    ({
        test_node_container_3(FnPtr::<fn(i32) -> (Value<i32>, Value<i32>)>::new(
            |i: i32| -> (Value<i32>, Value<i32>) {
                {
                    let i: Value<i32> = Rc::new(RefCell::new(i));
                    return (
                        Rc::new(RefCell::new((*i.borrow()))),
                        Rc::new(RefCell::new((*i.borrow()))),
                    );
                }
            },
        ))
    });
    return true;
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
    ({ test_4() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
