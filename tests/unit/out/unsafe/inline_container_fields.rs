extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct Elem {
    pub id: (),
    pub hist: [(); 4],
    pub coeffs: std_vector_short__std_allocator_short__,
}
impl Elem {
    pub unsafe fn move_from(_a0: *mut Elem) -> Self {
        let mut this = Self {
            id: (*_a0).id,
            hist: std::array::from_fn::<_, 4, _>(|__i: usize| (*_a0).hist[(__i) as usize]),
            coeffs: (*_a0).coeffs,
        };
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut Elem) -> *mut Elem {
        self.id = (*_a0).id;
        unimplemented!();
        self.coeffs = ((*_a0).coeffs).clone();
        return &mut (*(self as *mut Elem));
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe)]
pub struct State { pub count : () , pub grid : [ [ () ; 3] ; 2] , pub arr : std_array_int__3_ , pub elems : std_vector_Elem__std_allocator_Elem__ , pub rows : std_vector_std_vector_int__std_allocator_int____std_allocator_std_vector_int__std_allocator_int____ , }
impl State {
    pub unsafe fn move_from(_a0: *mut State) -> Self {
        let mut this = Self {
            count: (*_a0).count,
            grid: std::array::from_fn::<_, 2, _>(|__i: usize| {
                std::array::from_fn::<_, 3, _>(|__i2: usize| {
                    (*_a0).grid[(__i) as usize][(__i2) as usize]
                })
            }),
            arr: (*_a0).arr,
            elems: (*_a0).elems,
            rows: (*_a0).rows,
        };
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut State) -> *mut State {
        self.count = (*_a0).count;
        unimplemented!();
        self.arr = (*_a0).arr;
        self.elems = ((*_a0).elems).clone();
        self.rows = ((*_a0).rows).clone();
        return &mut (*(self as *mut State));
    }
}
impl Default for State {
    fn default() -> Self {
        unsafe {
            State { count : unimplemented!() , grid : [[unimplemented!(); 3]; 2] , arr : unsafe { std::mem::zeroed::<std_array_int__3_ >() } , elems : <std_vector_Elem__std_allocator_Elem__ >::default() , rows : <std_vector_std_vector_int__std_allocator_int____std_allocator_std_vector_int__std_allocator_int____ >::default() , }
        }
    }
}
pub unsafe fn sum_0(mut p: *const (), mut n: ()) -> () {
    let mut s: () = 0_();
    let mut i: () = 0_();
    'loop_: while ((i) < (n)) {
        s += (*p.offset((i) as isize));
        i.prefix_inc();
    }
    return s;
}
pub unsafe fn fill_1(mut s: *mut State) {
    let mut i: () = 0_();
    'loop_: while ((i) < (2_())) {
        let mut j: () = 0_();
        'loop_: while ((j) < (3_())) {
            (*s).grid[(i) as usize][(j) as usize] = (((i) * (3_())) + (j));
            j.prefix_inc();
        }
        i.prefix_inc();
    }
    (&mut (*s)).arr[(1_()) as usize] = 7_();
    (unsafe { (*s).elems.resize_40(3_()) });
    let mut i: () = 0_();
    'loop_: while ((i) < (3_())) {
        (&mut (*s)).elems[(i) as usize].id = i;
        (&mut (*s)).elems[(i) as usize].hist[(i) as usize] = ((10_()) * (i));
        (unsafe {
            let mut ___x: () = i;
            (&mut (*s)).elems[(i) as usize]
                .coeffs
                .resize_41(4_(), &mut ___x)
        });
        i.prefix_inc();
    }
    (unsafe {
        let mut ___x: std_vector_int__std_allocator_int__ =
            std_vector_int__std_allocator_int__::new_1({ unimplemented!() }, None);
        (*s).rows.push_back_58(&mut ___x)
    });
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut s: State = <State>::default();
    (unsafe { fill_1((&mut s as *mut State)) });
    unimplemented!();
    unimplemented!();
    let mut e: *mut Elem = (&mut s.elems[(2_()) as usize] as *mut Elem);
    let mut h: *mut () = (*e).hist.as_mut_ptr();
    let mut c: *mut () = (&mut (&mut (*e)).coeffs[(1_()) as usize] as *mut ());
    let mut id: *mut () = (&mut s.elems[(1_()) as usize].id as *mut ());
    let mut k: () = 0_();
    'loop_: while ((k) < (4_())) {
        (*h.offset((k) as isize)) += k;
        k.prefix_inc();
    }
    (*c) += 5_();
    (*id) = 11_();
    s.count = 3_();
    unimplemented!();
    unimplemented!();
    let mut row: *mut () = (&mut s.rows[(0_()) as usize][(0_()) as usize] as *mut ());
    (*row.offset((2_()) as isize)) = 9_();
    unimplemented!();
    let mut t: State = s.clone();
    t.elems[(2_()) as usize].hist[(0_()) as usize] = 100_();
    t.grid[(0_()) as usize][(0_()) as usize] = 100_();
    t.rows[(0_()) as usize][(0_()) as usize] = 100_();
    unimplemented!();
    unimplemented!();
    let mut p: *mut State = (&mut t as *mut State);
    (&mut (*p)).elems[(0_()) as usize].hist[(3_()) as usize] =
        (((*p).grid[(1_()) as usize][(1_()) as usize]) + ((&mut (*p)).arr[(1_()) as usize]));
    unimplemented!();
    (unsafe {
        let ___x: *const Elem = &(&mut (*p)).elems[(2_()) as usize];
        (*p).elems.push_back_57(___x)
    });
    unimplemented!();
    let mut elems: std_vector_Elem__std_allocator_Elem__ =
        std_vector_Elem__std_allocator_Elem__::new_1({ 2_() }, None);
    let mut first: *mut Elem = (&mut elems[(0_()) as usize] as *mut Elem);
    (unsafe { elems[(1_()) as usize].coeffs.resize_40(2_()) });
    elems[(1_()) as usize].coeffs[(1_()) as usize] = 4_();
    elems[(1_()) as usize].hist[(2_()) as usize] =
        ((elems[(1_()) as usize].coeffs[(1_()) as usize]) + (1_()));
    unimplemented!();
    let mut moved: State = State::move_from({ &mut t });
    unimplemented!();
    unimplemented!();
    let mut assigned: State = <State>::default();
    (unsafe { State::move_assign(&mut assigned, &mut moved) });
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
