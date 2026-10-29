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
pub struct Edge {
    pub u: (),
    pub v: (),
    pub weight: (),
}
pub unsafe fn partition_0(
    arr: *mut std_unique_ptr_Edgearrarr__std_default_delete_Edgearrarr__,
    mut start: (),
    mut end: (),
) -> () {
    let pivot: *mut Edge = &mut (&(*arr))[(start) as usize];
    let mut count: () = 0_();
    let mut i: () = ((start) + (1_()));
    'loop_: while ((i) <= (end)) {
        if (((&(*arr))[(i) as usize].weight) <= ((*pivot).weight)) {
            count.postfix_inc();
        }
        i.prefix_inc();
    }
    let mut pidx: () = ((start) + (count));
    let mut tmp: Edge = Edge {
        u: (&(*arr))[(pidx) as usize].u,
        v: (&(*arr))[(pidx) as usize].v,
        weight: (&(*arr))[(pidx) as usize].weight,
    };
    (&(*arr))[(pidx) as usize] = Edge {
        u: (&(*arr))[(start) as usize].u,
        v: (&(*arr))[(start) as usize].v,
        weight: (&(*arr))[(start) as usize].weight,
    };
    (&(*arr))[(start) as usize] = Edge {
        u: tmp.u,
        v: tmp.v,
        weight: tmp.weight,
    };
    let mut i: () = start;
    let mut j: () = end;
    'loop_: while ((i) < (pidx)) && ((j) > (pidx)) {
        'loop_: while (((&(*arr))[(i) as usize].weight) <= ((*pivot).weight)) {
            i.prefix_inc();
        }
        'loop_: while (((&(*arr))[(j) as usize].weight) > ((*pivot).weight)) {
            j.prefix_dec();
        }
        if ((i) < (pidx)) && ((j) > (pidx)) {
            tmp = Edge {
                u: (&(*arr))[(i) as usize].u,
                v: (&(*arr))[(i) as usize].v,
                weight: (&(*arr))[(i) as usize].weight,
            };
            (&(*arr))[(i) as usize] = Edge {
                u: (&(*arr))[(j) as usize].u,
                v: (&(*arr))[(j) as usize].v,
                weight: (&(*arr))[(j) as usize].weight,
            };
            (&(*arr))[(j) as usize] = Edge {
                u: tmp.u,
                v: tmp.v,
                weight: tmp.weight,
            };
            i.postfix_inc();
            j.postfix_dec();
        }
    }
    return pidx;
}
pub unsafe fn quicksort_1(
    arr: *mut std_unique_ptr_Edgearrarr__std_default_delete_Edgearrarr__,
    mut start: (),
    mut end: (),
) {
    if ((start) >= (end)) {
        return;
    }
    let mut p: () = (unsafe {
        let _arr: *mut std_unique_ptr_Edgearrarr__std_default_delete_Edgearrarr__ = arr;
        let _start: () = start;
        let _end: () = end;
        partition_0(_arr, _start, _end)
    });
    (unsafe {
        let _arr: *mut std_unique_ptr_Edgearrarr__std_default_delete_Edgearrarr__ = arr;
        let _start: () = start;
        let _end: () = ((p) - (1_()));
        quicksort_1(_arr, _start, _end)
    });
    (unsafe {
        let _arr: *mut std_unique_ptr_Edgearrarr__std_default_delete_Edgearrarr__ = arr;
        let _start: () = ((p) + (1_()));
        let _end: () = end;
        quicksort_1(_arr, _start, _end)
    });
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct DisjointSet {
    pub rank: std_unique_ptr_intarrarr__std_default_delete_intarrarr__,
    pub parent: std_unique_ptr_intarrarr__std_default_delete_intarrarr__,
    pub n: (),
}
impl DisjointSet {
    pub unsafe fn makeSet(&mut self) {
        let mut i: () = 0_();
        'loop_: while ((i) < (self.n)) {
            self.parent[(i) as usize] = i;
            self.rank[(i) as usize] = 1_();
            i.postfix_inc();
        }
    }
    pub unsafe fn find(&mut self, mut x: ()) -> () {
        if ((self.parent[(x) as usize]) != (x)) {
            self.parent[(x) as usize] = (unsafe {
                let _x: () = self.parent[(x) as usize];
                DisjointSet::find(self, _x)
            });
        }
        return self.parent[(x) as usize];
    }
    pub unsafe fn merge(&mut self, mut x: (), mut y: ()) {
        let mut xset: () = (unsafe { DisjointSet::find(self, x) });
        let mut yset: () = (unsafe { DisjointSet::find(self, y) });
        if ((xset) == (yset)) {
            return;
        }
        if ((self.rank[(xset) as usize]) < (self.rank[(yset) as usize])) {
            self.parent[(xset) as usize] = yset;
        } else if ((self.rank[(xset) as usize]) > (self.rank[(yset) as usize])) {
            self.parent[(yset) as usize] = xset;
        } else {
            self.parent[(yset) as usize] = xset;
            self.rank[(xset) as usize] = ((self.rank[(xset) as usize]) + (1_()));
        }
    }
    pub unsafe fn move_from(_a0: *mut DisjointSet) -> Self {
        let mut this = Self {
            rank: (*_a0).rank,
            parent: (*_a0).parent,
            n: (*_a0).n,
        };
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut DisjointSet) -> *mut DisjointSet {
        self.rank = ((*_a0).rank).clone();
        self.parent = ((*_a0).parent).clone();
        self.n = (*_a0).n;
        return &mut (*(self as *mut DisjointSet));
    }
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct Graph {
    pub edges: std_unique_ptr_Edgearrarr__std_default_delete_Edgearrarr__,
    pub V: (),
    pub E: (),
}
impl Graph {
    pub unsafe fn move_from(_a0: *mut Graph) -> Self {
        let mut this = Self {
            edges: (*_a0).edges,
            V: (*_a0).V,
            E: (*_a0).E,
        };
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut Graph) -> *mut Graph {
        self.edges = ((*_a0).edges).clone();
        self.V = (*_a0).V;
        self.E = (*_a0).E;
        return &mut (*(self as *mut Graph));
    }
}
pub unsafe fn MSTKruskal_2(graph: *mut Graph) -> () {
    (unsafe {
        let _arr: *mut std_unique_ptr_Edgearrarr__std_default_delete_Edgearrarr__ =
            &mut (*graph).edges;
        let _end: () = (((*graph).E) - (1_()));
        quicksort_1(_arr, 0_(), _end)
    });
    let mut set: DisjointSet = DisjointSet {
        rank: (unsafe { make_unique_3((*graph).V) }),
        parent: (unsafe { make_unique_3((*graph).V) }),
        n: (*graph).V,
    };
    (unsafe { DisjointSet::makeSet(&mut set) });
    let mut total_weight: () = 0_();
    let mut i: () = 0_();
    'loop_: while ((i) < ((*graph).E)) {
        let mut x: () = (&(*graph)).edges[(i) as usize].u;
        let mut y: () = (&(*graph)).edges[(i) as usize].v;
        let mut w: () = (&(*graph)).edges[(i) as usize].weight;
        if ((unsafe { DisjointSet::find(&mut set, x) })
            != (unsafe { DisjointSet::find(&mut set, y) }))
        {
            (unsafe { DisjointSet::merge(&mut set, x, y) });
            total_weight += w;
        }
        i.prefix_inc();
    }
    return total_weight;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut V: () = 4_();
    let mut E: () = 5_();
    let mut graph: Graph = Graph {
        edges: (unsafe { make_unique_4(E) }),
        V: V,
        E: E,
    };
    graph.edges[(0_()) as usize] = Edge {
        u: 0_(),
        v: 1_(),
        weight: 10_(),
    };
    graph.edges[(1_()) as usize] = Edge {
        u: 1_(),
        v: 3_(),
        weight: 15_(),
    };
    graph.edges[(2_()) as usize] = Edge {
        u: 2_(),
        v: 3_(),
        weight: 4_(),
    };
    graph.edges[(3_()) as usize] = Edge {
        u: 2_(),
        v: 0_(),
        weight: 6_(),
    };
    graph.edges[(4_()) as usize] = Edge {
        u: 0_(),
        v: 3_(),
        weight: 5_(),
    };
    let mut total_weight: () = (unsafe { MSTKruskal_2(&mut graph) });
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
