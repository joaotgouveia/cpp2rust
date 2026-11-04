extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_latin_unicode_0() {
    let u16str: Value<Box<[u16]>> =
        Rc::new(RefCell::new(Box::new([229_u16, 228_u16, 246_u16, 0_u16])));
    let u32str: Value<Box<[u32]>> =
        Rc::new(RefCell::new(Box::new([229_u32, 228_u32, 246_u32, 0_u32])));
    let str: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
        -61_i8, -91_i8, -61_i8, -92_i8, -61_i8, -74_i8, 0_i8,
    ])));
    let u8str: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([
        195_u8, 165_u8, 195_u8, 164_u8, 195_u8, 182_u8, 0_u8,
    ])));
    let wstr: Value<Box<[i32]>> =
        Rc::new(RefCell::new(Box::new([229_i32, 228_i32, 246_i32, 0_i32])));
    {
        let p: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __v: Vec<i8> = (u16str.as_pointer() as Ptr<u16>)
                .path_bytes()
                .into_iter()
                .map(|__b| __b as i8)
                .collect();
            __v.push(0);
            __v
        }));
        assert!(
            match (Rc::new(RefCell::new(unimplemented!())).as_pointer(),) {
                (__tmp_0,) => {
                    (u8str.as_pointer() as Ptr<u8>).with_c_str(|__s| {
                        (*__tmp_0.upgrade().deref())
                            [..(*__tmp_0.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
        assert!(
            match (Rc::new(RefCell::new(unimplemented!())).as_pointer(),) {
                (__tmp_1,) => {
                    (u16str.as_pointer() as Ptr<u16>).with_c_str(|__s| {
                        (*__tmp_1.upgrade().deref())
                            [..(*__tmp_1.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
        assert!(
            match (Rc::new(RefCell::new(unimplemented!())).as_pointer(),) {
                (__tmp_2,) => {
                    (u32str.as_pointer() as Ptr<u32>).with_c_str(|__s| {
                        (*__tmp_2.upgrade().deref())
                            [..(*__tmp_2.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
        assert!(
            match (Rc::new(RefCell::new(unimplemented!())).as_pointer(),) {
                (__tmp_3,) => {
                    (u16str.as_pointer() as Ptr<u16>).with_c_str(|__s| {
                        (*__tmp_3.upgrade().deref())
                            [..(*__tmp_3.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
        assert!(
            match (Rc::new(RefCell::new(unimplemented!())).as_pointer(),) {
                (__tmp_4,) => {
                    (u32str.as_pointer() as Ptr<u32>).with_c_str(|__s| {
                        (*__tmp_4.upgrade().deref())
                            [..(*__tmp_4.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let p: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __v: Vec<i8> = (u32str.as_pointer() as Ptr<u32>)
                .path_bytes()
                .into_iter()
                .map(|__b| __b as i8)
                .collect();
            __v.push(0);
            __v
        }));
        assert!(
            match (Rc::new(RefCell::new(unimplemented!())).as_pointer(),) {
                (__tmp_5,) => {
                    (u8str.as_pointer() as Ptr<u8>).with_c_str(|__s| {
                        (*__tmp_5.upgrade().deref())
                            [..(*__tmp_5.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
        assert!(
            match (Rc::new(RefCell::new(unimplemented!())).as_pointer(),) {
                (__tmp_6,) => {
                    (u16str.as_pointer() as Ptr<u16>).with_c_str(|__s| {
                        (*__tmp_6.upgrade().deref())
                            [..(*__tmp_6.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
        assert!(
            match (Rc::new(RefCell::new(unimplemented!())).as_pointer(),) {
                (__tmp_7,) => {
                    (u32str.as_pointer() as Ptr<u32>).with_c_str(|__s| {
                        (*__tmp_7.upgrade().deref())
                            [..(*__tmp_7.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
        assert!(
            match (Rc::new(RefCell::new(unimplemented!())).as_pointer(),) {
                (__tmp_8,) => {
                    (u16str.as_pointer() as Ptr<u16>).with_c_str(|__s| {
                        (*__tmp_8.upgrade().deref())
                            [..(*__tmp_8.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
        assert!(
            match (Rc::new(RefCell::new(unimplemented!())).as_pointer(),) {
                (__tmp_9,) => {
                    (u32str.as_pointer() as Ptr<u32>).with_c_str(|__s| {
                        (*__tmp_9.upgrade().deref())
                            [..(*__tmp_9.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let p: Value<Vec<i8>> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            match (Rc::new(RefCell::new(unimplemented!())).as_pointer(),) {
                (__tmp_10,) => {
                    (u8str.as_pointer() as Ptr<u8>).with_c_str(|__s| {
                        (*__tmp_10.upgrade().deref())
                            [..(*__tmp_10.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
        assert!(
            match (Rc::new(RefCell::new(unimplemented!())).as_pointer(),) {
                (__tmp_11,) => {
                    (u16str.as_pointer() as Ptr<u16>).with_c_str(|__s| {
                        (*__tmp_11.upgrade().deref())
                            [..(*__tmp_11.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
        assert!(
            match (Rc::new(RefCell::new(unimplemented!())).as_pointer(),) {
                (__tmp_12,) => {
                    (u32str.as_pointer() as Ptr<u32>).with_c_str(|__s| {
                        (*__tmp_12.upgrade().deref())
                            [..(*__tmp_12.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
        assert!(
            match (Rc::new(RefCell::new(unimplemented!())).as_pointer(),) {
                (__tmp_13,) => {
                    (u16str.as_pointer() as Ptr<u16>).with_c_str(|__s| {
                        (*__tmp_13.upgrade().deref())
                            [..(*__tmp_13.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
        assert!(
            match (Rc::new(RefCell::new(unimplemented!())).as_pointer(),) {
                (__tmp_14,) => {
                    (u32str.as_pointer() as Ptr<u32>).with_c_str(|__s| {
                        (*__tmp_14.upgrade().deref())
                            [..(*__tmp_14.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let p: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __v: Vec<i8> = (u8str.as_pointer() as Ptr<u8>)
                .path_bytes()
                .into_iter()
                .map(|__b| __b as i8)
                .collect();
            __v.push(0);
            __v
        }));
        assert!(
            match (Rc::new(RefCell::new(unimplemented!())).as_pointer(),) {
                (__tmp_15,) => {
                    (u8str.as_pointer() as Ptr<u8>).with_c_str(|__s| {
                        (*__tmp_15.upgrade().deref())
                            [..(*__tmp_15.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
        assert!(
            match (Rc::new(RefCell::new(unimplemented!())).as_pointer(),) {
                (__tmp_16,) => {
                    (u16str.as_pointer() as Ptr<u16>).with_c_str(|__s| {
                        (*__tmp_16.upgrade().deref())
                            [..(*__tmp_16.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
        assert!(
            match (Rc::new(RefCell::new(unimplemented!())).as_pointer(),) {
                (__tmp_17,) => {
                    (u32str.as_pointer() as Ptr<u32>).with_c_str(|__s| {
                        (*__tmp_17.upgrade().deref())
                            [..(*__tmp_17.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
        assert!(
            match (Rc::new(RefCell::new(unimplemented!())).as_pointer(),) {
                (__tmp_18,) => {
                    (u8str.as_pointer() as Ptr<u8>).with_c_str(|__s| {
                        (*__tmp_18.upgrade().deref())
                            [..(*__tmp_18.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
        assert!(
            match (Rc::new(RefCell::new(unimplemented!())).as_pointer(),) {
                (__tmp_19,) => {
                    (u16str.as_pointer() as Ptr<u16>).with_c_str(|__s| {
                        (*__tmp_19.upgrade().deref())
                            [..(*__tmp_19.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
        assert!(
            match (Rc::new(RefCell::new(unimplemented!())).as_pointer(),) {
                (__tmp_20,) => {
                    (u32str.as_pointer() as Ptr<u32>).with_c_str(|__s| {
                        (*__tmp_20.upgrade().deref())
                            [..(*__tmp_20.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let p: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __v: Vec<i8> = (u16str.as_pointer() as Ptr<u16>)
                .path_bytes()
                .into_iter()
                .map(|__b| __b as i8)
                .collect();
            __v.push(0);
            __v
        }));
        assert!(
            match (Rc::new(RefCell::new(unimplemented!())).as_pointer(),) {
                (__tmp_21,) => {
                    (u8str.as_pointer() as Ptr<u8>).with_c_str(|__s| {
                        (*__tmp_21.upgrade().deref())
                            [..(*__tmp_21.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let p: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __v: Vec<i8> = (u32str.as_pointer() as Ptr<u32>)
                .path_bytes()
                .into_iter()
                .map(|__b| __b as i8)
                .collect();
            __v.push(0);
            __v
        }));
        assert!(
            match (Rc::new(RefCell::new(unimplemented!())).as_pointer(),) {
                (__tmp_22,) => {
                    (u8str.as_pointer() as Ptr<u8>).with_c_str(|__s| {
                        (*__tmp_22.upgrade().deref())
                            [..(*__tmp_22.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let p: Value<Vec<i8>> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            match (Rc::new(RefCell::new(unimplemented!())).as_pointer(),) {
                (__tmp_23,) => {
                    (u8str.as_pointer() as Ptr<u8>).with_c_str(|__s| {
                        (*__tmp_23.upgrade().deref())
                            [..(*__tmp_23.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let p: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __v: Vec<i8> = (u16str.as_pointer() as Ptr<u16>)
                .path_bytes()
                .into_iter()
                .map(|__b| __b as i8)
                .collect();
            __v.push(0);
            __v
        }));
        assert!(
            match (Rc::new(RefCell::new(unimplemented!())).as_pointer(),) {
                (__tmp_24,) => {
                    (wstr.as_pointer() as Ptr<i32>).with_c_str(|__s| {
                        (*__tmp_24.upgrade().deref())
                            [..(*__tmp_24.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
        assert!(
            match (Rc::new(RefCell::new(unimplemented!())).as_pointer(),) {
                (__tmp_25,) => {
                    (wstr.as_pointer() as Ptr<i32>).with_c_str(|__s| {
                        (*__tmp_25.upgrade().deref())
                            [..(*__tmp_25.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let p: Value<Vec<i8>> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            match (Rc::new(RefCell::new(unimplemented!())).as_pointer(),) {
                (__tmp_26,) => {
                    (wstr.as_pointer() as Ptr<i32>).with_c_str(|__s| {
                        (*__tmp_26.upgrade().deref())
                            [..(*__tmp_26.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
        assert!(
            match (Rc::new(RefCell::new(unimplemented!())).as_pointer(),) {
                (__tmp_27,) => {
                    (wstr.as_pointer() as Ptr<i32>).with_c_str(|__s| {
                        (*__tmp_27.upgrade().deref())
                            [..(*__tmp_27.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let p: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __v: Vec<i8> = (wstr.as_pointer() as Ptr<i32>)
                .path_bytes()
                .into_iter()
                .map(|__b| __b as i8)
                .collect();
            __v.push(0);
            __v
        }));
        assert!(
            match (Rc::new(RefCell::new(unimplemented!())).as_pointer(),) {
                (__tmp_28,) => {
                    (wstr.as_pointer() as Ptr<i32>).with_c_str(|__s| {
                        (*__tmp_28.upgrade().deref())
                            [..(*__tmp_28.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
        assert!(
            match (Rc::new(RefCell::new(unimplemented!())).as_pointer(),) {
                (__tmp_29,) => {
                    (u8str.as_pointer() as Ptr<u8>).with_c_str(|__s| {
                        (*__tmp_29.upgrade().deref())
                            [..(*__tmp_29.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
        assert!(
            match (Rc::new(RefCell::new(unimplemented!())).as_pointer(),) {
                (__tmp_30,) => {
                    (u16str.as_pointer() as Ptr<u16>).with_c_str(|__s| {
                        (*__tmp_30.upgrade().deref())
                            [..(*__tmp_30.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
        assert!(
            match (Rc::new(RefCell::new(unimplemented!())).as_pointer(),) {
                (__tmp_31,) => {
                    (u32str.as_pointer() as Ptr<u32>).with_c_str(|__s| {
                        (*__tmp_31.upgrade().deref())
                            [..(*__tmp_31.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
        assert!(
            match (Rc::new(RefCell::new(unimplemented!())).as_pointer(),) {
                (__tmp_32,) => {
                    (wstr.as_pointer() as Ptr<i32>).with_c_str(|__s| {
                        (*__tmp_32.upgrade().deref())
                            [..(*__tmp_32.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let p: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __v: Vec<i8> = (str.as_pointer() as Ptr<i8>)
                .path_bytes()
                .into_iter()
                .map(|__b| __b as i8)
                .collect();
            __v.push(0);
            __v
        }));
        assert!(
            match (Rc::new(RefCell::new(unimplemented!())).as_pointer(),) {
                (__tmp_33,) => {
                    (str.as_pointer() as Ptr<i8>).with_c_str(|__s| {
                        (*__tmp_33.upgrade().deref())
                            [..(*__tmp_33.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
        assert!(
            match (Rc::new(RefCell::new(unimplemented!())).as_pointer(),) {
                (__tmp_34,) => {
                    (u16str.as_pointer() as Ptr<u16>).with_c_str(|__s| {
                        (*__tmp_34.upgrade().deref())
                            [..(*__tmp_34.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
        assert!(
            match (Rc::new(RefCell::new(unimplemented!())).as_pointer(),) {
                (__tmp_35,) => {
                    (str.as_pointer() as Ptr<i8>).with_c_str(|__s| {
                        (*__tmp_35.upgrade().deref())
                            [..(*__tmp_35.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let p: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __v: Vec<i8> = (u16str.as_pointer() as Ptr<u16>)
                .path_bytes()
                .into_iter()
                .map(|__b| __b as i8)
                .collect();
            __v.push(0);
            __v
        }));
        assert!(
            match (Rc::new(RefCell::new(unimplemented!())).as_pointer(),) {
                (__tmp_36,) => {
                    (str.as_pointer() as Ptr<i8>).with_c_str(|__s| {
                        (*__tmp_36.upgrade().deref())
                            [..(*__tmp_36.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
        assert!(
            match (Rc::new(RefCell::new(unimplemented!())).as_pointer(),) {
                (__tmp_37,) => {
                    (str.as_pointer() as Ptr<i8>).with_c_str(|__s| {
                        (*__tmp_37.upgrade().deref())
                            [..(*__tmp_37.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
}
pub fn test_wide_unicode_1() {
    let u16str: Value<Box<[u16]>> = Rc::new(RefCell::new(Box::new([55297_u16, 56375_u16, 0_u16])));
    let u32str: Value<Box<[u32]>> = Rc::new(RefCell::new(Box::new([66615_u32, 0_u32])));
    let u8str: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([
        240_u8, 144_u8, 144_u8, 183_u8, 0_u8,
    ])));
    let mut str: [i8; 5] = [-16_i8, -112_i8, -112_i8, -73_i8, 0_i8];
    {
        let p: Value<Vec<i8>> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            match (Rc::new(RefCell::new(unimplemented!())).as_pointer(),) {
                (__tmp_38,) => {
                    (u8str.as_pointer() as Ptr<u8>).with_c_str(|__s| {
                        (*__tmp_38.upgrade().deref())
                            [..(*__tmp_38.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
        assert!(
            match (Rc::new(RefCell::new(unimplemented!())).as_pointer(),) {
                (__tmp_39,) => {
                    (u16str.as_pointer() as Ptr<u16>).with_c_str(|__s| {
                        (*__tmp_39.upgrade().deref())
                            [..(*__tmp_39.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
        assert!(
            match (Rc::new(RefCell::new(unimplemented!())).as_pointer(),) {
                (__tmp_40,) => {
                    (u32str.as_pointer() as Ptr<u32>).with_c_str(|__s| {
                        (*__tmp_40.upgrade().deref())
                            [..(*__tmp_40.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let p: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __v: Vec<i8> = (u16str.as_pointer() as Ptr<u16>)
                .path_bytes()
                .into_iter()
                .map(|__b| __b as i8)
                .collect();
            __v.push(0);
            __v
        }));
        assert!(
            match (Rc::new(RefCell::new(unimplemented!())).as_pointer(),) {
                (__tmp_41,) => {
                    (u8str.as_pointer() as Ptr<u8>).with_c_str(|__s| {
                        (*__tmp_41.upgrade().deref())
                            [..(*__tmp_41.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
        assert!(
            match (Rc::new(RefCell::new(unimplemented!())).as_pointer(),) {
                (__tmp_42,) => {
                    (u16str.as_pointer() as Ptr<u16>).with_c_str(|__s| {
                        (*__tmp_42.upgrade().deref())
                            [..(*__tmp_42.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
        assert!(
            match (Rc::new(RefCell::new(unimplemented!())).as_pointer(),) {
                (__tmp_43,) => {
                    (u32str.as_pointer() as Ptr<u32>).with_c_str(|__s| {
                        (*__tmp_43.upgrade().deref())
                            [..(*__tmp_43.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let p: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __v: Vec<i8> = (u32str.as_pointer() as Ptr<u32>)
                .path_bytes()
                .into_iter()
                .map(|__b| __b as i8)
                .collect();
            __v.push(0);
            __v
        }));
        assert!(
            match (Rc::new(RefCell::new(unimplemented!())).as_pointer(),) {
                (__tmp_44,) => {
                    (u8str.as_pointer() as Ptr<u8>).with_c_str(|__s| {
                        (*__tmp_44.upgrade().deref())
                            [..(*__tmp_44.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
        assert!(
            match (Rc::new(RefCell::new(unimplemented!())).as_pointer(),) {
                (__tmp_45,) => {
                    (u16str.as_pointer() as Ptr<u16>).with_c_str(|__s| {
                        (*__tmp_45.upgrade().deref())
                            [..(*__tmp_45.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
        assert!(
            match (Rc::new(RefCell::new(unimplemented!())).as_pointer(),) {
                (__tmp_46,) => {
                    (u32str.as_pointer() as Ptr<u32>).with_c_str(|__s| {
                        (*__tmp_46.upgrade().deref())
                            [..(*__tmp_46.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    let wstr: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([66615_i32, 0_i32])));
    {
        let p: Value<Vec<i8>> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            match (Rc::new(RefCell::new(unimplemented!())).as_pointer(),) {
                (__tmp_47,) => {
                    (wstr.as_pointer() as Ptr<i32>).with_c_str(|__s| {
                        (*__tmp_47.upgrade().deref())
                            [..(*__tmp_47.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let p: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __v: Vec<i8> = (u16str.as_pointer() as Ptr<u16>)
                .path_bytes()
                .into_iter()
                .map(|__b| __b as i8)
                .collect();
            __v.push(0);
            __v
        }));
        assert!(
            match (Rc::new(RefCell::new(unimplemented!())).as_pointer(),) {
                (__tmp_48,) => {
                    (wstr.as_pointer() as Ptr<i32>).with_c_str(|__s| {
                        (*__tmp_48.upgrade().deref())
                            [..(*__tmp_48.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let p: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __v: Vec<i8> = (u32str.as_pointer() as Ptr<u32>)
                .path_bytes()
                .into_iter()
                .map(|__b| __b as i8)
                .collect();
            __v.push(0);
            __v
        }));
        assert!(
            match (Rc::new(RefCell::new(unimplemented!())).as_pointer(),) {
                (__tmp_49,) => {
                    (wstr.as_pointer() as Ptr<i32>).with_c_str(|__s| {
                        (*__tmp_49.upgrade().deref())
                            [..(*__tmp_49.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let p: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __v: Vec<i8> = (wstr.as_pointer() as Ptr<i32>)
                .path_bytes()
                .into_iter()
                .map(|__b| __b as i8)
                .collect();
            __v.push(0);
            __v
        }));
        assert!(
            match (Rc::new(RefCell::new(unimplemented!())).as_pointer(),) {
                (__tmp_50,) => {
                    (u8str.as_pointer() as Ptr<u8>).with_c_str(|__s| {
                        (*__tmp_50.upgrade().deref())
                            [..(*__tmp_50.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
        assert!(
            match (Rc::new(RefCell::new(unimplemented!())).as_pointer(),) {
                (__tmp_51,) => {
                    (u16str.as_pointer() as Ptr<u16>).with_c_str(|__s| {
                        (*__tmp_51.upgrade().deref())
                            [..(*__tmp_51.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
        assert!(
            match (Rc::new(RefCell::new(unimplemented!())).as_pointer(),) {
                (__tmp_52,) => {
                    (u32str.as_pointer() as Ptr<u32>).with_c_str(|__s| {
                        (*__tmp_52.upgrade().deref())
                            [..(*__tmp_52.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
        assert!(
            match (Rc::new(RefCell::new(unimplemented!())).as_pointer(),) {
                (__tmp_53,) => {
                    (wstr.as_pointer() as Ptr<i32>).with_c_str(|__s| {
                        (*__tmp_53.upgrade().deref())
                            [..(*__tmp_53.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
}
pub fn test_append_2() {
    let u16str: Value<Box<[u16]>> = Rc::new(RefCell::new(Box::new([55297_u16, 56375_u16, 0_u16])));
    let u32str: Value<Box<[u32]>> = Rc::new(RefCell::new(Box::new([66615_u32, 0_u32])));
    let u32ref: Value<Box<[u32]>> = Rc::new(RefCell::new(Box::new([
        66615_u32,
        (unimplemented!() as u32),
        66615_u32,
        (unimplemented!() as u32),
        66615_u32,
        0_u32,
    ])));
    let mut str: [i8; 5] = [-16_i8, -112_i8, -112_i8, -73_i8, 0_i8];
    {
        let p: Value<Vec<i8>> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            match (Rc::new(RefCell::new(unimplemented!())).as_pointer(),) {
                (__tmp_54,) => {
                    (u32ref.as_pointer() as Ptr<u32>).with_c_str(|__s| {
                        (*__tmp_54.upgrade().deref())
                            [..(*__tmp_54.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
        unimplemented!();
        assert!(
            match (Rc::new(RefCell::new(unimplemented!())).as_pointer(),) {
                (__tmp_55,) => {
                    (u32ref.as_pointer() as Ptr<u32>).with_c_str(|__s| {
                        (*__tmp_55.upgrade().deref())
                            [..(*__tmp_55.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
        unimplemented!();
        unimplemented!();
        unimplemented!();
        assert!(
            match (Rc::new(RefCell::new(unimplemented!())).as_pointer(),) {
                (__tmp_56,) => {
                    (u32ref.as_pointer() as Ptr<u32>).with_c_str(|__s| {
                        (*__tmp_56.upgrade().deref())
                            [..(*__tmp_56.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    let wstr: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([66615_i32, 0_i32])));
    {
        let p: Value<Vec<i8>> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            match (Rc::new(RefCell::new(unimplemented!())).as_pointer(),) {
                (__tmp_57,) => {
                    (u32ref.as_pointer() as Ptr<u32>).with_c_str(|__s| {
                        (*__tmp_57.upgrade().deref())
                            [..(*__tmp_57.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
        unimplemented!();
        assert!(
            match (Rc::new(RefCell::new(unimplemented!())).as_pointer(),) {
                (__tmp_58,) => {
                    (u32ref.as_pointer() as Ptr<u32>).with_c_str(|__s| {
                        (*__tmp_58.upgrade().deref())
                            [..(*__tmp_58.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
        unimplemented!();
        unimplemented!();
        unimplemented!();
        assert!(
            match (Rc::new(RefCell::new(unimplemented!())).as_pointer(),) {
                (__tmp_59,) => {
                    (u32ref.as_pointer() as Ptr<u32>).with_c_str(|__s| {
                        (*__tmp_59.upgrade().deref())
                            [..(*__tmp_59.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
}
pub fn test_concat_3() {
    let u16str: Value<Box<[u16]>> = Rc::new(RefCell::new(Box::new([55297_u16, 56375_u16, 0_u16])));
    let mut u32str: [u32; 2] = [66615_u32, 0_u32];
    let u32ref: Value<Box<[u32]>> = Rc::new(RefCell::new(Box::new([
        66615_u32, 66615_u32, 66615_u32, 0_u32,
    ])));
    let mut str: [i8; 5] = [-16_i8, -112_i8, -112_i8, -73_i8, 0_i8];
    {
        let p: Value<Vec<i8>> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        assert!(
            match (Rc::new(RefCell::new(unimplemented!())).as_pointer(),) {
                (__tmp_60,) => {
                    (u32ref.as_pointer() as Ptr<u32>).with_c_str(|__s| {
                        (*__tmp_60.upgrade().deref())
                            [..(*__tmp_60.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
        unimplemented!();
        assert!(
            match (Rc::new(RefCell::new(unimplemented!())).as_pointer(),) {
                (__tmp_61,) => {
                    (u32ref.as_pointer() as Ptr<u32>).with_c_str(|__s| {
                        (*__tmp_61.upgrade().deref())
                            [..(*__tmp_61.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    let mut wstr: [i32; 2] = [66615_i32, 0_i32];
    {
        let p: Value<Vec<i8>> = Rc::new(RefCell::new({
            let mut __v: Vec<i8> = (u16str.as_pointer() as Ptr<u16>)
                .path_bytes()
                .into_iter()
                .map(|__b| __b as i8)
                .collect();
            __v.push(0);
            __v
        }));
        unimplemented!();
        unimplemented!();
        assert!(
            match (Rc::new(RefCell::new(unimplemented!())).as_pointer(),) {
                (__tmp_62,) => {
                    (u32ref.as_pointer() as Ptr<u32>).with_c_str(|__s| {
                        (*__tmp_62.upgrade().deref())
                            [..(*__tmp_62.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
        unimplemented!();
        assert!(
            match (Rc::new(RefCell::new(unimplemented!())).as_pointer(),) {
                (__tmp_63,) => {
                    (u32ref.as_pointer() as Ptr<u32>).with_c_str(|__s| {
                        (*__tmp_63.upgrade().deref())
                            [..(*__tmp_63.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
}
pub fn test_append_concat_narrow_4() {
    let u16str: Value<Box<[u16]>> = Rc::new(RefCell::new(Box::new([229_u16, 0_u16])));
    let u32ref_append: Value<Box<[u32]>> = Rc::new(RefCell::new(Box::new([
        229_u32,
        (unimplemented!() as u32),
        229_u32,
        0_u32,
    ])));
    let u32ref_concat: Value<Box<[u32]>> =
        Rc::new(RefCell::new(Box::new([229_u32, 229_u32, 0_u32])));
    {
        let u8str: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([195_u8, 165_u8, 0_u8])));
        let p: Value<Vec<i8>> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            match (Rc::new(RefCell::new(unimplemented!())).as_pointer(),) {
                (__tmp_64,) => {
                    (u32ref_append.as_pointer() as Ptr<u32>).with_c_str(|__s| {
                        (*__tmp_64.upgrade().deref())
                            [..(*__tmp_64.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
        unimplemented!();
        assert!(
            match (Rc::new(RefCell::new(unimplemented!())).as_pointer(),) {
                (__tmp_65,) => {
                    (u32ref_append.as_pointer() as Ptr<u32>).with_c_str(|__s| {
                        (*__tmp_65.upgrade().deref())
                            [..(*__tmp_65.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
        unimplemented!();
        unimplemented!();
        assert!(
            match (Rc::new(RefCell::new(unimplemented!())).as_pointer(),) {
                (__tmp_66,) => {
                    (u32ref_append.as_pointer() as Ptr<u32>).with_c_str(|__s| {
                        (*__tmp_66.upgrade().deref())
                            [..(*__tmp_66.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
        unimplemented!();
        assert!(
            match (Rc::new(RefCell::new(unimplemented!())).as_pointer(),) {
                (__tmp_67,) => {
                    (u32ref_concat.as_pointer() as Ptr<u32>).with_c_str(|__s| {
                        (*__tmp_67.upgrade().deref())
                            [..(*__tmp_67.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
        unimplemented!();
        unimplemented!();
        assert!(
            match (Rc::new(RefCell::new(unimplemented!())).as_pointer(),) {
                (__tmp_68,) => {
                    (u32ref_concat.as_pointer() as Ptr<u32>).with_c_str(|__s| {
                        (*__tmp_68.upgrade().deref())
                            [..(*__tmp_68.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let str: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([-61_i8, -91_i8, 0_i8])));
        let p: Value<Vec<i8>> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            match (Rc::new(RefCell::new(unimplemented!())).as_pointer(),) {
                (__tmp_69,) => {
                    (u32ref_append.as_pointer() as Ptr<u32>).with_c_str(|__s| {
                        (*__tmp_69.upgrade().deref())
                            [..(*__tmp_69.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
        unimplemented!();
        assert!(
            match (Rc::new(RefCell::new(unimplemented!())).as_pointer(),) {
                (__tmp_70,) => {
                    (u32ref_append.as_pointer() as Ptr<u32>).with_c_str(|__s| {
                        (*__tmp_70.upgrade().deref())
                            [..(*__tmp_70.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
        unimplemented!();
        unimplemented!();
        assert!(
            match (Rc::new(RefCell::new(unimplemented!())).as_pointer(),) {
                (__tmp_71,) => {
                    (u32ref_append.as_pointer() as Ptr<u32>).with_c_str(|__s| {
                        (*__tmp_71.upgrade().deref())
                            [..(*__tmp_71.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
        unimplemented!();
        assert!(
            match (Rc::new(RefCell::new(unimplemented!())).as_pointer(),) {
                (__tmp_72,) => {
                    (u32ref_concat.as_pointer() as Ptr<u32>).with_c_str(|__s| {
                        (*__tmp_72.upgrade().deref())
                            [..(*__tmp_72.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
        unimplemented!();
        unimplemented!();
        assert!(
            match (Rc::new(RefCell::new(unimplemented!())).as_pointer(),) {
                (__tmp_73,) => {
                    (u32ref_concat.as_pointer() as Ptr<u32>).with_c_str(|__s| {
                        (*__tmp_73.upgrade().deref())
                            [..(*__tmp_73.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> i32 {
    ({ test_latin_unicode_0() });
    ({ test_wide_unicode_1() });
    ({ test_append_2() });
    ({ test_concat_3() });
    ({ test_append_concat_narrow_4() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
