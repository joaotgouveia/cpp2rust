extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe)]
pub struct Counted {
    pub copies: (),
    pub moves: (),
}
impl Counted {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            copies: 0_(),
            moves: 0_(),
        };
        this
    }
    pub unsafe fn copy_from(o: *const Counted) -> Self {
        let mut this = Self {
            copies: (((*o).copies) + (1_())),
            moves: (*o).moves,
        };
        this
    }
    pub unsafe fn move_from(o: *mut Counted) -> Self {
        let mut this = Self {
            copies: (*o).copies,
            moves: (((*o).moves) + (1_())),
        };
        this
    }
}
impl Clone for Counted {
    fn clone(&self) -> Self {
        unsafe { Counted::copy_from(self as *const Counted) }
    }
}
impl Default for Counted {
    fn default() -> Self {
        unsafe { Counted::new() }
    }
}
pub static mut drops_0: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, DestructorUnsafe)]
pub struct Dropped {}
impl Dropped {
    pub unsafe fn new() -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn copy_from(_a0: *const Dropped) -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn move_from(_a0: *mut Dropped) -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn destructor(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut drops_0)).postfix_inc();
    }
}
impl Clone for Dropped {
    fn clone(&self) -> Self {
        unsafe { Dropped::copy_from(self as *const Dropped) }
    }
}
impl Default for Dropped {
    fn default() -> Self {
        unsafe { Dropped::new() }
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut c: Counted = Counted::new();
    let mut f: FnPtr<fn() -> ()> = lambda_unsafe!(
        {
            let c: Counted = Counted::copy_from({ &c });
        },
        || -> () {
            return (((c.copies) * (10_())) + (c.moves));
        }
    );
    unimplemented!();
    let mut g: FnPtr<fn() -> ()> = f.copy_from();
    unimplemented!();
    unimplemented!();
    let mut h: () = f.move_from();
    unimplemented!();
    let mut returned: () = (unsafe {
        lambda_unsafe!(
            {
                let c: Counted = Counted::copy_from({ &c });
            },
            || -> Counted {
                return Counted::copy_from({ &c });
            }
        )
        .call()
    })
    .copies;
    unimplemented!();
    let mut arr: [Counted; 2] = std::array::from_fn::<_, 2, _>(|_| Counted::new());
    let mut a: FnPtr<fn() -> ()> = lambda_unsafe!(
        {
            let arr: [Counted; 2] = std::array::from_fn::<_, 2, _>(|__i: usize| {
                Counted::copy_from({ &arr[(__i) as usize] })
            });
        },
        || -> () {
            return ((arr[(0_()) as usize].copies) + (arr[(1_()) as usize].copies));
        }
    );
    unimplemented!();
    let mut a2: FnPtr<fn() -> ()> = a.copy_from();
    unimplemented!();
    let mut a3: () = a.move_from();
    unimplemented!();
    {
        let mut m: FnPtr<fn()> = lambda_unsafe!(
            {
                let d: Dropped = Dropped::new();
            },
            || {}
        );
        let _dtor_m = ScopedDestructorUnsafe::new(&raw mut m, |__f| __f.destroy());
        let mut m2: () = m.move_from();
        let _dtor_m2 = ScopedDestructorUnsafe::new(&raw mut m2, |__f| __f.destroy());
    }
    unimplemented!();
    {
        let mut k: FnPtr<fn()> = lambda_unsafe!(
            {
                let d: Dropped = Dropped::new();
            },
            || {}
        );
        let _dtor_k = ScopedDestructorUnsafe::new(&raw mut k, |__f| __f.destroy());
        let mut k2: FnPtr<fn()> = k.copy_from();
        let _dtor_k2 = ScopedDestructorUnsafe::new(&raw mut k2, |__f| __f.destroy());
    }
    unimplemented!();
    {
        let mut inner: FnPtr<fn()> = lambda_unsafe!(
            {
                let d: Dropped = Dropped::new();
            },
            || {}
        );
        let _dtor_inner = ScopedDestructorUnsafe::new(&raw mut inner, |__f| __f.destroy());
        let mut outer: FnPtr<fn()> = lambda_unsafe!(
            {
                let inner: FnPtr<fn()> = inner.copy_from();
            },
            || {}
        );
        let _dtor_outer = ScopedDestructorUnsafe::new(&raw mut outer, |__f| __f.destroy());
    }
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const drops_0);
}
