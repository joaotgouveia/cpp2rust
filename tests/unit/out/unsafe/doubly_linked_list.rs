extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Node {
    pub val: (),
    pub next: *mut Node,
    pub prev: *mut Node,
}
impl Node {
    pub unsafe fn SetNext(&mut self, mut n: *mut Node) {
        self.next = n;
    }
    pub unsafe fn SetPrev(&mut self, mut p: *mut Node) {
        self.prev = p;
    }
}
pub unsafe fn Find_0(mut head: *mut Node, mut idx: ()) -> *mut Node {
    let mut curr: *mut Node = head;
    let mut i: () = 0_();
    'loop_: while ((i) < (idx)) {
        curr = (*curr).next;
        i.postfix_inc();
    }
    return curr;
}
pub unsafe fn FindBack_1(mut tail: *mut Node, mut idx: ()) -> *mut Node {
    let mut curr: *mut Node = tail;
    let mut i: () = 0_();
    'loop_: while ((i) < (idx)) {
        curr = (*curr).prev;
        i.postfix_inc();
    }
    return curr;
}
pub unsafe fn Append_2(head: *mut Node, new_node: *mut Node) {
    let mut curr: *mut Node = (head);
    'loop_: while !(((*curr).next).is_null()) {
        curr = (*curr).next;
    }
    (unsafe { Node::SetNext(&mut (*curr), (new_node)) });
    (unsafe {
        let _p: *mut Node = curr;
        Node::SetPrev(&mut (*new_node), _p)
    });
}
pub unsafe fn Delete_3(mut head: *mut Node, mut val: ()) -> *mut Node {
    let mut curr: *mut Node = head;
    'loop_: while !((curr).is_null()) {
        if (((*curr).val) == (val)) {
            let mut prev: *mut Node = (*curr).prev;
            let mut next: *mut Node = (*curr).next;
            if !((prev).is_null()) {
                (*prev).next = next;
            }
            if !((next).is_null()) {
                (*next).prev = prev;
            }
            if !((prev).is_null()) {
                return head;
            } else {
                return next;
            }
        }
        curr = (*curr).next;
    }
    return head;
}
pub unsafe fn Tail_4(mut head: *mut Node) -> *mut Node {
    let mut curr: *mut Node = head;
    'loop_: while !(((*curr).next).is_null()) {
        curr = (*curr).next;
    }
    return curr;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut n0: Node = Node {
        val: 5_(),
        next: std::ptr::null_mut(),
        prev: std::ptr::null_mut(),
    };
    let mut head: *mut Node = (&mut n0 as *mut Node);
    let mut n1: Node = Node {
        val: 4_(),
        next: std::ptr::null_mut(),
        prev: std::ptr::null_mut(),
    };
    let mut n2: Node = Node {
        val: 3_(),
        next: std::ptr::null_mut(),
        prev: std::ptr::null_mut(),
    };
    let mut n3: Node = Node {
        val: 2_(),
        next: std::ptr::null_mut(),
        prev: std::ptr::null_mut(),
    };
    let mut n4: Node = Node {
        val: 1_(),
        next: std::ptr::null_mut(),
        prev: std::ptr::null_mut(),
    };
    let mut n5: Node = Node {
        val: 0_(),
        next: std::ptr::null_mut(),
        prev: std::ptr::null_mut(),
    };
    let mut n6: Node = Node {
        val: -1_(),
        next: std::ptr::null_mut(),
        prev: std::ptr::null_mut(),
    };
    let mut n7: Node = Node {
        val: -2_(),
        next: std::ptr::null_mut(),
        prev: std::ptr::null_mut(),
    };
    (unsafe {
        let _head: *mut Node = &mut (*head);
        let _new_node: *mut Node = &mut n1;
        Append_2(_head, _new_node)
    });
    (unsafe {
        let _head: *mut Node = &mut (*head);
        let _new_node: *mut Node = &mut n2;
        Append_2(_head, _new_node)
    });
    (unsafe {
        let _head: *mut Node = &mut (*head);
        let _new_node: *mut Node = &mut n3;
        Append_2(_head, _new_node)
    });
    (unsafe {
        let _head: *mut Node = &mut (*head);
        let _new_node: *mut Node = &mut n4;
        Append_2(_head, _new_node)
    });
    (unsafe {
        let _head: *mut Node = &mut (*head);
        let _new_node: *mut Node = &mut n5;
        Append_2(_head, _new_node)
    });
    (unsafe {
        let _head: *mut Node = &mut (*head);
        let _new_node: *mut Node = &mut n6;
        Append_2(_head, _new_node)
    });
    (unsafe {
        let _head: *mut Node = &mut (*head);
        let _new_node: *mut Node = &mut n7;
        Append_2(_head, _new_node)
    });
    head = (unsafe { Delete_3(head, 5_()) });
    head = (unsafe { Delete_3(head, 0_()) });
    head = (unsafe { Delete_3(head, -2_()) });
    let mut tail: *mut Node = (unsafe { Tail_4(head) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (*(*(unsafe { Find_0(head, 0_()) })).next).val = 30_();
    unimplemented!();
    (*(*(unsafe { Find_0(head, 1_()) })).next).val =
        (((*(unsafe { Find_0(head, 0_()) })).val) + ((*(unsafe { Find_0(head, 3_()) })).val));
    unimplemented!();
    let mut sum: () = ((((((*(unsafe { Find_0(head, 0_()) })).val)
        + ((*(unsafe { Find_0(head, 1_()) })).val))
        + ((*(unsafe { Find_0(head, 2_()) })).val))
        + ((*(unsafe { Find_0(head, 3_()) })).val))
        + ((*(unsafe { Find_0(head, 4_()) })).val));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
