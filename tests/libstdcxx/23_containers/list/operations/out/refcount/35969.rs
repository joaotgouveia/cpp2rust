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
        let node_of_interest: Value<std__List_iterator_int_> =
            Rc::new(RefCell::new(({ (*list2.borrow()).begin_24() })));
        ({
            let ___position: std__List_const_iterator_int_ = {
                let __tmp_0: Value<std__List_iterator_int_> =
                    Rc::new(RefCell::new(({ (*list1.borrow()).begin_24() })));
                std__List_const_iterator_int_::new_1({ __tmp_0.as_pointer() })
            };
            (*list1.borrow()).splice_69(
                ___position,
                list2.as_pointer(),
                std__List_const_iterator_int_::new_1({ node_of_interest.as_pointer() }),
            )
        });
        ({
            let ___position: std__List_const_iterator_int_ = {
                let __tmp_1: Value<std__List_iterator_int_> =
                    Rc::new(RefCell::new(({ (*list2.borrow()).begin_24() })));
                std__List_const_iterator_int_::new_1({ __tmp_1.as_pointer() })
            };
            (*list2.borrow()).splice_69(
                ___position,
                list1.as_pointer(),
                std__List_const_iterator_int_::new_1({ node_of_interest.as_pointer() }),
            )
        });
        ({ (*list1.borrow()).merge_77(list2.as_pointer()) });
        ({
            let ___position: std__List_const_iterator_int_ = {
                let __tmp_2: Value<std__List_iterator_int_> =
                    Rc::new(RefCell::new(({ (*list2.borrow()).begin_24() })));
                std__List_const_iterator_int_::new_1({ __tmp_2.as_pointer() })
            };
            (*list2.borrow()).splice_69(
                ___position,
                list1.as_pointer(),
                std__List_const_iterator_int_::new_1({ node_of_interest.as_pointer() }),
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
        let node_of_interest: Value<std__List_iterator_int_> =
            Rc::new(RefCell::new(({ (*list2.borrow()).begin_24() })));
        ({
            let ___position: std__List_const_iterator_int_ = {
                let __tmp_3: Value<std__List_iterator_int_> =
                    Rc::new(RefCell::new(({ (*list1.borrow()).begin_24() })));
                std__List_const_iterator_int_::new_1({ __tmp_3.as_pointer() })
            };
            (*list1.borrow()).splice_69(
                ___position,
                list2.as_pointer(),
                std__List_const_iterator_int_::new_1({ node_of_interest.as_pointer() }),
            )
        });
        ({
            let ___position: std__List_const_iterator_int_ = {
                let __tmp_4: Value<std__List_iterator_int_> =
                    Rc::new(RefCell::new(({ (*list2.borrow()).begin_24() })));
                std__List_const_iterator_int_::new_1({ __tmp_4.as_pointer() })
            };
            (*list2.borrow()).splice_69(
                ___position,
                list1.as_pointer(),
                std__List_const_iterator_int_::new_1({ node_of_interest.as_pointer() }),
            )
        });
        ({
            (*list1.borrow())
                .merge_79_struct_std_less_int_(list2.as_pointer(), <std_less_int_>::default())
        });
        ({
            let ___position: std__List_const_iterator_int_ = {
                let __tmp_5: Value<std__List_iterator_int_> =
                    Rc::new(RefCell::new(({ (*list2.borrow()).begin_24() })));
                std__List_const_iterator_int_::new_1({ __tmp_5.as_pointer() })
            };
            (*list2.borrow()).splice_69(
                ___position,
                list1.as_pointer(),
                std__List_const_iterator_int_::new_1({ node_of_interest.as_pointer() }),
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
