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
            let mut ar: [(); 9] = [
                {
                    let mut __tmp_0: () = 5_();
                    let mut __tmp_1: () = 1_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_0 }, { &mut __tmp_1 })
                },
                {
                    let mut __tmp_2: () = 5_();
                    let mut __tmp_3: () = 2_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_2 }, { &mut __tmp_3 })
                },
                {
                    let mut __tmp_4: () = 5_();
                    let mut __tmp_5: () = 3_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_4 }, { &mut __tmp_5 })
                },
                {
                    let mut __tmp_6: () = 7_();
                    let mut __tmp_7: () = 1_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_6 }, { &mut __tmp_7 })
                },
                {
                    let mut __tmp_8: () = 7_();
                    let mut __tmp_9: () = 2_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_8 }, { &mut __tmp_9 })
                },
                {
                    let mut __tmp_10: () = 7_();
                    let mut __tmp_11: () = 3_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_10 }, { &mut __tmp_11 })
                },
                {
                    let mut __tmp_12: () = 9_();
                    let mut __tmp_13: () = 1_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_12 }, { &mut __tmp_13 })
                },
                {
                    let mut __tmp_14: () = 9_();
                    let mut __tmp_15: () = 2_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_14 }, { &mut __tmp_15 })
                },
                {
                    let mut __tmp_16: () = 9_();
                    let mut __tmp_17: () = 3_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_16 }, { &mut __tmp_17 })
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
        }
        {
            let mut ar: [(); 9] = [
                {
                    let mut __tmp_18: () = 5_();
                    let mut __tmp_19: () = 1_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_18 }, { &mut __tmp_19 })
                },
                {
                    let mut __tmp_20: () = 5_();
                    let mut __tmp_21: () = 2_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_20 }, { &mut __tmp_21 })
                },
                {
                    let mut __tmp_22: () = 5_();
                    let mut __tmp_23: () = 3_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_22 }, { &mut __tmp_23 })
                },
                {
                    let mut __tmp_24: () = 7_();
                    let mut __tmp_25: () = 1_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_24 }, { &mut __tmp_25 })
                },
                {
                    let mut __tmp_26: () = 7_();
                    let mut __tmp_27: () = 2_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_26 }, { &mut __tmp_27 })
                },
                {
                    let mut __tmp_28: () = 7_();
                    let mut __tmp_29: () = 3_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_28 }, { &mut __tmp_29 })
                },
                {
                    let mut __tmp_30: () = 9_();
                    let mut __tmp_31: () = 1_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_30 }, { &mut __tmp_31 })
                },
                {
                    let mut __tmp_32: () = 9_();
                    let mut __tmp_33: () = 2_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_32 }, { &mut __tmp_33 })
                },
                {
                    let mut __tmp_34: () = 9_();
                    let mut __tmp_35: () = 3_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_34 }, { &mut __tmp_35 })
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
        }
    }
    {
        {
            let mut ar: [(); 9] = [
                {
                    let mut __tmp_36: () = 5_();
                    let mut __tmp_37: () = 1_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_36 }, { &mut __tmp_37 })
                },
                {
                    let mut __tmp_38: () = 5_();
                    let mut __tmp_39: () = 2_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_38 }, { &mut __tmp_39 })
                },
                {
                    let mut __tmp_40: () = 5_();
                    let mut __tmp_41: () = 3_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_40 }, { &mut __tmp_41 })
                },
                {
                    let mut __tmp_42: () = 7_();
                    let mut __tmp_43: () = 1_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_42 }, { &mut __tmp_43 })
                },
                {
                    let mut __tmp_44: () = 7_();
                    let mut __tmp_45: () = 2_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_44 }, { &mut __tmp_45 })
                },
                {
                    let mut __tmp_46: () = 7_();
                    let mut __tmp_47: () = 3_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_46 }, { &mut __tmp_47 })
                },
                {
                    let mut __tmp_48: () = 9_();
                    let mut __tmp_49: () = 1_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_48 }, { &mut __tmp_49 })
                },
                {
                    let mut __tmp_50: () = 9_();
                    let mut __tmp_51: () = 2_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_50 }, { &mut __tmp_51 })
                },
                {
                    let mut __tmp_52: () = 9_();
                    let mut __tmp_53: () = 3_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_52 }, { &mut __tmp_53 })
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
        }
        {
            let mut ar: [(); 9] = [
                {
                    let mut __tmp_54: () = 5_();
                    let mut __tmp_55: () = 1_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_54 }, { &mut __tmp_55 })
                },
                {
                    let mut __tmp_56: () = 5_();
                    let mut __tmp_57: () = 2_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_56 }, { &mut __tmp_57 })
                },
                {
                    let mut __tmp_58: () = 5_();
                    let mut __tmp_59: () = 3_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_58 }, { &mut __tmp_59 })
                },
                {
                    let mut __tmp_60: () = 7_();
                    let mut __tmp_61: () = 1_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_60 }, { &mut __tmp_61 })
                },
                {
                    let mut __tmp_62: () = 7_();
                    let mut __tmp_63: () = 2_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_62 }, { &mut __tmp_63 })
                },
                {
                    let mut __tmp_64: () = 7_();
                    let mut __tmp_65: () = 3_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_64 }, { &mut __tmp_65 })
                },
                {
                    let mut __tmp_66: () = 9_();
                    let mut __tmp_67: () = 1_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_66 }, { &mut __tmp_67 })
                },
                {
                    let mut __tmp_68: () = 9_();
                    let mut __tmp_69: () = 2_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_68 }, { &mut __tmp_69 })
                },
                {
                    let mut __tmp_70: () = 9_();
                    let mut __tmp_71: () = 3_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_70 }, { &mut __tmp_71 })
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
        }
    }
    {
        let mut ar: [(); 9] = [
            {
                let mut __tmp_72: () = 5_();
                let mut __tmp_73: () = 1_();
                std_pair_const_int__double_::new_1({ &mut __tmp_72 }, { &mut __tmp_73 })
            },
            {
                let mut __tmp_74: () = 5_();
                let mut __tmp_75: () = 2_();
                std_pair_const_int__double_::new_1({ &mut __tmp_74 }, { &mut __tmp_75 })
            },
            {
                let mut __tmp_76: () = 5_();
                let mut __tmp_77: () = 3_();
                std_pair_const_int__double_::new_1({ &mut __tmp_76 }, { &mut __tmp_77 })
            },
            {
                let mut __tmp_78: () = 7_();
                let mut __tmp_79: () = 1_();
                std_pair_const_int__double_::new_1({ &mut __tmp_78 }, { &mut __tmp_79 })
            },
            {
                let mut __tmp_80: () = 7_();
                let mut __tmp_81: () = 2_();
                std_pair_const_int__double_::new_1({ &mut __tmp_80 }, { &mut __tmp_81 })
            },
            {
                let mut __tmp_82: () = 7_();
                let mut __tmp_83: () = 3_();
                std_pair_const_int__double_::new_1({ &mut __tmp_82 }, { &mut __tmp_83 })
            },
            {
                let mut __tmp_84: () = 9_();
                let mut __tmp_85: () = 1_();
                std_pair_const_int__double_::new_1({ &mut __tmp_84 }, { &mut __tmp_85 })
            },
            {
                let mut __tmp_86: () = 9_();
                let mut __tmp_87: () = 2_();
                std_pair_const_int__double_::new_1({ &mut __tmp_86 }, { &mut __tmp_87 })
            },
            {
                let mut __tmp_88: () = 9_();
                let mut __tmp_89: () = 3_();
                std_pair_const_int__double_::new_1({ &mut __tmp_88 }, { &mut __tmp_89 })
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
    }
    {
        let mut m: () = unimplemented!();
        (unsafe {
            let mut ___p: () = unimplemented!();
            m.insert_41_struct_std_pair_struct_PrivateConstructor__double__0(&mut ___p)
        });
        (unsafe {
            let mut ___p: () = unimplemented!();
            m.insert_41_struct_std_pair_struct_PrivateConstructor__double__0(&mut ___p)
        });
        (unsafe {
            let mut ___p: () = unimplemented!();
            m.insert_41_struct_std_pair_struct_PrivateConstructor__double__0(&mut ___p)
        });
        (unsafe {
            let mut ___p: () = unimplemented!();
            m.insert_41_struct_std_pair_struct_PrivateConstructor__double__0(&mut ___p)
        });
        (unsafe {
            let mut ___p: () = unimplemented!();
            m.insert_41_struct_std_pair_struct_PrivateConstructor__double__0(&mut ___p)
        });
        (unsafe {
            let mut ___p: () = unimplemented!();
            m.insert_41_struct_std_pair_struct_PrivateConstructor__double__0(&mut ___p)
        });
        (unsafe {
            let mut ___p: () = unimplemented!();
            m.insert_41_struct_std_pair_struct_PrivateConstructor__double__0(&mut ___p)
        });
        (unsafe {
            let mut ___p: () = unimplemented!();
            m.insert_41_struct_std_pair_struct_PrivateConstructor__double__0(&mut ___p)
        });
        (unsafe {
            let mut ___p: () = unimplemented!();
            m.insert_41_struct_std_pair_struct_PrivateConstructor__double__0(&mut ___p)
        });
        let mut r: () = (unsafe {
            let mut ___k: () = 4_();
            m.lower_bound_74_int_struct_std_less_void__0(&mut ___k)
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
