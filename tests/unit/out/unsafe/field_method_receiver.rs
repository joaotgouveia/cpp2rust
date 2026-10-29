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
pub struct Counter {
    pub n: (),
}
impl Counter {
    pub unsafe fn get(&self) -> () {
        return self.n;
    }
    pub unsafe fn add(&mut self, mut k: ()) {
        self.n += k;
    }
    pub unsafe fn self_(&mut self) -> *mut Counter {
        return (self as *mut Counter);
    }
    pub unsafe fn take(&mut self, mut other: *mut Counter) {
        self.n += (*other).n;
        (*other).n = 0_();
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct S {
    pub tag: (),
    pub c: Counter,
    pub arr: [Counter; 2],
    pub v: std_vector_int__std_allocator_int__,
}
impl S {
    pub unsafe fn bump(&mut self) {
        (unsafe {
            let _k: () = self.tag;
            Counter::add(&mut self.c, _k)
        });
    }
    pub unsafe fn move_from(_a0: *mut S) -> Self {
        let mut this = Self {
            tag: (*_a0).tag,
            c: (*_a0).c,
            arr: std::array::from_fn::<_, 2, _>(|__i: usize| (*_a0).arr[(__i) as usize]),
            v: (*_a0).v,
        };
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut S) -> *mut S {
        self.tag = (*_a0).tag;
        self.c = (*_a0).c;
        unimplemented!();
        self.v = ((*_a0).v).clone();
        return &mut (*(self as *mut S));
    }
}
pub unsafe fn run_0(mut o: *mut S) {
    (unsafe { Counter::add(&mut (*o).c, 2_()) });
    unimplemented!();
    (unsafe { Counter::add(&mut (*o).arr[(1_()) as usize], 5_()) });
    unimplemented!();
    (unsafe { Counter::add(&mut (*(unsafe { Counter::self_(&mut (*o).c) })), 1_()) });
    unimplemented!();
    unimplemented!();
    (unsafe {
        let _other: *mut Counter = (&mut (*o).arr[(1_()) as usize] as *mut Counter);
        Counter::take(&mut (*o).arr[(0_()) as usize], _other)
    });
    unimplemented!();
    (unsafe {
        let _other: *mut Counter = (&mut (*o).c as *mut Counter);
        Counter::take(&mut (*o).c, _other)
    });
    unimplemented!();
    (unsafe { S::bump(&mut (*o)) });
    unimplemented!();
    (unsafe {
        let mut ___x: () = (unsafe { Counter::get(&(*o).c) });
        (*o).v.push_back_58(&mut ___x)
    });
    unimplemented!();
    unimplemented!();
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut local: S = <S>::default();
    local.tag = 1_();
    (unsafe { run_0((&mut local as *mut S)) });
    let mut heap: *mut S = (Box::leak(Box::new(<S>::default())) as *mut S);
    (*heap).tag = 1_();
    (unsafe { run_0(heap) });
    {
        let __p = heap;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(__p))
        }
    };
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
