extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(24)]
pub struct node_t {
    #[offset(0)]
    #[byte_size(8)]
    pub left: Ptr<node_t>,
    #[offset(8)]
    #[byte_size(8)]
    pub right: Ptr<node_t>,
    #[offset(16)]
    pub value: i32,
}
impl CppMove for node_t {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn find_0(node: Ptr<node_t>, mut value: i32) -> Ptr<node_t> {
    let node: Value<Ptr<node_t>> = Rc::new(RefCell::new(node));
    if ({ value } < { (*node.borrow()).with(|__s| __s.value) })
        && (!(((*node.borrow()).with(|__s| __s.left.clone())).is_null()))
    {
        return ({ find_0((*node.borrow()).with(|__s| __s.left.clone()), value) });
    } else if ({ value } > { (*node.borrow()).with(|__s| __s.value) })
        && (!(((*node.borrow()).with(|__s| __s.right.clone())).is_null()))
    {
        return ({ find_0((*node.borrow()).with(|__s| __s.right.clone()), value) });
    } else if ({ value } == { (*node.borrow()).with(|__s| __s.value) }) {
        return (*node.borrow()).clone();
    }
    return Ptr::<node_t>::null();
}
pub fn insert_1(node: Ptr<node_t>, new_node: Ptr<node_t>) -> Ptr<node_t> {
    let node: Value<Ptr<node_t>> = Rc::new(RefCell::new(node));
    let new_node: Value<Ptr<node_t>> = Rc::new(RefCell::new(new_node));
    if (*node.borrow()).is_null() {
        return (*new_node.borrow()).clone();
    }
    if ({ (*new_node.borrow()).with(|__s| __s.value) } < { (*node.borrow()).with(|__s| __s.value) })
    {
        let __rhs = ({
            insert_1(
                (*node.borrow()).with(|__s| __s.left.clone()),
                (*new_node.borrow()).clone(),
            )
        });
        field!((*node.borrow()), left).write(__rhs);
    } else if ({ (*new_node.borrow()).with(|__s| __s.value) } > {
        (*node.borrow()).with(|__s| __s.value)
    }) {
        let __rhs = ({
            insert_1(
                (*node.borrow()).with(|__s| __s.right.clone()),
                (*new_node.borrow()).clone(),
            )
        });
        field!((*node.borrow()), right).write(__rhs);
    }
    return (*node.borrow()).clone();
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let tree: Value<std_unique_ptr_node_t__std_default_delete_node_t__> = Rc::new(RefCell::new(
        ({
            let ___args: Value<node_t> = Rc::new(RefCell::new(node_t {
                left: Ptr::<node_t>::null(),
                right: Ptr::<node_t>::null(),
                value: 0,
            }));
            make_unique_2(___args.as_pointer())
        }),
    ));
    let n1: Value<std_unique_ptr_node_t__std_default_delete_node_t__> = Rc::new(RefCell::new(
        ({
            let ___args: Value<node_t> = Rc::new(RefCell::new(node_t {
                left: Ptr::<node_t>::null(),
                right: Ptr::<node_t>::null(),
                value: 1,
            }));
            make_unique_2(___args.as_pointer())
        }),
    ));
    let n2: Value<std_unique_ptr_node_t__std_default_delete_node_t__> = Rc::new(RefCell::new(
        ({
            let ___args: Value<node_t> = Rc::new(RefCell::new(node_t {
                left: Ptr::<node_t>::null(),
                right: Ptr::<node_t>::null(),
                value: 2,
            }));
            make_unique_2(___args.as_pointer())
        }),
    ));
    let n3: Value<std_unique_ptr_node_t__std_default_delete_node_t__> = Rc::new(RefCell::new(
        ({
            let ___args: Value<node_t> = Rc::new(RefCell::new(node_t {
                left: Ptr::<node_t>::null(),
                right: Ptr::<node_t>::null(),
                value: 3,
            }));
            make_unique_2(___args.as_pointer())
        }),
    ));
    let n4: Value<std_unique_ptr_node_t__std_default_delete_node_t__> = Rc::new(RefCell::new(
        ({
            let ___args: Value<node_t> = Rc::new(RefCell::new(node_t {
                left: Ptr::<node_t>::null(),
                right: Ptr::<node_t>::null(),
                value: 4,
            }));
            make_unique_2(___args.as_pointer())
        }),
    ));
    let mut ptr1: Ptr<node_t> = (tree.as_pointer());
    let __rhs = ({ insert_1((ptr1).clone(), (n1.as_pointer())) });
    ptr1 = __rhs;
    let __rhs = ({ insert_1((ptr1).clone(), (n2.as_pointer())) });
    ptr1 = __rhs;
    let __rhs = ({ insert_1((ptr1).clone(), (n3.as_pointer())) });
    ptr1 = __rhs;
    let __rhs = ({ insert_1((ptr1).clone(), (n4.as_pointer())) });
    ptr1 = __rhs;
    ({
        cpp2rust_assert_fail_3(
            (((((({ find_0((ptr1).clone(), 0) }).with(|__s| __s.value) == 0)
                && (({ find_0((ptr1).clone(), 1) }).with(|__s| __s.value) == 1))
                && (({ find_0((ptr1).clone(), 2) }).with(|__s| __s.value) == 2))
                && (({ find_0((ptr1).clone(), 3) }).with(|__s| __s.value) == 3))
                && (({ find_0((ptr1).clone(), 4) }).with(|__s| __s.value) == 4))
                && (({ find_0((ptr1).clone(), 5) }).is_null()),
        )
    });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
