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
    pub data: i8,
    #[offset(4)]
    pub freq: i32,
    #[offset(8)]
    #[byte_size(8)]
    pub left: Ptr<MinHeapNode>,
    #[offset(16)]
    #[byte_size(8)]
    pub right: Ptr<MinHeapNode>,
}
impl CppMove for MinHeapNode {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
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
    pub size: i32,
    #[offset(4)]
    pub capacity: i32,
    #[offset(8)]
    #[byte_size(8)]
    pub arr: std_unique_ptr_MinHeapNode_ptrarrarr__std_default_delete_MinHeapNode_ptrarrarr__,
    #[offset(16)]
    pub next: i32,
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
impl CppMove for MinHeap {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
pub fn AllocMinHeap_1(mut capacity: i32) -> std_unique_ptr_MinHeap__std_default_delete_MinHeap__ {
    let minHeap: Value<std_unique_ptr_MinHeap__std_default_delete_MinHeap__> =
        Rc::new(RefCell::new(
            ({
                let ___args: Value<MinHeap> = Rc::new(RefCell::new(MinHeap {
                    size: 0,
                    capacity: capacity,
                    arr: ({ make_unique_2((capacity as u64)) }),
                    next: 0,
                    alloc: ({ make_unique_3(10000_u64) }),
                }));
                make_unique_4(___args.as_pointer())
            }),
        ));
    return (*minHeap.borrow_mut()).clone();
}
pub fn Huffman_5(
    data: Ptr<std_unique_ptr_chararrarr__std_default_delete_chararrarr__>,
    freq: Ptr<std_unique_ptr_intarrarr__std_default_delete_intarrarr__>,
    mut size: i32,
) -> std_unique_ptr_MinHeap__std_default_delete_MinHeap__ {
    let minHeap: Value<std_unique_ptr_MinHeap__std_default_delete_MinHeap__> =
        Rc::new(RefCell::new(({ AllocMinHeap_1(size) })));
    ({
        let _data: Ptr<std_unique_ptr_chararrarr__std_default_delete_chararrarr__> = (data).clone();
        let _freq: Ptr<std_unique_ptr_intarrarr__std_default_delete_intarrarr__> = (freq).clone();
        let _n: i32 = size;
        MinHeapImpl::Build(&(minHeap.as_pointer()), _data, _freq, _n)
    });
    'loop_: while ({ (*(*minHeap.borrow()).upgrade().deref()).size } != 1) {
        let mut left: Ptr<MinHeapNode> = ({ MinHeapImpl::ExtractMin(&(minHeap.as_pointer())) });
        let mut right: Ptr<MinHeapNode> = ({ MinHeapImpl::ExtractMin(&(minHeap.as_pointer())) });
        let mut top: Ptr<MinHeapNode> = ({
            MinHeapImpl::Alloc(
                &(minHeap.as_pointer()),
                ('$' as i8),
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
    mut top: i32,
    out: Ptr<std_unique_ptr_intarrarr__std_default_delete_intarrarr__>,
    next: Ptr<i32>,
) {
    elem!(out, ((next.read()) as u64)).write(0);
    let mut i: i32 = 0;
    'loop_: while (i < top) {
        let __rhs = ((elem!(out, ((next.read()) as u64)).read()) * 10);
        elem!(out, ((next.read()) as u64)).write(__rhs);
        let __rhs =
            ({ (elem!(out, ((next.read()) as u64)).read()) } + { (elem!(arr, (i as u64)).read()) });
        elem!(out, ((next.read()) as u64)).write(__rhs);
        i.prefix_inc();
    }
    next.with_mut(|__v| {
        __v.prefix_inc();
    });
}
pub fn CollectCodes_7(
    mut root: Ptr<MinHeapNode>,
    arr: Ptr<std_unique_ptr_intarrarr__std_default_delete_intarrarr__>,
    mut top: i32,
    out: Ptr<std_unique_ptr_intarrarr__std_default_delete_intarrarr__>,
    next: Ptr<i32>,
) {
    if !((root.with(|__s| __s.left.clone())).is_null()) {
        elem!(arr, (top as u64)).write(0);
        ({
            let _root: Ptr<MinHeapNode> = root.with(|__s| __s.left.clone());
            let _arr: Ptr<std_unique_ptr_intarrarr__std_default_delete_intarrarr__> = (arr).clone();
            let _top: i32 = (top + 1);
            let _out: Ptr<std_unique_ptr_intarrarr__std_default_delete_intarrarr__> = (out).clone();
            let _next: Ptr<i32> = (next).clone();
            CollectCodes_7(_root, _arr, _top, _out, _next)
        });
    }
    if !((root.with(|__s| __s.right.clone())).is_null()) {
        elem!(arr, (top as u64)).write(1);
        ({
            let _root: Ptr<MinHeapNode> = root.with(|__s| __s.right.clone());
            let _arr: Ptr<std_unique_ptr_intarrarr__std_default_delete_intarrarr__> = (arr).clone();
            let _top: i32 = (top + 1);
            let _out: Ptr<std_unique_ptr_intarrarr__std_default_delete_intarrarr__> = (out).clone();
            let _next: Ptr<i32> = (next).clone();
            CollectCodes_7(_root, _arr, _top, _out, _next)
        });
    }
    if ({ MinHeapNodeImpl::IsLeaf(&root) }) {
        ({
            let _arr: Ptr<std_unique_ptr_intarrarr__std_default_delete_intarrarr__> = (arr).clone();
            let _top: i32 = top;
            let _out: Ptr<std_unique_ptr_intarrarr__std_default_delete_intarrarr__> = (out).clone();
            let _next: Ptr<i32> = (next).clone();
            CollectCode_6(_arr, _top, _out, _next)
        });
    }
}
pub fn HuffmanCodes_8(
    data: Ptr<std_unique_ptr_chararrarr__std_default_delete_chararrarr__>,
    freq: Ptr<std_unique_ptr_intarrarr__std_default_delete_intarrarr__>,
    mut size: i32,
) -> std_unique_ptr_intarrarr__std_default_delete_intarrarr__ {
    let minHeap: Value<std_unique_ptr_MinHeap__std_default_delete_MinHeap__> =
        Rc::new(RefCell::new(
            ({
                let _data: Ptr<std_unique_ptr_chararrarr__std_default_delete_chararrarr__> =
                    (data).clone();
                let _freq: Ptr<std_unique_ptr_intarrarr__std_default_delete_intarrarr__> =
                    (freq).clone();
                let _size: i32 = size;
                Huffman_5(_data, _freq, _size)
            }),
        ));
    let mut root: Ptr<MinHeapNode> = ({ MinHeapImpl::ExtractMin(&(minHeap.as_pointer())) });
    let arr: Value<std_unique_ptr_intarrarr__std_default_delete_intarrarr__> =
        Rc::new(RefCell::new(({ make_unique_9(100_u64) })));
    let out: Value<std_unique_ptr_intarrarr__std_default_delete_intarrarr__> =
        Rc::new(RefCell::new(({ make_unique_9(100_u64) })));
    let mut top: i32 = 0;
    let next: Value<i32> = Rc::new(RefCell::new(0));
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
fn main_0() -> i32 {
    let mut size: i32 = 6;
    let mut arr1: [i8; 6] = [
        ('a' as i8),
        ('b' as i8),
        ('c' as i8),
        ('d' as i8),
        ('e' as i8),
        ('f' as i8),
    ];
    let mut arr2: [i32; 6] = [5, 9, 12, 13, 16, 45];
    let data: Value<std_unique_ptr_chararrarr__std_default_delete_chararrarr__> =
        Rc::new(RefCell::new(({ make_unique_10((size as u64)) })));
    let freq: Value<std_unique_ptr_intarrarr__std_default_delete_intarrarr__> =
        Rc::new(RefCell::new(({ make_unique_9((size as u64)) })));
    let mut i: i32 = 0;
    'loop_: while (i < size) {
        let __rhs = arr1[(i) as usize];
        (*data.borrow_mut())[(i as u64)] = __rhs;
        let __rhs = arr2[(i) as usize];
        (*freq.borrow_mut())[(i as u64)] = __rhs;
        i.prefix_inc();
    }
    let out: Value<std_unique_ptr_intarrarr__std_default_delete_intarrarr__> = Rc::new(
        RefCell::new(({ HuffmanCodes_8(data.as_pointer(), freq.as_pointer(), size) })),
    );
    ({
        cpp2rust_assert_fail_11(
            ((((({ (*out.borrow())[0_u64] } == 0) && ({ (*out.borrow())[1_u64] } == 100))
                && ({ (*out.borrow())[2_u64] } == 101))
                && ({ (*out.borrow())[3_u64] } == 1100))
                && ({ (*out.borrow())[4_u64] } == 1101))
                && ({ (*out.borrow())[5_u64] } == 111),
        )
    });
    return 0;
}
pub trait MinHeapImpl {
    fn Alloc(&self, data: i8, freq: i32) -> Ptr<MinHeapNode>;
    fn Heapify(&self, idx: i32);
    fn ExtractMin(&self) -> Ptr<MinHeapNode>;
    fn Insert(&self, node: Ptr<MinHeapNode>);
    fn Build(
        &self,
        data: Ptr<std_unique_ptr_chararrarr__std_default_delete_chararrarr__>,
        freq: Ptr<std_unique_ptr_intarrarr__std_default_delete_intarrarr__>,
        n: i32,
    );
    fn move_assign(&self, _a0: Ptr<MinHeap>) -> Ptr<MinHeap>;
}
impl MinHeapImpl for Ptr<MinHeap> {
    fn Alloc(&self, mut data: i8, mut freq: i32) -> Ptr<MinHeapNode> {
        elem!(
            field_ptr!((*self), alloc),
            ((*self).with(|__s| __s.next) as u64)
        )
        .write(MinHeapNode {
            data: data,
            freq: freq,
            left: Ptr::<MinHeapNode>::null(),
            right: Ptr::<MinHeapNode>::null(),
        });
        return (field_ptr!((*self), alloc)
            .offset((field!((*self), next).with_mut(|__v| __v.postfix_inc()) as u64)));
    }
    fn Heapify(&self, mut idx: i32) {
        let mut smallest: i32 = idx;
        let mut left: i32 = ((2 * idx) + 1);
        let mut right: i32 = ((2 * idx) + 2);
        if (left < (*self).with(|__s| __s.size))
            && ((elem!(field_ptr!((*self), arr), (left as u64)).read()).with(|__s| __s.freq)
                < (elem!(field_ptr!((*self), arr), (smallest as u64)).read()).with(|__s| __s.freq))
        {
            smallest = left;
        }
        if (right < (*self).with(|__s| __s.size))
            && ((elem!(field_ptr!((*self), arr), (right as u64)).read()).with(|__s| __s.freq)
                < (elem!(field_ptr!((*self), arr), (smallest as u64)).read()).with(|__s| __s.freq))
        {
            smallest = right;
        }
        if (smallest != idx) {
            ({
                let _a: Ptr<MinHeapNode> =
                    (elem!(field_ptr!((*self), arr), (smallest as u64)).read());
                let _b: Ptr<MinHeapNode> = (elem!(field_ptr!((*self), arr), (idx as u64)).read());
                Swap_0(_a, _b)
            });
            ({ MinHeapImpl::Heapify(self, smallest) });
        }
    }
    fn ExtractMin(&self) -> Ptr<MinHeapNode> {
        let out: Value<Ptr<MinHeapNode>> = Rc::new(RefCell::new(
            (elem!(field_ptr!((*self), arr), 0_u64).read()),
        ));
        field!((*self), size).with_mut(|__v| {
            __v.prefix_dec();
        });
        let __rhs = (elem!(
            field_ptr!((*self), arr),
            ((*self).with(|__s| __s.size) as u64)
        )
        .read());
        elem!(field_ptr!((*self), arr), 0_u64).write(__rhs);
        ({ MinHeapImpl::Heapify(self, 0) });
        return (*out.borrow()).clone();
    }
    fn Insert(&self, mut node: Ptr<MinHeapNode>) {
        field!((*self), size).with_mut(|__v| {
            __v.prefix_inc();
        });
        let mut i: i32 = ((*self).with(|__s| __s.size) - 1);
        'loop_: while (i != 0)
            && ({ node.with(|__s| __s.freq) } < {
                (elem!(field_ptr!((*self), arr), (((i - 1) / 2) as u64)).read())
                    .with(|__s| __s.freq)
            })
        {
            let __rhs = (elem!(field_ptr!((*self), arr), (((i - 1) / 2) as u64)).read());
            elem!(field_ptr!((*self), arr), (i as u64)).write(__rhs);
            i = { ((i - 1) / 2) };
        }
        elem!(field_ptr!((*self), arr), (i as u64)).write((node).clone());
    }
    fn Build(
        &self,
        data: Ptr<std_unique_ptr_chararrarr__std_default_delete_chararrarr__>,
        freq: Ptr<std_unique_ptr_intarrarr__std_default_delete_intarrarr__>,
        mut n: i32,
    ) {
        let mut i: i32 = 0;
        'loop_: while (i < n) {
            elem!(
                field_ptr!((*self), arr),
                (field!((*self), size).with_mut(|__v| __v.postfix_inc()) as u64)
            )
            .write(
                ({
                    let _data: i8 = (elem!(data, (i as u64)).read());
                    let _freq: i32 = (elem!(freq, (i as u64)).read());
                    MinHeapImpl::Alloc(self, _data, _freq)
                }),
            );
            i.prefix_inc();
        }
        let mut i: i32 = (((*self).with(|__s| __s.size) - 2) / 2);
        'loop_: while (i >= 0) {
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
    fn IsLeaf(&self) -> bool;
}
impl MinHeapNodeImpl for Ptr<MinHeapNode> {
    fn IsLeaf(&self) -> bool {
        return (((*self).with(|__s| __s.left.clone())).is_null())
            && (((*self).with(|__s| __s.right.clone())).is_null());
    }
}
pub fn __cpp2rust_init_globals() {}
