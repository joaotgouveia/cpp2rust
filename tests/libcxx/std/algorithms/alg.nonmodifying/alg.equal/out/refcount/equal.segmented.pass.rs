extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_containers_0() {
    {
        let lhs: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([1, 2, 3, 4])));
        let rhs: Value<Vec<i32>> = Rc::new(RefCell::new(unimplemented!()));
        let mut ret: bool = unimplemented!();
        assert!((ret as bool));
    }
    {
        let lhs: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([1, 2, 3, 4])));
        let rhs: Value<Vec<i32>> = Rc::new(RefCell::new(unimplemented!()));
        let mut ret: bool = unimplemented!();
        assert!(!(ret));
    }
    {
        let lhs: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
        unimplemented!();
        let rhs: Value<Vec<i32>> = Rc::new(RefCell::new(unimplemented!()));
        assert!(unimplemented!());
    }
    {
        let lhs: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
        unimplemented!();
        let rhs: Value<Vec<i32>> = Rc::new(RefCell::new(unimplemented!()));
        rhs.decay().to_last().write(-1_i32);
        assert!(!(unimplemented!()));
    }
    {
        let lhs: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
        unimplemented!();
        let rhs: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        assert!(unimplemented!());
    }
    {
        let lhs: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
        unimplemented!();
        let rhs: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        rhs.decay().to_last().write(-1_i32);
        assert!(!(unimplemented!()));
    }
}
pub fn test_containers_1() {
    {
        let lhs: Value<Vec<i32>> = Rc::new(RefCell::new(unimplemented!()));
        let rhs: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([1, 2, 3, 4])));
        let mut ret: bool = unimplemented!();
        assert!((ret as bool));
    }
    {
        let lhs: Value<Vec<i32>> = Rc::new(RefCell::new(unimplemented!()));
        let rhs: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([2, 2, 3, 4])));
        let mut ret: bool = unimplemented!();
        assert!(!(ret));
    }
    {
        let lhs: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
        unimplemented!();
        let rhs: Value<Vec<i32>> = Rc::new(RefCell::new({
            let mut __v = Vec::new();
            let mut __it = unimplemented!();
            while __it != unimplemented!() {
                __v.push((__it.iter_deref().read()));
                __it.iter_inc();
            }
            __v
        }));
        assert!(unimplemented!());
    }
    {
        let lhs: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
        unimplemented!();
        let rhs: Value<Vec<i32>> = Rc::new(RefCell::new({
            let mut __v = Vec::new();
            let mut __it = unimplemented!();
            while __it != unimplemented!() {
                __v.push((__it.iter_deref().read()));
                __it.iter_inc();
            }
            __v
        }));
        rhs.decay().to_last().write(-1_i32);
        assert!(!(unimplemented!()));
    }
    {
        let lhs: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
        unimplemented!();
        let rhs: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
        {
            let __a0 = 10_usize as usize;
            (*rhs.borrow_mut()).resize_with(__a0, || 0_i32)
        };
        {
            let __pos = <RandomAccessIterator<i32>>::from(rhs.decay().end());
            let __start = __pos.index();
            let mut __tmp: Vec<i32> = Vec::new();
            let mut __it = unimplemented!();
            while __it != unimplemented!() {
                __tmp.push((__it.iter_deref().read()));
                __it.iter_inc();
            }
            rhs.as_pointer().with_mut(|__v: &mut Vec<i32>| {
                __v.splice(__start..__start, __tmp);
            });
            __pos
        };
        unimplemented!();
        assert!(unimplemented!());
    }
    {
        let lhs: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
        unimplemented!();
        let rhs: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
        {
            let __a0 = 10_usize as usize;
            (*rhs.borrow_mut()).resize_with(__a0, || 0_i32)
        };
        {
            let __pos = <RandomAccessIterator<i32>>::from(rhs.decay().end());
            let __start = __pos.index();
            let mut __tmp: Vec<i32> = Vec::new();
            let mut __it = unimplemented!();
            while __it != unimplemented!() {
                __tmp.push((__it.iter_deref().read()));
                __it.iter_inc();
            }
            rhs.as_pointer().with_mut(|__v: &mut Vec<i32>| {
                __v.splice(__start..__start, __tmp);
            });
            __pos
        };
        unimplemented!();
        rhs.decay().to_last().write(-1_i32);
        assert!(!(unimplemented!()));
    }
}
pub fn test_containers_2() {
    {
        let lhs: Value<Vec<i32>> = Rc::new(RefCell::new(unimplemented!()));
        let rhs: Value<Vec<i32>> = Rc::new(RefCell::new(unimplemented!()));
        let mut ret: bool = unimplemented!();
        assert!((ret as bool));
    }
    {
        let lhs: Value<Vec<i32>> = Rc::new(RefCell::new(unimplemented!()));
        let rhs: Value<Vec<i32>> = Rc::new(RefCell::new(unimplemented!()));
        let mut ret: bool = unimplemented!();
        assert!(!(ret));
    }
    {
        let lhs: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
        unimplemented!();
        let rhs: Value<Vec<i32>> = Rc::new(RefCell::new(unimplemented!()));
        assert!(unimplemented!());
    }
    {
        let lhs: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
        unimplemented!();
        let rhs: Value<Vec<i32>> = Rc::new(RefCell::new(unimplemented!()));
        rhs.decay().to_last().write(-1_i32);
        assert!(!(unimplemented!()));
    }
    {
        let lhs: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
        unimplemented!();
        let rhs: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        assert!(unimplemented!());
    }
    {
        let lhs: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
        unimplemented!();
        let rhs: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        rhs.decay().to_last().write(-1_i32);
        assert!(!(unimplemented!()));
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> i32 {
    ({ test_containers_0() });
    ({ test_containers_1() });
    ({ test_containers_2() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
