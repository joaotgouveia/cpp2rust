extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static dummy_0: Value<()> = Rc::new(RefCell::new(42_()));
);
pub fn return_value_1(mut _a0: ()) -> () {
    return dummy_0.with(|rc| *rc.borrow());
}
pub fn return_lvalue_2(mut _a0: ()) -> Ptr<()> {
    return dummy_0.with(|v| v.as_pointer());
}
pub fn return_const_lvalue_3(mut _a0: ()) -> Ptr<()> {
    return dummy_0.with(|v| v.as_pointer());
}
pub fn return_rvalue_4(mut _a0: ()) -> Ptr<()> {
    return dummy_0.with(|v| v.as_pointer());
}
pub fn return_const_rvalue_5(mut _a0: ()) -> Ptr<()> {
    return dummy_0.with(|v| v.as_pointer());
}
pub fn CheckCall_6() -> () {
    return unimplemented!();
}
pub fn CheckCall_7() -> () {
    return unimplemented!();
}
pub fn CheckCall_8() -> () {
    return unimplemented!();
}
pub fn CheckCall_9() -> () {
    return unimplemented!();
}
pub fn CheckCall_10() -> () {
    return unimplemented!();
}
pub fn CheckCall_11() -> () {
    return unimplemented!();
}
pub fn CheckCall_12() -> () {
    return unimplemented!();
}
pub fn CheckCall_13() -> () {
    return unimplemented!();
}
pub fn CheckCall_14() -> () {
    return unimplemented!();
}
pub fn CheckCall_15() -> () {
    return unimplemented!();
}
pub fn CheckCall_16() -> () {
    return unimplemented!();
}
pub fn CheckCall_17() -> () {
    return unimplemented!();
}
pub fn CheckCall_18() -> () {
    return unimplemented!();
}
pub fn CheckCall_19() -> () {
    return unimplemented!();
}
pub fn CheckCall_20() -> () {
    return unimplemented!();
}
pub fn CheckCall_21() -> () {
    return unimplemented!();
}
pub fn CheckCall_22() -> () {
    return unimplemented!();
}
pub fn CheckCall_23() -> () {
    return unimplemented!();
}
pub fn CheckCall_24() -> () {
    return unimplemented!();
}
pub fn CheckCall_25() -> () {
    return unimplemented!();
}
pub fn CheckCall_26() -> () {
    return unimplemented!();
}
pub fn CheckCall_27() -> () {
    return unimplemented!();
}
pub fn CheckCall_28() -> () {
    return unimplemented!();
}
pub fn CheckCall_29() -> () {
    return unimplemented!();
}
pub fn CheckCall_30() -> () {
    return unimplemented!();
}
pub fn CheckCall_31() -> () {
    return unimplemented!();
}
pub fn CheckCall_32() -> () {
    return unimplemented!();
}
pub fn CheckCall_33() -> () {
    return unimplemented!();
}
pub fn CheckCall_34() -> () {
    return unimplemented!();
}
pub fn CheckCall_35() -> () {
    return unimplemented!();
}
pub fn do_test_36(mut func: FnPtr<fn(()) -> ()>) {
    let ret: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let ret_r: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "std::is_same<Ret, Expect>::value");;
    const _: () = assert!(true, "std::is_same<Ret2, Expect>::value");;
    const _: () = assert!(true, "std::is_same<RetR, Expect>::value");;
    const _: () = assert!(true, "std::is_same<RetR2, Expect>::value");;
    let exp: Value<()> = Rc::new(RefCell::new(
        ({
            let ___args: Value<()> = Rc::new(RefCell::new(100_()));
            (*ret.borrow_mut())(___args.as_pointer())
        })
        .clone(),
    ));
    let exp2: Value<()> = Rc::new(RefCell::new(
        ({
            let ___args_0: Value<()> = Rc::new(RefCell::new(101_()));
            let ___args_1: Value<()> = Rc::new(RefCell::new(102_()));
            (*ret.borrow_mut())(___args_0.as_pointer(), ___args_1.as_pointer())
        })
        .clone(),
    ));
    let exp_r: Value<()> = Rc::new(RefCell::new(
        ({
            let ___args: Value<()> = Rc::new(RefCell::new(100_()));
            (*ret_r.borrow_mut())(___args.as_pointer())
        })
        .clone(),
    ));
    let exp_r2: Value<()> = Rc::new(RefCell::new(
        ({
            let ___args_0: Value<()> = Rc::new(RefCell::new(101_()));
            let ___args_1: Value<()> = Rc::new(RefCell::new(102_()));
            (*ret_r.borrow_mut())(___args_0.as_pointer(), ___args_1.as_pointer())
        })
        .clone(),
    ));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {}
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "CheckCall<Bind, int>()");;
        const _: () = assert!(true, "CheckCall<Bind, int, int>()");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "CheckCall<BindR, int>()");;
        const _: () = assert!(true, "CheckCall<BindR, int, int>()");;
    }
}
pub fn do_test_37(mut func: FnPtr<fn(()) -> Ptr<()>>) {
    let ret: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let ret_r: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "std::is_same<Ret, Expect>::value");;
    const _: () = assert!(true, "std::is_same<Ret2, Expect>::value");;
    const _: () = assert!(true, "std::is_same<RetR, Expect>::value");;
    const _: () = assert!(true, "std::is_same<RetR2, Expect>::value");;
    let exp: Ptr<()> = ({
        let ___args: Value<()> = Rc::new(RefCell::new(100_()));
        (*ret.borrow_mut())(___args.as_pointer())
    })
    .clone();
    let exp2: Ptr<()> = ({
        let ___args_0: Value<()> = Rc::new(RefCell::new(101_()));
        let ___args_1: Value<()> = Rc::new(RefCell::new(102_()));
        (*ret.borrow_mut())(___args_0.as_pointer(), ___args_1.as_pointer())
    })
    .clone();
    let exp_r: Ptr<()> = ({
        let ___args: Value<()> = Rc::new(RefCell::new(100_()));
        (*ret_r.borrow_mut())(___args.as_pointer())
    })
    .clone();
    let exp_r2: Ptr<()> = ({
        let ___args_0: Value<()> = Rc::new(RefCell::new(101_()));
        let ___args_1: Value<()> = Rc::new(RefCell::new(102_()));
        (*ret_r.borrow_mut())(___args_0.as_pointer(), ___args_1.as_pointer())
    })
    .clone();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "CheckCall<Bind, int>()");;
        const _: () = assert!(true, "CheckCall<Bind, int, int>()");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "CheckCall<BindR, int>()");;
        const _: () = assert!(true, "CheckCall<BindR, int, int>()");;
    }
}
pub fn do_test_38(mut func: FnPtr<fn(()) -> Ptr<()>>) {
    let ret: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let ret_r: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "std::is_same<Ret, Expect>::value");;
    const _: () = assert!(true, "std::is_same<Ret2, Expect>::value");;
    const _: () = assert!(true, "std::is_same<RetR, Expect>::value");;
    const _: () = assert!(true, "std::is_same<RetR2, Expect>::value");;
    let exp: Ptr<()> = ({
        let ___args: Value<()> = Rc::new(RefCell::new(100_()));
        (*ret.borrow_mut())(___args.as_pointer())
    })
    .clone();
    let exp2: Ptr<()> = ({
        let ___args_0: Value<()> = Rc::new(RefCell::new(101_()));
        let ___args_1: Value<()> = Rc::new(RefCell::new(102_()));
        (*ret.borrow_mut())(___args_0.as_pointer(), ___args_1.as_pointer())
    })
    .clone();
    let exp_r: Ptr<()> = ({
        let ___args: Value<()> = Rc::new(RefCell::new(100_()));
        (*ret_r.borrow_mut())(___args.as_pointer())
    })
    .clone();
    let exp_r2: Ptr<()> = ({
        let ___args_0: Value<()> = Rc::new(RefCell::new(101_()));
        let ___args_1: Value<()> = Rc::new(RefCell::new(102_()));
        (*ret_r.borrow_mut())(___args_0.as_pointer(), ___args_1.as_pointer())
    })
    .clone();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "CheckCall<Bind, int>()");;
        const _: () = assert!(true, "CheckCall<Bind, int, int>()");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "CheckCall<BindR, int>()");;
        const _: () = assert!(true, "CheckCall<BindR, int, int>()");;
    }
}
pub fn do_test_39(mut func: FnPtr<fn(()) -> Ptr<()>>) {
    let ret: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let ret_r: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "std::is_same<Ret, Expect>::value");;
    const _: () = assert!(true, "std::is_same<Ret2, Expect>::value");;
    const _: () = assert!(true, "std::is_same<RetR, Expect>::value");;
    const _: () = assert!(true, "std::is_same<RetR2, Expect>::value");;
    let exp: Ptr<()> = ({
        let ___args: Value<()> = Rc::new(RefCell::new(100_()));
        (*ret.borrow_mut())(___args.as_pointer())
    })
    .clone();
    let exp2: Ptr<()> = ({
        let ___args_0: Value<()> = Rc::new(RefCell::new(101_()));
        let ___args_1: Value<()> = Rc::new(RefCell::new(102_()));
        (*ret.borrow_mut())(___args_0.as_pointer(), ___args_1.as_pointer())
    })
    .clone();
    let exp_r: Ptr<()> = ({
        let ___args: Value<()> = Rc::new(RefCell::new(100_()));
        (*ret_r.borrow_mut())(___args.as_pointer())
    })
    .clone();
    let exp_r2: Ptr<()> = ({
        let ___args_0: Value<()> = Rc::new(RefCell::new(101_()));
        let ___args_1: Value<()> = Rc::new(RefCell::new(102_()));
        (*ret_r.borrow_mut())(___args_0.as_pointer(), ___args_1.as_pointer())
    })
    .clone();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "CheckCall<Bind, int>()");;
        const _: () = assert!(true, "CheckCall<Bind, int, int>()");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "CheckCall<BindR, int>()");;
        const _: () = assert!(true, "CheckCall<BindR, int, int>()");;
    }
}
pub fn do_test_40(mut func: FnPtr<fn(()) -> Ptr<()>>) {
    let ret: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let ret_r: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "std::is_same<Ret, Expect>::value");;
    const _: () = assert!(true, "std::is_same<Ret2, Expect>::value");;
    const _: () = assert!(true, "std::is_same<RetR, Expect>::value");;
    const _: () = assert!(true, "std::is_same<RetR2, Expect>::value");;
    let exp: Ptr<()> = ({
        let ___args: Value<()> = Rc::new(RefCell::new(100_()));
        (*ret.borrow_mut())(___args.as_pointer())
    })
    .clone();
    let exp2: Ptr<()> = ({
        let ___args_0: Value<()> = Rc::new(RefCell::new(101_()));
        let ___args_1: Value<()> = Rc::new(RefCell::new(102_()));
        (*ret.borrow_mut())(___args_0.as_pointer(), ___args_1.as_pointer())
    })
    .clone();
    let exp_r: Ptr<()> = ({
        let ___args: Value<()> = Rc::new(RefCell::new(100_()));
        (*ret_r.borrow_mut())(___args.as_pointer())
    })
    .clone();
    let exp_r2: Ptr<()> = ({
        let ___args_0: Value<()> = Rc::new(RefCell::new(101_()));
        let ___args_1: Value<()> = Rc::new(RefCell::new(102_()));
        (*ret_r.borrow_mut())(___args_0.as_pointer(), ___args_1.as_pointer())
    })
    .clone();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "CheckCall<Bind, int>()");;
        const _: () = assert!(true, "CheckCall<Bind, int, int>()");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "CheckCall<BindR, int>()");;
        const _: () = assert!(true, "CheckCall<BindR, int, int>()");;
    }
}
pub fn do_test_r_41(mut func: FnPtr<fn(()) -> ()>) {
    let ret: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "std::is_same<Ret, Expect>::value");;
    const _: () = assert!(true, "std::is_same<Ret2, Expect>::value");;
    let mut exp: () = ({
        let ___args: Value<()> = Rc::new(RefCell::new(100_()));
        (*ret.borrow_mut())(___args.as_pointer())
    })
    .clone();
    let mut exp2: () = ({
        let ___args_0: Value<()> = Rc::new(RefCell::new(101_()));
        let ___args_1: Value<()> = Rc::new(RefCell::new(102_()));
        (*ret.borrow_mut())(___args_0.as_pointer(), ___args_1.as_pointer())
    })
    .clone();
    unimplemented!();
    unimplemented!();
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "CheckCall<Bind, int>()");;
        const _: () = assert!(true, "CheckCall<Bind, int, int>()");;
    }
}
pub fn do_test_r_42(mut func: FnPtr<fn(()) -> Ptr<()>>) {
    let ret: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "std::is_same<Ret, Expect>::value");;
    const _: () = assert!(true, "std::is_same<Ret2, Expect>::value");;
    let mut exp: () = ({
        let ___args: Value<()> = Rc::new(RefCell::new(100_()));
        (*ret.borrow_mut())(___args.as_pointer())
    })
    .clone();
    let mut exp2: () = ({
        let ___args_0: Value<()> = Rc::new(RefCell::new(101_()));
        let ___args_1: Value<()> = Rc::new(RefCell::new(102_()));
        (*ret.borrow_mut())(___args_0.as_pointer(), ___args_1.as_pointer())
    })
    .clone();
    unimplemented!();
    unimplemented!();
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "CheckCall<Bind, int>()");;
        const _: () = assert!(true, "CheckCall<Bind, int, int>()");;
    }
}
pub fn do_test_r_43(mut func: FnPtr<fn(()) -> Ptr<()>>) {
    let ret: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "std::is_same<Ret, Expect>::value");;
    const _: () = assert!(true, "std::is_same<Ret2, Expect>::value");;
    let mut exp: () = ({
        let ___args: Value<()> = Rc::new(RefCell::new(100_()));
        (*ret.borrow_mut())(___args.as_pointer())
    })
    .clone();
    let mut exp2: () = ({
        let ___args_0: Value<()> = Rc::new(RefCell::new(101_()));
        let ___args_1: Value<()> = Rc::new(RefCell::new(102_()));
        (*ret.borrow_mut())(___args_0.as_pointer(), ___args_1.as_pointer())
    })
    .clone();
    unimplemented!();
    unimplemented!();
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "CheckCall<Bind, int>()");;
        const _: () = assert!(true, "CheckCall<Bind, int, int>()");;
    }
}
pub fn do_test_r_44(mut func: FnPtr<fn(()) -> Ptr<()>>) {
    let ret: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "std::is_same<Ret, Expect>::value");;
    const _: () = assert!(true, "std::is_same<Ret2, Expect>::value");;
    let mut exp: () = ({
        let ___args: Value<()> = Rc::new(RefCell::new(100_()));
        (*ret.borrow_mut())(___args.as_pointer())
    })
    .clone();
    let mut exp2: () = ({
        let ___args_0: Value<()> = Rc::new(RefCell::new(101_()));
        let ___args_1: Value<()> = Rc::new(RefCell::new(102_()));
        (*ret.borrow_mut())(___args_0.as_pointer(), ___args_1.as_pointer())
    })
    .clone();
    unimplemented!();
    unimplemented!();
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "CheckCall<Bind, int>()");;
        const _: () = assert!(true, "CheckCall<Bind, int, int>()");;
    }
}
pub fn do_test_r_45(mut func: FnPtr<fn(()) -> Ptr<()>>) {
    let ret: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "std::is_same<Ret, Expect>::value");;
    const _: () = assert!(true, "std::is_same<Ret2, Expect>::value");;
    let mut exp: () = ({
        let ___args: Value<()> = Rc::new(RefCell::new(100_()));
        (*ret.borrow_mut())(___args.as_pointer())
    })
    .clone();
    let mut exp2: () = ({
        let ___args_0: Value<()> = Rc::new(RefCell::new(101_()));
        let ___args_1: Value<()> = Rc::new(RefCell::new(102_()));
        (*ret.borrow_mut())(___args_0.as_pointer(), ___args_1.as_pointer())
    })
    .clone();
    unimplemented!();
    unimplemented!();
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "CheckCall<Bind, int>()");;
        const _: () = assert!(true, "CheckCall<Bind, int, int>()");;
    }
}
pub fn main() {
    let size: usize = ::std::env::args().map(|arg| arg.len() + 1).sum();
    let args: Value<Box<[i8]>> = Rc::new(RefCell::new(vec![0; size].into_boxed_slice()));
    let args_ptr: Ptr<i8> = args.as_pointer();
    let mut off = 0;
    let argv: Value<Box<[Ptr<i8>]>> = Rc::new(RefCell::new(
        ::std::env::args()
            .map(|arg| {
                let ptr = args_ptr.offset(off);
                ptr.write_c_bytes(arg.as_bytes());
                off += arg.len() + 1;
                ptr
            })
            .chain(::std::iter::once(Ptr::null()))
            .collect(),
    ));
    __cpp2rust_init_globals();
    ::std::process::exit(main_0(::std::env::args().len() as i32, argv.as_pointer()));
}
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    ({ do_test_36(FnPtr::<fn(()) -> ()>::new(return_value_1)) });
    ({ do_test_37(FnPtr::<fn(()) -> Ptr<()>>::new(return_lvalue_2)) });
    ({ do_test_38(FnPtr::<fn(()) -> Ptr<()>>::new(return_const_lvalue_3)) });
    ({ do_test_39(FnPtr::<fn(()) -> Ptr<()>>::new(return_rvalue_4)) });
    ({ do_test_40(FnPtr::<fn(()) -> Ptr<()>>::new(return_const_rvalue_5)) });
    ({ do_test_r_41(FnPtr::<fn(()) -> ()>::new(return_value_1)) });
    ({ do_test_r_42(FnPtr::<fn(()) -> Ptr<()>>::new(return_lvalue_2)) });
    ({ do_test_r_43(FnPtr::<fn(()) -> Ptr<()>>::new(return_const_lvalue_3)) });
    ({ do_test_r_44(FnPtr::<fn(()) -> Ptr<()>>::new(return_rvalue_4)) });
    ({ do_test_r_45(FnPtr::<fn(()) -> Ptr<()>>::new(return_const_rvalue_5)) });
    return 0_();
}
pub fn __cpp2rust_init_globals() {
    let _ = dummy_0.with(|_| ());
}
