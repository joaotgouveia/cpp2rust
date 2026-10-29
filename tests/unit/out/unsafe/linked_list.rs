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
}
impl Node {
    pub unsafe fn SetNext(&mut self, mut next: *mut Node) {
        self.next = next;
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
pub unsafe fn Append_1(head: *mut Node, new_node: *mut Node) {
    let mut curr: *mut Node = (head);
    'loop_: while !(((*curr).next).is_null()) {
        curr = (*curr).next;
    }
    (unsafe { Node::SetNext(&mut (*curr), (new_node)) });
}
pub unsafe fn Delete_2(mut head: *mut Node, mut val: ()) -> *mut Node {
    let mut curr: *mut Node = head;
    let mut prev: *mut Node = std::ptr::null_mut();
    'loop_: while !((curr).is_null()) {
        if (((*curr).val) == (val)) {
            if !((prev).is_null()) {
                (*prev).next = (*curr).next;
                return head;
            } else {
                return (*curr).next;
            }
        }
        prev = curr;
        curr = (*curr).next;
    }
    return head;
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
    };
    let mut head: *mut Node = (&mut n0 as *mut Node);
    let mut n1: Node = Node {
        val: 4_(),
        next: std::ptr::null_mut(),
    };
    let mut n2: Node = Node {
        val: 3_(),
        next: std::ptr::null_mut(),
    };
    let mut n3: Node = Node {
        val: 2_(),
        next: std::ptr::null_mut(),
    };
    let mut n4: Node = Node {
        val: 1_(),
        next: std::ptr::null_mut(),
    };
    let mut n5: Node = Node {
        val: 0_(),
        next: std::ptr::null_mut(),
    };
    let mut n6: Node = Node {
        val: -1_(),
        next: std::ptr::null_mut(),
    };
    let mut n7: Node = Node {
        val: -2_(),
        next: std::ptr::null_mut(),
    };
    (unsafe {
        let _head: *mut Node = &mut (*head);
        let _new_node: *mut Node = &mut n1;
        Append_1(_head, _new_node)
    });
    (unsafe {
        let _head: *mut Node = &mut (*head);
        let _new_node: *mut Node = &mut n2;
        Append_1(_head, _new_node)
    });
    (unsafe {
        let _head: *mut Node = &mut (*head);
        let _new_node: *mut Node = &mut n3;
        Append_1(_head, _new_node)
    });
    (unsafe {
        let _head: *mut Node = &mut (*head);
        let _new_node: *mut Node = &mut n4;
        Append_1(_head, _new_node)
    });
    (unsafe {
        let _head: *mut Node = &mut (*head);
        let _new_node: *mut Node = &mut n5;
        Append_1(_head, _new_node)
    });
    (unsafe {
        let _head: *mut Node = &mut (*head);
        let _new_node: *mut Node = &mut n6;
        Append_1(_head, _new_node)
    });
    (unsafe {
        let _head: *mut Node = &mut (*head);
        let _new_node: *mut Node = &mut n7;
        Append_1(_head, _new_node)
    });
    head = (unsafe { Delete_2(head, 5_()) });
    head = (unsafe { Delete_2(head, 0_()) });
    head = (unsafe { Delete_2(head, -2_()) });
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
