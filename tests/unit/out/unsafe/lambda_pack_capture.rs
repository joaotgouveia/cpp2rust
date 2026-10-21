extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn sum_0() -> () {
    return unimplemented!();
}
pub unsafe fn sum_1(mut t: (), mut u_1: (), mut u_2: ()) -> () {
    return ((t) + (unsafe { sum_2(u_1, u_2) }));
}
pub unsafe fn sum_2(mut t: (), mut u: ()) -> () {
    return ((t) + (unsafe { sum_3(u) }));
}
pub unsafe fn sum_3(mut t: ()) -> () {
    return ((t) + (unsafe { sum_0() }));
}
pub unsafe fn by_value_4() -> () {
    let mut f: FnPtr<fn() -> ()> = FnPtr::<fn() -> ()>::new(|| -> () {
        unsafe {
            return (unsafe { sum_0() });
        }
    });
    unimplemented!();
    return (unsafe { f.call() });
}
pub unsafe fn by_value_5(mut t_0: (), mut t_1: (), mut t_2: ()) -> () {
    let mut f: FnPtr<fn() -> ()> = lambda_unsafe!(
        {
            let t_0: () = t_0;
            let t_1: () = t_1;
            let t_2: () = t_2;
        },
        || -> () {
            return (unsafe { sum_1(t_0, t_1, t_2) });
        }
    );
    {
        t_0 = unimplemented!();
        {
            t_1 = unimplemented!();
            t_2 = unimplemented!()
        }
    };
    return (unsafe { f.call() });
}
pub unsafe fn by_ref_6(mut t_0: (), mut t_1: (), mut t_2: ()) -> () {
    let mut f: FnPtr<fn()> = lambda_unsafe!(
        {
            let t_0: *mut () = &mut t_0;
            let t_1: *mut () = &mut t_1;
            let t_2: *mut () = &mut t_2;
        },
        || {
            {
                (*t_0) *= 2_();
                {
                    (*t_1) *= 2_();
                    (*t_2) *= 2_()
                }
            };
        }
    );
    (unsafe { f.call() });
    return (unsafe { sum_1(t_0, t_1, t_2) });
}
pub unsafe fn init_pack_7(mut t_0: (), mut t_1: (), mut t_2: ()) -> () {
    let mut f: FnPtr<fn() -> ()> = lambda_unsafe!(
        {
            let xs_0: () = ((t_0) + (1_()));
            let xs_1: () = ((t_1) + (1_()));
            let xs_2: () = ((t_2) + (1_()));
        },
        || -> () {
            return (unsafe { sum_1(xs_0, xs_1, xs_2) });
        }
    );
    return (unsafe { f.call() });
}
pub unsafe fn implicit_8(mut t_0: (), mut t_1: ()) -> () {
    return (unsafe {
        lambda_unsafe!(
            {
                let t_0: () = t_0;
                let t_1: () = t_1;
            },
            || -> () {
                return (unsafe { sum_2(t_0, t_1) });
            }
        )
        .call()
    });
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
