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
pub struct MinHeapNode {
    pub data: (),
    pub freq: (),
    pub left: *mut MinHeapNode,
    pub right: *mut MinHeapNode,
}
impl MinHeapNode {
    pub unsafe fn IsLeaf(&self) -> () {
        return ((self.left).is_null()) && ((self.right).is_null());
    }
}
pub unsafe fn Swap_0(a: *mut MinHeapNode, b: *mut MinHeapNode) {
    let mut t: MinHeapNode = MinHeapNode {
        data: (*a).data,
        freq: (*a).freq,
        left: (*a).left,
        right: (*a).right,
    };
    (*a) = MinHeapNode {
        data: (*b).data,
        freq: (*b).freq,
        left: (*b).left,
        right: (*b).right,
    };
    (*b) = MinHeapNode {
        data: t.data,
        freq: t.freq,
        left: t.left,
        right: t.right,
    };
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct MinHeap {
    pub size: (),
    pub capacity: (),
    pub arr: (),
    pub next: (),
    pub alloc: (),
}
impl MinHeap {
    pub unsafe fn Alloc(&mut self, mut data: (), mut freq: ()) -> *mut MinHeapNode {
        (*unimplemented!()) = MinHeapNode {
            data: data,
            freq: freq,
            left: std::ptr::null_mut(),
            right: std::ptr::null_mut(),
        };
        return (&mut (*unimplemented!()) as *mut MinHeapNode);
    }
    pub unsafe fn Heapify(&mut self, mut idx: ()) {
        let mut smallest: () = idx;
        let mut left: () = (((2_()) * (idx)) + (1_()));
        let mut right: () = (((2_()) * (idx)) + (2_()));
        if ((left) < (self.size)) && (((*(*unimplemented!())).freq) < ((*(*unimplemented!())).freq))
        {
            smallest = left;
        }
        if ((right) < (self.size))
            && (((*(*unimplemented!())).freq) < ((*(*unimplemented!())).freq))
        {
            smallest = right;
        }
        if ((smallest) != (idx)) {
            (unsafe {
                let _a: *mut MinHeapNode = &mut (*(*unimplemented!()));
                let _b: *mut MinHeapNode = &mut (*(*unimplemented!()));
                Swap_0(_a, _b)
            });
            (unsafe { MinHeap::Heapify(self, smallest) });
        }
    }
    pub unsafe fn ExtractMin(&mut self) -> *mut MinHeapNode {
        let mut out: *mut MinHeapNode = (*unimplemented!());
        self.size.prefix_dec();
        (*unimplemented!()) = (*unimplemented!());
        (unsafe { MinHeap::Heapify(self, 0_()) });
        return out;
    }
    pub unsafe fn Insert(&mut self, mut node: *mut MinHeapNode) {
        self.size.prefix_inc();
        let mut i: () = ((self.size) - (1_()));
        'loop_: while ((i) != (0_())) && (((*node).freq) < ((*(*unimplemented!())).freq)) {
            (*unimplemented!()) = (*unimplemented!());
            i = (((i) - (1_())) / (2_()));
        }
        (*unimplemented!()) = node;
    }
    pub unsafe fn Build(&mut self, data: *mut (), freq: *mut (), mut n: ()) {
        let mut i: () = 0_();
        'loop_: while ((i) < (n)) {
            (*unimplemented!()) = (unsafe {
                let _data: () = (*unimplemented!());
                let _freq: () = (*unimplemented!());
                MinHeap::Alloc(self, _data, _freq)
            });
            i.prefix_inc();
        }
        let mut i: () = (((self.size) - (2_())) / (2_()));
        'loop_: while ((i) >= (0_())) {
            (unsafe { MinHeap::Heapify(self, i) });
            i.prefix_dec();
        }
    }
    pub unsafe fn move_from(_a0: *mut MinHeap) -> Self {
        let mut this = Self {
            size: (*_a0).size,
            capacity: (*_a0).capacity,
            arr: unimplemented!(),
            next: (*_a0).next,
            alloc: unimplemented!(),
        };
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut MinHeap) -> *mut MinHeap {
        self.size = (*_a0).size;
        self.capacity = (*_a0).capacity;
        unimplemented!();
        self.next = (*_a0).next;
        unimplemented!();
        return &mut (*(self as *mut MinHeap));
    }
}
pub unsafe fn AllocMinHeap_1(mut capacity: ()) -> () {
    let mut minHeap: () = unimplemented!();
    return unimplemented!();
}
pub unsafe fn Huffman_2(data: *mut (), freq: *mut (), mut size: ()) -> () {
    let mut minHeap: () = (unsafe { AllocMinHeap_1(size) });
    (unsafe {
        let _data: *mut () = data;
        let _freq: *mut () = freq;
        let _n: () = size;
        MinHeap::Build(&mut (*unimplemented!()), _data, _freq, _n)
    });
    'loop_: while (((*unimplemented!()).size) != (1_())) {
        let mut left: *mut MinHeapNode = (unsafe { MinHeap::ExtractMin(&mut (*unimplemented!())) });
        let mut right: *mut MinHeapNode =
            (unsafe { MinHeap::ExtractMin(&mut (*unimplemented!())) });
        let mut top: *mut MinHeapNode = (unsafe {
            MinHeap::Alloc(
                &mut (*unimplemented!()),
                ('$' as ()),
                (((*left).freq) + ((*right).freq)),
            )
        });
        (*top).left = left;
        (*top).right = right;
        (unsafe { MinHeap::Insert(&mut (*unimplemented!()), top) });
    }
    return unimplemented!();
}
pub unsafe fn CollectCode_3(arr: *mut (), mut top: (), out: *mut (), next: *mut ()) {
    (*unimplemented!()) = 0_();
    let mut i: () = 0_();
    'loop_: while ((i) < (top)) {
        (*unimplemented!()) = ((*unimplemented!()) * (10_()));
        (*unimplemented!()) = ((*unimplemented!()) + (*unimplemented!()));
        i.prefix_inc();
    }
    (*next).prefix_inc();
}
pub unsafe fn CollectCodes_4(
    mut root: *mut MinHeapNode,
    arr: *mut (),
    mut top: (),
    out: *mut (),
    next: *mut (),
) {
    if !(((*root).left).is_null()) {
        (*unimplemented!()) = 0_();
        (unsafe {
            let _root: *mut MinHeapNode = (*root).left;
            let _arr: *mut () = arr;
            let _top: () = ((top) + (1_()));
            let _out: *mut () = out;
            let _next: *mut () = next;
            CollectCodes_4(_root, _arr, _top, _out, _next)
        });
    }
    if !(((*root).right).is_null()) {
        (*unimplemented!()) = 1_();
        (unsafe {
            let _root: *mut MinHeapNode = (*root).right;
            let _arr: *mut () = arr;
            let _top: () = ((top) + (1_()));
            let _out: *mut () = out;
            let _next: *mut () = next;
            CollectCodes_4(_root, _arr, _top, _out, _next)
        });
    }
    if (unsafe { MinHeapNode::IsLeaf(&(*(root).cast_const())) }) {
        (unsafe {
            let _arr: *mut () = arr;
            let _top: () = top;
            let _out: *mut () = out;
            let _next: *mut () = next;
            CollectCode_3(_arr, _top, _out, _next)
        });
    }
}
pub unsafe fn HuffmanCodes_5(data: *mut (), freq: *mut (), mut size: ()) -> () {
    let mut minHeap: () = (unsafe {
        let _data: *mut () = data;
        let _freq: *mut () = freq;
        let _size: () = size;
        Huffman_2(_data, _freq, _size)
    });
    let mut root: *mut MinHeapNode = (unsafe { MinHeap::ExtractMin(&mut (*unimplemented!())) });
    let mut arr: () = (unsafe { make_unique_6(100_()) });
    let mut out: () = (unsafe { make_unique_6(100_()) });
    let mut top: () = 0_();
    let mut next: () = 0_();
    (unsafe { CollectCodes_4(root, &mut arr, top, &mut out, &mut next) });
    return unimplemented!();
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut size: () = 6_();
    let mut arr1: [(); 6] = [
        ('a' as ()),
        ('b' as ()),
        ('c' as ()),
        ('d' as ()),
        ('e' as ()),
        ('f' as ()),
    ];
    let mut arr2: [(); 6] = [5_(), 9_(), 12_(), 13_(), 16_(), 45_()];
    let mut data: () = (unsafe { make_unique_7(size) });
    let mut freq: () = (unsafe { make_unique_6(size) });
    let mut i: () = 0_();
    'loop_: while ((i) < (size)) {
        (*unimplemented!()) = arr1[(i) as usize];
        (*unimplemented!()) = arr2[(i) as usize];
        i.prefix_inc();
    }
    let mut out: () = (unsafe { HuffmanCodes_5(&mut data, &mut freq, size) });
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
