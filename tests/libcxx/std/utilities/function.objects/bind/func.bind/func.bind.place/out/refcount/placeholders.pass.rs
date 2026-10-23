extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_0(t: Ptr<std_placeholders___ph_1_>) {
    {
        let x: Value<std_placeholders___ph_1_> =
            Rc::new(RefCell::new(<std_placeholders___ph_1_>::default()));
        &(*x.borrow_mut());
    }
    {
        let x: Value<std_placeholders___ph_1_> =
            Rc::new(RefCell::new((*t.upgrade().deref()).clone()));
        &(*x.borrow_mut());
        const _: () = assert!(true, "std::is_nothrow_copy_constructible<T>::value");;
        const _: () = assert!(true, "std::is_nothrow_move_constructible<T>::value");;
    }
    {
        let x: Value<std_placeholders___ph_1_> =
            Rc::new(RefCell::new(<std_placeholders___ph_1_>::default()));
        (*x.borrow_mut()) = { (*t.upgrade().deref()).clone() };
        const _: () = assert!(true, "std::is_nothrow_copy_assignable<T>::value");;
        const _: () = assert!(true, "std::is_nothrow_move_assignable<T>::value");;
    }
}
pub fn test_1(t: Ptr<std_placeholders___ph_2_>) {
    {
        let x: Value<std_placeholders___ph_2_> =
            Rc::new(RefCell::new(<std_placeholders___ph_2_>::default()));
        &(*x.borrow_mut());
    }
    {
        let x: Value<std_placeholders___ph_2_> =
            Rc::new(RefCell::new((*t.upgrade().deref()).clone()));
        &(*x.borrow_mut());
        const _: () = assert!(true, "std::is_nothrow_copy_constructible<T>::value");;
        const _: () = assert!(true, "std::is_nothrow_move_constructible<T>::value");;
    }
    {
        let x: Value<std_placeholders___ph_2_> =
            Rc::new(RefCell::new(<std_placeholders___ph_2_>::default()));
        (*x.borrow_mut()) = { (*t.upgrade().deref()).clone() };
        const _: () = assert!(true, "std::is_nothrow_copy_assignable<T>::value");;
        const _: () = assert!(true, "std::is_nothrow_move_assignable<T>::value");;
    }
}
pub fn test_2(t: Ptr<std_placeholders___ph_3_>) {
    {
        let x: Value<std_placeholders___ph_3_> =
            Rc::new(RefCell::new(<std_placeholders___ph_3_>::default()));
        &(*x.borrow_mut());
    }
    {
        let x: Value<std_placeholders___ph_3_> =
            Rc::new(RefCell::new((*t.upgrade().deref()).clone()));
        &(*x.borrow_mut());
        const _: () = assert!(true, "std::is_nothrow_copy_constructible<T>::value");;
        const _: () = assert!(true, "std::is_nothrow_move_constructible<T>::value");;
    }
    {
        let x: Value<std_placeholders___ph_3_> =
            Rc::new(RefCell::new(<std_placeholders___ph_3_>::default()));
        (*x.borrow_mut()) = { (*t.upgrade().deref()).clone() };
        const _: () = assert!(true, "std::is_nothrow_copy_assignable<T>::value");;
        const _: () = assert!(true, "std::is_nothrow_move_assignable<T>::value");;
    }
}
pub fn test_3(t: Ptr<std_placeholders___ph_4_>) {
    {
        let x: Value<std_placeholders___ph_4_> =
            Rc::new(RefCell::new(<std_placeholders___ph_4_>::default()));
        &(*x.borrow_mut());
    }
    {
        let x: Value<std_placeholders___ph_4_> =
            Rc::new(RefCell::new((*t.upgrade().deref()).clone()));
        &(*x.borrow_mut());
        const _: () = assert!(true, "std::is_nothrow_copy_constructible<T>::value");;
        const _: () = assert!(true, "std::is_nothrow_move_constructible<T>::value");;
    }
    {
        let x: Value<std_placeholders___ph_4_> =
            Rc::new(RefCell::new(<std_placeholders___ph_4_>::default()));
        (*x.borrow_mut()) = { (*t.upgrade().deref()).clone() };
        const _: () = assert!(true, "std::is_nothrow_copy_assignable<T>::value");;
        const _: () = assert!(true, "std::is_nothrow_move_assignable<T>::value");;
    }
}
pub fn test_4(t: Ptr<std_placeholders___ph_5_>) {
    {
        let x: Value<std_placeholders___ph_5_> =
            Rc::new(RefCell::new(<std_placeholders___ph_5_>::default()));
        &(*x.borrow_mut());
    }
    {
        let x: Value<std_placeholders___ph_5_> =
            Rc::new(RefCell::new((*t.upgrade().deref()).clone()));
        &(*x.borrow_mut());
        const _: () = assert!(true, "std::is_nothrow_copy_constructible<T>::value");;
        const _: () = assert!(true, "std::is_nothrow_move_constructible<T>::value");;
    }
    {
        let x: Value<std_placeholders___ph_5_> =
            Rc::new(RefCell::new(<std_placeholders___ph_5_>::default()));
        (*x.borrow_mut()) = { (*t.upgrade().deref()).clone() };
        const _: () = assert!(true, "std::is_nothrow_copy_assignable<T>::value");;
        const _: () = assert!(true, "std::is_nothrow_move_assignable<T>::value");;
    }
}
pub fn test_5(t: Ptr<std_placeholders___ph_6_>) {
    {
        let x: Value<std_placeholders___ph_6_> =
            Rc::new(RefCell::new(<std_placeholders___ph_6_>::default()));
        &(*x.borrow_mut());
    }
    {
        let x: Value<std_placeholders___ph_6_> =
            Rc::new(RefCell::new((*t.upgrade().deref()).clone()));
        &(*x.borrow_mut());
        const _: () = assert!(true, "std::is_nothrow_copy_constructible<T>::value");;
        const _: () = assert!(true, "std::is_nothrow_move_constructible<T>::value");;
    }
    {
        let x: Value<std_placeholders___ph_6_> =
            Rc::new(RefCell::new(<std_placeholders___ph_6_>::default()));
        (*x.borrow_mut()) = { (*t.upgrade().deref()).clone() };
        const _: () = assert!(true, "std::is_nothrow_copy_assignable<T>::value");;
        const _: () = assert!(true, "std::is_nothrow_move_assignable<T>::value");;
    }
}
pub fn test_6(t: Ptr<std_placeholders___ph_7_>) {
    {
        let x: Value<std_placeholders___ph_7_> =
            Rc::new(RefCell::new(<std_placeholders___ph_7_>::default()));
        &(*x.borrow_mut());
    }
    {
        let x: Value<std_placeholders___ph_7_> =
            Rc::new(RefCell::new((*t.upgrade().deref()).clone()));
        &(*x.borrow_mut());
        const _: () = assert!(true, "std::is_nothrow_copy_constructible<T>::value");;
        const _: () = assert!(true, "std::is_nothrow_move_constructible<T>::value");;
    }
    {
        let x: Value<std_placeholders___ph_7_> =
            Rc::new(RefCell::new(<std_placeholders___ph_7_>::default()));
        (*x.borrow_mut()) = { (*t.upgrade().deref()).clone() };
        const _: () = assert!(true, "std::is_nothrow_copy_assignable<T>::value");;
        const _: () = assert!(true, "std::is_nothrow_move_assignable<T>::value");;
    }
}
pub fn test_7(t: Ptr<std_placeholders___ph_8_>) {
    {
        let x: Value<std_placeholders___ph_8_> =
            Rc::new(RefCell::new(<std_placeholders___ph_8_>::default()));
        &(*x.borrow_mut());
    }
    {
        let x: Value<std_placeholders___ph_8_> =
            Rc::new(RefCell::new((*t.upgrade().deref()).clone()));
        &(*x.borrow_mut());
        const _: () = assert!(true, "std::is_nothrow_copy_constructible<T>::value");;
        const _: () = assert!(true, "std::is_nothrow_move_constructible<T>::value");;
    }
    {
        let x: Value<std_placeholders___ph_8_> =
            Rc::new(RefCell::new(<std_placeholders___ph_8_>::default()));
        (*x.borrow_mut()) = { (*t.upgrade().deref()).clone() };
        const _: () = assert!(true, "std::is_nothrow_copy_assignable<T>::value");;
        const _: () = assert!(true, "std::is_nothrow_move_assignable<T>::value");;
    }
}
pub fn test_8(t: Ptr<std_placeholders___ph_9_>) {
    {
        let x: Value<std_placeholders___ph_9_> =
            Rc::new(RefCell::new(<std_placeholders___ph_9_>::default()));
        &(*x.borrow_mut());
    }
    {
        let x: Value<std_placeholders___ph_9_> =
            Rc::new(RefCell::new((*t.upgrade().deref()).clone()));
        &(*x.borrow_mut());
        const _: () = assert!(true, "std::is_nothrow_copy_constructible<T>::value");;
        const _: () = assert!(true, "std::is_nothrow_move_constructible<T>::value");;
    }
    {
        let x: Value<std_placeholders___ph_9_> =
            Rc::new(RefCell::new(<std_placeholders___ph_9_>::default()));
        (*x.borrow_mut()) = { (*t.upgrade().deref()).clone() };
        const _: () = assert!(true, "std::is_nothrow_copy_assignable<T>::value");;
        const _: () = assert!(true, "std::is_nothrow_move_assignable<T>::value");;
    }
}
pub fn test_9(t: Ptr<std_placeholders___ph_10_>) {
    {
        let x: Value<std_placeholders___ph_10_> =
            Rc::new(RefCell::new(<std_placeholders___ph_10_>::default()));
        &(*x.borrow_mut());
    }
    {
        let x: Value<std_placeholders___ph_10_> =
            Rc::new(RefCell::new((*t.upgrade().deref()).clone()));
        &(*x.borrow_mut());
        const _: () = assert!(true, "std::is_nothrow_copy_constructible<T>::value");;
        const _: () = assert!(true, "std::is_nothrow_move_constructible<T>::value");;
    }
    {
        let x: Value<std_placeholders___ph_10_> =
            Rc::new(RefCell::new(<std_placeholders___ph_10_>::default()));
        (*x.borrow_mut()) = { (*t.upgrade().deref()).clone() };
        const _: () = assert!(true, "std::is_nothrow_copy_assignable<T>::value");;
        const _: () = assert!(true, "std::is_nothrow_move_assignable<T>::value");;
    }
}
pub fn test_all_10() -> () {
    ({ test_0(_1_11.with(|v| v.as_pointer())) });
    ({ test_1(_2_12.with(|v| v.as_pointer())) });
    ({ test_2(_3_13.with(|v| v.as_pointer())) });
    ({ test_3(_4_14.with(|v| v.as_pointer())) });
    ({ test_4(_5_15.with(|v| v.as_pointer())) });
    ({ test_5(_6_16.with(|v| v.as_pointer())) });
    ({ test_6(_7_17.with(|v| v.as_pointer())) });
    ({ test_7(_8_18.with(|v| v.as_pointer())) });
    ({ test_8(_9_19.with(|v| v.as_pointer())) });
    ({ test_9(_10_20.with(|v| v.as_pointer())) });
    return true;
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
    ({ test_all_10() });
    const _: () = assert!(true, "test_all()");;
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
