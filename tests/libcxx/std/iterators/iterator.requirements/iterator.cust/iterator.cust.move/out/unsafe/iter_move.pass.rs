extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn iter_move_0(
    i: *mut check_unqualified_lookup_unqualified_lookup_wrapper_move_tracker_ptr_,
) -> *mut move_tracker {
    return &mut (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut iter_move_1))(&mut (*i).base_)
    });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct check_unqualified_lookup_unqualified_lookup_wrapper_move_tracker_ptr_ {
    base_: *mut move_tracker,
}
impl check_unqualified_lookup_unqualified_lookup_wrapper_move_tracker_ptr_ {
    pub unsafe fn new_1(mut i: *mut move_tracker) -> Self {
        let mut this = Self { base_: i };
        this
    }
    pub unsafe fn operator_deref(&self) -> *mut move_tracker {
        return &mut (*self.base_);
    }
    pub unsafe fn operator_inc_4(
        &mut self,
    ) -> *mut check_unqualified_lookup_unqualified_lookup_wrapper_move_tracker_ptr_ {
        self.base_.prefix_inc();
        return &mut (*(self
            as *mut check_unqualified_lookup_unqualified_lookup_wrapper_move_tracker_ptr_));
    }
    pub unsafe fn operator_eq(
        &self,
        other: *const check_unqualified_lookup_unqualified_lookup_wrapper_move_tracker_ptr_,
    ) -> () {
        return ((self.base_) == ((*other).base_));
    }
}
impl std::cmp::PartialEq for check_unqualified_lookup_unqualified_lookup_wrapper_move_tracker_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            check_unqualified_lookup_unqualified_lookup_wrapper_move_tracker_ptr_::operator_eq(
                self,
                other
                    as *const check_unqualified_lookup_unqualified_lookup_wrapper_move_tracker_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for check_unqualified_lookup_unqualified_lookup_wrapper_move_tracker_ptr_ {}
impl Default for check_unqualified_lookup_unqualified_lookup_wrapper_move_tracker_ptr_ {
    fn default() -> Self {
        unsafe {
            check_unqualified_lookup_unqualified_lookup_wrapper_move_tracker_ptr_ {
                base_: std::ptr::null_mut(),
            }
        }
    }
}
pub type check_unqualified_lookup_unscoped_enum = ();
pub const check_unqualified_lookup_unscoped_enum_a: check_unqualified_lookup_unscoped_enum = 0;
pub const check_unqualified_lookup_unscoped_enum_b: check_unqualified_lookup_unscoped_enum = 1;
pub const check_unqualified_lookup_unscoped_enum_c: check_unqualified_lookup_unscoped_enum = 2;
pub unsafe fn iter_move_2(
    e: *mut check_unqualified_lookup_unscoped_enum,
) -> check_unqualified_lookup_unscoped_enum {
    return (*e);
}
pub type check_unqualified_lookup_scoped_enum = ();
pub const check_unqualified_lookup_scoped_enum_a: check_unqualified_lookup_scoped_enum = 0;
pub const check_unqualified_lookup_scoped_enum_b: check_unqualified_lookup_scoped_enum = 1;
pub const check_unqualified_lookup_scoped_enum_c: check_unqualified_lookup_scoped_enum = 2;
pub unsafe fn iter_move_3(
    _a0: *mut check_unqualified_lookup_scoped_enum,
) -> *mut check_unqualified_lookup_scoped_enum {
    return std::ptr::null_mut();
}
#[repr(C)]
#[derive(Copy, Clone, FnPtrArg, VaArg)]
pub union check_unqualified_lookup_some_union {
    pub x: (),
    pub y: (),
}
impl Default for check_unqualified_lookup_some_union {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
pub unsafe fn iter_move_4(u: *mut check_unqualified_lookup_some_union) -> () {
    return (*u).x;
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe)]
pub struct move_tracker {
    moves_: (),
}
impl move_tracker {
    pub unsafe fn move_from(other: *mut move_tracker) -> Self {
        let mut this = Self {
            moves_: (((*other).moves_) + (1_())),
        };
        (*other).moves_ = 0_();
        this
    }
    pub unsafe fn move_assign(&mut self, other: *mut move_tracker) -> *mut move_tracker {
        self.moves_ = (((*other).moves_) + (1_()));
        (*other).moves_ = 0_();
        return &mut (*(self as *mut move_tracker));
    }
    pub unsafe fn moves(&self) -> () {
        return self.moves_;
    }
}
impl Default for move_tracker {
    fn default() -> Self {
        unsafe { move_tracker { moves_: 0_() } }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct iterator_wrapper_move_tracker_ptr_ {
    base_: *mut move_tracker,
}
impl iterator_wrapper_move_tracker_ptr_ {
    pub unsafe fn new_1(mut i: *mut move_tracker) -> Self {
        let mut this = Self { base_: i };
        this
    }
    pub unsafe fn operator_deref_3(&self) -> *mut move_tracker {
        return &mut (*self.base_);
    }
    pub unsafe fn operator_deref_4(&mut self) -> *mut move_tracker {
        return &mut (*self.base_);
    }
    pub unsafe fn operator_inc_5(&mut self) -> *mut iterator_wrapper_move_tracker_ptr_ {
        self.base_.prefix_inc();
        return &mut (*(self as *mut iterator_wrapper_move_tracker_ptr_));
    }
    pub unsafe fn operator_eq(&self, other: *const iterator_wrapper_move_tracker_ptr_) -> () {
        return ((self.base_) == ((*other).base_));
    }
}
impl std::cmp::PartialEq for iterator_wrapper_move_tracker_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            iterator_wrapper_move_tracker_ptr_::operator_eq(
                self,
                other as *const iterator_wrapper_move_tracker_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for iterator_wrapper_move_tracker_ptr_ {}
impl Default for iterator_wrapper_move_tracker_ptr_ {
    fn default() -> Self {
        unsafe {
            iterator_wrapper_move_tracker_ptr_ {
                base_: std::ptr::null_mut(),
            }
        }
    }
}
pub unsafe fn unqualified_lookup_move_5(
    mut first_: *mut move_tracker,
    mut last_: *mut move_tracker,
    mut result_first_: *mut move_tracker,
    mut result_last_: *mut move_tracker,
) {
    let mut first: check_unqualified_lookup_unqualified_lookup_wrapper_move_tracker_ptr_ =
        check_unqualified_lookup_unqualified_lookup_wrapper_move_tracker_ptr_::new_1({ first_ });
    let mut last: check_unqualified_lookup_unqualified_lookup_wrapper_move_tracker_ptr_ =
        check_unqualified_lookup_unqualified_lookup_wrapper_move_tracker_ptr_::new_1({ last_ });
    let mut result_first: check_unqualified_lookup_unqualified_lookup_wrapper_move_tracker_ptr_ =
        check_unqualified_lookup_unqualified_lookup_wrapper_move_tracker_ptr_::new_1({
            result_first_
        });
    let mut result_last: check_unqualified_lookup_unqualified_lookup_wrapper_move_tracker_ptr_ =
        check_unqualified_lookup_unqualified_lookup_wrapper_move_tracker_ptr_::new_1({
            result_last_
        });
    const _: () = assert!(true, "!noexcept(std::ranges::iter_move(first))");;
    'loop_: while (!(unsafe {
        check_unqualified_lookup_unqualified_lookup_wrapper_move_tracker_ptr_::operator_eq(
            &first, &last,
        )
    })) && (!(unsafe {
        check_unqualified_lookup_unqualified_lookup_wrapper_move_tracker_ptr_::operator_eq(
            &result_first,
            &result_last,
        )
    })) {
        (unsafe {
            move_tracker::move_assign(
                &mut (*(unsafe {
                    check_unqualified_lookup_unqualified_lookup_wrapper_move_tracker_ptr_ :: operator_deref ( & result_first  , )
                })),
                &mut (unsafe {
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut iter_move_1))(&mut first)
                }),
            )
        });
        {
            &(unsafe {
                check_unqualified_lookup_unqualified_lookup_wrapper_move_tracker_ptr_ :: operator_inc_4 ( &mut first  , )
            });
            (unsafe {
                check_unqualified_lookup_unqualified_lookup_wrapper_move_tracker_ptr_ :: operator_inc_4 ( &mut result_first  , )
            })
        };
    }
}
pub unsafe fn lvalue_move_6(
    mut first_: *mut move_tracker,
    mut last_: *mut move_tracker,
    mut result_first_: *mut move_tracker,
    mut result_last_: *mut move_tracker,
) {
    let mut first: iterator_wrapper_move_tracker_ptr_ =
        iterator_wrapper_move_tracker_ptr_::new_1({ first_ });
    let mut last: iterator_wrapper_move_tracker_ptr_ =
        iterator_wrapper_move_tracker_ptr_::new_1({ last_ });
    let mut result_first: iterator_wrapper_move_tracker_ptr_ =
        iterator_wrapper_move_tracker_ptr_::new_1({ result_first_ });
    let mut result_last: iterator_wrapper_move_tracker_ptr_ =
        iterator_wrapper_move_tracker_ptr_::new_1({ result_last_ });
    const _: () = assert!(true, "!noexcept(std::ranges::iter_move(first))");;
    'loop_: while (!(unsafe { iterator_wrapper_move_tracker_ptr_::operator_eq(&first, &last) }))
        && (!(unsafe {
            iterator_wrapper_move_tracker_ptr_::operator_eq(&result_first, &result_last)
        }))
    {
        (unsafe {
            move_tracker::move_assign(
                &mut (*(unsafe {
                    iterator_wrapper_move_tracker_ptr_::operator_deref_3(&result_first)
                })),
                &mut (unsafe {
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut iter_move_1))(&mut first)
                }),
            )
        });
        {
            &(unsafe { iterator_wrapper_move_tracker_ptr_::operator_inc_5(&mut first) });
            (unsafe { iterator_wrapper_move_tracker_ptr_::operator_inc_5(&mut result_first) })
        };
    }
}
pub unsafe fn rvalue_move_7(
    mut first_: *mut move_tracker,
    mut last_: *mut move_tracker,
    mut result_first_: *mut move_tracker,
    mut result_last_: *mut move_tracker,
) {
    let mut first: iterator_wrapper_move_tracker_ptr_ =
        iterator_wrapper_move_tracker_ptr_::new_1({ first_ });
    let mut last: iterator_wrapper_move_tracker_ptr_ =
        iterator_wrapper_move_tracker_ptr_::new_1({ last_ });
    let mut result_first: iterator_wrapper_move_tracker_ptr_ =
        iterator_wrapper_move_tracker_ptr_::new_1({ result_first_ });
    let mut result_last: iterator_wrapper_move_tracker_ptr_ =
        iterator_wrapper_move_tracker_ptr_::new_1({ result_last_ });
    const _: () = assert!(true, "noexcept(std::ranges::iter_move(std::move(first)))");;
    'loop_: while (!(unsafe { iterator_wrapper_move_tracker_ptr_::operator_eq(&first, &last) }))
        && (!(unsafe {
            iterator_wrapper_move_tracker_ptr_::operator_eq(&result_first, &result_last)
        }))
    {
        let mut i: iterator_wrapper_move_tracker_ptr_ = first;
        (unsafe {
            move_tracker::move_assign(
                &mut (*(unsafe {
                    iterator_wrapper_move_tracker_ptr_::operator_deref_3(&result_first)
                })),
                &mut (unsafe {
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut iter_move_1))(&mut i)
                }),
            )
        });
        {
            &(unsafe { iterator_wrapper_move_tracker_ptr_::operator_inc_5(&mut first) });
            (unsafe { iterator_wrapper_move_tracker_ptr_::operator_inc_5(&mut result_first) })
        };
    }
}
pub unsafe fn iter_move_8(_a0: *mut WithADL_true_) -> () {
    return 0_();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct WithADL_true_ {}
pub unsafe fn iter_move_9(_a0: *mut WithADL_false_) -> () {
    return 0_();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct WithADL_false_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct WithoutADL_true_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct WithoutADL_false_ {}
pub unsafe fn test_10() -> () {
    let full_size: () = 100_();
    let half_size: () = ((full_size) / (2_()));
    let reset: () = 0_();
    let mut v1: () = unimplemented!();
    let mut move_counter_is: FnPtr<Generic> =
        lambda_unsafe!(Generic, {}, |n: ()| -> FnPtr<Generic> {
            return lambda_unsafe!(
                Generic,
                {
                    let n: () = n;
                },
                |x: *const move_tracker| -> () {
                    return ((unsafe { move_tracker::moves(&(*x)) }) == (n));
                }
            );
        });
    let mut v2: () = unimplemented!();
    (unsafe {
        let _first_: *mut move_tracker = unimplemented!();
        let _last_: *mut move_tracker = unimplemented!();
        let _result_first_: *mut move_tracker = unimplemented!();
        let _result_last_: *mut move_tracker = unimplemented!();
        unqualified_lookup_move_5(_first_, _last_, _result_first_, _result_last_)
    });
    unimplemented!();
    unimplemented!();
    let mut v3: () = unimplemented!();
    (unsafe {
        let _first_: *mut move_tracker = unimplemented!().offset((half_size) as isize);
        let _last_: *mut move_tracker = unimplemented!();
        let _result_first_: *mut move_tracker = unimplemented!();
        let _result_last_: *mut move_tracker = unimplemented!();
        unqualified_lookup_move_5(_first_, _last_, _result_first_, _result_last_)
    });
    unimplemented!();
    unimplemented!();
    let mut v4: () = unimplemented!();
    (unsafe {
        let _first_: *mut move_tracker = unimplemented!();
        let _last_: *mut move_tracker = unimplemented!();
        let _result_first_: *mut move_tracker = unimplemented!();
        let _result_last_: *mut move_tracker = unimplemented!();
        unqualified_lookup_move_5(_first_, _last_, _result_first_, _result_last_)
    });
    unimplemented!();
    unimplemented!();
    (unsafe {
        let _first_: *mut move_tracker = unimplemented!();
        let _last_: *mut move_tracker = unimplemented!();
        let _result_first_: *mut move_tracker = unimplemented!().offset((half_size) as isize);
        let _result_last_: *mut move_tracker = unimplemented!();
        lvalue_move_6(_first_, _last_, _result_first_, _result_last_)
    });
    unimplemented!();
    unimplemented!();
    (unsafe {
        let _first_: *mut move_tracker = unimplemented!();
        let _last_: *mut move_tracker = unimplemented!();
        let _result_first_: *mut move_tracker = unimplemented!();
        let _result_last_: *mut move_tracker = unimplemented!();
        lvalue_move_6(_first_, _last_, _result_first_, _result_last_)
    });
    unimplemented!();
    unimplemented!();
    (unsafe {
        let _first_: *mut move_tracker = unimplemented!();
        let _last_: *mut move_tracker = unimplemented!();
        let _result_first_: *mut move_tracker = unimplemented!();
        let _result_last_: *mut move_tracker = unimplemented!();
        rvalue_move_7(_first_, _last_, _result_first_, _result_last_)
    });
    unimplemented!();
    unimplemented!();
    (unsafe {
        let _first_: *mut move_tracker = unimplemented!().offset((half_size) as isize);
        let _last_: *mut move_tracker = unimplemented!();
        let _result_first_: *mut move_tracker = unimplemented!();
        let _result_last_: *mut move_tracker = unimplemented!();
        rvalue_move_7(_first_, _last_, _result_first_, _result_last_)
    });
    unimplemented!();
    unimplemented!();
    let mut unscoped: check_unqualified_lookup_unscoped_enum =
        check_unqualified_lookup_unscoped_enum_a;
    unimplemented!();
    const _: () = assert!(true, "!noexcept(std::ranges::iter_move(unscoped))");;
    let mut scoped: check_unqualified_lookup_scoped_enum = check_unqualified_lookup_scoped_enum_a;
    unimplemented!();
    const _: () = assert!(true, "noexcept(std::ranges::iter_move(scoped))");;
    let mut some_union: check_unqualified_lookup_some_union =
        check_unqualified_lookup_some_union { x: 0_() };
    unimplemented!();
    const _: () = assert!(true, "!noexcept(std::ranges::iter_move(some_union))");;
    const _: () = assert!(
        true,
        "noexcept(std::ranges::iter_move(std::declval<WithADL<true>>()))"
    );;
    const _: () = assert!(
        true,
        "!noexcept(std::ranges::iter_move(std::declval<WithADL<false>>()))"
    );;
    const _: () = assert!(
        true,
        "noexcept(std::ranges::iter_move(std::declval<WithoutADL<true>>()))"
    );;
    const _: () = assert!(
        true,
        "!noexcept(std::ranges::iter_move(std::declval<WithoutADL<false>>()))"
    );;
    return true;
}
const _: () = assert!(true, "!std::is_invocable_v<IterMoveT, int*, int*>");
const _: () = assert!(true, "!std::is_invocable_v<IterMoveT, int>");
const _: () = assert!(true, "std::is_invocable_v<IterMoveT, Holder<Incomplete>**>");
const _: () = assert!(
    true,
    "std::is_invocable_v<IterMoveT, Holder<Incomplete>**&>"
);
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
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
    (unsafe { test_10() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
