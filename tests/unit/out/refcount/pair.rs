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
fn main_0() -> () {
    let p1: Value<std_pair_double__int_> = Rc::new(RefCell::new({
        let __tmp_0: Value<()> = Rc::new(RefCell::new(1_()));
        let __tmp_1: Value<()> = Rc::new(RefCell::new(2_()));
        std_pair_double__int_::new_1({ __tmp_0.as_pointer() }, { __tmp_1.as_pointer() })
    }));
    unimplemented!();
    unimplemented!();
    let p2: Value<std_pair_double__int_> = Rc::new(RefCell::new({
        let __tmp_2: Value<()> = Rc::new(RefCell::new(3_()));
        let __tmp_3: Value<()> = Rc::new(RefCell::new(4_()));
        std_pair_double__int_::new_1({ __tmp_2.as_pointer() }, { __tmp_3.as_pointer() })
    }));
    unimplemented!();
    unimplemented!();
    let p3: Value<std_pair_double__int_> = Rc::new(RefCell::new(
        ({
            let ___x: Value<()> = Rc::new(RefCell::new(5_()));
            let ___y: Value<()> = Rc::new(RefCell::new(6_()));
            make_pair_0(___x.as_pointer(), ___y.as_pointer())
        }),
    ));
    unimplemented!();
    unimplemented!();
    let arr: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 2_()])));
    let n: Value<()> = Rc::new(RefCell::new(2_()));
    let const_ptr_pairs : Value<std_vector_std_pair_const_int_ptr__unsigned_long___std_allocator_std_pair_const_int_ptr__unsigned_long___ > = Rc::new(RefCell::new(std_vector_std_pair_const_int_ptr__unsigned_long___std_allocator_std_pair_const_int_ptr__unsigned_long___ :: new_1 ( { 1_()  } , None , )  )) ;
    (*const_ptr_pairs.borrow_mut())[0_()] = ({
        let ___x: Value<Ptr<()>> = Rc::new(RefCell::new(
            (arr.as_pointer() as Ptr<()>).offset((1_()) as isize),
        ));
        make_pair_1(___x.as_pointer(), n.as_pointer())
    });
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
