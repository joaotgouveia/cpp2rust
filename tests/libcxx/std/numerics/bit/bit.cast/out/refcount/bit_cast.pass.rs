extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_roundtrip_through_buffer_0(from: bool) {
    let from: Value<bool> = Rc::new(RefCell::new(from));
    let middle: Value<Buffer_1> = Rc::new(RefCell::new(unimplemented!()));
    let to: Value<bool> = Rc::new(RefCell::new(unimplemented!()));
    let middle2: Value<Buffer_1> = Rc::new(RefCell::new(unimplemented!()));
    assert!(
        (((((*from.borrow()) as i32) == ((*to.borrow()) as i32)) as i32)
            == ((((*from.borrow()) as i32) == ((*from.borrow()) as i32)) as i32))
    );
    {
        assert!(
            (((from.as_pointer()) as Ptr<bool>).to_any().memcmp(
                &((middle.as_pointer()) as Ptr<Buffer_1>).to_any(),
                ::std::mem::size_of::<bool>()
            ) == 0)
        );
        assert!(
            (((to.as_pointer()) as Ptr<bool>).to_any().memcmp(
                &((middle.as_pointer()) as Ptr<Buffer_1>).to_any(),
                ::std::mem::size_of::<bool>()
            ) == 0)
        );
        assert!(
            (((middle.as_pointer()) as Ptr<Buffer_1>).to_any().memcmp(
                &((middle2.as_pointer()) as Ptr<Buffer_1>).to_any(),
                ::std::mem::size_of::<bool>()
            ) == 0)
        );
    }
}
pub fn test_roundtrip_through_buffer_2(from: i8) {
    let from: Value<i8> = Rc::new(RefCell::new(from));
    let middle: Value<Buffer_3> = Rc::new(RefCell::new(unimplemented!()));
    let to: Value<i8> = Rc::new(RefCell::new(unimplemented!()));
    let middle2: Value<Buffer_3> = Rc::new(RefCell::new(unimplemented!()));
    assert!(
        (((((*from.borrow()) as i32) == ((*to.borrow()) as i32)) as i32)
            == ((((*from.borrow()) as i32) == ((*from.borrow()) as i32)) as i32))
    );
    {
        assert!(
            (((from.as_pointer()) as Ptr<i8>).to_any().memcmp(
                &((middle.as_pointer()) as Ptr<Buffer_3>).to_any(),
                ::std::mem::size_of::<i8>()
            ) == 0)
        );
        assert!(
            (((to.as_pointer()) as Ptr<i8>).to_any().memcmp(
                &((middle.as_pointer()) as Ptr<Buffer_3>).to_any(),
                ::std::mem::size_of::<i8>()
            ) == 0)
        );
        assert!(
            (((middle.as_pointer()) as Ptr<Buffer_3>).to_any().memcmp(
                &((middle2.as_pointer()) as Ptr<Buffer_3>).to_any(),
                ::std::mem::size_of::<i8>()
            ) == 0)
        );
    }
}
pub fn test_roundtrip_through_buffer_4(from: i8) {
    let from: Value<i8> = Rc::new(RefCell::new(from));
    let middle: Value<Buffer_5> = Rc::new(RefCell::new(unimplemented!()));
    let to: Value<i8> = Rc::new(RefCell::new(unimplemented!()));
    let middle2: Value<Buffer_5> = Rc::new(RefCell::new(unimplemented!()));
    assert!(
        (((((*from.borrow()) as i32) == ((*to.borrow()) as i32)) as i32)
            == ((((*from.borrow()) as i32) == ((*from.borrow()) as i32)) as i32))
    );
    {
        assert!(
            (((from.as_pointer()) as Ptr<i8>).to_any().memcmp(
                &((middle.as_pointer()) as Ptr<Buffer_5>).to_any(),
                ::std::mem::size_of::<i8>()
            ) == 0)
        );
        assert!(
            (((to.as_pointer()) as Ptr<i8>).to_any().memcmp(
                &((middle.as_pointer()) as Ptr<Buffer_5>).to_any(),
                ::std::mem::size_of::<i8>()
            ) == 0)
        );
        assert!(
            (((middle.as_pointer()) as Ptr<Buffer_5>).to_any().memcmp(
                &((middle2.as_pointer()) as Ptr<Buffer_5>).to_any(),
                ::std::mem::size_of::<i8>()
            ) == 0)
        );
    }
}
pub fn test_roundtrip_through_buffer_6(from: i16) {
    let from: Value<i16> = Rc::new(RefCell::new(from));
    let middle: Value<Buffer_7> = Rc::new(RefCell::new(unimplemented!()));
    let to: Value<i16> = Rc::new(RefCell::new(unimplemented!()));
    let middle2: Value<Buffer_7> = Rc::new(RefCell::new(unimplemented!()));
    assert!(
        (((((*from.borrow()) as i32) == ((*to.borrow()) as i32)) as i32)
            == ((((*from.borrow()) as i32) == ((*from.borrow()) as i32)) as i32))
    );
    {
        assert!(
            (((from.as_pointer()) as Ptr<i16>).to_any().memcmp(
                &((middle.as_pointer()) as Ptr<Buffer_7>).to_any(),
                ::std::mem::size_of::<i16>()
            ) == 0)
        );
        assert!(
            (((to.as_pointer()) as Ptr<i16>).to_any().memcmp(
                &((middle.as_pointer()) as Ptr<Buffer_7>).to_any(),
                ::std::mem::size_of::<i16>()
            ) == 0)
        );
        assert!(
            (((middle.as_pointer()) as Ptr<Buffer_7>).to_any().memcmp(
                &((middle2.as_pointer()) as Ptr<Buffer_7>).to_any(),
                ::std::mem::size_of::<i16>()
            ) == 0)
        );
    }
}
pub fn test_roundtrip_through_buffer_8(from: i32) {
    let from: Value<i32> = Rc::new(RefCell::new(from));
    let middle: Value<Buffer_9> = Rc::new(RefCell::new(unimplemented!()));
    let to: Value<i32> = Rc::new(RefCell::new(unimplemented!()));
    let middle2: Value<Buffer_9> = Rc::new(RefCell::new(unimplemented!()));
    assert!(
        ((((*from.borrow()) == (*to.borrow())) as i32)
            == (((*from.borrow()) == (*from.borrow())) as i32))
    );
    {
        assert!(
            (((from.as_pointer()) as Ptr<i32>).to_any().memcmp(
                &((middle.as_pointer()) as Ptr<Buffer_9>).to_any(),
                ::std::mem::size_of::<i32>()
            ) == 0)
        );
        assert!(
            (((to.as_pointer()) as Ptr<i32>).to_any().memcmp(
                &((middle.as_pointer()) as Ptr<Buffer_9>).to_any(),
                ::std::mem::size_of::<i32>()
            ) == 0)
        );
        assert!(
            (((middle.as_pointer()) as Ptr<Buffer_9>).to_any().memcmp(
                &((middle2.as_pointer()) as Ptr<Buffer_9>).to_any(),
                ::std::mem::size_of::<i32>()
            ) == 0)
        );
    }
}
pub fn test_roundtrip_through_buffer_10(from: i64) {
    let from: Value<i64> = Rc::new(RefCell::new(from));
    let middle: Value<Buffer_11> = Rc::new(RefCell::new(unimplemented!()));
    let to: Value<i64> = Rc::new(RefCell::new(unimplemented!()));
    let middle2: Value<Buffer_11> = Rc::new(RefCell::new(unimplemented!()));
    assert!(
        ((((*from.borrow()) == (*to.borrow())) as i32)
            == (((*from.borrow()) == (*from.borrow())) as i32))
    );
    {
        assert!(
            (((from.as_pointer()) as Ptr<i64>).to_any().memcmp(
                &((middle.as_pointer()) as Ptr<Buffer_11>).to_any(),
                ::std::mem::size_of::<i64>()
            ) == 0)
        );
        assert!(
            (((to.as_pointer()) as Ptr<i64>).to_any().memcmp(
                &((middle.as_pointer()) as Ptr<Buffer_11>).to_any(),
                ::std::mem::size_of::<i64>()
            ) == 0)
        );
        assert!(
            (((middle.as_pointer()) as Ptr<Buffer_11>).to_any().memcmp(
                &((middle2.as_pointer()) as Ptr<Buffer_11>).to_any(),
                ::std::mem::size_of::<i64>()
            ) == 0)
        );
    }
}
pub fn test_roundtrip_through_buffer_12(from: i64) {
    let from: Value<i64> = Rc::new(RefCell::new(from));
    let middle: Value<Buffer_13> = Rc::new(RefCell::new(unimplemented!()));
    let to: Value<i64> = Rc::new(RefCell::new(unimplemented!()));
    let middle2: Value<Buffer_13> = Rc::new(RefCell::new(unimplemented!()));
    assert!(
        ((((*from.borrow()) == (*to.borrow())) as i32)
            == (((*from.borrow()) == (*from.borrow())) as i32))
    );
    {
        assert!(
            (((from.as_pointer()) as Ptr<i64>).to_any().memcmp(
                &((middle.as_pointer()) as Ptr<Buffer_13>).to_any(),
                ::std::mem::size_of::<i64>()
            ) == 0)
        );
        assert!(
            (((to.as_pointer()) as Ptr<i64>).to_any().memcmp(
                &((middle.as_pointer()) as Ptr<Buffer_13>).to_any(),
                ::std::mem::size_of::<i64>()
            ) == 0)
        );
        assert!(
            (((middle.as_pointer()) as Ptr<Buffer_13>).to_any().memcmp(
                &((middle2.as_pointer()) as Ptr<Buffer_13>).to_any(),
                ::std::mem::size_of::<i64>()
            ) == 0)
        );
    }
}
pub fn test_roundtrip_through_buffer_14(from: u8) {
    let from: Value<u8> = Rc::new(RefCell::new(from));
    let middle: Value<Buffer_15> = Rc::new(RefCell::new(unimplemented!()));
    let to: Value<u8> = Rc::new(RefCell::new(unimplemented!()));
    let middle2: Value<Buffer_15> = Rc::new(RefCell::new(unimplemented!()));
    assert!(
        (((((*from.borrow()) as i32) == ((*to.borrow()) as i32)) as i32)
            == ((((*from.borrow()) as i32) == ((*from.borrow()) as i32)) as i32))
    );
    {
        assert!(
            (((from.as_pointer()) as Ptr<u8>).to_any().memcmp(
                &((middle.as_pointer()) as Ptr<Buffer_15>).to_any(),
                ::std::mem::size_of::<u8>()
            ) == 0)
        );
        assert!(
            (((to.as_pointer()) as Ptr<u8>).to_any().memcmp(
                &((middle.as_pointer()) as Ptr<Buffer_15>).to_any(),
                ::std::mem::size_of::<u8>()
            ) == 0)
        );
        assert!(
            (((middle.as_pointer()) as Ptr<Buffer_15>).to_any().memcmp(
                &((middle2.as_pointer()) as Ptr<Buffer_15>).to_any(),
                ::std::mem::size_of::<u8>()
            ) == 0)
        );
    }
}
pub fn test_roundtrip_through_buffer_16(from: u16) {
    let from: Value<u16> = Rc::new(RefCell::new(from));
    let middle: Value<Buffer_17> = Rc::new(RefCell::new(unimplemented!()));
    let to: Value<u16> = Rc::new(RefCell::new(unimplemented!()));
    let middle2: Value<Buffer_17> = Rc::new(RefCell::new(unimplemented!()));
    assert!(
        (((((*from.borrow()) as i32) == ((*to.borrow()) as i32)) as i32)
            == ((((*from.borrow()) as i32) == ((*from.borrow()) as i32)) as i32))
    );
    {
        assert!(
            (((from.as_pointer()) as Ptr<u16>).to_any().memcmp(
                &((middle.as_pointer()) as Ptr<Buffer_17>).to_any(),
                ::std::mem::size_of::<u16>()
            ) == 0)
        );
        assert!(
            (((to.as_pointer()) as Ptr<u16>).to_any().memcmp(
                &((middle.as_pointer()) as Ptr<Buffer_17>).to_any(),
                ::std::mem::size_of::<u16>()
            ) == 0)
        );
        assert!(
            (((middle.as_pointer()) as Ptr<Buffer_17>).to_any().memcmp(
                &((middle2.as_pointer()) as Ptr<Buffer_17>).to_any(),
                ::std::mem::size_of::<u16>()
            ) == 0)
        );
    }
}
pub fn test_roundtrip_through_buffer_18(from: u32) {
    let from: Value<u32> = Rc::new(RefCell::new(from));
    let middle: Value<Buffer_19> = Rc::new(RefCell::new(unimplemented!()));
    let to: Value<u32> = Rc::new(RefCell::new(unimplemented!()));
    let middle2: Value<Buffer_19> = Rc::new(RefCell::new(unimplemented!()));
    assert!(
        ((((*from.borrow()) == (*to.borrow())) as i32)
            == (((*from.borrow()) == (*from.borrow())) as i32))
    );
    {
        assert!(
            (((from.as_pointer()) as Ptr<u32>).to_any().memcmp(
                &((middle.as_pointer()) as Ptr<Buffer_19>).to_any(),
                ::std::mem::size_of::<u32>()
            ) == 0)
        );
        assert!(
            (((to.as_pointer()) as Ptr<u32>).to_any().memcmp(
                &((middle.as_pointer()) as Ptr<Buffer_19>).to_any(),
                ::std::mem::size_of::<u32>()
            ) == 0)
        );
        assert!(
            (((middle.as_pointer()) as Ptr<Buffer_19>).to_any().memcmp(
                &((middle2.as_pointer()) as Ptr<Buffer_19>).to_any(),
                ::std::mem::size_of::<u32>()
            ) == 0)
        );
    }
}
pub fn test_roundtrip_through_buffer_20(from: u64) {
    let from: Value<u64> = Rc::new(RefCell::new(from));
    let middle: Value<Buffer_21> = Rc::new(RefCell::new(unimplemented!()));
    let to: Value<u64> = Rc::new(RefCell::new(unimplemented!()));
    let middle2: Value<Buffer_21> = Rc::new(RefCell::new(unimplemented!()));
    assert!(
        ((((*from.borrow()) == (*to.borrow())) as i32)
            == (((*from.borrow()) == (*from.borrow())) as i32))
    );
    {
        assert!(
            (((from.as_pointer()) as Ptr<u64>).to_any().memcmp(
                &((middle.as_pointer()) as Ptr<Buffer_21>).to_any(),
                ::std::mem::size_of::<u64>()
            ) == 0)
        );
        assert!(
            (((to.as_pointer()) as Ptr<u64>).to_any().memcmp(
                &((middle.as_pointer()) as Ptr<Buffer_21>).to_any(),
                ::std::mem::size_of::<u64>()
            ) == 0)
        );
        assert!(
            (((middle.as_pointer()) as Ptr<Buffer_21>).to_any().memcmp(
                &((middle2.as_pointer()) as Ptr<Buffer_21>).to_any(),
                ::std::mem::size_of::<u64>()
            ) == 0)
        );
    }
}
pub fn test_roundtrip_through_buffer_22(from: u64) {
    let from: Value<u64> = Rc::new(RefCell::new(from));
    let middle: Value<Buffer_23> = Rc::new(RefCell::new(unimplemented!()));
    let to: Value<u64> = Rc::new(RefCell::new(unimplemented!()));
    let middle2: Value<Buffer_23> = Rc::new(RefCell::new(unimplemented!()));
    assert!(
        ((((*from.borrow()) == (*to.borrow())) as i32)
            == (((*from.borrow()) == (*from.borrow())) as i32))
    );
    {
        assert!(
            (((from.as_pointer()) as Ptr<u64>).to_any().memcmp(
                &((middle.as_pointer()) as Ptr<Buffer_23>).to_any(),
                ::std::mem::size_of::<u64>()
            ) == 0)
        );
        assert!(
            (((to.as_pointer()) as Ptr<u64>).to_any().memcmp(
                &((middle.as_pointer()) as Ptr<Buffer_23>).to_any(),
                ::std::mem::size_of::<u64>()
            ) == 0)
        );
        assert!(
            (((middle.as_pointer()) as Ptr<Buffer_23>).to_any().memcmp(
                &((middle2.as_pointer()) as Ptr<Buffer_23>).to_any(),
                ::std::mem::size_of::<u64>()
            ) == 0)
        );
    }
}
pub fn test_roundtrip_through_buffer_24(from: f32) {
    let from: Value<f32> = Rc::new(RefCell::new(from));
    let middle: Value<Buffer_25> = Rc::new(RefCell::new(unimplemented!()));
    let to: Value<f32> = Rc::new(RefCell::new(unimplemented!()));
    let middle2: Value<Buffer_25> = Rc::new(RefCell::new(unimplemented!()));
    assert!(
        ((((*from.borrow()) == (*to.borrow())) as i32)
            == (((*from.borrow()) == (*from.borrow())) as i32))
    );
    {
        assert!(
            (((from.as_pointer()) as Ptr<f32>).to_any().memcmp(
                &((middle.as_pointer()) as Ptr<Buffer_25>).to_any(),
                ::std::mem::size_of::<f32>()
            ) == 0)
        );
        assert!(
            (((to.as_pointer()) as Ptr<f32>).to_any().memcmp(
                &((middle.as_pointer()) as Ptr<Buffer_25>).to_any(),
                ::std::mem::size_of::<f32>()
            ) == 0)
        );
        assert!(
            (((middle.as_pointer()) as Ptr<Buffer_25>).to_any().memcmp(
                &((middle2.as_pointer()) as Ptr<Buffer_25>).to_any(),
                ::std::mem::size_of::<f32>()
            ) == 0)
        );
    }
}
pub fn test_roundtrip_through_buffer_26(from: f64) {
    let from: Value<f64> = Rc::new(RefCell::new(from));
    let middle: Value<Buffer_27> = Rc::new(RefCell::new(unimplemented!()));
    let to: Value<f64> = Rc::new(RefCell::new(unimplemented!()));
    let middle2: Value<Buffer_27> = Rc::new(RefCell::new(unimplemented!()));
    assert!(
        ((((*from.borrow()) == (*to.borrow())) as i32)
            == (((*from.borrow()) == (*from.borrow())) as i32))
    );
    {
        assert!(
            (((from.as_pointer()) as Ptr<f64>).to_any().memcmp(
                &((middle.as_pointer()) as Ptr<Buffer_27>).to_any(),
                ::std::mem::size_of::<f64>()
            ) == 0)
        );
        assert!(
            (((to.as_pointer()) as Ptr<f64>).to_any().memcmp(
                &((middle.as_pointer()) as Ptr<Buffer_27>).to_any(),
                ::std::mem::size_of::<f64>()
            ) == 0)
        );
        assert!(
            (((middle.as_pointer()) as Ptr<Buffer_27>).to_any().memcmp(
                &((middle2.as_pointer()) as Ptr<Buffer_27>).to_any(),
                ::std::mem::size_of::<f64>()
            ) == 0)
        );
    }
}
pub fn test_roundtrip_through_buffer_28(mut from: f64) {
    let mut middle: Buffer_29 = unimplemented!();
    let mut to: f64 = unimplemented!();
    let mut middle2: Buffer_29 = unimplemented!();
    assert!((((from == to) as i32) == ((from == from) as i32)));
    {}
}
pub fn test_roundtrip_through_buffer_30(from: AnyPtr) {
    let from: Value<AnyPtr> = Rc::new(RefCell::new(from));
    let middle: Value<Buffer_31> = Rc::new(RefCell::new(unimplemented!()));
    let to: Value<AnyPtr> = Rc::new(RefCell::new(unimplemented!()));
    let middle2: Value<Buffer_31> = Rc::new(RefCell::new(unimplemented!()));
    assert!(
        ({ (({ (*from.borrow()).clone() } == { (*to.borrow()).clone() }) as i32) } == {
            (((*from.borrow()) == (*from.borrow())) as i32)
        })
    );
    {
        assert!(
            (((from.as_pointer()) as Ptr<AnyPtr>)
                .to_any()
                .memcmp(&((middle.as_pointer()) as Ptr<Buffer_31>).to_any(), 8usize)
                == 0)
        );
        assert!(
            (((to.as_pointer()) as Ptr<AnyPtr>)
                .to_any()
                .memcmp(&((middle.as_pointer()) as Ptr<Buffer_31>).to_any(), 8usize)
                == 0)
        );
        assert!(
            (((middle.as_pointer()) as Ptr<Buffer_31>)
                .to_any()
                .memcmp(&((middle2.as_pointer()) as Ptr<Buffer_31>).to_any(), 8usize)
                == 0)
        );
    }
}
pub fn test_roundtrip_through_buffer_32(from: Ptr<i32>) {
    let from: Value<Ptr<i32>> = Rc::new(RefCell::new(from));
    let middle: Value<Buffer_33> = Rc::new(RefCell::new(unimplemented!()));
    let to: Value<Ptr<i32>> = Rc::new(RefCell::new(unimplemented!()));
    let middle2: Value<Buffer_33> = Rc::new(RefCell::new(unimplemented!()));
    assert!(
        ({ (({ (*from.borrow()).clone() } == { (*to.borrow()).clone() }) as i32) } == {
            (((*from.borrow()) == (*from.borrow())) as i32)
        })
    );
    {
        assert!(
            (((from.as_pointer()) as Ptr<Ptr<i32>>)
                .to_any()
                .memcmp(&((middle.as_pointer()) as Ptr<Buffer_33>).to_any(), 8usize)
                == 0)
        );
        assert!(
            (((to.as_pointer()) as Ptr<Ptr<i32>>)
                .to_any()
                .memcmp(&((middle.as_pointer()) as Ptr<Buffer_33>).to_any(), 8usize)
                == 0)
        );
        assert!(
            (((middle.as_pointer()) as Ptr<Buffer_33>)
                .to_any()
                .memcmp(&((middle2.as_pointer()) as Ptr<Buffer_33>).to_any(), 8usize)
                == 0)
        );
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct Buffer_1 {
    #[offset(0)]
    #[byte_size(1)]
    pub buffer: Box<[i8]>,
}
impl Default for Buffer_1 {
    fn default() -> Self {
        {
            Buffer_1 {
                buffer: (0..1).map(|_| 0_i8).collect::<Box<[i8]>>(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct Buffer_3 {
    #[offset(0)]
    #[byte_size(1)]
    pub buffer: Box<[i8]>,
}
impl Default for Buffer_3 {
    fn default() -> Self {
        {
            Buffer_3 {
                buffer: (0..1).map(|_| 0_i8).collect::<Box<[i8]>>(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct Buffer_5 {
    #[offset(0)]
    #[byte_size(1)]
    pub buffer: Box<[i8]>,
}
impl Default for Buffer_5 {
    fn default() -> Self {
        {
            Buffer_5 {
                buffer: (0..1).map(|_| 0_i8).collect::<Box<[i8]>>(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(2)]
pub struct Buffer_7 {
    #[offset(0)]
    #[byte_size(2)]
    pub buffer: Box<[i8]>,
}
impl Default for Buffer_7 {
    fn default() -> Self {
        {
            Buffer_7 {
                buffer: (0..2).map(|_| 0_i8).collect::<Box<[i8]>>(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct Buffer_9 {
    #[offset(0)]
    #[byte_size(4)]
    pub buffer: Box<[i8]>,
}
impl Default for Buffer_9 {
    fn default() -> Self {
        {
            Buffer_9 {
                buffer: (0..4).map(|_| 0_i8).collect::<Box<[i8]>>(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct Buffer_11 {
    #[offset(0)]
    #[byte_size(8)]
    pub buffer: Box<[i8]>,
}
impl Default for Buffer_11 {
    fn default() -> Self {
        {
            Buffer_11 {
                buffer: (0..8).map(|_| 0_i8).collect::<Box<[i8]>>(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct Buffer_13 {
    #[offset(0)]
    #[byte_size(8)]
    pub buffer: Box<[i8]>,
}
impl Default for Buffer_13 {
    fn default() -> Self {
        {
            Buffer_13 {
                buffer: (0..8).map(|_| 0_i8).collect::<Box<[i8]>>(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct Buffer_15 {
    #[offset(0)]
    #[byte_size(1)]
    pub buffer: Box<[i8]>,
}
impl Default for Buffer_15 {
    fn default() -> Self {
        {
            Buffer_15 {
                buffer: (0..1).map(|_| 0_i8).collect::<Box<[i8]>>(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(2)]
pub struct Buffer_17 {
    #[offset(0)]
    #[byte_size(2)]
    pub buffer: Box<[i8]>,
}
impl Default for Buffer_17 {
    fn default() -> Self {
        {
            Buffer_17 {
                buffer: (0..2).map(|_| 0_i8).collect::<Box<[i8]>>(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct Buffer_19 {
    #[offset(0)]
    #[byte_size(4)]
    pub buffer: Box<[i8]>,
}
impl Default for Buffer_19 {
    fn default() -> Self {
        {
            Buffer_19 {
                buffer: (0..4).map(|_| 0_i8).collect::<Box<[i8]>>(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct Buffer_21 {
    #[offset(0)]
    #[byte_size(8)]
    pub buffer: Box<[i8]>,
}
impl Default for Buffer_21 {
    fn default() -> Self {
        {
            Buffer_21 {
                buffer: (0..8).map(|_| 0_i8).collect::<Box<[i8]>>(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct Buffer_23 {
    #[offset(0)]
    #[byte_size(8)]
    pub buffer: Box<[i8]>,
}
impl Default for Buffer_23 {
    fn default() -> Self {
        {
            Buffer_23 {
                buffer: (0..8).map(|_| 0_i8).collect::<Box<[i8]>>(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct Buffer_25 {
    #[offset(0)]
    #[byte_size(4)]
    pub buffer: Box<[i8]>,
}
impl Default for Buffer_25 {
    fn default() -> Self {
        {
            Buffer_25 {
                buffer: (0..4).map(|_| 0_i8).collect::<Box<[i8]>>(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct Buffer_27 {
    #[offset(0)]
    #[byte_size(8)]
    pub buffer: Box<[i8]>,
}
impl Default for Buffer_27 {
    fn default() -> Self {
        {
            Buffer_27 {
                buffer: (0..8).map(|_| 0_i8).collect::<Box<[i8]>>(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(16)]
pub struct Buffer_29 {
    #[offset(0)]
    #[byte_size(16)]
    pub buffer: Box<[i8]>,
}
impl Default for Buffer_29 {
    fn default() -> Self {
        {
            Buffer_29 {
                buffer: (0..16).map(|_| 0_i8).collect::<Box<[i8]>>(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct Buffer_31 {
    #[offset(0)]
    #[byte_size(8)]
    pub buffer: Box<[i8]>,
}
impl Default for Buffer_31 {
    fn default() -> Self {
        {
            Buffer_31 {
                buffer: (0..8).map(|_| 0_i8).collect::<Box<[i8]>>(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct Buffer_33 {
    #[offset(0)]
    #[byte_size(8)]
    pub buffer: Box<[i8]>,
}
impl Default for Buffer_33 {
    fn default() -> Self {
        {
            Buffer_33 {
                buffer: (0..8).map(|_| 0_i8).collect::<Box<[i8]>>(),
            }
        }
    }
}
pub fn test_roundtrip_through_nested_T_34(from: bool) {
    let from: Value<bool> = Rc::new(RefCell::new(from));
    const _: () = assert!(
        (1usize == ::std::mem::size_of::<bool>()),
        "sizeof(Nested) == sizeof(T)"
    );;
    let middle: Value<Nested_35> = Rc::new(RefCell::new(unimplemented!()));
    let to: Value<bool> = Rc::new(RefCell::new(unimplemented!()));
    let middle2: Value<Nested_35> = Rc::new(RefCell::new(unimplemented!()));
    assert!(
        (((((*from.borrow()) as i32) == ((*to.borrow()) as i32)) as i32)
            == ((((*from.borrow()) as i32) == ((*from.borrow()) as i32)) as i32))
    );
    {
        assert!(
            (((from.as_pointer()) as Ptr<bool>).to_any().memcmp(
                &((middle.as_pointer()) as Ptr<Nested_35>).to_any(),
                ::std::mem::size_of::<bool>()
            ) == 0)
        );
        assert!(
            (((to.as_pointer()) as Ptr<bool>).to_any().memcmp(
                &((middle.as_pointer()) as Ptr<Nested_35>).to_any(),
                ::std::mem::size_of::<bool>()
            ) == 0)
        );
        assert!(
            (((middle.as_pointer()) as Ptr<Nested_35>).to_any().memcmp(
                &((middle2.as_pointer()) as Ptr<Nested_35>).to_any(),
                ::std::mem::size_of::<bool>()
            ) == 0)
        );
    }
}
pub fn test_roundtrip_through_nested_T_36(from: i8) {
    let from: Value<i8> = Rc::new(RefCell::new(from));
    const _: () = assert!(
        (1usize == ::std::mem::size_of::<i8>()),
        "sizeof(Nested) == sizeof(T)"
    );;
    let middle: Value<Nested_37> = Rc::new(RefCell::new(unimplemented!()));
    let to: Value<i8> = Rc::new(RefCell::new(unimplemented!()));
    let middle2: Value<Nested_37> = Rc::new(RefCell::new(unimplemented!()));
    assert!(
        (((((*from.borrow()) as i32) == ((*to.borrow()) as i32)) as i32)
            == ((((*from.borrow()) as i32) == ((*from.borrow()) as i32)) as i32))
    );
    {
        assert!(
            (((from.as_pointer()) as Ptr<i8>).to_any().memcmp(
                &((middle.as_pointer()) as Ptr<Nested_37>).to_any(),
                ::std::mem::size_of::<i8>()
            ) == 0)
        );
        assert!(
            (((to.as_pointer()) as Ptr<i8>).to_any().memcmp(
                &((middle.as_pointer()) as Ptr<Nested_37>).to_any(),
                ::std::mem::size_of::<i8>()
            ) == 0)
        );
        assert!(
            (((middle.as_pointer()) as Ptr<Nested_37>).to_any().memcmp(
                &((middle2.as_pointer()) as Ptr<Nested_37>).to_any(),
                ::std::mem::size_of::<i8>()
            ) == 0)
        );
    }
}
pub fn test_roundtrip_through_nested_T_38(from: i8) {
    let from: Value<i8> = Rc::new(RefCell::new(from));
    const _: () = assert!(
        (1usize == ::std::mem::size_of::<i8>()),
        "sizeof(Nested) == sizeof(T)"
    );;
    let middle: Value<Nested_39> = Rc::new(RefCell::new(unimplemented!()));
    let to: Value<i8> = Rc::new(RefCell::new(unimplemented!()));
    let middle2: Value<Nested_39> = Rc::new(RefCell::new(unimplemented!()));
    assert!(
        (((((*from.borrow()) as i32) == ((*to.borrow()) as i32)) as i32)
            == ((((*from.borrow()) as i32) == ((*from.borrow()) as i32)) as i32))
    );
    {
        assert!(
            (((from.as_pointer()) as Ptr<i8>).to_any().memcmp(
                &((middle.as_pointer()) as Ptr<Nested_39>).to_any(),
                ::std::mem::size_of::<i8>()
            ) == 0)
        );
        assert!(
            (((to.as_pointer()) as Ptr<i8>).to_any().memcmp(
                &((middle.as_pointer()) as Ptr<Nested_39>).to_any(),
                ::std::mem::size_of::<i8>()
            ) == 0)
        );
        assert!(
            (((middle.as_pointer()) as Ptr<Nested_39>).to_any().memcmp(
                &((middle2.as_pointer()) as Ptr<Nested_39>).to_any(),
                ::std::mem::size_of::<i8>()
            ) == 0)
        );
    }
}
pub fn test_roundtrip_through_nested_T_40(from: i16) {
    let from: Value<i16> = Rc::new(RefCell::new(from));
    const _: () = assert!(
        (2usize == ::std::mem::size_of::<i16>()),
        "sizeof(Nested) == sizeof(T)"
    );;
    let middle: Value<Nested_41> = Rc::new(RefCell::new(unimplemented!()));
    let to: Value<i16> = Rc::new(RefCell::new(unimplemented!()));
    let middle2: Value<Nested_41> = Rc::new(RefCell::new(unimplemented!()));
    assert!(
        (((((*from.borrow()) as i32) == ((*to.borrow()) as i32)) as i32)
            == ((((*from.borrow()) as i32) == ((*from.borrow()) as i32)) as i32))
    );
    {
        assert!(
            (((from.as_pointer()) as Ptr<i16>).to_any().memcmp(
                &((middle.as_pointer()) as Ptr<Nested_41>).to_any(),
                ::std::mem::size_of::<i16>()
            ) == 0)
        );
        assert!(
            (((to.as_pointer()) as Ptr<i16>).to_any().memcmp(
                &((middle.as_pointer()) as Ptr<Nested_41>).to_any(),
                ::std::mem::size_of::<i16>()
            ) == 0)
        );
        assert!(
            (((middle.as_pointer()) as Ptr<Nested_41>).to_any().memcmp(
                &((middle2.as_pointer()) as Ptr<Nested_41>).to_any(),
                ::std::mem::size_of::<i16>()
            ) == 0)
        );
    }
}
pub fn test_roundtrip_through_nested_T_42(from: i32) {
    let from: Value<i32> = Rc::new(RefCell::new(from));
    const _: () = assert!(
        (4usize == ::std::mem::size_of::<i32>()),
        "sizeof(Nested) == sizeof(T)"
    );;
    let middle: Value<Nested_43> = Rc::new(RefCell::new(unimplemented!()));
    let to: Value<i32> = Rc::new(RefCell::new(unimplemented!()));
    let middle2: Value<Nested_43> = Rc::new(RefCell::new(unimplemented!()));
    assert!(
        ((((*from.borrow()) == (*to.borrow())) as i32)
            == (((*from.borrow()) == (*from.borrow())) as i32))
    );
    {
        assert!(
            (((from.as_pointer()) as Ptr<i32>).to_any().memcmp(
                &((middle.as_pointer()) as Ptr<Nested_43>).to_any(),
                ::std::mem::size_of::<i32>()
            ) == 0)
        );
        assert!(
            (((to.as_pointer()) as Ptr<i32>).to_any().memcmp(
                &((middle.as_pointer()) as Ptr<Nested_43>).to_any(),
                ::std::mem::size_of::<i32>()
            ) == 0)
        );
        assert!(
            (((middle.as_pointer()) as Ptr<Nested_43>).to_any().memcmp(
                &((middle2.as_pointer()) as Ptr<Nested_43>).to_any(),
                ::std::mem::size_of::<i32>()
            ) == 0)
        );
    }
}
pub fn test_roundtrip_through_nested_T_44(from: i64) {
    let from: Value<i64> = Rc::new(RefCell::new(from));
    const _: () = assert!(
        (8usize == ::std::mem::size_of::<i64>()),
        "sizeof(Nested) == sizeof(T)"
    );;
    let middle: Value<Nested_45> = Rc::new(RefCell::new(unimplemented!()));
    let to: Value<i64> = Rc::new(RefCell::new(unimplemented!()));
    let middle2: Value<Nested_45> = Rc::new(RefCell::new(unimplemented!()));
    assert!(
        ((((*from.borrow()) == (*to.borrow())) as i32)
            == (((*from.borrow()) == (*from.borrow())) as i32))
    );
    {
        assert!(
            (((from.as_pointer()) as Ptr<i64>).to_any().memcmp(
                &((middle.as_pointer()) as Ptr<Nested_45>).to_any(),
                ::std::mem::size_of::<i64>()
            ) == 0)
        );
        assert!(
            (((to.as_pointer()) as Ptr<i64>).to_any().memcmp(
                &((middle.as_pointer()) as Ptr<Nested_45>).to_any(),
                ::std::mem::size_of::<i64>()
            ) == 0)
        );
        assert!(
            (((middle.as_pointer()) as Ptr<Nested_45>).to_any().memcmp(
                &((middle2.as_pointer()) as Ptr<Nested_45>).to_any(),
                ::std::mem::size_of::<i64>()
            ) == 0)
        );
    }
}
pub fn test_roundtrip_through_nested_T_46(from: i64) {
    let from: Value<i64> = Rc::new(RefCell::new(from));
    const _: () = assert!(
        (8usize == ::std::mem::size_of::<i64>()),
        "sizeof(Nested) == sizeof(T)"
    );;
    let middle: Value<Nested_47> = Rc::new(RefCell::new(unimplemented!()));
    let to: Value<i64> = Rc::new(RefCell::new(unimplemented!()));
    let middle2: Value<Nested_47> = Rc::new(RefCell::new(unimplemented!()));
    assert!(
        ((((*from.borrow()) == (*to.borrow())) as i32)
            == (((*from.borrow()) == (*from.borrow())) as i32))
    );
    {
        assert!(
            (((from.as_pointer()) as Ptr<i64>).to_any().memcmp(
                &((middle.as_pointer()) as Ptr<Nested_47>).to_any(),
                ::std::mem::size_of::<i64>()
            ) == 0)
        );
        assert!(
            (((to.as_pointer()) as Ptr<i64>).to_any().memcmp(
                &((middle.as_pointer()) as Ptr<Nested_47>).to_any(),
                ::std::mem::size_of::<i64>()
            ) == 0)
        );
        assert!(
            (((middle.as_pointer()) as Ptr<Nested_47>).to_any().memcmp(
                &((middle2.as_pointer()) as Ptr<Nested_47>).to_any(),
                ::std::mem::size_of::<i64>()
            ) == 0)
        );
    }
}
pub fn test_roundtrip_through_nested_T_48(from: u8) {
    let from: Value<u8> = Rc::new(RefCell::new(from));
    const _: () = assert!(
        (1usize == ::std::mem::size_of::<u8>()),
        "sizeof(Nested) == sizeof(T)"
    );;
    let middle: Value<Nested_49> = Rc::new(RefCell::new(unimplemented!()));
    let to: Value<u8> = Rc::new(RefCell::new(unimplemented!()));
    let middle2: Value<Nested_49> = Rc::new(RefCell::new(unimplemented!()));
    assert!(
        (((((*from.borrow()) as i32) == ((*to.borrow()) as i32)) as i32)
            == ((((*from.borrow()) as i32) == ((*from.borrow()) as i32)) as i32))
    );
    {
        assert!(
            (((from.as_pointer()) as Ptr<u8>).to_any().memcmp(
                &((middle.as_pointer()) as Ptr<Nested_49>).to_any(),
                ::std::mem::size_of::<u8>()
            ) == 0)
        );
        assert!(
            (((to.as_pointer()) as Ptr<u8>).to_any().memcmp(
                &((middle.as_pointer()) as Ptr<Nested_49>).to_any(),
                ::std::mem::size_of::<u8>()
            ) == 0)
        );
        assert!(
            (((middle.as_pointer()) as Ptr<Nested_49>).to_any().memcmp(
                &((middle2.as_pointer()) as Ptr<Nested_49>).to_any(),
                ::std::mem::size_of::<u8>()
            ) == 0)
        );
    }
}
pub fn test_roundtrip_through_nested_T_50(from: u16) {
    let from: Value<u16> = Rc::new(RefCell::new(from));
    const _: () = assert!(
        (2usize == ::std::mem::size_of::<u16>()),
        "sizeof(Nested) == sizeof(T)"
    );;
    let middle: Value<Nested_51> = Rc::new(RefCell::new(unimplemented!()));
    let to: Value<u16> = Rc::new(RefCell::new(unimplemented!()));
    let middle2: Value<Nested_51> = Rc::new(RefCell::new(unimplemented!()));
    assert!(
        (((((*from.borrow()) as i32) == ((*to.borrow()) as i32)) as i32)
            == ((((*from.borrow()) as i32) == ((*from.borrow()) as i32)) as i32))
    );
    {
        assert!(
            (((from.as_pointer()) as Ptr<u16>).to_any().memcmp(
                &((middle.as_pointer()) as Ptr<Nested_51>).to_any(),
                ::std::mem::size_of::<u16>()
            ) == 0)
        );
        assert!(
            (((to.as_pointer()) as Ptr<u16>).to_any().memcmp(
                &((middle.as_pointer()) as Ptr<Nested_51>).to_any(),
                ::std::mem::size_of::<u16>()
            ) == 0)
        );
        assert!(
            (((middle.as_pointer()) as Ptr<Nested_51>).to_any().memcmp(
                &((middle2.as_pointer()) as Ptr<Nested_51>).to_any(),
                ::std::mem::size_of::<u16>()
            ) == 0)
        );
    }
}
pub fn test_roundtrip_through_nested_T_52(from: u32) {
    let from: Value<u32> = Rc::new(RefCell::new(from));
    const _: () = assert!(
        (4usize == ::std::mem::size_of::<u32>()),
        "sizeof(Nested) == sizeof(T)"
    );;
    let middle: Value<Nested_53> = Rc::new(RefCell::new(unimplemented!()));
    let to: Value<u32> = Rc::new(RefCell::new(unimplemented!()));
    let middle2: Value<Nested_53> = Rc::new(RefCell::new(unimplemented!()));
    assert!(
        ((((*from.borrow()) == (*to.borrow())) as i32)
            == (((*from.borrow()) == (*from.borrow())) as i32))
    );
    {
        assert!(
            (((from.as_pointer()) as Ptr<u32>).to_any().memcmp(
                &((middle.as_pointer()) as Ptr<Nested_53>).to_any(),
                ::std::mem::size_of::<u32>()
            ) == 0)
        );
        assert!(
            (((to.as_pointer()) as Ptr<u32>).to_any().memcmp(
                &((middle.as_pointer()) as Ptr<Nested_53>).to_any(),
                ::std::mem::size_of::<u32>()
            ) == 0)
        );
        assert!(
            (((middle.as_pointer()) as Ptr<Nested_53>).to_any().memcmp(
                &((middle2.as_pointer()) as Ptr<Nested_53>).to_any(),
                ::std::mem::size_of::<u32>()
            ) == 0)
        );
    }
}
pub fn test_roundtrip_through_nested_T_54(from: u64) {
    let from: Value<u64> = Rc::new(RefCell::new(from));
    const _: () = assert!(
        (8usize == ::std::mem::size_of::<u64>()),
        "sizeof(Nested) == sizeof(T)"
    );;
    let middle: Value<Nested_55> = Rc::new(RefCell::new(unimplemented!()));
    let to: Value<u64> = Rc::new(RefCell::new(unimplemented!()));
    let middle2: Value<Nested_55> = Rc::new(RefCell::new(unimplemented!()));
    assert!(
        ((((*from.borrow()) == (*to.borrow())) as i32)
            == (((*from.borrow()) == (*from.borrow())) as i32))
    );
    {
        assert!(
            (((from.as_pointer()) as Ptr<u64>).to_any().memcmp(
                &((middle.as_pointer()) as Ptr<Nested_55>).to_any(),
                ::std::mem::size_of::<u64>()
            ) == 0)
        );
        assert!(
            (((to.as_pointer()) as Ptr<u64>).to_any().memcmp(
                &((middle.as_pointer()) as Ptr<Nested_55>).to_any(),
                ::std::mem::size_of::<u64>()
            ) == 0)
        );
        assert!(
            (((middle.as_pointer()) as Ptr<Nested_55>).to_any().memcmp(
                &((middle2.as_pointer()) as Ptr<Nested_55>).to_any(),
                ::std::mem::size_of::<u64>()
            ) == 0)
        );
    }
}
pub fn test_roundtrip_through_nested_T_56(from: u64) {
    let from: Value<u64> = Rc::new(RefCell::new(from));
    const _: () = assert!(
        (8usize == ::std::mem::size_of::<u64>()),
        "sizeof(Nested) == sizeof(T)"
    );;
    let middle: Value<Nested_57> = Rc::new(RefCell::new(unimplemented!()));
    let to: Value<u64> = Rc::new(RefCell::new(unimplemented!()));
    let middle2: Value<Nested_57> = Rc::new(RefCell::new(unimplemented!()));
    assert!(
        ((((*from.borrow()) == (*to.borrow())) as i32)
            == (((*from.borrow()) == (*from.borrow())) as i32))
    );
    {
        assert!(
            (((from.as_pointer()) as Ptr<u64>).to_any().memcmp(
                &((middle.as_pointer()) as Ptr<Nested_57>).to_any(),
                ::std::mem::size_of::<u64>()
            ) == 0)
        );
        assert!(
            (((to.as_pointer()) as Ptr<u64>).to_any().memcmp(
                &((middle.as_pointer()) as Ptr<Nested_57>).to_any(),
                ::std::mem::size_of::<u64>()
            ) == 0)
        );
        assert!(
            (((middle.as_pointer()) as Ptr<Nested_57>).to_any().memcmp(
                &((middle2.as_pointer()) as Ptr<Nested_57>).to_any(),
                ::std::mem::size_of::<u64>()
            ) == 0)
        );
    }
}
pub fn test_roundtrip_through_nested_T_58(from: f32) {
    let from: Value<f32> = Rc::new(RefCell::new(from));
    const _: () = assert!(
        (4usize == ::std::mem::size_of::<f32>()),
        "sizeof(Nested) == sizeof(T)"
    );;
    let middle: Value<Nested_59> = Rc::new(RefCell::new(unimplemented!()));
    let to: Value<f32> = Rc::new(RefCell::new(unimplemented!()));
    let middle2: Value<Nested_59> = Rc::new(RefCell::new(unimplemented!()));
    assert!(
        ((((*from.borrow()) == (*to.borrow())) as i32)
            == (((*from.borrow()) == (*from.borrow())) as i32))
    );
    {
        assert!(
            (((from.as_pointer()) as Ptr<f32>).to_any().memcmp(
                &((middle.as_pointer()) as Ptr<Nested_59>).to_any(),
                ::std::mem::size_of::<f32>()
            ) == 0)
        );
        assert!(
            (((to.as_pointer()) as Ptr<f32>).to_any().memcmp(
                &((middle.as_pointer()) as Ptr<Nested_59>).to_any(),
                ::std::mem::size_of::<f32>()
            ) == 0)
        );
        assert!(
            (((middle.as_pointer()) as Ptr<Nested_59>).to_any().memcmp(
                &((middle2.as_pointer()) as Ptr<Nested_59>).to_any(),
                ::std::mem::size_of::<f32>()
            ) == 0)
        );
    }
}
pub fn test_roundtrip_through_nested_T_60(from: f64) {
    let from: Value<f64> = Rc::new(RefCell::new(from));
    const _: () = assert!(
        (8usize == ::std::mem::size_of::<f64>()),
        "sizeof(Nested) == sizeof(T)"
    );;
    let middle: Value<Nested_61> = Rc::new(RefCell::new(unimplemented!()));
    let to: Value<f64> = Rc::new(RefCell::new(unimplemented!()));
    let middle2: Value<Nested_61> = Rc::new(RefCell::new(unimplemented!()));
    assert!(
        ((((*from.borrow()) == (*to.borrow())) as i32)
            == (((*from.borrow()) == (*from.borrow())) as i32))
    );
    {
        assert!(
            (((from.as_pointer()) as Ptr<f64>).to_any().memcmp(
                &((middle.as_pointer()) as Ptr<Nested_61>).to_any(),
                ::std::mem::size_of::<f64>()
            ) == 0)
        );
        assert!(
            (((to.as_pointer()) as Ptr<f64>).to_any().memcmp(
                &((middle.as_pointer()) as Ptr<Nested_61>).to_any(),
                ::std::mem::size_of::<f64>()
            ) == 0)
        );
        assert!(
            (((middle.as_pointer()) as Ptr<Nested_61>).to_any().memcmp(
                &((middle2.as_pointer()) as Ptr<Nested_61>).to_any(),
                ::std::mem::size_of::<f64>()
            ) == 0)
        );
    }
}
pub fn test_roundtrip_through_nested_T_62(mut from: f64) {
    const _: () = assert!(
        (16usize == ::std::mem::size_of::<f64>()),
        "sizeof(Nested) == sizeof(T)"
    );;
    let mut middle: Nested_63 = unimplemented!();
    let mut to: f64 = unimplemented!();
    let mut middle2: Nested_63 = unimplemented!();
    assert!((((from == to) as i32) == ((from == from) as i32)));
    {}
}
pub fn test_roundtrip_through_nested_T_64(from: AnyPtr) {
    let from: Value<AnyPtr> = Rc::new(RefCell::new(from));
    const _: () = assert!((8usize == 8usize), "sizeof(Nested) == sizeof(T)");;
    let middle: Value<Nested_65> = Rc::new(RefCell::new(unimplemented!()));
    let to: Value<AnyPtr> = Rc::new(RefCell::new(unimplemented!()));
    let middle2: Value<Nested_65> = Rc::new(RefCell::new(unimplemented!()));
    assert!(
        ({ (({ (*from.borrow()).clone() } == { (*to.borrow()).clone() }) as i32) } == {
            (((*from.borrow()) == (*from.borrow())) as i32)
        })
    );
    {
        assert!(
            (((from.as_pointer()) as Ptr<AnyPtr>)
                .to_any()
                .memcmp(&((middle.as_pointer()) as Ptr<Nested_65>).to_any(), 8usize)
                == 0)
        );
        assert!(
            (((to.as_pointer()) as Ptr<AnyPtr>)
                .to_any()
                .memcmp(&((middle.as_pointer()) as Ptr<Nested_65>).to_any(), 8usize)
                == 0)
        );
        assert!(
            (((middle.as_pointer()) as Ptr<Nested_65>)
                .to_any()
                .memcmp(&((middle2.as_pointer()) as Ptr<Nested_65>).to_any(), 8usize)
                == 0)
        );
    }
}
pub fn test_roundtrip_through_nested_T_66(from: Ptr<i32>) {
    let from: Value<Ptr<i32>> = Rc::new(RefCell::new(from));
    const _: () = assert!((8usize == 8usize), "sizeof(Nested) == sizeof(T)");;
    let middle: Value<Nested_67> = Rc::new(RefCell::new(unimplemented!()));
    let to: Value<Ptr<i32>> = Rc::new(RefCell::new(unimplemented!()));
    let middle2: Value<Nested_67> = Rc::new(RefCell::new(unimplemented!()));
    assert!(
        ({ (({ (*from.borrow()).clone() } == { (*to.borrow()).clone() }) as i32) } == {
            (((*from.borrow()) == (*from.borrow())) as i32)
        })
    );
    {
        assert!(
            (((from.as_pointer()) as Ptr<Ptr<i32>>)
                .to_any()
                .memcmp(&((middle.as_pointer()) as Ptr<Nested_67>).to_any(), 8usize)
                == 0)
        );
        assert!(
            (((to.as_pointer()) as Ptr<Ptr<i32>>)
                .to_any()
                .memcmp(&((middle.as_pointer()) as Ptr<Nested_67>).to_any(), 8usize)
                == 0)
        );
        assert!(
            (((middle.as_pointer()) as Ptr<Nested_67>)
                .to_any()
                .memcmp(&((middle2.as_pointer()) as Ptr<Nested_67>).to_any(), 8usize)
                == 0)
        );
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Nested_35 {
    #[offset(0)]
    pub x: bool,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Nested_37 {
    #[offset(0)]
    pub x: i8,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Nested_39 {
    #[offset(0)]
    pub x: i8,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(2)]
pub struct Nested_41 {
    #[offset(0)]
    pub x: i16,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Nested_43 {
    #[offset(0)]
    pub x: i32,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Nested_45 {
    #[offset(0)]
    pub x: i64,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Nested_47 {
    #[offset(0)]
    pub x: i64,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Nested_49 {
    #[offset(0)]
    pub x: u8,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(2)]
pub struct Nested_51 {
    #[offset(0)]
    pub x: u16,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Nested_53 {
    #[offset(0)]
    pub x: u32,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Nested_55 {
    #[offset(0)]
    pub x: u64,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Nested_57 {
    #[offset(0)]
    pub x: u64,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Nested_59 {
    #[offset(0)]
    pub x: f32,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Nested_61 {
    #[offset(0)]
    pub x: f64,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct Nested_63 {
    #[offset(0)]
    #[byte_size(16)]
    pub x: f64,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Nested_65 {
    #[offset(0)]
    #[byte_size(8)]
    pub x: AnyPtr,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Nested_67 {
    #[offset(0)]
    #[byte_size(8)]
    pub x: Ptr<i32>,
}
pub fn test_roundtrip_through_68(from: bool) {
    let from: Value<bool> = Rc::new(RefCell::new(from));
    const _: () = assert!(
        (::std::mem::size_of::<i8>() == ::std::mem::size_of::<bool>()),
        "sizeof(Intermediate) == sizeof(T)"
    );;
    let middle: Value<i8> = Rc::new(RefCell::new(unimplemented!()));
    let to: Value<bool> = Rc::new(RefCell::new(unimplemented!()));
    let middle2: Value<i8> = Rc::new(RefCell::new(unimplemented!()));
    assert!(
        (((((*from.borrow()) as i32) == ((*to.borrow()) as i32)) as i32)
            == ((((*from.borrow()) as i32) == ((*from.borrow()) as i32)) as i32))
    );
    {
        assert!(
            (((from.as_pointer()) as Ptr<bool>).to_any().memcmp(
                &((middle.as_pointer()) as Ptr<i8>).to_any(),
                ::std::mem::size_of::<bool>()
            ) == 0)
        );
        assert!(
            (((to.as_pointer()) as Ptr<bool>).to_any().memcmp(
                &((middle.as_pointer()) as Ptr<i8>).to_any(),
                ::std::mem::size_of::<bool>()
            ) == 0)
        );
        assert!(
            (((middle.as_pointer()) as Ptr<i8>).to_any().memcmp(
                &((middle2.as_pointer()) as Ptr<i8>).to_any(),
                ::std::mem::size_of::<bool>()
            ) == 0)
        );
    }
}
pub fn test_roundtrip_through_69(from: i32) {
    let from: Value<i32> = Rc::new(RefCell::new(from));
    const _: () = assert!(
        (::std::mem::size_of::<f32>() == ::std::mem::size_of::<i32>()),
        "sizeof(Intermediate) == sizeof(T)"
    );;
    let middle: Value<f32> = Rc::new(RefCell::new(unimplemented!()));
    let to: Value<i32> = Rc::new(RefCell::new(unimplemented!()));
    let middle2: Value<f32> = Rc::new(RefCell::new(unimplemented!()));
    assert!(
        ((((*from.borrow()) == (*to.borrow())) as i32)
            == (((*from.borrow()) == (*from.borrow())) as i32))
    );
    {
        assert!(
            (((from.as_pointer()) as Ptr<i32>).to_any().memcmp(
                &((middle.as_pointer()) as Ptr<f32>).to_any(),
                ::std::mem::size_of::<i32>()
            ) == 0)
        );
        assert!(
            (((to.as_pointer()) as Ptr<i32>).to_any().memcmp(
                &((middle.as_pointer()) as Ptr<f32>).to_any(),
                ::std::mem::size_of::<i32>()
            ) == 0)
        );
        assert!(
            (((middle.as_pointer()) as Ptr<f32>).to_any().memcmp(
                &((middle2.as_pointer()) as Ptr<f32>).to_any(),
                ::std::mem::size_of::<i32>()
            ) == 0)
        );
    }
}
pub fn test_roundtrip_through_70(from: i64) {
    let from: Value<i64> = Rc::new(RefCell::new(from));
    const _: () = assert!(
        (::std::mem::size_of::<f64>() == ::std::mem::size_of::<i64>()),
        "sizeof(Intermediate) == sizeof(T)"
    );;
    let middle: Value<f64> = Rc::new(RefCell::new(unimplemented!()));
    let to: Value<i64> = Rc::new(RefCell::new(unimplemented!()));
    let middle2: Value<f64> = Rc::new(RefCell::new(unimplemented!()));
    assert!(
        ((((*from.borrow()) == (*to.borrow())) as i32)
            == (((*from.borrow()) == (*from.borrow())) as i32))
    );
    {
        assert!(
            (((from.as_pointer()) as Ptr<i64>).to_any().memcmp(
                &((middle.as_pointer()) as Ptr<f64>).to_any(),
                ::std::mem::size_of::<i64>()
            ) == 0)
        );
        assert!(
            (((to.as_pointer()) as Ptr<i64>).to_any().memcmp(
                &((middle.as_pointer()) as Ptr<f64>).to_any(),
                ::std::mem::size_of::<i64>()
            ) == 0)
        );
        assert!(
            (((middle.as_pointer()) as Ptr<f64>).to_any().memcmp(
                &((middle2.as_pointer()) as Ptr<f64>).to_any(),
                ::std::mem::size_of::<i64>()
            ) == 0)
        );
    }
}
pub fn test_roundtrip_through_71(from: u32) {
    let from: Value<u32> = Rc::new(RefCell::new(from));
    const _: () = assert!(
        (::std::mem::size_of::<f32>() == ::std::mem::size_of::<u32>()),
        "sizeof(Intermediate) == sizeof(T)"
    );;
    let middle: Value<f32> = Rc::new(RefCell::new(unimplemented!()));
    let to: Value<u32> = Rc::new(RefCell::new(unimplemented!()));
    let middle2: Value<f32> = Rc::new(RefCell::new(unimplemented!()));
    assert!(
        ((((*from.borrow()) == (*to.borrow())) as i32)
            == (((*from.borrow()) == (*from.borrow())) as i32))
    );
    {
        assert!(
            (((from.as_pointer()) as Ptr<u32>).to_any().memcmp(
                &((middle.as_pointer()) as Ptr<f32>).to_any(),
                ::std::mem::size_of::<u32>()
            ) == 0)
        );
        assert!(
            (((to.as_pointer()) as Ptr<u32>).to_any().memcmp(
                &((middle.as_pointer()) as Ptr<f32>).to_any(),
                ::std::mem::size_of::<u32>()
            ) == 0)
        );
        assert!(
            (((middle.as_pointer()) as Ptr<f32>).to_any().memcmp(
                &((middle2.as_pointer()) as Ptr<f32>).to_any(),
                ::std::mem::size_of::<u32>()
            ) == 0)
        );
    }
}
pub fn test_roundtrip_through_72(from: u64) {
    let from: Value<u64> = Rc::new(RefCell::new(from));
    const _: () = assert!(
        (::std::mem::size_of::<f64>() == ::std::mem::size_of::<u64>()),
        "sizeof(Intermediate) == sizeof(T)"
    );;
    let middle: Value<f64> = Rc::new(RefCell::new(unimplemented!()));
    let to: Value<u64> = Rc::new(RefCell::new(unimplemented!()));
    let middle2: Value<f64> = Rc::new(RefCell::new(unimplemented!()));
    assert!(
        ((((*from.borrow()) == (*to.borrow())) as i32)
            == (((*from.borrow()) == (*from.borrow())) as i32))
    );
    {
        assert!(
            (((from.as_pointer()) as Ptr<u64>).to_any().memcmp(
                &((middle.as_pointer()) as Ptr<f64>).to_any(),
                ::std::mem::size_of::<u64>()
            ) == 0)
        );
        assert!(
            (((to.as_pointer()) as Ptr<u64>).to_any().memcmp(
                &((middle.as_pointer()) as Ptr<f64>).to_any(),
                ::std::mem::size_of::<u64>()
            ) == 0)
        );
        assert!(
            (((middle.as_pointer()) as Ptr<f64>).to_any().memcmp(
                &((middle2.as_pointer()) as Ptr<f64>).to_any(),
                ::std::mem::size_of::<u64>()
            ) == 0)
        );
    }
}
pub fn test_roundtrip_through_73(from: i32) {
    let from: Value<i32> = Rc::new(RefCell::new(from));
    const _: () = assert!(
        (::std::mem::size_of::<i32>() == ::std::mem::size_of::<i32>()),
        "sizeof(Intermediate) == sizeof(T)"
    );;
    let middle: Value<i32> = Rc::new(RefCell::new(unimplemented!()));
    let to: Value<i32> = Rc::new(RefCell::new(unimplemented!()));
    let middle2: Value<i32> = Rc::new(RefCell::new(unimplemented!()));
    assert!(
        ((((*from.borrow()) == (*to.borrow())) as i32)
            == (((*from.borrow()) == (*from.borrow())) as i32))
    );
    {
        assert!(
            (((from.as_pointer()) as Ptr<i32>).to_any().memcmp(
                &((middle.as_pointer()) as Ptr<i32>).to_any(),
                ::std::mem::size_of::<i32>()
            ) == 0)
        );
        assert!(
            (((to.as_pointer()) as Ptr<i32>).to_any().memcmp(
                &((middle.as_pointer()) as Ptr<i32>).to_any(),
                ::std::mem::size_of::<i32>()
            ) == 0)
        );
        assert!(
            (((middle.as_pointer()) as Ptr<i32>).to_any().memcmp(
                &((middle2.as_pointer()) as Ptr<i32>).to_any(),
                ::std::mem::size_of::<i32>()
            ) == 0)
        );
    }
}
pub fn test_roundtrip_through_74(from: i32) {
    let from: Value<i32> = Rc::new(RefCell::new(from));
    const _: () = assert!(
        (::std::mem::size_of::<u32>() == ::std::mem::size_of::<i32>()),
        "sizeof(Intermediate) == sizeof(T)"
    );;
    let middle: Value<u32> = Rc::new(RefCell::new(unimplemented!()));
    let to: Value<i32> = Rc::new(RefCell::new(unimplemented!()));
    let middle2: Value<u32> = Rc::new(RefCell::new(unimplemented!()));
    assert!(
        ((((*from.borrow()) == (*to.borrow())) as i32)
            == (((*from.borrow()) == (*from.borrow())) as i32))
    );
    {
        assert!(
            (((from.as_pointer()) as Ptr<i32>).to_any().memcmp(
                &((middle.as_pointer()) as Ptr<u32>).to_any(),
                ::std::mem::size_of::<i32>()
            ) == 0)
        );
        assert!(
            (((to.as_pointer()) as Ptr<i32>).to_any().memcmp(
                &((middle.as_pointer()) as Ptr<u32>).to_any(),
                ::std::mem::size_of::<i32>()
            ) == 0)
        );
        assert!(
            (((middle.as_pointer()) as Ptr<u32>).to_any().memcmp(
                &((middle2.as_pointer()) as Ptr<u32>).to_any(),
                ::std::mem::size_of::<i32>()
            ) == 0)
        );
    }
}
pub fn test_roundtrip_through_75(from: i64) {
    let from: Value<i64> = Rc::new(RefCell::new(from));
    const _: () = assert!(
        (::std::mem::size_of::<i64>() == ::std::mem::size_of::<i64>()),
        "sizeof(Intermediate) == sizeof(T)"
    );;
    let middle: Value<i64> = Rc::new(RefCell::new(unimplemented!()));
    let to: Value<i64> = Rc::new(RefCell::new(unimplemented!()));
    let middle2: Value<i64> = Rc::new(RefCell::new(unimplemented!()));
    assert!(
        ((((*from.borrow()) == (*to.borrow())) as i32)
            == (((*from.borrow()) == (*from.borrow())) as i32))
    );
    {
        assert!(
            (((from.as_pointer()) as Ptr<i64>).to_any().memcmp(
                &((middle.as_pointer()) as Ptr<i64>).to_any(),
                ::std::mem::size_of::<i64>()
            ) == 0)
        );
        assert!(
            (((to.as_pointer()) as Ptr<i64>).to_any().memcmp(
                &((middle.as_pointer()) as Ptr<i64>).to_any(),
                ::std::mem::size_of::<i64>()
            ) == 0)
        );
        assert!(
            (((middle.as_pointer()) as Ptr<i64>).to_any().memcmp(
                &((middle2.as_pointer()) as Ptr<i64>).to_any(),
                ::std::mem::size_of::<i64>()
            ) == 0)
        );
    }
}
pub fn test_roundtrip_through_76(from: i64) {
    let from: Value<i64> = Rc::new(RefCell::new(from));
    const _: () = assert!(
        (::std::mem::size_of::<u64>() == ::std::mem::size_of::<i64>()),
        "sizeof(Intermediate) == sizeof(T)"
    );;
    let middle: Value<u64> = Rc::new(RefCell::new(unimplemented!()));
    let to: Value<i64> = Rc::new(RefCell::new(unimplemented!()));
    let middle2: Value<u64> = Rc::new(RefCell::new(unimplemented!()));
    assert!(
        ((((*from.borrow()) == (*to.borrow())) as i32)
            == (((*from.borrow()) == (*from.borrow())) as i32))
    );
    {
        assert!(
            (((from.as_pointer()) as Ptr<i64>).to_any().memcmp(
                &((middle.as_pointer()) as Ptr<u64>).to_any(),
                ::std::mem::size_of::<i64>()
            ) == 0)
        );
        assert!(
            (((to.as_pointer()) as Ptr<i64>).to_any().memcmp(
                &((middle.as_pointer()) as Ptr<u64>).to_any(),
                ::std::mem::size_of::<i64>()
            ) == 0)
        );
        assert!(
            (((middle.as_pointer()) as Ptr<u64>).to_any().memcmp(
                &((middle2.as_pointer()) as Ptr<u64>).to_any(),
                ::std::mem::size_of::<i64>()
            ) == 0)
        );
    }
}
pub fn test_roundtrip_through_77(from: i64) {
    let from: Value<i64> = Rc::new(RefCell::new(from));
    const _: () = assert!(
        (::std::mem::size_of::<f64>() == ::std::mem::size_of::<i64>()),
        "sizeof(Intermediate) == sizeof(T)"
    );;
    let middle: Value<f64> = Rc::new(RefCell::new(unimplemented!()));
    let to: Value<i64> = Rc::new(RefCell::new(unimplemented!()));
    let middle2: Value<f64> = Rc::new(RefCell::new(unimplemented!()));
    assert!(
        ((((*from.borrow()) == (*to.borrow())) as i32)
            == (((*from.borrow()) == (*from.borrow())) as i32))
    );
    {
        assert!(
            (((from.as_pointer()) as Ptr<i64>).to_any().memcmp(
                &((middle.as_pointer()) as Ptr<f64>).to_any(),
                ::std::mem::size_of::<i64>()
            ) == 0)
        );
        assert!(
            (((to.as_pointer()) as Ptr<i64>).to_any().memcmp(
                &((middle.as_pointer()) as Ptr<f64>).to_any(),
                ::std::mem::size_of::<i64>()
            ) == 0)
        );
        assert!(
            (((middle.as_pointer()) as Ptr<f64>).to_any().memcmp(
                &((middle2.as_pointer()) as Ptr<f64>).to_any(),
                ::std::mem::size_of::<i64>()
            ) == 0)
        );
    }
}
pub fn test_roundtrip_through_78(from: u32) {
    let from: Value<u32> = Rc::new(RefCell::new(from));
    const _: () = assert!(
        (::std::mem::size_of::<i32>() == ::std::mem::size_of::<u32>()),
        "sizeof(Intermediate) == sizeof(T)"
    );;
    let middle: Value<i32> = Rc::new(RefCell::new(unimplemented!()));
    let to: Value<u32> = Rc::new(RefCell::new(unimplemented!()));
    let middle2: Value<i32> = Rc::new(RefCell::new(unimplemented!()));
    assert!(
        ((((*from.borrow()) == (*to.borrow())) as i32)
            == (((*from.borrow()) == (*from.borrow())) as i32))
    );
    {
        assert!(
            (((from.as_pointer()) as Ptr<u32>).to_any().memcmp(
                &((middle.as_pointer()) as Ptr<i32>).to_any(),
                ::std::mem::size_of::<u32>()
            ) == 0)
        );
        assert!(
            (((to.as_pointer()) as Ptr<u32>).to_any().memcmp(
                &((middle.as_pointer()) as Ptr<i32>).to_any(),
                ::std::mem::size_of::<u32>()
            ) == 0)
        );
        assert!(
            (((middle.as_pointer()) as Ptr<i32>).to_any().memcmp(
                &((middle2.as_pointer()) as Ptr<i32>).to_any(),
                ::std::mem::size_of::<u32>()
            ) == 0)
        );
    }
}
pub fn test_roundtrip_through_79(from: u64) {
    let from: Value<u64> = Rc::new(RefCell::new(from));
    const _: () = assert!(
        (::std::mem::size_of::<i64>() == ::std::mem::size_of::<u64>()),
        "sizeof(Intermediate) == sizeof(T)"
    );;
    let middle: Value<i64> = Rc::new(RefCell::new(unimplemented!()));
    let to: Value<u64> = Rc::new(RefCell::new(unimplemented!()));
    let middle2: Value<i64> = Rc::new(RefCell::new(unimplemented!()));
    assert!(
        ((((*from.borrow()) == (*to.borrow())) as i32)
            == (((*from.borrow()) == (*from.borrow())) as i32))
    );
    {
        assert!(
            (((from.as_pointer()) as Ptr<u64>).to_any().memcmp(
                &((middle.as_pointer()) as Ptr<i64>).to_any(),
                ::std::mem::size_of::<u64>()
            ) == 0)
        );
        assert!(
            (((to.as_pointer()) as Ptr<u64>).to_any().memcmp(
                &((middle.as_pointer()) as Ptr<i64>).to_any(),
                ::std::mem::size_of::<u64>()
            ) == 0)
        );
        assert!(
            (((middle.as_pointer()) as Ptr<i64>).to_any().memcmp(
                &((middle2.as_pointer()) as Ptr<i64>).to_any(),
                ::std::mem::size_of::<u64>()
            ) == 0)
        );
    }
}
pub fn test_roundtrip_through_80(from: u64) {
    let from: Value<u64> = Rc::new(RefCell::new(from));
    const _: () = assert!(
        (::std::mem::size_of::<i64>() == ::std::mem::size_of::<u64>()),
        "sizeof(Intermediate) == sizeof(T)"
    );;
    let middle: Value<i64> = Rc::new(RefCell::new(unimplemented!()));
    let to: Value<u64> = Rc::new(RefCell::new(unimplemented!()));
    let middle2: Value<i64> = Rc::new(RefCell::new(unimplemented!()));
    assert!(
        ((((*from.borrow()) == (*to.borrow())) as i32)
            == (((*from.borrow()) == (*from.borrow())) as i32))
    );
    {
        assert!(
            (((from.as_pointer()) as Ptr<u64>).to_any().memcmp(
                &((middle.as_pointer()) as Ptr<i64>).to_any(),
                ::std::mem::size_of::<u64>()
            ) == 0)
        );
        assert!(
            (((to.as_pointer()) as Ptr<u64>).to_any().memcmp(
                &((middle.as_pointer()) as Ptr<i64>).to_any(),
                ::std::mem::size_of::<u64>()
            ) == 0)
        );
        assert!(
            (((middle.as_pointer()) as Ptr<i64>).to_any().memcmp(
                &((middle2.as_pointer()) as Ptr<i64>).to_any(),
                ::std::mem::size_of::<u64>()
            ) == 0)
        );
    }
}
pub fn test_roundtrip_through_81(from: u64) {
    let from: Value<u64> = Rc::new(RefCell::new(from));
    const _: () = assert!(
        (::std::mem::size_of::<f64>() == ::std::mem::size_of::<u64>()),
        "sizeof(Intermediate) == sizeof(T)"
    );;
    let middle: Value<f64> = Rc::new(RefCell::new(unimplemented!()));
    let to: Value<u64> = Rc::new(RefCell::new(unimplemented!()));
    let middle2: Value<f64> = Rc::new(RefCell::new(unimplemented!()));
    assert!(
        ((((*from.borrow()) == (*to.borrow())) as i32)
            == (((*from.borrow()) == (*from.borrow())) as i32))
    );
    {
        assert!(
            (((from.as_pointer()) as Ptr<u64>).to_any().memcmp(
                &((middle.as_pointer()) as Ptr<f64>).to_any(),
                ::std::mem::size_of::<u64>()
            ) == 0)
        );
        assert!(
            (((to.as_pointer()) as Ptr<u64>).to_any().memcmp(
                &((middle.as_pointer()) as Ptr<f64>).to_any(),
                ::std::mem::size_of::<u64>()
            ) == 0)
        );
        assert!(
            (((middle.as_pointer()) as Ptr<f64>).to_any().memcmp(
                &((middle2.as_pointer()) as Ptr<f64>).to_any(),
                ::std::mem::size_of::<u64>()
            ) == 0)
        );
    }
}
pub fn test_roundtrip_through_82(from: f32) {
    let from: Value<f32> = Rc::new(RefCell::new(from));
    const _: () = assert!(
        (::std::mem::size_of::<i32>() == ::std::mem::size_of::<f32>()),
        "sizeof(Intermediate) == sizeof(T)"
    );;
    let middle: Value<i32> = Rc::new(RefCell::new(unimplemented!()));
    let to: Value<f32> = Rc::new(RefCell::new(unimplemented!()));
    let middle2: Value<i32> = Rc::new(RefCell::new(unimplemented!()));
    assert!(
        ((((*from.borrow()) == (*to.borrow())) as i32)
            == (((*from.borrow()) == (*from.borrow())) as i32))
    );
    {
        assert!(
            (((from.as_pointer()) as Ptr<f32>).to_any().memcmp(
                &((middle.as_pointer()) as Ptr<i32>).to_any(),
                ::std::mem::size_of::<f32>()
            ) == 0)
        );
        assert!(
            (((to.as_pointer()) as Ptr<f32>).to_any().memcmp(
                &((middle.as_pointer()) as Ptr<i32>).to_any(),
                ::std::mem::size_of::<f32>()
            ) == 0)
        );
        assert!(
            (((middle.as_pointer()) as Ptr<i32>).to_any().memcmp(
                &((middle2.as_pointer()) as Ptr<i32>).to_any(),
                ::std::mem::size_of::<f32>()
            ) == 0)
        );
    }
}
pub fn test_roundtrip_through_83(from: f64) {
    let from: Value<f64> = Rc::new(RefCell::new(from));
    const _: () = assert!(
        (::std::mem::size_of::<i64>() == ::std::mem::size_of::<f64>()),
        "sizeof(Intermediate) == sizeof(T)"
    );;
    let middle: Value<i64> = Rc::new(RefCell::new(unimplemented!()));
    let to: Value<f64> = Rc::new(RefCell::new(unimplemented!()));
    let middle2: Value<i64> = Rc::new(RefCell::new(unimplemented!()));
    assert!(
        ((((*from.borrow()) == (*to.borrow())) as i32)
            == (((*from.borrow()) == (*from.borrow())) as i32))
    );
    {
        assert!(
            (((from.as_pointer()) as Ptr<f64>).to_any().memcmp(
                &((middle.as_pointer()) as Ptr<i64>).to_any(),
                ::std::mem::size_of::<f64>()
            ) == 0)
        );
        assert!(
            (((to.as_pointer()) as Ptr<f64>).to_any().memcmp(
                &((middle.as_pointer()) as Ptr<i64>).to_any(),
                ::std::mem::size_of::<f64>()
            ) == 0)
        );
        assert!(
            (((middle.as_pointer()) as Ptr<i64>).to_any().memcmp(
                &((middle2.as_pointer()) as Ptr<i64>).to_any(),
                ::std::mem::size_of::<f64>()
            ) == 0)
        );
    }
}
pub fn test_roundtrip_through_84(mut from: f64) {
    const _: () = assert!(
        (::std::mem::size_of::<i128>() == ::std::mem::size_of::<f64>()),
        "sizeof(Intermediate) == sizeof(T)"
    );;
    let mut middle: i128 = unimplemented!();
    let mut to: f64 = unimplemented!();
    let mut middle2: i128 = unimplemented!();
    assert!((((from == to) as i32) == ((from == from) as i32)));
    {}
}
pub fn test_roundtrip_through_85(mut from: f64) {
    const _: () = assert!(
        (::std::mem::size_of::<u128>() == ::std::mem::size_of::<f64>()),
        "sizeof(Intermediate) == sizeof(T)"
    );;
    let mut middle: u128 = unimplemented!();
    let mut to: f64 = unimplemented!();
    let mut middle2: u128 = unimplemented!();
    assert!((((from == to) as i32) == ((from == from) as i32)));
    {}
}
pub fn test_roundtrip_through_86(from: AnyPtr) {
    let from: Value<AnyPtr> = Rc::new(RefCell::new(from));
    const _: () = assert!((8usize == 8usize), "sizeof(Intermediate) == sizeof(T)");;
    let middle: Value<AnyPtr> = Rc::new(RefCell::new(unimplemented!()));
    let to: Value<AnyPtr> = Rc::new(RefCell::new(unimplemented!()));
    let middle2: Value<AnyPtr> = Rc::new(RefCell::new(unimplemented!()));
    assert!(
        ({ (({ (*from.borrow()).clone() } == { (*to.borrow()).clone() }) as i32) } == {
            (((*from.borrow()) == (*from.borrow())) as i32)
        })
    );
    {
        assert!(
            (((from.as_pointer()) as Ptr<AnyPtr>)
                .to_any()
                .memcmp(&((middle.as_pointer()) as Ptr<AnyPtr>).to_any(), 8usize)
                == 0)
        );
        assert!(
            (((to.as_pointer()) as Ptr<AnyPtr>)
                .to_any()
                .memcmp(&((middle.as_pointer()) as Ptr<AnyPtr>).to_any(), 8usize)
                == 0)
        );
        assert!(
            (((middle.as_pointer()) as Ptr<AnyPtr>)
                .to_any()
                .memcmp(&((middle2.as_pointer()) as Ptr<AnyPtr>).to_any(), 8usize)
                == 0)
        );
    }
}
pub fn test_roundtrip_through_87(from: AnyPtr) {
    let from: Value<AnyPtr> = Rc::new(RefCell::new(from));
    const _: () = assert!((8usize == 8usize), "sizeof(Intermediate) == sizeof(T)");;
    let middle: Value<Ptr<i8>> = Rc::new(RefCell::new(unimplemented!()));
    let to: Value<AnyPtr> = Rc::new(RefCell::new(unimplemented!()));
    let middle2: Value<Ptr<i8>> = Rc::new(RefCell::new(unimplemented!()));
    assert!(
        ({ (({ (*from.borrow()).clone() } == { (*to.borrow()).clone() }) as i32) } == {
            (((*from.borrow()) == (*from.borrow())) as i32)
        })
    );
    {
        assert!(
            (((from.as_pointer()) as Ptr<AnyPtr>)
                .to_any()
                .memcmp(&((middle.as_pointer()) as Ptr<Ptr<i8>>).to_any(), 8usize)
                == 0)
        );
        assert!(
            (((to.as_pointer()) as Ptr<AnyPtr>)
                .to_any()
                .memcmp(&((middle.as_pointer()) as Ptr<Ptr<i8>>).to_any(), 8usize)
                == 0)
        );
        assert!(
            (((middle.as_pointer()) as Ptr<Ptr<i8>>)
                .to_any()
                .memcmp(&((middle2.as_pointer()) as Ptr<Ptr<i8>>).to_any(), 8usize)
                == 0)
        );
    }
}
pub fn test_roundtrip_through_88(from: AnyPtr) {
    let from: Value<AnyPtr> = Rc::new(RefCell::new(from));
    const _: () = assert!((8usize == 8usize), "sizeof(Intermediate) == sizeof(T)");;
    let middle: Value<Ptr<i32>> = Rc::new(RefCell::new(unimplemented!()));
    let to: Value<AnyPtr> = Rc::new(RefCell::new(unimplemented!()));
    let middle2: Value<Ptr<i32>> = Rc::new(RefCell::new(unimplemented!()));
    assert!(
        ({ (({ (*from.borrow()).clone() } == { (*to.borrow()).clone() }) as i32) } == {
            (((*from.borrow()) == (*from.borrow())) as i32)
        })
    );
    {
        assert!(
            (((from.as_pointer()) as Ptr<AnyPtr>)
                .to_any()
                .memcmp(&((middle.as_pointer()) as Ptr<Ptr<i32>>).to_any(), 8usize)
                == 0)
        );
        assert!(
            (((to.as_pointer()) as Ptr<AnyPtr>)
                .to_any()
                .memcmp(&((middle.as_pointer()) as Ptr<Ptr<i32>>).to_any(), 8usize)
                == 0)
        );
        assert!(
            (((middle.as_pointer()) as Ptr<Ptr<i32>>)
                .to_any()
                .memcmp(&((middle2.as_pointer()) as Ptr<Ptr<i32>>).to_any(), 8usize)
                == 0)
        );
    }
}
pub fn test_roundtrip_through_89(from: Ptr<i32>) {
    let from: Value<Ptr<i32>> = Rc::new(RefCell::new(from));
    const _: () = assert!((8usize == 8usize), "sizeof(Intermediate) == sizeof(T)");;
    let middle: Value<Ptr<i32>> = Rc::new(RefCell::new(unimplemented!()));
    let to: Value<Ptr<i32>> = Rc::new(RefCell::new(unimplemented!()));
    let middle2: Value<Ptr<i32>> = Rc::new(RefCell::new(unimplemented!()));
    assert!(
        ({ (({ (*from.borrow()).clone() } == { (*to.borrow()).clone() }) as i32) } == {
            (((*from.borrow()) == (*from.borrow())) as i32)
        })
    );
    {
        assert!(
            (((from.as_pointer()) as Ptr<Ptr<i32>>)
                .to_any()
                .memcmp(&((middle.as_pointer()) as Ptr<Ptr<i32>>).to_any(), 8usize)
                == 0)
        );
        assert!(
            (((to.as_pointer()) as Ptr<Ptr<i32>>)
                .to_any()
                .memcmp(&((middle.as_pointer()) as Ptr<Ptr<i32>>).to_any(), 8usize)
                == 0)
        );
        assert!(
            (((middle.as_pointer()) as Ptr<Ptr<i32>>)
                .to_any()
                .memcmp(&((middle2.as_pointer()) as Ptr<Ptr<i32>>).to_any(), 8usize)
                == 0)
        );
    }
}
pub fn test_roundtrip_through_90(from: Ptr<i32>) {
    let from: Value<Ptr<i32>> = Rc::new(RefCell::new(from));
    const _: () = assert!((8usize == 8usize), "sizeof(Intermediate) == sizeof(T)");;
    let middle: Value<Ptr<i8>> = Rc::new(RefCell::new(unimplemented!()));
    let to: Value<Ptr<i32>> = Rc::new(RefCell::new(unimplemented!()));
    let middle2: Value<Ptr<i8>> = Rc::new(RefCell::new(unimplemented!()));
    assert!(
        ({ (({ (*from.borrow()).clone() } == { (*to.borrow()).clone() }) as i32) } == {
            (((*from.borrow()) == (*from.borrow())) as i32)
        })
    );
    {
        assert!(
            (((from.as_pointer()) as Ptr<Ptr<i32>>)
                .to_any()
                .memcmp(&((middle.as_pointer()) as Ptr<Ptr<i8>>).to_any(), 8usize)
                == 0)
        );
        assert!(
            (((to.as_pointer()) as Ptr<Ptr<i32>>)
                .to_any()
                .memcmp(&((middle.as_pointer()) as Ptr<Ptr<i8>>).to_any(), 8usize)
                == 0)
        );
        assert!(
            (((middle.as_pointer()) as Ptr<Ptr<i8>>)
                .to_any()
                .memcmp(&((middle2.as_pointer()) as Ptr<Ptr<i8>>).to_any(), 8usize)
                == 0)
        );
    }
}
pub fn test_roundtrip_through_91(from: Ptr<i32>) {
    let from: Value<Ptr<i32>> = Rc::new(RefCell::new(from));
    const _: () = assert!((8usize == 8usize), "sizeof(Intermediate) == sizeof(T)");;
    let middle: Value<AnyPtr> = Rc::new(RefCell::new(unimplemented!()));
    let to: Value<Ptr<i32>> = Rc::new(RefCell::new(unimplemented!()));
    let middle2: Value<AnyPtr> = Rc::new(RefCell::new(unimplemented!()));
    assert!(
        ({ (({ (*from.borrow()).clone() } == { (*to.borrow()).clone() }) as i32) } == {
            (((*from.borrow()) == (*from.borrow())) as i32)
        })
    );
    {
        assert!(
            (((from.as_pointer()) as Ptr<Ptr<i32>>)
                .to_any()
                .memcmp(&((middle.as_pointer()) as Ptr<AnyPtr>).to_any(), 8usize)
                == 0)
        );
        assert!(
            (((to.as_pointer()) as Ptr<Ptr<i32>>)
                .to_any()
                .memcmp(&((middle.as_pointer()) as Ptr<AnyPtr>).to_any(), 8usize)
                == 0)
        );
        assert!(
            (((middle.as_pointer()) as Ptr<AnyPtr>)
                .to_any()
                .memcmp(&((middle2.as_pointer()) as Ptr<AnyPtr>).to_any(), 8usize)
                == 0)
        );
    }
}
pub fn generate_signed_integral_values_92() -> Vec<i8> {
    return Vec::from([
        i8::MIN,
        (((i8::MIN as i32) + 1) as i8),
        (-2_i32 as i8),
        (-1_i32 as i8),
        0_i8,
        1_i8,
        2_i8,
        3_i8,
        (((i8::MAX as i32) - 1) as i8),
        i8::MAX,
    ]);
}
pub fn generate_signed_integral_values_93() -> Vec<i16> {
    return Vec::from([
        i16::MIN,
        (((i16::MIN as i32) + 1) as i16),
        (-2_i32 as i16),
        (-1_i32 as i16),
        0_i16,
        1_i16,
        2_i16,
        3_i16,
        (((i16::MAX as i32) - 1) as i16),
        i16::MAX,
    ]);
}
pub fn generate_signed_integral_values_94() -> Vec<i32> {
    return Vec::from([
        i32::MIN,
        (i32::MIN + 1),
        (-2_i32 as i32),
        (-1_i32 as i32),
        (0 as i32),
        (1 as i32),
        (2 as i32),
        (3 as i32),
        (i32::MAX - 1),
        i32::MAX,
    ]);
}
pub fn generate_signed_integral_values_95() -> Vec<i64> {
    return Vec::from([
        i64::MIN,
        (i64::MIN + 1_i64),
        (-2_i32 as i64),
        (-1_i32 as i64),
        0_i64,
        1_i64,
        2_i64,
        3_i64,
        (i64::MAX - 1_i64),
        i64::MAX,
    ]);
}
pub fn generate_signed_integral_values_96() -> Vec<i64> {
    return Vec::from([
        i64::MIN,
        (i64::MIN + 1_i64),
        (-2_i32 as i64),
        (-1_i32 as i64),
        0_i64,
        1_i64,
        2_i64,
        3_i64,
        (i64::MAX - 1_i64),
        i64::MAX,
    ]);
}
pub fn generate_unsigned_integral_values_97() -> Vec<u8> {
    return Vec::from([
        0_u8,
        1_u8,
        2_u8,
        3_u8,
        (((u8::MAX as i32) - 1) as u8),
        u8::MAX,
    ]);
}
pub fn generate_unsigned_integral_values_98() -> Vec<u16> {
    return Vec::from([
        0_u16,
        1_u16,
        2_u16,
        3_u16,
        (((u16::MAX as i32) - 1) as u16),
        u16::MAX,
    ]);
}
pub fn generate_unsigned_integral_values_99() -> Vec<u32> {
    return Vec::from([
        0_u32,
        1_u32,
        2_u32,
        3_u32,
        (u32::MAX as u32).wrapping_sub(1_u32),
        u32::MAX,
    ]);
}
pub fn generate_unsigned_integral_values_100() -> Vec<u64> {
    return Vec::from([
        0_u64,
        1_u64,
        2_u64,
        3_u64,
        (u64::MAX as u64).wrapping_sub(1_u64),
        u64::MAX,
    ]);
}
pub fn generate_unsigned_integral_values_101() -> Vec<u64> {
    return Vec::from([
        0_u64,
        1_u64,
        2_u64,
        3_u64,
        (unimplemented!() as u64).wrapping_sub(1_u64),
        unimplemented!(),
    ]);
}
pub fn tests_102() -> bool {
    'loop_: for b in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut b: bool = (b.read());
        ({ test_roundtrip_through_nested_T_34(b) });
        ({ test_roundtrip_through_buffer_0(b) });
        ({ test_roundtrip_through_68(b) });
    }
    'loop_: for c in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut c: i8 = (c.read());
        ({ test_roundtrip_through_nested_T_36(c) });
        ({ test_roundtrip_through_buffer_2(c) });
    }
    let __tmp_0: Value<Vec<i8>> = Rc::new(RefCell::new(({ generate_signed_integral_values_92() })));
    let __range1: Ptr<Vec<i8>> = __tmp_0.as_pointer();
    'loop_: for i in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut i: i8 = (i.read());
        ({ test_roundtrip_through_nested_T_38(i) });
        ({ test_roundtrip_through_buffer_4(i) });
    }
    let __tmp_1: Value<Vec<i16>> =
        Rc::new(RefCell::new(({ generate_signed_integral_values_93() })));
    let __range1: Ptr<Vec<i16>> = __tmp_1.as_pointer();
    'loop_: for i in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut i: i16 = (i.read());
        ({ test_roundtrip_through_nested_T_40(i) });
        ({ test_roundtrip_through_buffer_6(i) });
    }
    let __tmp_2: Value<Vec<i32>> =
        Rc::new(RefCell::new(({ generate_signed_integral_values_94() })));
    let __range1: Ptr<Vec<i32>> = __tmp_2.as_pointer();
    'loop_: for i in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut i: i32 = (i.read());
        ({ test_roundtrip_through_nested_T_42(i) });
        ({ test_roundtrip_through_buffer_8(i) });
        ({ test_roundtrip_through_69(i) });
    }
    let __tmp_3: Value<Vec<i64>> =
        Rc::new(RefCell::new(({ generate_signed_integral_values_95() })));
    let __range1: Ptr<Vec<i64>> = __tmp_3.as_pointer();
    'loop_: for i in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut i: i64 = (i.read());
        ({ test_roundtrip_through_nested_T_44(i) });
        ({ test_roundtrip_through_buffer_10(i) });
    }
    let __tmp_4: Value<Vec<i64>> =
        Rc::new(RefCell::new(({ generate_signed_integral_values_96() })));
    let __range1: Ptr<Vec<i64>> = __tmp_4.as_pointer();
    'loop_: for i in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut i: i64 = (i.read());
        ({ test_roundtrip_through_nested_T_46(i) });
        ({ test_roundtrip_through_buffer_12(i) });
        ({ test_roundtrip_through_70(i) });
    }
    let __tmp_5: Value<Vec<u8>> =
        Rc::new(RefCell::new(({ generate_unsigned_integral_values_97() })));
    let __range1: Ptr<Vec<u8>> = __tmp_5.as_pointer();
    'loop_: for i in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut i: u8 = (i.read());
        ({ test_roundtrip_through_nested_T_48(i) });
        ({ test_roundtrip_through_buffer_14(i) });
    }
    let __tmp_6: Value<Vec<u16>> =
        Rc::new(RefCell::new(({ generate_unsigned_integral_values_98() })));
    let __range1: Ptr<Vec<u16>> = __tmp_6.as_pointer();
    'loop_: for i in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut i: u16 = (i.read());
        ({ test_roundtrip_through_nested_T_50(i) });
        ({ test_roundtrip_through_buffer_16(i) });
    }
    let __tmp_7: Value<Vec<u32>> =
        Rc::new(RefCell::new(({ generate_unsigned_integral_values_99() })));
    let __range1: Ptr<Vec<u32>> = __tmp_7.as_pointer();
    'loop_: for i in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut i: u32 = (i.read());
        ({ test_roundtrip_through_nested_T_52(i) });
        ({ test_roundtrip_through_buffer_18(i) });
        ({ test_roundtrip_through_71(i) });
    }
    let __tmp_8: Value<Vec<u64>> =
        Rc::new(RefCell::new(({ generate_unsigned_integral_values_100() })));
    let __range1: Ptr<Vec<u64>> = __tmp_8.as_pointer();
    'loop_: for i in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut i: u64 = (i.read());
        ({ test_roundtrip_through_nested_T_54(i) });
        ({ test_roundtrip_through_buffer_20(i) });
    }
    let __tmp_9: Value<Vec<u64>> =
        Rc::new(RefCell::new(({ generate_unsigned_integral_values_101() })));
    let __range1: Ptr<Vec<u64>> = __tmp_9.as_pointer();
    'loop_: for i in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut i: u64 = (i.read());
        ({ test_roundtrip_through_nested_T_56(i) });
        ({ test_roundtrip_through_buffer_22(i) });
        ({ test_roundtrip_through_72(i) });
    }
    let __tmp_10: Value<Vec<i32>> =
        Rc::new(RefCell::new(({ generate_signed_integral_values_94() })));
    let __range1: Ptr<Vec<i32>> = __tmp_10.as_pointer();
    'loop_: for i in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut i: i32 = (i.read());
        ({ test_roundtrip_through_nested_T_42(i) });
        ({ test_roundtrip_through_buffer_8(i) });
        ({ test_roundtrip_through_73(i) });
        ({ test_roundtrip_through_74(i) });
        ({ test_roundtrip_through_69(i) });
    }
    let __tmp_11: Value<Vec<i64>> =
        Rc::new(RefCell::new(({ generate_signed_integral_values_95() })));
    let __range1: Ptr<Vec<i64>> = __tmp_11.as_pointer();
    'loop_: for i in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut i: i64 = (i.read());
        ({ test_roundtrip_through_nested_T_44(i) });
        ({ test_roundtrip_through_buffer_10(i) });
        ({ test_roundtrip_through_75(i) });
        ({ test_roundtrip_through_76(i) });
        ({ test_roundtrip_through_77(i) });
    }
    let __tmp_12: Value<Vec<u32>> =
        Rc::new(RefCell::new(({ generate_unsigned_integral_values_99() })));
    let __range1: Ptr<Vec<u32>> = __tmp_12.as_pointer();
    'loop_: for i in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut i: u32 = (i.read());
        ({ test_roundtrip_through_nested_T_52(i) });
        ({ test_roundtrip_through_buffer_18(i) });
        ({ test_roundtrip_through_78(i) });
        ({ test_roundtrip_through_78(i) });
        ({ test_roundtrip_through_71(i) });
    }
    let __tmp_13: Value<Vec<u64>> =
        Rc::new(RefCell::new(({ generate_unsigned_integral_values_100() })));
    let __range1: Ptr<Vec<u64>> = __tmp_13.as_pointer();
    'loop_: for i in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut i: u64 = (i.read());
        ({ test_roundtrip_through_nested_T_54(i) });
        ({ test_roundtrip_through_buffer_20(i) });
        ({ test_roundtrip_through_79(i) });
        ({ test_roundtrip_through_80(i) });
        ({ test_roundtrip_through_81(i) });
    }
    let __tmp_14: Value<Vec<f32>> = Rc::new(RefCell::new(Vec::from([
        0_f32,
        1_f32,
        -1_f32,
        10_f32,
        -10_f32,
        1e+10_f32,
        1e-10_f32,
        1e+20_f32,
        1e-20_f32,
        2.71828_f32,
        3.14159_f32,
        f32::NAN,
        f32::NAN,
        unimplemented!(),
        unimplemented!(),
    ])));
    let __range1: Ptr<Vec<f32>> = __tmp_14.as_pointer();
    'loop_: for i in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut i: f32 = (i.read());
        ({ test_roundtrip_through_nested_T_58(i) });
        ({ test_roundtrip_through_buffer_24(i) });
        ({ test_roundtrip_through_82(i) });
    }
    let __tmp_15: Value<Vec<f64>> = Rc::new(RefCell::new(Vec::from([
        0_f64,
        1_f64,
        -1_f64,
        10_f64,
        -10_f64,
        1e+10_f64,
        1e-10_f64,
        1e+100_f64,
        1e-100_f64,
        2.718281828459045_f64,
        3.141592653589793_f64,
        f64::NAN,
        unimplemented!(),
        unimplemented!(),
    ])));
    let __range1: Ptr<Vec<f64>> = __tmp_15.as_pointer();
    'loop_: for i in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut i: f64 = (i.read());
        ({ test_roundtrip_through_nested_T_60(i) });
        ({ test_roundtrip_through_buffer_26(i) });
        ({ test_roundtrip_through_83(i) });
    }
    let __tmp_16: Value<Vec<f64>> = Rc::new(RefCell::new(Vec::from([
        0_f64,
        1_f64,
        -1_f64,
        10_f64,
        -10_f64,
        1e+10_f64,
        1e-10_f64,
        1e+100_f64,
        1e-100_f64,
        2.718281828459045_f64,
        3.141592653589793_f64,
        f64::NAN,
        unimplemented!(),
        unimplemented!(),
    ])));
    let __range1: Ptr<Vec<f64>> = __tmp_16.as_pointer();
    'loop_: for i in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut i: f64 = (i.read());
        ({ test_roundtrip_through_nested_T_62(i) });
        ({ test_roundtrip_through_buffer_28(i) });
        ({ test_roundtrip_through_84(i) });
        ({ test_roundtrip_through_85(i) });
    }
    {
        {
            let obj: Value<i32> = Rc::new(RefCell::new(3));
            let mut p: AnyPtr = ((obj.as_pointer()) as Ptr<i32>).to_any();
            ({ test_roundtrip_through_nested_T_64((p).clone()) });
            ({ test_roundtrip_through_buffer_30((p).clone()) });
            ({ test_roundtrip_through_86((p).clone()) });
            ({ test_roundtrip_through_87((p).clone()) });
            ({ test_roundtrip_through_88((p).clone()) });
        }
        {
            let obj: Value<i32> = Rc::new(RefCell::new(3));
            let mut p: Ptr<i32> = (obj.as_pointer());
            ({ test_roundtrip_through_nested_T_66((p).clone()) });
            ({ test_roundtrip_through_buffer_32((p).clone()) });
            ({ test_roundtrip_through_89((p).clone()) });
            ({ test_roundtrip_through_90((p).clone()) });
            ({ test_roundtrip_through_91((p).clone()) });
        }
    }
    return true;
}
pub fn basic_constexpr_test_103() -> bool {
    let mut from: i32 = 3;
    let mut middle: Nested_104 = unimplemented!();
    let mut to: i32 = unimplemented!();
    assert!((from == to));
    return true;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct Nested_104 {
    #[offset(0)]
    #[byte_size(4)]
    pub buffer: Box<[i8]>,
}
impl Default for Nested_104 {
    fn default() -> Self {
        {
            Nested_104 {
                buffer: (0..4).map(|_| 0_i8).collect::<Box<[i8]>>(),
            }
        }
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
    ({ tests_102() });
    const _: () = assert!(true, "basic_constexpr_test()");;
    return 0;
}
pub fn __cpp2rust_init_globals() {}
