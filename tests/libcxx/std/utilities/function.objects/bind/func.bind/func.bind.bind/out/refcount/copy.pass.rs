extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn _pow_0(mut a: f32, mut b: f32) -> f32 {
    return ({ pow_1(a, b) });
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
    let fnc: Value<std_function_float__float__float__> =
        Rc::new(RefCell::new(std_function_float__float__float__::new_1({
            FnPtr::<fn(f32, f32) -> f32>::new(_pow_0)
        })));
    let task: Value<std__Bind_std_function_float__float__float____float__float__> =
        Rc::new(RefCell::new(
            ({
                let ___args_1: Value<f32> = Rc::new(RefCell::new(2_f32));
                let ___args_2: Value<f32> = Rc::new(RefCell::new(4_f32));
                bind_2(
                    fnc.as_pointer(),
                    ___args_1.as_pointer(),
                    ___args_2.as_pointer(),
                )
            }),
        ));
    let task2: Value<std__Bind_std_function_float__float__float____float__float__> =
        Rc::new(RefCell::new((*task.borrow()).clone()));
    assert!(({ (({ (*task.borrow_mut())() }) == 16_f32) }));
    assert!(({ (({ (*task2.borrow_mut())() }) == 16_f32) }));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
