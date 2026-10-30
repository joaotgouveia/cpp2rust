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
pub unsafe fn partition_0(arr: *mut (), mut start: (), mut end: ()) -> () {
    let pivot: *mut Edge = &mut (*unimplemented!());
    let mut count: () = 0_();
    let mut i: () = ((start) + (1_()));
    'loop_: while ((i) <= (end)) {
        if (((*unimplemented!()).weight) <= ((*pivot).weight)) {
            count.postfix_inc();
        }
        i.prefix_inc();
    }
    let mut pidx: () = ((start) + (count));
    let mut tmp: Edge = Edge {
        u: (*unimplemented!()).u,
        v: (*unimplemented!()).v,
        weight: (*unimplemented!()).weight,
    };
    (*unimplemented!()) = Edge {
        u: (*unimplemented!()).u,
        v: (*unimplemented!()).v,
        weight: (*unimplemented!()).weight,
    };
    (*unimplemented!()) = Edge {
        u: tmp.u,
        v: tmp.v,
        weight: tmp.weight,
    };
    let mut i: () = start;
    let mut j: () = end;
    'loop_: while ((i) < (pidx)) && ((j) > (pidx)) {
        'loop_: while (((*unimplemented!()).weight) <= ((*pivot).weight)) {
            i.prefix_inc();
        }
        'loop_: while (((*unimplemented!()).weight) > ((*pivot).weight)) {
            j.prefix_dec();
        }
        if ((i) < (pidx)) && ((j) > (pidx)) {
            tmp = Edge {
                u: (*unimplemented!()).u,
                v: (*unimplemented!()).v,
                weight: (*unimplemented!()).weight,
            };
            (*unimplemented!()) = Edge {
                u: (*unimplemented!()).u,
                v: (*unimplemented!()).v,
                weight: (*unimplemented!()).weight,
            };
            (*unimplemented!()) = Edge {
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
pub unsafe fn quicksort_1(arr: *mut (), mut start: (), mut end: ()) {
    if ((start) >= (end)) {
        return;
    }
    let mut p: () = (unsafe {
        let _arr: *mut () = arr;
        let _start: () = start;
        let _end: () = end;
        partition_0(_arr, _start, _end)
    });
    (unsafe {
        let _arr: *mut () = arr;
        let _start: () = start;
        let _end: () = ((p) - (1_()));
        quicksort_1(_arr, _start, _end)
    });
    (unsafe {
        let _arr: *mut () = arr;
        let _start: () = ((p) + (1_()));
        let _end: () = end;
        quicksort_1(_arr, _start, _end)
    });
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct DisjointSet {
    pub rank: (),
    pub parent: (),
    pub n: (),
}
impl DisjointSet {
    pub unsafe fn makeSet(&mut self) {
        let mut i: () = 0_();
        'loop_: while ((i) < (self.n)) {
            (*unimplemented!()) = i;
            (*unimplemented!()) = 1_();
            i.postfix_inc();
        }
    }
    pub unsafe fn find(&mut self, mut x: ()) -> () {
        if ((*unimplemented!()) != (x)) {
            (*unimplemented!()) = (unsafe {
                let _x: () = (*unimplemented!());
                DisjointSet::find(self, _x)
            });
        }
        return (*unimplemented!());
    }
    pub unsafe fn merge(&mut self, mut x: (), mut y: ()) {
        let mut xset: () = (unsafe { DisjointSet::find(self, x) });
        let mut yset: () = (unsafe { DisjointSet::find(self, y) });
        if ((xset) == (yset)) {
            return;
        }
        if ((*unimplemented!()) < (*unimplemented!())) {
            (*unimplemented!()) = yset;
        } else if ((*unimplemented!()) > (*unimplemented!())) {
            (*unimplemented!()) = xset;
        } else {
            (*unimplemented!()) = xset;
            (*unimplemented!()) = ((*unimplemented!()) + (1_()));
        }
    }
    pub unsafe fn move_from(_a0: *mut DisjointSet) -> Self {
        let mut this = Self {
            rank: unimplemented!(),
            parent: unimplemented!(),
            n: (*_a0).n,
        };
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut DisjointSet) -> *mut DisjointSet {
        unimplemented!();
        unimplemented!();
        self.n = (*_a0).n;
        return &mut (*(self as *mut DisjointSet));
    }
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct Graph {
    pub edges: (),
    pub V: (),
    pub E: (),
}
impl Graph {
    pub unsafe fn move_from(_a0: *mut Graph) -> Self {
        let mut this = Self {
            edges: unimplemented!(),
            V: (*_a0).V,
            E: (*_a0).E,
        };
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut Graph) -> *mut Graph {
        unimplemented!();
        self.V = (*_a0).V;
        self.E = (*_a0).E;
        return &mut (*(self as *mut Graph));
    }
}
pub unsafe fn MSTKruskal_2(graph: *mut Graph) -> () {
    (unsafe {
        let _arr: *mut () = &mut (*graph).edges;
        let _end: () = (((*graph).E) - (1_()));
        quicksort_1(_arr, 0_(), _end)
    });
    let mut set: DisjointSet = DisjointSet {
        rank: unimplemented!(),
        parent: unimplemented!(),
        n: (*graph).V,
    };
    (unsafe { DisjointSet::makeSet(&mut set) });
    let mut total_weight: () = 0_();
    let mut i: () = 0_();
    'loop_: while ((i) < ((*graph).E)) {
        let mut x: () = (*unimplemented!()).u;
        let mut y: () = (*unimplemented!()).v;
        let mut w: () = (*unimplemented!()).weight;
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
        edges: unimplemented!(),
        V: V,
        E: E,
    };
    (*unimplemented!()) = Edge {
        u: 0_(),
        v: 1_(),
        weight: 10_(),
    };
    (*unimplemented!()) = Edge {
        u: 1_(),
        v: 3_(),
        weight: 15_(),
    };
    (*unimplemented!()) = Edge {
        u: 2_(),
        v: 3_(),
        weight: 4_(),
    };
    (*unimplemented!()) = Edge {
        u: 2_(),
        v: 0_(),
        weight: 6_(),
    };
    (*unimplemented!()) = Edge {
        u: 0_(),
        v: 3_(),
        weight: 5_(),
    };
    let mut total_weight: () = (unsafe { MSTKruskal_2(&mut graph) });
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
