extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let x: Value<Vec<i8>> = Rc::new(RefCell::new(vec![i8::default()]));
    ({
        let ___p: RandomAccessIterator<i8> =
            <RandomAccessIterator<i8>>::from(RandomAccessIterator::Vec(x.decay()));
        (*x.borrow_mut()).insert_123(___p, 10_usize, ('x' as i8))
    });
    let test: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"test\0")));
    ({
        let ___i1: RandomAccessIterator<i8> =
            <RandomAccessIterator<i8>>::from(RandomAccessIterator::Vec(x.decay()));
        let ___i2: RandomAccessIterator<i8> =
            <RandomAccessIterator<i8>>::from(RandomAccessIterator::Vec(x.decay().to_last()));
        (*x.borrow_mut()).replace_145(___i1, ___i2, (test.as_pointer() as Ptr<i8>), 4_usize)
    });
    ({
        let ___i1: RandomAccessIterator<i8> =
            <RandomAccessIterator<i8>>::from(RandomAccessIterator::Vec(x.decay()));
        let ___i2: RandomAccessIterator<i8> =
            <RandomAccessIterator<i8>>::from(RandomAccessIterator::Vec(x.decay().to_last()));
        (*x.borrow_mut()).replace_146(___i1, ___i2, (test.as_pointer() as Ptr<i8>))
    });
    let best: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"best\0")));
    ({
        let ___i1: RandomAccessIterator<i8> =
            <RandomAccessIterator<i8>>::from(RandomAccessIterator::Vec(x.decay()));
        let ___i2: RandomAccessIterator<i8> =
            <RandomAccessIterator<i8>>::from(RandomAccessIterator::Vec(x.decay().to_last()));
        let ___k1: Ptr<i8> = (best.as_pointer() as Ptr<i8>);
        let ___k2: Ptr<i8> = (best.as_pointer() as Ptr<i8>).offset((4) as isize);
        (*x.borrow_mut()).replace_149(___i1, ___i2, ___k1, ___k2)
    });
    ({
        let ___i1: RandomAccessIterator<i8> =
            <RandomAccessIterator<i8>>::from(RandomAccessIterator::Vec(x.decay()));
        let ___i2: RandomAccessIterator<i8> =
            <RandomAccessIterator<i8>>::from(RandomAccessIterator::Vec(x.decay().to_last()));
        let ___str: Ptr<Vec<i8>> = (x.as_pointer()).reinterpret_cast::<Vec<i8>>();
        (*x.borrow_mut()).replace_144(___i1, ___i2, ___str)
    });
    ({
        let ___first: RandomAccessIterator<i8> =
            <RandomAccessIterator<i8>>::from(RandomAccessIterator::Vec(x.decay()));
        let ___last: RandomAccessIterator<i8> =
            <RandomAccessIterator<i8>>::from(RandomAccessIterator::Vec(x.decay().to_last()));
        (*x.borrow_mut()).erase_137(___first, ___last)
    });
    let w: Value<Vec<i32>> = Rc::new(RefCell::new(vec![i32::default()]));
    ({
        let ___p: RandomAccessIterator<i32> =
            <RandomAccessIterator<i32>>::from(RandomAccessIterator::Vec(w.decay()));
        (*w.borrow_mut()).insert_123(___p, 10_usize, (120 as i32))
    });
    let west: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::from([
        119 as i32, 101 as i32, 115 as i32, 116 as i32, 0 as i32,
    ])));
    ({
        let ___i1: RandomAccessIterator<i32> =
            <RandomAccessIterator<i32>>::from(RandomAccessIterator::Vec(w.decay()));
        let ___i2: RandomAccessIterator<i32> =
            <RandomAccessIterator<i32>>::from(RandomAccessIterator::Vec(w.decay().to_last()));
        (*w.borrow_mut()).replace_145(___i1, ___i2, (west.as_pointer() as Ptr<i32>), 4_usize)
    });
    ({
        let ___i1: RandomAccessIterator<i32> =
            <RandomAccessIterator<i32>>::from(RandomAccessIterator::Vec(w.decay()));
        let ___i2: RandomAccessIterator<i32> =
            <RandomAccessIterator<i32>>::from(RandomAccessIterator::Vec(w.decay().to_last()));
        (*w.borrow_mut()).replace_146(___i1, ___i2, (west.as_pointer() as Ptr<i32>))
    });
    let rest: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::from([
        114 as i32, 101 as i32, 115 as i32, 116 as i32, 0 as i32,
    ])));
    ({
        let ___i1: RandomAccessIterator<i32> =
            <RandomAccessIterator<i32>>::from(RandomAccessIterator::Vec(w.decay()));
        let ___i2: RandomAccessIterator<i32> =
            <RandomAccessIterator<i32>>::from(RandomAccessIterator::Vec(w.decay().to_last()));
        let ___k1: Ptr<i32> = (rest.as_pointer() as Ptr<i32>);
        let ___k2: Ptr<i32> = (rest.as_pointer() as Ptr<i32>).offset((4) as isize);
        (*w.borrow_mut()).replace_149(___i1, ___i2, ___k1, ___k2)
    });
    ({
        let ___i1: RandomAccessIterator<i32> =
            <RandomAccessIterator<i32>>::from(RandomAccessIterator::Vec(w.decay()));
        let ___i2: RandomAccessIterator<i32> =
            <RandomAccessIterator<i32>>::from(RandomAccessIterator::Vec(w.decay().to_last()));
        let ___str: Ptr<Vec<i32>> = (w.as_pointer()).reinterpret_cast::<Vec<i32>>();
        (*w.borrow_mut()).replace_144(___i1, ___i2, ___str)
    });
    ({
        let ___first: RandomAccessIterator<i32> =
            <RandomAccessIterator<i32>>::from(RandomAccessIterator::Vec(w.decay()));
        let ___last: RandomAccessIterator<i32> =
            <RandomAccessIterator<i32>>::from(RandomAccessIterator::Vec(w.decay().to_last()));
        (*w.borrow_mut()).erase_137(___first, ___last)
    });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
