extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn where_0(_a0: *const ()) {}
pub unsafe fn where_1(_a0: *const ()) {}
pub unsafe fn where_2(_a0: *const ()) {}
pub unsafe fn where_3(_a0: *const ()) {}
pub unsafe fn test1_4() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut multiplier_5))) });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut increment_6))) });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut modulus_7))) });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut default_seed_8))) });
}
pub unsafe fn test1_9() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut multiplier_10))) });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut increment_11))) });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut modulus_12))) });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut default_seed_13))) });
}
pub unsafe fn test1_14() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut multiplier_15))) });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut increment_16))) });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut modulus_17))) });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut default_seed_18))) });
}
pub unsafe fn test1_19() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut multiplier_20))) });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut increment_21))) });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut modulus_22))) });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut default_seed_23))) });
}
pub unsafe fn test1_24() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut multiplier_25))) });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut increment_26))) });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut modulus_27))) });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut default_seed_28))) });
}
pub unsafe fn test1_29() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut multiplier_30))) });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut increment_31))) });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut modulus_32))) });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut default_seed_33))) });
}
pub unsafe fn test1_34() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut multiplier_35))) });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut increment_36))) });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut modulus_37))) });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut default_seed_38))) });
}
pub unsafe fn test1_39() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut multiplier_40))) });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut increment_41))) });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut modulus_42))) });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut default_seed_43))) });
}
pub unsafe fn test1_44() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut multiplier_45))) });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut increment_46))) });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut modulus_47))) });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut default_seed_48))) });
}
pub unsafe fn test1_49() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut multiplier_50))) });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut increment_51))) });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut modulus_52))) });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut default_seed_53))) });
}
pub unsafe fn test1_54() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut multiplier_55))) });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut increment_56))) });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut modulus_57))) });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut default_seed_58))) });
}
pub unsafe fn test1_59() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut multiplier_60))) });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut increment_61))) });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut modulus_62))) });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut default_seed_63))) });
}
pub unsafe fn test1_64() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut multiplier_65))) });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut increment_66))) });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut modulus_67))) });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut default_seed_68))) });
}
pub unsafe fn test1_69() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut multiplier_70))) });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut increment_71))) });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut modulus_72))) });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut default_seed_73))) });
}
pub unsafe fn test1_74() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut multiplier_75))) });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut increment_76))) });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut modulus_77))) });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut default_seed_78))) });
}
pub unsafe fn test1_79() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut multiplier_80))) });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut increment_81))) });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut modulus_82))) });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut default_seed_83))) });
}
pub unsafe fn test1_84() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut multiplier_85))) });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut increment_86))) });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut modulus_87))) });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut default_seed_88))) });
}
pub unsafe fn test1_89() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut multiplier_90))) });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut increment_91))) });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut modulus_92))) });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut default_seed_93))) });
}
pub unsafe fn test1_94() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut multiplier_95))) });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut increment_96))) });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut modulus_97))) });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut default_seed_98))) });
}
pub unsafe fn test1_99() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut multiplier_100))) });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut increment_101))) });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut modulus_102))) });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut default_seed_103))) });
}
pub unsafe fn test1_104() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    (unsafe { where_1(&(*std::cell::LazyCell::force_mut(&mut *&raw mut multiplier_105))) });
    (unsafe { where_1(&(*std::cell::LazyCell::force_mut(&mut *&raw mut increment_106))) });
    (unsafe { where_1(&(*std::cell::LazyCell::force_mut(&mut *&raw mut modulus_107))) });
    (unsafe { where_1(&(*std::cell::LazyCell::force_mut(&mut *&raw mut default_seed_108))) });
}
pub unsafe fn test1_109() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    (unsafe { where_1(&(*std::cell::LazyCell::force_mut(&mut *&raw mut multiplier_110))) });
    (unsafe { where_1(&(*std::cell::LazyCell::force_mut(&mut *&raw mut increment_111))) });
    (unsafe { where_1(&(*std::cell::LazyCell::force_mut(&mut *&raw mut modulus_112))) });
    (unsafe { where_1(&(*std::cell::LazyCell::force_mut(&mut *&raw mut default_seed_113))) });
}
pub unsafe fn test1_114() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    (unsafe { where_1(&(*std::cell::LazyCell::force_mut(&mut *&raw mut multiplier_115))) });
    (unsafe { where_1(&(*std::cell::LazyCell::force_mut(&mut *&raw mut increment_116))) });
    (unsafe { where_1(&(*std::cell::LazyCell::force_mut(&mut *&raw mut modulus_117))) });
    (unsafe { where_1(&(*std::cell::LazyCell::force_mut(&mut *&raw mut default_seed_118))) });
}
pub unsafe fn test1_119() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    (unsafe { where_1(&(*std::cell::LazyCell::force_mut(&mut *&raw mut multiplier_120))) });
    (unsafe { where_1(&(*std::cell::LazyCell::force_mut(&mut *&raw mut increment_121))) });
    (unsafe { where_1(&(*std::cell::LazyCell::force_mut(&mut *&raw mut modulus_122))) });
    (unsafe { where_1(&(*std::cell::LazyCell::force_mut(&mut *&raw mut default_seed_123))) });
}
pub unsafe fn test1_124() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    (unsafe { where_1(&(*std::cell::LazyCell::force_mut(&mut *&raw mut multiplier_125))) });
    (unsafe { where_1(&(*std::cell::LazyCell::force_mut(&mut *&raw mut increment_126))) });
    (unsafe { where_1(&(*std::cell::LazyCell::force_mut(&mut *&raw mut modulus_127))) });
    (unsafe { where_1(&(*std::cell::LazyCell::force_mut(&mut *&raw mut default_seed_128))) });
}
pub unsafe fn test1_129() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    (unsafe { where_1(&(*std::cell::LazyCell::force_mut(&mut *&raw mut multiplier_130))) });
    (unsafe { where_1(&(*std::cell::LazyCell::force_mut(&mut *&raw mut increment_131))) });
    (unsafe { where_1(&(*std::cell::LazyCell::force_mut(&mut *&raw mut modulus_132))) });
    (unsafe { where_1(&(*std::cell::LazyCell::force_mut(&mut *&raw mut default_seed_133))) });
}
pub unsafe fn test1_134() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    (unsafe { where_1(&(*std::cell::LazyCell::force_mut(&mut *&raw mut multiplier_135))) });
    (unsafe { where_1(&(*std::cell::LazyCell::force_mut(&mut *&raw mut increment_136))) });
    (unsafe { where_1(&(*std::cell::LazyCell::force_mut(&mut *&raw mut modulus_137))) });
    (unsafe { where_1(&(*std::cell::LazyCell::force_mut(&mut *&raw mut default_seed_138))) });
}
pub unsafe fn test1_139() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    (unsafe { where_1(&(*std::cell::LazyCell::force_mut(&mut *&raw mut multiplier_140))) });
    (unsafe { where_1(&(*std::cell::LazyCell::force_mut(&mut *&raw mut increment_141))) });
    (unsafe { where_1(&(*std::cell::LazyCell::force_mut(&mut *&raw mut modulus_142))) });
    (unsafe { where_1(&(*std::cell::LazyCell::force_mut(&mut *&raw mut default_seed_143))) });
}
pub unsafe fn test1_144() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    (unsafe { where_1(&(*std::cell::LazyCell::force_mut(&mut *&raw mut multiplier_145))) });
    (unsafe { where_1(&(*std::cell::LazyCell::force_mut(&mut *&raw mut increment_146))) });
    (unsafe { where_1(&(*std::cell::LazyCell::force_mut(&mut *&raw mut modulus_147))) });
    (unsafe { where_1(&(*std::cell::LazyCell::force_mut(&mut *&raw mut default_seed_148))) });
}
pub unsafe fn test1_149() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    (unsafe { where_1(&(*std::cell::LazyCell::force_mut(&mut *&raw mut multiplier_150))) });
    (unsafe { where_1(&(*std::cell::LazyCell::force_mut(&mut *&raw mut increment_151))) });
    (unsafe { where_1(&(*std::cell::LazyCell::force_mut(&mut *&raw mut modulus_152))) });
    (unsafe { where_1(&(*std::cell::LazyCell::force_mut(&mut *&raw mut default_seed_153))) });
}
pub unsafe fn test1_154() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    (unsafe { where_1(&(*std::cell::LazyCell::force_mut(&mut *&raw mut multiplier_155))) });
    (unsafe { where_1(&(*std::cell::LazyCell::force_mut(&mut *&raw mut increment_156))) });
    (unsafe { where_1(&(*std::cell::LazyCell::force_mut(&mut *&raw mut modulus_157))) });
    (unsafe { where_1(&(*std::cell::LazyCell::force_mut(&mut *&raw mut default_seed_158))) });
}
pub unsafe fn test1_159() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    (unsafe { where_1(&(*std::cell::LazyCell::force_mut(&mut *&raw mut multiplier_160))) });
    (unsafe { where_1(&(*std::cell::LazyCell::force_mut(&mut *&raw mut increment_161))) });
    (unsafe { where_1(&(*std::cell::LazyCell::force_mut(&mut *&raw mut modulus_162))) });
    (unsafe { where_1(&(*std::cell::LazyCell::force_mut(&mut *&raw mut default_seed_163))) });
}
pub unsafe fn test1_164() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    (unsafe { where_1(&(*std::cell::LazyCell::force_mut(&mut *&raw mut multiplier_165))) });
    (unsafe { where_1(&(*std::cell::LazyCell::force_mut(&mut *&raw mut increment_166))) });
    (unsafe { where_1(&(*std::cell::LazyCell::force_mut(&mut *&raw mut modulus_167))) });
    (unsafe { where_1(&(*std::cell::LazyCell::force_mut(&mut *&raw mut default_seed_168))) });
}
pub unsafe fn test1_169() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    (unsafe { where_1(&(*std::cell::LazyCell::force_mut(&mut *&raw mut multiplier_170))) });
    (unsafe { where_1(&(*std::cell::LazyCell::force_mut(&mut *&raw mut increment_171))) });
    (unsafe { where_1(&(*std::cell::LazyCell::force_mut(&mut *&raw mut modulus_172))) });
    (unsafe { where_1(&(*std::cell::LazyCell::force_mut(&mut *&raw mut default_seed_173))) });
}
pub unsafe fn test1_174() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    (unsafe { where_1(&(*std::cell::LazyCell::force_mut(&mut *&raw mut multiplier_175))) });
    (unsafe { where_1(&(*std::cell::LazyCell::force_mut(&mut *&raw mut increment_176))) });
    (unsafe { where_1(&(*std::cell::LazyCell::force_mut(&mut *&raw mut modulus_177))) });
    (unsafe { where_1(&(*std::cell::LazyCell::force_mut(&mut *&raw mut default_seed_178))) });
}
pub unsafe fn test1_179() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    (unsafe { where_1(&(*std::cell::LazyCell::force_mut(&mut *&raw mut multiplier_180))) });
    (unsafe { where_1(&(*std::cell::LazyCell::force_mut(&mut *&raw mut increment_181))) });
    (unsafe { where_1(&(*std::cell::LazyCell::force_mut(&mut *&raw mut modulus_182))) });
    (unsafe { where_1(&(*std::cell::LazyCell::force_mut(&mut *&raw mut default_seed_183))) });
}
pub unsafe fn test1_184() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    (unsafe { where_1(&(*std::cell::LazyCell::force_mut(&mut *&raw mut multiplier_185))) });
    (unsafe { where_1(&(*std::cell::LazyCell::force_mut(&mut *&raw mut increment_186))) });
    (unsafe { where_1(&(*std::cell::LazyCell::force_mut(&mut *&raw mut modulus_187))) });
    (unsafe { where_1(&(*std::cell::LazyCell::force_mut(&mut *&raw mut default_seed_188))) });
}
pub unsafe fn test1_189() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    (unsafe { where_1(&(*std::cell::LazyCell::force_mut(&mut *&raw mut multiplier_190))) });
    (unsafe { where_1(&(*std::cell::LazyCell::force_mut(&mut *&raw mut increment_191))) });
    (unsafe { where_1(&(*std::cell::LazyCell::force_mut(&mut *&raw mut modulus_192))) });
    (unsafe { where_1(&(*std::cell::LazyCell::force_mut(&mut *&raw mut default_seed_193))) });
}
pub unsafe fn test1_194() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    (unsafe { where_1(&(*std::cell::LazyCell::force_mut(&mut *&raw mut multiplier_195))) });
    (unsafe { where_1(&(*std::cell::LazyCell::force_mut(&mut *&raw mut increment_196))) });
    (unsafe { where_1(&(*std::cell::LazyCell::force_mut(&mut *&raw mut modulus_197))) });
    (unsafe { where_1(&(*std::cell::LazyCell::force_mut(&mut *&raw mut default_seed_198))) });
}
pub unsafe fn test1_199() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    (unsafe { where_1(&(*std::cell::LazyCell::force_mut(&mut *&raw mut multiplier_200))) });
    (unsafe { where_1(&(*std::cell::LazyCell::force_mut(&mut *&raw mut increment_201))) });
    (unsafe { where_1(&(*std::cell::LazyCell::force_mut(&mut *&raw mut modulus_202))) });
    (unsafe { where_1(&(*std::cell::LazyCell::force_mut(&mut *&raw mut default_seed_203))) });
}
pub unsafe fn test1_204() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    (unsafe { where_2(&(*std::cell::LazyCell::force_mut(&mut *&raw mut multiplier_205))) });
    (unsafe { where_2(&(*std::cell::LazyCell::force_mut(&mut *&raw mut increment_206))) });
    (unsafe { where_2(&(*std::cell::LazyCell::force_mut(&mut *&raw mut modulus_207))) });
    (unsafe { where_2(&(*std::cell::LazyCell::force_mut(&mut *&raw mut default_seed_208))) });
}
pub unsafe fn test1_209() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    (unsafe { where_2(&(*std::cell::LazyCell::force_mut(&mut *&raw mut multiplier_210))) });
    (unsafe { where_2(&(*std::cell::LazyCell::force_mut(&mut *&raw mut increment_211))) });
    (unsafe { where_2(&(*std::cell::LazyCell::force_mut(&mut *&raw mut modulus_212))) });
    (unsafe { where_2(&(*std::cell::LazyCell::force_mut(&mut *&raw mut default_seed_213))) });
}
pub unsafe fn test1_214() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    (unsafe { where_2(&(*std::cell::LazyCell::force_mut(&mut *&raw mut multiplier_215))) });
    (unsafe { where_2(&(*std::cell::LazyCell::force_mut(&mut *&raw mut increment_216))) });
    (unsafe { where_2(&(*std::cell::LazyCell::force_mut(&mut *&raw mut modulus_217))) });
    (unsafe { where_2(&(*std::cell::LazyCell::force_mut(&mut *&raw mut default_seed_218))) });
}
pub unsafe fn test1_219() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    (unsafe { where_2(&(*std::cell::LazyCell::force_mut(&mut *&raw mut multiplier_220))) });
    (unsafe { where_2(&(*std::cell::LazyCell::force_mut(&mut *&raw mut increment_221))) });
    (unsafe { where_2(&(*std::cell::LazyCell::force_mut(&mut *&raw mut modulus_222))) });
    (unsafe { where_2(&(*std::cell::LazyCell::force_mut(&mut *&raw mut default_seed_223))) });
}
pub unsafe fn test1_224() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    (unsafe { where_2(&(*std::cell::LazyCell::force_mut(&mut *&raw mut multiplier_225))) });
    (unsafe { where_2(&(*std::cell::LazyCell::force_mut(&mut *&raw mut increment_226))) });
    (unsafe { where_2(&(*std::cell::LazyCell::force_mut(&mut *&raw mut modulus_227))) });
    (unsafe { where_2(&(*std::cell::LazyCell::force_mut(&mut *&raw mut default_seed_228))) });
}
pub unsafe fn test1_229() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    (unsafe { where_2(&(*std::cell::LazyCell::force_mut(&mut *&raw mut multiplier_230))) });
    (unsafe { where_2(&(*std::cell::LazyCell::force_mut(&mut *&raw mut increment_231))) });
    (unsafe { where_2(&(*std::cell::LazyCell::force_mut(&mut *&raw mut modulus_232))) });
    (unsafe { where_2(&(*std::cell::LazyCell::force_mut(&mut *&raw mut default_seed_233))) });
}
pub unsafe fn test1_234() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    (unsafe { where_2(&(*std::cell::LazyCell::force_mut(&mut *&raw mut multiplier_235))) });
    (unsafe { where_2(&(*std::cell::LazyCell::force_mut(&mut *&raw mut increment_236))) });
    (unsafe { where_2(&(*std::cell::LazyCell::force_mut(&mut *&raw mut modulus_237))) });
    (unsafe { where_2(&(*std::cell::LazyCell::force_mut(&mut *&raw mut default_seed_238))) });
}
pub unsafe fn test1_239() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    (unsafe { where_2(&(*std::cell::LazyCell::force_mut(&mut *&raw mut multiplier_240))) });
    (unsafe { where_2(&(*std::cell::LazyCell::force_mut(&mut *&raw mut increment_241))) });
    (unsafe { where_2(&(*std::cell::LazyCell::force_mut(&mut *&raw mut modulus_242))) });
    (unsafe { where_2(&(*std::cell::LazyCell::force_mut(&mut *&raw mut default_seed_243))) });
}
pub unsafe fn test1_244() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    (unsafe { where_2(&(*std::cell::LazyCell::force_mut(&mut *&raw mut multiplier_245))) });
    (unsafe { where_2(&(*std::cell::LazyCell::force_mut(&mut *&raw mut increment_246))) });
    (unsafe { where_2(&(*std::cell::LazyCell::force_mut(&mut *&raw mut modulus_247))) });
    (unsafe { where_2(&(*std::cell::LazyCell::force_mut(&mut *&raw mut default_seed_248))) });
}
pub unsafe fn test1_249() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    (unsafe { where_2(&(*std::cell::LazyCell::force_mut(&mut *&raw mut multiplier_250))) });
    (unsafe { where_2(&(*std::cell::LazyCell::force_mut(&mut *&raw mut increment_251))) });
    (unsafe { where_2(&(*std::cell::LazyCell::force_mut(&mut *&raw mut modulus_252))) });
    (unsafe { where_2(&(*std::cell::LazyCell::force_mut(&mut *&raw mut default_seed_253))) });
}
pub unsafe fn test1_254() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    (unsafe { where_2(&(*std::cell::LazyCell::force_mut(&mut *&raw mut multiplier_255))) });
    (unsafe { where_2(&(*std::cell::LazyCell::force_mut(&mut *&raw mut increment_256))) });
    (unsafe { where_2(&(*std::cell::LazyCell::force_mut(&mut *&raw mut modulus_257))) });
    (unsafe { where_2(&(*std::cell::LazyCell::force_mut(&mut *&raw mut default_seed_258))) });
}
pub unsafe fn test1_259() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    (unsafe { where_2(&(*std::cell::LazyCell::force_mut(&mut *&raw mut multiplier_260))) });
    (unsafe { where_2(&(*std::cell::LazyCell::force_mut(&mut *&raw mut increment_261))) });
    (unsafe { where_2(&(*std::cell::LazyCell::force_mut(&mut *&raw mut modulus_262))) });
    (unsafe { where_2(&(*std::cell::LazyCell::force_mut(&mut *&raw mut default_seed_263))) });
}
pub unsafe fn test1_264() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    (unsafe { where_2(&(*std::cell::LazyCell::force_mut(&mut *&raw mut multiplier_265))) });
    (unsafe { where_2(&(*std::cell::LazyCell::force_mut(&mut *&raw mut increment_266))) });
    (unsafe { where_2(&(*std::cell::LazyCell::force_mut(&mut *&raw mut modulus_267))) });
    (unsafe { where_2(&(*std::cell::LazyCell::force_mut(&mut *&raw mut default_seed_268))) });
}
pub unsafe fn test1_269() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    (unsafe { where_2(&(*std::cell::LazyCell::force_mut(&mut *&raw mut multiplier_270))) });
    (unsafe { where_2(&(*std::cell::LazyCell::force_mut(&mut *&raw mut increment_271))) });
    (unsafe { where_2(&(*std::cell::LazyCell::force_mut(&mut *&raw mut modulus_272))) });
    (unsafe { where_2(&(*std::cell::LazyCell::force_mut(&mut *&raw mut default_seed_273))) });
}
pub unsafe fn test1_274() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    (unsafe { where_2(&(*std::cell::LazyCell::force_mut(&mut *&raw mut multiplier_275))) });
    (unsafe { where_2(&(*std::cell::LazyCell::force_mut(&mut *&raw mut increment_276))) });
    (unsafe { where_2(&(*std::cell::LazyCell::force_mut(&mut *&raw mut modulus_277))) });
    (unsafe { where_2(&(*std::cell::LazyCell::force_mut(&mut *&raw mut default_seed_278))) });
}
pub unsafe fn test1_279() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    (unsafe { where_2(&(*std::cell::LazyCell::force_mut(&mut *&raw mut multiplier_280))) });
    (unsafe { where_2(&(*std::cell::LazyCell::force_mut(&mut *&raw mut increment_281))) });
    (unsafe { where_2(&(*std::cell::LazyCell::force_mut(&mut *&raw mut modulus_282))) });
    (unsafe { where_2(&(*std::cell::LazyCell::force_mut(&mut *&raw mut default_seed_283))) });
}
pub unsafe fn test1_284() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    (unsafe { where_2(&(*std::cell::LazyCell::force_mut(&mut *&raw mut multiplier_285))) });
    (unsafe { where_2(&(*std::cell::LazyCell::force_mut(&mut *&raw mut increment_286))) });
    (unsafe { where_2(&(*std::cell::LazyCell::force_mut(&mut *&raw mut modulus_287))) });
    (unsafe { where_2(&(*std::cell::LazyCell::force_mut(&mut *&raw mut default_seed_288))) });
}
pub unsafe fn test1_289() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    (unsafe { where_2(&(*std::cell::LazyCell::force_mut(&mut *&raw mut multiplier_290))) });
    (unsafe { where_2(&(*std::cell::LazyCell::force_mut(&mut *&raw mut increment_291))) });
    (unsafe { where_2(&(*std::cell::LazyCell::force_mut(&mut *&raw mut modulus_292))) });
    (unsafe { where_2(&(*std::cell::LazyCell::force_mut(&mut *&raw mut default_seed_293))) });
}
pub unsafe fn test1_294() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    (unsafe { where_2(&(*std::cell::LazyCell::force_mut(&mut *&raw mut multiplier_295))) });
    (unsafe { where_2(&(*std::cell::LazyCell::force_mut(&mut *&raw mut increment_296))) });
    (unsafe { where_2(&(*std::cell::LazyCell::force_mut(&mut *&raw mut modulus_297))) });
    (unsafe { where_2(&(*std::cell::LazyCell::force_mut(&mut *&raw mut default_seed_298))) });
}
pub unsafe fn test1_299() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    (unsafe { where_2(&(*std::cell::LazyCell::force_mut(&mut *&raw mut multiplier_300))) });
    (unsafe { where_2(&(*std::cell::LazyCell::force_mut(&mut *&raw mut increment_301))) });
    (unsafe { where_2(&(*std::cell::LazyCell::force_mut(&mut *&raw mut modulus_302))) });
    (unsafe { where_2(&(*std::cell::LazyCell::force_mut(&mut *&raw mut default_seed_303))) });
}
pub unsafe fn test1_304() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    (unsafe { where_3(&(*std::cell::LazyCell::force_mut(&mut *&raw mut multiplier_305))) });
    (unsafe { where_3(&(*std::cell::LazyCell::force_mut(&mut *&raw mut increment_306))) });
    (unsafe { where_3(&(*std::cell::LazyCell::force_mut(&mut *&raw mut modulus_307))) });
    (unsafe { where_3(&(*std::cell::LazyCell::force_mut(&mut *&raw mut default_seed_308))) });
}
pub unsafe fn test1_309() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    (unsafe { where_3(&(*std::cell::LazyCell::force_mut(&mut *&raw mut multiplier_310))) });
    (unsafe { where_3(&(*std::cell::LazyCell::force_mut(&mut *&raw mut increment_311))) });
    (unsafe { where_3(&(*std::cell::LazyCell::force_mut(&mut *&raw mut modulus_312))) });
    (unsafe { where_3(&(*std::cell::LazyCell::force_mut(&mut *&raw mut default_seed_313))) });
}
pub unsafe fn test1_314() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    (unsafe { where_3(&(*std::cell::LazyCell::force_mut(&mut *&raw mut multiplier_315))) });
    (unsafe { where_3(&(*std::cell::LazyCell::force_mut(&mut *&raw mut increment_316))) });
    (unsafe { where_3(&(*std::cell::LazyCell::force_mut(&mut *&raw mut modulus_317))) });
    (unsafe { where_3(&(*std::cell::LazyCell::force_mut(&mut *&raw mut default_seed_318))) });
}
pub unsafe fn test1_319() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    (unsafe { where_3(&(*std::cell::LazyCell::force_mut(&mut *&raw mut multiplier_320))) });
    (unsafe { where_3(&(*std::cell::LazyCell::force_mut(&mut *&raw mut increment_321))) });
    (unsafe { where_3(&(*std::cell::LazyCell::force_mut(&mut *&raw mut modulus_322))) });
    (unsafe { where_3(&(*std::cell::LazyCell::force_mut(&mut *&raw mut default_seed_323))) });
}
pub unsafe fn test1_324() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    (unsafe { where_3(&(*std::cell::LazyCell::force_mut(&mut *&raw mut multiplier_325))) });
    (unsafe { where_3(&(*std::cell::LazyCell::force_mut(&mut *&raw mut increment_326))) });
    (unsafe { where_3(&(*std::cell::LazyCell::force_mut(&mut *&raw mut modulus_327))) });
    (unsafe { where_3(&(*std::cell::LazyCell::force_mut(&mut *&raw mut default_seed_328))) });
}
pub unsafe fn test1_329() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    (unsafe { where_3(&(*std::cell::LazyCell::force_mut(&mut *&raw mut multiplier_330))) });
    (unsafe { where_3(&(*std::cell::LazyCell::force_mut(&mut *&raw mut increment_331))) });
    (unsafe { where_3(&(*std::cell::LazyCell::force_mut(&mut *&raw mut modulus_332))) });
    (unsafe { where_3(&(*std::cell::LazyCell::force_mut(&mut *&raw mut default_seed_333))) });
}
pub unsafe fn test1_334() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    (unsafe { where_3(&(*std::cell::LazyCell::force_mut(&mut *&raw mut multiplier_335))) });
    (unsafe { where_3(&(*std::cell::LazyCell::force_mut(&mut *&raw mut increment_336))) });
    (unsafe { where_3(&(*std::cell::LazyCell::force_mut(&mut *&raw mut modulus_337))) });
    (unsafe { where_3(&(*std::cell::LazyCell::force_mut(&mut *&raw mut default_seed_338))) });
}
pub unsafe fn test1_339() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    (unsafe { where_3(&(*std::cell::LazyCell::force_mut(&mut *&raw mut multiplier_340))) });
    (unsafe { where_3(&(*std::cell::LazyCell::force_mut(&mut *&raw mut increment_341))) });
    (unsafe { where_3(&(*std::cell::LazyCell::force_mut(&mut *&raw mut modulus_342))) });
    (unsafe { where_3(&(*std::cell::LazyCell::force_mut(&mut *&raw mut default_seed_343))) });
}
pub unsafe fn test1_344() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    (unsafe { where_3(&(*std::cell::LazyCell::force_mut(&mut *&raw mut multiplier_345))) });
    (unsafe { where_3(&(*std::cell::LazyCell::force_mut(&mut *&raw mut increment_346))) });
    (unsafe { where_3(&(*std::cell::LazyCell::force_mut(&mut *&raw mut modulus_347))) });
    (unsafe { where_3(&(*std::cell::LazyCell::force_mut(&mut *&raw mut default_seed_348))) });
}
pub unsafe fn test1_349() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    (unsafe { where_3(&(*std::cell::LazyCell::force_mut(&mut *&raw mut multiplier_350))) });
    (unsafe { where_3(&(*std::cell::LazyCell::force_mut(&mut *&raw mut increment_351))) });
    (unsafe { where_3(&(*std::cell::LazyCell::force_mut(&mut *&raw mut modulus_352))) });
    (unsafe { where_3(&(*std::cell::LazyCell::force_mut(&mut *&raw mut default_seed_353))) });
}
pub unsafe fn test1_354() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    (unsafe { where_3(&(*std::cell::LazyCell::force_mut(&mut *&raw mut multiplier_355))) });
    (unsafe { where_3(&(*std::cell::LazyCell::force_mut(&mut *&raw mut increment_356))) });
    (unsafe { where_3(&(*std::cell::LazyCell::force_mut(&mut *&raw mut modulus_357))) });
    (unsafe { where_3(&(*std::cell::LazyCell::force_mut(&mut *&raw mut default_seed_358))) });
}
pub unsafe fn test1_359() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    (unsafe { where_3(&(*std::cell::LazyCell::force_mut(&mut *&raw mut multiplier_360))) });
    (unsafe { where_3(&(*std::cell::LazyCell::force_mut(&mut *&raw mut increment_361))) });
    (unsafe { where_3(&(*std::cell::LazyCell::force_mut(&mut *&raw mut modulus_362))) });
    (unsafe { where_3(&(*std::cell::LazyCell::force_mut(&mut *&raw mut default_seed_363))) });
}
pub unsafe fn test1_364() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    (unsafe { where_3(&(*std::cell::LazyCell::force_mut(&mut *&raw mut multiplier_365))) });
    (unsafe { where_3(&(*std::cell::LazyCell::force_mut(&mut *&raw mut increment_366))) });
    (unsafe { where_3(&(*std::cell::LazyCell::force_mut(&mut *&raw mut modulus_367))) });
    (unsafe { where_3(&(*std::cell::LazyCell::force_mut(&mut *&raw mut default_seed_368))) });
}
pub unsafe fn test1_369() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    (unsafe { where_3(&(*std::cell::LazyCell::force_mut(&mut *&raw mut multiplier_370))) });
    (unsafe { where_3(&(*std::cell::LazyCell::force_mut(&mut *&raw mut increment_371))) });
    (unsafe { where_3(&(*std::cell::LazyCell::force_mut(&mut *&raw mut modulus_372))) });
    (unsafe { where_3(&(*std::cell::LazyCell::force_mut(&mut *&raw mut default_seed_373))) });
}
pub unsafe fn test1_374() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    (unsafe { where_3(&(*std::cell::LazyCell::force_mut(&mut *&raw mut multiplier_375))) });
    (unsafe { where_3(&(*std::cell::LazyCell::force_mut(&mut *&raw mut increment_376))) });
    (unsafe { where_3(&(*std::cell::LazyCell::force_mut(&mut *&raw mut modulus_377))) });
    (unsafe { where_3(&(*std::cell::LazyCell::force_mut(&mut *&raw mut default_seed_378))) });
}
pub unsafe fn test1_379() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    (unsafe { where_3(&(*std::cell::LazyCell::force_mut(&mut *&raw mut multiplier_380))) });
    (unsafe { where_3(&(*std::cell::LazyCell::force_mut(&mut *&raw mut increment_381))) });
    (unsafe { where_3(&(*std::cell::LazyCell::force_mut(&mut *&raw mut modulus_382))) });
    (unsafe { where_3(&(*std::cell::LazyCell::force_mut(&mut *&raw mut default_seed_383))) });
}
pub unsafe fn test1_384() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    (unsafe { where_3(&(*std::cell::LazyCell::force_mut(&mut *&raw mut multiplier_385))) });
    (unsafe { where_3(&(*std::cell::LazyCell::force_mut(&mut *&raw mut increment_386))) });
    (unsafe { where_3(&(*std::cell::LazyCell::force_mut(&mut *&raw mut modulus_387))) });
    (unsafe { where_3(&(*std::cell::LazyCell::force_mut(&mut *&raw mut default_seed_388))) });
}
pub unsafe fn test1_389() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    (unsafe { where_3(&(*std::cell::LazyCell::force_mut(&mut *&raw mut multiplier_390))) });
    (unsafe { where_3(&(*std::cell::LazyCell::force_mut(&mut *&raw mut increment_391))) });
    (unsafe { where_3(&(*std::cell::LazyCell::force_mut(&mut *&raw mut modulus_392))) });
    (unsafe { where_3(&(*std::cell::LazyCell::force_mut(&mut *&raw mut default_seed_393))) });
}
pub unsafe fn test1_394() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    (unsafe { where_3(&(*std::cell::LazyCell::force_mut(&mut *&raw mut multiplier_395))) });
    (unsafe { where_3(&(*std::cell::LazyCell::force_mut(&mut *&raw mut increment_396))) });
    (unsafe { where_3(&(*std::cell::LazyCell::force_mut(&mut *&raw mut modulus_397))) });
    (unsafe { where_3(&(*std::cell::LazyCell::force_mut(&mut *&raw mut default_seed_398))) });
}
pub unsafe fn test1_399() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    (unsafe { where_3(&(*std::cell::LazyCell::force_mut(&mut *&raw mut multiplier_400))) });
    (unsafe { where_3(&(*std::cell::LazyCell::force_mut(&mut *&raw mut increment_401))) });
    (unsafe { where_3(&(*std::cell::LazyCell::force_mut(&mut *&raw mut modulus_402))) });
    (unsafe { where_3(&(*std::cell::LazyCell::force_mut(&mut *&raw mut default_seed_403))) });
}
pub unsafe fn test_404() {
    let W: () = (::std::mem::size_of::<()>() as ()).wrapping_mul(8_());
    let M: () = -1_();
    let A: () = (((1_()) << ((W) / (2_()))) - (1_()));
    (unsafe { test1_4() });
    (unsafe { test1_9() });
    (unsafe { test1_14() });
    (unsafe { test1_19() });
    (unsafe { test1_24() });
    (unsafe { test1_29() });
    (unsafe { test1_34() });
    (unsafe { test1_39() });
    (unsafe { test1_44() });
    (unsafe { test1_49() });
    (unsafe { test1_54() });
    (unsafe { test1_59() });
    (unsafe { test1_64() });
    (unsafe { test1_69() });
}
pub unsafe fn test_405() {
    let W: () = (::std::mem::size_of::<()>() as ()).wrapping_mul(8_());
    let M: () = -1_();
    let A: () = (((1_()) << ((W) / (2_()))) as ()).wrapping_sub(1_());
    (unsafe { test1_104() });
    (unsafe { test1_109() });
    (unsafe { test1_114() });
    (unsafe { test1_119() });
    (unsafe { test1_124() });
    (unsafe { test1_129() });
    (unsafe { test1_134() });
    (unsafe { test1_139() });
    (unsafe { test1_144() });
    (unsafe { test1_149() });
    (unsafe { test1_154() });
    (unsafe { test1_159() });
    (unsafe { test1_164() });
    (unsafe { test1_169() });
}
pub unsafe fn test_406() {
    let W: () = (::std::mem::size_of::<()>() as ()).wrapping_mul(8_());
    let M: () = -1_();
    let A: () = (((1_()) << ((W) / (2_()))) as ()).wrapping_sub(1_());
    (unsafe { test1_204() });
    (unsafe { test1_209() });
    (unsafe { test1_214() });
    (unsafe { test1_219() });
    (unsafe { test1_224() });
    (unsafe { test1_229() });
    (unsafe { test1_234() });
    (unsafe { test1_239() });
    (unsafe { test1_244() });
    (unsafe { test1_249() });
    (unsafe { test1_254() });
    (unsafe { test1_259() });
    (unsafe { test1_264() });
    (unsafe { test1_269() });
}
pub unsafe fn test_407() {
    let W: () = (::std::mem::size_of::<()>() as ()).wrapping_mul(8_());
    let M: () = -1_();
    let A: () = (((1_()) << ((W) / (2_()))) as ()).wrapping_sub(1_());
    (unsafe { test1_304() });
    (unsafe { test1_309() });
    (unsafe { test1_314() });
    (unsafe { test1_319() });
    (unsafe { test1_324() });
    (unsafe { test1_329() });
    (unsafe { test1_334() });
    (unsafe { test1_339() });
    (unsafe { test1_344() });
    (unsafe { test1_349() });
    (unsafe { test1_354() });
    (unsafe { test1_359() });
    (unsafe { test1_364() });
    (unsafe { test1_369() });
}
pub unsafe fn test_ext_408() {
    let M: () = -1_();
    (unsafe { test1_74() });
    (unsafe { test1_79() });
    (unsafe { test1_84() });
    (unsafe { test1_89() });
    (unsafe { test1_94() });
    (unsafe { test1_99() });
}
pub unsafe fn test_ext_409() {
    let M: () = -1_();
    (unsafe { test1_174() });
    (unsafe { test1_179() });
    (unsafe { test1_184() });
    (unsafe { test1_189() });
    (unsafe { test1_194() });
    (unsafe { test1_199() });
}
pub unsafe fn test_ext_410() {
    let M: () = -1_();
    (unsafe { test1_274() });
    (unsafe { test1_279() });
    (unsafe { test1_284() });
    (unsafe { test1_289() });
    (unsafe { test1_294() });
    (unsafe { test1_299() });
}
pub unsafe fn test_ext_411() {
    let M: () = -1_();
    (unsafe { test1_374() });
    (unsafe { test1_379() });
    (unsafe { test1_384() });
    (unsafe { test1_389() });
    (unsafe { test1_394() });
    (unsafe { test1_399() });
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
    (unsafe { test_404() });
    (unsafe { test_ext_408() });
    (unsafe { test_405() });
    (unsafe { test_ext_409() });
    (unsafe { test_406() });
    (unsafe { test_ext_410() });
    (unsafe { test_407() });
    (unsafe { test_ext_411() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
