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
    pub arr: std_unique_ptr_MinHeapNode_ptrarrarr__std_default_delete_MinHeapNode_ptrarrarr__,
    pub next: (),
    pub alloc: std_unique_ptr_MinHeapNodearrarr__std_default_delete_MinHeapNodearrarr__,
}
impl MinHeap {
    pub unsafe fn Alloc(&mut self, mut data: (), mut freq: ()) -> *mut MinHeapNode {
        self.alloc[(self.next) as usize] = MinHeapNode {
            data: data,
            freq: freq,
            left: std::ptr::null_mut(),
            right: std::ptr::null_mut(),
        };
        return (&mut self.alloc[(self.next.postfix_inc()) as usize] as *mut MinHeapNode);
    }
    pub unsafe fn Heapify(&mut self, mut idx: ()) {
        let mut smallest: () = idx;
        let mut left: () = (((2_()) * (idx)) + (1_()));
        let mut right: () = (((2_()) * (idx)) + (2_()));
        if ((left) < (self.size))
            && (((*self.arr[(left) as usize]).freq) < ((*self.arr[(smallest) as usize]).freq))
        {
            smallest = left;
        }
        if ((right) < (self.size))
            && (((*self.arr[(right) as usize]).freq) < ((*self.arr[(smallest) as usize]).freq))
        {
            smallest = right;
        }
        if ((smallest) != (idx)) {
            (unsafe {
                let _a: *mut MinHeapNode = &mut (*self.arr[(smallest) as usize]);
                let _b: *mut MinHeapNode = &mut (*self.arr[(idx) as usize]);
                Swap_0(_a, _b)
            });
            (unsafe { MinHeap::Heapify(self, smallest) });
        }
    }
    pub unsafe fn ExtractMin(&mut self) -> *mut MinHeapNode {
        let mut out: *mut MinHeapNode = self.arr[(0_()) as usize];
        self.size.prefix_dec();
        self.arr[(0_()) as usize] = self.arr[(self.size) as usize];
        (unsafe { MinHeap::Heapify(self, 0_()) });
        return out;
    }
    pub unsafe fn Insert(&mut self, mut node: *mut MinHeapNode) {
        self.size.prefix_inc();
        let mut i: () = ((self.size) - (1_()));
        'loop_: while ((i) != (0_()))
            && (((*node).freq) < ((*self.arr[(((i) - (1_())) / (2_())) as usize]).freq))
        {
            self.arr[(i) as usize] = self.arr[(((i) - (1_())) / (2_())) as usize];
            i = (((i) - (1_())) / (2_()));
        }
        self.arr[(i) as usize] = node;
    }
    pub unsafe fn Build(
        &mut self,
        data: *mut std_unique_ptr_chararrarr__std_default_delete_chararrarr__,
        freq: *mut std_unique_ptr_intarrarr__std_default_delete_intarrarr__,
        mut n: (),
    ) {
        let mut i: () = 0_();
        'loop_: while ((i) < (n)) {
            self.arr[(self.size.postfix_inc()) as usize] = (unsafe {
                let _data: () = (&(*data))[(i) as usize];
                let _freq: () = (&(*freq))[(i) as usize];
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
            arr: (*_a0).arr,
            next: (*_a0).next,
            alloc: (*_a0).alloc,
        };
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut MinHeap) -> *mut MinHeap {
        self.size = (*_a0).size;
        self.capacity = (*_a0).capacity;
        self.arr = ((*_a0).arr).clone();
        self.next = (*_a0).next;
        self.alloc = ((*_a0).alloc).clone();
        return &mut (*(self as *mut MinHeap));
    }
}
pub unsafe fn AllocMinHeap_1(
    mut capacity: (),
) -> std_unique_ptr_MinHeap__std_default_delete_MinHeap__ {
    let mut minHeap: std_unique_ptr_MinHeap__std_default_delete_MinHeap__ = (unsafe {
        let mut ___args: MinHeap = MinHeap {
            size: 0_(),
            capacity: capacity,
            arr: (unsafe { make_unique_2(capacity) }),
            next: 0_(),
            alloc: (unsafe { make_unique_3(10000_()) }),
        };
        make_unique_4(&mut ___args)
    });
    return minHeap;
}
pub unsafe fn Huffman_5(
    data: *mut std_unique_ptr_chararrarr__std_default_delete_chararrarr__,
    freq: *mut std_unique_ptr_intarrarr__std_default_delete_intarrarr__,
    mut size: (),
) -> std_unique_ptr_MinHeap__std_default_delete_MinHeap__ {
    let mut minHeap: std_unique_ptr_MinHeap__std_default_delete_MinHeap__ =
        (unsafe { AllocMinHeap_1(size) });
    (unsafe {
        let _data: *mut std_unique_ptr_chararrarr__std_default_delete_chararrarr__ = data;
        let _freq: *mut std_unique_ptr_intarrarr__std_default_delete_intarrarr__ = freq;
        let _n: () = size;
        MinHeap::Build(&mut (*minHeap), _data, _freq, _n)
    });
    'loop_: while (((*minHeap).size) != (1_())) {
        let mut left: *mut MinHeapNode = (unsafe { MinHeap::ExtractMin(&mut (*minHeap)) });
        let mut right: *mut MinHeapNode = (unsafe { MinHeap::ExtractMin(&mut (*minHeap)) });
        let mut top: *mut MinHeapNode = (unsafe {
            MinHeap::Alloc(
                &mut (*minHeap),
                ('$' as ()),
                (((*left).freq) + ((*right).freq)),
            )
        });
        (*top).left = left;
        (*top).right = right;
        (unsafe { MinHeap::Insert(&mut (*minHeap), top) });
    }
    return minHeap;
}
pub unsafe fn CollectCode_6(
    arr: *mut std_unique_ptr_intarrarr__std_default_delete_intarrarr__,
    mut top: (),
    out: *mut std_unique_ptr_intarrarr__std_default_delete_intarrarr__,
    next: *mut (),
) {
    (&(*out))[(*next) as usize] = 0_();
    let mut i: () = 0_();
    'loop_: while ((i) < (top)) {
        (&(*out))[(*next) as usize] = (((&(*out))[(*next) as usize]) * (10_()));
        (&(*out))[(*next) as usize] = (((&(*out))[(*next) as usize]) + ((&(*arr))[(i) as usize]));
        i.prefix_inc();
    }
    (*next).prefix_inc();
}
pub unsafe fn CollectCodes_7(
    mut root: *mut MinHeapNode,
    arr: *mut std_unique_ptr_intarrarr__std_default_delete_intarrarr__,
    mut top: (),
    out: *mut std_unique_ptr_intarrarr__std_default_delete_intarrarr__,
    next: *mut (),
) {
    if !(((*root).left).is_null()) {
        (&(*arr))[(top) as usize] = 0_();
        (unsafe {
            let _root: *mut MinHeapNode = (*root).left;
            let _arr: *mut std_unique_ptr_intarrarr__std_default_delete_intarrarr__ = arr;
            let _top: () = ((top) + (1_()));
            let _out: *mut std_unique_ptr_intarrarr__std_default_delete_intarrarr__ = out;
            let _next: *mut () = next;
            CollectCodes_7(_root, _arr, _top, _out, _next)
        });
    }
    if !(((*root).right).is_null()) {
        (&(*arr))[(top) as usize] = 1_();
        (unsafe {
            let _root: *mut MinHeapNode = (*root).right;
            let _arr: *mut std_unique_ptr_intarrarr__std_default_delete_intarrarr__ = arr;
            let _top: () = ((top) + (1_()));
            let _out: *mut std_unique_ptr_intarrarr__std_default_delete_intarrarr__ = out;
            let _next: *mut () = next;
            CollectCodes_7(_root, _arr, _top, _out, _next)
        });
    }
    if (unsafe { MinHeapNode::IsLeaf(&(*(root).cast_const())) }) {
        (unsafe {
            let _arr: *mut std_unique_ptr_intarrarr__std_default_delete_intarrarr__ = arr;
            let _top: () = top;
            let _out: *mut std_unique_ptr_intarrarr__std_default_delete_intarrarr__ = out;
            let _next: *mut () = next;
            CollectCode_6(_arr, _top, _out, _next)
        });
    }
}
pub unsafe fn HuffmanCodes_8(
    data: *mut std_unique_ptr_chararrarr__std_default_delete_chararrarr__,
    freq: *mut std_unique_ptr_intarrarr__std_default_delete_intarrarr__,
    mut size: (),
) -> std_unique_ptr_intarrarr__std_default_delete_intarrarr__ {
    let mut minHeap: std_unique_ptr_MinHeap__std_default_delete_MinHeap__ = (unsafe {
        let _data: *mut std_unique_ptr_chararrarr__std_default_delete_chararrarr__ = data;
        let _freq: *mut std_unique_ptr_intarrarr__std_default_delete_intarrarr__ = freq;
        let _size: () = size;
        Huffman_5(_data, _freq, _size)
    });
    let mut root: *mut MinHeapNode = (unsafe { MinHeap::ExtractMin(&mut (*minHeap)) });
    let mut arr: std_unique_ptr_intarrarr__std_default_delete_intarrarr__ =
        (unsafe { make_unique_9(100_()) });
    let mut out: std_unique_ptr_intarrarr__std_default_delete_intarrarr__ =
        (unsafe { make_unique_9(100_()) });
    let mut top: () = 0_();
    let mut next: () = 0_();
    (unsafe { CollectCodes_7(root, &mut arr, top, &mut out, &mut next) });
    return out;
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
    let mut data: std_unique_ptr_chararrarr__std_default_delete_chararrarr__ =
        (unsafe { make_unique_10(size) });
    let mut freq: std_unique_ptr_intarrarr__std_default_delete_intarrarr__ =
        (unsafe { make_unique_9(size) });
    let mut i: () = 0_();
    'loop_: while ((i) < (size)) {
        data[(i) as usize] = arr1[(i) as usize];
        freq[(i) as usize] = arr2[(i) as usize];
        i.prefix_inc();
    }
    let mut out: std_unique_ptr_intarrarr__std_default_delete_intarrarr__ =
        (unsafe { HuffmanCodes_8(&mut data, &mut freq, size) });
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
