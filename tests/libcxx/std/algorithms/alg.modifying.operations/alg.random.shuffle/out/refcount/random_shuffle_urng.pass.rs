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
    let ia: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5, 6, 7, 8, 9, 10])));
    let ia1: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([2, 7, 1, 4, 3, 6, 5, 10, 9, 8])));
    let ia2: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 8, 3, 4, 6, 9, 5, 7, 2, 10])));
    let mut sa: u32 = (((::std::mem::size_of::<[i32; 10]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let g: Value<std_linear_congruential_engine_unsigned_long__48271__0__2147483647_> =
        Rc::new(RefCell::new(
            <std_linear_congruential_engine_unsigned_long__48271__0__2147483647_>::default(),
        ));
    ({
        let ___first: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
        let ___last: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((sa) as isize);
        shuffle_0(___first, ___last, g.as_pointer())
    });
    const _: () = assert!(true, "");;
    assert!(
        ({
            let ___first1: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___last1: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((sa) as isize);
            is_permutation_1(___first1, ___last1, (ia1.as_pointer() as Ptr<i32>))
        })
    );
    ({
        let ___first: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
        let ___last: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((sa) as isize);
        shuffle_2(___first, ___last, g.as_pointer())
    });
    const _: () = assert!(true, "");;
    assert!(
        ({
            let ___first1: Ptr<i32> = (ia.as_pointer() as Ptr<i32>);
            let ___last1: Ptr<i32> = (ia.as_pointer() as Ptr<i32>).offset((sa) as isize);
            is_permutation_1(___first1, ___last1, (ia2.as_pointer() as Ptr<i32>))
        })
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
