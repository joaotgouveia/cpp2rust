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
    pub value: (),
}
pub fn find_0(mut node: Ptr<node_t>, mut value: ()) -> Ptr<node_t> {
    if ({ value } < { node.with(|__s| __s.value) })
        && (!((node.with(|__s| __s.left.clone())).is_null()))
    {
        return ({ find_0(node.with(|__s| __s.left.clone()), value) });
    } else if ({ value } > { node.with(|__s| __s.value) })
        && (!((node.with(|__s| __s.right.clone())).is_null()))
    {
        return ({ find_0(node.with(|__s| __s.right.clone()), value) });
    } else if ({ value } == { node.with(|__s| __s.value) }) {
        return node;
    }
    return Ptr::<node_t>::null();
}
pub fn insert_1(mut node: Ptr<node_t>, mut new_node: Ptr<node_t>) -> Ptr<node_t> {
    if (node).is_null() {
        return new_node;
    }
    if ({ new_node.with(|__s| __s.value) } < { node.with(|__s| __s.value) }) {
        let __rhs = ({ insert_1(node.with(|__s| __s.left.clone()), (new_node).clone()) });
        field!(node, left).write(__rhs);
    } else if ({ new_node.with(|__s| __s.value) } > { node.with(|__s| __s.value) }) {
        let __rhs = ({ insert_1(node.with(|__s| __s.right.clone()), (new_node).clone()) });
        field!(node, right).write(__rhs);
    }
    return node;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let tree: Value<std_unique_ptr_node_t__std_default_delete_node_t__> = Rc::new(RefCell::new(
        ({
            let ___args: Value<node_t> = Rc::new(RefCell::new(node_t {
                left: Ptr::<node_t>::null(),
                right: Ptr::<node_t>::null(),
                value: 0_(),
            }));
            make_unique_2(___args.as_pointer())
        }),
    ));
    let n1: Value<std_unique_ptr_node_t__std_default_delete_node_t__> = Rc::new(RefCell::new(
        ({
            let ___args: Value<node_t> = Rc::new(RefCell::new(node_t {
                left: Ptr::<node_t>::null(),
                right: Ptr::<node_t>::null(),
                value: 1_(),
            }));
            make_unique_2(___args.as_pointer())
        }),
    ));
    let n2: Value<std_unique_ptr_node_t__std_default_delete_node_t__> = Rc::new(RefCell::new(
        ({
            let ___args: Value<node_t> = Rc::new(RefCell::new(node_t {
                left: Ptr::<node_t>::null(),
                right: Ptr::<node_t>::null(),
                value: 2_(),
            }));
            make_unique_2(___args.as_pointer())
        }),
    ));
    let n3: Value<std_unique_ptr_node_t__std_default_delete_node_t__> = Rc::new(RefCell::new(
        ({
            let ___args: Value<node_t> = Rc::new(RefCell::new(node_t {
                left: Ptr::<node_t>::null(),
                right: Ptr::<node_t>::null(),
                value: 3_(),
            }));
            make_unique_2(___args.as_pointer())
        }),
    ));
    let n4: Value<std_unique_ptr_node_t__std_default_delete_node_t__> = Rc::new(RefCell::new(
        ({
            let ___args: Value<node_t> = Rc::new(RefCell::new(node_t {
                left: Ptr::<node_t>::null(),
                right: Ptr::<node_t>::null(),
                value: 4_(),
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
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
