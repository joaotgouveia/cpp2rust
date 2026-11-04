extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct Edge {
    #[offset(0)]
    pub u: i32,
    #[offset(4)]
    pub v: i32,
    #[offset(8)]
    pub weight: f64,
}
impl CppMove for Edge {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn partition_0(
    arr: Ptr<std_unique_ptr_Edgearrarr__std_default_delete_Edgearrarr__>,
    mut start: i32,
    mut end: i32,
) -> i32 {
    let pivot: Ptr<Edge> = arr.offset((start as u64));
    let mut count: i32 = 0;
    let mut i: i32 = (start + 1);
    'loop_: while (i <= end) {
        if ({ elem!(arr, (i as u64)).with(|__s| __s.weight) } <= { pivot.with(|__s| __s.weight) }) {
            count.postfix_inc();
        }
        i.prefix_inc();
    }
    let pidx: Value<i32> = Rc::new(RefCell::new((start + count)));
    let mut tmp: Edge = Edge {
        u: elem!(arr, ((*pidx.borrow()) as u64)).with(|__s| __s.u),
        v: elem!(arr, ((*pidx.borrow()) as u64)).with(|__s| __s.v),
        weight: elem!(arr, ((*pidx.borrow()) as u64)).with(|__s| __s.weight),
    };
    let __rhs = Edge {
        u: elem!(arr, (start as u64)).with(|__s| __s.u),
        v: elem!(arr, (start as u64)).with(|__s| __s.v),
        weight: elem!(arr, (start as u64)).with(|__s| __s.weight),
    };
    elem!(arr, ((*pidx.borrow()) as u64)).write(__rhs);
    elem!(arr, (start as u64)).write(Edge {
        u: tmp.u,
        v: tmp.v,
        weight: tmp.weight,
    });
    let mut i: i32 = start;
    let mut j: i32 = end;
    'loop_: while (i < (*pidx.borrow())) && (j > (*pidx.borrow())) {
        'loop_: while ({ elem!(arr, (i as u64)).with(|__s| __s.weight) } <= {
            pivot.with(|__s| __s.weight)
        }) {
            i.prefix_inc();
        }
        'loop_: while ({ elem!(arr, (j as u64)).with(|__s| __s.weight) } > {
            pivot.with(|__s| __s.weight)
        }) {
            j.prefix_dec();
        }
        if (i < (*pidx.borrow())) && (j > (*pidx.borrow())) {
            tmp = Edge {
                u: elem!(arr, (i as u64)).with(|__s| __s.u),
                v: elem!(arr, (i as u64)).with(|__s| __s.v),
                weight: elem!(arr, (i as u64)).with(|__s| __s.weight),
            };
            let __rhs = Edge {
                u: elem!(arr, (j as u64)).with(|__s| __s.u),
                v: elem!(arr, (j as u64)).with(|__s| __s.v),
                weight: elem!(arr, (j as u64)).with(|__s| __s.weight),
            };
            elem!(arr, (i as u64)).write(__rhs);
            elem!(arr, (j as u64)).write(Edge {
                u: tmp.u,
                v: tmp.v,
                weight: tmp.weight,
            });
            i.postfix_inc();
            j.postfix_dec();
        }
    }
    return (*pidx.borrow());
}
pub fn quicksort_1(
    arr: Ptr<std_unique_ptr_Edgearrarr__std_default_delete_Edgearrarr__>,
    mut start: i32,
    mut end: i32,
) {
    if (start >= end) {
        return;
    }
    let mut p: i32 = ({
        let _arr: Ptr<std_unique_ptr_Edgearrarr__std_default_delete_Edgearrarr__> = (arr).clone();
        let _start: i32 = start;
        let _end: i32 = end;
        partition_0(_arr, _start, _end)
    });
    ({
        let _arr: Ptr<std_unique_ptr_Edgearrarr__std_default_delete_Edgearrarr__> = (arr).clone();
        let _start: i32 = start;
        let _end: i32 = (p - 1);
        quicksort_1(_arr, _start, _end)
    });
    ({
        let _arr: Ptr<std_unique_ptr_Edgearrarr__std_default_delete_Edgearrarr__> = (arr).clone();
        let _start: i32 = (p + 1);
        let _end: i32 = end;
        quicksort_1(_arr, _start, _end)
    });
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(24)]
pub struct DisjointSet {
    #[offset(0)]
    #[byte_size(8)]
    pub rank: std_unique_ptr_intarrarr__std_default_delete_intarrarr__,
    #[offset(8)]
    #[byte_size(8)]
    pub parent: std_unique_ptr_intarrarr__std_default_delete_intarrarr__,
    #[offset(16)]
    pub n: i32,
}
impl DisjointSet {
    pub fn move_from(_a0: Ptr<DisjointSet>) -> Self {
        Self {
            rank: field!(_a0, rank).with_mut(std::mem::take),
            parent: field!(_a0, parent).with_mut(std::mem::take),
            n: { (*_a0.upgrade().deref()).n },
        }
    }
}
impl CppMove for DisjointSet {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct Graph {
    #[offset(0)]
    #[byte_size(8)]
    pub edges: std_unique_ptr_Edgearrarr__std_default_delete_Edgearrarr__,
    #[offset(8)]
    pub V: i32,
    #[offset(12)]
    pub E: i32,
}
impl Graph {
    pub fn move_from(_a0: Ptr<Graph>) -> Self {
        Self {
            edges: field!(_a0, edges).with_mut(std::mem::take),
            V: { (*_a0.upgrade().deref()).V },
            E: { (*_a0.upgrade().deref()).E },
        }
    }
}
impl CppMove for Graph {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
pub fn MSTKruskal_2(graph: Ptr<Graph>) -> f64 {
    ({
        let _arr: Ptr<std_unique_ptr_Edgearrarr__std_default_delete_Edgearrarr__> =
            field_ptr!(graph, edges);
        let _end: i32 = (graph.with(|__s| __s.E) - 1);
        quicksort_1(_arr, 0, _end)
    });
    let set: Value<DisjointSet> = Rc::new(RefCell::new(DisjointSet {
        rank: ({ make_unique_3((graph.with(|__s| __s.V) as u64)) }),
        parent: ({ make_unique_3((graph.with(|__s| __s.V) as u64)) }),
        n: graph.with(|__s| __s.V),
    }));
    ({ DisjointSetImpl::makeSet(&set.as_pointer()) });
    let total_weight: Value<f64> = Rc::new(RefCell::new(0_f64));
    let mut i: i32 = 0;
    'loop_: while ({ i } < { graph.with(|__s| __s.E) }) {
        let mut x: i32 = elem!(field_ptr!(graph, edges), (i as u64)).with(|__s| __s.u);
        let mut y: i32 = elem!(field_ptr!(graph, edges), (i as u64)).with(|__s| __s.v);
        let mut w: f64 = elem!(field_ptr!(graph, edges), (i as u64)).with(|__s| __s.weight);
        if (({ DisjointSetImpl::find(&set.as_pointer(), x) })
            != ({ DisjointSetImpl::find(&set.as_pointer(), y) }))
        {
            ({ DisjointSetImpl::merge(&set.as_pointer(), x, y) });
            (*total_weight.borrow_mut()) += w;
        }
        i.prefix_inc();
    }
    return (*total_weight.borrow());
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut V: i32 = 4;
    let mut E: i32 = 5;
    let graph: Value<Graph> = Rc::new(RefCell::new(Graph {
        edges: ({ make_unique_4((E as u64)) }),
        V: V,
        E: E,
    }));
    elem!(field_ptr!(graph.as_pointer(), edges), 0_u64).write(Edge {
        u: 0,
        v: 1,
        weight: 10_f64,
    });
    elem!(field_ptr!(graph.as_pointer(), edges), 1_u64).write(Edge {
        u: 1,
        v: 3,
        weight: 15_f64,
    });
    elem!(field_ptr!(graph.as_pointer(), edges), 2_u64).write(Edge {
        u: 2,
        v: 3,
        weight: 4_f64,
    });
    elem!(field_ptr!(graph.as_pointer(), edges), 3_u64).write(Edge {
        u: 2,
        v: 0,
        weight: 6_f64,
    });
    elem!(field_ptr!(graph.as_pointer(), edges), 4_u64).write(Edge {
        u: 0,
        v: 3,
        weight: 5_f64,
    });
    let mut total_weight: f64 = ({ MSTKruskal_2(graph.as_pointer()) });
    ({ cpp2rust_assert_fail_5((total_weight == 19_f64)) });
    return 0;
}
pub trait DisjointSetImpl {
    fn makeSet(&self);
    fn find(&self, x: i32) -> i32;
    fn merge(&self, x: i32, y: i32);
    fn move_assign(&self, _a0: Ptr<DisjointSet>) -> Ptr<DisjointSet>;
}
impl DisjointSetImpl for Ptr<DisjointSet> {
    fn makeSet(&self) {
        let mut i: i32 = 0;
        'loop_: while (i < (*self).with(|__s| __s.n)) {
            let __rhs = i;
            elem!(field_ptr!((*self), parent), (i as u64)).write(__rhs);
            elem!(field_ptr!((*self), rank), (i as u64)).write(1);
            i.postfix_inc();
        }
    }
    fn find(&self, mut x: i32) -> i32 {
        if ((elem!(field_ptr!((*self), parent), (x as u64)).read()) != x) {
            let __rhs = ({
                let _x: i32 = (elem!(field_ptr!((*self), parent), (x as u64)).read());
                DisjointSetImpl::find(self, _x)
            });
            elem!(field_ptr!((*self), parent), (x as u64)).write(__rhs);
        }
        return (elem!(field_ptr!((*self), parent), (x as u64)).read());
    }
    fn merge(&self, mut x: i32, mut y: i32) {
        let mut xset: i32 = ({ DisjointSetImpl::find(self, x) });
        let mut yset: i32 = ({ DisjointSetImpl::find(self, y) });
        if (xset == yset) {
            return;
        }
        if ((elem!(field_ptr!((*self), rank), (xset as u64)).read())
            < (elem!(field_ptr!((*self), rank), (yset as u64)).read()))
        {
            elem!(field_ptr!((*self), parent), (xset as u64)).write(yset);
        } else if ((elem!(field_ptr!((*self), rank), (xset as u64)).read())
            > (elem!(field_ptr!((*self), rank), (yset as u64)).read()))
        {
            elem!(field_ptr!((*self), parent), (yset as u64)).write(xset);
        } else {
            elem!(field_ptr!((*self), parent), (yset as u64)).write(xset);
            let __rhs = ((elem!(field_ptr!((*self), rank), (xset as u64)).read()) + 1);
            elem!(field_ptr!((*self), rank), (xset as u64)).write(__rhs);
        }
    }
    fn move_assign(&self, _a0: Ptr<DisjointSet>) -> Ptr<DisjointSet> {
        field!((*self), rank).write({ { (*_a0.upgrade().deref()).rank.clone() } });
        field!((*self), parent).write({ { (*_a0.upgrade().deref()).parent.clone() } });
        field!((*self), n).write({ { (*_a0.upgrade().deref()).n } });
        return (*self).clone();
    }
}
pub trait GraphImpl {
    fn move_assign(&self, _a0: Ptr<Graph>) -> Ptr<Graph>;
}
impl GraphImpl for Ptr<Graph> {
    fn move_assign(&self, _a0: Ptr<Graph>) -> Ptr<Graph> {
        field!((*self), edges).write({ { (*_a0.upgrade().deref()).edges.clone() } });
        field!((*self), V).write({ { (*_a0.upgrade().deref()).V } });
        field!((*self), E).write({ { (*_a0.upgrade().deref()).E } });
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
