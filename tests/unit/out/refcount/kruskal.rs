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
    pub u: (),
    #[offset(4)]
    pub v: (),
    #[offset(8)]
    pub weight: (),
}
pub fn partition_0(
    arr: Ptr<std_unique_ptr_Edgearrarr__std_default_delete_Edgearrarr__>,
    mut start: (),
    mut end: (),
) -> () {
    let pivot: Ptr<Edge> = arr.offset(start);
    let mut count: () = 0_();
    let mut i: () = (start + 1_());
    'loop_: while (i <= end) {
        if ({ elem!(arr, i).with(|__s| __s.weight) } <= { pivot.with(|__s| __s.weight) }) {
            count.postfix_inc();
        }
        i.prefix_inc();
    }
    let mut pidx: () = (start + count);
    let mut tmp: Edge = Edge {
        u: elem!(arr, pidx).with(|__s| __s.u),
        v: elem!(arr, pidx).with(|__s| __s.v),
        weight: elem!(arr, pidx).with(|__s| __s.weight),
    };
    let __rhs = Edge {
        u: elem!(arr, start).with(|__s| __s.u),
        v: elem!(arr, start).with(|__s| __s.v),
        weight: elem!(arr, start).with(|__s| __s.weight),
    };
    elem!(arr, pidx).write(__rhs);
    elem!(arr, start).write(Edge {
        u: tmp.u,
        v: tmp.v,
        weight: tmp.weight,
    });
    let mut i: () = start;
    let mut j: () = end;
    'loop_: while (i < pidx) && (j > pidx) {
        'loop_: while ({ elem!(arr, i).with(|__s| __s.weight) } <= { pivot.with(|__s| __s.weight) })
        {
            i.prefix_inc();
        }
        'loop_: while ({ elem!(arr, j).with(|__s| __s.weight) } > { pivot.with(|__s| __s.weight) })
        {
            j.prefix_dec();
        }
        if (i < pidx) && (j > pidx) {
            tmp = Edge {
                u: elem!(arr, i).with(|__s| __s.u),
                v: elem!(arr, i).with(|__s| __s.v),
                weight: elem!(arr, i).with(|__s| __s.weight),
            };
            let __rhs = Edge {
                u: elem!(arr, j).with(|__s| __s.u),
                v: elem!(arr, j).with(|__s| __s.v),
                weight: elem!(arr, j).with(|__s| __s.weight),
            };
            elem!(arr, i).write(__rhs);
            elem!(arr, j).write(Edge {
                u: tmp.u,
                v: tmp.v,
                weight: tmp.weight,
            });
            i.postfix_inc();
            j.postfix_dec();
        }
    }
    return pidx;
}
pub fn quicksort_1(
    arr: Ptr<std_unique_ptr_Edgearrarr__std_default_delete_Edgearrarr__>,
    mut start: (),
    mut end: (),
) {
    if (start >= end) {
        return;
    }
    let mut p: () = ({
        let _arr: Ptr<std_unique_ptr_Edgearrarr__std_default_delete_Edgearrarr__> = (arr).clone();
        let _start: () = start;
        let _end: () = end;
        partition_0(_arr, _start, _end)
    });
    ({
        let _arr: Ptr<std_unique_ptr_Edgearrarr__std_default_delete_Edgearrarr__> = (arr).clone();
        let _start: () = start;
        let _end: () = (p - 1_());
        quicksort_1(_arr, _start, _end)
    });
    ({
        let _arr: Ptr<std_unique_ptr_Edgearrarr__std_default_delete_Edgearrarr__> = (arr).clone();
        let _start: () = (p + 1_());
        let _end: () = end;
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
    pub n: (),
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
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(16)]
pub struct Graph {
    #[offset(0)]
    #[byte_size(8)]
    pub edges: std_unique_ptr_Edgearrarr__std_default_delete_Edgearrarr__,
    #[offset(8)]
    pub V: (),
    #[offset(12)]
    pub E: (),
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
pub fn MSTKruskal_2(graph: Ptr<Graph>) -> () {
    ({
        let _arr: Ptr<std_unique_ptr_Edgearrarr__std_default_delete_Edgearrarr__> =
            field_ptr!(graph, edges);
        let _end: () = (graph.with(|__s| __s.E) - 1_());
        quicksort_1(_arr, 0_(), _end)
    });
    let set: Value<DisjointSet> = Rc::new(RefCell::new(DisjointSet {
        rank: ({ make_unique_3(graph.with(|__s| __s.V)) }),
        parent: ({ make_unique_3(graph.with(|__s| __s.V)) }),
        n: graph.with(|__s| __s.V),
    }));
    ({ DisjointSetImpl::makeSet(&set.as_pointer()) });
    let mut total_weight: () = 0_();
    let mut i: () = 0_();
    'loop_: while ({ i } < { graph.with(|__s| __s.E) }) {
        let mut x: () = elem!(field_ptr!(graph, edges), i).with(|__s| __s.u);
        let mut y: () = elem!(field_ptr!(graph, edges), i).with(|__s| __s.v);
        let mut w: () = elem!(field_ptr!(graph, edges), i).with(|__s| __s.weight);
        if (({ DisjointSetImpl::find(&set.as_pointer(), x) })
            != ({ DisjointSetImpl::find(&set.as_pointer(), y) }))
        {
            ({ DisjointSetImpl::merge(&set.as_pointer(), x, y) });
            total_weight += w;
        }
        i.prefix_inc();
    }
    return total_weight;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let mut V: () = 4_();
    let mut E: () = 5_();
    let graph: Value<Graph> = Rc::new(RefCell::new(Graph {
        edges: ({ make_unique_4(E) }),
        V: V,
        E: E,
    }));
    elem!(field_ptr!(graph.as_pointer(), edges), 0_()).write(Edge {
        u: 0_(),
        v: 1_(),
        weight: 10_(),
    });
    elem!(field_ptr!(graph.as_pointer(), edges), 1_()).write(Edge {
        u: 1_(),
        v: 3_(),
        weight: 15_(),
    });
    elem!(field_ptr!(graph.as_pointer(), edges), 2_()).write(Edge {
        u: 2_(),
        v: 3_(),
        weight: 4_(),
    });
    elem!(field_ptr!(graph.as_pointer(), edges), 3_()).write(Edge {
        u: 2_(),
        v: 0_(),
        weight: 6_(),
    });
    elem!(field_ptr!(graph.as_pointer(), edges), 4_()).write(Edge {
        u: 0_(),
        v: 3_(),
        weight: 5_(),
    });
    let mut total_weight: () = ({ MSTKruskal_2(graph.as_pointer()) });
    unimplemented!();
    return 0_();
}
pub trait DisjointSetImpl {
    fn makeSet(&self);
    fn find(&self, x: ()) -> ();
    fn merge(&self, x: (), y: ());
    fn move_assign(&self, _a0: Ptr<DisjointSet>) -> Ptr<DisjointSet>;
}
impl DisjointSetImpl for Ptr<DisjointSet> {
    fn makeSet(&self) {
        let mut i: () = 0_();
        'loop_: while (i < (*self).with(|__s| __s.n)) {
            let __rhs = i;
            elem!(field_ptr!((*self), parent), i).write(__rhs);
            elem!(field_ptr!((*self), rank), i).write(1_());
            i.postfix_inc();
        }
    }
    fn find(&self, mut x: ()) -> () {
        if ((elem!(field_ptr!((*self), parent), x).read()) != x) {
            let __rhs = ({
                let _x: () = (elem!(field_ptr!((*self), parent), x).read());
                DisjointSetImpl::find(self, _x)
            });
            elem!(field_ptr!((*self), parent), x).write(__rhs);
        }
        return (elem!(field_ptr!((*self), parent), x).read());
    }
    fn merge(&self, mut x: (), mut y: ()) {
        let mut xset: () = ({ DisjointSetImpl::find(self, x) });
        let mut yset: () = ({ DisjointSetImpl::find(self, y) });
        if (xset == yset) {
            return;
        }
        if ((elem!(field_ptr!((*self), rank), xset).read())
            < (elem!(field_ptr!((*self), rank), yset).read()))
        {
            elem!(field_ptr!((*self), parent), xset).write(yset);
        } else if ((elem!(field_ptr!((*self), rank), xset).read())
            > (elem!(field_ptr!((*self), rank), yset).read()))
        {
            elem!(field_ptr!((*self), parent), yset).write(xset);
        } else {
            elem!(field_ptr!((*self), parent), yset).write(xset);
            let __rhs = ((elem!(field_ptr!((*self), rank), xset).read()) + 1_());
            elem!(field_ptr!((*self), rank), xset).write(__rhs);
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
