extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_0() -> bool {
    {
        const _: () = assert!(
            true,
            "std::three_way_comparable<std::ranges::iterator_t<R>>"
        );;
        let r: Value<std_ranges_repeat_view_int__std_unreachable_sentinel_t_> =
            Rc::new(RefCell::new({
                let __tmp_0: Value<i32> = Rc::new(RefCell::new(42));
                std_ranges_repeat_view_int__std_unreachable_sentinel_t_::new_1(
                    { __tmp_0.as_pointer() },
                    None,
                )
            }));
        let iter1: Value<std_ranges_repeat_view_int___Iterator> =
            Rc::new(RefCell::new(({ (*r.borrow()).begin() })));
        let iter2: Value<std_ranges_repeat_view_int___Iterator> = Rc::new(RefCell::new(
            ({ (*iter1.borrow()).clone() }).iter_add(1_isize),
        ));
        const _: () = assert!(true, "std::same_as<decltype(iter1 == iter2), bool>");;
        assert!(({ (*iter1.borrow()) == (*iter1.borrow()) }));
        assert!(({ !((*iter1.borrow()) == (*iter2.borrow())) }));
        assert!(({ (*iter2.borrow()) == (*iter2.borrow()) }));
        assert!(({ !((*iter1.borrow()) < (*iter1.borrow())) }));
        assert!((*iter1.borrow()) < (*iter2.borrow()));
        assert!(({ !((*iter2.borrow()) < (*iter1.borrow())) }));
        assert!((*iter1.borrow()) <= (*iter1.borrow()));
        assert!((*iter1.borrow()) <= (*iter2.borrow()));
        assert!(({ !((*iter2.borrow()) <= (*iter1.borrow())) }));
        assert!(({ !((*iter1.borrow()) > (*iter1.borrow())) }));
        assert!(({ !((*iter1.borrow()) > (*iter2.borrow())) }));
        assert!((*iter2.borrow()) > (*iter1.borrow()));
        assert!((*iter1.borrow()) >= (*iter1.borrow()));
        assert!(({ !((*iter1.borrow()) >= (*iter2.borrow())) }));
        assert!((*iter2.borrow()) >= (*iter1.borrow()));
        assert!(({ (*iter1.borrow()) == (*iter1.borrow()) }));
        assert!(({ !((*iter1.borrow()) == (*iter2.borrow())) }));
        assert!(({ (*iter2.borrow()) == (*iter2.borrow()) }));
        assert!(({ !((*iter1.borrow()) != (*iter1.borrow())) }));
        assert!((*iter1.borrow()) != (*iter2.borrow()));
        assert!(({ !((*iter2.borrow()) != (*iter2.borrow())) }));
        assert!(
            ({ ((*iter1.borrow()).partial_cmp(&(*iter2.borrow())).unwrap()) })
                == ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) })
        );
        assert!(
            ({ ((*iter1.borrow()).partial_cmp(&(*iter1.borrow())).unwrap()) })
                == ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) })
        );
        assert!(
            ({ ((*iter2.borrow()).partial_cmp(&(*iter1.borrow())).unwrap()) })
                == ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) })
        );
        const _: () = assert!(
            true,
            "std::same_as<decltype(iter1 <=> iter2), std::strong_ordering>"
        );;
    }
    {
        const _: () = assert!(
            true,
            "std::three_way_comparable<std::ranges::iterator_t<R>>"
        );;
        let r: Value<std_ranges_repeat_view_int__int_> = Rc::new(RefCell::new({
            let __tmp_1: Value<i32> = Rc::new(RefCell::new(42));
            std_ranges_repeat_view_int__int_::new_1({ __tmp_1.as_pointer() }, { 10 })
        }));
        let iter1: Value<std_ranges_repeat_view_int__int___Iterator> =
            Rc::new(RefCell::new(({ (*r.borrow()).begin() })));
        let iter2: Value<std_ranges_repeat_view_int__int___Iterator> =
            Rc::new(RefCell::new(({ (*iter1.borrow()).clone() }).iter_add(1)));
        const _: () = assert!(true, "std::same_as<decltype(iter1 == iter2), bool>");;
        assert!(({ (*iter1.borrow()) == (*iter1.borrow()) }));
        assert!(({ !((*iter1.borrow()) == (*iter2.borrow())) }));
        assert!(({ (*iter2.borrow()) == (*iter2.borrow()) }));
        assert!(({ !((*iter1.borrow()) < (*iter1.borrow())) }));
        assert!((*iter1.borrow()) < (*iter2.borrow()));
        assert!(({ !((*iter2.borrow()) < (*iter1.borrow())) }));
        assert!((*iter1.borrow()) <= (*iter1.borrow()));
        assert!((*iter1.borrow()) <= (*iter2.borrow()));
        assert!(({ !((*iter2.borrow()) <= (*iter1.borrow())) }));
        assert!(({ !((*iter1.borrow()) > (*iter1.borrow())) }));
        assert!(({ !((*iter1.borrow()) > (*iter2.borrow())) }));
        assert!((*iter2.borrow()) > (*iter1.borrow()));
        assert!((*iter1.borrow()) >= (*iter1.borrow()));
        assert!(({ !((*iter1.borrow()) >= (*iter2.borrow())) }));
        assert!((*iter2.borrow()) >= (*iter1.borrow()));
        assert!(({ (*iter1.borrow()) == (*iter1.borrow()) }));
        assert!(({ !((*iter1.borrow()) == (*iter2.borrow())) }));
        assert!(({ (*iter2.borrow()) == (*iter2.borrow()) }));
        assert!(({ !((*iter1.borrow()) != (*iter1.borrow())) }));
        assert!((*iter1.borrow()) != (*iter2.borrow()));
        assert!(({ !((*iter2.borrow()) != (*iter2.borrow())) }));
        assert!(
            ({ ((*iter1.borrow()).partial_cmp(&(*iter2.borrow())).unwrap()) })
                == ({ (*libcc2rs::strong_ordering_less().upgrade().deref()) })
        );
        assert!(
            ({ ((*iter1.borrow()).partial_cmp(&(*iter1.borrow())).unwrap()) })
                == ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) })
        );
        assert!(
            ({ ((*iter2.borrow()).partial_cmp(&(*iter1.borrow())).unwrap()) })
                == ({ (*libcc2rs::strong_ordering_greater().upgrade().deref()) })
        );
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
    ({ test_0() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub fn __cpp2rust_init_globals() {}
