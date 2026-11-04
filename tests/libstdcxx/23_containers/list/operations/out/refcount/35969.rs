extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test01_0() {
    {
        let list1: Value<std_list_int__std_allocator_int__> =
            Rc::new(RefCell::new(<std_list_int__std_allocator_int__>::default()));
        let list2: Value<std_list_int__std_allocator_int__> =
            Rc::new(RefCell::new(<std_list_int__std_allocator_int__>::default()));
        let i: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*i.borrow()) < 10) {
            ({ (*list1.borrow()).push_back_51(i.as_pointer()) });
            ({
                let ___x: Value<i32> = Rc::new(RefCell::new((10 - (*i.borrow()))));
                (*list2.borrow()).push_back_52(___x.as_pointer())
            });
            (*i.borrow_mut()).prefix_inc();
        }
        ({ (*list1.borrow()).sort() });
        ({ (*list2.borrow()).sort() });
        let node_of_interest: Value<BidirectionalIterator<i32>> =
            Rc::new(RefCell::new(({ (*list2.borrow()).begin_24() })));
        ({
            let ___position: BidirectionalIterator<i32> =
                <BidirectionalIterator<i32>>::from(({ (*list1.borrow()).begin_24() }));
            (*list1.borrow()).splice_69(
                ___position,
                list2.as_pointer(),
                <BidirectionalIterator<i32>>::from(({ (*node_of_interest.borrow()).clone() })),
            )
        });
        ({
            let ___position: BidirectionalIterator<i32> =
                <BidirectionalIterator<i32>>::from(({ (*list2.borrow()).begin_24() }));
            (*list2.borrow()).splice_69(
                ___position,
                list1.as_pointer(),
                <BidirectionalIterator<i32>>::from(({ (*node_of_interest.borrow()).clone() })),
            )
        });
        ({ (*list1.borrow()).merge_77(list2.as_pointer()) });
        ({
            let ___position: BidirectionalIterator<i32> =
                <BidirectionalIterator<i32>>::from(({ (*list2.borrow()).begin_24() }));
            (*list2.borrow()).splice_69(
                ___position,
                list1.as_pointer(),
                <BidirectionalIterator<i32>>::from(({ (*node_of_interest.borrow()).clone() })),
            )
        });
    }
    {
        let list1: Value<std_list_int__std_allocator_int__> =
            Rc::new(RefCell::new(<std_list_int__std_allocator_int__>::default()));
        let list2: Value<std_list_int__std_allocator_int__> =
            Rc::new(RefCell::new(<std_list_int__std_allocator_int__>::default()));
        let i: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*i.borrow()) < 10) {
            ({ (*list1.borrow()).push_back_51(i.as_pointer()) });
            ({
                let ___x: Value<i32> = Rc::new(RefCell::new((10 - (*i.borrow()))));
                (*list2.borrow()).push_back_52(___x.as_pointer())
            });
            (*i.borrow_mut()).prefix_inc();
        }
        ({ (*list1.borrow()).sort() });
        ({ (*list2.borrow()).sort() });
        let node_of_interest: Value<BidirectionalIterator<i32>> =
            Rc::new(RefCell::new(({ (*list2.borrow()).begin_24() })));
        ({
            let ___position: BidirectionalIterator<i32> =
                <BidirectionalIterator<i32>>::from(({ (*list1.borrow()).begin_24() }));
            (*list1.borrow()).splice_69(
                ___position,
                list2.as_pointer(),
                <BidirectionalIterator<i32>>::from(({ (*node_of_interest.borrow()).clone() })),
            )
        });
        ({
            let ___position: BidirectionalIterator<i32> =
                <BidirectionalIterator<i32>>::from(({ (*list2.borrow()).begin_24() }));
            (*list2.borrow()).splice_69(
                ___position,
                list1.as_pointer(),
                <BidirectionalIterator<i32>>::from(({ (*node_of_interest.borrow()).clone() })),
            )
        });
        ({
            (*list1.borrow())
                .merge_79_struct_std_less_int_(list2.as_pointer(), <std_less_int_>::default())
        });
        ({
            let ___position: BidirectionalIterator<i32> =
                <BidirectionalIterator<i32>>::from(({ (*list2.borrow()).begin_24() }));
            (*list2.borrow()).splice_69(
                ___position,
                list1.as_pointer(),
                <BidirectionalIterator<i32>>::from(({ (*node_of_interest.borrow()).clone() })),
            )
        });
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    ({ test01_0() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
