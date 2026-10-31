extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn assert_contiguous_0(c: Ptr<Vec<f64>>) {
    let mut i: usize = 0_usize;
    'loop_: while ({ i } < { (*c.upgrade().deref()).len() }) {
        assert!(
            ({ ((unimplemented!().offset((i) as isize)).read()) } == {
                ((unimplemented!().offset((i) as isize)).read())
            })
        );
        i.prefix_inc();
    }
}
pub fn assert_contiguous_1(c: Ptr<Vec<f64>>) {
    let mut i: usize = 0_usize;
    'loop_: while ({ i } < { (*c.upgrade().deref()).len() }) {
        assert!(
            ({ ((unimplemented!().offset((i) as isize)).read()) } == {
                ((unimplemented!().offset((i) as isize)).read())
            })
        );
        i.prefix_inc();
    }
}
pub fn assert_contiguous_2(c: Ptr<Vec<f64>>) {
    let mut i: usize = 0_usize;
    'loop_: while ({ i } < { (*c.upgrade().deref()).len() }) {
        assert!(
            ({ ((unimplemented!().offset((i) as isize)).read()) } == {
                ((unimplemented!().offset((i) as isize)).read())
            })
        );
        i.prefix_inc();
    }
}
pub fn assert_contiguous_3(c: Ptr<Vec<f64>>) {
    let mut i: usize = 0_usize;
    'loop_: while ({ i } < { (*c.upgrade().deref()).len() }) {
        assert!(
            ({ ((unimplemented!().offset((i) as isize)).read()) } == {
                ((unimplemented!().offset((i) as isize)).read())
            })
        );
        i.prefix_inc();
    }
}
pub fn assert_contiguous_4(c: Ptr<Vec<i8>>) {
    let mut i: usize = 0_usize;
    'loop_: while ({ i } < { (*c.upgrade().deref()).len() }) {
        assert!(
            ({ (((unimplemented!().offset((i) as isize)).read()) as i32) } == {
                (((unimplemented!().offset((i) as isize)).read()) as i32)
            })
        );
        i.prefix_inc();
    }
}
pub fn assert_contiguous_5(c: Ptr<Vec<i8>>) {
    let mut i: usize = 0_usize;
    'loop_: while ({ i } < { (*c.upgrade().deref()).len() }) {
        assert!(
            ({ (((unimplemented!().offset((i) as isize)).read()) as i32) } == {
                (((unimplemented!().offset((i) as isize)).read()) as i32)
            })
        );
        i.prefix_inc();
    }
}
pub fn assert_contiguous_6(c: Ptr<Vec<i8>>) {
    let mut i: usize = 0_usize;
    'loop_: while ({ i } < { (*c.upgrade().deref()).len() }) {
        assert!(
            ({ (((unimplemented!().offset((i) as isize)).read()) as i32) } == {
                (((unimplemented!().offset((i) as isize)).read()) as i32)
            })
        );
        i.prefix_inc();
    }
}
pub fn assert_contiguous_7(c: Ptr<Vec<i8>>) {
    let mut i: usize = 0_usize;
    'loop_: while ({ i } < { (*c.upgrade().deref()).len() }) {
        assert!(
            ({ (((unimplemented!().offset((i) as isize)).read()) as i32) } == {
                (((unimplemented!().offset((i) as isize)).read()) as i32)
            })
        );
        i.prefix_inc();
    }
}
pub fn tests_8() -> bool {
    ({
        let _c: Value<Vec<f64>> = Rc::new(RefCell::new(
            std::array::from_fn::<_, 0, _>(|_| 0_f64).to_vec(),
        ));
        assert_contiguous_0(_c.as_pointer())
    });
    ({
        let _c: Value<Vec<f64>> = Rc::new(RefCell::new(
            std::array::from_fn::<_, 1, _>(|_| 0_f64).to_vec(),
        ));
        assert_contiguous_1(_c.as_pointer())
    });
    ({
        let _c: Value<Vec<f64>> = Rc::new(RefCell::new(
            std::array::from_fn::<_, 2, _>(|_| 0_f64).to_vec(),
        ));
        assert_contiguous_2(_c.as_pointer())
    });
    ({
        let _c: Value<Vec<f64>> = Rc::new(RefCell::new(
            std::array::from_fn::<_, 3, _>(|_| 0_f64).to_vec(),
        ));
        assert_contiguous_3(_c.as_pointer())
    });
    ({
        let _c: Value<Vec<i8>> = Rc::new(RefCell::new(
            std::array::from_fn::<_, 0, _>(|_| 0_i8).to_vec(),
        ));
        assert_contiguous_4(_c.as_pointer())
    });
    ({
        let _c: Value<Vec<i8>> = Rc::new(RefCell::new(
            std::array::from_fn::<_, 1, _>(|_| 0_i8).to_vec(),
        ));
        assert_contiguous_5(_c.as_pointer())
    });
    ({
        let _c: Value<Vec<i8>> = Rc::new(RefCell::new(
            std::array::from_fn::<_, 2, _>(|_| 0_i8).to_vec(),
        ));
        assert_contiguous_6(_c.as_pointer())
    });
    ({
        let _c: Value<Vec<i8>> = Rc::new(RefCell::new(
            std::array::from_fn::<_, 3, _>(|_| 0_i8).to_vec(),
        ));
        assert_contiguous_7(_c.as_pointer())
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
    ({ tests_8() });
    const _: () = assert!(true, "tests()");;
    return 0;
}
pub fn __cpp2rust_init_globals() {}
