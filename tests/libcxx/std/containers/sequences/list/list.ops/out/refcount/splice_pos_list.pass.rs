extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub fn outstanding_alloc() -> usize {
        assert!((alloc_count_1.with(|rc| *rc.borrow()) >= dealloc_count_2.with(|rc| *rc.borrow())));
        return ((alloc_count_1.with(|rc| *rc.borrow()))
            .wrapping_sub(dealloc_count_2.with(|rc| *rc.borrow())));
    }
    pub fn reset() {
        assert!((({ malloc_allocator_base::outstanding_alloc() }) == 0_usize));
        disable_default_constructor_3.with(|rc| *rc.borrow_mut() = false);
        outstanding_bytes_0.with(|rc| *rc.borrow_mut() = 0_usize);
        alloc_count_1.with(|rc| *rc.borrow_mut() = 0_usize);
        dealloc_count_2.with(|rc| *rc.borrow_mut() = 0_usize);
    }
}
impl CppMove for malloc_allocator_base {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static outstanding_bytes_0: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static alloc_count_1: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static dealloc_count_2: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static disable_default_constructor_3: Value<bool> = Rc::new(RefCell::new(false));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_int_ {}
impl CppMove for min_allocator_int_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn test_4() -> bool {
    let a1: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3])));
    let a2: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([4, 5, 6])));
    {
        let l1: Value<std_list_int__std_allocator_int__> =
            Rc::new(RefCell::new(<std_list_int__std_allocator_int__>::default()));
        let l2: Value<std_list_int__std_allocator_int__> =
            Rc::new(RefCell::new(<std_list_int__std_allocator_int__>::default()));
        ({
            let ___position: BidirectionalIterator<i32> =
                <BidirectionalIterator<i32>>::from(({ (*l1.borrow()).end_26() }));
            (*l1.borrow()).splice_67(___position, l2.as_pointer())
        });
        assert!(({ (({ (*l1.borrow()).size() }) == 0_u64) }));
        assert!(
            ({
                (({
                    let ___first: BidirectionalIterator<i32> = ({ (*l1.borrow()).begin_24() });
                    let ___last: BidirectionalIterator<i32> = ({ (*l1.borrow()).end_26() });
                    distance_5(___first, ___last)
                }) == 0_isize)
            })
        );
        assert!(({ (({ (*l2.borrow()).size() }) == 0_u64) }));
        assert!(
            ({
                (({
                    let ___first: BidirectionalIterator<i32> = ({ (*l2.borrow()).begin_24() });
                    let ___last: BidirectionalIterator<i32> = ({ (*l2.borrow()).end_26() });
                    distance_5(___first, ___last)
                }) == 0_isize)
            })
        );
    }
    {
        let l1: Value<std_list_int__std_allocator_int__> =
            Rc::new(RefCell::new(<std_list_int__std_allocator_int__>::default()));
        let l2: Value<std_list_int__std_allocator_int__> =
            Rc::new(RefCell::new(std_list_int__std_allocator_int__::new_1(
                { (a2.as_pointer() as Ptr<i32>) },
                { (a2.as_pointer() as Ptr<i32>).offset((1) as isize) },
                None,
            )));
        ({
            let ___position: BidirectionalIterator<i32> =
                <BidirectionalIterator<i32>>::from(({ (*l1.borrow()).end_26() }));
            (*l1.borrow()).splice_67(___position, l2.as_pointer())
        });
        assert!(({ (({ (*l1.borrow()).size() }) == 1_u64) }));
        assert!(
            ({
                (({
                    let ___first: BidirectionalIterator<i32> = ({ (*l1.borrow()).begin_24() });
                    let ___last: BidirectionalIterator<i32> = ({ (*l1.borrow()).end_26() });
                    distance_5(___first, ___last)
                }) == 1_isize)
            })
        );
        assert!(({ (({ (*l2.borrow()).size() }) == 0_u64) }));
        assert!(
            ({
                (({
                    let ___first: BidirectionalIterator<i32> = ({ (*l2.borrow()).begin_24() });
                    let ___last: BidirectionalIterator<i32> = ({ (*l2.borrow()).end_26() });
                    distance_5(___first, ___last)
                }) == 0_isize)
            })
        );
        let i: Value<BidirectionalIterator<i32>> =
            Rc::new(RefCell::new(<BidirectionalIterator<i32>>::from(
                ({ (*l1.borrow()).begin_24() }),
            )));
        assert!(({ (((*i.borrow()).iter_deref().read()) == 4) }));
    }
    {
        let l1: Value<std_list_int__std_allocator_int__> =
            Rc::new(RefCell::new(<std_list_int__std_allocator_int__>::default()));
        let l2: Value<std_list_int__std_allocator_int__> =
            Rc::new(RefCell::new(std_list_int__std_allocator_int__::new_1(
                { (a2.as_pointer() as Ptr<i32>) },
                { (a2.as_pointer() as Ptr<i32>).offset((2) as isize) },
                None,
            )));
        ({
            let ___position: BidirectionalIterator<i32> =
                <BidirectionalIterator<i32>>::from(({ (*l1.borrow()).end_26() }));
            (*l1.borrow()).splice_67(___position, l2.as_pointer())
        });
        assert!(({ (({ (*l1.borrow()).size() }) == 2_u64) }));
        assert!(
            ({
                (({
                    let ___first: BidirectionalIterator<i32> = ({ (*l1.borrow()).begin_24() });
                    let ___last: BidirectionalIterator<i32> = ({ (*l1.borrow()).end_26() });
                    distance_5(___first, ___last)
                }) == 2_isize)
            })
        );
        assert!(({ (({ (*l2.borrow()).size() }) == 0_u64) }));
        assert!(
            ({
                (({
                    let ___first: BidirectionalIterator<i32> = ({ (*l2.borrow()).begin_24() });
                    let ___last: BidirectionalIterator<i32> = ({ (*l2.borrow()).end_26() });
                    distance_5(___first, ___last)
                }) == 0_isize)
            })
        );
        let i: Value<BidirectionalIterator<i32>> =
            Rc::new(RefCell::new(<BidirectionalIterator<i32>>::from(
                ({ (*l1.borrow()).begin_24() }),
            )));
        assert!(({ (((*i.borrow()).iter_deref().read()) == 4) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 5) }));
    }
    {
        let l1: Value<std_list_int__std_allocator_int__> =
            Rc::new(RefCell::new(<std_list_int__std_allocator_int__>::default()));
        let l2: Value<std_list_int__std_allocator_int__> =
            Rc::new(RefCell::new(std_list_int__std_allocator_int__::new_1(
                { (a2.as_pointer() as Ptr<i32>) },
                { (a2.as_pointer() as Ptr<i32>).offset((3) as isize) },
                None,
            )));
        ({
            let ___position: BidirectionalIterator<i32> =
                <BidirectionalIterator<i32>>::from(({ (*l1.borrow()).end_26() }));
            (*l1.borrow()).splice_67(___position, l2.as_pointer())
        });
        assert!(({ (({ (*l1.borrow()).size() }) == 3_u64) }));
        assert!(
            ({
                (({
                    let ___first: BidirectionalIterator<i32> = ({ (*l1.borrow()).begin_24() });
                    let ___last: BidirectionalIterator<i32> = ({ (*l1.borrow()).end_26() });
                    distance_5(___first, ___last)
                }) == 3_isize)
            })
        );
        assert!(({ (({ (*l2.borrow()).size() }) == 0_u64) }));
        assert!(
            ({
                (({
                    let ___first: BidirectionalIterator<i32> = ({ (*l2.borrow()).begin_24() });
                    let ___last: BidirectionalIterator<i32> = ({ (*l2.borrow()).end_26() });
                    distance_5(___first, ___last)
                }) == 0_isize)
            })
        );
        let i: Value<BidirectionalIterator<i32>> =
            Rc::new(RefCell::new(<BidirectionalIterator<i32>>::from(
                ({ (*l1.borrow()).begin_24() }),
            )));
        assert!(({ (((*i.borrow()).iter_deref().read()) == 4) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 5) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 6) }));
    }
    {
        let l1: Value<std_list_int__std_allocator_int__> =
            Rc::new(RefCell::new(std_list_int__std_allocator_int__::new_1(
                { (a1.as_pointer() as Ptr<i32>) },
                { (a1.as_pointer() as Ptr<i32>).offset((1) as isize) },
                None,
            )));
        let l2: Value<std_list_int__std_allocator_int__> =
            Rc::new(RefCell::new(<std_list_int__std_allocator_int__>::default()));
        ({
            let ___position: BidirectionalIterator<i32> =
                <BidirectionalIterator<i32>>::from(({ (*l1.borrow()).begin_24() }));
            (*l1.borrow()).splice_67(___position, l2.as_pointer())
        });
        assert!(({ (({ (*l1.borrow()).size() }) == 1_u64) }));
        assert!(
            ({
                (({
                    let ___first: BidirectionalIterator<i32> = ({ (*l1.borrow()).begin_24() });
                    let ___last: BidirectionalIterator<i32> = ({ (*l1.borrow()).end_26() });
                    distance_5(___first, ___last)
                }) == 1_isize)
            })
        );
        assert!(({ (({ (*l2.borrow()).size() }) == 0_u64) }));
        assert!(
            ({
                (({
                    let ___first: BidirectionalIterator<i32> = ({ (*l2.borrow()).begin_24() });
                    let ___last: BidirectionalIterator<i32> = ({ (*l2.borrow()).end_26() });
                    distance_5(___first, ___last)
                }) == 0_isize)
            })
        );
        let i: Value<BidirectionalIterator<i32>> =
            Rc::new(RefCell::new(<BidirectionalIterator<i32>>::from(
                ({ (*l1.borrow()).begin_24() }),
            )));
        assert!(({ (((*i.borrow()).iter_deref().read()) == 1) }));
    }
    {
        let l1: Value<std_list_int__std_allocator_int__> =
            Rc::new(RefCell::new(std_list_int__std_allocator_int__::new_1(
                { (a1.as_pointer() as Ptr<i32>) },
                { (a1.as_pointer() as Ptr<i32>).offset((1) as isize) },
                None,
            )));
        let l2: Value<std_list_int__std_allocator_int__> =
            Rc::new(RefCell::new(<std_list_int__std_allocator_int__>::default()));
        ({
            let ___position: BidirectionalIterator<i32> =
                <BidirectionalIterator<i32>>::from(({ (*l1.borrow()).end_26() }));
            (*l1.borrow()).splice_67(___position, l2.as_pointer())
        });
        assert!(({ (({ (*l1.borrow()).size() }) == 1_u64) }));
        assert!(
            ({
                (({
                    let ___first: BidirectionalIterator<i32> = ({ (*l1.borrow()).begin_24() });
                    let ___last: BidirectionalIterator<i32> = ({ (*l1.borrow()).end_26() });
                    distance_5(___first, ___last)
                }) == 1_isize)
            })
        );
        assert!(({ (({ (*l2.borrow()).size() }) == 0_u64) }));
        assert!(
            ({
                (({
                    let ___first: BidirectionalIterator<i32> = ({ (*l2.borrow()).begin_24() });
                    let ___last: BidirectionalIterator<i32> = ({ (*l2.borrow()).end_26() });
                    distance_5(___first, ___last)
                }) == 0_isize)
            })
        );
        let i: Value<BidirectionalIterator<i32>> =
            Rc::new(RefCell::new(<BidirectionalIterator<i32>>::from(
                ({ (*l1.borrow()).begin_24() }),
            )));
        assert!(({ (((*i.borrow()).iter_deref().read()) == 1) }));
    }
    {
        let l1: Value<std_list_int__std_allocator_int__> =
            Rc::new(RefCell::new(std_list_int__std_allocator_int__::new_1(
                { (a1.as_pointer() as Ptr<i32>) },
                { (a1.as_pointer() as Ptr<i32>).offset((1) as isize) },
                None,
            )));
        let l2: Value<std_list_int__std_allocator_int__> =
            Rc::new(RefCell::new(std_list_int__std_allocator_int__::new_1(
                { (a2.as_pointer() as Ptr<i32>) },
                { (a2.as_pointer() as Ptr<i32>).offset((1) as isize) },
                None,
            )));
        ({
            let ___position: BidirectionalIterator<i32> =
                <BidirectionalIterator<i32>>::from(({ (*l1.borrow()).begin_24() }));
            (*l1.borrow()).splice_67(___position, l2.as_pointer())
        });
        assert!(({ (({ (*l1.borrow()).size() }) == 2_u64) }));
        assert!(
            ({
                (({
                    let ___first: BidirectionalIterator<i32> = ({ (*l1.borrow()).begin_24() });
                    let ___last: BidirectionalIterator<i32> = ({ (*l1.borrow()).end_26() });
                    distance_5(___first, ___last)
                }) == 2_isize)
            })
        );
        assert!(({ (({ (*l2.borrow()).size() }) == 0_u64) }));
        assert!(
            ({
                (({
                    let ___first: BidirectionalIterator<i32> = ({ (*l2.borrow()).begin_24() });
                    let ___last: BidirectionalIterator<i32> = ({ (*l2.borrow()).end_26() });
                    distance_5(___first, ___last)
                }) == 0_isize)
            })
        );
        let i: Value<BidirectionalIterator<i32>> =
            Rc::new(RefCell::new(<BidirectionalIterator<i32>>::from(
                ({ (*l1.borrow()).begin_24() }),
            )));
        assert!(({ (((*i.borrow()).iter_deref().read()) == 4) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 1) }));
    }
    {
        let l1: Value<std_list_int__std_allocator_int__> =
            Rc::new(RefCell::new(std_list_int__std_allocator_int__::new_1(
                { (a1.as_pointer() as Ptr<i32>) },
                { (a1.as_pointer() as Ptr<i32>).offset((1) as isize) },
                None,
            )));
        let l2: Value<std_list_int__std_allocator_int__> =
            Rc::new(RefCell::new(std_list_int__std_allocator_int__::new_1(
                { (a2.as_pointer() as Ptr<i32>) },
                { (a2.as_pointer() as Ptr<i32>).offset((1) as isize) },
                None,
            )));
        ({
            let ___position: BidirectionalIterator<i32> =
                <BidirectionalIterator<i32>>::from(({ (*l1.borrow()).end_26() }));
            (*l1.borrow()).splice_67(___position, l2.as_pointer())
        });
        assert!(({ (({ (*l1.borrow()).size() }) == 2_u64) }));
        assert!(
            ({
                (({
                    let ___first: BidirectionalIterator<i32> = ({ (*l1.borrow()).begin_24() });
                    let ___last: BidirectionalIterator<i32> = ({ (*l1.borrow()).end_26() });
                    distance_5(___first, ___last)
                }) == 2_isize)
            })
        );
        assert!(({ (({ (*l2.borrow()).size() }) == 0_u64) }));
        assert!(
            ({
                (({
                    let ___first: BidirectionalIterator<i32> = ({ (*l2.borrow()).begin_24() });
                    let ___last: BidirectionalIterator<i32> = ({ (*l2.borrow()).end_26() });
                    distance_5(___first, ___last)
                }) == 0_isize)
            })
        );
        let i: Value<BidirectionalIterator<i32>> =
            Rc::new(RefCell::new(<BidirectionalIterator<i32>>::from(
                ({ (*l1.borrow()).begin_24() }),
            )));
        assert!(({ (((*i.borrow()).iter_deref().read()) == 1) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 4) }));
    }
    {
        let l1: Value<std_list_int__std_allocator_int__> =
            Rc::new(RefCell::new(std_list_int__std_allocator_int__::new_1(
                { (a1.as_pointer() as Ptr<i32>) },
                { (a1.as_pointer() as Ptr<i32>).offset((1) as isize) },
                None,
            )));
        let l2: Value<std_list_int__std_allocator_int__> =
            Rc::new(RefCell::new(std_list_int__std_allocator_int__::new_1(
                { (a2.as_pointer() as Ptr<i32>) },
                { (a2.as_pointer() as Ptr<i32>).offset((2) as isize) },
                None,
            )));
        ({
            let ___position: BidirectionalIterator<i32> =
                <BidirectionalIterator<i32>>::from(({ (*l1.borrow()).begin_24() }));
            (*l1.borrow()).splice_67(___position, l2.as_pointer())
        });
        assert!(({ (({ (*l1.borrow()).size() }) == 3_u64) }));
        assert!(
            ({
                (({
                    let ___first: BidirectionalIterator<i32> = ({ (*l1.borrow()).begin_24() });
                    let ___last: BidirectionalIterator<i32> = ({ (*l1.borrow()).end_26() });
                    distance_5(___first, ___last)
                }) == 3_isize)
            })
        );
        assert!(({ (({ (*l2.borrow()).size() }) == 0_u64) }));
        assert!(
            ({
                (({
                    let ___first: BidirectionalIterator<i32> = ({ (*l2.borrow()).begin_24() });
                    let ___last: BidirectionalIterator<i32> = ({ (*l2.borrow()).end_26() });
                    distance_5(___first, ___last)
                }) == 0_isize)
            })
        );
        let i: Value<BidirectionalIterator<i32>> =
            Rc::new(RefCell::new(<BidirectionalIterator<i32>>::from(
                ({ (*l1.borrow()).begin_24() }),
            )));
        assert!(({ (((*i.borrow()).iter_deref().read()) == 4) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 5) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 1) }));
    }
    {
        let l1: Value<std_list_int__std_allocator_int__> =
            Rc::new(RefCell::new(std_list_int__std_allocator_int__::new_1(
                { (a1.as_pointer() as Ptr<i32>) },
                { (a1.as_pointer() as Ptr<i32>).offset((1) as isize) },
                None,
            )));
        let l2: Value<std_list_int__std_allocator_int__> =
            Rc::new(RefCell::new(std_list_int__std_allocator_int__::new_1(
                { (a2.as_pointer() as Ptr<i32>) },
                { (a2.as_pointer() as Ptr<i32>).offset((2) as isize) },
                None,
            )));
        ({
            let ___position: BidirectionalIterator<i32> =
                <BidirectionalIterator<i32>>::from(({ (*l1.borrow()).end_26() }));
            (*l1.borrow()).splice_67(___position, l2.as_pointer())
        });
        assert!(({ (({ (*l1.borrow()).size() }) == 3_u64) }));
        assert!(
            ({
                (({
                    let ___first: BidirectionalIterator<i32> = ({ (*l1.borrow()).begin_24() });
                    let ___last: BidirectionalIterator<i32> = ({ (*l1.borrow()).end_26() });
                    distance_5(___first, ___last)
                }) == 3_isize)
            })
        );
        assert!(({ (({ (*l2.borrow()).size() }) == 0_u64) }));
        assert!(
            ({
                (({
                    let ___first: BidirectionalIterator<i32> = ({ (*l2.borrow()).begin_24() });
                    let ___last: BidirectionalIterator<i32> = ({ (*l2.borrow()).end_26() });
                    distance_5(___first, ___last)
                }) == 0_isize)
            })
        );
        let i: Value<BidirectionalIterator<i32>> =
            Rc::new(RefCell::new(<BidirectionalIterator<i32>>::from(
                ({ (*l1.borrow()).begin_24() }),
            )));
        assert!(({ (((*i.borrow()).iter_deref().read()) == 1) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 4) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 5) }));
    }
    {
        let l1: Value<std_list_int__std_allocator_int__> =
            Rc::new(RefCell::new(std_list_int__std_allocator_int__::new_1(
                { (a1.as_pointer() as Ptr<i32>) },
                { (a1.as_pointer() as Ptr<i32>).offset((1) as isize) },
                None,
            )));
        let l2: Value<std_list_int__std_allocator_int__> =
            Rc::new(RefCell::new(std_list_int__std_allocator_int__::new_1(
                { (a2.as_pointer() as Ptr<i32>) },
                { (a2.as_pointer() as Ptr<i32>).offset((3) as isize) },
                None,
            )));
        ({
            let ___position: BidirectionalIterator<i32> =
                <BidirectionalIterator<i32>>::from(({ (*l1.borrow()).begin_24() }));
            (*l1.borrow()).splice_67(___position, l2.as_pointer())
        });
        assert!(({ (({ (*l1.borrow()).size() }) == 4_u64) }));
        assert!(
            ({
                (({
                    let ___first: BidirectionalIterator<i32> = ({ (*l1.borrow()).begin_24() });
                    let ___last: BidirectionalIterator<i32> = ({ (*l1.borrow()).end_26() });
                    distance_5(___first, ___last)
                }) == 4_isize)
            })
        );
        assert!(({ (({ (*l2.borrow()).size() }) == 0_u64) }));
        assert!(
            ({
                (({
                    let ___first: BidirectionalIterator<i32> = ({ (*l2.borrow()).begin_24() });
                    let ___last: BidirectionalIterator<i32> = ({ (*l2.borrow()).end_26() });
                    distance_5(___first, ___last)
                }) == 0_isize)
            })
        );
        let i: Value<BidirectionalIterator<i32>> =
            Rc::new(RefCell::new(<BidirectionalIterator<i32>>::from(
                ({ (*l1.borrow()).begin_24() }),
            )));
        assert!(({ (((*i.borrow()).iter_deref().read()) == 4) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 5) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 6) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 1) }));
    }
    {
        let l1: Value<std_list_int__std_allocator_int__> =
            Rc::new(RefCell::new(std_list_int__std_allocator_int__::new_1(
                { (a1.as_pointer() as Ptr<i32>) },
                { (a1.as_pointer() as Ptr<i32>).offset((1) as isize) },
                None,
            )));
        let l2: Value<std_list_int__std_allocator_int__> =
            Rc::new(RefCell::new(std_list_int__std_allocator_int__::new_1(
                { (a2.as_pointer() as Ptr<i32>) },
                { (a2.as_pointer() as Ptr<i32>).offset((3) as isize) },
                None,
            )));
        ({
            let ___position: BidirectionalIterator<i32> =
                <BidirectionalIterator<i32>>::from(({ (*l1.borrow()).end_26() }));
            (*l1.borrow()).splice_67(___position, l2.as_pointer())
        });
        assert!(({ (({ (*l1.borrow()).size() }) == 4_u64) }));
        assert!(
            ({
                (({
                    let ___first: BidirectionalIterator<i32> = ({ (*l1.borrow()).begin_24() });
                    let ___last: BidirectionalIterator<i32> = ({ (*l1.borrow()).end_26() });
                    distance_5(___first, ___last)
                }) == 4_isize)
            })
        );
        assert!(({ (({ (*l2.borrow()).size() }) == 0_u64) }));
        assert!(
            ({
                (({
                    let ___first: BidirectionalIterator<i32> = ({ (*l2.borrow()).begin_24() });
                    let ___last: BidirectionalIterator<i32> = ({ (*l2.borrow()).end_26() });
                    distance_5(___first, ___last)
                }) == 0_isize)
            })
        );
        let i: Value<BidirectionalIterator<i32>> =
            Rc::new(RefCell::new(<BidirectionalIterator<i32>>::from(
                ({ (*l1.borrow()).begin_24() }),
            )));
        assert!(({ (((*i.borrow()).iter_deref().read()) == 1) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 4) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 5) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 6) }));
    }
    {
        let l1: Value<std_list_int__std_allocator_int__> =
            Rc::new(RefCell::new(std_list_int__std_allocator_int__::new_1(
                { (a1.as_pointer() as Ptr<i32>) },
                { (a1.as_pointer() as Ptr<i32>).offset((2) as isize) },
                None,
            )));
        let l2: Value<std_list_int__std_allocator_int__> =
            Rc::new(RefCell::new(<std_list_int__std_allocator_int__>::default()));
        ({
            let ___position: BidirectionalIterator<i32> =
                <BidirectionalIterator<i32>>::from(({ (*l1.borrow()).begin_24() }));
            (*l1.borrow()).splice_67(___position, l2.as_pointer())
        });
        assert!(({ (({ (*l1.borrow()).size() }) == 2_u64) }));
        assert!(
            ({
                (({
                    let ___first: BidirectionalIterator<i32> = ({ (*l1.borrow()).begin_24() });
                    let ___last: BidirectionalIterator<i32> = ({ (*l1.borrow()).end_26() });
                    distance_5(___first, ___last)
                }) == 2_isize)
            })
        );
        assert!(({ (({ (*l2.borrow()).size() }) == 0_u64) }));
        assert!(
            ({
                (({
                    let ___first: BidirectionalIterator<i32> = ({ (*l2.borrow()).begin_24() });
                    let ___last: BidirectionalIterator<i32> = ({ (*l2.borrow()).end_26() });
                    distance_5(___first, ___last)
                }) == 0_isize)
            })
        );
        let i: Value<BidirectionalIterator<i32>> =
            Rc::new(RefCell::new(<BidirectionalIterator<i32>>::from(
                ({ (*l1.borrow()).begin_24() }),
            )));
        assert!(({ (((*i.borrow()).iter_deref().read()) == 1) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 2) }));
    }
    {
        let l1: Value<std_list_int__std_allocator_int__> =
            Rc::new(RefCell::new(std_list_int__std_allocator_int__::new_1(
                { (a1.as_pointer() as Ptr<i32>) },
                { (a1.as_pointer() as Ptr<i32>).offset((2) as isize) },
                None,
            )));
        let l2: Value<std_list_int__std_allocator_int__> =
            Rc::new(RefCell::new(<std_list_int__std_allocator_int__>::default()));
        ({
            let ___position: BidirectionalIterator<i32> = <BidirectionalIterator<i32>>::from(
                libcc2rs::next_refcount(({ ({ (*l1.borrow()).begin_24() }) }), 1_isize),
            );
            (*l1.borrow()).splice_67(___position, l2.as_pointer())
        });
        assert!(({ (({ (*l1.borrow()).size() }) == 2_u64) }));
        assert!(
            ({
                (({
                    let ___first: BidirectionalIterator<i32> = ({ (*l1.borrow()).begin_24() });
                    let ___last: BidirectionalIterator<i32> = ({ (*l1.borrow()).end_26() });
                    distance_5(___first, ___last)
                }) == 2_isize)
            })
        );
        assert!(({ (({ (*l2.borrow()).size() }) == 0_u64) }));
        assert!(
            ({
                (({
                    let ___first: BidirectionalIterator<i32> = ({ (*l2.borrow()).begin_24() });
                    let ___last: BidirectionalIterator<i32> = ({ (*l2.borrow()).end_26() });
                    distance_5(___first, ___last)
                }) == 0_isize)
            })
        );
        let i: Value<BidirectionalIterator<i32>> =
            Rc::new(RefCell::new(<BidirectionalIterator<i32>>::from(
                ({ (*l1.borrow()).begin_24() }),
            )));
        assert!(({ (((*i.borrow()).iter_deref().read()) == 1) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 2) }));
    }
    {
        let l1: Value<std_list_int__std_allocator_int__> =
            Rc::new(RefCell::new(std_list_int__std_allocator_int__::new_1(
                { (a1.as_pointer() as Ptr<i32>) },
                { (a1.as_pointer() as Ptr<i32>).offset((2) as isize) },
                None,
            )));
        let l2: Value<std_list_int__std_allocator_int__> =
            Rc::new(RefCell::new(<std_list_int__std_allocator_int__>::default()));
        ({
            let ___position: BidirectionalIterator<i32> = <BidirectionalIterator<i32>>::from(
                libcc2rs::next_refcount(({ ({ (*l1.borrow()).begin_24() }) }), 2_isize),
            );
            (*l1.borrow()).splice_67(___position, l2.as_pointer())
        });
        assert!(({ (({ (*l1.borrow()).size() }) == 2_u64) }));
        assert!(
            ({
                (({
                    let ___first: BidirectionalIterator<i32> = ({ (*l1.borrow()).begin_24() });
                    let ___last: BidirectionalIterator<i32> = ({ (*l1.borrow()).end_26() });
                    distance_5(___first, ___last)
                }) == 2_isize)
            })
        );
        assert!(({ (({ (*l2.borrow()).size() }) == 0_u64) }));
        assert!(
            ({
                (({
                    let ___first: BidirectionalIterator<i32> = ({ (*l2.borrow()).begin_24() });
                    let ___last: BidirectionalIterator<i32> = ({ (*l2.borrow()).end_26() });
                    distance_5(___first, ___last)
                }) == 0_isize)
            })
        );
        let i: Value<BidirectionalIterator<i32>> =
            Rc::new(RefCell::new(<BidirectionalIterator<i32>>::from(
                ({ (*l1.borrow()).begin_24() }),
            )));
        assert!(({ (((*i.borrow()).iter_deref().read()) == 1) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 2) }));
    }
    {
        let l1: Value<std_list_int__std_allocator_int__> =
            Rc::new(RefCell::new(std_list_int__std_allocator_int__::new_1(
                { (a1.as_pointer() as Ptr<i32>) },
                { (a1.as_pointer() as Ptr<i32>).offset((2) as isize) },
                None,
            )));
        let l2: Value<std_list_int__std_allocator_int__> =
            Rc::new(RefCell::new(std_list_int__std_allocator_int__::new_1(
                { (a2.as_pointer() as Ptr<i32>) },
                { (a2.as_pointer() as Ptr<i32>).offset((1) as isize) },
                None,
            )));
        ({
            let ___position: BidirectionalIterator<i32> =
                <BidirectionalIterator<i32>>::from(({ (*l1.borrow()).begin_24() }));
            (*l1.borrow()).splice_67(___position, l2.as_pointer())
        });
        assert!(({ (({ (*l1.borrow()).size() }) == 3_u64) }));
        assert!(
            ({
                (({
                    let ___first: BidirectionalIterator<i32> = ({ (*l1.borrow()).begin_24() });
                    let ___last: BidirectionalIterator<i32> = ({ (*l1.borrow()).end_26() });
                    distance_5(___first, ___last)
                }) == 3_isize)
            })
        );
        assert!(({ (({ (*l2.borrow()).size() }) == 0_u64) }));
        assert!(
            ({
                (({
                    let ___first: BidirectionalIterator<i32> = ({ (*l2.borrow()).begin_24() });
                    let ___last: BidirectionalIterator<i32> = ({ (*l2.borrow()).end_26() });
                    distance_5(___first, ___last)
                }) == 0_isize)
            })
        );
        let i: Value<BidirectionalIterator<i32>> =
            Rc::new(RefCell::new(<BidirectionalIterator<i32>>::from(
                ({ (*l1.borrow()).begin_24() }),
            )));
        assert!(({ (((*i.borrow()).iter_deref().read()) == 4) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 1) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 2) }));
    }
    {
        let l1: Value<std_list_int__std_allocator_int__> =
            Rc::new(RefCell::new(std_list_int__std_allocator_int__::new_1(
                { (a1.as_pointer() as Ptr<i32>) },
                { (a1.as_pointer() as Ptr<i32>).offset((2) as isize) },
                None,
            )));
        let l2: Value<std_list_int__std_allocator_int__> =
            Rc::new(RefCell::new(std_list_int__std_allocator_int__::new_1(
                { (a2.as_pointer() as Ptr<i32>) },
                { (a2.as_pointer() as Ptr<i32>).offset((1) as isize) },
                None,
            )));
        ({
            let ___position: BidirectionalIterator<i32> = <BidirectionalIterator<i32>>::from(
                libcc2rs::next_refcount(({ ({ (*l1.borrow()).begin_24() }) }), 1_isize),
            );
            (*l1.borrow()).splice_67(___position, l2.as_pointer())
        });
        assert!(({ (({ (*l1.borrow()).size() }) == 3_u64) }));
        assert!(
            ({
                (({
                    let ___first: BidirectionalIterator<i32> = ({ (*l1.borrow()).begin_24() });
                    let ___last: BidirectionalIterator<i32> = ({ (*l1.borrow()).end_26() });
                    distance_5(___first, ___last)
                }) == 3_isize)
            })
        );
        assert!(({ (({ (*l2.borrow()).size() }) == 0_u64) }));
        assert!(
            ({
                (({
                    let ___first: BidirectionalIterator<i32> = ({ (*l2.borrow()).begin_24() });
                    let ___last: BidirectionalIterator<i32> = ({ (*l2.borrow()).end_26() });
                    distance_5(___first, ___last)
                }) == 0_isize)
            })
        );
        let i: Value<BidirectionalIterator<i32>> =
            Rc::new(RefCell::new(<BidirectionalIterator<i32>>::from(
                ({ (*l1.borrow()).begin_24() }),
            )));
        assert!(({ (((*i.borrow()).iter_deref().read()) == 1) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 4) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 2) }));
    }
    {
        let l1: Value<std_list_int__std_allocator_int__> =
            Rc::new(RefCell::new(std_list_int__std_allocator_int__::new_1(
                { (a1.as_pointer() as Ptr<i32>) },
                { (a1.as_pointer() as Ptr<i32>).offset((2) as isize) },
                None,
            )));
        let l2: Value<std_list_int__std_allocator_int__> =
            Rc::new(RefCell::new(std_list_int__std_allocator_int__::new_1(
                { (a2.as_pointer() as Ptr<i32>) },
                { (a2.as_pointer() as Ptr<i32>).offset((1) as isize) },
                None,
            )));
        ({
            let ___position: BidirectionalIterator<i32> = <BidirectionalIterator<i32>>::from(
                libcc2rs::next_refcount(({ ({ (*l1.borrow()).begin_24() }) }), 2_isize),
            );
            (*l1.borrow()).splice_67(___position, l2.as_pointer())
        });
        assert!(({ (({ (*l1.borrow()).size() }) == 3_u64) }));
        assert!(
            ({
                (({
                    let ___first: BidirectionalIterator<i32> = ({ (*l1.borrow()).begin_24() });
                    let ___last: BidirectionalIterator<i32> = ({ (*l1.borrow()).end_26() });
                    distance_5(___first, ___last)
                }) == 3_isize)
            })
        );
        assert!(({ (({ (*l2.borrow()).size() }) == 0_u64) }));
        assert!(
            ({
                (({
                    let ___first: BidirectionalIterator<i32> = ({ (*l2.borrow()).begin_24() });
                    let ___last: BidirectionalIterator<i32> = ({ (*l2.borrow()).end_26() });
                    distance_5(___first, ___last)
                }) == 0_isize)
            })
        );
        let i: Value<BidirectionalIterator<i32>> =
            Rc::new(RefCell::new(<BidirectionalIterator<i32>>::from(
                ({ (*l1.borrow()).begin_24() }),
            )));
        assert!(({ (((*i.borrow()).iter_deref().read()) == 1) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 2) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 4) }));
    }
    {
        let l1: Value<std_list_int__std_allocator_int__> =
            Rc::new(RefCell::new(std_list_int__std_allocator_int__::new_1(
                { (a1.as_pointer() as Ptr<i32>) },
                { (a1.as_pointer() as Ptr<i32>).offset((2) as isize) },
                None,
            )));
        let l2: Value<std_list_int__std_allocator_int__> =
            Rc::new(RefCell::new(std_list_int__std_allocator_int__::new_1(
                { (a2.as_pointer() as Ptr<i32>) },
                { (a2.as_pointer() as Ptr<i32>).offset((2) as isize) },
                None,
            )));
        ({
            let ___position: BidirectionalIterator<i32> =
                <BidirectionalIterator<i32>>::from(({ (*l1.borrow()).begin_24() }));
            (*l1.borrow()).splice_67(___position, l2.as_pointer())
        });
        assert!(({ (({ (*l1.borrow()).size() }) == 4_u64) }));
        assert!(
            ({
                (({
                    let ___first: BidirectionalIterator<i32> = ({ (*l1.borrow()).begin_24() });
                    let ___last: BidirectionalIterator<i32> = ({ (*l1.borrow()).end_26() });
                    distance_5(___first, ___last)
                }) == 4_isize)
            })
        );
        assert!(({ (({ (*l2.borrow()).size() }) == 0_u64) }));
        assert!(
            ({
                (({
                    let ___first: BidirectionalIterator<i32> = ({ (*l2.borrow()).begin_24() });
                    let ___last: BidirectionalIterator<i32> = ({ (*l2.borrow()).end_26() });
                    distance_5(___first, ___last)
                }) == 0_isize)
            })
        );
        let i: Value<BidirectionalIterator<i32>> =
            Rc::new(RefCell::new(<BidirectionalIterator<i32>>::from(
                ({ (*l1.borrow()).begin_24() }),
            )));
        assert!(({ (((*i.borrow()).iter_deref().read()) == 4) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 5) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 1) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 2) }));
    }
    {
        let l1: Value<std_list_int__std_allocator_int__> =
            Rc::new(RefCell::new(std_list_int__std_allocator_int__::new_1(
                { (a1.as_pointer() as Ptr<i32>) },
                { (a1.as_pointer() as Ptr<i32>).offset((2) as isize) },
                None,
            )));
        let l2: Value<std_list_int__std_allocator_int__> =
            Rc::new(RefCell::new(std_list_int__std_allocator_int__::new_1(
                { (a2.as_pointer() as Ptr<i32>) },
                { (a2.as_pointer() as Ptr<i32>).offset((2) as isize) },
                None,
            )));
        ({
            let ___position: BidirectionalIterator<i32> = <BidirectionalIterator<i32>>::from(
                libcc2rs::next_refcount(({ ({ (*l1.borrow()).begin_24() }) }), 1_isize),
            );
            (*l1.borrow()).splice_67(___position, l2.as_pointer())
        });
        assert!(({ (({ (*l1.borrow()).size() }) == 4_u64) }));
        assert!(
            ({
                (({
                    let ___first: BidirectionalIterator<i32> = ({ (*l1.borrow()).begin_24() });
                    let ___last: BidirectionalIterator<i32> = ({ (*l1.borrow()).end_26() });
                    distance_5(___first, ___last)
                }) == 4_isize)
            })
        );
        assert!(({ (({ (*l2.borrow()).size() }) == 0_u64) }));
        assert!(
            ({
                (({
                    let ___first: BidirectionalIterator<i32> = ({ (*l2.borrow()).begin_24() });
                    let ___last: BidirectionalIterator<i32> = ({ (*l2.borrow()).end_26() });
                    distance_5(___first, ___last)
                }) == 0_isize)
            })
        );
        let i: Value<BidirectionalIterator<i32>> =
            Rc::new(RefCell::new(<BidirectionalIterator<i32>>::from(
                ({ (*l1.borrow()).begin_24() }),
            )));
        assert!(({ (((*i.borrow()).iter_deref().read()) == 1) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 4) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 5) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 2) }));
    }
    {
        let l1: Value<std_list_int__std_allocator_int__> =
            Rc::new(RefCell::new(std_list_int__std_allocator_int__::new_1(
                { (a1.as_pointer() as Ptr<i32>) },
                { (a1.as_pointer() as Ptr<i32>).offset((2) as isize) },
                None,
            )));
        let l2: Value<std_list_int__std_allocator_int__> =
            Rc::new(RefCell::new(std_list_int__std_allocator_int__::new_1(
                { (a2.as_pointer() as Ptr<i32>) },
                { (a2.as_pointer() as Ptr<i32>).offset((2) as isize) },
                None,
            )));
        ({
            let ___position: BidirectionalIterator<i32> = <BidirectionalIterator<i32>>::from(
                libcc2rs::next_refcount(({ ({ (*l1.borrow()).begin_24() }) }), 2_isize),
            );
            (*l1.borrow()).splice_67(___position, l2.as_pointer())
        });
        assert!(({ (({ (*l1.borrow()).size() }) == 4_u64) }));
        assert!(
            ({
                (({
                    let ___first: BidirectionalIterator<i32> = ({ (*l1.borrow()).begin_24() });
                    let ___last: BidirectionalIterator<i32> = ({ (*l1.borrow()).end_26() });
                    distance_5(___first, ___last)
                }) == 4_isize)
            })
        );
        assert!(({ (({ (*l2.borrow()).size() }) == 0_u64) }));
        assert!(
            ({
                (({
                    let ___first: BidirectionalIterator<i32> = ({ (*l2.borrow()).begin_24() });
                    let ___last: BidirectionalIterator<i32> = ({ (*l2.borrow()).end_26() });
                    distance_5(___first, ___last)
                }) == 0_isize)
            })
        );
        let i: Value<BidirectionalIterator<i32>> =
            Rc::new(RefCell::new(<BidirectionalIterator<i32>>::from(
                ({ (*l1.borrow()).begin_24() }),
            )));
        assert!(({ (((*i.borrow()).iter_deref().read()) == 1) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 2) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 4) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 5) }));
    }
    {
        let l1: Value<std_list_int__std_allocator_int__> =
            Rc::new(RefCell::new(std_list_int__std_allocator_int__::new_1(
                { (a1.as_pointer() as Ptr<i32>) },
                { (a1.as_pointer() as Ptr<i32>).offset((3) as isize) },
                None,
            )));
        let l2: Value<std_list_int__std_allocator_int__> =
            Rc::new(RefCell::new(std_list_int__std_allocator_int__::new_1(
                { (a2.as_pointer() as Ptr<i32>) },
                { (a2.as_pointer() as Ptr<i32>).offset((3) as isize) },
                None,
            )));
        ({
            let ___position: BidirectionalIterator<i32> =
                <BidirectionalIterator<i32>>::from(({ (*l1.borrow()).begin_24() }));
            (*l1.borrow()).splice_67(___position, l2.as_pointer())
        });
        assert!(({ (({ (*l1.borrow()).size() }) == 6_u64) }));
        assert!(
            ({
                (({
                    let ___first: BidirectionalIterator<i32> = ({ (*l1.borrow()).begin_24() });
                    let ___last: BidirectionalIterator<i32> = ({ (*l1.borrow()).end_26() });
                    distance_5(___first, ___last)
                }) == 6_isize)
            })
        );
        assert!(({ (({ (*l2.borrow()).size() }) == 0_u64) }));
        assert!(
            ({
                (({
                    let ___first: BidirectionalIterator<i32> = ({ (*l2.borrow()).begin_24() });
                    let ___last: BidirectionalIterator<i32> = ({ (*l2.borrow()).end_26() });
                    distance_5(___first, ___last)
                }) == 0_isize)
            })
        );
        let i: Value<BidirectionalIterator<i32>> =
            Rc::new(RefCell::new(<BidirectionalIterator<i32>>::from(
                ({ (*l1.borrow()).begin_24() }),
            )));
        assert!(({ (((*i.borrow()).iter_deref().read()) == 4) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 5) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 6) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 1) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 2) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 3) }));
    }
    {
        let l1: Value<std_list_int__std_allocator_int__> =
            Rc::new(RefCell::new(std_list_int__std_allocator_int__::new_1(
                { (a1.as_pointer() as Ptr<i32>) },
                { (a1.as_pointer() as Ptr<i32>).offset((3) as isize) },
                None,
            )));
        let l2: Value<std_list_int__std_allocator_int__> =
            Rc::new(RefCell::new(std_list_int__std_allocator_int__::new_1(
                { (a2.as_pointer() as Ptr<i32>) },
                { (a2.as_pointer() as Ptr<i32>).offset((3) as isize) },
                None,
            )));
        ({
            let ___position: BidirectionalIterator<i32> = <BidirectionalIterator<i32>>::from(
                libcc2rs::next_refcount(({ ({ (*l1.borrow()).begin_24() }) }), 1_isize),
            );
            (*l1.borrow()).splice_67(___position, l2.as_pointer())
        });
        assert!(({ (({ (*l1.borrow()).size() }) == 6_u64) }));
        assert!(
            ({
                (({
                    let ___first: BidirectionalIterator<i32> = ({ (*l1.borrow()).begin_24() });
                    let ___last: BidirectionalIterator<i32> = ({ (*l1.borrow()).end_26() });
                    distance_5(___first, ___last)
                }) == 6_isize)
            })
        );
        assert!(({ (({ (*l2.borrow()).size() }) == 0_u64) }));
        assert!(
            ({
                (({
                    let ___first: BidirectionalIterator<i32> = ({ (*l2.borrow()).begin_24() });
                    let ___last: BidirectionalIterator<i32> = ({ (*l2.borrow()).end_26() });
                    distance_5(___first, ___last)
                }) == 0_isize)
            })
        );
        let i: Value<BidirectionalIterator<i32>> =
            Rc::new(RefCell::new(<BidirectionalIterator<i32>>::from(
                ({ (*l1.borrow()).begin_24() }),
            )));
        assert!(({ (((*i.borrow()).iter_deref().read()) == 1) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 4) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 5) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 6) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 2) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 3) }));
    }
    {
        let l1: Value<std_list_int__std_allocator_int__> =
            Rc::new(RefCell::new(std_list_int__std_allocator_int__::new_1(
                { (a1.as_pointer() as Ptr<i32>) },
                { (a1.as_pointer() as Ptr<i32>).offset((3) as isize) },
                None,
            )));
        let l2: Value<std_list_int__std_allocator_int__> =
            Rc::new(RefCell::new(std_list_int__std_allocator_int__::new_1(
                { (a2.as_pointer() as Ptr<i32>) },
                { (a2.as_pointer() as Ptr<i32>).offset((3) as isize) },
                None,
            )));
        ({
            let ___position: BidirectionalIterator<i32> = <BidirectionalIterator<i32>>::from(
                libcc2rs::next_refcount(({ ({ (*l1.borrow()).begin_24() }) }), 2_isize),
            );
            (*l1.borrow()).splice_67(___position, l2.as_pointer())
        });
        assert!(({ (({ (*l1.borrow()).size() }) == 6_u64) }));
        assert!(
            ({
                (({
                    let ___first: BidirectionalIterator<i32> = ({ (*l1.borrow()).begin_24() });
                    let ___last: BidirectionalIterator<i32> = ({ (*l1.borrow()).end_26() });
                    distance_5(___first, ___last)
                }) == 6_isize)
            })
        );
        assert!(({ (({ (*l2.borrow()).size() }) == 0_u64) }));
        assert!(
            ({
                (({
                    let ___first: BidirectionalIterator<i32> = ({ (*l2.borrow()).begin_24() });
                    let ___last: BidirectionalIterator<i32> = ({ (*l2.borrow()).end_26() });
                    distance_5(___first, ___last)
                }) == 0_isize)
            })
        );
        let i: Value<BidirectionalIterator<i32>> =
            Rc::new(RefCell::new(<BidirectionalIterator<i32>>::from(
                ({ (*l1.borrow()).begin_24() }),
            )));
        assert!(({ (((*i.borrow()).iter_deref().read()) == 1) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 2) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 4) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 5) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 6) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 3) }));
    }
    {
        let l1: Value<std_list_int__std_allocator_int__> =
            Rc::new(RefCell::new(std_list_int__std_allocator_int__::new_1(
                { (a1.as_pointer() as Ptr<i32>) },
                { (a1.as_pointer() as Ptr<i32>).offset((3) as isize) },
                None,
            )));
        let l2: Value<std_list_int__std_allocator_int__> =
            Rc::new(RefCell::new(std_list_int__std_allocator_int__::new_1(
                { (a2.as_pointer() as Ptr<i32>) },
                { (a2.as_pointer() as Ptr<i32>).offset((3) as isize) },
                None,
            )));
        ({
            let ___position: BidirectionalIterator<i32> = <BidirectionalIterator<i32>>::from(
                libcc2rs::next_refcount(({ ({ (*l1.borrow()).begin_24() }) }), 3_isize),
            );
            (*l1.borrow()).splice_67(___position, l2.as_pointer())
        });
        assert!(({ (({ (*l1.borrow()).size() }) == 6_u64) }));
        assert!(
            ({
                (({
                    let ___first: BidirectionalIterator<i32> = ({ (*l1.borrow()).begin_24() });
                    let ___last: BidirectionalIterator<i32> = ({ (*l1.borrow()).end_26() });
                    distance_5(___first, ___last)
                }) == 6_isize)
            })
        );
        assert!(({ (({ (*l2.borrow()).size() }) == 0_u64) }));
        assert!(
            ({
                (({
                    let ___first: BidirectionalIterator<i32> = ({ (*l2.borrow()).begin_24() });
                    let ___last: BidirectionalIterator<i32> = ({ (*l2.borrow()).end_26() });
                    distance_5(___first, ___last)
                }) == 0_isize)
            })
        );
        let i: Value<BidirectionalIterator<i32>> =
            Rc::new(RefCell::new(<BidirectionalIterator<i32>>::from(
                ({ (*l1.borrow()).begin_24() }),
            )));
        assert!(({ (((*i.borrow()).iter_deref().read()) == 1) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 2) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 3) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 4) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 5) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 6) }));
    }
    {
        let l1: Value<std_list_int__min_allocator_int__> =
            Rc::new(RefCell::new(<std_list_int__min_allocator_int__>::default()));
        let l2: Value<std_list_int__min_allocator_int__> =
            Rc::new(RefCell::new(<std_list_int__min_allocator_int__>::default()));
        ({
            let ___position: BidirectionalIterator<i32> =
                <BidirectionalIterator<i32>>::from(({ (*l1.borrow()).end_26() }));
            (*l1.borrow()).splice_67(___position, l2.as_pointer())
        });
        assert!(({ (({ (*l1.borrow()).size() }) == 0_u64) }));
        assert!(
            ({
                (({
                    let ___first: BidirectionalIterator<i32> = ({ (*l1.borrow()).begin_24() });
                    let ___last: BidirectionalIterator<i32> = ({ (*l1.borrow()).end_26() });
                    distance_6(___first, ___last)
                }) == 0_isize)
            })
        );
        assert!(({ (({ (*l2.borrow()).size() }) == 0_u64) }));
        assert!(
            ({
                (({
                    let ___first: BidirectionalIterator<i32> = ({ (*l2.borrow()).begin_24() });
                    let ___last: BidirectionalIterator<i32> = ({ (*l2.borrow()).end_26() });
                    distance_6(___first, ___last)
                }) == 0_isize)
            })
        );
    }
    {
        let l1: Value<std_list_int__min_allocator_int__> =
            Rc::new(RefCell::new(<std_list_int__min_allocator_int__>::default()));
        let l2: Value<std_list_int__min_allocator_int__> =
            Rc::new(RefCell::new(std_list_int__min_allocator_int__::new_1(
                { (a2.as_pointer() as Ptr<i32>) },
                { (a2.as_pointer() as Ptr<i32>).offset((1) as isize) },
                None,
            )));
        ({
            let ___position: BidirectionalIterator<i32> =
                <BidirectionalIterator<i32>>::from(({ (*l1.borrow()).end_26() }));
            (*l1.borrow()).splice_67(___position, l2.as_pointer())
        });
        assert!(({ (({ (*l1.borrow()).size() }) == 1_u64) }));
        assert!(
            ({
                (({
                    let ___first: BidirectionalIterator<i32> = ({ (*l1.borrow()).begin_24() });
                    let ___last: BidirectionalIterator<i32> = ({ (*l1.borrow()).end_26() });
                    distance_6(___first, ___last)
                }) == 1_isize)
            })
        );
        assert!(({ (({ (*l2.borrow()).size() }) == 0_u64) }));
        assert!(
            ({
                (({
                    let ___first: BidirectionalIterator<i32> = ({ (*l2.borrow()).begin_24() });
                    let ___last: BidirectionalIterator<i32> = ({ (*l2.borrow()).end_26() });
                    distance_6(___first, ___last)
                }) == 0_isize)
            })
        );
        let i: Value<BidirectionalIterator<i32>> =
            Rc::new(RefCell::new(<BidirectionalIterator<i32>>::from(
                ({ (*l1.borrow()).begin_24() }),
            )));
        assert!(({ (((*i.borrow()).iter_deref().read()) == 4) }));
    }
    {
        let l1: Value<std_list_int__min_allocator_int__> =
            Rc::new(RefCell::new(<std_list_int__min_allocator_int__>::default()));
        let l2: Value<std_list_int__min_allocator_int__> =
            Rc::new(RefCell::new(std_list_int__min_allocator_int__::new_1(
                { (a2.as_pointer() as Ptr<i32>) },
                { (a2.as_pointer() as Ptr<i32>).offset((2) as isize) },
                None,
            )));
        ({
            let ___position: BidirectionalIterator<i32> =
                <BidirectionalIterator<i32>>::from(({ (*l1.borrow()).end_26() }));
            (*l1.borrow()).splice_67(___position, l2.as_pointer())
        });
        assert!(({ (({ (*l1.borrow()).size() }) == 2_u64) }));
        assert!(
            ({
                (({
                    let ___first: BidirectionalIterator<i32> = ({ (*l1.borrow()).begin_24() });
                    let ___last: BidirectionalIterator<i32> = ({ (*l1.borrow()).end_26() });
                    distance_6(___first, ___last)
                }) == 2_isize)
            })
        );
        assert!(({ (({ (*l2.borrow()).size() }) == 0_u64) }));
        assert!(
            ({
                (({
                    let ___first: BidirectionalIterator<i32> = ({ (*l2.borrow()).begin_24() });
                    let ___last: BidirectionalIterator<i32> = ({ (*l2.borrow()).end_26() });
                    distance_6(___first, ___last)
                }) == 0_isize)
            })
        );
        let i: Value<BidirectionalIterator<i32>> =
            Rc::new(RefCell::new(<BidirectionalIterator<i32>>::from(
                ({ (*l1.borrow()).begin_24() }),
            )));
        assert!(({ (((*i.borrow()).iter_deref().read()) == 4) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 5) }));
    }
    {
        let l1: Value<std_list_int__min_allocator_int__> =
            Rc::new(RefCell::new(<std_list_int__min_allocator_int__>::default()));
        let l2: Value<std_list_int__min_allocator_int__> =
            Rc::new(RefCell::new(std_list_int__min_allocator_int__::new_1(
                { (a2.as_pointer() as Ptr<i32>) },
                { (a2.as_pointer() as Ptr<i32>).offset((3) as isize) },
                None,
            )));
        ({
            let ___position: BidirectionalIterator<i32> =
                <BidirectionalIterator<i32>>::from(({ (*l1.borrow()).end_26() }));
            (*l1.borrow()).splice_67(___position, l2.as_pointer())
        });
        assert!(({ (({ (*l1.borrow()).size() }) == 3_u64) }));
        assert!(
            ({
                (({
                    let ___first: BidirectionalIterator<i32> = ({ (*l1.borrow()).begin_24() });
                    let ___last: BidirectionalIterator<i32> = ({ (*l1.borrow()).end_26() });
                    distance_6(___first, ___last)
                }) == 3_isize)
            })
        );
        assert!(({ (({ (*l2.borrow()).size() }) == 0_u64) }));
        assert!(
            ({
                (({
                    let ___first: BidirectionalIterator<i32> = ({ (*l2.borrow()).begin_24() });
                    let ___last: BidirectionalIterator<i32> = ({ (*l2.borrow()).end_26() });
                    distance_6(___first, ___last)
                }) == 0_isize)
            })
        );
        let i: Value<BidirectionalIterator<i32>> =
            Rc::new(RefCell::new(<BidirectionalIterator<i32>>::from(
                ({ (*l1.borrow()).begin_24() }),
            )));
        assert!(({ (((*i.borrow()).iter_deref().read()) == 4) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 5) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 6) }));
    }
    {
        let l1: Value<std_list_int__min_allocator_int__> =
            Rc::new(RefCell::new(std_list_int__min_allocator_int__::new_1(
                { (a1.as_pointer() as Ptr<i32>) },
                { (a1.as_pointer() as Ptr<i32>).offset((1) as isize) },
                None,
            )));
        let l2: Value<std_list_int__min_allocator_int__> =
            Rc::new(RefCell::new(<std_list_int__min_allocator_int__>::default()));
        ({
            let ___position: BidirectionalIterator<i32> =
                <BidirectionalIterator<i32>>::from(({ (*l1.borrow()).begin_24() }));
            (*l1.borrow()).splice_67(___position, l2.as_pointer())
        });
        assert!(({ (({ (*l1.borrow()).size() }) == 1_u64) }));
        assert!(
            ({
                (({
                    let ___first: BidirectionalIterator<i32> = ({ (*l1.borrow()).begin_24() });
                    let ___last: BidirectionalIterator<i32> = ({ (*l1.borrow()).end_26() });
                    distance_6(___first, ___last)
                }) == 1_isize)
            })
        );
        assert!(({ (({ (*l2.borrow()).size() }) == 0_u64) }));
        assert!(
            ({
                (({
                    let ___first: BidirectionalIterator<i32> = ({ (*l2.borrow()).begin_24() });
                    let ___last: BidirectionalIterator<i32> = ({ (*l2.borrow()).end_26() });
                    distance_6(___first, ___last)
                }) == 0_isize)
            })
        );
        let i: Value<BidirectionalIterator<i32>> =
            Rc::new(RefCell::new(<BidirectionalIterator<i32>>::from(
                ({ (*l1.borrow()).begin_24() }),
            )));
        assert!(({ (((*i.borrow()).iter_deref().read()) == 1) }));
    }
    {
        let l1: Value<std_list_int__min_allocator_int__> =
            Rc::new(RefCell::new(std_list_int__min_allocator_int__::new_1(
                { (a1.as_pointer() as Ptr<i32>) },
                { (a1.as_pointer() as Ptr<i32>).offset((1) as isize) },
                None,
            )));
        let l2: Value<std_list_int__min_allocator_int__> =
            Rc::new(RefCell::new(<std_list_int__min_allocator_int__>::default()));
        ({
            let ___position: BidirectionalIterator<i32> =
                <BidirectionalIterator<i32>>::from(({ (*l1.borrow()).end_26() }));
            (*l1.borrow()).splice_67(___position, l2.as_pointer())
        });
        assert!(({ (({ (*l1.borrow()).size() }) == 1_u64) }));
        assert!(
            ({
                (({
                    let ___first: BidirectionalIterator<i32> = ({ (*l1.borrow()).begin_24() });
                    let ___last: BidirectionalIterator<i32> = ({ (*l1.borrow()).end_26() });
                    distance_6(___first, ___last)
                }) == 1_isize)
            })
        );
        assert!(({ (({ (*l2.borrow()).size() }) == 0_u64) }));
        assert!(
            ({
                (({
                    let ___first: BidirectionalIterator<i32> = ({ (*l2.borrow()).begin_24() });
                    let ___last: BidirectionalIterator<i32> = ({ (*l2.borrow()).end_26() });
                    distance_6(___first, ___last)
                }) == 0_isize)
            })
        );
        let i: Value<BidirectionalIterator<i32>> =
            Rc::new(RefCell::new(<BidirectionalIterator<i32>>::from(
                ({ (*l1.borrow()).begin_24() }),
            )));
        assert!(({ (((*i.borrow()).iter_deref().read()) == 1) }));
    }
    {
        let l1: Value<std_list_int__min_allocator_int__> =
            Rc::new(RefCell::new(std_list_int__min_allocator_int__::new_1(
                { (a1.as_pointer() as Ptr<i32>) },
                { (a1.as_pointer() as Ptr<i32>).offset((1) as isize) },
                None,
            )));
        let l2: Value<std_list_int__min_allocator_int__> =
            Rc::new(RefCell::new(std_list_int__min_allocator_int__::new_1(
                { (a2.as_pointer() as Ptr<i32>) },
                { (a2.as_pointer() as Ptr<i32>).offset((1) as isize) },
                None,
            )));
        ({
            let ___position: BidirectionalIterator<i32> =
                <BidirectionalIterator<i32>>::from(({ (*l1.borrow()).begin_24() }));
            (*l1.borrow()).splice_67(___position, l2.as_pointer())
        });
        assert!(({ (({ (*l1.borrow()).size() }) == 2_u64) }));
        assert!(
            ({
                (({
                    let ___first: BidirectionalIterator<i32> = ({ (*l1.borrow()).begin_24() });
                    let ___last: BidirectionalIterator<i32> = ({ (*l1.borrow()).end_26() });
                    distance_6(___first, ___last)
                }) == 2_isize)
            })
        );
        assert!(({ (({ (*l2.borrow()).size() }) == 0_u64) }));
        assert!(
            ({
                (({
                    let ___first: BidirectionalIterator<i32> = ({ (*l2.borrow()).begin_24() });
                    let ___last: BidirectionalIterator<i32> = ({ (*l2.borrow()).end_26() });
                    distance_6(___first, ___last)
                }) == 0_isize)
            })
        );
        let i: Value<BidirectionalIterator<i32>> =
            Rc::new(RefCell::new(<BidirectionalIterator<i32>>::from(
                ({ (*l1.borrow()).begin_24() }),
            )));
        assert!(({ (((*i.borrow()).iter_deref().read()) == 4) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 1) }));
    }
    {
        let l1: Value<std_list_int__min_allocator_int__> =
            Rc::new(RefCell::new(std_list_int__min_allocator_int__::new_1(
                { (a1.as_pointer() as Ptr<i32>) },
                { (a1.as_pointer() as Ptr<i32>).offset((1) as isize) },
                None,
            )));
        let l2: Value<std_list_int__min_allocator_int__> =
            Rc::new(RefCell::new(std_list_int__min_allocator_int__::new_1(
                { (a2.as_pointer() as Ptr<i32>) },
                { (a2.as_pointer() as Ptr<i32>).offset((1) as isize) },
                None,
            )));
        ({
            let ___position: BidirectionalIterator<i32> =
                <BidirectionalIterator<i32>>::from(({ (*l1.borrow()).end_26() }));
            (*l1.borrow()).splice_67(___position, l2.as_pointer())
        });
        assert!(({ (({ (*l1.borrow()).size() }) == 2_u64) }));
        assert!(
            ({
                (({
                    let ___first: BidirectionalIterator<i32> = ({ (*l1.borrow()).begin_24() });
                    let ___last: BidirectionalIterator<i32> = ({ (*l1.borrow()).end_26() });
                    distance_6(___first, ___last)
                }) == 2_isize)
            })
        );
        assert!(({ (({ (*l2.borrow()).size() }) == 0_u64) }));
        assert!(
            ({
                (({
                    let ___first: BidirectionalIterator<i32> = ({ (*l2.borrow()).begin_24() });
                    let ___last: BidirectionalIterator<i32> = ({ (*l2.borrow()).end_26() });
                    distance_6(___first, ___last)
                }) == 0_isize)
            })
        );
        let i: Value<BidirectionalIterator<i32>> =
            Rc::new(RefCell::new(<BidirectionalIterator<i32>>::from(
                ({ (*l1.borrow()).begin_24() }),
            )));
        assert!(({ (((*i.borrow()).iter_deref().read()) == 1) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 4) }));
    }
    {
        let l1: Value<std_list_int__min_allocator_int__> =
            Rc::new(RefCell::new(std_list_int__min_allocator_int__::new_1(
                { (a1.as_pointer() as Ptr<i32>) },
                { (a1.as_pointer() as Ptr<i32>).offset((1) as isize) },
                None,
            )));
        let l2: Value<std_list_int__min_allocator_int__> =
            Rc::new(RefCell::new(std_list_int__min_allocator_int__::new_1(
                { (a2.as_pointer() as Ptr<i32>) },
                { (a2.as_pointer() as Ptr<i32>).offset((2) as isize) },
                None,
            )));
        ({
            let ___position: BidirectionalIterator<i32> =
                <BidirectionalIterator<i32>>::from(({ (*l1.borrow()).begin_24() }));
            (*l1.borrow()).splice_67(___position, l2.as_pointer())
        });
        assert!(({ (({ (*l1.borrow()).size() }) == 3_u64) }));
        assert!(
            ({
                (({
                    let ___first: BidirectionalIterator<i32> = ({ (*l1.borrow()).begin_24() });
                    let ___last: BidirectionalIterator<i32> = ({ (*l1.borrow()).end_26() });
                    distance_6(___first, ___last)
                }) == 3_isize)
            })
        );
        assert!(({ (({ (*l2.borrow()).size() }) == 0_u64) }));
        assert!(
            ({
                (({
                    let ___first: BidirectionalIterator<i32> = ({ (*l2.borrow()).begin_24() });
                    let ___last: BidirectionalIterator<i32> = ({ (*l2.borrow()).end_26() });
                    distance_6(___first, ___last)
                }) == 0_isize)
            })
        );
        let i: Value<BidirectionalIterator<i32>> =
            Rc::new(RefCell::new(<BidirectionalIterator<i32>>::from(
                ({ (*l1.borrow()).begin_24() }),
            )));
        assert!(({ (((*i.borrow()).iter_deref().read()) == 4) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 5) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 1) }));
    }
    {
        let l1: Value<std_list_int__min_allocator_int__> =
            Rc::new(RefCell::new(std_list_int__min_allocator_int__::new_1(
                { (a1.as_pointer() as Ptr<i32>) },
                { (a1.as_pointer() as Ptr<i32>).offset((1) as isize) },
                None,
            )));
        let l2: Value<std_list_int__min_allocator_int__> =
            Rc::new(RefCell::new(std_list_int__min_allocator_int__::new_1(
                { (a2.as_pointer() as Ptr<i32>) },
                { (a2.as_pointer() as Ptr<i32>).offset((2) as isize) },
                None,
            )));
        ({
            let ___position: BidirectionalIterator<i32> =
                <BidirectionalIterator<i32>>::from(({ (*l1.borrow()).end_26() }));
            (*l1.borrow()).splice_67(___position, l2.as_pointer())
        });
        assert!(({ (({ (*l1.borrow()).size() }) == 3_u64) }));
        assert!(
            ({
                (({
                    let ___first: BidirectionalIterator<i32> = ({ (*l1.borrow()).begin_24() });
                    let ___last: BidirectionalIterator<i32> = ({ (*l1.borrow()).end_26() });
                    distance_6(___first, ___last)
                }) == 3_isize)
            })
        );
        assert!(({ (({ (*l2.borrow()).size() }) == 0_u64) }));
        assert!(
            ({
                (({
                    let ___first: BidirectionalIterator<i32> = ({ (*l2.borrow()).begin_24() });
                    let ___last: BidirectionalIterator<i32> = ({ (*l2.borrow()).end_26() });
                    distance_6(___first, ___last)
                }) == 0_isize)
            })
        );
        let i: Value<BidirectionalIterator<i32>> =
            Rc::new(RefCell::new(<BidirectionalIterator<i32>>::from(
                ({ (*l1.borrow()).begin_24() }),
            )));
        assert!(({ (((*i.borrow()).iter_deref().read()) == 1) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 4) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 5) }));
    }
    {
        let l1: Value<std_list_int__min_allocator_int__> =
            Rc::new(RefCell::new(std_list_int__min_allocator_int__::new_1(
                { (a1.as_pointer() as Ptr<i32>) },
                { (a1.as_pointer() as Ptr<i32>).offset((1) as isize) },
                None,
            )));
        let l2: Value<std_list_int__min_allocator_int__> =
            Rc::new(RefCell::new(std_list_int__min_allocator_int__::new_1(
                { (a2.as_pointer() as Ptr<i32>) },
                { (a2.as_pointer() as Ptr<i32>).offset((3) as isize) },
                None,
            )));
        ({
            let ___position: BidirectionalIterator<i32> =
                <BidirectionalIterator<i32>>::from(({ (*l1.borrow()).begin_24() }));
            (*l1.borrow()).splice_67(___position, l2.as_pointer())
        });
        assert!(({ (({ (*l1.borrow()).size() }) == 4_u64) }));
        assert!(
            ({
                (({
                    let ___first: BidirectionalIterator<i32> = ({ (*l1.borrow()).begin_24() });
                    let ___last: BidirectionalIterator<i32> = ({ (*l1.borrow()).end_26() });
                    distance_6(___first, ___last)
                }) == 4_isize)
            })
        );
        assert!(({ (({ (*l2.borrow()).size() }) == 0_u64) }));
        assert!(
            ({
                (({
                    let ___first: BidirectionalIterator<i32> = ({ (*l2.borrow()).begin_24() });
                    let ___last: BidirectionalIterator<i32> = ({ (*l2.borrow()).end_26() });
                    distance_6(___first, ___last)
                }) == 0_isize)
            })
        );
        let i: Value<BidirectionalIterator<i32>> =
            Rc::new(RefCell::new(<BidirectionalIterator<i32>>::from(
                ({ (*l1.borrow()).begin_24() }),
            )));
        assert!(({ (((*i.borrow()).iter_deref().read()) == 4) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 5) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 6) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 1) }));
    }
    {
        let l1: Value<std_list_int__min_allocator_int__> =
            Rc::new(RefCell::new(std_list_int__min_allocator_int__::new_1(
                { (a1.as_pointer() as Ptr<i32>) },
                { (a1.as_pointer() as Ptr<i32>).offset((1) as isize) },
                None,
            )));
        let l2: Value<std_list_int__min_allocator_int__> =
            Rc::new(RefCell::new(std_list_int__min_allocator_int__::new_1(
                { (a2.as_pointer() as Ptr<i32>) },
                { (a2.as_pointer() as Ptr<i32>).offset((3) as isize) },
                None,
            )));
        ({
            let ___position: BidirectionalIterator<i32> =
                <BidirectionalIterator<i32>>::from(({ (*l1.borrow()).end_26() }));
            (*l1.borrow()).splice_67(___position, l2.as_pointer())
        });
        assert!(({ (({ (*l1.borrow()).size() }) == 4_u64) }));
        assert!(
            ({
                (({
                    let ___first: BidirectionalIterator<i32> = ({ (*l1.borrow()).begin_24() });
                    let ___last: BidirectionalIterator<i32> = ({ (*l1.borrow()).end_26() });
                    distance_6(___first, ___last)
                }) == 4_isize)
            })
        );
        assert!(({ (({ (*l2.borrow()).size() }) == 0_u64) }));
        assert!(
            ({
                (({
                    let ___first: BidirectionalIterator<i32> = ({ (*l2.borrow()).begin_24() });
                    let ___last: BidirectionalIterator<i32> = ({ (*l2.borrow()).end_26() });
                    distance_6(___first, ___last)
                }) == 0_isize)
            })
        );
        let i: Value<BidirectionalIterator<i32>> =
            Rc::new(RefCell::new(<BidirectionalIterator<i32>>::from(
                ({ (*l1.borrow()).begin_24() }),
            )));
        assert!(({ (((*i.borrow()).iter_deref().read()) == 1) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 4) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 5) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 6) }));
    }
    {
        let l1: Value<std_list_int__min_allocator_int__> =
            Rc::new(RefCell::new(std_list_int__min_allocator_int__::new_1(
                { (a1.as_pointer() as Ptr<i32>) },
                { (a1.as_pointer() as Ptr<i32>).offset((2) as isize) },
                None,
            )));
        let l2: Value<std_list_int__min_allocator_int__> =
            Rc::new(RefCell::new(<std_list_int__min_allocator_int__>::default()));
        ({
            let ___position: BidirectionalIterator<i32> =
                <BidirectionalIterator<i32>>::from(({ (*l1.borrow()).begin_24() }));
            (*l1.borrow()).splice_67(___position, l2.as_pointer())
        });
        assert!(({ (({ (*l1.borrow()).size() }) == 2_u64) }));
        assert!(
            ({
                (({
                    let ___first: BidirectionalIterator<i32> = ({ (*l1.borrow()).begin_24() });
                    let ___last: BidirectionalIterator<i32> = ({ (*l1.borrow()).end_26() });
                    distance_6(___first, ___last)
                }) == 2_isize)
            })
        );
        assert!(({ (({ (*l2.borrow()).size() }) == 0_u64) }));
        assert!(
            ({
                (({
                    let ___first: BidirectionalIterator<i32> = ({ (*l2.borrow()).begin_24() });
                    let ___last: BidirectionalIterator<i32> = ({ (*l2.borrow()).end_26() });
                    distance_6(___first, ___last)
                }) == 0_isize)
            })
        );
        let i: Value<BidirectionalIterator<i32>> =
            Rc::new(RefCell::new(<BidirectionalIterator<i32>>::from(
                ({ (*l1.borrow()).begin_24() }),
            )));
        assert!(({ (((*i.borrow()).iter_deref().read()) == 1) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 2) }));
    }
    {
        let l1: Value<std_list_int__min_allocator_int__> =
            Rc::new(RefCell::new(std_list_int__min_allocator_int__::new_1(
                { (a1.as_pointer() as Ptr<i32>) },
                { (a1.as_pointer() as Ptr<i32>).offset((2) as isize) },
                None,
            )));
        let l2: Value<std_list_int__min_allocator_int__> =
            Rc::new(RefCell::new(<std_list_int__min_allocator_int__>::default()));
        ({
            let ___position: BidirectionalIterator<i32> = <BidirectionalIterator<i32>>::from(
                libcc2rs::next_refcount(({ ({ (*l1.borrow()).begin_24() }) }), 1_isize),
            );
            (*l1.borrow()).splice_67(___position, l2.as_pointer())
        });
        assert!(({ (({ (*l1.borrow()).size() }) == 2_u64) }));
        assert!(
            ({
                (({
                    let ___first: BidirectionalIterator<i32> = ({ (*l1.borrow()).begin_24() });
                    let ___last: BidirectionalIterator<i32> = ({ (*l1.borrow()).end_26() });
                    distance_6(___first, ___last)
                }) == 2_isize)
            })
        );
        assert!(({ (({ (*l2.borrow()).size() }) == 0_u64) }));
        assert!(
            ({
                (({
                    let ___first: BidirectionalIterator<i32> = ({ (*l2.borrow()).begin_24() });
                    let ___last: BidirectionalIterator<i32> = ({ (*l2.borrow()).end_26() });
                    distance_6(___first, ___last)
                }) == 0_isize)
            })
        );
        let i: Value<BidirectionalIterator<i32>> =
            Rc::new(RefCell::new(<BidirectionalIterator<i32>>::from(
                ({ (*l1.borrow()).begin_24() }),
            )));
        assert!(({ (((*i.borrow()).iter_deref().read()) == 1) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 2) }));
    }
    {
        let l1: Value<std_list_int__min_allocator_int__> =
            Rc::new(RefCell::new(std_list_int__min_allocator_int__::new_1(
                { (a1.as_pointer() as Ptr<i32>) },
                { (a1.as_pointer() as Ptr<i32>).offset((2) as isize) },
                None,
            )));
        let l2: Value<std_list_int__min_allocator_int__> =
            Rc::new(RefCell::new(<std_list_int__min_allocator_int__>::default()));
        ({
            let ___position: BidirectionalIterator<i32> = <BidirectionalIterator<i32>>::from(
                libcc2rs::next_refcount(({ ({ (*l1.borrow()).begin_24() }) }), 2_isize),
            );
            (*l1.borrow()).splice_67(___position, l2.as_pointer())
        });
        assert!(({ (({ (*l1.borrow()).size() }) == 2_u64) }));
        assert!(
            ({
                (({
                    let ___first: BidirectionalIterator<i32> = ({ (*l1.borrow()).begin_24() });
                    let ___last: BidirectionalIterator<i32> = ({ (*l1.borrow()).end_26() });
                    distance_6(___first, ___last)
                }) == 2_isize)
            })
        );
        assert!(({ (({ (*l2.borrow()).size() }) == 0_u64) }));
        assert!(
            ({
                (({
                    let ___first: BidirectionalIterator<i32> = ({ (*l2.borrow()).begin_24() });
                    let ___last: BidirectionalIterator<i32> = ({ (*l2.borrow()).end_26() });
                    distance_6(___first, ___last)
                }) == 0_isize)
            })
        );
        let i: Value<BidirectionalIterator<i32>> =
            Rc::new(RefCell::new(<BidirectionalIterator<i32>>::from(
                ({ (*l1.borrow()).begin_24() }),
            )));
        assert!(({ (((*i.borrow()).iter_deref().read()) == 1) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 2) }));
    }
    {
        let l1: Value<std_list_int__min_allocator_int__> =
            Rc::new(RefCell::new(std_list_int__min_allocator_int__::new_1(
                { (a1.as_pointer() as Ptr<i32>) },
                { (a1.as_pointer() as Ptr<i32>).offset((2) as isize) },
                None,
            )));
        let l2: Value<std_list_int__min_allocator_int__> =
            Rc::new(RefCell::new(std_list_int__min_allocator_int__::new_1(
                { (a2.as_pointer() as Ptr<i32>) },
                { (a2.as_pointer() as Ptr<i32>).offset((1) as isize) },
                None,
            )));
        ({
            let ___position: BidirectionalIterator<i32> =
                <BidirectionalIterator<i32>>::from(({ (*l1.borrow()).begin_24() }));
            (*l1.borrow()).splice_67(___position, l2.as_pointer())
        });
        assert!(({ (({ (*l1.borrow()).size() }) == 3_u64) }));
        assert!(
            ({
                (({
                    let ___first: BidirectionalIterator<i32> = ({ (*l1.borrow()).begin_24() });
                    let ___last: BidirectionalIterator<i32> = ({ (*l1.borrow()).end_26() });
                    distance_6(___first, ___last)
                }) == 3_isize)
            })
        );
        assert!(({ (({ (*l2.borrow()).size() }) == 0_u64) }));
        assert!(
            ({
                (({
                    let ___first: BidirectionalIterator<i32> = ({ (*l2.borrow()).begin_24() });
                    let ___last: BidirectionalIterator<i32> = ({ (*l2.borrow()).end_26() });
                    distance_6(___first, ___last)
                }) == 0_isize)
            })
        );
        let i: Value<BidirectionalIterator<i32>> =
            Rc::new(RefCell::new(<BidirectionalIterator<i32>>::from(
                ({ (*l1.borrow()).begin_24() }),
            )));
        assert!(({ (((*i.borrow()).iter_deref().read()) == 4) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 1) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 2) }));
    }
    {
        let l1: Value<std_list_int__min_allocator_int__> =
            Rc::new(RefCell::new(std_list_int__min_allocator_int__::new_1(
                { (a1.as_pointer() as Ptr<i32>) },
                { (a1.as_pointer() as Ptr<i32>).offset((2) as isize) },
                None,
            )));
        let l2: Value<std_list_int__min_allocator_int__> =
            Rc::new(RefCell::new(std_list_int__min_allocator_int__::new_1(
                { (a2.as_pointer() as Ptr<i32>) },
                { (a2.as_pointer() as Ptr<i32>).offset((1) as isize) },
                None,
            )));
        ({
            let ___position: BidirectionalIterator<i32> = <BidirectionalIterator<i32>>::from(
                libcc2rs::next_refcount(({ ({ (*l1.borrow()).begin_24() }) }), 1_isize),
            );
            (*l1.borrow()).splice_67(___position, l2.as_pointer())
        });
        assert!(({ (({ (*l1.borrow()).size() }) == 3_u64) }));
        assert!(
            ({
                (({
                    let ___first: BidirectionalIterator<i32> = ({ (*l1.borrow()).begin_24() });
                    let ___last: BidirectionalIterator<i32> = ({ (*l1.borrow()).end_26() });
                    distance_6(___first, ___last)
                }) == 3_isize)
            })
        );
        assert!(({ (({ (*l2.borrow()).size() }) == 0_u64) }));
        assert!(
            ({
                (({
                    let ___first: BidirectionalIterator<i32> = ({ (*l2.borrow()).begin_24() });
                    let ___last: BidirectionalIterator<i32> = ({ (*l2.borrow()).end_26() });
                    distance_6(___first, ___last)
                }) == 0_isize)
            })
        );
        let i: Value<BidirectionalIterator<i32>> =
            Rc::new(RefCell::new(<BidirectionalIterator<i32>>::from(
                ({ (*l1.borrow()).begin_24() }),
            )));
        assert!(({ (((*i.borrow()).iter_deref().read()) == 1) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 4) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 2) }));
    }
    {
        let l1: Value<std_list_int__min_allocator_int__> =
            Rc::new(RefCell::new(std_list_int__min_allocator_int__::new_1(
                { (a1.as_pointer() as Ptr<i32>) },
                { (a1.as_pointer() as Ptr<i32>).offset((2) as isize) },
                None,
            )));
        let l2: Value<std_list_int__min_allocator_int__> =
            Rc::new(RefCell::new(std_list_int__min_allocator_int__::new_1(
                { (a2.as_pointer() as Ptr<i32>) },
                { (a2.as_pointer() as Ptr<i32>).offset((1) as isize) },
                None,
            )));
        ({
            let ___position: BidirectionalIterator<i32> = <BidirectionalIterator<i32>>::from(
                libcc2rs::next_refcount(({ ({ (*l1.borrow()).begin_24() }) }), 2_isize),
            );
            (*l1.borrow()).splice_67(___position, l2.as_pointer())
        });
        assert!(({ (({ (*l1.borrow()).size() }) == 3_u64) }));
        assert!(
            ({
                (({
                    let ___first: BidirectionalIterator<i32> = ({ (*l1.borrow()).begin_24() });
                    let ___last: BidirectionalIterator<i32> = ({ (*l1.borrow()).end_26() });
                    distance_6(___first, ___last)
                }) == 3_isize)
            })
        );
        assert!(({ (({ (*l2.borrow()).size() }) == 0_u64) }));
        assert!(
            ({
                (({
                    let ___first: BidirectionalIterator<i32> = ({ (*l2.borrow()).begin_24() });
                    let ___last: BidirectionalIterator<i32> = ({ (*l2.borrow()).end_26() });
                    distance_6(___first, ___last)
                }) == 0_isize)
            })
        );
        let i: Value<BidirectionalIterator<i32>> =
            Rc::new(RefCell::new(<BidirectionalIterator<i32>>::from(
                ({ (*l1.borrow()).begin_24() }),
            )));
        assert!(({ (((*i.borrow()).iter_deref().read()) == 1) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 2) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 4) }));
    }
    {
        let l1: Value<std_list_int__min_allocator_int__> =
            Rc::new(RefCell::new(std_list_int__min_allocator_int__::new_1(
                { (a1.as_pointer() as Ptr<i32>) },
                { (a1.as_pointer() as Ptr<i32>).offset((2) as isize) },
                None,
            )));
        let l2: Value<std_list_int__min_allocator_int__> =
            Rc::new(RefCell::new(std_list_int__min_allocator_int__::new_1(
                { (a2.as_pointer() as Ptr<i32>) },
                { (a2.as_pointer() as Ptr<i32>).offset((2) as isize) },
                None,
            )));
        ({
            let ___position: BidirectionalIterator<i32> =
                <BidirectionalIterator<i32>>::from(({ (*l1.borrow()).begin_24() }));
            (*l1.borrow()).splice_67(___position, l2.as_pointer())
        });
        assert!(({ (({ (*l1.borrow()).size() }) == 4_u64) }));
        assert!(
            ({
                (({
                    let ___first: BidirectionalIterator<i32> = ({ (*l1.borrow()).begin_24() });
                    let ___last: BidirectionalIterator<i32> = ({ (*l1.borrow()).end_26() });
                    distance_6(___first, ___last)
                }) == 4_isize)
            })
        );
        assert!(({ (({ (*l2.borrow()).size() }) == 0_u64) }));
        assert!(
            ({
                (({
                    let ___first: BidirectionalIterator<i32> = ({ (*l2.borrow()).begin_24() });
                    let ___last: BidirectionalIterator<i32> = ({ (*l2.borrow()).end_26() });
                    distance_6(___first, ___last)
                }) == 0_isize)
            })
        );
        let i: Value<BidirectionalIterator<i32>> =
            Rc::new(RefCell::new(<BidirectionalIterator<i32>>::from(
                ({ (*l1.borrow()).begin_24() }),
            )));
        assert!(({ (((*i.borrow()).iter_deref().read()) == 4) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 5) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 1) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 2) }));
    }
    {
        let l1: Value<std_list_int__min_allocator_int__> =
            Rc::new(RefCell::new(std_list_int__min_allocator_int__::new_1(
                { (a1.as_pointer() as Ptr<i32>) },
                { (a1.as_pointer() as Ptr<i32>).offset((2) as isize) },
                None,
            )));
        let l2: Value<std_list_int__min_allocator_int__> =
            Rc::new(RefCell::new(std_list_int__min_allocator_int__::new_1(
                { (a2.as_pointer() as Ptr<i32>) },
                { (a2.as_pointer() as Ptr<i32>).offset((2) as isize) },
                None,
            )));
        ({
            let ___position: BidirectionalIterator<i32> = <BidirectionalIterator<i32>>::from(
                libcc2rs::next_refcount(({ ({ (*l1.borrow()).begin_24() }) }), 1_isize),
            );
            (*l1.borrow()).splice_67(___position, l2.as_pointer())
        });
        assert!(({ (({ (*l1.borrow()).size() }) == 4_u64) }));
        assert!(
            ({
                (({
                    let ___first: BidirectionalIterator<i32> = ({ (*l1.borrow()).begin_24() });
                    let ___last: BidirectionalIterator<i32> = ({ (*l1.borrow()).end_26() });
                    distance_6(___first, ___last)
                }) == 4_isize)
            })
        );
        assert!(({ (({ (*l2.borrow()).size() }) == 0_u64) }));
        assert!(
            ({
                (({
                    let ___first: BidirectionalIterator<i32> = ({ (*l2.borrow()).begin_24() });
                    let ___last: BidirectionalIterator<i32> = ({ (*l2.borrow()).end_26() });
                    distance_6(___first, ___last)
                }) == 0_isize)
            })
        );
        let i: Value<BidirectionalIterator<i32>> =
            Rc::new(RefCell::new(<BidirectionalIterator<i32>>::from(
                ({ (*l1.borrow()).begin_24() }),
            )));
        assert!(({ (((*i.borrow()).iter_deref().read()) == 1) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 4) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 5) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 2) }));
    }
    {
        let l1: Value<std_list_int__min_allocator_int__> =
            Rc::new(RefCell::new(std_list_int__min_allocator_int__::new_1(
                { (a1.as_pointer() as Ptr<i32>) },
                { (a1.as_pointer() as Ptr<i32>).offset((2) as isize) },
                None,
            )));
        let l2: Value<std_list_int__min_allocator_int__> =
            Rc::new(RefCell::new(std_list_int__min_allocator_int__::new_1(
                { (a2.as_pointer() as Ptr<i32>) },
                { (a2.as_pointer() as Ptr<i32>).offset((2) as isize) },
                None,
            )));
        ({
            let ___position: BidirectionalIterator<i32> = <BidirectionalIterator<i32>>::from(
                libcc2rs::next_refcount(({ ({ (*l1.borrow()).begin_24() }) }), 2_isize),
            );
            (*l1.borrow()).splice_67(___position, l2.as_pointer())
        });
        assert!(({ (({ (*l1.borrow()).size() }) == 4_u64) }));
        assert!(
            ({
                (({
                    let ___first: BidirectionalIterator<i32> = ({ (*l1.borrow()).begin_24() });
                    let ___last: BidirectionalIterator<i32> = ({ (*l1.borrow()).end_26() });
                    distance_6(___first, ___last)
                }) == 4_isize)
            })
        );
        assert!(({ (({ (*l2.borrow()).size() }) == 0_u64) }));
        assert!(
            ({
                (({
                    let ___first: BidirectionalIterator<i32> = ({ (*l2.borrow()).begin_24() });
                    let ___last: BidirectionalIterator<i32> = ({ (*l2.borrow()).end_26() });
                    distance_6(___first, ___last)
                }) == 0_isize)
            })
        );
        let i: Value<BidirectionalIterator<i32>> =
            Rc::new(RefCell::new(<BidirectionalIterator<i32>>::from(
                ({ (*l1.borrow()).begin_24() }),
            )));
        assert!(({ (((*i.borrow()).iter_deref().read()) == 1) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 2) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 4) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 5) }));
    }
    {
        let l1: Value<std_list_int__min_allocator_int__> =
            Rc::new(RefCell::new(std_list_int__min_allocator_int__::new_1(
                { (a1.as_pointer() as Ptr<i32>) },
                { (a1.as_pointer() as Ptr<i32>).offset((3) as isize) },
                None,
            )));
        let l2: Value<std_list_int__min_allocator_int__> =
            Rc::new(RefCell::new(std_list_int__min_allocator_int__::new_1(
                { (a2.as_pointer() as Ptr<i32>) },
                { (a2.as_pointer() as Ptr<i32>).offset((3) as isize) },
                None,
            )));
        ({
            let ___position: BidirectionalIterator<i32> =
                <BidirectionalIterator<i32>>::from(({ (*l1.borrow()).begin_24() }));
            (*l1.borrow()).splice_67(___position, l2.as_pointer())
        });
        assert!(({ (({ (*l1.borrow()).size() }) == 6_u64) }));
        assert!(
            ({
                (({
                    let ___first: BidirectionalIterator<i32> = ({ (*l1.borrow()).begin_24() });
                    let ___last: BidirectionalIterator<i32> = ({ (*l1.borrow()).end_26() });
                    distance_6(___first, ___last)
                }) == 6_isize)
            })
        );
        assert!(({ (({ (*l2.borrow()).size() }) == 0_u64) }));
        assert!(
            ({
                (({
                    let ___first: BidirectionalIterator<i32> = ({ (*l2.borrow()).begin_24() });
                    let ___last: BidirectionalIterator<i32> = ({ (*l2.borrow()).end_26() });
                    distance_6(___first, ___last)
                }) == 0_isize)
            })
        );
        let i: Value<BidirectionalIterator<i32>> =
            Rc::new(RefCell::new(<BidirectionalIterator<i32>>::from(
                ({ (*l1.borrow()).begin_24() }),
            )));
        assert!(({ (((*i.borrow()).iter_deref().read()) == 4) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 5) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 6) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 1) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 2) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 3) }));
    }
    {
        let l1: Value<std_list_int__min_allocator_int__> =
            Rc::new(RefCell::new(std_list_int__min_allocator_int__::new_1(
                { (a1.as_pointer() as Ptr<i32>) },
                { (a1.as_pointer() as Ptr<i32>).offset((3) as isize) },
                None,
            )));
        let l2: Value<std_list_int__min_allocator_int__> =
            Rc::new(RefCell::new(std_list_int__min_allocator_int__::new_1(
                { (a2.as_pointer() as Ptr<i32>) },
                { (a2.as_pointer() as Ptr<i32>).offset((3) as isize) },
                None,
            )));
        ({
            let ___position: BidirectionalIterator<i32> = <BidirectionalIterator<i32>>::from(
                libcc2rs::next_refcount(({ ({ (*l1.borrow()).begin_24() }) }), 1_isize),
            );
            (*l1.borrow()).splice_67(___position, l2.as_pointer())
        });
        assert!(({ (({ (*l1.borrow()).size() }) == 6_u64) }));
        assert!(
            ({
                (({
                    let ___first: BidirectionalIterator<i32> = ({ (*l1.borrow()).begin_24() });
                    let ___last: BidirectionalIterator<i32> = ({ (*l1.borrow()).end_26() });
                    distance_6(___first, ___last)
                }) == 6_isize)
            })
        );
        assert!(({ (({ (*l2.borrow()).size() }) == 0_u64) }));
        assert!(
            ({
                (({
                    let ___first: BidirectionalIterator<i32> = ({ (*l2.borrow()).begin_24() });
                    let ___last: BidirectionalIterator<i32> = ({ (*l2.borrow()).end_26() });
                    distance_6(___first, ___last)
                }) == 0_isize)
            })
        );
        let i: Value<BidirectionalIterator<i32>> =
            Rc::new(RefCell::new(<BidirectionalIterator<i32>>::from(
                ({ (*l1.borrow()).begin_24() }),
            )));
        assert!(({ (((*i.borrow()).iter_deref().read()) == 1) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 4) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 5) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 6) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 2) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 3) }));
    }
    {
        let l1: Value<std_list_int__min_allocator_int__> =
            Rc::new(RefCell::new(std_list_int__min_allocator_int__::new_1(
                { (a1.as_pointer() as Ptr<i32>) },
                { (a1.as_pointer() as Ptr<i32>).offset((3) as isize) },
                None,
            )));
        let l2: Value<std_list_int__min_allocator_int__> =
            Rc::new(RefCell::new(std_list_int__min_allocator_int__::new_1(
                { (a2.as_pointer() as Ptr<i32>) },
                { (a2.as_pointer() as Ptr<i32>).offset((3) as isize) },
                None,
            )));
        ({
            let ___position: BidirectionalIterator<i32> = <BidirectionalIterator<i32>>::from(
                libcc2rs::next_refcount(({ ({ (*l1.borrow()).begin_24() }) }), 2_isize),
            );
            (*l1.borrow()).splice_67(___position, l2.as_pointer())
        });
        assert!(({ (({ (*l1.borrow()).size() }) == 6_u64) }));
        assert!(
            ({
                (({
                    let ___first: BidirectionalIterator<i32> = ({ (*l1.borrow()).begin_24() });
                    let ___last: BidirectionalIterator<i32> = ({ (*l1.borrow()).end_26() });
                    distance_6(___first, ___last)
                }) == 6_isize)
            })
        );
        assert!(({ (({ (*l2.borrow()).size() }) == 0_u64) }));
        assert!(
            ({
                (({
                    let ___first: BidirectionalIterator<i32> = ({ (*l2.borrow()).begin_24() });
                    let ___last: BidirectionalIterator<i32> = ({ (*l2.borrow()).end_26() });
                    distance_6(___first, ___last)
                }) == 0_isize)
            })
        );
        let i: Value<BidirectionalIterator<i32>> =
            Rc::new(RefCell::new(<BidirectionalIterator<i32>>::from(
                ({ (*l1.borrow()).begin_24() }),
            )));
        assert!(({ (((*i.borrow()).iter_deref().read()) == 1) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 2) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 4) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 5) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 6) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 3) }));
    }
    {
        let l1: Value<std_list_int__min_allocator_int__> =
            Rc::new(RefCell::new(std_list_int__min_allocator_int__::new_1(
                { (a1.as_pointer() as Ptr<i32>) },
                { (a1.as_pointer() as Ptr<i32>).offset((3) as isize) },
                None,
            )));
        let l2: Value<std_list_int__min_allocator_int__> =
            Rc::new(RefCell::new(std_list_int__min_allocator_int__::new_1(
                { (a2.as_pointer() as Ptr<i32>) },
                { (a2.as_pointer() as Ptr<i32>).offset((3) as isize) },
                None,
            )));
        ({
            let ___position: BidirectionalIterator<i32> = <BidirectionalIterator<i32>>::from(
                libcc2rs::next_refcount(({ ({ (*l1.borrow()).begin_24() }) }), 3_isize),
            );
            (*l1.borrow()).splice_67(___position, l2.as_pointer())
        });
        assert!(({ (({ (*l1.borrow()).size() }) == 6_u64) }));
        assert!(
            ({
                (({
                    let ___first: BidirectionalIterator<i32> = ({ (*l1.borrow()).begin_24() });
                    let ___last: BidirectionalIterator<i32> = ({ (*l1.borrow()).end_26() });
                    distance_6(___first, ___last)
                }) == 6_isize)
            })
        );
        assert!(({ (({ (*l2.borrow()).size() }) == 0_u64) }));
        assert!(
            ({
                (({
                    let ___first: BidirectionalIterator<i32> = ({ (*l2.borrow()).begin_24() });
                    let ___last: BidirectionalIterator<i32> = ({ (*l2.borrow()).end_26() });
                    distance_6(___first, ___last)
                }) == 0_isize)
            })
        );
        let i: Value<BidirectionalIterator<i32>> =
            Rc::new(RefCell::new(<BidirectionalIterator<i32>>::from(
                ({ (*l1.borrow()).begin_24() }),
            )));
        assert!(({ (((*i.borrow()).iter_deref().read()) == 1) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 2) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 3) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 4) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 5) }));
        {
            i.as_pointer().with_mut(|__it| {
                __it.iter_inc();
            });
            i.as_pointer()
        };
        assert!(({ (((*i.borrow()).iter_deref().read()) == 6) }));
    }
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
    assert!(({ test_4() }));
    return 0;
}
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct min_pointer_int__std_integral_constant_unsigned_long__0__;
pub trait min_allocator_int_Impl {
    fn allocate(&self, mut n: usize) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn deallocate(
        &self,
        mut p: min_pointer_int__std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        unimplemented!()
    }
}
impl min_allocator_int_Impl for Ptr<min_allocator_int_> {}
pub fn __cpp2rust_init_globals() {
    let _ = outstanding_bytes_0.with(|_| ());
    let _ = alloc_count_1.with(|_| ());
    let _ = dealloc_count_2.with(|_| ());
    let _ = disable_default_constructor_3.with(|_| ());
}
