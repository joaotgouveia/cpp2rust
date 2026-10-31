extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
impl A1_int_ {
    pub unsafe fn new_1(a: *mut A1_char_) -> Self {
        let mut this = Self {
            id_: (unsafe { A1_char_::id(&(*a)) }),
        };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut move_called_0)) = true;
        this
    }
}
pub static mut copy_called_1: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { false });
pub static mut move_called_0: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { false });
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, CppFromUnsafe, Default)]
#[cpp_from_move(A1_char_, new_1, ptr)]
pub struct A1_int_ {
    id_: i32,
}
impl A1_int_ {
    pub unsafe fn new_2(mut id: i32) -> Self {
        let mut this = Self { id_: id };
        this
    }
    pub unsafe fn id(&self) -> i32 {
        return self.id_;
    }
    pub unsafe fn copy_from(a: *const A1_int_) -> Self {
        let mut this = Self {
            id_: (unsafe { A1_int_::id(&(*a)) }),
        };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copy_called_1)) = true;
        this
    }
    pub unsafe fn move_from(a: *mut A1_int_) -> Self {
        let mut this = Self {
            id_: (unsafe { A1_int_::id(&(*a)) }),
        };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut move_called_0)) = true;
        this
    }
}
impl std::cmp::PartialEq for A1_int_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_4(self as *const A1_int_, other as *const A1_int_) }
    }
}
impl std::cmp::Eq for A1_int_ {}
impl Clone for A1_int_ {
    fn clone(&self) -> Self {
        unsafe { A1_int_::copy_from(self as *const A1_int_) }
    }
}
#[repr(C)]
#[derive(FnPtrArg, Default)]
pub struct A1_char_ {
    id_: i32,
}
impl A1_char_ {
    pub unsafe fn new(mut id: i32) -> Self {
        let mut this = Self { id_: id };
        this
    }
    pub unsafe fn id(&self) -> i32 {
        return self.id_;
    }
}
pub unsafe fn operator_eq_4(x: *const A1_int_, y: *const A1_int_) -> bool {
    return ((unsafe { A1_int_::id(&(*x)) }) == (unsafe { A1_int_::id(&(*y)) }));
}
pub static mut copy_called_9: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { false });
pub static mut move_called_10: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { false });
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, Default)]
pub struct A2_int_ {
    id_: i32,
}
impl A2_int_ {
    pub unsafe fn new(mut id: i32) -> Self {
        let mut this = Self { id_: id };
        this
    }
    pub unsafe fn id(&self) -> i32 {
        return self.id_;
    }
    pub unsafe fn copy_from(a: *const A2_int_) -> Self {
        let mut this = Self {
            id_: (unsafe { A2_int_::id(&(*a)) }),
        };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copy_called_9)) = true;
        this
    }
    pub unsafe fn move_from(a: *mut A2_int_) -> Self {
        let mut this = Self {
            id_: (unsafe { A2_int_::id(&(*a)) }),
        };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut move_called_10)) = true;
        this
    }
}
impl std::cmp::PartialEq for A2_int_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_12(self as *const A2_int_, other as *const A2_int_) }
    }
}
impl std::cmp::Eq for A2_int_ {}
impl Clone for A2_int_ {
    fn clone(&self) -> Self {
        unsafe { A2_int_::copy_from(self as *const A2_int_) }
    }
}
pub unsafe fn operator_eq_12(x: *const A2_int_, y: *const A2_int_) -> bool {
    return ((unsafe { A2_int_::id(&(*x)) }) == (unsafe { A2_int_::id(&(*y)) }));
}
pub static mut copy_called_13: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { false });
pub static mut move_called_14: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { false });
#[repr(C)]
#[derive(VaArg, FnPtrArg, Default)]
pub struct A3_int_ {
    id_: i32,
}
impl A3_int_ {
    pub unsafe fn new(mut id: i32) -> Self {
        let mut this = Self { id_: id };
        this
    }
    pub unsafe fn id(&self) -> i32 {
        return self.id_;
    }
    pub unsafe fn copy_from(a: *const A3_int_) -> Self {
        let mut this = Self {
            id_: (unsafe { A3_int_::id(&(*a)) }),
        };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copy_called_13)) = true;
        this
    }
}
impl std::cmp::PartialEq for A3_int_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_17(self as *const A3_int_, other as *const A3_int_) }
    }
}
impl std::cmp::Eq for A3_int_ {}
impl Clone for A3_int_ {
    fn clone(&self) -> Self {
        unsafe { A3_int_::copy_from(self as *const A3_int_) }
    }
}
pub unsafe fn operator_eq_17(x: *const A3_int_, y: *const A3_int_) -> bool {
    return ((unsafe { A3_int_::id(&(*x)) }) == (unsafe { A3_int_::id(&(*y)) }));
}
pub fn main() {
    let mut args: Vec<Vec<u8>> = std::env::args()
        .map(|arg| arg.as_bytes().to_vec())
        .collect();
    args.iter_mut().for_each(|v| v.push(0));
    let mut argv: Vec<*mut libc::c_char> = args
        .iter()
        .map(|arg| arg.as_ptr() as *mut libc::c_char)
        .collect();
    argv.push(::std::ptr::null_mut());
    unsafe {
        __cpp2rust_init_globals();
        ::std::process::exit(main_0((argv.len() - 1) as i32, argv.as_mut_ptr()) as i32)
    }
}
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    {
        let mut a3: A1_int_ = A1_int_::new_2({ 3 });
        let mut a: () = unimplemented!();
        assert!(
            (unsafe {
                let _x: *const A1_int_ = unimplemented!();
                let mut _y: A1_int_ = A1_int_::new_2({ 3 });
                operator_eq_4(_x, &mut _y)
            })
        );
        assert!(unimplemented!());
        assert!(
            (((*std::cell::LazyCell::force_mut(&mut *&raw mut copy_called_1)) as i32)
                == (true as i32))
        );
        assert!(
            (((*std::cell::LazyCell::force_mut(&mut *&raw mut move_called_0)) as i32)
                == (false as i32))
        );
    }
    (*std::cell::LazyCell::force_mut(&mut *&raw mut copy_called_1)) = false;
    {
        let mut a: () = unimplemented!();
        assert!(
            (unsafe {
                let _x: *const A1_int_ = unimplemented!();
                let mut _y: A1_int_ = A1_int_::new_2({ 3 });
                operator_eq_4(_x, &mut _y)
            })
        );
        assert!(unimplemented!());
        assert!(
            (((*std::cell::LazyCell::force_mut(&mut *&raw mut copy_called_1)) as i32)
                == (false as i32))
        );
        assert!(
            (((*std::cell::LazyCell::force_mut(&mut *&raw mut move_called_0)) as i32)
                == (true as i32))
        );
    }
    (*std::cell::LazyCell::force_mut(&mut *&raw mut move_called_0)) = false;
    {
        let mut a4: A1_int_ = A1_int_::new_2({ 4 });
        let mut a: () = unimplemented!();
        assert!(
            (((*std::cell::LazyCell::force_mut(&mut *&raw mut copy_called_1)) as i32)
                == (true as i32))
        );
        assert!(
            (((*std::cell::LazyCell::force_mut(&mut *&raw mut move_called_0)) as i32)
                == (false as i32))
        );
        assert!(
            (((*std::cell::LazyCell::force_mut(&mut *&raw mut copy_called_9)) as i32)
                == (true as i32))
        );
        assert!(
            (((*std::cell::LazyCell::force_mut(&mut *&raw mut move_called_10)) as i32)
                == (false as i32))
        );
        assert!(
            (unsafe {
                let _x: *const A1_int_ = unimplemented!();
                let mut _y: A1_int_ = A1_int_::new_2({ 4 });
                operator_eq_4(_x, &mut _y)
            })
        );
        assert!(unimplemented!());
    }
    (*std::cell::LazyCell::force_mut(&mut *&raw mut copy_called_1)) = false;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut move_called_0)) = false;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut copy_called_9)) = false;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut move_called_10)) = false;
    {
        let mut a: () = unimplemented!();
        assert!(
            (((*std::cell::LazyCell::force_mut(&mut *&raw mut copy_called_1)) as i32)
                == (false as i32))
        );
        assert!(
            (((*std::cell::LazyCell::force_mut(&mut *&raw mut move_called_0)) as i32)
                == (true as i32))
        );
        assert!(
            (((*std::cell::LazyCell::force_mut(&mut *&raw mut copy_called_9)) as i32)
                == (true as i32))
        );
        assert!(
            (((*std::cell::LazyCell::force_mut(&mut *&raw mut move_called_10)) as i32)
                == (false as i32))
        );
        assert!(
            (unsafe {
                let _x: *const A1_int_ = unimplemented!();
                let mut _y: A1_int_ = A1_int_::new_2({ 4 });
                operator_eq_4(_x, &mut _y)
            })
        );
        assert!(unimplemented!());
    }
    (*std::cell::LazyCell::force_mut(&mut *&raw mut copy_called_1)) = false;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut move_called_0)) = false;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut copy_called_9)) = false;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut move_called_10)) = false;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut move_called_0)) = false;
    {
        let mut a4: A1_int_ = A1_int_::new_2({ 4 });
        let mut a: () = unimplemented!();
        assert!(
            (((*std::cell::LazyCell::force_mut(&mut *&raw mut copy_called_1)) as i32)
                == (true as i32))
        );
        assert!(
            (((*std::cell::LazyCell::force_mut(&mut *&raw mut move_called_0)) as i32)
                == (false as i32))
        );
        assert!(
            (((*std::cell::LazyCell::force_mut(&mut *&raw mut copy_called_9)) as i32)
                == (true as i32))
        );
        assert!(
            (((*std::cell::LazyCell::force_mut(&mut *&raw mut move_called_10)) as i32)
                == (false as i32))
        );
        assert!(
            (((*std::cell::LazyCell::force_mut(&mut *&raw mut copy_called_13)) as i32)
                == (true as i32))
        );
        assert!(
            (((*std::cell::LazyCell::force_mut(&mut *&raw mut move_called_14)) as i32)
                == (false as i32))
        );
        assert!(
            (unsafe {
                let _x: *const A1_int_ = unimplemented!();
                let mut _y: A1_int_ = A1_int_::new_2({ 4 });
                operator_eq_4(_x, &mut _y)
            })
        );
        assert!((unimplemented!()));
    }
    (*std::cell::LazyCell::force_mut(&mut *&raw mut copy_called_1)) = false;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut move_called_0)) = false;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut copy_called_9)) = false;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut move_called_10)) = false;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut copy_called_13)) = false;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut move_called_14)) = false;
    {
        let mut a: () = unimplemented!();
        assert!(
            (((*std::cell::LazyCell::force_mut(&mut *&raw mut copy_called_1)) as i32)
                == (false as i32))
        );
        assert!(
            (((*std::cell::LazyCell::force_mut(&mut *&raw mut move_called_0)) as i32)
                == (true as i32))
        );
        assert!(
            (((*std::cell::LazyCell::force_mut(&mut *&raw mut copy_called_9)) as i32)
                == (true as i32))
        );
        assert!(
            (((*std::cell::LazyCell::force_mut(&mut *&raw mut move_called_10)) as i32)
                == (false as i32))
        );
        assert!(
            (((*std::cell::LazyCell::force_mut(&mut *&raw mut copy_called_13)) as i32)
                == (true as i32))
        );
        assert!(
            (((*std::cell::LazyCell::force_mut(&mut *&raw mut move_called_14)) as i32)
                == (false as i32))
        );
        assert!(
            (unsafe {
                let _x: *const A1_int_ = unimplemented!();
                let mut _y: A1_int_ = A1_int_::new_2({ 4 });
                operator_eq_4(_x, &mut _y)
            })
        );
        assert!((unimplemented!()));
    }
    {
        const _: () = assert!(true, "!std::is_convertible<A1<int>, A2<int>>::value");;
        const _: () = assert!(
            true,
            "!std::is_convertible< std::scoped_allocator_adaptor<A1<int>>, std::scoped_allocator_adaptor<A2<int>>>::value"
        );;
    }
    {
        let mut a: () = unimplemented!();
        assert!(
            (unsafe {
                let _x: *const A1_int_ = unimplemented!();
                let mut _y: A1_int_ = A1_int_::new_2({ 4 });
                operator_eq_4(_x, &mut _y)
            })
        );
        assert!(
            (unsafe {
                let _x: *const A1_int_ = unimplemented!();
                let mut _y: A1_int_ = A1_int_::new_2({ 5 });
                operator_eq_4(_x, &mut _y)
            })
        );
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const copy_called_1);
    std::cell::LazyCell::force(&*&raw const move_called_0);
    std::cell::LazyCell::force(&*&raw const copy_called_9);
    std::cell::LazyCell::force(&*&raw const move_called_10);
    std::cell::LazyCell::force(&*&raw const copy_called_13);
    std::cell::LazyCell::force(&*&raw const move_called_14);
}
