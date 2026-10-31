extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static dummy_0: Value<i32> = Rc::new(RefCell::new(42));
);
pub fn return_value_1(mut _a0: i32) -> i32 {
    return dummy_0.with(|rc| *rc.borrow());
}
pub fn return_lvalue_2(mut _a0: i32) -> Ptr<i32> {
    return dummy_0.with(|v| v.as_pointer());
}
pub fn return_const_lvalue_3(mut _a0: i32) -> Ptr<i32> {
    return dummy_0.with(|v| v.as_pointer());
}
pub fn return_rvalue_4(mut _a0: i32) -> Ptr<i32> {
    return dummy_0.with(|v| v.as_pointer());
}
pub fn return_const_rvalue_5(mut _a0: i32) -> Ptr<i32> {
    return dummy_0.with(|v| v.as_pointer());
}
pub fn CheckCall_6() -> bool {
    return unimplemented!();
}
pub fn CheckCall_7() -> bool {
    return unimplemented!();
}
pub fn CheckCall_8() -> bool {
    return unimplemented!();
}
pub fn CheckCall_9() -> bool {
    return unimplemented!();
}
pub fn CheckCall_10() -> bool {
    return unimplemented!();
}
pub fn CheckCall_11() -> bool {
    return unimplemented!();
}
pub fn CheckCall_12() -> bool {
    return unimplemented!();
}
pub fn CheckCall_13() -> bool {
    return unimplemented!();
}
pub fn CheckCall_14() -> bool {
    return unimplemented!();
}
pub fn CheckCall_15() -> bool {
    return unimplemented!();
}
pub fn CheckCall_16() -> bool {
    return unimplemented!();
}
pub fn CheckCall_17() -> bool {
    return unimplemented!();
}
pub fn CheckCall_18() -> bool {
    return unimplemented!();
}
pub fn CheckCall_19() -> bool {
    return unimplemented!();
}
pub fn CheckCall_20() -> bool {
    return unimplemented!();
}
pub fn CheckCall_21() -> bool {
    return unimplemented!();
}
pub fn CheckCall_22() -> bool {
    return unimplemented!();
}
pub fn CheckCall_23() -> bool {
    return unimplemented!();
}
pub fn CheckCall_24() -> bool {
    return unimplemented!();
}
pub fn CheckCall_25() -> bool {
    return unimplemented!();
}
pub fn CheckCall_26() -> bool {
    return unimplemented!();
}
pub fn CheckCall_27() -> bool {
    return unimplemented!();
}
pub fn CheckCall_28() -> bool {
    return unimplemented!();
}
pub fn CheckCall_29() -> bool {
    return unimplemented!();
}
pub fn CheckCall_30() -> bool {
    return unimplemented!();
}
pub fn CheckCall_31() -> bool {
    return unimplemented!();
}
pub fn CheckCall_32() -> bool {
    return unimplemented!();
}
pub fn CheckCall_33() -> bool {
    return unimplemented!();
}
pub fn CheckCall_34() -> bool {
    return unimplemented!();
}
pub fn CheckCall_35() -> bool {
    return unimplemented!();
}
pub fn do_test_36(mut func: FnPtr<fn(i32) -> i32>) {
    let ret: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let ret_r: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "std::is_same<Ret, Expect>::value");;
    const _: () = assert!(true, "std::is_same<Ret2, Expect>::value");;
    const _: () = assert!(true, "std::is_same<RetR, Expect>::value");;
    const _: () = assert!(true, "std::is_same<RetR2, Expect>::value");;
    let exp: Value<i32> = Rc::new(RefCell::new(
        ({
            let ___args: Value<i32> = Rc::new(RefCell::new(100));
            (*ret.borrow_mut())(___args.as_pointer())
        })
        .clone(),
    ));
    let exp2: Value<i32> = Rc::new(RefCell::new(
        ({
            let ___args_0: Value<i32> = Rc::new(RefCell::new(101));
            let ___args_1: Value<i32> = Rc::new(RefCell::new(102));
            (*ret.borrow_mut())(___args_0.as_pointer(), ___args_1.as_pointer())
        })
        .clone(),
    ));
    let exp_r: Value<i32> = Rc::new(RefCell::new(
        ({
            let ___args: Value<i32> = Rc::new(RefCell::new(100));
            (*ret_r.borrow_mut())(___args.as_pointer())
        })
        .clone(),
    ));
    let exp_r2: Value<i32> = Rc::new(RefCell::new(
        ({
            let ___args_0: Value<i32> = Rc::new(RefCell::new(101));
            let ___args_1: Value<i32> = Rc::new(RefCell::new(102));
            (*ret_r.borrow_mut())(___args_0.as_pointer(), ___args_1.as_pointer())
        })
        .clone(),
    ));
    assert!(((*exp.borrow()) == 42));
    assert!(((*exp2.borrow()) == 42));
    assert!(((*exp_r.borrow()) == 42));
    assert!(((*exp_r2.borrow()) == 42));
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
pub fn do_test_37(mut func: FnPtr<fn(i32) -> Ptr<i32>>) {
    let ret: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let ret_r: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "std::is_same<Ret, Expect>::value");;
    const _: () = assert!(true, "std::is_same<Ret2, Expect>::value");;
    const _: () = assert!(true, "std::is_same<RetR, Expect>::value");;
    const _: () = assert!(true, "std::is_same<RetR2, Expect>::value");;
    let exp: Ptr<i32> = ({
        let ___args: Value<i32> = Rc::new(RefCell::new(100));
        (*ret.borrow_mut())(___args.as_pointer())
    })
    .clone();
    let exp2: Ptr<i32> = ({
        let ___args_0: Value<i32> = Rc::new(RefCell::new(101));
        let ___args_1: Value<i32> = Rc::new(RefCell::new(102));
        (*ret.borrow_mut())(___args_0.as_pointer(), ___args_1.as_pointer())
    })
    .clone();
    let exp_r: Ptr<i32> = ({
        let ___args: Value<i32> = Rc::new(RefCell::new(100));
        (*ret_r.borrow_mut())(___args.as_pointer())
    })
    .clone();
    let exp_r2: Ptr<i32> = ({
        let ___args_0: Value<i32> = Rc::new(RefCell::new(101));
        let ___args_1: Value<i32> = Rc::new(RefCell::new(102));
        (*ret_r.borrow_mut())(___args_0.as_pointer(), ___args_1.as_pointer())
    })
    .clone();
    assert!(((exp.read()) == 42));
    assert!(((exp2.read()) == 42));
    assert!(((exp_r.read()) == 42));
    assert!(((exp_r2.read()) == 42));
    {
        assert!(({ (exp).clone() } == { (dummy_0.with(|v| v.as_pointer())) }));
        assert!(({ (exp2).clone() } == { (dummy_0.with(|v| v.as_pointer())) }));
        assert!(({ (exp_r).clone() } == { (dummy_0.with(|v| v.as_pointer())) }));
        assert!(({ (exp_r2).clone() } == { (dummy_0.with(|v| v.as_pointer())) }));
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
pub fn do_test_38(mut func: FnPtr<fn(i32) -> Ptr<i32>>) {
    let ret: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let ret_r: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "std::is_same<Ret, Expect>::value");;
    const _: () = assert!(true, "std::is_same<Ret2, Expect>::value");;
    const _: () = assert!(true, "std::is_same<RetR, Expect>::value");;
    const _: () = assert!(true, "std::is_same<RetR2, Expect>::value");;
    let exp: Ptr<i32> = ({
        let ___args: Value<i32> = Rc::new(RefCell::new(100));
        (*ret.borrow_mut())(___args.as_pointer())
    })
    .clone();
    let exp2: Ptr<i32> = ({
        let ___args_0: Value<i32> = Rc::new(RefCell::new(101));
        let ___args_1: Value<i32> = Rc::new(RefCell::new(102));
        (*ret.borrow_mut())(___args_0.as_pointer(), ___args_1.as_pointer())
    })
    .clone();
    let exp_r: Ptr<i32> = ({
        let ___args: Value<i32> = Rc::new(RefCell::new(100));
        (*ret_r.borrow_mut())(___args.as_pointer())
    })
    .clone();
    let exp_r2: Ptr<i32> = ({
        let ___args_0: Value<i32> = Rc::new(RefCell::new(101));
        let ___args_1: Value<i32> = Rc::new(RefCell::new(102));
        (*ret_r.borrow_mut())(___args_0.as_pointer(), ___args_1.as_pointer())
    })
    .clone();
    assert!(((exp.read()) == 42));
    assert!(((exp2.read()) == 42));
    assert!(((exp_r.read()) == 42));
    assert!(((exp_r2.read()) == 42));
    {
        assert!(({ (exp).clone() } == { (dummy_0.with(|v| v.as_pointer())) }));
        assert!(({ (exp2).clone() } == { (dummy_0.with(|v| v.as_pointer())) }));
        assert!(({ (exp_r).clone() } == { (dummy_0.with(|v| v.as_pointer())) }));
        assert!(({ (exp_r2).clone() } == { (dummy_0.with(|v| v.as_pointer())) }));
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
pub fn do_test_39(mut func: FnPtr<fn(i32) -> Ptr<i32>>) {
    let ret: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let ret_r: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "std::is_same<Ret, Expect>::value");;
    const _: () = assert!(true, "std::is_same<Ret2, Expect>::value");;
    const _: () = assert!(true, "std::is_same<RetR, Expect>::value");;
    const _: () = assert!(true, "std::is_same<RetR2, Expect>::value");;
    let exp: Ptr<i32> = ({
        let ___args: Value<i32> = Rc::new(RefCell::new(100));
        (*ret.borrow_mut())(___args.as_pointer())
    })
    .clone();
    let exp2: Ptr<i32> = ({
        let ___args_0: Value<i32> = Rc::new(RefCell::new(101));
        let ___args_1: Value<i32> = Rc::new(RefCell::new(102));
        (*ret.borrow_mut())(___args_0.as_pointer(), ___args_1.as_pointer())
    })
    .clone();
    let exp_r: Ptr<i32> = ({
        let ___args: Value<i32> = Rc::new(RefCell::new(100));
        (*ret_r.borrow_mut())(___args.as_pointer())
    })
    .clone();
    let exp_r2: Ptr<i32> = ({
        let ___args_0: Value<i32> = Rc::new(RefCell::new(101));
        let ___args_1: Value<i32> = Rc::new(RefCell::new(102));
        (*ret_r.borrow_mut())(___args_0.as_pointer(), ___args_1.as_pointer())
    })
    .clone();
    assert!(((exp.read()) == 42));
    assert!(((exp2.read()) == 42));
    assert!(((exp_r.read()) == 42));
    assert!(((exp_r2.read()) == 42));
    {
        assert!(({ (exp).clone() } == { (dummy_0.with(|v| v.as_pointer())) }));
        assert!(({ (exp2).clone() } == { (dummy_0.with(|v| v.as_pointer())) }));
        assert!(({ (exp_r).clone() } == { (dummy_0.with(|v| v.as_pointer())) }));
        assert!(({ (exp_r2).clone() } == { (dummy_0.with(|v| v.as_pointer())) }));
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
pub fn do_test_40(mut func: FnPtr<fn(i32) -> Ptr<i32>>) {
    let ret: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let ret_r: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "std::is_same<Ret, Expect>::value");;
    const _: () = assert!(true, "std::is_same<Ret2, Expect>::value");;
    const _: () = assert!(true, "std::is_same<RetR, Expect>::value");;
    const _: () = assert!(true, "std::is_same<RetR2, Expect>::value");;
    let exp: Ptr<i32> = ({
        let ___args: Value<i32> = Rc::new(RefCell::new(100));
        (*ret.borrow_mut())(___args.as_pointer())
    })
    .clone();
    let exp2: Ptr<i32> = ({
        let ___args_0: Value<i32> = Rc::new(RefCell::new(101));
        let ___args_1: Value<i32> = Rc::new(RefCell::new(102));
        (*ret.borrow_mut())(___args_0.as_pointer(), ___args_1.as_pointer())
    })
    .clone();
    let exp_r: Ptr<i32> = ({
        let ___args: Value<i32> = Rc::new(RefCell::new(100));
        (*ret_r.borrow_mut())(___args.as_pointer())
    })
    .clone();
    let exp_r2: Ptr<i32> = ({
        let ___args_0: Value<i32> = Rc::new(RefCell::new(101));
        let ___args_1: Value<i32> = Rc::new(RefCell::new(102));
        (*ret_r.borrow_mut())(___args_0.as_pointer(), ___args_1.as_pointer())
    })
    .clone();
    assert!(((exp.read()) == 42));
    assert!(((exp2.read()) == 42));
    assert!(((exp_r.read()) == 42));
    assert!(((exp_r2.read()) == 42));
    {
        assert!(({ (exp).clone() } == { (dummy_0.with(|v| v.as_pointer())) }));
        assert!(({ (exp2).clone() } == { (dummy_0.with(|v| v.as_pointer())) }));
        assert!(({ (exp_r).clone() } == { (dummy_0.with(|v| v.as_pointer())) }));
        assert!(({ (exp_r2).clone() } == { (dummy_0.with(|v| v.as_pointer())) }));
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
pub fn do_test_r_41(mut func: FnPtr<fn(i32) -> i32>) {
    let ret: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "std::is_same<Ret, Expect>::value");;
    const _: () = assert!(true, "std::is_same<Ret2, Expect>::value");;
    let mut exp: i64 = ({
        let ___args: Value<i32> = Rc::new(RefCell::new(100));
        (*ret.borrow_mut())(___args.as_pointer())
    })
    .clone();
    let mut exp2: i64 = ({
        let ___args_0: Value<i32> = Rc::new(RefCell::new(101));
        let ___args_1: Value<i32> = Rc::new(RefCell::new(102));
        (*ret.borrow_mut())(___args_0.as_pointer(), ___args_1.as_pointer())
    })
    .clone();
    assert!((exp == 42_i64));
    assert!((exp2 == 42_i64));
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "CheckCall<Bind, int>()");;
        const _: () = assert!(true, "CheckCall<Bind, int, int>()");;
    }
}
pub fn do_test_r_42(mut func: FnPtr<fn(i32) -> Ptr<i32>>) {
    let ret: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "std::is_same<Ret, Expect>::value");;
    const _: () = assert!(true, "std::is_same<Ret2, Expect>::value");;
    let mut exp: i64 = ({
        let ___args: Value<i32> = Rc::new(RefCell::new(100));
        (*ret.borrow_mut())(___args.as_pointer())
    })
    .clone();
    let mut exp2: i64 = ({
        let ___args_0: Value<i32> = Rc::new(RefCell::new(101));
        let ___args_1: Value<i32> = Rc::new(RefCell::new(102));
        (*ret.borrow_mut())(___args_0.as_pointer(), ___args_1.as_pointer())
    })
    .clone();
    assert!((exp == 42_i64));
    assert!((exp2 == 42_i64));
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "CheckCall<Bind, int>()");;
        const _: () = assert!(true, "CheckCall<Bind, int, int>()");;
    }
}
pub fn do_test_r_43(mut func: FnPtr<fn(i32) -> Ptr<i32>>) {
    let ret: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "std::is_same<Ret, Expect>::value");;
    const _: () = assert!(true, "std::is_same<Ret2, Expect>::value");;
    let mut exp: i64 = ({
        let ___args: Value<i32> = Rc::new(RefCell::new(100));
        (*ret.borrow_mut())(___args.as_pointer())
    })
    .clone();
    let mut exp2: i64 = ({
        let ___args_0: Value<i32> = Rc::new(RefCell::new(101));
        let ___args_1: Value<i32> = Rc::new(RefCell::new(102));
        (*ret.borrow_mut())(___args_0.as_pointer(), ___args_1.as_pointer())
    })
    .clone();
    assert!((exp == 42_i64));
    assert!((exp2 == 42_i64));
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "CheckCall<Bind, int>()");;
        const _: () = assert!(true, "CheckCall<Bind, int, int>()");;
    }
}
pub fn do_test_r_44(mut func: FnPtr<fn(i32) -> Ptr<i32>>) {
    let ret: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "std::is_same<Ret, Expect>::value");;
    const _: () = assert!(true, "std::is_same<Ret2, Expect>::value");;
    let mut exp: i64 = ({
        let ___args: Value<i32> = Rc::new(RefCell::new(100));
        (*ret.borrow_mut())(___args.as_pointer())
    })
    .clone();
    let mut exp2: i64 = ({
        let ___args_0: Value<i32> = Rc::new(RefCell::new(101));
        let ___args_1: Value<i32> = Rc::new(RefCell::new(102));
        (*ret.borrow_mut())(___args_0.as_pointer(), ___args_1.as_pointer())
    })
    .clone();
    assert!((exp == 42_i64));
    assert!((exp2 == 42_i64));
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "CheckCall<Bind, int>()");;
        const _: () = assert!(true, "CheckCall<Bind, int, int>()");;
    }
}
pub fn do_test_r_45(mut func: FnPtr<fn(i32) -> Ptr<i32>>) {
    let ret: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "std::is_same<Ret, Expect>::value");;
    const _: () = assert!(true, "std::is_same<Ret2, Expect>::value");;
    let mut exp: i64 = ({
        let ___args: Value<i32> = Rc::new(RefCell::new(100));
        (*ret.borrow_mut())(___args.as_pointer())
    })
    .clone();
    let mut exp2: i64 = ({
        let ___args_0: Value<i32> = Rc::new(RefCell::new(101));
        let ___args_1: Value<i32> = Rc::new(RefCell::new(102));
        (*ret.borrow_mut())(___args_0.as_pointer(), ___args_1.as_pointer())
    })
    .clone();
    assert!((exp == 42_i64));
    assert!((exp2 == 42_i64));
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> i32 {
    ({ do_test_36(FnPtr::<fn(i32) -> i32>::new(return_value_1)) });
    ({ do_test_37(FnPtr::<fn(i32) -> Ptr<i32>>::new(return_lvalue_2)) });
    ({ do_test_38(FnPtr::<fn(i32) -> Ptr<i32>>::new(return_const_lvalue_3)) });
    ({ do_test_39(FnPtr::<fn(i32) -> Ptr<i32>>::new(return_rvalue_4)) });
    ({ do_test_40(FnPtr::<fn(i32) -> Ptr<i32>>::new(return_const_rvalue_5)) });
    ({ do_test_r_41(FnPtr::<fn(i32) -> i32>::new(return_value_1)) });
    ({ do_test_r_42(FnPtr::<fn(i32) -> Ptr<i32>>::new(return_lvalue_2)) });
    ({ do_test_r_43(FnPtr::<fn(i32) -> Ptr<i32>>::new(return_const_lvalue_3)) });
    ({ do_test_r_44(FnPtr::<fn(i32) -> Ptr<i32>>::new(return_rvalue_4)) });
    ({ do_test_r_45(FnPtr::<fn(i32) -> Ptr<i32>>::new(return_const_rvalue_5)) });
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = dummy_0.with(|_| ());
}
