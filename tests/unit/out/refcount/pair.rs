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
    let p1: Value<std_pair_double__int_> = Rc::new(RefCell::new({
        let __tmp_0: Value<i32> = Rc::new(RefCell::new(1));
        let __tmp_1: Value<i32> = Rc::new(RefCell::new(2));
        std_pair_double__int_::new_1({ __tmp_0.as_pointer() }, { __tmp_1.as_pointer() })
    }));
    ({ cpp2rust_assert_fail_0(({ (*p1.borrow()).first } == 1_f64)) });
    ({ cpp2rust_assert_fail_0(({ (*p1.borrow()).second } == 2)) });
    let p2: Value<std_pair_double__int_> = Rc::new(RefCell::new({
        let __tmp_2: Value<i32> = Rc::new(RefCell::new(3));
        let __tmp_3: Value<i32> = Rc::new(RefCell::new(4));
        std_pair_double__int_::new_1({ __tmp_2.as_pointer() }, { __tmp_3.as_pointer() })
    }));
    ({ cpp2rust_assert_fail_0(({ (*p2.borrow()).first } == 3_f64)) });
    ({ cpp2rust_assert_fail_0(({ (*p2.borrow()).second } == 4)) });
    let p3: Value<std_pair_double__int_> = Rc::new(RefCell::new(
        ({
            let ___x: Value<i32> = Rc::new(RefCell::new(5));
            let ___y: Value<i32> = Rc::new(RefCell::new(6));
            make_pair_1(___x.as_pointer(), ___y.as_pointer())
        }),
    ));
    ({ cpp2rust_assert_fail_0(({ (*p3.borrow()).first } == 5_f64)) });
    ({ cpp2rust_assert_fail_0(({ (*p3.borrow()).second } == 6)) });
    let arr: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2])));
    let n: Value<u32> = Rc::new(RefCell::new(2_u32));
    let const_ptr_pairs : Value<std_vector_std_pair_const_int_ptr__unsigned_long___std_allocator_std_pair_const_int_ptr__unsigned_long___ > = Rc::new(RefCell::new(std_vector_std_pair_const_int_ptr__unsigned_long___std_allocator_std_pair_const_int_ptr__unsigned_long___ :: new_1 ( { 1_u64  } , None , )  )) ;
    (*const_ptr_pairs.borrow_mut())[0_u64] = ({
        let ___x: Value<Ptr<i32>> = Rc::new(RefCell::new(
            (arr.as_pointer() as Ptr<i32>).offset((1) as isize),
        ));
        make_pair_2(___x.as_pointer(), n.as_pointer())
    });
    ({
        cpp2rust_assert_fail_0(
            (({ (*const_ptr_pairs.borrow())[0_u64].first.clone() }.read()) == 2)
                && ({ (*const_ptr_pairs.borrow())[0_u64].second } == 2_u64),
        )
    });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
