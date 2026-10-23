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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    {
        let t2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let t1 : Value<() > = Rc::new(RefCell::new(std_chrono_time_point_std_chrono_system_clock__std_chrono_duration_long_long__std_ratio_1__1000000___ :: new_1 ( { t2 .as_pointer()  } , )  )) ;
        unimplemented!();
    }
    {
        let t2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let t1 : Value<() > = Rc::new(RefCell::new(std_chrono_time_point_std_chrono_system_clock__std_chrono_duration_long_long__std_ratio_1__1000000___ :: new_1 ( { t2 .as_pointer()  } , )  )) ;
        const _: () = assert!(true, "t1.time_since_epoch() == Duration1(3000)");;
    }
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
