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
pub struct Inner {
    pub x: (),
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct Explicit {
    pub v: (),
    pub inner: Inner,
    pub arr: [(); 2],
}
impl Explicit {
    pub unsafe fn new(mut v: ()) -> Self {
        let mut this = Self {
            v: v,
            inner: Inner { x: ((v) * (10_())) },
            arr: [v, ((v) + (1_()))],
        };
        this
    }
    pub unsafe fn destructor(&mut self) {}
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Implicit {
    pub v: (),
    pub inner: Inner,
    pub arr: [(); 2],
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct DefaultCopyUserMove {
    pub v: (),
}
impl DefaultCopyUserMove {
    pub unsafe fn new(mut v: ()) -> Self {
        let mut this = Self { v: v };
        this
    }
    pub unsafe fn move_from(o: *mut DefaultCopyUserMove) -> Self {
        let mut this = Self { v: (*o).v };
        (*o).v = 0_();
        this
    }
    pub unsafe fn move_assign(&mut self, o: *mut DefaultCopyUserMove) -> *mut DefaultCopyUserMove {
        self.v = (*o).v;
        (*o).v = 0_();
        return &mut (*(self as *mut DefaultCopyUserMove));
    }
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, CopyAssignUnsafe, MoveAssignUnsafe, Default)]
pub struct UserCopyDefaultMove {
    pub v: (),
}
impl UserCopyDefaultMove {
    pub unsafe fn new(mut v: ()) -> Self {
        let mut this = Self { v: v };
        this
    }
    pub unsafe fn copy_from(o: *const UserCopyDefaultMove) -> Self {
        let mut this = Self {
            v: (((*o).v) + (100_())),
        };
        this
    }
    pub unsafe fn move_from(_a0: *mut UserCopyDefaultMove) -> Self {
        let mut this = Self { v: (*_a0).v };
        this
    }
    pub unsafe fn copy_assign(
        &mut self,
        o: *const UserCopyDefaultMove,
    ) -> *mut UserCopyDefaultMove {
        self.v = (((*o).v) + (100_()));
        return &mut (*(self as *mut UserCopyDefaultMove));
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut UserCopyDefaultMove,
    ) -> *mut UserCopyDefaultMove {
        self.v = (*_a0).v;
        return &mut (*(self as *mut UserCopyDefaultMove));
    }
}
impl Clone for UserCopyDefaultMove {
    fn clone(&self) -> Self {
        unsafe { UserCopyDefaultMove::copy_from(self as *const UserCopyDefaultMove) }
    }
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct Buffer { pub data : std_vector_int__std_allocator_int__ , pub rows : std_vector_std_vector_int__std_allocator_int____std_allocator_std_vector_int__std_allocator_int____ , pub n : () , pub arr : [ () ; 2] , }
impl Buffer {
    pub unsafe fn new(mut n: ()) -> Self {
        let mut this = Self { data : std_vector_int__std_allocator_int__ :: new_1 ( { n } , { & n } , None , )  , rows : <std_vector_std_vector_int__std_allocator_int____std_allocator_std_vector_int__std_allocator_int____ >::default() , n : n , arr : [ n , ( ( n ) + ( 1_() ) ) , ] , } ;
        (unsafe {
            let ___x: *const std_vector_int__std_allocator_int__ = &this.data;
            this.rows.push_back_57(___x)
        });
        this
    }
    pub unsafe fn move_from(_a0: *mut Buffer) -> Self {
        let mut this = Self {
            data: (*_a0).data,
            rows: (*_a0).rows,
            n: (*_a0).n,
            arr: std::array::from_fn::<_, 2, _>(|__i: usize| (*_a0).arr[(__i) as usize]),
        };
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut Buffer) -> *mut Buffer {
        self.data = ((*_a0).data).clone();
        self.rows = ((*_a0).rows).clone();
        self.n = (*_a0).n;
        unimplemented!();
        return &mut (*(self as *mut Buffer));
    }
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct Owner {
    pub data: std_vector_int__std_allocator_int__,
    pub n: (),
    pub arr: [(); 2],
    pub p: std_unique_ptr_int__std_default_delete_int__,
}
impl Owner {
    pub unsafe fn move_from(_a0: *mut Owner) -> Self {
        let mut this = Self {
            data: (*_a0).data,
            n: (*_a0).n,
            arr: std::array::from_fn::<_, 2, _>(|__i: usize| (*_a0).arr[(__i) as usize]),
            p: (*_a0).p,
        };
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut Owner) -> *mut Owner {
        self.data = ((*_a0).data).clone();
        self.n = (*_a0).n;
        unimplemented!();
        self.p = ((*_a0).p).clone();
        return &mut (*(self as *mut Owner));
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct Copyable {
    pub data: std_vector_int__std_allocator_int__,
}
impl Copyable {
    pub unsafe fn move_from(_a0: *mut Copyable) -> Self {
        let mut this = Self { data: (*_a0).data };
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut Copyable) -> *mut Copyable {
        self.data = ((*_a0).data).clone();
        return &mut (*(self as *mut Copyable));
    }
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, DestructorUnsafe, Default)]
pub struct Holder {
    pub inner: Inner,
    pub e: Explicit,
    pub p: std_unique_ptr_int__std_default_delete_int__,
}
impl Holder {
    pub unsafe fn new(mut v: ()) -> Self {
        let mut this = Self {
            inner: Inner { x: v },
            e: Explicit::new({ v }),
            p: <std_unique_ptr_int__std_default_delete_int__>::default(),
        };
        this
    }
    pub unsafe fn move_from(_a0: *mut Holder) -> Self {
        let mut this = Self {
            inner: (*_a0).inner,
            e: (*_a0).e.clone(),
            p: (*_a0).p,
        };
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut Holder) -> *mut Holder {
        self.inner = (*_a0).inner;
        self.e = ((*_a0).e).clone();
        self.p = ((*_a0).p).clone();
        return &mut (*(self as *mut Holder));
    }
}
impl Holder {
    pub unsafe fn destructor(&mut self) {
        Explicit::destructor(&mut self.e);
    }
}
pub unsafe fn same_0(a: *const Explicit, b: *const Explicit) -> () {
    return (((((*a).v) == ((*b).v)) && (((*a).inner.x) == ((*b).inner.x)))
        && (((*a).arr[(0_()) as usize]) == ((*b).arr[(0_()) as usize])))
        && (((*a).arr[(1_()) as usize]) == ((*b).arr[(1_()) as usize]));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut a: Explicit = Explicit::new({ 1_() });
    let _dtor_a = ScopedDestructorUnsafe::new(&raw mut a, Explicit::destructor);
    let mut b: Explicit = a.clone();
    let _dtor_b = ScopedDestructorUnsafe::new(&raw mut b, Explicit::destructor);
    let mut c: Explicit = a.clone();
    let _dtor_c = ScopedDestructorUnsafe::new(&raw mut c, Explicit::destructor);
    let mut d: Explicit = a.clone();
    let _dtor_d = ScopedDestructorUnsafe::new(&raw mut d, Explicit::destructor);
    unimplemented!();
    let mut e: Explicit = Explicit::new({ 2_() });
    let _dtor_e = ScopedDestructorUnsafe::new(&raw mut e, Explicit::destructor);
    let mut f: Explicit = Explicit::new({ 3_() });
    let _dtor_f = ScopedDestructorUnsafe::new(&raw mut f, Explicit::destructor);
    e = (b).clone();
    f = (c).clone();
    unimplemented!();
    let mut g: Explicit = Explicit::new({ 4_() });
    let _dtor_g = ScopedDestructorUnsafe::new(&raw mut g, Explicit::destructor);
    g = {
        e = (f).clone();
        (e).clone()
    };
    unimplemented!();
    let mut i: Implicit = Implicit {
        v: 5_(),
        inner: Inner { x: 50_() },
        arr: [5_(), 6_()],
    };
    let mut j: Implicit = i;
    let mut k: Implicit = i;
    unimplemented!();
    unimplemented!();
    let mut l: Implicit = Implicit {
        v: 0_(),
        inner: Inner { x: 0_() },
        arr: [0_(), 0_()],
    };
    l = j;
    unimplemented!();
    let mut vec_: std_vector_Explicit__std_allocator_Explicit__ =
        <std_vector_Explicit__std_allocator_Explicit__>::default();
    (unsafe { vec_.push_back_57(&b) });
    (unsafe {
        let mut ___x: Explicit = Explicit::new({ 9_() });
        vec_.push_back_58(&mut ___x)
    });
    unimplemented!();
    let mut m: DefaultCopyUserMove = DefaultCopyUserMove::new({ 7_() });
    let mut m1: DefaultCopyUserMove = m;
    let mut m2: DefaultCopyUserMove = DefaultCopyUserMove::move_from({ &mut m });
    unimplemented!();
    let mut m3: DefaultCopyUserMove = DefaultCopyUserMove::new({ 1_() });
    let mut m4: DefaultCopyUserMove = DefaultCopyUserMove::new({ 1_() });
    m3 = m1;
    (unsafe { DefaultCopyUserMove::move_assign(&mut m4, &mut m1) });
    unimplemented!();
    let mut u: UserCopyDefaultMove = UserCopyDefaultMove::new({ 8_() });
    let mut u1: UserCopyDefaultMove = UserCopyDefaultMove::copy_from({ &u });
    let mut u2: UserCopyDefaultMove = UserCopyDefaultMove::move_from({ &mut u });
    unimplemented!();
    let mut u3: UserCopyDefaultMove = UserCopyDefaultMove::new({ 1_() });
    let mut u4: UserCopyDefaultMove = UserCopyDefaultMove::new({ 1_() });
    (unsafe { UserCopyDefaultMove::copy_assign(&mut u3, &u2) });
    (unsafe { UserCopyDefaultMove::move_assign(&mut u4, &mut u2) });
    unimplemented!();
    let mut p: Buffer = Buffer::new({ 3_() });
    let mut q: Buffer = Buffer::move_from({ &mut p });
    unimplemented!();
    let mut r: Buffer = Buffer::new({ 1_() });
    (unsafe { Buffer::move_assign(&mut r, &mut q) });
    unimplemented!();
    unimplemented!();
    let mut bufs: std_vector_Buffer__std_allocator_Buffer__ =
        <std_vector_Buffer__std_allocator_Buffer__>::default();
    (unsafe { bufs.push_back_58(&mut r) });
    (unsafe {
        let ___args: *mut Buffer = &mut bufs[(0_()) as usize];
        bufs.emplace_back(___args)
    });
    unimplemented!();
    let mut o1: Owner = <Owner>::default();
    (unsafe {
        let mut ___x: () = 5_();
        o1.data.push_back_58(&mut ___x)
    });
    o1.n = 5_();
    o1.arr[(0_()) as usize] = 5_();
    o1.arr[(1_()) as usize] = 6_();
    (unsafe { o1.p.reset((Box::leak(Box::new(7_())) as *mut ())) });
    let mut o2: Owner = Owner::move_from({ &mut o1 });
    unimplemented!();
    unimplemented!();
    let mut o3: Owner = <Owner>::default();
    (unsafe { Owner::move_assign(&mut o3, &mut o2) });
    unimplemented!();
    unimplemented!();
    let mut c1: Copyable = Copyable {
        data: std_vector_int__std_allocator_int__::new_2({ unimplemented!() }, None),
    };
    let mut c2: Copyable = c1.clone();
    let mut c3: Copyable = Copyable::move_from({ &mut c1 });
    unimplemented!();
    unimplemented!();
    let mut c4: Copyable = <Copyable>::default();
    (unsafe { Copyable::move_assign(&mut c4, &mut c3) });
    unimplemented!();
    let mut h1: Holder = Holder::new({ 4_() });
    let _dtor_h1 = ScopedDestructorUnsafe::new(&raw mut h1, Holder::destructor);
    (unsafe { h1.p.reset((Box::leak(Box::new(9_())) as *mut ())) });
    let mut h2: Holder = Holder::move_from({ &mut h1 });
    let _dtor_h2 = ScopedDestructorUnsafe::new(&raw mut h2, Holder::destructor);
    unimplemented!();
    let mut h3: Holder = Holder::new({ 1_() });
    let _dtor_h3 = ScopedDestructorUnsafe::new(&raw mut h3, Holder::destructor);
    (unsafe { Holder::move_assign(&mut h3, &mut h2) });
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
