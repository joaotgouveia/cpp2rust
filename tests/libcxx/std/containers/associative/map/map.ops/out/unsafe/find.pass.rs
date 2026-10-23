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
pub struct min_pointer_std_pair_const_int__double___std_integral_constant_unsigned_long__0__ {
    ptr_: *mut (),
}
impl min_pointer_std_pair_const_int__double___std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    ptr_: (),
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_std_pair_const_int__double__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct PrivateConstructor {
    val: (),
}
impl PrivateConstructor {
    pub unsafe fn make(mut v: ()) -> PrivateConstructor {
        return PrivateConstructor::new({ v });
    }
    pub unsafe fn get(&self) -> () {
        return self.val;
    }
    unsafe fn new(mut v: ()) -> Self {
        let mut this = Self { val: v };
        this
    }
}
impl std::cmp::Ord for PrivateConstructor {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if operator_lt_4(
                self as *const PrivateConstructor,
                other as *const PrivateConstructor,
            ) {
                std::cmp::Ordering::Less
            } else if operator_lt_4(
                other as *const PrivateConstructor,
                self as *const PrivateConstructor,
            ) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for PrivateConstructor {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for PrivateConstructor {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            !(operator_lt_4(
                self as *const PrivateConstructor,
                other as *const PrivateConstructor,
            )) && !(operator_lt_4(
                other as *const PrivateConstructor,
                self as *const PrivateConstructor,
            ))
        }
    }
}
impl std::cmp::Eq for PrivateConstructor {}
pub unsafe fn operator_lt_4(lhs: *const PrivateConstructor, rhs: *const PrivateConstructor) -> () {
    return ((unsafe { PrivateConstructor::get(&(*lhs)) })
        < (unsafe { PrivateConstructor::get(&(*rhs)) }));
}
pub unsafe fn operator_lt_5(lhs: *const PrivateConstructor, mut rhs: ()) -> () {
    return ((unsafe { PrivateConstructor::get(&(*lhs)) }) < (rhs));
}
pub unsafe fn operator_lt_6(mut lhs: (), rhs: *const PrivateConstructor) -> () {
    return ((lhs) < (unsafe { PrivateConstructor::get(&(*rhs)) }));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct transparent_less {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct transparent_less_not_referenceable {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct transparent_less_nonempty_is_transparent {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct transparent_less_nonempty {
    pub pad_: transparent_less_nonempty_is_transparent,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct transparent_less_no_type {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct transparent_less_private {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct transparent_less_not_a_type {
    pub is_transparent: (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct C2Int {
    i_: (),
}
impl C2Int {
    pub unsafe fn new_1() -> Self {
        let mut this = Self { i_: 0_() };
        this
    }
    pub unsafe fn new_2(mut i: ()) -> Self {
        let mut this = Self { i_: i };
        this
    }
    pub unsafe fn get(&self) -> () {
        return self.i_;
    }
}
impl std::cmp::Ord for C2Int {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if operator_lt_7(self as *const C2Int, other as *const C2Int) {
                std::cmp::Ordering::Less
            } else if operator_lt_7(other as *const C2Int, self as *const C2Int) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for C2Int {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for C2Int {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            !(operator_lt_7(self as *const C2Int, other as *const C2Int))
                && !(operator_lt_7(other as *const C2Int, self as *const C2Int))
        }
    }
}
impl std::cmp::Eq for C2Int {}
impl Default for C2Int {
    fn default() -> Self {
        unsafe { C2Int::new_1() }
    }
}
pub unsafe fn operator_lt_8(mut rhs: (), lhs: *const C2Int) -> () {
    return ((rhs) < (unsafe { C2Int::get(&(*lhs)) }));
}
pub unsafe fn operator_lt_7(rhs: *const C2Int, lhs: *const C2Int) -> () {
    return ((unsafe { C2Int::get(&(*rhs)) }) < (unsafe { C2Int::get(&(*lhs)) }));
}
pub unsafe fn operator_lt_9(rhs: *const C2Int, mut lhs: ()) -> () {
    return ((unsafe { C2Int::get(&(*rhs)) }) < (lhs));
}
pub unsafe fn test_10() -> () {
    {
        {
            let mut ar: [(); 8] = [
                {
                    let mut __tmp_0: () = 5_();
                    let mut __tmp_1: () = 5_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_0 }, { &mut __tmp_1 })
                },
                {
                    let mut __tmp_2: () = 6_();
                    let mut __tmp_3: () = 6_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_2 }, { &mut __tmp_3 })
                },
                {
                    let mut __tmp_4: () = 7_();
                    let mut __tmp_5: () = 7_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_4 }, { &mut __tmp_5 })
                },
                {
                    let mut __tmp_6: () = 8_();
                    let mut __tmp_7: () = 8_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_6 }, { &mut __tmp_7 })
                },
                {
                    let mut __tmp_8: () = 9_();
                    let mut __tmp_9: () = 9_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_8 }, { &mut __tmp_9 })
                },
                {
                    let mut __tmp_10: () = 10_();
                    let mut __tmp_11: () = 10_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_10 }, { &mut __tmp_11 })
                },
                {
                    let mut __tmp_12: () = 11_();
                    let mut __tmp_13: () = 11_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_12 }, { &mut __tmp_13 })
                },
                {
                    let mut __tmp_14: () = 12_();
                    let mut __tmp_15: () = 12_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_14 }, { &mut __tmp_15 })
                },
            ];
            let mut m: () = unimplemented!();
            let mut r: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut ar: [(); 8] = [
                {
                    let mut __tmp_16: () = 5_();
                    let mut __tmp_17: () = 5_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_16 }, { &mut __tmp_17 })
                },
                {
                    let mut __tmp_18: () = 6_();
                    let mut __tmp_19: () = 6_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_18 }, { &mut __tmp_19 })
                },
                {
                    let mut __tmp_20: () = 7_();
                    let mut __tmp_21: () = 7_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_20 }, { &mut __tmp_21 })
                },
                {
                    let mut __tmp_22: () = 8_();
                    let mut __tmp_23: () = 8_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_22 }, { &mut __tmp_23 })
                },
                {
                    let mut __tmp_24: () = 9_();
                    let mut __tmp_25: () = 9_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_24 }, { &mut __tmp_25 })
                },
                {
                    let mut __tmp_26: () = 10_();
                    let mut __tmp_27: () = 10_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_26 }, { &mut __tmp_27 })
                },
                {
                    let mut __tmp_28: () = 11_();
                    let mut __tmp_29: () = 11_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_28 }, { &mut __tmp_29 })
                },
                {
                    let mut __tmp_30: () = 12_();
                    let mut __tmp_31: () = 12_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_30 }, { &mut __tmp_31 })
                },
            ];
            let m: () = unimplemented!();
            let mut r: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let mut ar: [(); 8] = [
            {
                let mut __tmp_32: () = 5_();
                let mut __tmp_33: () = 5_();
                std_pair_const_int__int_::new_1({ &mut __tmp_32 }, { &mut __tmp_33 })
            },
            {
                let mut __tmp_34: () = 6_();
                let mut __tmp_35: () = 6_();
                std_pair_const_int__int_::new_1({ &mut __tmp_34 }, { &mut __tmp_35 })
            },
            {
                let mut __tmp_36: () = 7_();
                let mut __tmp_37: () = 7_();
                std_pair_const_int__int_::new_1({ &mut __tmp_36 }, { &mut __tmp_37 })
            },
            {
                let mut __tmp_38: () = 8_();
                let mut __tmp_39: () = 8_();
                std_pair_const_int__int_::new_1({ &mut __tmp_38 }, { &mut __tmp_39 })
            },
            {
                let mut __tmp_40: () = 9_();
                let mut __tmp_41: () = 9_();
                std_pair_const_int__int_::new_1({ &mut __tmp_40 }, { &mut __tmp_41 })
            },
            {
                let mut __tmp_42: () = 10_();
                let mut __tmp_43: () = 10_();
                std_pair_const_int__int_::new_1({ &mut __tmp_42 }, { &mut __tmp_43 })
            },
            {
                let mut __tmp_44: () = 11_();
                let mut __tmp_45: () = 11_();
                std_pair_const_int__int_::new_1({ &mut __tmp_44 }, { &mut __tmp_45 })
            },
            {
                let mut __tmp_46: () = 12_();
                let mut __tmp_47: () = 12_();
                std_pair_const_int__int_::new_1({ &mut __tmp_46 }, { &mut __tmp_47 })
            },
        ];
        let mut m: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        {
            let mut ar: [(); 8] = [
                {
                    let mut __tmp_48: () = 5_();
                    let mut __tmp_49: () = 5_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_48 }, { &mut __tmp_49 })
                },
                {
                    let mut __tmp_50: () = 6_();
                    let mut __tmp_51: () = 6_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_50 }, { &mut __tmp_51 })
                },
                {
                    let mut __tmp_52: () = 7_();
                    let mut __tmp_53: () = 7_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_52 }, { &mut __tmp_53 })
                },
                {
                    let mut __tmp_54: () = 8_();
                    let mut __tmp_55: () = 8_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_54 }, { &mut __tmp_55 })
                },
                {
                    let mut __tmp_56: () = 9_();
                    let mut __tmp_57: () = 9_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_56 }, { &mut __tmp_57 })
                },
                {
                    let mut __tmp_58: () = 10_();
                    let mut __tmp_59: () = 10_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_58 }, { &mut __tmp_59 })
                },
                {
                    let mut __tmp_60: () = 11_();
                    let mut __tmp_61: () = 11_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_60 }, { &mut __tmp_61 })
                },
                {
                    let mut __tmp_62: () = 12_();
                    let mut __tmp_63: () = 12_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_62 }, { &mut __tmp_63 })
                },
            ];
            &(ar[(0_()) as usize].second);
            let mut m: () = unimplemented!();
            let mut r: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut ar: [(); 8] = [
                {
                    let mut __tmp_64: () = 5_();
                    let mut __tmp_65: () = 5_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_64 }, { &mut __tmp_65 })
                },
                {
                    let mut __tmp_66: () = 6_();
                    let mut __tmp_67: () = 6_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_66 }, { &mut __tmp_67 })
                },
                {
                    let mut __tmp_68: () = 7_();
                    let mut __tmp_69: () = 7_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_68 }, { &mut __tmp_69 })
                },
                {
                    let mut __tmp_70: () = 8_();
                    let mut __tmp_71: () = 8_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_70 }, { &mut __tmp_71 })
                },
                {
                    let mut __tmp_72: () = 9_();
                    let mut __tmp_73: () = 9_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_72 }, { &mut __tmp_73 })
                },
                {
                    let mut __tmp_74: () = 10_();
                    let mut __tmp_75: () = 10_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_74 }, { &mut __tmp_75 })
                },
                {
                    let mut __tmp_76: () = 11_();
                    let mut __tmp_77: () = 11_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_76 }, { &mut __tmp_77 })
                },
                {
                    let mut __tmp_78: () = 12_();
                    let mut __tmp_79: () = 12_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_78 }, { &mut __tmp_79 })
                },
            ];
            let m: () = unimplemented!();
            let mut r: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let mut ar: [(); 8] = [
            {
                let mut __tmp_80: () = 5_();
                let mut __tmp_81: () = 5_();
                std_pair_const_int__double_::new_1({ &mut __tmp_80 }, { &mut __tmp_81 })
            },
            {
                let mut __tmp_82: () = 6_();
                let mut __tmp_83: () = 6_();
                std_pair_const_int__double_::new_1({ &mut __tmp_82 }, { &mut __tmp_83 })
            },
            {
                let mut __tmp_84: () = 7_();
                let mut __tmp_85: () = 7_();
                std_pair_const_int__double_::new_1({ &mut __tmp_84 }, { &mut __tmp_85 })
            },
            {
                let mut __tmp_86: () = 8_();
                let mut __tmp_87: () = 8_();
                std_pair_const_int__double_::new_1({ &mut __tmp_86 }, { &mut __tmp_87 })
            },
            {
                let mut __tmp_88: () = 9_();
                let mut __tmp_89: () = 9_();
                std_pair_const_int__double_::new_1({ &mut __tmp_88 }, { &mut __tmp_89 })
            },
            {
                let mut __tmp_90: () = 10_();
                let mut __tmp_91: () = 10_();
                std_pair_const_int__double_::new_1({ &mut __tmp_90 }, { &mut __tmp_91 })
            },
            {
                let mut __tmp_92: () = 11_();
                let mut __tmp_93: () = 11_();
                std_pair_const_int__double_::new_1({ &mut __tmp_92 }, { &mut __tmp_93 })
            },
            {
                let mut __tmp_94: () = 12_();
                let mut __tmp_95: () = 12_();
                std_pair_const_int__double_::new_1({ &mut __tmp_94 }, { &mut __tmp_95 })
            },
        ];
        let mut m: () = unimplemented!();
        let mut r: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut m: () = unimplemented!();
        (*unimplemented!()) = 5_();
        (*unimplemented!()) = 6_();
        (*unimplemented!()) = 7_();
        (*unimplemented!()) = 8_();
        (*unimplemented!()) = 9_();
        (*unimplemented!()) = 10_();
        (*unimplemented!()) = 11_();
        (*unimplemented!()) = 12_();
        let mut r: () = (unsafe {
            let mut ___k: () = 5_();
            m.find_80_int_struct_std_less_void__0(&mut ___k)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
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
    (unsafe { test_10() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_0);
    std::cell::LazyCell::force(&*&raw const alloc_count_1);
    std::cell::LazyCell::force(&*&raw const dealloc_count_2);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_3);
}
