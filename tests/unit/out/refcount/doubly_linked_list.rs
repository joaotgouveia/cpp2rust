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
pub struct Node {
    #[offset(0)]
    pub val: i32,
    #[offset(8)]
    #[byte_size(8)]
    pub next: Ptr<Node>,
    #[offset(16)]
    #[byte_size(8)]
    pub prev: Ptr<Node>,
}
pub fn Find_0(mut head: Ptr<Node>, mut idx: i32) -> Ptr<Node> {
    let mut curr: Ptr<Node> = (head).clone();
    let mut i: i32 = 0;
    'loop_: while (i < idx) {
        curr = { curr.with(|__s| __s.next.clone()) };
        i.postfix_inc();
    }
    return curr;
}
pub fn FindBack_1(mut tail: Ptr<Node>, mut idx: i32) -> Ptr<Node> {
    let mut curr: Ptr<Node> = (tail).clone();
    let mut i: i32 = 0;
    'loop_: while (i < idx) {
        curr = { curr.with(|__s| __s.prev.clone()) };
        i.postfix_inc();
    }
    return curr;
}
pub fn Append_2(head: Ptr<Node>, new_node: Ptr<Node>) {
    let mut curr: Ptr<Node> = (head).clone();
    'loop_: while !((curr.with(|__s| __s.next.clone())).is_null()) {
        curr = { curr.with(|__s| __s.next.clone()) };
    }
    ({ NodeImpl::SetNext(&curr, (new_node).clone()) });
    ({
        let _p: Ptr<Node> = (curr).clone();
        NodeImpl::SetPrev(&new_node, _p)
    });
}
pub fn Delete_3(mut head: Ptr<Node>, mut val: i32) -> Ptr<Node> {
    let mut curr: Ptr<Node> = (head).clone();
    'loop_: while !((curr).is_null()) {
        if ({ curr.with(|__s| __s.val) } == { val }) {
            let mut prev: Ptr<Node> = curr.with(|__s| __s.prev.clone());
            let mut next: Ptr<Node> = curr.with(|__s| __s.next.clone());
            if !((prev).is_null()) {
                field!(prev, next).write((next).clone());
            }
            if !((next).is_null()) {
                field!(next, prev).write((prev).clone());
            }
            if !((prev).is_null()) {
                return head;
            } else {
                return next;
            }
        }
        curr = { curr.with(|__s| __s.next.clone()) };
    }
    return head;
}
pub fn Tail_4(mut head: Ptr<Node>) -> Ptr<Node> {
    let mut curr: Ptr<Node> = (head).clone();
    'loop_: while !((curr.with(|__s| __s.next.clone())).is_null()) {
        curr = { curr.with(|__s| __s.next.clone()) };
    }
    return curr;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let n0: Value<Node> = Rc::new(RefCell::new(Node {
        val: 5,
        next: Ptr::<Node>::null(),
        prev: Ptr::<Node>::null(),
    }));
    let mut head: Ptr<Node> = (n0.as_pointer());
    let n1: Value<Node> = Rc::new(RefCell::new(Node {
        val: 4,
        next: Ptr::<Node>::null(),
        prev: Ptr::<Node>::null(),
    }));
    let n2: Value<Node> = Rc::new(RefCell::new(Node {
        val: 3,
        next: Ptr::<Node>::null(),
        prev: Ptr::<Node>::null(),
    }));
    let n3: Value<Node> = Rc::new(RefCell::new(Node {
        val: 2,
        next: Ptr::<Node>::null(),
        prev: Ptr::<Node>::null(),
    }));
    let n4: Value<Node> = Rc::new(RefCell::new(Node {
        val: 1,
        next: Ptr::<Node>::null(),
        prev: Ptr::<Node>::null(),
    }));
    let n5: Value<Node> = Rc::new(RefCell::new(Node {
        val: 0,
        next: Ptr::<Node>::null(),
        prev: Ptr::<Node>::null(),
    }));
    let n6: Value<Node> = Rc::new(RefCell::new(Node {
        val: -1_i32,
        next: Ptr::<Node>::null(),
        prev: Ptr::<Node>::null(),
    }));
    let n7: Value<Node> = Rc::new(RefCell::new(Node {
        val: -2_i32,
        next: Ptr::<Node>::null(),
        prev: Ptr::<Node>::null(),
    }));
    ({
        let _head: Ptr<Node> = (head).clone();
        let _new_node: Ptr<Node> = n1.as_pointer();
        Append_2(_head, _new_node)
    });
    ({
        let _head: Ptr<Node> = (head).clone();
        let _new_node: Ptr<Node> = n2.as_pointer();
        Append_2(_head, _new_node)
    });
    ({
        let _head: Ptr<Node> = (head).clone();
        let _new_node: Ptr<Node> = n3.as_pointer();
        Append_2(_head, _new_node)
    });
    ({
        let _head: Ptr<Node> = (head).clone();
        let _new_node: Ptr<Node> = n4.as_pointer();
        Append_2(_head, _new_node)
    });
    ({
        let _head: Ptr<Node> = (head).clone();
        let _new_node: Ptr<Node> = n5.as_pointer();
        Append_2(_head, _new_node)
    });
    ({
        let _head: Ptr<Node> = (head).clone();
        let _new_node: Ptr<Node> = n6.as_pointer();
        Append_2(_head, _new_node)
    });
    ({
        let _head: Ptr<Node> = (head).clone();
        let _new_node: Ptr<Node> = n7.as_pointer();
        Append_2(_head, _new_node)
    });
    let __rhs = ({ Delete_3((head).clone(), 5) });
    head = __rhs;
    let __rhs = ({ Delete_3((head).clone(), 0) });
    head = __rhs;
    let __rhs = ({ Delete_3((head).clone(), -2_i32) });
    head = __rhs;
    let mut tail: Ptr<Node> = ({ Tail_4((head).clone()) });
    (if (({ Find_0((head).clone(), 0) }).with(|__s| __s.val) == 4) {
        &(0);
    } else {
        ({
            __assert_fail_5(
                Ptr::<i8>::from_string_literal(b"Find(head, 0)->val == 4"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ Find_0((head).clone(), 1) }).with(|__s| __s.val) == 3) {
        &(0);
    } else {
        ({
            __assert_fail_5(
                Ptr::<i8>::from_string_literal(b"Find(head, 1)->val == 3"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ Find_0((head).clone(), 2) }).with(|__s| __s.val) == 2) {
        &(0);
    } else {
        ({
            __assert_fail_5(
                Ptr::<i8>::from_string_literal(b"Find(head, 2)->val == 2"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ Find_0((head).clone(), 3) }).with(|__s| __s.val) == 1) {
        &(0);
    } else {
        ({
            __assert_fail_5(
                Ptr::<i8>::from_string_literal(b"Find(head, 3)->val == 1"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ Find_0((head).clone(), 4) }).with(|__s| __s.val) == -1_i32) {
        &(0);
    } else {
        ({
            __assert_fail_5(
                Ptr::<i8>::from_string_literal(b"Find(head, 4)->val == -1"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ({ Find_0((head).clone(), 5) }).is_null() {
        &(0);
    } else {
        ({
            __assert_fail_5(
                Ptr::<i8>::from_string_literal(b"Find(head, 5) == nullptr"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ FindBack_1((tail).clone(), 0) }).with(|__s| __s.val) == -1_i32) {
        &(0);
    } else {
        ({
            __assert_fail_5(
                Ptr::<i8>::from_string_literal(b"FindBack(tail, 0)->val == -1"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ FindBack_1((tail).clone(), 1) }).with(|__s| __s.val) == 1) {
        &(0);
    } else {
        ({
            __assert_fail_5(
                Ptr::<i8>::from_string_literal(b"FindBack(tail, 1)->val == 1"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ FindBack_1((tail).clone(), 2) }).with(|__s| __s.val) == 2) {
        &(0);
    } else {
        ({
            __assert_fail_5(
                Ptr::<i8>::from_string_literal(b"FindBack(tail, 2)->val == 2"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ FindBack_1((tail).clone(), 3) }).with(|__s| __s.val) == 3) {
        &(0);
    } else {
        ({
            __assert_fail_5(
                Ptr::<i8>::from_string_literal(b"FindBack(tail, 3)->val == 3"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ FindBack_1((tail).clone(), 4) }).with(|__s| __s.val) == 4) {
        &(0);
    } else {
        ({
            __assert_fail_5(
                Ptr::<i8>::from_string_literal(b"FindBack(tail, 4)->val == 4"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ FindBack_1((tail).clone(), 4) }).with(|__s| __s.prev.clone())).is_null() {
        &(0);
    } else {
        ({
            __assert_fail_5(
                Ptr::<i8>::from_string_literal(b"FindBack(tail, 4)->prev == nullptr"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ Find_0((head).clone(), 0) })
        .with(|__s| __s.next.clone())
        .with(|__s| __s.val)
        == 3)
    {
        &(0);
    } else {
        ({
            __assert_fail_5(
                Ptr::<i8>::from_string_literal(b"Find(head, 0)->next->val == 3"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ Find_0((head).clone(), 1) })
        .with(|__s| __s.next.clone())
        .with(|__s| __s.next.clone())
        .with(|__s| __s.val)
        == 1)
    {
        &(0);
    } else {
        ({
            __assert_fail_5(
                Ptr::<i8>::from_string_literal(b"Find(head, 1)->next->next->val == 1"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ Find_0((head).clone(), 2) })
        .with(|__s| __s.prev.clone())
        .with(|__s| __s.val)
        == 3)
    {
        &(0);
    } else {
        ({
            __assert_fail_5(
                Ptr::<i8>::from_string_literal(b"Find(head, 2)->prev->val == 3"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ Find_0((head).clone(), 4) }).with(|__s| __s.next.clone())).is_null() {
        &(0);
    } else {
        ({
            __assert_fail_5(
                Ptr::<i8>::from_string_literal(b"Find(head, 4)->next == nullptr"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ FindBack_1((tail).clone(), 1) })
        .with(|__s| __s.prev.clone())
        .with(|__s| __s.prev.clone())
        .with(|__s| __s.val)
        == 3)
    {
        &(0);
    } else {
        ({
            __assert_fail_5(
                Ptr::<i8>::from_string_literal(b"FindBack(tail, 1)->prev->prev->val == 3"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    field!(
        ({ Find_0((head).clone(), 0,) }).with(|__s| __s.next.clone()),
        val
    )
    .write(30);
    (if (({ Find_0((head).clone(), 1) }).with(|__s| __s.val) == 30) {
        &(0);
    } else {
        ({
            __assert_fail_5(
                Ptr::<i8>::from_string_literal(b"Find(head, 1)->val == 30"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let __rhs = (({ Find_0((head).clone(), 0) }).with(|__s| __s.val)
        + ({ Find_0((head).clone(), 3) }).with(|__s| __s.val));
    field!(
        ({ Find_0((head).clone(), 1,) }).with(|__s| __s.next.clone()),
        val
    )
    .write(__rhs);
    (if (({ Find_0((head).clone(), 2) }).with(|__s| __s.val) == (4 + 1)) {
        &(0);
    } else {
        ({
            __assert_fail_5(
                Ptr::<i8>::from_string_literal(b"Find(head, 2)->val == 4 + 1"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut sum: i32 = ((((({ Find_0((head).clone(), 0) }).with(|__s| __s.val)
        + ({ Find_0((head).clone(), 1) }).with(|__s| __s.val))
        + ({ Find_0((head).clone(), 2) }).with(|__s| __s.val))
        + ({ Find_0((head).clone(), 3) }).with(|__s| __s.val))
        + ({ Find_0((head).clone(), 4) }).with(|__s| __s.val));
    (if (sum == ((((4 + 30) + 5) + 1) + -1_i32)) {
        &(0);
    } else {
        ({
            __assert_fail_5(
                Ptr::<i8>::from_string_literal(b"sum == 4 + 30 + 5 + 1 + -1"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ ({ Find_0((head).clone(), 0) }).with(|__s| __s.val) } + {
        ({ FindBack_1((tail).clone(), 0) }).with(|__s| __s.val)
    }) == (4 + -1_i32))
    {
        &(0);
    } else {
        ({
            __assert_fail_5(
                Ptr::<i8>::from_string_literal(
                    b"Find(head, 0)->val + FindBack(tail, 0)->val == 4 + -1",
                ),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ({
        ({ Find_0((head).clone(), 2) })
            .with(|__s| __s.next.clone())
            .with(|__s| __s.val)
    } == { ({ FindBack_1((tail).clone(), 1) }).with(|__s| __s.val) })
    {
        &(0);
    } else {
        ({
            __assert_fail_5(
                Ptr::<i8>::from_string_literal(
                    b"Find(head, 2)->next->val == FindBack(tail, 1)->val",
                ),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ({ ({ Find_0((head).clone(), 0) }).with(|__s| __s.prev.clone()) } == {
        ({ FindBack_1((tail).clone(), 4) }).with(|__s| __s.prev.clone())
    }) {
        &(0);
    } else {
        ({
            __assert_fail_5(
                Ptr::<i8>::from_string_literal(b"Find(head, 0)->prev == FindBack(tail, 4)->prev"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    return 0;
}
pub fn __assert_fail_5(
    mut __assertion: Ptr<i8>,
    mut __file: Ptr<i8>,
    mut __line: u32,
    mut __function: Ptr<i8>,
) {
    unimplemented!()
}
pub trait NodeImpl {
    fn SetNext(&self, n: Ptr<Node>);
    fn SetPrev(&self, p: Ptr<Node>);
}
impl NodeImpl for Ptr<Node> {
    fn SetNext(&self, mut n: Ptr<Node>) {
        field!((*self), next).write((n).clone());
    }
    fn SetPrev(&self, mut p: Ptr<Node>) {
        field!((*self), prev).write((p).clone());
    }
}
pub fn __cpp2rust_init_globals() {}
