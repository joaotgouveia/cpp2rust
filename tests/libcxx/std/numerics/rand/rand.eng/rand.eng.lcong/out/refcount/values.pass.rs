extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn where_0(_a0: Ptr<()>) {}
pub fn where_1(_a0: Ptr<()>) {}
pub fn where_2(_a0: Ptr<()>) {}
pub fn where_3(_a0: Ptr<()>) {}
pub fn test1_4() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    ({ where_0(multiplier_5.with(|v| v.as_pointer())) });
    ({ where_0(increment_6.with(|v| v.as_pointer())) });
    ({ where_0(modulus_7.with(|v| v.as_pointer())) });
    ({ where_0(default_seed_8.with(|v| v.as_pointer())) });
}
pub fn test1_9() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    ({ where_0(multiplier_10.with(|v| v.as_pointer())) });
    ({ where_0(increment_11.with(|v| v.as_pointer())) });
    ({ where_0(modulus_12.with(|v| v.as_pointer())) });
    ({ where_0(default_seed_13.with(|v| v.as_pointer())) });
}
pub fn test1_14() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    ({ where_0(multiplier_15.with(|v| v.as_pointer())) });
    ({ where_0(increment_16.with(|v| v.as_pointer())) });
    ({ where_0(modulus_17.with(|v| v.as_pointer())) });
    ({ where_0(default_seed_18.with(|v| v.as_pointer())) });
}
pub fn test1_19() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    ({ where_0(multiplier_20.with(|v| v.as_pointer())) });
    ({ where_0(increment_21.with(|v| v.as_pointer())) });
    ({ where_0(modulus_22.with(|v| v.as_pointer())) });
    ({ where_0(default_seed_23.with(|v| v.as_pointer())) });
}
pub fn test1_24() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    ({ where_0(multiplier_25.with(|v| v.as_pointer())) });
    ({ where_0(increment_26.with(|v| v.as_pointer())) });
    ({ where_0(modulus_27.with(|v| v.as_pointer())) });
    ({ where_0(default_seed_28.with(|v| v.as_pointer())) });
}
pub fn test1_29() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    ({ where_0(multiplier_30.with(|v| v.as_pointer())) });
    ({ where_0(increment_31.with(|v| v.as_pointer())) });
    ({ where_0(modulus_32.with(|v| v.as_pointer())) });
    ({ where_0(default_seed_33.with(|v| v.as_pointer())) });
}
pub fn test1_34() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    ({ where_0(multiplier_35.with(|v| v.as_pointer())) });
    ({ where_0(increment_36.with(|v| v.as_pointer())) });
    ({ where_0(modulus_37.with(|v| v.as_pointer())) });
    ({ where_0(default_seed_38.with(|v| v.as_pointer())) });
}
pub fn test1_39() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    ({ where_0(multiplier_40.with(|v| v.as_pointer())) });
    ({ where_0(increment_41.with(|v| v.as_pointer())) });
    ({ where_0(modulus_42.with(|v| v.as_pointer())) });
    ({ where_0(default_seed_43.with(|v| v.as_pointer())) });
}
pub fn test1_44() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    ({ where_0(multiplier_45.with(|v| v.as_pointer())) });
    ({ where_0(increment_46.with(|v| v.as_pointer())) });
    ({ where_0(modulus_47.with(|v| v.as_pointer())) });
    ({ where_0(default_seed_48.with(|v| v.as_pointer())) });
}
pub fn test1_49() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    ({ where_0(multiplier_50.with(|v| v.as_pointer())) });
    ({ where_0(increment_51.with(|v| v.as_pointer())) });
    ({ where_0(modulus_52.with(|v| v.as_pointer())) });
    ({ where_0(default_seed_53.with(|v| v.as_pointer())) });
}
pub fn test1_54() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    ({ where_0(multiplier_55.with(|v| v.as_pointer())) });
    ({ where_0(increment_56.with(|v| v.as_pointer())) });
    ({ where_0(modulus_57.with(|v| v.as_pointer())) });
    ({ where_0(default_seed_58.with(|v| v.as_pointer())) });
}
pub fn test1_59() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    ({ where_0(multiplier_60.with(|v| v.as_pointer())) });
    ({ where_0(increment_61.with(|v| v.as_pointer())) });
    ({ where_0(modulus_62.with(|v| v.as_pointer())) });
    ({ where_0(default_seed_63.with(|v| v.as_pointer())) });
}
pub fn test1_64() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    ({ where_0(multiplier_65.with(|v| v.as_pointer())) });
    ({ where_0(increment_66.with(|v| v.as_pointer())) });
    ({ where_0(modulus_67.with(|v| v.as_pointer())) });
    ({ where_0(default_seed_68.with(|v| v.as_pointer())) });
}
pub fn test1_69() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    ({ where_0(multiplier_70.with(|v| v.as_pointer())) });
    ({ where_0(increment_71.with(|v| v.as_pointer())) });
    ({ where_0(modulus_72.with(|v| v.as_pointer())) });
    ({ where_0(default_seed_73.with(|v| v.as_pointer())) });
}
pub fn test1_74() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    ({ where_0(multiplier_75.with(|v| v.as_pointer())) });
    ({ where_0(increment_76.with(|v| v.as_pointer())) });
    ({ where_0(modulus_77.with(|v| v.as_pointer())) });
    ({ where_0(default_seed_78.with(|v| v.as_pointer())) });
}
pub fn test1_79() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    ({ where_0(multiplier_80.with(|v| v.as_pointer())) });
    ({ where_0(increment_81.with(|v| v.as_pointer())) });
    ({ where_0(modulus_82.with(|v| v.as_pointer())) });
    ({ where_0(default_seed_83.with(|v| v.as_pointer())) });
}
pub fn test1_84() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    ({ where_0(multiplier_85.with(|v| v.as_pointer())) });
    ({ where_0(increment_86.with(|v| v.as_pointer())) });
    ({ where_0(modulus_87.with(|v| v.as_pointer())) });
    ({ where_0(default_seed_88.with(|v| v.as_pointer())) });
}
pub fn test1_89() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    ({ where_0(multiplier_90.with(|v| v.as_pointer())) });
    ({ where_0(increment_91.with(|v| v.as_pointer())) });
    ({ where_0(modulus_92.with(|v| v.as_pointer())) });
    ({ where_0(default_seed_93.with(|v| v.as_pointer())) });
}
pub fn test1_94() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    ({ where_0(multiplier_95.with(|v| v.as_pointer())) });
    ({ where_0(increment_96.with(|v| v.as_pointer())) });
    ({ where_0(modulus_97.with(|v| v.as_pointer())) });
    ({ where_0(default_seed_98.with(|v| v.as_pointer())) });
}
pub fn test1_99() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    ({ where_0(multiplier_100.with(|v| v.as_pointer())) });
    ({ where_0(increment_101.with(|v| v.as_pointer())) });
    ({ where_0(modulus_102.with(|v| v.as_pointer())) });
    ({ where_0(default_seed_103.with(|v| v.as_pointer())) });
}
pub fn test1_104() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    ({ where_1(multiplier_105.with(|v| v.as_pointer())) });
    ({ where_1(increment_106.with(|v| v.as_pointer())) });
    ({ where_1(modulus_107.with(|v| v.as_pointer())) });
    ({ where_1(default_seed_108.with(|v| v.as_pointer())) });
}
pub fn test1_109() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    ({ where_1(multiplier_110.with(|v| v.as_pointer())) });
    ({ where_1(increment_111.with(|v| v.as_pointer())) });
    ({ where_1(modulus_112.with(|v| v.as_pointer())) });
    ({ where_1(default_seed_113.with(|v| v.as_pointer())) });
}
pub fn test1_114() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    ({ where_1(multiplier_115.with(|v| v.as_pointer())) });
    ({ where_1(increment_116.with(|v| v.as_pointer())) });
    ({ where_1(modulus_117.with(|v| v.as_pointer())) });
    ({ where_1(default_seed_118.with(|v| v.as_pointer())) });
}
pub fn test1_119() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    ({ where_1(multiplier_120.with(|v| v.as_pointer())) });
    ({ where_1(increment_121.with(|v| v.as_pointer())) });
    ({ where_1(modulus_122.with(|v| v.as_pointer())) });
    ({ where_1(default_seed_123.with(|v| v.as_pointer())) });
}
pub fn test1_124() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    ({ where_1(multiplier_125.with(|v| v.as_pointer())) });
    ({ where_1(increment_126.with(|v| v.as_pointer())) });
    ({ where_1(modulus_127.with(|v| v.as_pointer())) });
    ({ where_1(default_seed_128.with(|v| v.as_pointer())) });
}
pub fn test1_129() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    ({ where_1(multiplier_130.with(|v| v.as_pointer())) });
    ({ where_1(increment_131.with(|v| v.as_pointer())) });
    ({ where_1(modulus_132.with(|v| v.as_pointer())) });
    ({ where_1(default_seed_133.with(|v| v.as_pointer())) });
}
pub fn test1_134() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    ({ where_1(multiplier_135.with(|v| v.as_pointer())) });
    ({ where_1(increment_136.with(|v| v.as_pointer())) });
    ({ where_1(modulus_137.with(|v| v.as_pointer())) });
    ({ where_1(default_seed_138.with(|v| v.as_pointer())) });
}
pub fn test1_139() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    ({ where_1(multiplier_140.with(|v| v.as_pointer())) });
    ({ where_1(increment_141.with(|v| v.as_pointer())) });
    ({ where_1(modulus_142.with(|v| v.as_pointer())) });
    ({ where_1(default_seed_143.with(|v| v.as_pointer())) });
}
pub fn test1_144() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    ({ where_1(multiplier_145.with(|v| v.as_pointer())) });
    ({ where_1(increment_146.with(|v| v.as_pointer())) });
    ({ where_1(modulus_147.with(|v| v.as_pointer())) });
    ({ where_1(default_seed_148.with(|v| v.as_pointer())) });
}
pub fn test1_149() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    ({ where_1(multiplier_150.with(|v| v.as_pointer())) });
    ({ where_1(increment_151.with(|v| v.as_pointer())) });
    ({ where_1(modulus_152.with(|v| v.as_pointer())) });
    ({ where_1(default_seed_153.with(|v| v.as_pointer())) });
}
pub fn test1_154() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    ({ where_1(multiplier_155.with(|v| v.as_pointer())) });
    ({ where_1(increment_156.with(|v| v.as_pointer())) });
    ({ where_1(modulus_157.with(|v| v.as_pointer())) });
    ({ where_1(default_seed_158.with(|v| v.as_pointer())) });
}
pub fn test1_159() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    ({ where_1(multiplier_160.with(|v| v.as_pointer())) });
    ({ where_1(increment_161.with(|v| v.as_pointer())) });
    ({ where_1(modulus_162.with(|v| v.as_pointer())) });
    ({ where_1(default_seed_163.with(|v| v.as_pointer())) });
}
pub fn test1_164() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    ({ where_1(multiplier_165.with(|v| v.as_pointer())) });
    ({ where_1(increment_166.with(|v| v.as_pointer())) });
    ({ where_1(modulus_167.with(|v| v.as_pointer())) });
    ({ where_1(default_seed_168.with(|v| v.as_pointer())) });
}
pub fn test1_169() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    ({ where_1(multiplier_170.with(|v| v.as_pointer())) });
    ({ where_1(increment_171.with(|v| v.as_pointer())) });
    ({ where_1(modulus_172.with(|v| v.as_pointer())) });
    ({ where_1(default_seed_173.with(|v| v.as_pointer())) });
}
pub fn test1_174() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    ({ where_1(multiplier_175.with(|v| v.as_pointer())) });
    ({ where_1(increment_176.with(|v| v.as_pointer())) });
    ({ where_1(modulus_177.with(|v| v.as_pointer())) });
    ({ where_1(default_seed_178.with(|v| v.as_pointer())) });
}
pub fn test1_179() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    ({ where_1(multiplier_180.with(|v| v.as_pointer())) });
    ({ where_1(increment_181.with(|v| v.as_pointer())) });
    ({ where_1(modulus_182.with(|v| v.as_pointer())) });
    ({ where_1(default_seed_183.with(|v| v.as_pointer())) });
}
pub fn test1_184() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    ({ where_1(multiplier_185.with(|v| v.as_pointer())) });
    ({ where_1(increment_186.with(|v| v.as_pointer())) });
    ({ where_1(modulus_187.with(|v| v.as_pointer())) });
    ({ where_1(default_seed_188.with(|v| v.as_pointer())) });
}
pub fn test1_189() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    ({ where_1(multiplier_190.with(|v| v.as_pointer())) });
    ({ where_1(increment_191.with(|v| v.as_pointer())) });
    ({ where_1(modulus_192.with(|v| v.as_pointer())) });
    ({ where_1(default_seed_193.with(|v| v.as_pointer())) });
}
pub fn test1_194() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    ({ where_1(multiplier_195.with(|v| v.as_pointer())) });
    ({ where_1(increment_196.with(|v| v.as_pointer())) });
    ({ where_1(modulus_197.with(|v| v.as_pointer())) });
    ({ where_1(default_seed_198.with(|v| v.as_pointer())) });
}
pub fn test1_199() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    ({ where_1(multiplier_200.with(|v| v.as_pointer())) });
    ({ where_1(increment_201.with(|v| v.as_pointer())) });
    ({ where_1(modulus_202.with(|v| v.as_pointer())) });
    ({ where_1(default_seed_203.with(|v| v.as_pointer())) });
}
pub fn test1_204() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    ({ where_2(multiplier_205.with(|v| v.as_pointer())) });
    ({ where_2(increment_206.with(|v| v.as_pointer())) });
    ({ where_2(modulus_207.with(|v| v.as_pointer())) });
    ({ where_2(default_seed_208.with(|v| v.as_pointer())) });
}
pub fn test1_209() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    ({ where_2(multiplier_210.with(|v| v.as_pointer())) });
    ({ where_2(increment_211.with(|v| v.as_pointer())) });
    ({ where_2(modulus_212.with(|v| v.as_pointer())) });
    ({ where_2(default_seed_213.with(|v| v.as_pointer())) });
}
pub fn test1_214() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    ({ where_2(multiplier_215.with(|v| v.as_pointer())) });
    ({ where_2(increment_216.with(|v| v.as_pointer())) });
    ({ where_2(modulus_217.with(|v| v.as_pointer())) });
    ({ where_2(default_seed_218.with(|v| v.as_pointer())) });
}
pub fn test1_219() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    ({ where_2(multiplier_220.with(|v| v.as_pointer())) });
    ({ where_2(increment_221.with(|v| v.as_pointer())) });
    ({ where_2(modulus_222.with(|v| v.as_pointer())) });
    ({ where_2(default_seed_223.with(|v| v.as_pointer())) });
}
pub fn test1_224() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    ({ where_2(multiplier_225.with(|v| v.as_pointer())) });
    ({ where_2(increment_226.with(|v| v.as_pointer())) });
    ({ where_2(modulus_227.with(|v| v.as_pointer())) });
    ({ where_2(default_seed_228.with(|v| v.as_pointer())) });
}
pub fn test1_229() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    ({ where_2(multiplier_230.with(|v| v.as_pointer())) });
    ({ where_2(increment_231.with(|v| v.as_pointer())) });
    ({ where_2(modulus_232.with(|v| v.as_pointer())) });
    ({ where_2(default_seed_233.with(|v| v.as_pointer())) });
}
pub fn test1_234() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    ({ where_2(multiplier_235.with(|v| v.as_pointer())) });
    ({ where_2(increment_236.with(|v| v.as_pointer())) });
    ({ where_2(modulus_237.with(|v| v.as_pointer())) });
    ({ where_2(default_seed_238.with(|v| v.as_pointer())) });
}
pub fn test1_239() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    ({ where_2(multiplier_240.with(|v| v.as_pointer())) });
    ({ where_2(increment_241.with(|v| v.as_pointer())) });
    ({ where_2(modulus_242.with(|v| v.as_pointer())) });
    ({ where_2(default_seed_243.with(|v| v.as_pointer())) });
}
pub fn test1_244() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    ({ where_2(multiplier_245.with(|v| v.as_pointer())) });
    ({ where_2(increment_246.with(|v| v.as_pointer())) });
    ({ where_2(modulus_247.with(|v| v.as_pointer())) });
    ({ where_2(default_seed_248.with(|v| v.as_pointer())) });
}
pub fn test1_249() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    ({ where_2(multiplier_250.with(|v| v.as_pointer())) });
    ({ where_2(increment_251.with(|v| v.as_pointer())) });
    ({ where_2(modulus_252.with(|v| v.as_pointer())) });
    ({ where_2(default_seed_253.with(|v| v.as_pointer())) });
}
pub fn test1_254() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    ({ where_2(multiplier_255.with(|v| v.as_pointer())) });
    ({ where_2(increment_256.with(|v| v.as_pointer())) });
    ({ where_2(modulus_257.with(|v| v.as_pointer())) });
    ({ where_2(default_seed_258.with(|v| v.as_pointer())) });
}
pub fn test1_259() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    ({ where_2(multiplier_260.with(|v| v.as_pointer())) });
    ({ where_2(increment_261.with(|v| v.as_pointer())) });
    ({ where_2(modulus_262.with(|v| v.as_pointer())) });
    ({ where_2(default_seed_263.with(|v| v.as_pointer())) });
}
pub fn test1_264() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    ({ where_2(multiplier_265.with(|v| v.as_pointer())) });
    ({ where_2(increment_266.with(|v| v.as_pointer())) });
    ({ where_2(modulus_267.with(|v| v.as_pointer())) });
    ({ where_2(default_seed_268.with(|v| v.as_pointer())) });
}
pub fn test1_269() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    ({ where_2(multiplier_270.with(|v| v.as_pointer())) });
    ({ where_2(increment_271.with(|v| v.as_pointer())) });
    ({ where_2(modulus_272.with(|v| v.as_pointer())) });
    ({ where_2(default_seed_273.with(|v| v.as_pointer())) });
}
pub fn test1_274() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    ({ where_2(multiplier_275.with(|v| v.as_pointer())) });
    ({ where_2(increment_276.with(|v| v.as_pointer())) });
    ({ where_2(modulus_277.with(|v| v.as_pointer())) });
    ({ where_2(default_seed_278.with(|v| v.as_pointer())) });
}
pub fn test1_279() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    ({ where_2(multiplier_280.with(|v| v.as_pointer())) });
    ({ where_2(increment_281.with(|v| v.as_pointer())) });
    ({ where_2(modulus_282.with(|v| v.as_pointer())) });
    ({ where_2(default_seed_283.with(|v| v.as_pointer())) });
}
pub fn test1_284() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    ({ where_2(multiplier_285.with(|v| v.as_pointer())) });
    ({ where_2(increment_286.with(|v| v.as_pointer())) });
    ({ where_2(modulus_287.with(|v| v.as_pointer())) });
    ({ where_2(default_seed_288.with(|v| v.as_pointer())) });
}
pub fn test1_289() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    ({ where_2(multiplier_290.with(|v| v.as_pointer())) });
    ({ where_2(increment_291.with(|v| v.as_pointer())) });
    ({ where_2(modulus_292.with(|v| v.as_pointer())) });
    ({ where_2(default_seed_293.with(|v| v.as_pointer())) });
}
pub fn test1_294() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    ({ where_2(multiplier_295.with(|v| v.as_pointer())) });
    ({ where_2(increment_296.with(|v| v.as_pointer())) });
    ({ where_2(modulus_297.with(|v| v.as_pointer())) });
    ({ where_2(default_seed_298.with(|v| v.as_pointer())) });
}
pub fn test1_299() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    ({ where_2(multiplier_300.with(|v| v.as_pointer())) });
    ({ where_2(increment_301.with(|v| v.as_pointer())) });
    ({ where_2(modulus_302.with(|v| v.as_pointer())) });
    ({ where_2(default_seed_303.with(|v| v.as_pointer())) });
}
pub fn test1_304() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    ({ where_3(multiplier_305.with(|v| v.as_pointer())) });
    ({ where_3(increment_306.with(|v| v.as_pointer())) });
    ({ where_3(modulus_307.with(|v| v.as_pointer())) });
    ({ where_3(default_seed_308.with(|v| v.as_pointer())) });
}
pub fn test1_309() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    ({ where_3(multiplier_310.with(|v| v.as_pointer())) });
    ({ where_3(increment_311.with(|v| v.as_pointer())) });
    ({ where_3(modulus_312.with(|v| v.as_pointer())) });
    ({ where_3(default_seed_313.with(|v| v.as_pointer())) });
}
pub fn test1_314() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    ({ where_3(multiplier_315.with(|v| v.as_pointer())) });
    ({ where_3(increment_316.with(|v| v.as_pointer())) });
    ({ where_3(modulus_317.with(|v| v.as_pointer())) });
    ({ where_3(default_seed_318.with(|v| v.as_pointer())) });
}
pub fn test1_319() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    ({ where_3(multiplier_320.with(|v| v.as_pointer())) });
    ({ where_3(increment_321.with(|v| v.as_pointer())) });
    ({ where_3(modulus_322.with(|v| v.as_pointer())) });
    ({ where_3(default_seed_323.with(|v| v.as_pointer())) });
}
pub fn test1_324() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    ({ where_3(multiplier_325.with(|v| v.as_pointer())) });
    ({ where_3(increment_326.with(|v| v.as_pointer())) });
    ({ where_3(modulus_327.with(|v| v.as_pointer())) });
    ({ where_3(default_seed_328.with(|v| v.as_pointer())) });
}
pub fn test1_329() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    ({ where_3(multiplier_330.with(|v| v.as_pointer())) });
    ({ where_3(increment_331.with(|v| v.as_pointer())) });
    ({ where_3(modulus_332.with(|v| v.as_pointer())) });
    ({ where_3(default_seed_333.with(|v| v.as_pointer())) });
}
pub fn test1_334() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    ({ where_3(multiplier_335.with(|v| v.as_pointer())) });
    ({ where_3(increment_336.with(|v| v.as_pointer())) });
    ({ where_3(modulus_337.with(|v| v.as_pointer())) });
    ({ where_3(default_seed_338.with(|v| v.as_pointer())) });
}
pub fn test1_339() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    ({ where_3(multiplier_340.with(|v| v.as_pointer())) });
    ({ where_3(increment_341.with(|v| v.as_pointer())) });
    ({ where_3(modulus_342.with(|v| v.as_pointer())) });
    ({ where_3(default_seed_343.with(|v| v.as_pointer())) });
}
pub fn test1_344() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    ({ where_3(multiplier_345.with(|v| v.as_pointer())) });
    ({ where_3(increment_346.with(|v| v.as_pointer())) });
    ({ where_3(modulus_347.with(|v| v.as_pointer())) });
    ({ where_3(default_seed_348.with(|v| v.as_pointer())) });
}
pub fn test1_349() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    ({ where_3(multiplier_350.with(|v| v.as_pointer())) });
    ({ where_3(increment_351.with(|v| v.as_pointer())) });
    ({ where_3(modulus_352.with(|v| v.as_pointer())) });
    ({ where_3(default_seed_353.with(|v| v.as_pointer())) });
}
pub fn test1_354() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    ({ where_3(multiplier_355.with(|v| v.as_pointer())) });
    ({ where_3(increment_356.with(|v| v.as_pointer())) });
    ({ where_3(modulus_357.with(|v| v.as_pointer())) });
    ({ where_3(default_seed_358.with(|v| v.as_pointer())) });
}
pub fn test1_359() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    ({ where_3(multiplier_360.with(|v| v.as_pointer())) });
    ({ where_3(increment_361.with(|v| v.as_pointer())) });
    ({ where_3(modulus_362.with(|v| v.as_pointer())) });
    ({ where_3(default_seed_363.with(|v| v.as_pointer())) });
}
pub fn test1_364() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    ({ where_3(multiplier_365.with(|v| v.as_pointer())) });
    ({ where_3(increment_366.with(|v| v.as_pointer())) });
    ({ where_3(modulus_367.with(|v| v.as_pointer())) });
    ({ where_3(default_seed_368.with(|v| v.as_pointer())) });
}
pub fn test1_369() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    ({ where_3(multiplier_370.with(|v| v.as_pointer())) });
    ({ where_3(increment_371.with(|v| v.as_pointer())) });
    ({ where_3(modulus_372.with(|v| v.as_pointer())) });
    ({ where_3(default_seed_373.with(|v| v.as_pointer())) });
}
pub fn test1_374() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    ({ where_3(multiplier_375.with(|v| v.as_pointer())) });
    ({ where_3(increment_376.with(|v| v.as_pointer())) });
    ({ where_3(modulus_377.with(|v| v.as_pointer())) });
    ({ where_3(default_seed_378.with(|v| v.as_pointer())) });
}
pub fn test1_379() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    ({ where_3(multiplier_380.with(|v| v.as_pointer())) });
    ({ where_3(increment_381.with(|v| v.as_pointer())) });
    ({ where_3(modulus_382.with(|v| v.as_pointer())) });
    ({ where_3(default_seed_383.with(|v| v.as_pointer())) });
}
pub fn test1_384() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    ({ where_3(multiplier_385.with(|v| v.as_pointer())) });
    ({ where_3(increment_386.with(|v| v.as_pointer())) });
    ({ where_3(modulus_387.with(|v| v.as_pointer())) });
    ({ where_3(default_seed_388.with(|v| v.as_pointer())) });
}
pub fn test1_389() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    ({ where_3(multiplier_390.with(|v| v.as_pointer())) });
    ({ where_3(increment_391.with(|v| v.as_pointer())) });
    ({ where_3(modulus_392.with(|v| v.as_pointer())) });
    ({ where_3(default_seed_393.with(|v| v.as_pointer())) });
}
pub fn test1_394() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    ({ where_3(multiplier_395.with(|v| v.as_pointer())) });
    ({ where_3(increment_396.with(|v| v.as_pointer())) });
    ({ where_3(modulus_397.with(|v| v.as_pointer())) });
    ({ where_3(default_seed_398.with(|v| v.as_pointer())) });
}
pub fn test1_399() {
    const _: () = assert!(true, "(LCE::multiplier == a)");;
    const _: () = assert!(true, "(LCE::increment == c)");;
    const _: () = assert!(true, "(LCE::modulus == m)");;
    const _: () = assert!(true, "(LCE::min() == (c == 0u ? 1u: 0u))");;
    const _: () = assert!(true, "(LCE::max() == result_type(m - 1u))");;
    const _: () = assert!(true, "(LCE::default_seed == 1)");;
    ({ where_3(multiplier_400.with(|v| v.as_pointer())) });
    ({ where_3(increment_401.with(|v| v.as_pointer())) });
    ({ where_3(modulus_402.with(|v| v.as_pointer())) });
    ({ where_3(default_seed_403.with(|v| v.as_pointer())) });
}
pub fn test_404() {
    let mut W: () = (::std::mem::size_of::<()>() as ()).wrapping_mul(8_());
    let M: Value<()> = Rc::new(RefCell::new(-1_()));
    let A: Value<()> = Rc::new(RefCell::new(((1_() << (W / 2_())) - 1_())));
    ({ test1_4() });
    ({ test1_9() });
    ({ test1_14() });
    ({ test1_19() });
    ({ test1_24() });
    ({ test1_29() });
    ({ test1_34() });
    ({ test1_39() });
    ({ test1_44() });
    ({ test1_49() });
    ({ test1_54() });
    ({ test1_59() });
    ({ test1_64() });
    ({ test1_69() });
}
pub fn test_405() {
    let mut W: () = (::std::mem::size_of::<()>() as ()).wrapping_mul(8_());
    let M: Value<()> = Rc::new(RefCell::new(-1_()));
    let A: Value<()> = Rc::new(RefCell::new(
        ((1_() << (W / 2_())) as ()).wrapping_sub(1_()),
    ));
    ({ test1_104() });
    ({ test1_109() });
    ({ test1_114() });
    ({ test1_119() });
    ({ test1_124() });
    ({ test1_129() });
    ({ test1_134() });
    ({ test1_139() });
    ({ test1_144() });
    ({ test1_149() });
    ({ test1_154() });
    ({ test1_159() });
    ({ test1_164() });
    ({ test1_169() });
}
pub fn test_406() {
    let mut W: () = (::std::mem::size_of::<()>() as ()).wrapping_mul(8_());
    let M: Value<()> = Rc::new(RefCell::new(-1_()));
    let A: Value<()> = Rc::new(RefCell::new(
        ((1_() << (W / 2_())) as ()).wrapping_sub(1_()),
    ));
    ({ test1_204() });
    ({ test1_209() });
    ({ test1_214() });
    ({ test1_219() });
    ({ test1_224() });
    ({ test1_229() });
    ({ test1_234() });
    ({ test1_239() });
    ({ test1_244() });
    ({ test1_249() });
    ({ test1_254() });
    ({ test1_259() });
    ({ test1_264() });
    ({ test1_269() });
}
pub fn test_407() {
    let mut W: () = (::std::mem::size_of::<()>() as ()).wrapping_mul(8_());
    let M: Value<()> = Rc::new(RefCell::new(-1_()));
    let A: Value<()> = Rc::new(RefCell::new(
        ((1_() << (W / 2_())) as ()).wrapping_sub(1_()),
    ));
    ({ test1_304() });
    ({ test1_309() });
    ({ test1_314() });
    ({ test1_319() });
    ({ test1_324() });
    ({ test1_329() });
    ({ test1_334() });
    ({ test1_339() });
    ({ test1_344() });
    ({ test1_349() });
    ({ test1_354() });
    ({ test1_359() });
    ({ test1_364() });
    ({ test1_369() });
}
pub fn test_ext_408() {
    let M: Value<()> = Rc::new(RefCell::new(-1_()));
    ({ test1_74() });
    ({ test1_79() });
    ({ test1_84() });
    ({ test1_89() });
    ({ test1_94() });
    ({ test1_99() });
}
pub fn test_ext_409() {
    let M: Value<()> = Rc::new(RefCell::new(-1_()));
    ({ test1_174() });
    ({ test1_179() });
    ({ test1_184() });
    ({ test1_189() });
    ({ test1_194() });
    ({ test1_199() });
}
pub fn test_ext_410() {
    let M: Value<()> = Rc::new(RefCell::new(-1_()));
    ({ test1_274() });
    ({ test1_279() });
    ({ test1_284() });
    ({ test1_289() });
    ({ test1_294() });
    ({ test1_299() });
}
pub fn test_ext_411() {
    let M: Value<()> = Rc::new(RefCell::new(-1_()));
    ({ test1_374() });
    ({ test1_379() });
    ({ test1_384() });
    ({ test1_389() });
    ({ test1_394() });
    ({ test1_399() });
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
    ({ test_404() });
    ({ test_ext_408() });
    ({ test_405() });
    ({ test_ext_409() });
    ({ test_406() });
    ({ test_ext_410() });
    ({ test_407() });
    ({ test_ext_411() });
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
