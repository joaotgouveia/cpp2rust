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
    let v: Value<std_vector_int__std_allocator_int__> = Rc::new(RefCell::new(
        <std_vector_int__std_allocator_int__>::default(),
    ));
    ({
        let ___x: Value<i32> = Rc::new(RefCell::new(10));
        (*v.borrow()).push_back_60(___x.as_pointer())
    });
    ({
        let ___x: Value<i32> = Rc::new(RefCell::new(1));
        (*v.borrow()).push_back_60(___x.as_pointer())
    });
    ({
        let ___x: Value<i32> = Rc::new(RefCell::new(9));
        (*v.borrow()).push_back_60(___x.as_pointer())
    });
    ({
        let ___x: Value<i32> = Rc::new(RefCell::new(2));
        (*v.borrow()).push_back_60(___x.as_pointer())
    });
    ({
        let ___x: Value<i32> = Rc::new(RefCell::new(8));
        (*v.borrow()).push_back_60(___x.as_pointer())
    });
    ({
        let ___x: Value<i32> = Rc::new(RefCell::new(3));
        (*v.borrow()).push_back_60(___x.as_pointer())
    });
    ({
        let ___x: Value<i32> = Rc::new(RefCell::new(7));
        (*v.borrow()).push_back_60(___x.as_pointer())
    });
    ({
        let ___x: Value<i32> = Rc::new(RefCell::new(4));
        (*v.borrow()).push_back_60(___x.as_pointer())
    });
    ({
        let ___x: Value<i32> = Rc::new(RefCell::new(5));
        (*v.borrow()).push_back_60(___x.as_pointer())
    });
    ({
        let ___x: Value<i32> = Rc::new(RefCell::new(6));
        (*v.borrow()).push_back_60(___x.as_pointer())
    });
    ({
        let ___first: __gnu_cxx___normal_iterator_int_ptr__std_vector_int__std_allocator_int___ =
            ({ (*v.borrow()).begin_28() });
        let ___last: __gnu_cxx___normal_iterator_int_ptr__std_vector_int__std_allocator_int___ =
            ({ (*v.borrow()).end_30() });
        sort_0(___first, ___last)
    });
    let mut i: u32 = 0_u32;
    'loop_: while ((i as u64) < ({ (*v.borrow()).size() }).wrapping_sub(1_u64)) {
        ({
            cpp2rust_assert_fail_1(
                ({ (*v.borrow())[(i as u64)] } < {
                    (*v.borrow())[(((i).wrapping_add(1_u32)) as u64)]
                }),
            )
        });
        i.prefix_inc();
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
