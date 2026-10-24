extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type SwapKind = ();
pub const SwapKind_no_swap: SwapKind = 0;
pub const SwapKind_with_same_type: SwapKind = 1;
pub const SwapKind_with_different_type: SwapKind = 2;
pub type IterKind = ();
pub const IterKind_inner_view: IterKind = 0;
pub const IterKind_pattern: IterKind = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct IterSwapTrackingIterator_int_ptr__IterKind_inner_view_ {
    iter_: *mut (),
    flag_: *mut SwapKind,
}
impl IterSwapTrackingIterator_int_ptr__IterKind_inner_view_ {
    pub unsafe fn operator_deref(&self) -> *mut () {
        return &mut (*self.iter_);
    }
}
impl Default for IterSwapTrackingIterator_int_ptr__IterKind_inner_view_ {
    fn default() -> Self {
        unsafe {
            IterSwapTrackingIterator_int_ptr__IterKind_inner_view_ {
                iter_: std::ptr::null_mut(),
                flag_: std::ptr::null_mut(),
            }
        }
    }
}
pub unsafe fn iter_swap_0(
    lhs: *const IterSwapTrackingIterator_std___wrap_iter_int_ptr___IterKind_inner_view_,
    rhs: *const IterSwapTrackingIterator_std___wrap_iter_int_ptr___IterKind_inner_view_,
) {
    unimplemented!();
    (*(*lhs).flag_) = {
        (*(*rhs).flag_) = SwapKind_with_same_type;
        (*(*rhs).flag_)
    };
    (unsafe {
        let ___x: *const () = &(*lhs).iter_;
        let ___y: *const () = &(*rhs).iter_;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut iter_swap_1))(___x, ___y)
    });
    return;
}
pub unsafe fn iter_swap_2(
    lhs: *const IterSwapTrackingIterator_std___wrap_iter_int_ptr___IterKind_inner_view_,
    rhs: *const IterSwapTrackingIterator_int_ptr__IterKind_pattern_,
) {
    unimplemented!();
    (*(*lhs).flag_) = {
        (*(unsafe { IterSwapTrackingIterator_int_ptr__IterKind_pattern_::get_flag(&(*rhs)) })) =
            SwapKind_with_different_type;
        (*(unsafe { IterSwapTrackingIterator_int_ptr__IterKind_pattern_::get_flag(&(*rhs)) }))
    };
    (unsafe {
        let mut ___y: *mut () =
            (unsafe { IterSwapTrackingIterator_int_ptr__IterKind_pattern_::get_iter(&(*rhs)) });
        (*std::cell::LazyCell::force_mut(&mut *&raw mut iter_swap_1))(&(*lhs).iter_, &mut ___y)
    });
    return;
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg)]
pub struct IterSwapTrackingIterator_std___wrap_iter_int_ptr___IterKind_inner_view_ {
    iter_: (),
    flag_: *mut SwapKind,
}
impl IterSwapTrackingIterator_std___wrap_iter_int_ptr___IterKind_inner_view_ {
    pub unsafe fn get_iter(&self) -> () {
        return unimplemented!();
    }
    pub unsafe fn get_flag(&self) -> *mut SwapKind {
        return self.flag_;
    }
    pub unsafe fn new_1(mut iter: (), mut flag: Option<*mut SwapKind>) -> Self {
        let mut flag: *mut SwapKind = flag.unwrap_or_else(|| unsafe { std::ptr::null_mut() });
        let mut this = Self {
            iter_: (*unimplemented!()),
            flag_: flag,
        };
        this
    }
    pub unsafe fn operator_inc_5(
        &mut self,
    ) -> *mut IterSwapTrackingIterator_std___wrap_iter_int_ptr___IterKind_inner_view_ {
        unimplemented!();
        return &mut (*(self
            as *mut IterSwapTrackingIterator_std___wrap_iter_int_ptr___IterKind_inner_view_));
    }
    pub unsafe fn operator_deref(&self) -> *mut () {
        return &mut (*unimplemented!());
    }
    pub unsafe fn operator_eq(
        &self,
        other: *const IterSwapTrackingIterator_std___wrap_iter_int_ptr___IterKind_inner_view_,
    ) -> () {
        return unimplemented!();
    }
}
impl std::cmp::PartialEq
    for IterSwapTrackingIterator_std___wrap_iter_int_ptr___IterKind_inner_view_
{
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            IterSwapTrackingIterator_std___wrap_iter_int_ptr___IterKind_inner_view_::operator_eq(self, other as *const IterSwapTrackingIterator_std___wrap_iter_int_ptr___IterKind_inner_view_)
        }
    }
}
impl std::cmp::Eq for IterSwapTrackingIterator_std___wrap_iter_int_ptr___IterKind_inner_view_ {}
impl Default for IterSwapTrackingIterator_std___wrap_iter_int_ptr___IterKind_inner_view_ {
    fn default() -> Self {
        unsafe {
            IterSwapTrackingIterator_std___wrap_iter_int_ptr___IterKind_inner_view_ {
                iter_: unimplemented!(),
                flag_: std::ptr::null_mut(),
            }
        }
    }
}
pub unsafe fn iter_swap_3(
    lhs: *const IterSwapTrackingIterator_int_ptr__IterKind_pattern_,
    rhs: *const IterSwapTrackingIterator_int_ptr__IterKind_pattern_,
) {
    unimplemented!();
    (*(*lhs).flag_) = {
        (*(*rhs).flag_) = SwapKind_with_same_type;
        (*(*rhs).flag_)
    };
    (unsafe {
        let ___x: *const *mut () = &(*lhs).iter_;
        let ___y: *const *mut () = &(*rhs).iter_;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut iter_swap_1))(___x, ___y)
    });
    return;
}
pub unsafe fn iter_swap_4(
    lhs: *const IterSwapTrackingIterator_int_ptr__IterKind_pattern_,
    rhs: *const IterSwapTrackingIterator_std___wrap_iter_int_ptr___IterKind_inner_view_,
) {
    unimplemented!();
    (*(*lhs).flag_) = {
        (*(unsafe {
            IterSwapTrackingIterator_std___wrap_iter_int_ptr___IterKind_inner_view_::get_flag(
                &(*rhs),
            )
        })) = SwapKind_with_different_type;
        (*(unsafe {
            IterSwapTrackingIterator_std___wrap_iter_int_ptr___IterKind_inner_view_::get_flag(
                &(*rhs),
            )
        }))
    };
    (unsafe {
        let mut ___y: () = (unsafe {
            IterSwapTrackingIterator_std___wrap_iter_int_ptr___IterKind_inner_view_::get_iter(
                &(*rhs),
            )
        });
        (*std::cell::LazyCell::force_mut(&mut *&raw mut iter_swap_1))(&(*lhs).iter_, &mut ___y)
    });
    return;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct IterSwapTrackingIterator_int_ptr__IterKind_pattern_ {
    iter_: *mut (),
    flag_: *mut SwapKind,
}
impl IterSwapTrackingIterator_int_ptr__IterKind_pattern_ {
    pub unsafe fn get_iter(&self) -> *mut () {
        return self.iter_;
    }
    pub unsafe fn get_flag(&self) -> *mut SwapKind {
        return self.flag_;
    }
    pub unsafe fn new_1(mut iter: *mut (), mut flag: Option<*mut SwapKind>) -> Self {
        let mut flag: *mut SwapKind = flag.unwrap_or_else(|| unsafe { std::ptr::null_mut() });
        let mut this = Self {
            iter_: (*unimplemented!()),
            flag_: flag,
        };
        this
    }
    pub unsafe fn operator_inc_5(
        &mut self,
    ) -> *mut IterSwapTrackingIterator_int_ptr__IterKind_pattern_ {
        self.iter_.prefix_inc();
        return &mut (*(self as *mut IterSwapTrackingIterator_int_ptr__IterKind_pattern_));
    }
    pub unsafe fn operator_deref(&self) -> *mut () {
        return &mut (*self.iter_);
    }
    pub unsafe fn operator_eq(
        &self,
        other: *const IterSwapTrackingIterator_int_ptr__IterKind_pattern_,
    ) -> () {
        return ((self.iter_) == ((*other).iter_));
    }
}
impl std::cmp::PartialEq for IterSwapTrackingIterator_int_ptr__IterKind_pattern_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            IterSwapTrackingIterator_int_ptr__IterKind_pattern_::operator_eq(
                self,
                other as *const IterSwapTrackingIterator_int_ptr__IterKind_pattern_,
            )
        }
    }
}
impl std::cmp::Eq for IterSwapTrackingIterator_int_ptr__IterKind_pattern_ {}
impl Default for IterSwapTrackingIterator_int_ptr__IterKind_pattern_ {
    fn default() -> Self {
        unsafe {
            IterSwapTrackingIterator_int_ptr__IterKind_pattern_ {
                iter_: std::ptr::null_mut(),
                flag_: std::ptr::null_mut(),
            }
        }
    }
}
const _: () = assert!(
    true,
    "std::forward_iterator<IterSwapTrackingIterator<int*, IterKind::inner_view>> &&\n              !std::bidirectional_iterator<IterSwapTrackingIterator<int*, IterKind::inner_view>>"
);
pub unsafe fn test_5() -> () {
    {
        let mut jwv : () = std_ranges_join_with_view_std_ranges_owning_view_std_vector_std_string__std_allocator_std_string_____std_ranges_owning_view_std_string__ :: new_1 ( { unimplemented!() } , { unimplemented!() } , )  ;
        unimplemented!();
        let mut it: () = unimplemented!();
        (unsafe {
            let ___x: *const () = &it;
            let mut ___y: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut next_6))(it.clone(), 2_())
            });
            iter_swap_7(___x, &mut ___y)
        });
        unimplemented!();
        (unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut advance_8))(&mut it, 3_()) });
        (unsafe {
            let ___x: *const () = &(*unimplemented!());
            let mut ___y: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut next_6))(it.clone(), 2_())
            });
            iter_swap_7(___x, &mut ___y)
        });
        unimplemented!();
        (unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut advance_8))(&mut it, 3_()) });
        let it2: () = unimplemented!();
        (unsafe { iter_swap_7(&(*unimplemented!()), &it2) });
        unimplemented!();
        (unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut advance_8))(&mut it, 6_()) });
        (unsafe { iter_swap_7(&(*unimplemented!()), &it2) });
        unimplemented!();
        const _: () = assert!(true, "std::is_void_v<decltype(iter_swap(it, it))>");;
        const _: () = assert!(true, "std::is_void_v<decltype(iter_swap(it2, it2))>");;
        const _: () = assert!(!(false), "!CanIterSwap<std::ranges::iterator_t<const JWV>>");;
        const _: () = assert!(
            !(false),
            "!CanIterSwap<const std::ranges::iterator_t<const JWV>>"
        );;
    }
    {
        let mut v: () = unimplemented!();
        let mut pat: () = unimplemented!();
        let mut v_swap_kind: SwapKind = SwapKind_no_swap;
        let mut tracking_v: () = unimplemented!();
        let mut pat_swap_kind: SwapKind = SwapKind_no_swap;
        let mut tracking_pat: () = unimplemented!();
        let mut jwv: () = unimplemented!();
        let mut it1: () = unimplemented!();
        let mut it2: () =
            (unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut next_6))(it1.clone()) });
        unimplemented!();
        (unsafe { iter_swap_9(&it1, &it2) });
        unimplemented!();
        unimplemented!();
        (unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut advance_8))(&mut it2, 2_()) });
        v_swap_kind = SwapKind_no_swap;
        unimplemented!();
        (unsafe { iter_swap_9(&it1, &it2) });
        unimplemented!();
        unimplemented!();
        (unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut advance_8))(&mut it1, 4_()) });
        v_swap_kind = {
            pat_swap_kind = SwapKind_no_swap;
            pat_swap_kind
        };
        (unsafe { iter_swap_9(&it1, &it2) });
        unimplemented!();
        unimplemented!();
        (unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut advance_8))(&mut it2, 3_()) });
        v_swap_kind = {
            pat_swap_kind = SwapKind_no_swap;
            pat_swap_kind
        };
        (unsafe { iter_swap_9(&it1, &it2) });
        unimplemented!();
        unimplemented!();
    }
    {
        const _: () = assert!(!(false), "!CanIterSwap<std::ranges::iterator_t<JWV>>");;
        const _: () = assert!(!(false), "!CanIterSwap<const std::ranges::iterator_t<JWV>>");;
        const _: () = assert!(!(false), "!CanIterSwap<std::ranges::iterator_t<const JWV>>");;
        const _: () = assert!(
            !(false),
            "!CanIterSwap<const std::ranges::iterator_t<const JWV>>"
        );;
    }
    return true;
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
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
    (unsafe { test_5() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
