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
#[derive(Clone, VaArg, FnPtrArg, CopyAssignUnsafe, Default)]
pub struct Arrays {
    pub a: [Tracked; 2],
    pub b: [Tracked; 2],
}
impl Arrays {
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
#[derive(Clone, VaArg, FnPtrArg, CopyAssignUnsafe, DestructorUnsafe, Default)]
pub struct MovedPair {
    pub x: Moved,
    pub y: Moved,
}
impl MovedPair {
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
    pub p: (),
}
impl Owner {
    pub unsafe fn move_from(_a0: *mut Owner) -> Self {
        let mut this = Self {
            p: unimplemented!(),
        };
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut Owner) -> *mut Owner {
        unimplemented!();
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
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut m1: () = unimplemented!();
    (unsafe {
        let mut _o: Tracked = Tracked::new_2({ 10_() });
        Tracked::move_assign(&mut (*unimplemented!()), &mut _o)
    });
    (unsafe {
        let mut _o: Tracked = Tracked::new_2({ 20_() });
        Tracked::move_assign(&mut (*unimplemented!()), &mut _o)
    });
    let mut m2: () = unimplemented!();
    (unsafe {
        let mut _o: Tracked = Tracked::new_2({ 30_() });
        Tracked::move_assign(&mut (*unimplemented!()), &mut _o)
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut arr: [Tracked; 3] = [
        Tracked::new_2({ 1_() }),
        Tracked::new_2({ 2_() }),
        Tracked::new_2({ 3_() }),
    ];
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut vec_: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
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
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut arrays: Arrays = <Arrays>::default();
    arrays.a[(0_()) as usize].v = 1_();
    arrays.b[(1_()) as usize].v = 2_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut grid: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut maps: [(); 2] = std::array::from_fn::<_, 2, _>(|_| unimplemented!());
    (unsafe {
        let mut _o: Tracked = Tracked::new_2({ 1_() });
        Tracked::move_assign(&mut (*unimplemented!()), &mut _o)
    });
    (unsafe {
        let mut _o: Tracked = Tracked::new_2({ 2_() });
        Tracked::move_assign(&mut (*unimplemented!()), &mut _o)
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut c1: CopyOnly = CopyOnly::new({ 1_() });
    let _dtor_c1 = ScopedDestructorUnsafe::new(&raw mut c1, CopyOnly::destructor);
    let mut c2: CopyOnly = CopyOnly::new({ 2_() });
    let _dtor_c2 = ScopedDestructorUnsafe::new(&raw mut c2, CopyOnly::destructor);
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut cp: CopyOnlyPair = CopyOnlyPair {
        x: CopyOnly::new({ 3_() }),
        y: CopyOnly::new({ 4_() }),
    };
    let _dtor_cp = ScopedDestructorUnsafe::new(&raw mut cp, CopyOnlyPair::destructor);
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut carr: [CopyOnly; 2] = [CopyOnly::new({ 5_() }), CopyOnly::new({ 6_() })];
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut mv1: Moved = Moved::new({ 1_() });
    let _dtor_mv1 = ScopedDestructorUnsafe::new(&raw mut mv1, Moved::destructor);
    let mut mv2: Moved = Moved::new({ 2_() });
    let _dtor_mv2 = ScopedDestructorUnsafe::new(&raw mut mv2, Moved::destructor);
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut mp: MovedPair = MovedPair {
        x: Moved::new({ 3_() }),
        y: Moved::new({ 4_() }),
    };
    let _dtor_mp = ScopedDestructorUnsafe::new(&raw mut mp, MovedPair::destructor);
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut marr: [Moved; 2] = [Moved::new({ 5_() }), Moved::new({ 6_() })];
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut o1: Owner = Owner {
        p: unimplemented!(),
    };
    let mut o2: Owner = Owner {
        p: unimplemented!(),
    };
    unimplemented!();
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
