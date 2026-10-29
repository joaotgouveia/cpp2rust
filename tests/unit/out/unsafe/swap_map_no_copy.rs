extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut copies_0: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, CopyAssignUnsafe, MoveAssignUnsafe)]
pub struct Tracked {
    pub v: (),
}
impl Tracked {
    pub unsafe fn new_1() -> Self {
        let mut this = Self { v: 0_() };
        this
    }
    pub unsafe fn new_2(mut v: ()) -> Self {
        let mut this = Self { v: v };
        this
    }
    pub unsafe fn copy_from(o: *const Tracked) -> Self {
        let mut this = Self { v: (*o).v };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)).prefix_inc();
        this
    }
    pub unsafe fn move_from(o: *mut Tracked) -> Self {
        let mut this = Self { v: (*o).v };
        this
    }
    pub unsafe fn copy_assign(&mut self, o: *const Tracked) -> *mut Tracked {
        self.v = (*o).v;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)).prefix_inc();
        return &mut (*(self as *mut Tracked));
    }
    pub unsafe fn move_assign(&mut self, o: *mut Tracked) -> *mut Tracked {
        self.v = (*o).v;
        return &mut (*(self as *mut Tracked));
    }
}
impl Clone for Tracked {
    fn clone(&self) -> Self {
        unsafe { Tracked::copy_from(self as *const Tracked) }
    }
}
impl Default for Tracked {
    fn default() -> Self {
        unsafe { Tracked::new_1() }
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, MoveCtorUnsafe, CopyAssignUnsafe, MoveAssignUnsafe, Default)]
pub struct Pair {
    pub x: Tracked,
    pub y: Tracked,
}
impl Pair {
    pub unsafe fn move_from(_a0: *mut Pair) -> Self {
        let mut this = Self {
            x: Tracked::move_from({ &mut (*_a0).x }),
            y: Tracked::move_from({ &mut (*_a0).y }),
        };
        this
    }
    pub unsafe fn copy_assign(&mut self, _a0: *const Pair) -> *mut Pair {
        (unsafe {
            let _o: *const Tracked = &(*_a0).x;
            Tracked::copy_assign(&mut self.x, _o)
        });
        (unsafe {
            let _o: *const Tracked = &(*_a0).y;
            Tracked::copy_assign(&mut self.y, _o)
        });
        return &mut (*(self as *mut Pair));
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut Pair) -> *mut Pair {
        (unsafe {
            let _o: *mut Tracked = &mut (*_a0).x;
            Tracked::move_assign(&mut self.x, _o)
        });
        (unsafe {
            let _o: *mut Tracked = &mut (*_a0).y;
            Tracked::move_assign(&mut self.y, _o)
        });
        return &mut (*(self as *mut Pair));
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, MoveCtorUnsafe, CopyAssignUnsafe, MoveAssignUnsafe, Default)]
pub struct Arrays {
    pub a: [Tracked; 2],
    pub b: [Tracked; 2],
}
impl Arrays {
    pub unsafe fn move_from(_a0: *mut Arrays) -> Self {
        let mut this = Self {
            a: std::array::from_fn::<_, 2, _>(|__i: usize| {
                Tracked::move_from({ &mut (*_a0).a[(__i) as usize] })
            }),
            b: std::array::from_fn::<_, 2, _>(|__i: usize| {
                Tracked::move_from({ &mut (*_a0).b[(__i) as usize] })
            }),
        };
        this
    }
    pub unsafe fn copy_assign(&mut self, _a0: *const Arrays) -> *mut Arrays {
        let mut __i0: () = 0_();
        'loop_: while ((__i0) != (2_())) {
            (unsafe {
                let _o: *const Tracked = &(*_a0).a[(__i0) as usize];
                Tracked::copy_assign(&mut self.a[(__i0) as usize], _o)
            });
            __i0.prefix_inc();
        }
        let mut __i0: () = 0_();
        'loop_: while ((__i0) != (2_())) {
            (unsafe {
                let _o: *const Tracked = &(*_a0).b[(__i0) as usize];
                Tracked::copy_assign(&mut self.b[(__i0) as usize], _o)
            });
            __i0.prefix_inc();
        }
        return &mut (*(self as *mut Arrays));
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut Arrays) -> *mut Arrays {
        let mut __i0: () = 0_();
        'loop_: while ((__i0) != (2_())) {
            (unsafe {
                let _o: *mut Tracked = &mut (*_a0).a[(__i0) as usize];
                Tracked::move_assign(&mut self.a[(__i0) as usize], _o)
            });
            __i0.prefix_inc();
        }
        let mut __i0: () = 0_();
        'loop_: while ((__i0) != (2_())) {
            (unsafe {
                let _o: *mut Tracked = &mut (*_a0).b[(__i0) as usize];
                Tracked::move_assign(&mut self.b[(__i0) as usize], _o)
            });
            __i0.prefix_inc();
        }
        return &mut (*(self as *mut Arrays));
    }
}
pub static mut copy_only_copies_1: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut copy_only_dtors_2: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe, DestructorUnsafe, Default)]
pub struct CopyOnly {
    pub v: (),
}
impl CopyOnly {
    pub unsafe fn new(mut v: ()) -> Self {
        let mut this = Self { v: v };
        this
    }
    pub unsafe fn copy_from(o: *const CopyOnly) -> Self {
        let mut this = Self { v: (*o).v };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copy_only_copies_1)).prefix_inc();
        this
    }
    pub unsafe fn destructor(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copy_only_dtors_2)).prefix_inc();
    }
    pub unsafe fn copy_assign(&mut self, o: *const CopyOnly) -> *mut CopyOnly {
        self.v = (*o).v;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copy_only_copies_1)).prefix_inc();
        return &mut (*(self as *mut CopyOnly));
    }
}
impl Clone for CopyOnly {
    fn clone(&self) -> Self {
        unsafe { CopyOnly::copy_from(self as *const CopyOnly) }
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, CopyAssignUnsafe, DestructorUnsafe, Default)]
pub struct CopyOnlyPair {
    pub x: CopyOnly,
    pub y: CopyOnly,
}
impl CopyOnlyPair {
    pub unsafe fn copy_assign(&mut self, _a0: *const CopyOnlyPair) -> *mut CopyOnlyPair {
        (unsafe {
            let _o: *const CopyOnly = &(*_a0).x;
            CopyOnly::copy_assign(&mut self.x, _o)
        });
        (unsafe {
            let _o: *const CopyOnly = &(*_a0).y;
            CopyOnly::copy_assign(&mut self.y, _o)
        });
        return &mut (*(self as *mut CopyOnlyPair));
    }
}
impl CopyOnlyPair {
    pub unsafe fn destructor(&mut self) {
        CopyOnly::destructor(&mut self.y);
        CopyOnly::destructor(&mut self.x);
    }
}
pub static mut moves_3: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut move_assigns_4: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut moved_dtors_5: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
#[repr(C)]
#[derive(
    VaArg, FnPtrArg, MoveCtorUnsafe, CopyAssignUnsafe, MoveAssignUnsafe, DestructorUnsafe, Default,
)]
pub struct Moved {
    pub v: (),
}
impl Moved {
    pub unsafe fn new(mut v: ()) -> Self {
        let mut this = Self { v: v };
        this
    }
    pub unsafe fn copy_from(o: *const Moved) -> Self {
        let mut this = Self { v: (*o).v };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)).prefix_inc();
        this
    }
    pub unsafe fn move_from(o: *mut Moved) -> Self {
        let mut this = Self { v: (*o).v };
        (*o).v = -1_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut moves_3)).prefix_inc();
        this
    }
    pub unsafe fn copy_assign(&mut self, o: *const Moved) -> *mut Moved {
        self.v = (*o).v;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)).prefix_inc();
        return &mut (*(self as *mut Moved));
    }
    pub unsafe fn move_assign(&mut self, o: *mut Moved) -> *mut Moved {
        self.v = (*o).v;
        (*o).v = -1_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut move_assigns_4)).prefix_inc();
        return &mut (*(self as *mut Moved));
    }
    pub unsafe fn destructor(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut moved_dtors_5)).prefix_inc();
    }
}
impl Clone for Moved {
    fn clone(&self) -> Self {
        unsafe { Moved::copy_from(self as *const Moved) }
    }
}
#[repr(C)]
#[derive(
    Clone,
    VaArg,
    FnPtrArg,
    MoveCtorUnsafe,
    CopyAssignUnsafe,
    MoveAssignUnsafe,
    DestructorUnsafe,
    Default,
)]
pub struct MovedPair {
    pub x: Moved,
    pub y: Moved,
}
impl MovedPair {
    pub unsafe fn move_from(_a0: *mut MovedPair) -> Self {
        let mut this = Self {
            x: Moved::move_from({ &mut (*_a0).x }),
            y: Moved::move_from({ &mut (*_a0).y }),
        };
        this
    }
    pub unsafe fn copy_assign(&mut self, _a0: *const MovedPair) -> *mut MovedPair {
        (unsafe {
            let _o: *const Moved = &(*_a0).x;
            Moved::copy_assign(&mut self.x, _o)
        });
        (unsafe {
            let _o: *const Moved = &(*_a0).y;
            Moved::copy_assign(&mut self.y, _o)
        });
        return &mut (*(self as *mut MovedPair));
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut MovedPair) -> *mut MovedPair {
        (unsafe {
            let _o: *mut Moved = &mut (*_a0).x;
            Moved::move_assign(&mut self.x, _o)
        });
        (unsafe {
            let _o: *mut Moved = &mut (*_a0).y;
            Moved::move_assign(&mut self.y, _o)
        });
        return &mut (*(self as *mut MovedPair));
    }
}
impl MovedPair {
    pub unsafe fn destructor(&mut self) {
        Moved::destructor(&mut self.y);
        Moved::destructor(&mut self.x);
    }
}
pub unsafe fn swapped_once_6() -> () {
    let mut ok: () = (((*std::cell::LazyCell::force_mut(&mut *&raw mut moves_3)) == (1_()))
        && ((*std::cell::LazyCell::force_mut(&mut *&raw mut move_assigns_4)) == (2_())))
        && ((*std::cell::LazyCell::force_mut(&mut *&raw mut moved_dtors_5)) == (1_()));
    (*std::cell::LazyCell::force_mut(&mut *&raw mut moves_3)) = {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut move_assigns_4)) = {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut moved_dtors_5)) = 0_();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut moved_dtors_5))
        };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut move_assigns_4))
    };
    return ok;
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct Owner {
    pub p: std_unique_ptr_int__std_default_delete_int__,
}
impl Owner {
    pub unsafe fn move_from(_a0: *mut Owner) -> Self {
        let mut this = Self { p: (*_a0).p };
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut Owner) -> *mut Owner {
        self.p = ((*_a0).p).clone();
        return &mut (*(self as *mut Owner));
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut a: Tracked = Tracked::new_2({ 1_() });
    let mut b: Tracked = Tracked::new_2({ 2_() });
    (unsafe { swap_7(&mut a, &mut b) });
    unimplemented!();
    unimplemented!();
    let mut m1: std_map_int__Tracked__std_less_int___std_allocator_std_pair_const_int__Tracked___ =
        <std_map_int__Tracked__std_less_int___std_allocator_std_pair_const_int__Tracked___>::default(
        );
    (unsafe {
        let mut _o: Tracked = Tracked::new_2({ 10_() });
        Tracked::move_assign(&mut m1[(1_()) as usize], &mut _o)
    });
    (unsafe {
        let mut _o: Tracked = Tracked::new_2({ 20_() });
        Tracked::move_assign(&mut m1[(2_()) as usize], &mut _o)
    });
    let mut m2: std_map_int__Tracked__std_less_int___std_allocator_std_pair_const_int__Tracked___ =
        <std_map_int__Tracked__std_less_int___std_allocator_std_pair_const_int__Tracked___>::default(
        );
    (unsafe {
        let mut _o: Tracked = Tracked::new_2({ 30_() });
        Tracked::move_assign(&mut m2[(3_()) as usize], &mut _o)
    });
    unimplemented!();
    (unsafe {
        let ___a: *mut Tracked = &mut m1[(1_()) as usize];
        let ___b: *mut Tracked = &mut m1[(2_()) as usize];
        swap_7(___a, ___b)
    });
    unimplemented!();
    unimplemented!();
    (unsafe { swap_8(&mut m1, &mut m2) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut arr: [Tracked; 3] = [
        Tracked::new_2({ 1_() }),
        Tracked::new_2({ 2_() }),
        Tracked::new_2({ 3_() }),
    ];
    (unsafe {
        let ___a: *mut Tracked = &mut arr[(0_()) as usize];
        let ___b: *mut Tracked = &mut arr[(2_()) as usize];
        swap_7(___a, ___b)
    });
    unimplemented!();
    (unsafe {
        let ___a: *mut Tracked = &mut arr[(1_()) as usize];
        let ___b: *mut Tracked = &mut arr[(1_()) as usize];
        swap_7(___a, ___b)
    });
    unimplemented!();
    unimplemented!();
    let mut vec_: std_vector_Tracked__std_allocator_Tracked__ =
        <std_vector_Tracked__std_allocator_Tracked__>::default();
    (unsafe {
        let mut ___args: () = 4_();
        vec_.emplace_back(&mut ___args)
    });
    (unsafe {
        let mut ___args: () = 5_();
        vec_.emplace_back(&mut ___args)
    });
    (unsafe {
        let ___a: *mut Tracked = &mut vec_[(0_()) as usize];
        let ___b: *mut Tracked = &mut vec_[(1_()) as usize];
        swap_7(___a, ___b)
    });
    unimplemented!();
    unimplemented!();
    let mut pairs: [Pair; 2] = [
        Pair {
            x: Tracked::new_2({ 6_() }),
            y: Tracked::new_2({ 7_() }),
        },
        Pair {
            x: Tracked::new_2({ 8_() }),
            y: Tracked::new_2({ 9_() }),
        },
    ];
    let mut p: Pair = Pair {
        x: Tracked::new_2({ 10_() }),
        y: Tracked::new_2({ 11_() }),
    };
    (unsafe { swap_7(&mut pairs[(0_()) as usize].x, &mut p.y) });
    unimplemented!();
    (unsafe {
        let ___a: *mut Pair = &mut pairs[(0_()) as usize];
        let ___b: *mut Pair = &mut pairs[(1_()) as usize];
        swap_9(___a, ___b)
    });
    unimplemented!();
    unimplemented!();
    (unsafe {
        let ___a: *mut Tracked = &mut p.x;
        let ___b: *mut Tracked = &mut p.y;
        swap_7(___a, ___b)
    });
    unimplemented!();
    (unsafe {
        let ___a: *mut Tracked = &mut pairs[(0_()) as usize].x;
        let ___b: *mut Tracked = &mut pairs[(1_()) as usize].y;
        swap_7(___a, ___b)
    });
    unimplemented!();
    unimplemented!();
    let mut arrays: Arrays = <Arrays>::default();
    arrays.a[(0_()) as usize].v = 1_();
    arrays.b[(1_()) as usize].v = 2_();
    (unsafe {
        let ___a: *mut Tracked = &mut arrays.a[(0_()) as usize];
        let ___b: *mut Tracked = &mut arrays.b[(1_()) as usize];
        swap_7(___a, ___b)
    });
    unimplemented!();
    (unsafe {
        let ___a: *mut Tracked = &mut arrays.a[(0_()) as usize];
        let ___b: *mut Tracked = &mut arrays.a[(1_()) as usize];
        swap_7(___a, ___b)
    });
    unimplemented!();
    unimplemented!();
    let mut grid : std_vector_std_vector_Tracked__std_allocator_Tracked____std_allocator_std_vector_Tracked__std_allocator_Tracked____ = std_vector_std_vector_Tracked__std_allocator_Tracked____std_allocator_std_vector_Tracked__std_allocator_Tracked____ :: new_1 ( { 2_() } , None , )  ;
    (unsafe {
        let mut ___args: () = 1_();
        grid[(0_()) as usize].emplace_back(&mut ___args)
    });
    (unsafe {
        let mut ___args: () = 2_();
        grid[(0_()) as usize].emplace_back(&mut ___args)
    });
    (unsafe {
        let mut ___args: () = 3_();
        grid[(1_()) as usize].emplace_back(&mut ___args)
    });
    (unsafe {
        let ___a: *mut Tracked = &mut grid[(0_()) as usize][(1_()) as usize];
        let ___b: *mut Tracked = &mut grid[(1_()) as usize][(0_()) as usize];
        swap_7(___a, ___b)
    });
    unimplemented!();
    (unsafe {
        let ___x: *mut std_vector_Tracked__std_allocator_Tracked__ = &mut grid[(0_()) as usize];
        let ___y: *mut std_vector_Tracked__std_allocator_Tracked__ = &mut grid[(1_()) as usize];
        swap_10(___x, ___y)
    });
    unimplemented!();
    unimplemented!();
    let mut maps : [ std_map_int__Tracked__std_less_int___std_allocator_std_pair_const_int__Tracked___ ; 2] = std::array::from_fn::<_, 2, _>(|_| <std_map_int__Tracked__std_less_int___std_allocator_std_pair_const_int__Tracked___ >::default()) ;
    (unsafe {
        let mut _o: Tracked = Tracked::new_2({ 1_() });
        Tracked::move_assign(&mut maps[(0_()) as usize][(1_()) as usize], &mut _o)
    });
    (unsafe {
        let mut _o: Tracked = Tracked::new_2({ 2_() });
        Tracked::move_assign(&mut maps[(1_()) as usize][(2_()) as usize], &mut _o)
    });
    (unsafe {
        let ___x: *mut std_map_int__Tracked__std_less_int___std_allocator_std_pair_const_int__Tracked___  = & mut maps [ ( 0_() ) as usize ] ;
        let ___y: *mut std_map_int__Tracked__std_less_int___std_allocator_std_pair_const_int__Tracked___  = & mut maps [ ( 1_() ) as usize ] ;
        swap_8(___x, ___y)
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut c1: CopyOnly = CopyOnly::new({ 1_() });
    let _dtor_c1 = ScopedDestructorUnsafe::new(&raw mut c1, CopyOnly::destructor);
    let mut c2: CopyOnly = CopyOnly::new({ 2_() });
    let _dtor_c2 = ScopedDestructorUnsafe::new(&raw mut c2, CopyOnly::destructor);
    (unsafe { swap_11(&mut c1, &mut c2) });
    unimplemented!();
    unimplemented!();
    let mut cp: CopyOnlyPair = CopyOnlyPair {
        x: CopyOnly::new({ 3_() }),
        y: CopyOnly::new({ 4_() }),
    };
    let _dtor_cp = ScopedDestructorUnsafe::new(&raw mut cp, CopyOnlyPair::destructor);
    (unsafe {
        let ___a: *mut CopyOnly = &mut cp.x;
        let ___b: *mut CopyOnly = &mut cp.y;
        swap_11(___a, ___b)
    });
    unimplemented!();
    unimplemented!();
    let mut carr: [CopyOnly; 2] = [CopyOnly::new({ 5_() }), CopyOnly::new({ 6_() })];
    (unsafe {
        let ___a: *mut CopyOnly = &mut carr[(0_()) as usize];
        let ___b: *mut CopyOnly = &mut carr[(1_()) as usize];
        swap_11(___a, ___b)
    });
    unimplemented!();
    unimplemented!();
    let mut mv1: Moved = Moved::new({ 1_() });
    let _dtor_mv1 = ScopedDestructorUnsafe::new(&raw mut mv1, Moved::destructor);
    let mut mv2: Moved = Moved::new({ 2_() });
    let _dtor_mv2 = ScopedDestructorUnsafe::new(&raw mut mv2, Moved::destructor);
    (unsafe { swap_12(&mut mv1, &mut mv2) });
    unimplemented!();
    unimplemented!();
    let mut mp: MovedPair = MovedPair {
        x: Moved::new({ 3_() }),
        y: Moved::new({ 4_() }),
    };
    let _dtor_mp = ScopedDestructorUnsafe::new(&raw mut mp, MovedPair::destructor);
    (unsafe {
        let ___a: *mut Moved = &mut mp.x;
        let ___b: *mut Moved = &mut mp.y;
        swap_12(___a, ___b)
    });
    unimplemented!();
    unimplemented!();
    let mut marr: [Moved; 2] = [Moved::new({ 5_() }), Moved::new({ 6_() })];
    (unsafe {
        let ___a: *mut Moved = &mut marr[(0_()) as usize];
        let ___b: *mut Moved = &mut marr[(1_()) as usize];
        swap_12(___a, ___b)
    });
    unimplemented!();
    unimplemented!();
    (unsafe {
        let ___a: *mut Moved = &mut mv1;
        let ___b: *mut Moved = &mut mv1;
        swap_12(___a, ___b)
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut o1: Owner = Owner {
        p: (unsafe {
            let mut ___args: () = 1_();
            make_unique_13(&mut ___args)
        }),
    };
    let mut o2: Owner = Owner {
        p: (unsafe {
            let mut ___args: () = 2_();
            make_unique_13(&mut ___args)
        }),
    };
    (unsafe { swap_14(&mut o1, &mut o2) });
    unimplemented!();
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0));
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const copies_0);
    std::cell::LazyCell::force(&*&raw const copy_only_copies_1);
    std::cell::LazyCell::force(&*&raw const copy_only_dtors_2);
    std::cell::LazyCell::force(&*&raw const moves_3);
    std::cell::LazyCell::force(&*&raw const move_assigns_4);
    std::cell::LazyCell::force(&*&raw const moved_dtors_5);
}
