extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_node_container_0(conv: FnPtr<fn(()) -> ()>) {
    let conv: Value<FnPtr<fn(()) -> ()>> = Rc::new(RefCell::new(conv));
    let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut i: () = 0_();
    'loop_: while (i != 10_()) {
        unimplemented!();
        i.prefix_inc();
    }
    {
        let mut i: () = 0_();
        'loop_: while (i != 10_()) {
            let invoke_count: Value<()> = Rc::new(RefCell::new(i));
            unimplemented!();
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let mut i: () = 0_();
        'loop_: while (i != 10_()) {
            let invoke_count: Value<()> = Rc::new(RefCell::new(0_()));
            unimplemented!();
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        {
            let invoke_count: Value<()> = Rc::new(RefCell::new(0_()));
            unimplemented!();
            unimplemented!();
        }
        {
            let invoke_count: Value<()> = Rc::new(RefCell::new(0_()));
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let invoke_count: Value<()> = Rc::new(RefCell::new(0_()));
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_node_container_1(conv: FnPtr<fn(()) -> ()>) {
    let conv: Value<FnPtr<fn(()) -> ()>> = Rc::new(RefCell::new(conv));
    let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut i: () = 0_();
    'loop_: while (i != 10_()) {
        unimplemented!();
        i.prefix_inc();
    }
    {
        let mut i: () = 0_();
        'loop_: while (i != 10_()) {
            let invoke_count: Value<()> = Rc::new(RefCell::new(i));
            unimplemented!();
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let mut i: () = 0_();
        'loop_: while (i != 10_()) {
            let invoke_count: Value<()> = Rc::new(RefCell::new(0_()));
            unimplemented!();
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        {
            let invoke_count: Value<()> = Rc::new(RefCell::new(0_()));
            unimplemented!();
            unimplemented!();
        }
        {
            let invoke_count: Value<()> = Rc::new(RefCell::new(0_()));
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let invoke_count: Value<()> = Rc::new(RefCell::new(0_()));
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_node_container_2(conv: FnPtr<fn(()) -> ()>) {
    let conv: Value<FnPtr<fn(()) -> ()>> = Rc::new(RefCell::new(conv));
    let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut i: () = 0_();
    'loop_: while (i != 10_()) {
        unimplemented!();
        i.prefix_inc();
    }
    {
        let mut i: () = 0_();
        'loop_: while (i != 10_()) {
            let invoke_count: Value<()> = Rc::new(RefCell::new(i));
            unimplemented!();
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let mut i: () = 0_();
        'loop_: while (i != 10_()) {
            let invoke_count: Value<()> = Rc::new(RefCell::new(0_()));
            unimplemented!();
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        {
            let invoke_count: Value<()> = Rc::new(RefCell::new(0_()));
            unimplemented!();
            unimplemented!();
        }
        {
            let invoke_count: Value<()> = Rc::new(RefCell::new(0_()));
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let invoke_count: Value<()> = Rc::new(RefCell::new(0_()));
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_node_container_3(conv: FnPtr<fn(()) -> ()>) {
    let conv: Value<FnPtr<fn(()) -> ()>> = Rc::new(RefCell::new(conv));
    let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut i: () = 0_();
    'loop_: while (i != 10_()) {
        unimplemented!();
        i.prefix_inc();
    }
    {
        let mut i: () = 0_();
        'loop_: while (i != 10_()) {
            let invoke_count: Value<()> = Rc::new(RefCell::new(i));
            unimplemented!();
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let mut i: () = 0_();
        'loop_: while (i != 10_()) {
            let invoke_count: Value<()> = Rc::new(RefCell::new(0_()));
            unimplemented!();
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        {
            let invoke_count: Value<()> = Rc::new(RefCell::new(0_()));
            unimplemented!();
            unimplemented!();
        }
        {
            let invoke_count: Value<()> = Rc::new(RefCell::new(0_()));
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let invoke_count: Value<()> = Rc::new(RefCell::new(0_()));
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_4() -> () {
    ({
        test_node_container_0(FnPtr::<fn(()) -> ()>::new(|i: ()| -> () {
            {
                let i: Value<()> = Rc::new(RefCell::new(i));
                return (*i.borrow());
            }
        }))
    });
    ({
        test_node_container_1(FnPtr::<fn(()) -> ()>::new(|i: ()| -> () {
            {
                let i: Value<()> = Rc::new(RefCell::new(i));
                return (*i.borrow());
            }
        }))
    });
    ({
        test_node_container_2(FnPtr::<fn(()) -> ()>::new(|mut i: ()| -> () {
            {
                return unimplemented!();
            }
        }))
    });
    ({
        test_node_container_3(FnPtr::<fn(()) -> ()>::new(|mut i: ()| -> () {
            {
                return unimplemented!();
            }
        }))
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    ({ test_4() });
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
