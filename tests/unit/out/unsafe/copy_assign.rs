extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut assigns_0: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe, Default)]
pub struct Partial {
    pub v: (),
    pub keep: (),
}
impl Partial {
    pub unsafe fn new(mut v: (), mut keep: ()) -> Self {
        let mut this = Self { v: v, keep: keep };
        this
    }
    pub unsafe fn copy_from(o: *const Partial) -> Self {
        let mut this = Self {
            v: (*o).v,
            keep: (*o).keep,
        };
        this
    }
    pub unsafe fn copy_assign(&mut self, o: *const Partial) -> *mut Partial {
        if (((self as *mut Partial).cast_const()) == (o)) {
            return &mut (*(self as *mut Partial));
        }
        self.v = (*o).v;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut assigns_0)).prefix_inc();
        return &mut (*(self as *mut Partial));
    }
}
impl Clone for Partial {
    fn clone(&self) -> Self {
        unsafe { Partial::copy_from(self as *const Partial) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct NonConstAssign {
    pub mark: (),
}
impl NonConstAssign {
    pub unsafe fn new() -> Self {
        let mut this = Self { mark: 0_() };
        this
    }
    pub unsafe fn operator_assign_2(&mut self, o: *mut NonConstAssign) -> *mut NonConstAssign {
        self.mark = (((*o).mark) + (1_()));
        return &mut (*(self as *mut NonConstAssign));
    }
    pub unsafe fn operator_assign_3(&mut self, o: *const NonConstAssign) -> *mut NonConstAssign {
        self.mark = (((*o).mark) + (10_()));
        return &mut (*(self as *mut NonConstAssign));
    }
}
impl Default for NonConstAssign {
    fn default() -> Self {
        unsafe { NonConstAssign::new() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CopyAssignUnsafe)]
pub struct RefQualified {
    pub mark: (),
}
impl RefQualified {
    pub unsafe fn new() -> Self {
        let mut this = Self { mark: 0_() };
        this
    }
    pub unsafe fn copy_assign(&mut self, o: *const RefQualified) -> *mut RefQualified {
        self.mark = (((*o).mark) + (1_()));
        return &mut (*(self as *mut RefQualified));
    }
}
impl Default for RefQualified {
    fn default() -> Self {
        unsafe { RefQualified::new() }
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, CopyAssignUnsafe, Default)]
pub struct Holder {
    pub p: Partial,
    pub arr: [Partial; 2],
}
impl Holder {
    pub unsafe fn copy_assign(&mut self, _a0: *const Holder) -> *mut Holder {
        (unsafe {
            let _o: *const Partial = &(*_a0).p;
            Partial::copy_assign(&mut self.p, _o)
        });
        let mut __i0: () = 0_();
        'loop_: while ((__i0) != (2_())) {
            (unsafe {
                let _o: *const Partial = &(*_a0).arr[(__i0) as usize];
                Partial::copy_assign(&mut self.arr[(__i0) as usize], _o)
            });
            __i0.prefix_inc();
        }
        return &mut (*(self as *mut Holder));
    }
}
pub static mut copies_1: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe, Default)]
pub struct CopyCounted {
    pub v: (),
}
impl CopyCounted {
    pub unsafe fn new(mut v: ()) -> Self {
        let mut this = Self { v: v };
        this
    }
    pub unsafe fn copy_from(o: *const CopyCounted) -> Self {
        let mut this = Self { v: (*o).v };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copies_1)).prefix_inc();
        this
    }
    pub unsafe fn copy_assign(&mut self, _a0: *const CopyCounted) -> *mut CopyCounted {
        self.v = (*_a0).v;
        return &mut (*(self as *mut CopyCounted));
    }
}
impl Clone for CopyCounted {
    fn clone(&self) -> Self {
        unsafe { CopyCounted::copy_from(self as *const CopyCounted) }
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, CopyAssignUnsafe, Default)]
pub struct HasCopyCounted {
    pub c: CopyCounted,
    pub arr: [CopyCounted; 2],
}
impl HasCopyCounted {
    pub unsafe fn copy_assign(&mut self, _a0: *const HasCopyCounted) -> *mut HasCopyCounted {
        (unsafe {
            let _arg0: *const CopyCounted = &(*_a0).c;
            CopyCounted::copy_assign(&mut self.c, _arg0)
        });
        unimplemented!();
        return &mut (*(self as *mut HasCopyCounted));
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut a: Partial = Partial::new({ 1_() }, { 100_() });
    let mut b: Partial = Partial::new({ 2_() }, { 200_() });
    let mut c: Partial = Partial::new({ 3_() }, { 300_() });
    (unsafe { Partial::copy_assign(&mut a, &b) });
    unimplemented!();
    unimplemented!();
    (unsafe { Partial::copy_assign(&mut c, &(*(unsafe { Partial::copy_assign(&mut a, &b) }))) });
    unimplemented!();
    unimplemented!();
    (unsafe {
        let _o: *const Partial = &a;
        Partial::copy_assign(&mut a, _o)
    });
    unimplemented!();
    (unsafe {
        let mut _o: Partial = Partial::new({ 9_() }, { 900_() });
        Partial::copy_assign(&mut a, &mut _o)
    });
    unimplemented!();
    unimplemented!();
    let ra: *mut Partial = &mut a;
    (unsafe {
        let _o: *const Partial = &c;
        Partial::copy_assign(&mut (*ra), _o)
    });
    unimplemented!();
    let mut pa: *mut Partial = (&mut a as *mut Partial);
    (unsafe {
        let _o: *const Partial = &b;
        Partial::copy_assign(&mut (*pa), _o)
    });
    unimplemented!();
    unimplemented!();
    let mut h: Holder = Holder {
        p: Partial::new({ 4_() }, { 40_() }),
        arr: [
            Partial::new({ 5_() }, { 50_() }),
            Partial::new({ 6_() }, { 60_() }),
        ],
    };
    (unsafe { Partial::copy_assign(&mut h.p, &b) });
    (unsafe { Partial::copy_assign(&mut h.arr[(1_()) as usize], &c) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut n: NonConstAssign = NonConstAssign::new();
    let mut n1: NonConstAssign = NonConstAssign::new();
    let mut n2: NonConstAssign = NonConstAssign::new();
    let cn: NonConstAssign = NonConstAssign::new();
    (unsafe { NonConstAssign::operator_assign_2(&mut n1, &mut n) });
    (unsafe { NonConstAssign::operator_assign_3(&mut n2, &cn) });
    unimplemented!();
    unimplemented!();
    let mut r: RefQualified = RefQualified::new();
    let mut r1: RefQualified = RefQualified::new();
    (unsafe { RefQualified::copy_assign(&mut r1, &r) });
    unimplemented!();
    let mut h2: Holder = Holder {
        p: Partial::new({ 7_() }, { 70_() }),
        arr: [
            Partial::new({ 8_() }, { 80_() }),
            Partial::new({ 9_() }, { 90_() }),
        ],
    };
    (unsafe { Holder::copy_assign(&mut h2, &h) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut cc1: CopyCounted = CopyCounted::new({ 1_() });
    let mut cc2: CopyCounted = CopyCounted::new({ 2_() });
    (unsafe { CopyCounted::copy_assign(&mut cc1, &cc2) });
    unimplemented!();
    let mut hc1: HasCopyCounted = HasCopyCounted {
        c: CopyCounted::new({ 1_() }),
        arr: [CopyCounted::new({ 2_() }), CopyCounted::new({ 3_() })],
    };
    let mut hc2: HasCopyCounted = HasCopyCounted {
        c: CopyCounted::new({ 4_() }),
        arr: [CopyCounted::new({ 5_() }), CopyCounted::new({ 6_() })],
    };
    (unsafe { HasCopyCounted::copy_assign(&mut hc1, &hc2) });
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const assigns_0);
    std::cell::LazyCell::force(&*&raw const copies_1);
}
