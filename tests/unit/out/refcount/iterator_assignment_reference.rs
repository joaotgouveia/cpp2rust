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
    let m: Value<Map<i32, (Value<i32>, Value<i32>)>> = Rc::new(RefCell::new(Map::new()));
    Map::subscript_move(
        m.as_pointer(),
        Rc::new(RefCell::new(1)).as_pointer(),
        |__k| cpp_move!(__k),
    )
    .write(1);
    Map::subscript_move(
        m.as_pointer(),
        Rc::new(RefCell::new(2)).as_pointer(),
        |__k| cpp_move!(__k),
    )
    .write(2);
    let it: Value<BidirectionalIterator<(Value<i32>, Value<i32>)>> = Rc::new(RefCell::new(
        m.as_pointer()
            .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.begin()),
    ));
    let saved: Value<BidirectionalIterator<(Value<i32>, Value<i32>)>> =
        Rc::new(RefCell::new(BidirectionalIterator::default()));
    let res: Ptr<BidirectionalIterator<(Value<i32>, Value<i32>)>> = ({
        saved.as_pointer().write(({ (*it.borrow()).clone() }));
        saved.as_pointer()
    });
    assert!(({ (res).clone() } == { (saved.as_pointer()) }));
    assert!(
        ({
            ((*(*(*saved.borrow()).iter_deref().upgrade().deref())
                .0
                .borrow())
                == 1)
        })
    );
    let adv: Ptr<BidirectionalIterator<(Value<i32>, Value<i32>)>> = {
        saved.as_pointer().with_mut(|__it| {
            __it.iter_inc();
        });
        saved.as_pointer()
    };
    assert!(({ (adv).clone() } == { (saved.as_pointer()) }));
    assert!(
        ({
            ((*(*(*saved.borrow()).iter_deref().upgrade().deref())
                .0
                .borrow())
                == 2)
        })
    );
    let copied: Value<BidirectionalIterator<(Value<i32>, Value<i32>)>> =
        Rc::new(RefCell::new(BidirectionalIterator::default()));
    let cres: Ptr<BidirectionalIterator<(Value<i32>, Value<i32>)>> = ({
        copied.as_pointer().write((*saved.borrow()).clone());
        copied.as_pointer()
    });
    assert!(({ (cres).clone() } == { (copied.as_pointer()) }));
    assert!(
        ({
            ((*(*(*copied.borrow()).iter_deref().upgrade().deref())
                .0
                .borrow())
                == 2)
        })
    );
    let v: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([5, 6])));
    let vit: Value<RandomAccessIterator<i32>> =
        Rc::new(RefCell::new(RandomAccessIterator::default()));
    let vres: Ptr<RandomAccessIterator<i32>> = ({
        vit.as_pointer().write(v.decay().begin().iter_add(1_isize));
        vit.as_pointer()
    });
    assert!(({ (vres).clone() } == { (vit.as_pointer()) }));
    assert!(({ (((*vit.borrow()).iter_deref().read()) == 6) }));
    let moved: Value<RandomAccessIterator<i32>> =
        Rc::new(RefCell::new(RandomAccessIterator::default()));
    let mres: Ptr<RandomAccessIterator<i32>> = ({
        moved.as_pointer().write(({ (*vit.borrow()).clone() }));
        moved.as_pointer()
    });
    assert!(({ (mres).clone() } == { (moved.as_pointer()) }));
    assert!(({ (((*moved.borrow()).iter_deref().read()) == 6) }));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
