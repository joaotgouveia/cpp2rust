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
    pub key: i32,
    #[offset(4)]
    pub value: i32,
}
impl CppMove for Item {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn Compare_0(a: Ptr<Item>, b: Ptr<Item>) -> bool {
    return ({ a.with(|__s| __s.key) } < { b.with(|__s| __s.key) });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let v: Value<std_vector_Item__std_allocator_Item__> = Rc::new(RefCell::new(
        <std_vector_Item__std_allocator_Item__>::default(),
    ));
    ({
        let ___x: Value<Item> = Rc::new(RefCell::new(Item { key: 3, value: 30 }));
        (*v.borrow()).push_back_60(___x.as_pointer())
    });
    ({
        let ___x: Value<Item> = Rc::new(RefCell::new(Item { key: 1, value: 10 }));
        (*v.borrow()).push_back_60(___x.as_pointer())
    });
    ({
        let ___x: Value<Item> = Rc::new(RefCell::new(Item { key: 2, value: 20 }));
        (*v.borrow()).push_back_60(___x.as_pointer())
    });
    ({
        let ___first: __gnu_cxx___normal_iterator_Item_ptr__std_vector_Item__std_allocator_Item___ =
            ({ (*v.borrow()).begin_28() });
        let ___last: __gnu_cxx___normal_iterator_Item_ptr__std_vector_Item__std_allocator_Item___ =
            ({ (*v.borrow()).end_30() });
        stable_sort_1(
            ___first,
            ___last,
            FnPtr::<fn(Ptr<Item>, Ptr<Item>) -> bool>::new(Compare_0),
        )
    });
    ({ cpp2rust_assert_fail_2(({ (*v.borrow())[0_u64].key } == 1)) });
    ({ cpp2rust_assert_fail_2(({ (*v.borrow())[1_u64].key } == 2)) });
    ({ cpp2rust_assert_fail_2(({ (*v.borrow())[2_u64].key } == 3)) });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
