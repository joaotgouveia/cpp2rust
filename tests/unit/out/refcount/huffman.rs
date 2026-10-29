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
pub struct MinHeapNode {
    #[offset(0)]
    pub data: (),
    #[offset(4)]
    pub freq: (),
    #[offset(8)]
    #[byte_size(8)]
    pub left: Ptr<MinHeapNode>,
    #[offset(16)]
    #[byte_size(8)]
    pub right: Ptr<MinHeapNode>,
}
pub fn Swap_0(a: Ptr<MinHeapNode>, b: Ptr<MinHeapNode>) {
    let mut t: MinHeapNode = MinHeapNode {
        data: a.with(|__s| __s.data),
        freq: a.with(|__s| __s.freq),
        left: a.with(|__s| __s.left.clone()),
        right: a.with(|__s| __s.right.clone()),
    };
    a.write({
        MinHeapNode {
            data: b.with(|__s| __s.data),
            freq: b.with(|__s| __s.freq),
            left: b.with(|__s| __s.left.clone()),
            right: b.with(|__s| __s.right.clone()),
        }
    });
    b.write({
        MinHeapNode {
            data: t.data,
            freq: t.freq,
            left: (t.left).clone(),
            right: (t.right).clone(),
        }
    });
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(32)]
pub struct MinHeap {
    #[offset(0)]
    pub size: (),
    #[offset(4)]
    pub capacity: (),
    #[offset(8)]
    #[byte_size(8)]
    pub arr: std_unique_ptr_MinHeapNode_ptrarrarr__std_default_delete_MinHeapNode_ptrarrarr__,
    #[offset(16)]
    pub next: (),
    #[offset(24)]
    #[byte_size(8)]
    pub alloc: std_unique_ptr_MinHeapNodearrarr__std_default_delete_MinHeapNodearrarr__,
}
impl MinHeap {
    pub fn move_from(_a0: Ptr<MinHeap>) -> Self {
        Self {
            size: { (*_a0.upgrade().deref()).size },
            capacity: { (*_a0.upgrade().deref()).capacity },
            arr: field!(_a0, arr).with_mut(std::mem::take),
            next: { (*_a0.upgrade().deref()).next },
            alloc: field!(_a0, alloc).with_mut(std::mem::take),
        }
    }
}
pub fn AllocMinHeap_1(mut capacity: ()) -> std_unique_ptr_MinHeap__std_default_delete_MinHeap__ {
    let minHeap: Value<std_unique_ptr_MinHeap__std_default_delete_MinHeap__> =
        Rc::new(RefCell::new(
            ({
                let ___args: Value<MinHeap> = Rc::new(RefCell::new(MinHeap {
                    size: 0_(),
                    capacity: capacity,
                    arr: ({ make_unique_2(capacity) }),
                    next: 0_(),
                    alloc: ({ make_unique_3(10000_()) }),
                }));
                make_unique_4(___args.as_pointer())
            }),
        ));
    return (*minHeap.borrow_mut()).clone();
}
pub fn Huffman_5(
    data: Ptr<std_unique_ptr_chararrarr__std_default_delete_chararrarr__>,
    freq: Ptr<std_unique_ptr_intarrarr__std_default_delete_intarrarr__>,
    mut size: (),
) -> std_unique_ptr_MinHeap__std_default_delete_MinHeap__ {
    let minHeap: Value<std_unique_ptr_MinHeap__std_default_delete_MinHeap__> =
        Rc::new(RefCell::new(({ AllocMinHeap_1(size) })));
    ({
        let _data: Ptr<std_unique_ptr_chararrarr__std_default_delete_chararrarr__> = (data).clone();
        let _freq: Ptr<std_unique_ptr_intarrarr__std_default_delete_intarrarr__> = (freq).clone();
        let _n: () = size;
        MinHeapImpl::Build(&(minHeap.as_pointer()), _data, _freq, _n)
    });
    'loop_: while ({ (*(*minHeap.borrow()).upgrade().deref()).size } != 1_()) {
        let mut left: Ptr<MinHeapNode> = ({ MinHeapImpl::ExtractMin(&(minHeap.as_pointer())) });
        let mut right: Ptr<MinHeapNode> = ({ MinHeapImpl::ExtractMin(&(minHeap.as_pointer())) });
        let mut top: Ptr<MinHeapNode> = ({
            MinHeapImpl::Alloc(
                &(minHeap.as_pointer()),
                ('$' as ()),
                ({ left.with(|__s| __s.freq) } + { right.with(|__s| __s.freq) }),
            )
        });
        field!(top, left).write((left).clone());
        field!(top, right).write((right).clone());
        ({ MinHeapImpl::Insert(&(minHeap.as_pointer()), (top).clone()) });
    }
    return (*minHeap.borrow_mut()).clone();
}
pub fn CollectCode_6(
    arr: Ptr<std_unique_ptr_intarrarr__std_default_delete_intarrarr__>,
    mut top: (),
    out: Ptr<std_unique_ptr_intarrarr__std_default_delete_intarrarr__>,
    next: Ptr<()>,
) {
    elem!(out, (next.read())).write(0_());
    let mut i: () = 0_();
    'loop_: while (i < top) {
        let __rhs = ((elem!(out, (next.read())).read()) * 10_());
        elem!(out, (next.read())).write(__rhs);
        let __rhs = ({ (elem!(out, (next.read())).read()) } + { (elem!(arr, i).read()) });
        elem!(out, (next.read())).write(__rhs);
        i.prefix_inc();
    }
    next.with_mut(|__v| __v.prefix_inc());
}
pub fn CollectCodes_7(
    mut root: Ptr<MinHeapNode>,
    arr: Ptr<std_unique_ptr_intarrarr__std_default_delete_intarrarr__>,
    mut top: (),
    out: Ptr<std_unique_ptr_intarrarr__std_default_delete_intarrarr__>,
    next: Ptr<()>,
) {
    if !((root.with(|__s| __s.left.clone())).is_null()) {
        elem!(arr, top).write(0_());
        ({
            let _root: Ptr<MinHeapNode> = root.with(|__s| __s.left.clone());
            let _arr: Ptr<std_unique_ptr_intarrarr__std_default_delete_intarrarr__> = (arr).clone();
            let _top: () = (top + 1_());
            let _out: Ptr<std_unique_ptr_intarrarr__std_default_delete_intarrarr__> = (out).clone();
            let _next: Ptr<()> = (next).clone();
            CollectCodes_7(_root, _arr, _top, _out, _next)
        });
    }
    if !((root.with(|__s| __s.right.clone())).is_null()) {
        elem!(arr, top).write(1_());
        ({
            let _root: Ptr<MinHeapNode> = root.with(|__s| __s.right.clone());
            let _arr: Ptr<std_unique_ptr_intarrarr__std_default_delete_intarrarr__> = (arr).clone();
            let _top: () = (top + 1_());
            let _out: Ptr<std_unique_ptr_intarrarr__std_default_delete_intarrarr__> = (out).clone();
            let _next: Ptr<()> = (next).clone();
            CollectCodes_7(_root, _arr, _top, _out, _next)
        });
    }
    if ({ MinHeapNodeImpl::IsLeaf(&root) }) {
        ({
            let _arr: Ptr<std_unique_ptr_intarrarr__std_default_delete_intarrarr__> = (arr).clone();
            let _top: () = top;
            let _out: Ptr<std_unique_ptr_intarrarr__std_default_delete_intarrarr__> = (out).clone();
            let _next: Ptr<()> = (next).clone();
            CollectCode_6(_arr, _top, _out, _next)
        });
    }
}
pub fn HuffmanCodes_8(
    data: Ptr<std_unique_ptr_chararrarr__std_default_delete_chararrarr__>,
    freq: Ptr<std_unique_ptr_intarrarr__std_default_delete_intarrarr__>,
    mut size: (),
) -> std_unique_ptr_intarrarr__std_default_delete_intarrarr__ {
    let minHeap: Value<std_unique_ptr_MinHeap__std_default_delete_MinHeap__> =
        Rc::new(RefCell::new(
            ({
                let _data: Ptr<std_unique_ptr_chararrarr__std_default_delete_chararrarr__> =
                    (data).clone();
                let _freq: Ptr<std_unique_ptr_intarrarr__std_default_delete_intarrarr__> =
                    (freq).clone();
                let _size: () = size;
                Huffman_5(_data, _freq, _size)
            }),
        ));
    let mut root: Ptr<MinHeapNode> = ({ MinHeapImpl::ExtractMin(&(minHeap.as_pointer())) });
    let arr: Value<std_unique_ptr_intarrarr__std_default_delete_intarrarr__> =
        Rc::new(RefCell::new(({ make_unique_9(100_()) })));
    let out: Value<std_unique_ptr_intarrarr__std_default_delete_intarrarr__> =
        Rc::new(RefCell::new(({ make_unique_9(100_()) })));
    let mut top: () = 0_();
    let next: Value<()> = Rc::new(RefCell::new(0_()));
    ({
        CollectCodes_7(
            (root).clone(),
            arr.as_pointer(),
            top,
            out.as_pointer(),
            next.as_pointer(),
        )
    });
    return (*out.borrow_mut()).clone();
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
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
    let data: Value<std_unique_ptr_chararrarr__std_default_delete_chararrarr__> =
        Rc::new(RefCell::new(({ make_unique_10(size) })));
    let freq: Value<std_unique_ptr_intarrarr__std_default_delete_intarrarr__> =
        Rc::new(RefCell::new(({ make_unique_9(size) })));
    let mut i: () = 0_();
    'loop_: while (i < size) {
        let __rhs = arr1[(i) as usize];
        (*data.borrow_mut())[i] = __rhs;
        let __rhs = arr2[(i) as usize];
        (*freq.borrow_mut())[i] = __rhs;
        i.prefix_inc();
    }
    let out: Value<std_unique_ptr_intarrarr__std_default_delete_intarrarr__> = Rc::new(
        RefCell::new(({ HuffmanCodes_8(data.as_pointer(), freq.as_pointer(), size) })),
    );
    unimplemented!();
    return 0_();
}
pub trait MinHeapImpl {
    fn Alloc(&self, data: (), freq: ()) -> Ptr<MinHeapNode>;
    fn Heapify(&self, idx: ());
    fn ExtractMin(&self) -> Ptr<MinHeapNode>;
    fn Insert(&self, node: Ptr<MinHeapNode>);
    fn Build(
        &self,
        data: Ptr<std_unique_ptr_chararrarr__std_default_delete_chararrarr__>,
        freq: Ptr<std_unique_ptr_intarrarr__std_default_delete_intarrarr__>,
        n: (),
    );
    fn move_assign(&self, _a0: Ptr<MinHeap>) -> Ptr<MinHeap>;
}
impl MinHeapImpl for Ptr<MinHeap> {
    fn Alloc(&self, mut data: (), mut freq: ()) -> Ptr<MinHeapNode> {
        elem!(field_ptr!((*self), alloc), (*self).with(|__s| __s.next)).write(MinHeapNode {
            data: data,
            freq: freq,
            left: Ptr::<MinHeapNode>::null(),
            right: Ptr::<MinHeapNode>::null(),
        });
        return (field_ptr!((*self), alloc)
            .offset(field!((*self), next).with_mut(|__v| __v.postfix_inc())));
    }
    fn Heapify(&self, mut idx: ()) {
        let mut smallest: () = idx;
        let mut left: () = ((2_() * idx) + 1_());
        let mut right: () = ((2_() * idx) + 2_());
        if (left < (*self).with(|__s| __s.size))
            && ((elem!(field_ptr!((*self), arr), left).read()).with(|__s| __s.freq)
                < (elem!(field_ptr!((*self), arr), smallest).read()).with(|__s| __s.freq))
        {
            smallest = left;
        }
        if (right < (*self).with(|__s| __s.size))
            && ((elem!(field_ptr!((*self), arr), right).read()).with(|__s| __s.freq)
                < (elem!(field_ptr!((*self), arr), smallest).read()).with(|__s| __s.freq))
        {
            smallest = right;
        }
        if (smallest != idx) {
            ({
                let _a: Ptr<MinHeapNode> = (elem!(field_ptr!((*self), arr), smallest).read());
                let _b: Ptr<MinHeapNode> = (elem!(field_ptr!((*self), arr), idx).read());
                Swap_0(_a, _b)
            });
            ({ MinHeapImpl::Heapify(self, smallest) });
        }
    }
    fn ExtractMin(&self) -> Ptr<MinHeapNode> {
        let mut out: Ptr<MinHeapNode> = (elem!(field_ptr!((*self), arr), 0_()).read());
        field!((*self), size).with_mut(|__v| __v.prefix_dec());
        let __rhs = (elem!(field_ptr!((*self), arr), (*self).with(|__s| __s.size)).read());
        elem!(field_ptr!((*self), arr), 0_()).write(__rhs);
        ({ MinHeapImpl::Heapify(self, 0_()) });
        return out;
    }
    fn Insert(&self, mut node: Ptr<MinHeapNode>) {
        field!((*self), size).with_mut(|__v| __v.prefix_inc());
        let mut i: () = ((*self).with(|__s| __s.size) - 1_());
        'loop_: while (i != 0_())
            && ({ node.with(|__s| __s.freq) } < {
                (elem!(field_ptr!((*self), arr), ((i - 1_()) / 2_())).read()).with(|__s| __s.freq)
            })
        {
            let __rhs = (elem!(field_ptr!((*self), arr), ((i - 1_()) / 2_())).read());
            elem!(field_ptr!((*self), arr), i).write(__rhs);
            i = { ((i - 1_()) / 2_()) };
        }
        elem!(field_ptr!((*self), arr), i).write((node).clone());
    }
    fn Build(
        &self,
        data: Ptr<std_unique_ptr_chararrarr__std_default_delete_chararrarr__>,
        freq: Ptr<std_unique_ptr_intarrarr__std_default_delete_intarrarr__>,
        mut n: (),
    ) {
        let mut i: () = 0_();
        'loop_: while (i < n) {
            elem!(
                field_ptr!((*self), arr),
                field!((*self), size).with_mut(|__v| __v.postfix_inc())
            )
            .write(
                ({
                    let _data: () = (elem!(data, i).read());
                    let _freq: () = (elem!(freq, i).read());
                    MinHeapImpl::Alloc(self, _data, _freq)
                }),
            );
            i.prefix_inc();
        }
        let mut i: () = (((*self).with(|__s| __s.size) - 2_()) / 2_());
        'loop_: while (i >= 0_()) {
            ({ MinHeapImpl::Heapify(self, i) });
            i.prefix_dec();
        }
    }
    fn move_assign(&self, _a0: Ptr<MinHeap>) -> Ptr<MinHeap> {
        field!((*self), size).write({ { (*_a0.upgrade().deref()).size } });
        field!((*self), capacity).write({ { (*_a0.upgrade().deref()).capacity } });
        field!((*self), arr).write({ { (*_a0.upgrade().deref()).arr.clone() } });
        field!((*self), next).write({ { (*_a0.upgrade().deref()).next } });
        field!((*self), alloc).write({ { (*_a0.upgrade().deref()).alloc.clone() } });
        return (*self).clone();
    }
}
pub trait MinHeapNodeImpl {
    fn IsLeaf(&self) -> ();
}
impl MinHeapNodeImpl for Ptr<MinHeapNode> {
    fn IsLeaf(&self) -> () {
        return (((*self).with(|__s| __s.left.clone())).is_null())
            && (((*self).with(|__s| __s.right.clone())).is_null());
    }
}
pub fn __cpp2rust_init_globals() {}
