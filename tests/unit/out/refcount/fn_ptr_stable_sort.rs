extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Item {
    #[offset(0)]
    pub key: (),
    #[offset(4)]
    pub value: (),
}
pub fn Compare_0(a: Ptr<Item>, b: Ptr<Item>) -> () {
    return ({ a.with(|__s| __s.key) } < { b.with(|__s| __s.key) });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let v: Value<std_vector_Item__std_allocator_Item__> = Rc::new(RefCell::new(
        <std_vector_Item__std_allocator_Item__>::default(),
    ));
    ({
        let ___x: Value<Item> = Rc::new(RefCell::new(Item {
            key: 3_(),
            value: 30_(),
        }));
        (*v.borrow()).push_back_58(___x.as_pointer())
    });
    ({
        let ___x: Value<Item> = Rc::new(RefCell::new(Item {
            key: 1_(),
            value: 10_(),
        }));
        (*v.borrow()).push_back_58(___x.as_pointer())
    });
    ({
        let ___x: Value<Item> = Rc::new(RefCell::new(Item {
            key: 2_(),
            value: 20_(),
        }));
        (*v.borrow()).push_back_58(___x.as_pointer())
    });
    ({
        let ___first: __gnu_cxx___normal_iterator_Item_ptr__std_vector_Item__std_allocator_Item___ =
            ({ (*v.borrow()).begin_26() });
        let ___last: __gnu_cxx___normal_iterator_Item_ptr__std_vector_Item__std_allocator_Item___ =
            ({ (*v.borrow()).end_28() });
        stable_sort_1(
            ___first,
            ___last,
            FnPtr::<fn(Ptr<Item>, Ptr<Item>) -> ()>::new(Compare_0),
        )
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
