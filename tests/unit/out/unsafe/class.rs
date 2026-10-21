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
pub struct Pair {
    pub first: (),
    pub second: (),
}
impl Pair {
    pub unsafe fn NOP(&mut self) {}
    pub unsafe fn GetFirst(&self) -> () {
        return self.first;
    }
    pub unsafe fn GetSecond(&self) -> () {
        return self.second;
    }
    pub unsafe fn Set(&mut self, field: *mut (), mut new_val: ()) -> () {
        (unsafe { Pair::NOP(self) });
        let mut old_val: () = (*field);
        (*field) = new_val;
        return old_val;
    }
    pub unsafe fn SetFirst(&mut self, mut new_first: ()) -> () {
        return ((unsafe { Pair::GetFirst(self) })
            + (unsafe {
                let _field: *mut () = &mut self.first;
                Pair::Set(self, _field, new_first)
            }));
    }
    pub unsafe fn SetSecond(&mut self, mut new_second: ()) -> () {
        return ((unsafe { Pair::GetSecond(self) })
            + (unsafe {
                let _field: *mut () = &mut self.second;
                Pair::Set(self, _field, new_second)
            }));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Route {
    pub path: Pair,
    pub cost: (),
}
impl Route {
    pub unsafe fn SetCost(&mut self, mut new_cost: ()) -> () {
        let mut old_cost: () = self.cost;
        self.cost = new_cost;
        return old_cost;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Counter {
    pub v: (),
    pub calls: (),
}
impl Counter {
    pub unsafe fn Get(&self) -> () {
        (*(&raw const self.calls).cast_mut()).prefix_inc();
        return self.v;
    }
    pub unsafe fn operator_eq(&self, o: *const Counter) -> () {
        (*(&raw const self.calls).cast_mut()).prefix_inc();
        return ((self.v) == ((*o).v));
    }
}
impl std::cmp::PartialEq for Counter {
    fn eq(&self, other: &Self) -> bool {
        unsafe { Counter::operator_eq(self, other as *const Counter) }
    }
}
impl std::cmp::Eq for Counter {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Tally {
    pub calls: (),
}
impl Tally {
    pub unsafe fn SameNameBetweenConstAndNonConst_1(&mut self) -> () {
        self.calls += 1_();
        return 1_();
    }
    pub unsafe fn SameNameBetweenConstAndNonConst_2(&self) -> () {
        (*(&raw const self.calls).cast_mut()) += 10_();
        return 2_();
    }
}
pub unsafe fn RandomRoute_0(route: *mut Route) -> () {
    if ((((*route).path.first) % (2_())) != 0) {
        return (unsafe {
            let _new_first: () = (unsafe { Pair::SetSecond(&mut (*route).path, 10_()) });
            Pair::SetFirst(&mut (*route).path, _new_first)
        });
    } else {
        return (unsafe {
            let _new_second: () = (unsafe { Pair::SetFirst(&mut (*route).path, -10_()) });
            Pair::SetSecond(&mut (*route).path, _new_second)
        });
    }
    panic!("ub: non-void function does not return a value")
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut route1: Route = Route {
        path: Pair {
            first: unimplemented!(),
            second: 1_(),
        },
        cost: 5_(),
    };
    let mut route2: Route = Route {
        path: Pair {
            first: 1_(),
            second: unimplemented!(),
        },
        cost: 10_(),
    };
    let mut old_cost: () =
        (unsafe { Route::SetCost(&mut route1, (unsafe { Route::SetCost(&mut route2, 15_()) })) });
    unimplemented!();
    let mut c1: Counter = Counter {
        v: 3_(),
        calls: unimplemented!(),
    };
    let c2: Counter = Counter {
        v: 3_(),
        calls: unimplemented!(),
    };
    let mut pc: *const Counter = (&mut c1 as *mut Counter).cast_const();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut t: Tally = Tally {
        calls: unimplemented!(),
    };
    let ct: Tally = Tally {
        calls: unimplemented!(),
    };
    let rt: *const Tally = &t;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
