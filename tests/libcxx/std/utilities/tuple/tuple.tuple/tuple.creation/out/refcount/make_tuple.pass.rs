extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_0() -> bool {
    let i: Value<i32> = Rc::new(RefCell::new(0));
    let j: Value<f32> = Rc::new(RefCell::new(0_f32));
    let t: Value<std_tuple_int__int_ref__float_ref_> = Rc::new(RefCell::new(
        ({
            let ___args_0: Value<i32> = Rc::new(RefCell::new(1));
            let ___args_1: Value<Ptr<i32>> = Rc::new(RefCell::new(({ ref_1(i.as_pointer()) })));
            let ___args_2: Value<Ptr<f32>> = Rc::new(RefCell::new(({ ref_2(j.as_pointer()) })));
            make_tuple_3(
                ___args_0.as_pointer(),
                ___args_1.as_pointer(),
                ___args_2.as_pointer(),
            )
        }),
    ));
    assert!(((({ get_4(t.as_pointer(),) }).read()) == 1));
    assert!(((({ get_5(t.as_pointer(),) }).read()) == 0));
    assert!(((({ get_6(t.as_pointer(),) }).read()) == 0_f32));
    (*i.borrow_mut()) = 2;
    (*j.borrow_mut()) = (3.5_f64 as f32);
    assert!(((({ get_4(t.as_pointer(),) }).read()) == 1));
    assert!(((({ get_5(t.as_pointer(),) }).read()) == 2));
    assert!((((({ get_6(t.as_pointer(),) }).read()) as f64) == 3.5_f64));
    ({ get_5(t.as_pointer()) }).write(0);
    ({ get_6(t.as_pointer()) }).write(0_f32);
    assert!(({ ((*i.borrow()) == 0) }));
    assert!(({ ((*j.borrow()) == 0_f32) }));
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
    ({ test_0() });
    const _: () = assert!(true, "test()");;
    {
        let t1: Value<std_tuple_int__int__double_> = Rc::new(RefCell::new(
            ({
                let ___args_0: Value<i32> = Rc::new(RefCell::new(0));
                let ___args_1: Value<i32> = Rc::new(RefCell::new(1));
                let ___args_2: Value<f64> = Rc::new(RefCell::new(3.14_f64));
                make_tuple_7(
                    ___args_0.as_pointer(),
                    ___args_1.as_pointer(),
                    ___args_2.as_pointer(),
                )
            }),
        ));
        let mut i1: i32 = 1;
        let mut d1: f64 = 3.14_f64;
        const _: () = assert!(true, "i1 == 1");;
        const _: () = assert!(true, "d1 == 3.14");;
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
