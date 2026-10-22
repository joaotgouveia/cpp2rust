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
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe, Default)]
pub struct Counted {
    pub v: (),
}
impl Counted {
    pub unsafe fn new(mut v: ()) -> Self {
        let mut this = Self { v: v };
        this
    }
    pub unsafe fn copy_from(o: *const Counted) -> Self {
        let mut this = Self { v: (*o).v };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)).prefix_inc();
        this
    }
    pub unsafe fn copy_assign(&mut self, _a0: *const Counted) -> *mut Counted {
        self.v = (*_a0).v;
        return &mut (*(self as *mut Counted));
    }
}
impl Clone for Counted {
    fn clone(&self) -> Self {
        unsafe { Counted::copy_from(self as *const Counted) }
    }
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe)]
pub struct NonConst {
    pub mark: (),
}
impl NonConst {
    pub unsafe fn new() -> Self {
        let mut this = Self { mark: 0_() };
        this
    }
    pub unsafe fn new_1(o: *mut NonConst) -> Self {
        let mut this = Self {
            mark: (((*o).mark) + (1_())),
        };
        this
    }
    pub unsafe fn new_2(o: *const NonConst) -> Self {
        let mut this = Self {
            mark: (((*o).mark) + (10_())),
        };
        this
    }
    pub unsafe fn copy_assign(&mut self, _a0: *const NonConst) -> *mut NonConst {
        self.mark = (*_a0).mark;
        return &mut (*(self as *mut NonConst));
    }
}
impl Clone for NonConst {
    fn clone(&self) -> Self {
        unsafe { NonConst::new_1(self as *const NonConst as *mut NonConst) }
    }
}
impl Default for NonConst {
    fn default() -> Self {
        unsafe { NonConst::new() }
    }
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe, Default)]
pub struct Ignored {
    pub v: (),
}
impl Ignored {
    pub unsafe fn new(mut v: ()) -> Self {
        let mut this = Self { v: v };
        this
    }
    pub unsafe fn copy_from(_a0: *const Ignored) -> Self {
        let mut this = Self { v: -1_() };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)).prefix_inc();
        this
    }
    pub unsafe fn copy_assign(&mut self, _a0: *const Ignored) -> *mut Ignored {
        self.v = (*_a0).v;
        return &mut (*(self as *mut Ignored));
    }
}
impl Clone for Ignored {
    fn clone(&self) -> Self {
        unsafe { Ignored::copy_from(self as *const Ignored) }
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, CopyAssignUnsafe, Default)]
pub struct Holder {
    pub c: Counted,
    pub arr: [Counted; 2],
}
impl Holder {
    pub unsafe fn copy_assign(&mut self, _a0: *const Holder) -> *mut Holder {
        (unsafe {
            let _arg0: *const Counted = &(*_a0).c;
            Counted::copy_assign(&mut self.c, _arg0)
        });
        unimplemented!();
        return &mut (*(self as *mut Holder));
    }
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, Default)]
pub struct Box_int_ {
    pub val: (),
}
impl Box_int_ {
    pub unsafe fn new(mut v: ()) -> Self {
        let mut this = Self {
            val: (*unimplemented!()),
        };
        this
    }
    pub unsafe fn copy_from(o: *const Box_int_) -> Self {
        let mut this = Self { val: (*o).val };
        this
    }
    pub unsafe fn move_from(_a0: *mut Box_int_) -> Self {
        let mut this = Self { val: (*_a0).val };
        this
    }
}
impl Clone for Box_int_ {
    fn clone(&self) -> Self {
        unsafe { Box_int_::copy_from(self as *const Box_int_) }
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, MoveCtorUnsafe, Default)]
pub struct Owner {
    pub box_: Box_int_,
}
impl Owner {
    pub unsafe fn move_from(_a0: *mut Owner) -> Self {
        let mut this = Self {
            box_: Box_int_::move_from({ &mut (*_a0).box_ }),
        };
        this
    }
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe, Default)]
pub struct OutOfLine {
    pub v: (),
}
impl OutOfLine {
    pub unsafe fn new(mut v: ()) -> Self {
        let mut this = Self { v: v };
        this
    }
    pub unsafe fn copy_from(o: *const OutOfLine) -> Self {
        let mut this = Self {
            v: (((*o).v) + (100_())),
        };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)).prefix_inc();
        this
    }
    pub unsafe fn copy_assign(&mut self, _a0: *const OutOfLine) -> *mut OutOfLine {
        self.v = (*_a0).v;
        return &mut (*(self as *mut OutOfLine));
    }
}
impl Clone for OutOfLine {
    fn clone(&self) -> Self {
        unsafe { OutOfLine::copy_from(self as *const OutOfLine) }
    }
}
impl OutOfLine {}
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe)]
pub struct Tracked {
    pub copied_from: (),
}
impl Tracked {
    pub unsafe fn copy_from(o: *const Tracked) -> Self {
        let mut this = Self { copied_from: 0_() };
        (*(&raw const (*o).copied_from).cast_mut()).prefix_inc();
        this
    }
    pub unsafe fn copy_assign(&mut self, o: *const Tracked) -> *mut Tracked {
        (*(&raw const (*o).copied_from).cast_mut()).prefix_inc();
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
        unsafe { Tracked { copied_from: 0_() } }
    }
}
pub unsafe fn touch_1(mut t: *const Tracked) {
    (*(&raw const (*t).copied_from).cast_mut()).prefix_inc();
}
pub unsafe fn by_value_2(mut c: Counted) -> () {
    return c.v;
}
pub unsafe fn make_3(mut v: ()) -> Counted {
    let mut c: Counted = Counted::new({ v });
    return Counted::copy_from({ &(*unimplemented!()) });
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut a: Counted = Counted::new({ 1_() });
    let mut b: Counted = Counted::copy_from({ &a });
    let mut c: Counted = Counted::copy_from({ &a });
    let mut d: Counted = Counted::copy_from({ &a });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut e: Counted = (unsafe { make_3(5_()) });
    unimplemented!();
    unimplemented!();
    let mut f: Counted = Counted::new({ 6_() });
    unimplemented!();
    unimplemented!();
    let g: Counted = Counted::new({ 7_() });
    let mut h: Counted = Counted::copy_from({ &g });
    unimplemented!();
    unimplemented!();
    let mut hold: Holder = Holder {
        c: Counted::new({ 8_() }),
        arr: [Counted::new({ 9_() }), Counted::new({ 10_() })],
    };
    let mut hold2: Holder = hold.clone();
    unimplemented!();
    unimplemented!();
    let mut vec_: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut i1: Ignored = Ignored::new({ 1_() });
    let mut i2: Ignored = Ignored::copy_from({ &i1 });
    unimplemented!();
    unimplemented!();
    let mut o: Owner = Owner {
        box_: Box_int_::new({ 3_() }),
    };
    let mut o2: Owner = Owner::move_from({ &mut (*unimplemented!()) });
    unimplemented!();
    let mut ol1: OutOfLine = OutOfLine::new({ 5_() });
    let mut ol2: OutOfLine = OutOfLine::copy_from({ &ol1 });
    unimplemented!();
    unimplemented!();
    let mut n: NonConst = NonConst::new();
    let mut n1: NonConst = NonConst::new_1({ &mut n });
    let cn: NonConst = NonConst::new();
    let mut n2: NonConst = NonConst::new_2({ &cn });
    unimplemented!();
    unimplemented!();
    let mut t1: Tracked = <Tracked>::default();
    let mut t2: Tracked = Tracked::copy_from({ &t1 });
    (unsafe { Tracked::copy_assign(&mut t2, &t1) });
    unimplemented!();
    let ct: Tracked = <Tracked>::default();
    let mut t3: Tracked = Tracked::copy_from({ &ct });
    (unsafe { touch_1((&ct as *const Tracked)) });
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const copies_0);
}
