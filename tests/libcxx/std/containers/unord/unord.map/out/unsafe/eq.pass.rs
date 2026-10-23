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
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub unsafe fn outstanding_alloc() -> () {
        unimplemented!();
        return ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_1))
            .wrapping_sub((*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_2))));
    }
    pub unsafe fn reset() {
        unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut disable_default_constructor_3)) = false;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut outstanding_bytes_0)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_1)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_2)) = 0_();
    }
}
pub static mut outstanding_bytes_0: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut alloc_count_1: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut dealloc_count_2: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut disable_default_constructor_3: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { false });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_std_pair_const_int__std_string___std_integral_constant_unsigned_long__0__ {
    ptr_: *mut (),
}
impl min_pointer_std_pair_const_int__std_string___std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    ptr_: (),
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_const_std_pair_const_int__std_string___std_integral_constant_unsigned_long__0__
{
    ptr_: *const (),
}
impl min_pointer_const_std_pair_const_int__std_string___std_integral_constant_unsigned_long__0__ {
    unsafe fn new_1(mut p: *const ()) -> Self {
        let mut this = Self { ptr_: p };
        this
    }
    pub unsafe fn operator_arrow(&self) -> *const () {
        return self.ptr_;
    }
    pub unsafe fn pointer_to(
        t: *const (),
    ) -> min_pointer_const_std_pair_const_int__std_string___std_integral_constant_unsigned_long__0__
    {
        return min_pointer_const_std_pair_const_int__std_string___std_integral_constant_unsigned_long__0__ :: new_1 ( { unimplemented!()  } , )   ;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_const_void__std_integral_constant_unsigned_long__0__ {
    ptr_: (),
}
impl min_pointer_const_void__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__std_string___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { ptr_ : *mut min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__std_string___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ , }
impl min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__std_string___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { unsafe fn new_1 ( mut p : *mut min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__std_string___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ , ) -> Self { let mut this = Self { ptr_ : p , } ;
 this } pub unsafe fn new_2 ( mut _a0 : () , ) -> Self { let mut this = Self { ptr_ : std::ptr::null_mut() , } ;
 this } pub unsafe fn to_ ( &self , ) -> () { return ! ( ( self . ptr_  ).is_null() )  ;
 } pub unsafe fn operator_deref ( &self , ) -> *mut min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__std_string___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ { return & mut ( * self . ptr_   )  ;
 } pub unsafe fn operator_inc_8 ( &mut self , ) -> *mut min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__std_string___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { self . ptr_  .prefix_inc() ;
 return & mut ( * ( self as *mut min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__std_string___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  )  )  ;
 } pub unsafe fn operator_index ( &self , mut n : () , ) -> *mut min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__std_string___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ { return & mut ( * self . ptr_  . offset ( ( n ) as isize ) )  ;
 } }
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_std_pair_const_int__std_string__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__std_string___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0___
{}
impl min_allocator_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__std_string___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0___ { pub unsafe fn allocate ( &mut self , mut n : () , ) -> min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__std_string___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { return min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__std_string___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ :: new_1 ( { unimplemented!()  } , )   ;
 } pub unsafe fn deallocate ( &mut self , mut p : min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__std_string___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ , mut n : () , ) { unimplemented!()  ;
 } }
pub unsafe fn testEquality_4(t1: *const (), t2: *const (), mut isEqual: ()) -> () {
    if isEqual {
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (!(unimplemented!())) {
            return false;
        }
        if (!(unimplemented!())) {
            return false;
        }
    } else {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
    }
    return true;
}
pub unsafe fn testEquality_5(t1: *const (), t2: *const (), mut isEqual: ()) -> () {
    if isEqual {
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (!(unimplemented!())) {
            return false;
        }
        if (!(unimplemented!())) {
            return false;
        }
    } else {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
    }
    return true;
}
pub unsafe fn operator_lt_6(lhs: *const LessAndEqComp, rhs: *const LessAndEqComp) -> () {
    return (((*lhs).value) < ((*rhs).value));
}
pub unsafe fn operator_eq_7(lhs: *const LessAndEqComp, rhs: *const LessAndEqComp) -> () {
    return (((*lhs).value) == ((*rhs).value));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct LessAndEqComp {
    pub value: (),
}
impl LessAndEqComp {
    pub unsafe fn new(mut v: ()) -> Self {
        let mut this = Self { value: v };
        this
    }
}
impl std::cmp::Ord for LessAndEqComp {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if operator_lt_6(self as *const LessAndEqComp, other as *const LessAndEqComp) {
                std::cmp::Ordering::Less
            } else if operator_lt_6(other as *const LessAndEqComp, self as *const LessAndEqComp) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for LessAndEqComp {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for LessAndEqComp {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_7(self as *const LessAndEqComp, other as *const LessAndEqComp) }
    }
}
impl std::cmp::Eq for LessAndEqComp {}
pub unsafe fn operator_cmp_8(mut _a0: StrongOrder, mut _a1: StrongOrder) -> () {
    {
        let mut cmp: () = std::cmp::Ord::cmp(&(_a0.value), &(_a1.value));
        if !(unimplemented!()) {
            return unimplemented!();
        }
    }
    return unimplemented!();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct StrongOrder {
    pub value: (),
}
impl StrongOrder {
    pub unsafe fn new(mut v: ()) -> Self {
        let mut this = Self { value: v };
        this
    }
}
impl std::cmp::Ord for StrongOrder {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { operator_cmp_8(self.clone(), other.clone()) }
    }
}
impl std::cmp::PartialOrd for StrongOrder {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for StrongOrder {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_cmp_8(self.clone(), other.clone()) == std::cmp::Ordering::Equal }
    }
}
impl std::cmp::Eq for StrongOrder {}
pub unsafe fn operator_cmp_9(mut _a0: WeakOrder, mut _a1: WeakOrder) -> () {
    {
        let mut cmp: () = unimplemented!();
        if !(unimplemented!()) {
            return unimplemented!();
        }
    }
    return unimplemented!();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct WeakOrder {
    pub value: (),
}
impl WeakOrder {
    pub unsafe fn new(mut v: ()) -> Self {
        let mut this = Self { value: v };
        this
    }
}
impl std::cmp::Ord for WeakOrder {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { operator_cmp_9(self.clone(), other.clone()) }
    }
}
impl std::cmp::PartialOrd for WeakOrder {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for WeakOrder {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_cmp_9(self.clone(), other.clone()) == std::cmp::Ordering::Equal }
    }
}
impl std::cmp::Eq for WeakOrder {}
pub unsafe fn operator_cmp_10(mut lhs: PartialOrder, mut rhs: PartialOrder) -> () {
    if ((lhs.value) == (unsafe { std_numeric_limits_int_::min() }))
        || ((rhs.value) == (unsafe { std_numeric_limits_int_::min() }))
    {
        return unimplemented!();
    }
    if ((lhs.value) == (unsafe { std_numeric_limits_int_::max() }))
        || ((rhs.value) == (unsafe { std_numeric_limits_int_::max() }))
    {
        return unimplemented!();
    }
    return unimplemented!();
}
pub unsafe fn operator_eq_11(mut lhs: PartialOrder, mut rhs: PartialOrder) -> () {
    return unimplemented!();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct PartialOrder {
    pub value: (),
}
impl PartialOrder {
    pub unsafe fn new(mut v: ()) -> Self {
        let mut this = Self { value: v };
        this
    }
}
impl std::cmp::Ord for PartialOrder {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { operator_cmp_10(self.clone(), other.clone()) }
    }
}
impl std::cmp::PartialOrd for PartialOrder {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for PartialOrder {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_11(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for PartialOrder {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct NonComparable {}
const _: () = assert!(!(false), "!std::equality_comparable<NonComparable>");
const _: () = assert!(!(false), "!HasOperatorEqual<NonComparable>");
const _: () = assert!(!(false), "!HasOperatorGreaterThan<NonComparable>");
const _: () = assert!(!(false), "!HasOperatorGreaterThanEqual<NonComparable>");
const _: () = assert!(!(false), "!HasOperatorLessThan<NonComparable>");
const _: () = assert!(!(false), "!HasOperatorLessThanEqual<NonComparable>");
const _: () = assert!(!(false), "!HasOperatorNotEqual<NonComparable>");
const _: () = assert!(!(false), "!HasOperatorSpaceship<NonComparable>");
pub unsafe fn operator_eq_12(_a0: *const EqualityComparable, _a1: *const EqualityComparable) -> () {
    return (((*_a0).value_) == ((*_a1).value_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct EqualityComparable {
    value_: (),
}
impl EqualityComparable {
    pub unsafe fn new(mut value: ()) -> Self {
        let mut this = Self { value_: value };
        this
    }
}
impl std::cmp::PartialEq for EqualityComparable {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_12(
                self as *const EqualityComparable,
                other as *const EqualityComparable,
            )
        }
    }
}
impl std::cmp::Eq for EqualityComparable {}
const _: () = assert!(true, "std::equality_comparable<EqualityComparable>");
const _: () = assert!(true, "HasOperatorEqual<EqualityComparable>");
const _: () = assert!(true, "HasOperatorNotEqual<EqualityComparable>");
pub unsafe fn operator_eq_13(_a0: *const ThreeWayComparable, _a1: *const ThreeWayComparable) -> () {
    return (((*_a0).value_) == ((*_a1).value_));
}
pub unsafe fn operator_cmp_14(
    _a0: *const ThreeWayComparable,
    _a1: *const ThreeWayComparable,
) -> () {
    {
        let mut cmp: () = std::cmp::Ord::cmp(&((*_a0).value_), &((*_a1).value_));
        if !(unimplemented!()) {
            return unimplemented!();
        }
    }
    return unimplemented!();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ThreeWayComparable {
    value_: (),
}
impl ThreeWayComparable {
    pub unsafe fn new(mut value: ()) -> Self {
        let mut this = Self { value_: value };
        this
    }
}
impl std::cmp::Ord for ThreeWayComparable {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            operator_cmp_14(
                self as *const ThreeWayComparable,
                other as *const ThreeWayComparable,
            )
        }
    }
}
impl std::cmp::PartialOrd for ThreeWayComparable {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for ThreeWayComparable {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_13(
                self as *const ThreeWayComparable,
                other as *const ThreeWayComparable,
            )
        }
    }
}
impl std::cmp::Eq for ThreeWayComparable {}
const _: () = assert!(true, "std::equality_comparable<ThreeWayComparable>");
const _: () = assert!(true, "std::three_way_comparable<ThreeWayComparable>");
const _: () = assert!(true, "HasOperatorEqual<ThreeWayComparable>");
const _: () = assert!(true, "HasOperatorGreaterThan<ThreeWayComparable>");
const _: () = assert!(true, "HasOperatorGreaterThanEqual<ThreeWayComparable>");
const _: () = assert!(true, "HasOperatorLessThan<ThreeWayComparable>");
const _: () = assert!(true, "HasOperatorLessThanEqual<ThreeWayComparable>");
const _: () = assert!(true, "HasOperatorNotEqual<ThreeWayComparable>");
const _: () = assert!(true, "HasOperatorSpaceship<ThreeWayComparable>");
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ImplicitBool_E1 {
    pub x: (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ImplicitBool_E2 {
    pub y: (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ImplicitBool {
    pub val: (),
}
impl ImplicitBool {
    pub unsafe fn to__1(&self) -> () {
        return self.val;
    }
}
pub unsafe fn operator_eq_15(mut lhs: ImplicitBool_E1, mut rhs: ImplicitBool_E2) -> ImplicitBool {
    return ImplicitBool {
        val: ((lhs.x) == (rhs.y)),
    };
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
    {
        let mut a: [(); 8] = [
            {
                let mut __tmp_0: () = 10_();
                std_pair_int__std_string_::new_1({ &mut __tmp_0 }, {
                    &std::mem::transmute(*b"ten\0")
                })
            },
            {
                let mut __tmp_1: () = 20_();
                std_pair_int__std_string_::new_2({ &mut __tmp_1 }, {
                    &std::mem::transmute(*b"twenty\0")
                })
            },
            {
                let mut __tmp_2: () = 30_();
                std_pair_int__std_string_::new_2({ &mut __tmp_2 }, {
                    &std::mem::transmute(*b"thirty\0")
                })
            },
            {
                let mut __tmp_3: () = 40_();
                std_pair_int__std_string_::new_3({ &mut __tmp_3 }, {
                    &std::mem::transmute(*b"forty\0")
                })
            },
            {
                let mut __tmp_4: () = 50_();
                std_pair_int__std_string_::new_3({ &mut __tmp_4 }, {
                    &std::mem::transmute(*b"fifty\0")
                })
            },
            {
                let mut __tmp_5: () = 60_();
                std_pair_int__std_string_::new_3({ &mut __tmp_5 }, {
                    &std::mem::transmute(*b"sixty\0")
                })
            },
            {
                let mut __tmp_6: () = 70_();
                std_pair_int__std_string_::new_4({ &mut __tmp_6 }, {
                    &std::mem::transmute(*b"seventy\0")
                })
            },
            {
                let mut __tmp_7: () = 80_();
                std_pair_int__std_string_::new_2({ &mut __tmp_7 }, {
                    &std::mem::transmute(*b"eighty\0")
                })
            },
        ];
        let c1: () = unimplemented!();
        let c2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut a: [(); 8] = [
            {
                let mut __tmp_8: () = 10_();
                std_pair_int__std_string_::new_1({ &mut __tmp_8 }, {
                    &std::mem::transmute(*b"ten\0")
                })
            },
            {
                let mut __tmp_9: () = 20_();
                std_pair_int__std_string_::new_2({ &mut __tmp_9 }, {
                    &std::mem::transmute(*b"twenty\0")
                })
            },
            {
                let mut __tmp_10: () = 30_();
                std_pair_int__std_string_::new_2({ &mut __tmp_10 }, {
                    &std::mem::transmute(*b"thirty\0")
                })
            },
            {
                let mut __tmp_11: () = 40_();
                std_pair_int__std_string_::new_3({ &mut __tmp_11 }, {
                    &std::mem::transmute(*b"forty\0")
                })
            },
            {
                let mut __tmp_12: () = 50_();
                std_pair_int__std_string_::new_3({ &mut __tmp_12 }, {
                    &std::mem::transmute(*b"fifty\0")
                })
            },
            {
                let mut __tmp_13: () = 60_();
                std_pair_int__std_string_::new_3({ &mut __tmp_13 }, {
                    &std::mem::transmute(*b"sixty\0")
                })
            },
            {
                let mut __tmp_14: () = 70_();
                std_pair_int__std_string_::new_4({ &mut __tmp_14 }, {
                    &std::mem::transmute(*b"seventy\0")
                })
            },
            {
                let mut __tmp_15: () = 80_();
                std_pair_int__std_string_::new_2({ &mut __tmp_15 }, {
                    &std::mem::transmute(*b"eighty\0")
                })
            },
        ];
        let c1: () = unimplemented!();
        let c2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut a: [(); 8] = [
            {
                let mut __tmp_16: () = 10_();
                std_pair_int__std_string_::new_1({ &mut __tmp_16 }, {
                    &std::mem::transmute(*b"ten\0")
                })
            },
            {
                let mut __tmp_17: () = 20_();
                std_pair_int__std_string_::new_2({ &mut __tmp_17 }, {
                    &std::mem::transmute(*b"twenty\0")
                })
            },
            {
                let mut __tmp_18: () = 30_();
                std_pair_int__std_string_::new_2({ &mut __tmp_18 }, {
                    &std::mem::transmute(*b"thirty\0")
                })
            },
            {
                let mut __tmp_19: () = 40_();
                std_pair_int__std_string_::new_3({ &mut __tmp_19 }, {
                    &std::mem::transmute(*b"forty\0")
                })
            },
            {
                let mut __tmp_20: () = 50_();
                std_pair_int__std_string_::new_3({ &mut __tmp_20 }, {
                    &std::mem::transmute(*b"fifty\0")
                })
            },
            {
                let mut __tmp_21: () = 60_();
                std_pair_int__std_string_::new_3({ &mut __tmp_21 }, {
                    &std::mem::transmute(*b"sixty\0")
                })
            },
            {
                let mut __tmp_22: () = 70_();
                std_pair_int__std_string_::new_4({ &mut __tmp_22 }, {
                    &std::mem::transmute(*b"seventy\0")
                })
            },
            {
                let mut __tmp_23: () = 80_();
                std_pair_int__std_string_::new_2({ &mut __tmp_23 }, {
                    &std::mem::transmute(*b"eighty\0")
                })
            },
        ];
        let mut c1: () = unimplemented!();
        let mut c2: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        (unsafe {
            let mut ___x: () = {
                let mut __tmp_24: () = 90_();
                std_pair_int__std_string_::new_2({ &mut __tmp_24 }, {
                    &std::mem::transmute(*b"ninety\0")
                })
            };
            c2.insert_45_struct_std_pair_int__std_string__0(&mut ___x)
        });
        unimplemented!();
        (unsafe {
            let mut ___x: () = {
                let mut __tmp_25: () = 90_();
                std_pair_int__std_string_::new_2({ &mut __tmp_25 }, {
                    &std::mem::transmute(*b"ninety\0")
                })
            };
            c1.insert_45_struct_std_pair_int__std_string__0(&mut ___x)
        });
        unimplemented!();
    }
    {
        let mut a: [(); 8] = [
            {
                let mut __tmp_26: () = 10_();
                std_pair_int__std_string_::new_1({ &mut __tmp_26 }, {
                    &std::mem::transmute(*b"ten\0")
                })
            },
            {
                let mut __tmp_27: () = 20_();
                std_pair_int__std_string_::new_2({ &mut __tmp_27 }, {
                    &std::mem::transmute(*b"twenty\0")
                })
            },
            {
                let mut __tmp_28: () = 30_();
                std_pair_int__std_string_::new_2({ &mut __tmp_28 }, {
                    &std::mem::transmute(*b"thirty\0")
                })
            },
            {
                let mut __tmp_29: () = 40_();
                std_pair_int__std_string_::new_3({ &mut __tmp_29 }, {
                    &std::mem::transmute(*b"forty\0")
                })
            },
            {
                let mut __tmp_30: () = 50_();
                std_pair_int__std_string_::new_3({ &mut __tmp_30 }, {
                    &std::mem::transmute(*b"fifty\0")
                })
            },
            {
                let mut __tmp_31: () = 60_();
                std_pair_int__std_string_::new_3({ &mut __tmp_31 }, {
                    &std::mem::transmute(*b"sixty\0")
                })
            },
            {
                let mut __tmp_32: () = 70_();
                std_pair_int__std_string_::new_4({ &mut __tmp_32 }, {
                    &std::mem::transmute(*b"seventy\0")
                })
            },
            {
                let mut __tmp_33: () = 80_();
                std_pair_int__std_string_::new_2({ &mut __tmp_33 }, {
                    &std::mem::transmute(*b"eighty\0")
                })
            },
        ];
        let mut c1: () = unimplemented!();
        let mut c2: () = unimplemented!();
        unimplemented!();
        (unsafe {
            let mut ___x: () = {
                let mut __tmp_34: () = 90_();
                std_pair_int__std_string_::new_2({ &mut __tmp_34 }, {
                    &std::mem::transmute(*b"ninety\0")
                })
            };
            c1.insert_45_struct_std_pair_int__std_string__0(&mut ___x)
        });
        (unsafe {
            let mut ___x: () = {
                let mut __tmp_35: () = 100_();
                std_pair_int__std_string_::new_5({ &mut __tmp_35 }, {
                    &std::mem::transmute(*b"onehundred\0")
                })
            };
            c2.insert_45_struct_std_pair_int__std_string__0(&mut ___x)
        });
        unimplemented!();
    }
    {
        let mut a: [(); 8] = [
            {
                let mut __tmp_36: () = 10_();
                std_pair_int__std_string_::new_1({ &mut __tmp_36 }, {
                    &std::mem::transmute(*b"ten\0")
                })
            },
            {
                let mut __tmp_37: () = 20_();
                std_pair_int__std_string_::new_2({ &mut __tmp_37 }, {
                    &std::mem::transmute(*b"twenty\0")
                })
            },
            {
                let mut __tmp_38: () = 30_();
                std_pair_int__std_string_::new_2({ &mut __tmp_38 }, {
                    &std::mem::transmute(*b"thirty\0")
                })
            },
            {
                let mut __tmp_39: () = 40_();
                std_pair_int__std_string_::new_3({ &mut __tmp_39 }, {
                    &std::mem::transmute(*b"forty\0")
                })
            },
            {
                let mut __tmp_40: () = 50_();
                std_pair_int__std_string_::new_3({ &mut __tmp_40 }, {
                    &std::mem::transmute(*b"fifty\0")
                })
            },
            {
                let mut __tmp_41: () = 60_();
                std_pair_int__std_string_::new_3({ &mut __tmp_41 }, {
                    &std::mem::transmute(*b"sixty\0")
                })
            },
            {
                let mut __tmp_42: () = 70_();
                std_pair_int__std_string_::new_4({ &mut __tmp_42 }, {
                    &std::mem::transmute(*b"seventy\0")
                })
            },
            {
                let mut __tmp_43: () = 80_();
                std_pair_int__std_string_::new_2({ &mut __tmp_43 }, {
                    &std::mem::transmute(*b"eighty\0")
                })
            },
        ];
        let c1: () = unimplemented!();
        let c2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut a: [(); 8] = [
            {
                let mut __tmp_44: () = 10_();
                std_pair_int__std_string_::new_1({ &mut __tmp_44 }, {
                    &std::mem::transmute(*b"ten\0")
                })
            },
            {
                let mut __tmp_45: () = 20_();
                std_pair_int__std_string_::new_2({ &mut __tmp_45 }, {
                    &std::mem::transmute(*b"twenty\0")
                })
            },
            {
                let mut __tmp_46: () = 30_();
                std_pair_int__std_string_::new_2({ &mut __tmp_46 }, {
                    &std::mem::transmute(*b"thirty\0")
                })
            },
            {
                let mut __tmp_47: () = 40_();
                std_pair_int__std_string_::new_3({ &mut __tmp_47 }, {
                    &std::mem::transmute(*b"forty\0")
                })
            },
            {
                let mut __tmp_48: () = 50_();
                std_pair_int__std_string_::new_3({ &mut __tmp_48 }, {
                    &std::mem::transmute(*b"fifty\0")
                })
            },
            {
                let mut __tmp_49: () = 60_();
                std_pair_int__std_string_::new_3({ &mut __tmp_49 }, {
                    &std::mem::transmute(*b"sixty\0")
                })
            },
            {
                let mut __tmp_50: () = 70_();
                std_pair_int__std_string_::new_4({ &mut __tmp_50 }, {
                    &std::mem::transmute(*b"seventy\0")
                })
            },
            {
                let mut __tmp_51: () = 80_();
                std_pair_int__std_string_::new_2({ &mut __tmp_51 }, {
                    &std::mem::transmute(*b"eighty\0")
                })
            },
        ];
        let c1: () = unimplemented!();
        let c2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut a: [(); 8] = [
            {
                let mut __tmp_52: () = 10_();
                std_pair_int__std_string_::new_1({ &mut __tmp_52 }, {
                    &std::mem::transmute(*b"ten\0")
                })
            },
            {
                let mut __tmp_53: () = 20_();
                std_pair_int__std_string_::new_2({ &mut __tmp_53 }, {
                    &std::mem::transmute(*b"twenty\0")
                })
            },
            {
                let mut __tmp_54: () = 30_();
                std_pair_int__std_string_::new_2({ &mut __tmp_54 }, {
                    &std::mem::transmute(*b"thirty\0")
                })
            },
            {
                let mut __tmp_55: () = 40_();
                std_pair_int__std_string_::new_3({ &mut __tmp_55 }, {
                    &std::mem::transmute(*b"forty\0")
                })
            },
            {
                let mut __tmp_56: () = 50_();
                std_pair_int__std_string_::new_3({ &mut __tmp_56 }, {
                    &std::mem::transmute(*b"fifty\0")
                })
            },
            {
                let mut __tmp_57: () = 60_();
                std_pair_int__std_string_::new_3({ &mut __tmp_57 }, {
                    &std::mem::transmute(*b"sixty\0")
                })
            },
            {
                let mut __tmp_58: () = 70_();
                std_pair_int__std_string_::new_4({ &mut __tmp_58 }, {
                    &std::mem::transmute(*b"seventy\0")
                })
            },
            {
                let mut __tmp_59: () = 80_();
                std_pair_int__std_string_::new_2({ &mut __tmp_59 }, {
                    &std::mem::transmute(*b"eighty\0")
                })
            },
        ];
        let mut c1: () = unimplemented!();
        let mut c2: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        (unsafe {
            let mut ___x: () = {
                let mut __tmp_60: () = 90_();
                std_pair_int__std_string_::new_2({ &mut __tmp_60 }, {
                    &std::mem::transmute(*b"ninety\0")
                })
            };
            c2.insert_45_struct_std_pair_int__std_string__0(&mut ___x)
        });
        unimplemented!();
        (unsafe {
            let mut ___x: () = {
                let mut __tmp_61: () = 90_();
                std_pair_int__std_string_::new_2({ &mut __tmp_61 }, {
                    &std::mem::transmute(*b"ninety\0")
                })
            };
            c1.insert_45_struct_std_pair_int__std_string__0(&mut ___x)
        });
        unimplemented!();
    }
    {
        let mut a: [(); 8] = [
            {
                let mut __tmp_62: () = 10_();
                std_pair_int__std_string_::new_1({ &mut __tmp_62 }, {
                    &std::mem::transmute(*b"ten\0")
                })
            },
            {
                let mut __tmp_63: () = 20_();
                std_pair_int__std_string_::new_2({ &mut __tmp_63 }, {
                    &std::mem::transmute(*b"twenty\0")
                })
            },
            {
                let mut __tmp_64: () = 30_();
                std_pair_int__std_string_::new_2({ &mut __tmp_64 }, {
                    &std::mem::transmute(*b"thirty\0")
                })
            },
            {
                let mut __tmp_65: () = 40_();
                std_pair_int__std_string_::new_3({ &mut __tmp_65 }, {
                    &std::mem::transmute(*b"forty\0")
                })
            },
            {
                let mut __tmp_66: () = 50_();
                std_pair_int__std_string_::new_3({ &mut __tmp_66 }, {
                    &std::mem::transmute(*b"fifty\0")
                })
            },
            {
                let mut __tmp_67: () = 60_();
                std_pair_int__std_string_::new_3({ &mut __tmp_67 }, {
                    &std::mem::transmute(*b"sixty\0")
                })
            },
            {
                let mut __tmp_68: () = 70_();
                std_pair_int__std_string_::new_4({ &mut __tmp_68 }, {
                    &std::mem::transmute(*b"seventy\0")
                })
            },
            {
                let mut __tmp_69: () = 80_();
                std_pair_int__std_string_::new_2({ &mut __tmp_69 }, {
                    &std::mem::transmute(*b"eighty\0")
                })
            },
        ];
        let mut c1: () = unimplemented!();
        let mut c2: () = unimplemented!();
        unimplemented!();
        (unsafe {
            let mut ___x: () = {
                let mut __tmp_70: () = 90_();
                std_pair_int__std_string_::new_2({ &mut __tmp_70 }, {
                    &std::mem::transmute(*b"ninety\0")
                })
            };
            c1.insert_45_struct_std_pair_int__std_string__0(&mut ___x)
        });
        (unsafe {
            let mut ___x: () = {
                let mut __tmp_71: () = 100_();
                std_pair_int__std_string_::new_5({ &mut __tmp_71 }, {
                    &std::mem::transmute(*b"onehundred\0")
                })
            };
            c2.insert_45_struct_std_pair_int__std_string__0(&mut ___x)
        });
        unimplemented!();
    }
    return 0_();
}
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__std_string___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__;
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_0);
    std::cell::LazyCell::force(&*&raw const alloc_count_1);
    std::cell::LazyCell::force(&*&raw const dealloc_count_2);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_3);
}
