extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn apply_int_0(mut fn_: FnPtr<fn(()) -> ()>, mut x: ()) -> () {
    return (unsafe { fn_.call(x) });
}
pub unsafe fn apply_int_1(mut fn_: FnPtr<Generic>, mut x: ()) -> () {
    return (unsafe { fn_.spec::<fn(()) -> ()>(0).call(x) });
}
pub unsafe fn apply_int_2(mut fn_: FnPtr<Generic>, mut x: ()) -> () {
    return (unsafe { fn_.spec::<fn(()) -> ()>(0).call(x) });
}
pub unsafe fn apply_int_3(mut fn_: FnPtr<Generic>, mut x: ()) -> () {
    return (unsafe { fn_.spec::<fn(()) -> ()>(0).call(x) });
}
pub unsafe fn apply_double_4(mut fn_: FnPtr<Generic>, mut x: ()) -> () {
    return (unsafe { fn_.spec::<fn(()) -> ()>(1).call(x) });
}
pub unsafe fn apply_double_5(mut fn_: FnPtr<Generic>, mut x: ()) -> () {
    return (unsafe { fn_.spec::<fn(()) -> ()>(1).call(x) });
}
pub unsafe fn apply_twice_6(mut fn_: FnPtr<fn(()) -> ()>, mut x: ()) -> () {
    return (unsafe {
        let _x: () = (unsafe { fn_.call(x) });
        fn_.call(_x)
    });
}
pub unsafe fn none_match_7(mut pred: FnPtr<fn(()) -> ()>, mut a: (), mut b: ()) -> () {
    let mut neg_pred: FnPtr<fn(()) -> ()> = lambda_unsafe!(
        {
            let pred: *mut FnPtr<fn(()) -> ()> = &mut pred;
        },
        |mut x: ()| -> () {
            return !(unsafe { (*pred).call(x) });
        }
    );
    return (unsafe { neg_pred.call(a) }) && (unsafe { neg_pred.call(b) });
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut factor: () = 3_();
    let mut scale: FnPtr<fn(()) -> ()> = lambda_unsafe!(
        {
            let factor: () = factor;
        },
        |mut x: ()| -> () {
            return ((x) * (factor));
        }
    );
    unimplemented!();
    unimplemented!();
    let mut generic_scale: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let factor: () = factor;
        },
        |mut x: ()| -> () {
            return ((x) * (factor));
        },
        |mut x: ()| -> () {
            return ((x) * (factor));
        }
    );
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut offset: FnPtr<Generic> = lambda_unsafe!(
        Generic,
        {
            let factor: () = factor;
        },
        |mut x: ()| -> () {
            return ((x) + (factor));
        },
        |mut x: ()| -> () {
            return ((x) + (factor));
        }
    );
    unimplemented!();
    unimplemented!();
    let mut is_even: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut x: ()| -> () {
        unsafe {
            return (((x) % (2_())) == (unimplemented!()));
        }
    });
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
