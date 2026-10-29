extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn tests_0() -> bool {
    let a: Value<Box<[bool]>> = Rc::new(RefCell::new(Box::new([false, true, false, true])));
    let mut an: Ptr<bool> = (a.as_pointer() as Ptr<bool>).offset(
        ((::std::mem::size_of::<[bool; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<bool>() as usize))) as isize,
    );
    let v: Value<Vec<bool>> = Rc::new(RefCell::new({
        let mut __v = Vec::new();
        let mut __it = (a.as_pointer() as Ptr<bool>);
        while __it != an {
            __v.push(<bool>::try_from(__it.iter_deref().read()).ok().unwrap());
            __it.iter_inc();
        }
        __v
    }));
    let r1: Value<bool> = Rc::new(RefCell::new({ (*v.borrow())[0_usize] }));
    let r2: Value<bool> = Rc::new(RefCell::new({ (*v.borrow())[3_usize] }));
    const _: () = assert!(true, "(noexcept(v.swap(r1, r2)))");;
    assert!(!((*r1.borrow()) as bool));
    assert!(((*r2.borrow()) as bool));
    ({ std_vector_bool__std_allocator_bool__::swap_51((*r1.borrow()), (*r2.borrow())) });
    assert!(((*r1.borrow()) as bool));
    assert!(!((*r2.borrow()) as bool));
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
    ({ tests_0() });
    const _: () = assert!(true, "tests()");;
    return 0;
}
pub fn __cpp2rust_init_globals() {}
