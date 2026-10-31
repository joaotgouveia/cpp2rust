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
    pub unsafe fn outstanding_alloc() -> usize {
        assert!(
            ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_1))
                >= (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_2)))
        );
        return ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_1))
            .wrapping_sub((*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_2))));
    }
    pub unsafe fn reset() {
        assert!(((unsafe { malloc_allocator_base::outstanding_alloc() }) == (0_usize)));
        (*std::cell::LazyCell::force_mut(&mut *&raw mut disable_default_constructor_3)) = false;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut outstanding_bytes_0)) = 0_usize;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_1)) = 0_usize;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_2)) = 0_usize;
    }
}
pub static mut outstanding_bytes_0: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut alloc_count_1: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut dealloc_count_2: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut disable_default_constructor_3: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { false });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____std_integral_constant_unsigned_long__0__
{
    ptr_: *mut (i32, Vec<libc::c_char>),
}
impl min_pointer_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____std_integral_constant_unsigned_long__0__ { }
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut ::libc::c_void,
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_int__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut i32,
}
impl min_pointer_int__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__
{
    ptr_: *mut Vec<libc::c_char>,
}
impl min_pointer_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__ { }
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char____
{}
pub unsafe fn testEquality_4(t1: *const (), t2: *const (), mut isEqual: bool) -> bool {
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
pub unsafe fn testEquality_5(t1: *const (), t2: *const (), mut isEqual: bool) -> bool {
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
pub unsafe fn testEquality_6(t1: *const (), t2: *const (), mut isEqual: bool) -> bool {
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
pub unsafe fn operator_lt_7(lhs: *const LessAndEqComp, rhs: *const LessAndEqComp) -> bool {
    return (((*lhs).value) < ((*rhs).value));
}
pub unsafe fn operator_eq_8(lhs: *const LessAndEqComp, rhs: *const LessAndEqComp) -> bool {
    return (((*lhs).value) == ((*rhs).value));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct LessAndEqComp {
    pub value: i32,
}
impl LessAndEqComp {
    pub unsafe fn new(mut v: i32) -> Self {
        let mut this = Self { value: v };
        this
    }
}
impl std::cmp::Ord for LessAndEqComp {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if operator_lt_7(self as *const LessAndEqComp, other as *const LessAndEqComp) {
                std::cmp::Ordering::Less
            } else if operator_lt_7(other as *const LessAndEqComp, self as *const LessAndEqComp) {
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
        unsafe { operator_eq_8(self as *const LessAndEqComp, other as *const LessAndEqComp) }
    }
}
impl std::cmp::Eq for LessAndEqComp {}
pub unsafe fn operator_cmp_9(mut _a0: StrongOrder, mut _a1: StrongOrder) -> std::cmp::Ordering {
    {
        let mut cmp: std::cmp::Ordering = std::cmp::Ord::cmp(&(_a0.value), &(_a1.value));
        if !(cmp == std::cmp::Ordering::Equal) {
            return cmp;
        }
    }
    return std::cmp::Ordering::Equal;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct StrongOrder {
    pub value: i32,
}
impl StrongOrder {
    pub unsafe fn new(mut v: i32) -> Self {
        let mut this = Self { value: v };
        this
    }
}
impl std::cmp::Ord for StrongOrder {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { operator_cmp_9(self.clone(), other.clone()) }
    }
}
impl std::cmp::PartialOrd for StrongOrder {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for StrongOrder {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_cmp_9(self.clone(), other.clone()) == std::cmp::Ordering::Equal }
    }
}
impl std::cmp::Eq for StrongOrder {}
pub unsafe fn operator_cmp_10(mut _a0: WeakOrder, mut _a1: WeakOrder) -> () {
    {
        let mut cmp: () = unimplemented!();
        if !(unimplemented!()) {
            return unimplemented!();
        }
    }
    return unimplemented!();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct WeakOrder {
    pub value: i32,
}
impl WeakOrder {
    pub unsafe fn new(mut v: i32) -> Self {
        let mut this = Self { value: v };
        this
    }
}
impl std::cmp::Ord for WeakOrder {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { operator_cmp_10(self.clone(), other.clone()) }
    }
}
impl std::cmp::PartialOrd for WeakOrder {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for WeakOrder {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_cmp_10(self.clone(), other.clone()) == std::cmp::Ordering::Equal }
    }
}
impl std::cmp::Eq for WeakOrder {}
pub unsafe fn operator_cmp_11(mut lhs: PartialOrder, mut rhs: PartialOrder) -> () {
    if ((lhs.value) == (i32::MIN)) || ((rhs.value) == (i32::MIN)) {
        return unimplemented!();
    }
    if ((lhs.value) == (i32::MAX)) || ((rhs.value) == (i32::MAX)) {
        return unimplemented!();
    }
    return unimplemented!();
}
pub unsafe fn operator_eq_12(mut lhs: PartialOrder, mut rhs: PartialOrder) -> bool {
    return unimplemented!();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct PartialOrder {
    pub value: i32,
}
impl PartialOrder {
    pub unsafe fn new(mut v: i32) -> Self {
        let mut this = Self { value: v };
        this
    }
}
impl std::cmp::Ord for PartialOrder {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { operator_cmp_11(self.clone(), other.clone()) }
    }
}
impl std::cmp::PartialOrd for PartialOrder {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for PartialOrder {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_12(self.clone(), other.clone()) }
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
pub unsafe fn operator_eq_13(
    _a0: *const EqualityComparable,
    _a1: *const EqualityComparable,
) -> bool {
    return (((*_a0).value_) == ((*_a1).value_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct EqualityComparable {
    value_: i32,
}
impl EqualityComparable {
    pub unsafe fn new(mut value: i32) -> Self {
        let mut this = Self { value_: value };
        this
    }
}
impl std::cmp::PartialEq for EqualityComparable {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_13(
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
pub unsafe fn operator_eq_14(
    _a0: *const ThreeWayComparable,
    _a1: *const ThreeWayComparable,
) -> bool {
    return (((*_a0).value_) == ((*_a1).value_));
}
pub unsafe fn operator_cmp_15(
    _a0: *const ThreeWayComparable,
    _a1: *const ThreeWayComparable,
) -> std::cmp::Ordering {
    {
        let mut cmp: std::cmp::Ordering = std::cmp::Ord::cmp(&((*_a0).value_), &((*_a1).value_));
        if !(cmp == std::cmp::Ordering::Equal) {
            return cmp;
        }
    }
    return std::cmp::Ordering::Equal;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct ThreeWayComparable {
    value_: i32,
}
impl ThreeWayComparable {
    pub unsafe fn new(mut value: i32) -> Self {
        let mut this = Self { value_: value };
        this
    }
}
impl std::cmp::Ord for ThreeWayComparable {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            operator_cmp_15(
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
            operator_eq_14(
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
    pub x: i32,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ImplicitBool_E2 {
    pub y: i32,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_into(bool, to_bool_1)]
pub struct ImplicitBool {
    pub val: bool,
}
impl ImplicitBool {
    pub unsafe fn to_bool_1(&self) -> bool {
        return self.val;
    }
}
pub unsafe fn operator_eq_16(mut lhs: ImplicitBool_E1, mut rhs: ImplicitBool_E2) -> ImplicitBool {
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
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    {
        let mut a: [(i32, Vec<libc::c_char>); 11] = [
            (
                10,
                std::slice::from_raw_parts(
                    c"ten".as_ptr(),
                    libcc2rs::c_str_len(c"ten".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                20,
                std::slice::from_raw_parts(
                    c"twenty".as_ptr(),
                    libcc2rs::c_str_len(c"twenty".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                20,
                std::slice::from_raw_parts(
                    c"twenty 2".as_ptr(),
                    libcc2rs::c_str_len(c"twenty 2".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                30,
                std::slice::from_raw_parts(
                    c"thirty".as_ptr(),
                    libcc2rs::c_str_len(c"thirty".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                40,
                std::slice::from_raw_parts(
                    c"forty".as_ptr(),
                    libcc2rs::c_str_len(c"forty".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                50,
                std::slice::from_raw_parts(
                    c"fifty".as_ptr(),
                    libcc2rs::c_str_len(c"fifty".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                50,
                std::slice::from_raw_parts(
                    c"fifty 2".as_ptr(),
                    libcc2rs::c_str_len(c"fifty 2".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                50,
                std::slice::from_raw_parts(
                    c"fifty 3".as_ptr(),
                    libcc2rs::c_str_len(c"fifty 3".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                60,
                std::slice::from_raw_parts(
                    c"sixty".as_ptr(),
                    libcc2rs::c_str_len(c"sixty".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                70,
                std::slice::from_raw_parts(
                    c"seventy".as_ptr(),
                    libcc2rs::c_str_len(c"seventy".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                80,
                std::slice::from_raw_parts(
                    c"eighty".as_ptr(),
                    libcc2rs::c_str_len(c"eighty".as_ptr()) + 1,
                )
                .to_vec(),
            ),
        ];
        let c1: () = unimplemented!();
        let c2: () = unimplemented!();
        assert!((unsafe { testEquality_4(&c1, &c2, false,) }));
    }
    {
        let mut a: [(i32, Vec<libc::c_char>); 11] = [
            (
                10,
                std::slice::from_raw_parts(
                    c"ten".as_ptr(),
                    libcc2rs::c_str_len(c"ten".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                20,
                std::slice::from_raw_parts(
                    c"twenty".as_ptr(),
                    libcc2rs::c_str_len(c"twenty".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                20,
                std::slice::from_raw_parts(
                    c"twenty 2".as_ptr(),
                    libcc2rs::c_str_len(c"twenty 2".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                30,
                std::slice::from_raw_parts(
                    c"thirty".as_ptr(),
                    libcc2rs::c_str_len(c"thirty".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                40,
                std::slice::from_raw_parts(
                    c"forty".as_ptr(),
                    libcc2rs::c_str_len(c"forty".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                50,
                std::slice::from_raw_parts(
                    c"fifty".as_ptr(),
                    libcc2rs::c_str_len(c"fifty".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                50,
                std::slice::from_raw_parts(
                    c"fifty 2".as_ptr(),
                    libcc2rs::c_str_len(c"fifty 2".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                50,
                std::slice::from_raw_parts(
                    c"fifty 3".as_ptr(),
                    libcc2rs::c_str_len(c"fifty 3".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                60,
                std::slice::from_raw_parts(
                    c"sixty".as_ptr(),
                    libcc2rs::c_str_len(c"sixty".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                70,
                std::slice::from_raw_parts(
                    c"seventy".as_ptr(),
                    libcc2rs::c_str_len(c"seventy".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                80,
                std::slice::from_raw_parts(
                    c"eighty".as_ptr(),
                    libcc2rs::c_str_len(c"eighty".as_ptr()) + 1,
                )
                .to_vec(),
            ),
        ];
        let c1: () = unimplemented!();
        let c2: () = unimplemented!();
        assert!((unsafe { testEquality_4(&c1, &c2, true,) }));
    }
    {
        let mut a: [(i32, Vec<libc::c_char>); 11] = [
            (
                10,
                std::slice::from_raw_parts(
                    c"ten".as_ptr(),
                    libcc2rs::c_str_len(c"ten".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                20,
                std::slice::from_raw_parts(
                    c"twenty".as_ptr(),
                    libcc2rs::c_str_len(c"twenty".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                20,
                std::slice::from_raw_parts(
                    c"twenty 2".as_ptr(),
                    libcc2rs::c_str_len(c"twenty 2".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                30,
                std::slice::from_raw_parts(
                    c"thirty".as_ptr(),
                    libcc2rs::c_str_len(c"thirty".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                40,
                std::slice::from_raw_parts(
                    c"forty".as_ptr(),
                    libcc2rs::c_str_len(c"forty".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                50,
                std::slice::from_raw_parts(
                    c"fifty".as_ptr(),
                    libcc2rs::c_str_len(c"fifty".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                50,
                std::slice::from_raw_parts(
                    c"fifty 2".as_ptr(),
                    libcc2rs::c_str_len(c"fifty 2".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                50,
                std::slice::from_raw_parts(
                    c"fifty 3".as_ptr(),
                    libcc2rs::c_str_len(c"fifty 3".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                60,
                std::slice::from_raw_parts(
                    c"sixty".as_ptr(),
                    libcc2rs::c_str_len(c"sixty".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                70,
                std::slice::from_raw_parts(
                    c"seventy".as_ptr(),
                    libcc2rs::c_str_len(c"seventy".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                80,
                std::slice::from_raw_parts(
                    c"eighty".as_ptr(),
                    libcc2rs::c_str_len(c"eighty".as_ptr()) + 1,
                )
                .to_vec(),
            ),
        ];
        let mut c1: () = unimplemented!();
        let mut c2: () = unimplemented!();
        unimplemented!();
        assert!((unsafe { testEquality_4(&c1, &c2, true,) }));
        unimplemented!();
        assert!((unsafe { testEquality_4(&c1, &c2, false,) }));
        unimplemented!();
        assert!((unsafe { testEquality_4(&c1, &c2, true,) }));
    }
    {
        let mut a: [(i32, Vec<libc::c_char>); 11] = [
            (
                10,
                std::slice::from_raw_parts(
                    c"ten".as_ptr(),
                    libcc2rs::c_str_len(c"ten".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                20,
                std::slice::from_raw_parts(
                    c"twenty".as_ptr(),
                    libcc2rs::c_str_len(c"twenty".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                20,
                std::slice::from_raw_parts(
                    c"twenty 2".as_ptr(),
                    libcc2rs::c_str_len(c"twenty 2".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                30,
                std::slice::from_raw_parts(
                    c"thirty".as_ptr(),
                    libcc2rs::c_str_len(c"thirty".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                40,
                std::slice::from_raw_parts(
                    c"forty".as_ptr(),
                    libcc2rs::c_str_len(c"forty".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                50,
                std::slice::from_raw_parts(
                    c"fifty".as_ptr(),
                    libcc2rs::c_str_len(c"fifty".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                50,
                std::slice::from_raw_parts(
                    c"fifty 2".as_ptr(),
                    libcc2rs::c_str_len(c"fifty 2".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                50,
                std::slice::from_raw_parts(
                    c"fifty 3".as_ptr(),
                    libcc2rs::c_str_len(c"fifty 3".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                60,
                std::slice::from_raw_parts(
                    c"sixty".as_ptr(),
                    libcc2rs::c_str_len(c"sixty".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                70,
                std::slice::from_raw_parts(
                    c"seventy".as_ptr(),
                    libcc2rs::c_str_len(c"seventy".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                80,
                std::slice::from_raw_parts(
                    c"eighty".as_ptr(),
                    libcc2rs::c_str_len(c"eighty".as_ptr()) + 1,
                )
                .to_vec(),
            ),
        ];
        let mut c1: () = unimplemented!();
        let mut c2: () = unimplemented!();
        assert!((unsafe { testEquality_4(&c1, &c2, true,) }));
        unimplemented!();
        unimplemented!();
        assert!((unsafe { testEquality_4(&c1, &c2, false,) }));
    }
    {
        let mut a: [(i32, Vec<libc::c_char>); 11] = [
            (
                10,
                std::slice::from_raw_parts(
                    c"ten".as_ptr(),
                    libcc2rs::c_str_len(c"ten".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                20,
                std::slice::from_raw_parts(
                    c"twenty".as_ptr(),
                    libcc2rs::c_str_len(c"twenty".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                20,
                std::slice::from_raw_parts(
                    c"twenty 2".as_ptr(),
                    libcc2rs::c_str_len(c"twenty 2".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                30,
                std::slice::from_raw_parts(
                    c"thirty".as_ptr(),
                    libcc2rs::c_str_len(c"thirty".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                40,
                std::slice::from_raw_parts(
                    c"forty".as_ptr(),
                    libcc2rs::c_str_len(c"forty".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                50,
                std::slice::from_raw_parts(
                    c"fifty".as_ptr(),
                    libcc2rs::c_str_len(c"fifty".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                50,
                std::slice::from_raw_parts(
                    c"fifty 2".as_ptr(),
                    libcc2rs::c_str_len(c"fifty 2".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                50,
                std::slice::from_raw_parts(
                    c"fifty 3".as_ptr(),
                    libcc2rs::c_str_len(c"fifty 3".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                60,
                std::slice::from_raw_parts(
                    c"sixty".as_ptr(),
                    libcc2rs::c_str_len(c"sixty".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                70,
                std::slice::from_raw_parts(
                    c"seventy".as_ptr(),
                    libcc2rs::c_str_len(c"seventy".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                80,
                std::slice::from_raw_parts(
                    c"eighty".as_ptr(),
                    libcc2rs::c_str_len(c"eighty".as_ptr()) + 1,
                )
                .to_vec(),
            ),
        ];
        let c1: () = unimplemented!();
        let c2: () = unimplemented!();
        assert!((unsafe { testEquality_5(&c1, &c2, false,) }));
    }
    {
        let mut a: [(i32, Vec<libc::c_char>); 11] = [
            (
                10,
                std::slice::from_raw_parts(
                    c"ten".as_ptr(),
                    libcc2rs::c_str_len(c"ten".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                20,
                std::slice::from_raw_parts(
                    c"twenty".as_ptr(),
                    libcc2rs::c_str_len(c"twenty".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                20,
                std::slice::from_raw_parts(
                    c"twenty 2".as_ptr(),
                    libcc2rs::c_str_len(c"twenty 2".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                30,
                std::slice::from_raw_parts(
                    c"thirty".as_ptr(),
                    libcc2rs::c_str_len(c"thirty".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                40,
                std::slice::from_raw_parts(
                    c"forty".as_ptr(),
                    libcc2rs::c_str_len(c"forty".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                50,
                std::slice::from_raw_parts(
                    c"fifty".as_ptr(),
                    libcc2rs::c_str_len(c"fifty".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                50,
                std::slice::from_raw_parts(
                    c"fifty 2".as_ptr(),
                    libcc2rs::c_str_len(c"fifty 2".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                50,
                std::slice::from_raw_parts(
                    c"fifty 3".as_ptr(),
                    libcc2rs::c_str_len(c"fifty 3".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                60,
                std::slice::from_raw_parts(
                    c"sixty".as_ptr(),
                    libcc2rs::c_str_len(c"sixty".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                70,
                std::slice::from_raw_parts(
                    c"seventy".as_ptr(),
                    libcc2rs::c_str_len(c"seventy".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                80,
                std::slice::from_raw_parts(
                    c"eighty".as_ptr(),
                    libcc2rs::c_str_len(c"eighty".as_ptr()) + 1,
                )
                .to_vec(),
            ),
        ];
        let c1: () = unimplemented!();
        let c2: () = unimplemented!();
        assert!((unsafe { testEquality_5(&c1, &c2, true,) }));
    }
    {
        let mut a: [(i32, Vec<libc::c_char>); 11] = [
            (
                10,
                std::slice::from_raw_parts(
                    c"ten".as_ptr(),
                    libcc2rs::c_str_len(c"ten".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                20,
                std::slice::from_raw_parts(
                    c"twenty".as_ptr(),
                    libcc2rs::c_str_len(c"twenty".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                20,
                std::slice::from_raw_parts(
                    c"twenty 2".as_ptr(),
                    libcc2rs::c_str_len(c"twenty 2".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                30,
                std::slice::from_raw_parts(
                    c"thirty".as_ptr(),
                    libcc2rs::c_str_len(c"thirty".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                40,
                std::slice::from_raw_parts(
                    c"forty".as_ptr(),
                    libcc2rs::c_str_len(c"forty".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                50,
                std::slice::from_raw_parts(
                    c"fifty".as_ptr(),
                    libcc2rs::c_str_len(c"fifty".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                50,
                std::slice::from_raw_parts(
                    c"fifty 2".as_ptr(),
                    libcc2rs::c_str_len(c"fifty 2".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                50,
                std::slice::from_raw_parts(
                    c"fifty 3".as_ptr(),
                    libcc2rs::c_str_len(c"fifty 3".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                60,
                std::slice::from_raw_parts(
                    c"sixty".as_ptr(),
                    libcc2rs::c_str_len(c"sixty".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                70,
                std::slice::from_raw_parts(
                    c"seventy".as_ptr(),
                    libcc2rs::c_str_len(c"seventy".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                80,
                std::slice::from_raw_parts(
                    c"eighty".as_ptr(),
                    libcc2rs::c_str_len(c"eighty".as_ptr()) + 1,
                )
                .to_vec(),
            ),
        ];
        let mut c1: () = unimplemented!();
        let mut c2: () = unimplemented!();
        unimplemented!();
        assert!((unsafe { testEquality_5(&c1, &c2, true,) }));
        unimplemented!();
        assert!((unsafe { testEquality_5(&c1, &c2, false,) }));
        unimplemented!();
        assert!((unsafe { testEquality_5(&c1, &c2, true,) }));
    }
    {
        let mut a: [(i32, Vec<libc::c_char>); 11] = [
            (
                10,
                std::slice::from_raw_parts(
                    c"ten".as_ptr(),
                    libcc2rs::c_str_len(c"ten".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                20,
                std::slice::from_raw_parts(
                    c"twenty".as_ptr(),
                    libcc2rs::c_str_len(c"twenty".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                20,
                std::slice::from_raw_parts(
                    c"twenty 2".as_ptr(),
                    libcc2rs::c_str_len(c"twenty 2".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                30,
                std::slice::from_raw_parts(
                    c"thirty".as_ptr(),
                    libcc2rs::c_str_len(c"thirty".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                40,
                std::slice::from_raw_parts(
                    c"forty".as_ptr(),
                    libcc2rs::c_str_len(c"forty".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                50,
                std::slice::from_raw_parts(
                    c"fifty".as_ptr(),
                    libcc2rs::c_str_len(c"fifty".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                50,
                std::slice::from_raw_parts(
                    c"fifty 2".as_ptr(),
                    libcc2rs::c_str_len(c"fifty 2".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                50,
                std::slice::from_raw_parts(
                    c"fifty 3".as_ptr(),
                    libcc2rs::c_str_len(c"fifty 3".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                60,
                std::slice::from_raw_parts(
                    c"sixty".as_ptr(),
                    libcc2rs::c_str_len(c"sixty".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                70,
                std::slice::from_raw_parts(
                    c"seventy".as_ptr(),
                    libcc2rs::c_str_len(c"seventy".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                80,
                std::slice::from_raw_parts(
                    c"eighty".as_ptr(),
                    libcc2rs::c_str_len(c"eighty".as_ptr()) + 1,
                )
                .to_vec(),
            ),
        ];
        let mut c1: () = unimplemented!();
        let mut c2: () = unimplemented!();
        assert!((unsafe { testEquality_5(&c1, &c2, true,) }));
        unimplemented!();
        unimplemented!();
        assert!((unsafe { testEquality_5(&c1, &c2, false,) }));
    }
    {
        let mut a: [(i32, libc::c_char); 4] = [
            (1, ('a' as libc::c_char)),
            (1, ('b' as libc::c_char)),
            (1, ('d' as libc::c_char)),
            (2, ('b' as libc::c_char)),
        ];
        let mut b: [(i32, libc::c_char); 5] = [
            (1, ('a' as libc::c_char)),
            (1, ('b' as libc::c_char)),
            (1, ('b' as libc::c_char)),
            (1, ('d' as libc::c_char)),
            (2, ('b' as libc::c_char)),
        ];
        let mut c1: () = unimplemented!();
        let mut c2: () = unimplemented!();
        assert!((unsafe { testEquality_6(&c1, &c2, false,) }));
    }
    {
        let mut a: [(i32, libc::c_char); 4] = [
            (1, ('a' as libc::c_char)),
            (1, ('b' as libc::c_char)),
            (1, ('d' as libc::c_char)),
            (2, ('b' as libc::c_char)),
        ];
        let mut b: [(i32, libc::c_char); 4] = [
            (1, ('a' as libc::c_char)),
            (1, ('b' as libc::c_char)),
            (1, ('E' as libc::c_char)),
            (2, ('b' as libc::c_char)),
        ];
        let mut c1: () = unimplemented!();
        let mut c2: () = unimplemented!();
        assert!((unsafe { testEquality_6(&c1, &c2, false,) }));
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_0);
    std::cell::LazyCell::force(&*&raw const alloc_count_1);
    std::cell::LazyCell::force(&*&raw const dealloc_count_2);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_3);
}
