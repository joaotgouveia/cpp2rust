extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_0() {
    {
        let ii1: Value<RandomAccessIterator<i8>> =
            Rc::new(RefCell::new(RandomAccessIterator::default()));
        let ii2: Value<RandomAccessIterator<i8>> =
            Rc::new(RefCell::new(RandomAccessIterator::default()));
        let ii4: Value<RandomAccessIterator<i8>> = Rc::new(RefCell::new((*ii1.borrow()).clone()));
        let cii: Value<RandomAccessIterator<i8>> =
            Rc::new(RefCell::new(RandomAccessIterator::default()));
        assert!(({ (*ii1.borrow()) == (*ii2.borrow()) }));
        assert!(({ (*ii1.borrow()) == (*ii4.borrow()) }));
        assert!(({ !((*ii1.borrow()) != (*ii2.borrow())) }));
        assert!(({ ((*ii1.borrow()) == (*cii.borrow())) }));
        assert!(({ ((*cii.borrow()) == (*ii1.borrow())) }));
        assert!(({ !((*ii1.borrow()) != (*cii.borrow())) }));
        assert!(({ !((*cii.borrow()) != (*ii1.borrow())) }));
        assert!(({ !((*ii1.borrow()) < ({ (*cii.borrow()).clone() }).into()) }));
        assert!(({ !((*cii.borrow()) < ({ (*ii1.borrow()).clone() }).into()) }));
        assert!(({ ((*ii1.borrow()) <= ({ (*cii.borrow()).clone() }).into()) }));
        assert!(({ ((*cii.borrow()) <= ({ (*ii1.borrow()).clone() }).into()) }));
        assert!(({ !((*ii1.borrow()) > ({ (*cii.borrow()).clone() }).into()) }));
        assert!(({ !((*cii.borrow()) > ({ (*ii1.borrow()).clone() }).into()) }));
        assert!(({ ((*ii1.borrow()) >= ({ (*cii.borrow()).clone() }).into()) }));
        assert!(({ ((*cii.borrow()) >= ({ (*ii1.borrow()).clone() }).into()) }));
        assert!(
            ({
                ((*cii.borrow()).iter_distance(&({ (*ii1.borrow()).clone() }).into()) as i64
                    == 0_i64)
            })
        );
        assert!(
            ({
                ((*ii1.borrow()).iter_distance(&({ (*cii.borrow()).clone() }).into()) as i64
                    == 0_i64)
            })
        );
    }
    {
        let a: Value<Vec<i8>> = Rc::new(RefCell::new(vec![i8::default()]));
        let i1: Value<RandomAccessIterator<i8>> =
            Rc::new(RefCell::new(RandomAccessIterator::Vec(a.decay())));
        let i2: Value<RandomAccessIterator<i8>> =
            Rc::new(RefCell::new(RandomAccessIterator::default()));
        (*i2.borrow_mut()) = (*i1.borrow()).clone();
        assert!(({ (*i1.borrow()) == (*i2.borrow()) }));
    }
}
pub fn test_1() {
    {
        let ii1: Value<RandomAccessIterator<i32>> =
            Rc::new(RefCell::new(RandomAccessIterator::default()));
        let ii2: Value<RandomAccessIterator<i32>> =
            Rc::new(RefCell::new(RandomAccessIterator::default()));
        let ii4: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new((*ii1.borrow()).clone()));
        let cii: Value<RandomAccessIterator<i32>> =
            Rc::new(RefCell::new(RandomAccessIterator::default()));
        assert!(({ (*ii1.borrow()) == (*ii2.borrow()) }));
        assert!(({ (*ii1.borrow()) == (*ii4.borrow()) }));
        assert!(({ !((*ii1.borrow()) != (*ii2.borrow())) }));
        assert!(({ ((*ii1.borrow()) == (*cii.borrow())) }));
        assert!(({ ((*cii.borrow()) == (*ii1.borrow())) }));
        assert!(({ !((*ii1.borrow()) != (*cii.borrow())) }));
        assert!(({ !((*cii.borrow()) != (*ii1.borrow())) }));
        assert!(({ !((*ii1.borrow()) < ({ (*cii.borrow()).clone() }).into()) }));
        assert!(({ !((*cii.borrow()) < ({ (*ii1.borrow()).clone() }).into()) }));
        assert!(({ ((*ii1.borrow()) <= ({ (*cii.borrow()).clone() }).into()) }));
        assert!(({ ((*cii.borrow()) <= ({ (*ii1.borrow()).clone() }).into()) }));
        assert!(({ !((*ii1.borrow()) > ({ (*cii.borrow()).clone() }).into()) }));
        assert!(({ !((*cii.borrow()) > ({ (*ii1.borrow()).clone() }).into()) }));
        assert!(({ ((*ii1.borrow()) >= ({ (*cii.borrow()).clone() }).into()) }));
        assert!(({ ((*cii.borrow()) >= ({ (*ii1.borrow()).clone() }).into()) }));
        assert!(
            ({
                ((*cii.borrow()).iter_distance(&({ (*ii1.borrow()).clone() }).into()) as i64
                    == 0_i64)
            })
        );
        assert!(
            ({
                ((*ii1.borrow()).iter_distance(&({ (*cii.borrow()).clone() }).into()) as i64
                    == 0_i64)
            })
        );
    }
    {
        let a: Value<Vec<i32>> = Rc::new(RefCell::new(vec![i32::default()]));
        let i1: Value<RandomAccessIterator<i32>> =
            Rc::new(RefCell::new(RandomAccessIterator::Vec(a.decay())));
        let i2: Value<RandomAccessIterator<i32>> =
            Rc::new(RefCell::new(RandomAccessIterator::default()));
        (*i2.borrow_mut()) = (*i1.borrow()).clone();
        assert!(({ (*i1.borrow()) == (*i2.borrow()) }));
    }
}
pub fn test_2() {
    {
        let ii1: Value<RandomAccessIterator<u8>> =
            Rc::new(RefCell::new(RandomAccessIterator::default()));
        let ii2: Value<RandomAccessIterator<u8>> =
            Rc::new(RefCell::new(RandomAccessIterator::default()));
        let ii4: Value<RandomAccessIterator<u8>> = Rc::new(RefCell::new((*ii1.borrow()).clone()));
        let cii: Value<RandomAccessIterator<u8>> =
            Rc::new(RefCell::new(RandomAccessIterator::default()));
        assert!(({ (*ii1.borrow()) == (*ii2.borrow()) }));
        assert!(({ (*ii1.borrow()) == (*ii4.borrow()) }));
        assert!(({ !((*ii1.borrow()) != (*ii2.borrow())) }));
        assert!(({ ((*ii1.borrow()) == (*cii.borrow())) }));
        assert!(({ ((*cii.borrow()) == (*ii1.borrow())) }));
        assert!(({ !((*ii1.borrow()) != (*cii.borrow())) }));
        assert!(({ !((*cii.borrow()) != (*ii1.borrow())) }));
        assert!(({ !((*ii1.borrow()) < ({ (*cii.borrow()).clone() }).into()) }));
        assert!(({ !((*cii.borrow()) < ({ (*ii1.borrow()).clone() }).into()) }));
        assert!(({ ((*ii1.borrow()) <= ({ (*cii.borrow()).clone() }).into()) }));
        assert!(({ ((*cii.borrow()) <= ({ (*ii1.borrow()).clone() }).into()) }));
        assert!(({ !((*ii1.borrow()) > ({ (*cii.borrow()).clone() }).into()) }));
        assert!(({ !((*cii.borrow()) > ({ (*ii1.borrow()).clone() }).into()) }));
        assert!(({ ((*ii1.borrow()) >= ({ (*cii.borrow()).clone() }).into()) }));
        assert!(({ ((*cii.borrow()) >= ({ (*ii1.borrow()).clone() }).into()) }));
        assert!(
            ({
                ((*cii.borrow()).iter_distance(&({ (*ii1.borrow()).clone() }).into()) as i64
                    == 0_i64)
            })
        );
        assert!(
            ({
                ((*ii1.borrow()).iter_distance(&({ (*cii.borrow()).clone() }).into()) as i64
                    == 0_i64)
            })
        );
    }
    {
        let a: Value<Vec<u8>> = Rc::new(RefCell::new(vec![u8::default()]));
        let i1: Value<RandomAccessIterator<u8>> =
            Rc::new(RefCell::new(RandomAccessIterator::Vec(a.decay())));
        let i2: Value<RandomAccessIterator<u8>> =
            Rc::new(RefCell::new(RandomAccessIterator::default()));
        (*i2.borrow_mut()) = (*i1.borrow()).clone();
        assert!(({ (*i1.borrow()) == (*i2.borrow()) }));
    }
}
pub fn test_3() {
    {
        let ii1: Value<RandomAccessIterator<u16>> =
            Rc::new(RefCell::new(RandomAccessIterator::default()));
        let ii2: Value<RandomAccessIterator<u16>> =
            Rc::new(RefCell::new(RandomAccessIterator::default()));
        let ii4: Value<RandomAccessIterator<u16>> = Rc::new(RefCell::new((*ii1.borrow()).clone()));
        let cii: Value<RandomAccessIterator<u16>> =
            Rc::new(RefCell::new(RandomAccessIterator::default()));
        assert!(({ (*ii1.borrow()) == (*ii2.borrow()) }));
        assert!(({ (*ii1.borrow()) == (*ii4.borrow()) }));
        assert!(({ !((*ii1.borrow()) != (*ii2.borrow())) }));
        assert!(({ ((*ii1.borrow()) == (*cii.borrow())) }));
        assert!(({ ((*cii.borrow()) == (*ii1.borrow())) }));
        assert!(({ !((*ii1.borrow()) != (*cii.borrow())) }));
        assert!(({ !((*cii.borrow()) != (*ii1.borrow())) }));
        assert!(({ !((*ii1.borrow()) < ({ (*cii.borrow()).clone() }).into()) }));
        assert!(({ !((*cii.borrow()) < ({ (*ii1.borrow()).clone() }).into()) }));
        assert!(({ ((*ii1.borrow()) <= ({ (*cii.borrow()).clone() }).into()) }));
        assert!(({ ((*cii.borrow()) <= ({ (*ii1.borrow()).clone() }).into()) }));
        assert!(({ !((*ii1.borrow()) > ({ (*cii.borrow()).clone() }).into()) }));
        assert!(({ !((*cii.borrow()) > ({ (*ii1.borrow()).clone() }).into()) }));
        assert!(({ ((*ii1.borrow()) >= ({ (*cii.borrow()).clone() }).into()) }));
        assert!(({ ((*cii.borrow()) >= ({ (*ii1.borrow()).clone() }).into()) }));
        assert!(
            ({
                ((*cii.borrow()).iter_distance(&({ (*ii1.borrow()).clone() }).into()) as i64
                    == 0_i64)
            })
        );
        assert!(
            ({
                ((*ii1.borrow()).iter_distance(&({ (*cii.borrow()).clone() }).into()) as i64
                    == 0_i64)
            })
        );
    }
    {
        let a: Value<Vec<u16>> = Rc::new(RefCell::new(vec![u16::default()]));
        let i1: Value<RandomAccessIterator<u16>> =
            Rc::new(RefCell::new(RandomAccessIterator::Vec(a.decay())));
        let i2: Value<RandomAccessIterator<u16>> =
            Rc::new(RefCell::new(RandomAccessIterator::default()));
        (*i2.borrow_mut()) = (*i1.borrow()).clone();
        assert!(({ (*i1.borrow()) == (*i2.borrow()) }));
    }
}
pub fn test_4() {
    {
        let ii1: Value<RandomAccessIterator<u32>> =
            Rc::new(RefCell::new(RandomAccessIterator::default()));
        let ii2: Value<RandomAccessIterator<u32>> =
            Rc::new(RefCell::new(RandomAccessIterator::default()));
        let ii4: Value<RandomAccessIterator<u32>> = Rc::new(RefCell::new((*ii1.borrow()).clone()));
        let cii: Value<RandomAccessIterator<u32>> =
            Rc::new(RefCell::new(RandomAccessIterator::default()));
        assert!(({ (*ii1.borrow()) == (*ii2.borrow()) }));
        assert!(({ (*ii1.borrow()) == (*ii4.borrow()) }));
        assert!(({ !((*ii1.borrow()) != (*ii2.borrow())) }));
        assert!(({ ((*ii1.borrow()) == (*cii.borrow())) }));
        assert!(({ ((*cii.borrow()) == (*ii1.borrow())) }));
        assert!(({ !((*ii1.borrow()) != (*cii.borrow())) }));
        assert!(({ !((*cii.borrow()) != (*ii1.borrow())) }));
        assert!(({ !((*ii1.borrow()) < ({ (*cii.borrow()).clone() }).into()) }));
        assert!(({ !((*cii.borrow()) < ({ (*ii1.borrow()).clone() }).into()) }));
        assert!(({ ((*ii1.borrow()) <= ({ (*cii.borrow()).clone() }).into()) }));
        assert!(({ ((*cii.borrow()) <= ({ (*ii1.borrow()).clone() }).into()) }));
        assert!(({ !((*ii1.borrow()) > ({ (*cii.borrow()).clone() }).into()) }));
        assert!(({ !((*cii.borrow()) > ({ (*ii1.borrow()).clone() }).into()) }));
        assert!(({ ((*ii1.borrow()) >= ({ (*cii.borrow()).clone() }).into()) }));
        assert!(({ ((*cii.borrow()) >= ({ (*ii1.borrow()).clone() }).into()) }));
        assert!(
            ({
                ((*cii.borrow()).iter_distance(&({ (*ii1.borrow()).clone() }).into()) as i64
                    == 0_i64)
            })
        );
        assert!(
            ({
                ((*ii1.borrow()).iter_distance(&({ (*cii.borrow()).clone() }).into()) as i64
                    == 0_i64)
            })
        );
    }
    {
        let a: Value<Vec<u32>> = Rc::new(RefCell::new(vec![u32::default()]));
        let i1: Value<RandomAccessIterator<u32>> =
            Rc::new(RefCell::new(RandomAccessIterator::Vec(a.decay())));
        let i2: Value<RandomAccessIterator<u32>> =
            Rc::new(RefCell::new(RandomAccessIterator::default()));
        (*i2.borrow_mut()) = (*i1.borrow()).clone();
        assert!(({ (*i1.borrow()) == (*i2.borrow()) }));
    }
}
pub fn test_5() -> bool {
    ({ test_0() });
    ({ test_1() });
    ({ test_2() });
    ({ test_3() });
    ({ test_4() });
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
    ({ test_5() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub fn __cpp2rust_init_globals() {}
