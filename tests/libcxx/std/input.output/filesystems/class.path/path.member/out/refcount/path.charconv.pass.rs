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
            (u8str.as_pointer() as Ptr::<u8>).with_c_str(|__s| unimplemented!()
                [..unimplemented!().len().saturating_sub(1)]
                == *__s)
        );
        assert!(
            (u16str.as_pointer() as Ptr::<u16>).with_c_str(|__s| unimplemented!()
                [..unimplemented!().len().saturating_sub(1)]
                == *__s)
        );
        assert!(
            (u32str.as_pointer() as Ptr::<u32>).with_c_str(|__s| unimplemented!()
                [..unimplemented!().len().saturating_sub(1)]
                == *__s)
        );
        assert!(
            (u16str.as_pointer() as Ptr::<u16>).with_c_str(|__s| unimplemented!()
                [..unimplemented!().len().saturating_sub(1)]
                == *__s)
        );
        assert!(
            (u32str.as_pointer() as Ptr::<u32>).with_c_str(|__s| unimplemented!()
                [..unimplemented!().len().saturating_sub(1)]
                == *__s)
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
            (u8str.as_pointer() as Ptr::<u8>).with_c_str(|__s| unimplemented!()
                [..unimplemented!().len().saturating_sub(1)]
                == *__s)
        );
        assert!(
            (u16str.as_pointer() as Ptr::<u16>).with_c_str(|__s| unimplemented!()
                [..unimplemented!().len().saturating_sub(1)]
                == *__s)
        );
        assert!(
            (u32str.as_pointer() as Ptr::<u32>).with_c_str(|__s| unimplemented!()
                [..unimplemented!().len().saturating_sub(1)]
                == *__s)
        );
        assert!(
            (u16str.as_pointer() as Ptr::<u16>).with_c_str(|__s| unimplemented!()
                [..unimplemented!().len().saturating_sub(1)]
                == *__s)
        );
        assert!(
            (u32str.as_pointer() as Ptr::<u32>).with_c_str(|__s| unimplemented!()
                [..unimplemented!().len().saturating_sub(1)]
                == *__s)
        );
    }
    {
        let p: Value<Vec<i8>> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            (u8str.as_pointer() as Ptr::<u8>).with_c_str(|__s| unimplemented!()
                [..unimplemented!().len().saturating_sub(1)]
                == *__s)
        );
        assert!(
            (u16str.as_pointer() as Ptr::<u16>).with_c_str(|__s| unimplemented!()
                [..unimplemented!().len().saturating_sub(1)]
                == *__s)
        );
        assert!(
            (u32str.as_pointer() as Ptr::<u32>).with_c_str(|__s| unimplemented!()
                [..unimplemented!().len().saturating_sub(1)]
                == *__s)
        );
        assert!(
            (u16str.as_pointer() as Ptr::<u16>).with_c_str(|__s| unimplemented!()
                [..unimplemented!().len().saturating_sub(1)]
                == *__s)
        );
        assert!(
            (u32str.as_pointer() as Ptr::<u32>).with_c_str(|__s| unimplemented!()
                [..unimplemented!().len().saturating_sub(1)]
                == *__s)
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
            (u8str.as_pointer() as Ptr::<u8>).with_c_str(|__s| unimplemented!()
                [..unimplemented!().len().saturating_sub(1)]
                == *__s)
        );
        assert!(
            (u16str.as_pointer() as Ptr::<u16>).with_c_str(|__s| unimplemented!()
                [..unimplemented!().len().saturating_sub(1)]
                == *__s)
        );
        assert!(
            (u32str.as_pointer() as Ptr::<u32>).with_c_str(|__s| unimplemented!()
                [..unimplemented!().len().saturating_sub(1)]
                == *__s)
        );
        assert!(
            (u8str.as_pointer() as Ptr::<u8>).with_c_str(|__s| unimplemented!()
                [..unimplemented!().len().saturating_sub(1)]
                == *__s)
        );
        assert!(
            (u16str.as_pointer() as Ptr::<u16>).with_c_str(|__s| unimplemented!()
                [..unimplemented!().len().saturating_sub(1)]
                == *__s)
        );
        assert!(
            (u32str.as_pointer() as Ptr::<u32>).with_c_str(|__s| unimplemented!()
                [..unimplemented!().len().saturating_sub(1)]
                == *__s)
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
            (u8str.as_pointer() as Ptr::<u8>).with_c_str(|__s| unimplemented!()
                [..unimplemented!().len().saturating_sub(1)]
                == *__s)
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
            (u8str.as_pointer() as Ptr::<u8>).with_c_str(|__s| unimplemented!()
                [..unimplemented!().len().saturating_sub(1)]
                == *__s)
        );
    }
    {
        let p: Value<Vec<i8>> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            (u8str.as_pointer() as Ptr::<u8>).with_c_str(|__s| unimplemented!()
                [..unimplemented!().len().saturating_sub(1)]
                == *__s)
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
            (wstr.as_pointer() as Ptr::<i32>).with_c_str(|__s| unimplemented!()
                [..unimplemented!().len().saturating_sub(1)]
                == *__s)
        );
        assert!(
            (wstr.as_pointer() as Ptr::<i32>).with_c_str(|__s| unimplemented!()
                [..unimplemented!().len().saturating_sub(1)]
                == *__s)
        );
    }
    {
        let p: Value<Vec<i8>> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            (wstr.as_pointer() as Ptr::<i32>).with_c_str(|__s| unimplemented!()
                [..unimplemented!().len().saturating_sub(1)]
                == *__s)
        );
        assert!(
            (wstr.as_pointer() as Ptr::<i32>).with_c_str(|__s| unimplemented!()
                [..unimplemented!().len().saturating_sub(1)]
                == *__s)
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
            (wstr.as_pointer() as Ptr::<i32>).with_c_str(|__s| unimplemented!()
                [..unimplemented!().len().saturating_sub(1)]
                == *__s)
        );
        assert!(
            (u8str.as_pointer() as Ptr::<u8>).with_c_str(|__s| unimplemented!()
                [..unimplemented!().len().saturating_sub(1)]
                == *__s)
        );
        assert!(
            (u16str.as_pointer() as Ptr::<u16>).with_c_str(|__s| unimplemented!()
                [..unimplemented!().len().saturating_sub(1)]
                == *__s)
        );
        assert!(
            (u32str.as_pointer() as Ptr::<u32>).with_c_str(|__s| unimplemented!()
                [..unimplemented!().len().saturating_sub(1)]
                == *__s)
        );
        assert!(
            (wstr.as_pointer() as Ptr::<i32>).with_c_str(|__s| unimplemented!()
                [..unimplemented!().len().saturating_sub(1)]
                == *__s)
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
            (str.as_pointer() as Ptr::<i8>).with_c_str(|__s| unimplemented!()
                [..unimplemented!().len().saturating_sub(1)]
                == *__s)
        );
        assert!(
            (u16str.as_pointer() as Ptr::<u16>).with_c_str(|__s| unimplemented!()
                [..unimplemented!().len().saturating_sub(1)]
                == *__s)
        );
        assert!(
            (str.as_pointer() as Ptr::<i8>).with_c_str(|__s| unimplemented!()
                [..unimplemented!().len().saturating_sub(1)]
                == *__s)
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
            (str.as_pointer() as Ptr::<i8>).with_c_str(|__s| unimplemented!()
                [..unimplemented!().len().saturating_sub(1)]
                == *__s)
        );
        assert!(
            (str.as_pointer() as Ptr::<i8>).with_c_str(|__s| unimplemented!()
                [..unimplemented!().len().saturating_sub(1)]
                == *__s)
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
            (u8str.as_pointer() as Ptr::<u8>).with_c_str(|__s| unimplemented!()
                [..unimplemented!().len().saturating_sub(1)]
                == *__s)
        );
        assert!(
            (u16str.as_pointer() as Ptr::<u16>).with_c_str(|__s| unimplemented!()
                [..unimplemented!().len().saturating_sub(1)]
                == *__s)
        );
        assert!(
            (u32str.as_pointer() as Ptr::<u32>).with_c_str(|__s| unimplemented!()
                [..unimplemented!().len().saturating_sub(1)]
                == *__s)
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
            (u8str.as_pointer() as Ptr::<u8>).with_c_str(|__s| unimplemented!()
                [..unimplemented!().len().saturating_sub(1)]
                == *__s)
        );
        assert!(
            (u16str.as_pointer() as Ptr::<u16>).with_c_str(|__s| unimplemented!()
                [..unimplemented!().len().saturating_sub(1)]
                == *__s)
        );
        assert!(
            (u32str.as_pointer() as Ptr::<u32>).with_c_str(|__s| unimplemented!()
                [..unimplemented!().len().saturating_sub(1)]
                == *__s)
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
            (u8str.as_pointer() as Ptr::<u8>).with_c_str(|__s| unimplemented!()
                [..unimplemented!().len().saturating_sub(1)]
                == *__s)
        );
        assert!(
            (u16str.as_pointer() as Ptr::<u16>).with_c_str(|__s| unimplemented!()
                [..unimplemented!().len().saturating_sub(1)]
                == *__s)
        );
        assert!(
            (u32str.as_pointer() as Ptr::<u32>).with_c_str(|__s| unimplemented!()
                [..unimplemented!().len().saturating_sub(1)]
                == *__s)
        );
    }
    let wstr: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([66615_i32, 0_i32])));
    {
        let p: Value<Vec<i8>> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            (wstr.as_pointer() as Ptr::<i32>).with_c_str(|__s| unimplemented!()
                [..unimplemented!().len().saturating_sub(1)]
                == *__s)
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
            (wstr.as_pointer() as Ptr::<i32>).with_c_str(|__s| unimplemented!()
                [..unimplemented!().len().saturating_sub(1)]
                == *__s)
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
            (wstr.as_pointer() as Ptr::<i32>).with_c_str(|__s| unimplemented!()
                [..unimplemented!().len().saturating_sub(1)]
                == *__s)
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
            (u8str.as_pointer() as Ptr::<u8>).with_c_str(|__s| unimplemented!()
                [..unimplemented!().len().saturating_sub(1)]
                == *__s)
        );
        assert!(
            (u16str.as_pointer() as Ptr::<u16>).with_c_str(|__s| unimplemented!()
                [..unimplemented!().len().saturating_sub(1)]
                == *__s)
        );
        assert!(
            (u32str.as_pointer() as Ptr::<u32>).with_c_str(|__s| unimplemented!()
                [..unimplemented!().len().saturating_sub(1)]
                == *__s)
        );
        assert!(
            (wstr.as_pointer() as Ptr::<i32>).with_c_str(|__s| unimplemented!()
                [..unimplemented!().len().saturating_sub(1)]
                == *__s)
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
            (u32ref.as_pointer() as Ptr::<u32>).with_c_str(|__s| unimplemented!()
                [..unimplemented!().len().saturating_sub(1)]
                == *__s)
        );
        unimplemented!();
        assert!(
            (u32ref.as_pointer() as Ptr::<u32>).with_c_str(|__s| unimplemented!()
                [..unimplemented!().len().saturating_sub(1)]
                == *__s)
        );
        unimplemented!();
        unimplemented!();
        unimplemented!();
        assert!(
            (u32ref.as_pointer() as Ptr::<u32>).with_c_str(|__s| unimplemented!()
                [..unimplemented!().len().saturating_sub(1)]
                == *__s)
        );
    }
    let wstr: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([66615_i32, 0_i32])));
    {
        let p: Value<Vec<i8>> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            (u32ref.as_pointer() as Ptr::<u32>).with_c_str(|__s| unimplemented!()
                [..unimplemented!().len().saturating_sub(1)]
                == *__s)
        );
        unimplemented!();
        assert!(
            (u32ref.as_pointer() as Ptr::<u32>).with_c_str(|__s| unimplemented!()
                [..unimplemented!().len().saturating_sub(1)]
                == *__s)
        );
        unimplemented!();
        unimplemented!();
        unimplemented!();
        assert!(
            (u32ref.as_pointer() as Ptr::<u32>).with_c_str(|__s| unimplemented!()
                [..unimplemented!().len().saturating_sub(1)]
                == *__s)
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
            (u32ref.as_pointer() as Ptr::<u32>).with_c_str(|__s| unimplemented!()
                [..unimplemented!().len().saturating_sub(1)]
                == *__s)
        );
        unimplemented!();
        assert!(
            (u32ref.as_pointer() as Ptr::<u32>).with_c_str(|__s| unimplemented!()
                [..unimplemented!().len().saturating_sub(1)]
                == *__s)
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
            (u32ref.as_pointer() as Ptr::<u32>).with_c_str(|__s| unimplemented!()
                [..unimplemented!().len().saturating_sub(1)]
                == *__s)
        );
        unimplemented!();
        assert!(
            (u32ref.as_pointer() as Ptr::<u32>).with_c_str(|__s| unimplemented!()
                [..unimplemented!().len().saturating_sub(1)]
                == *__s)
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
            (u32ref_append.as_pointer() as Ptr::<u32>).with_c_str(|__s| unimplemented!()
                [..unimplemented!().len().saturating_sub(1)]
                == *__s)
        );
        unimplemented!();
        assert!(
            (u32ref_append.as_pointer() as Ptr::<u32>).with_c_str(|__s| unimplemented!()
                [..unimplemented!().len().saturating_sub(1)]
                == *__s)
        );
        unimplemented!();
        unimplemented!();
        assert!(
            (u32ref_append.as_pointer() as Ptr::<u32>).with_c_str(|__s| unimplemented!()
                [..unimplemented!().len().saturating_sub(1)]
                == *__s)
        );
        unimplemented!();
        assert!(
            (u32ref_concat.as_pointer() as Ptr::<u32>).with_c_str(|__s| unimplemented!()
                [..unimplemented!().len().saturating_sub(1)]
                == *__s)
        );
        unimplemented!();
        unimplemented!();
        assert!(
            (u32ref_concat.as_pointer() as Ptr::<u32>).with_c_str(|__s| unimplemented!()
                [..unimplemented!().len().saturating_sub(1)]
                == *__s)
        );
    }
    {
        let str: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([-61_i8, -91_i8, 0_i8])));
        let p: Value<Vec<i8>> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            (u32ref_append.as_pointer() as Ptr::<u32>).with_c_str(|__s| unimplemented!()
                [..unimplemented!().len().saturating_sub(1)]
                == *__s)
        );
        unimplemented!();
        assert!(
            (u32ref_append.as_pointer() as Ptr::<u32>).with_c_str(|__s| unimplemented!()
                [..unimplemented!().len().saturating_sub(1)]
                == *__s)
        );
        unimplemented!();
        unimplemented!();
        assert!(
            (u32ref_append.as_pointer() as Ptr::<u32>).with_c_str(|__s| unimplemented!()
                [..unimplemented!().len().saturating_sub(1)]
                == *__s)
        );
        unimplemented!();
        assert!(
            (u32ref_concat.as_pointer() as Ptr::<u32>).with_c_str(|__s| unimplemented!()
                [..unimplemented!().len().saturating_sub(1)]
                == *__s)
        );
        unimplemented!();
        unimplemented!();
        assert!(
            (u32ref_concat.as_pointer() as Ptr::<u32>).with_c_str(|__s| unimplemented!()
                [..unimplemented!().len().saturating_sub(1)]
                == *__s)
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
