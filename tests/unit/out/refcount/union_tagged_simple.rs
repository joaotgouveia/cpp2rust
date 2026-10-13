extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type Kind_enum = u32;
pub const Kind_enum_KIND_NONE: Kind_enum = 0;
pub const Kind_enum_KIND_DONE: Kind_enum = 1;
#[derive(VaArg, FnPtrArg, ByteRepr, DeepClone)]
#[byte_size(8)]
pub struct anon_0 {
    #[offset(0)]
    #[byte_size(8)]
    __bytes: Value<Box<[u8]>>,
}
impl anon_0 {
    pub fn obj(this: Ptr<Self>) -> Ptr<AnyPtr> {
        this.reinterpret_cast()
    }
    pub fn code(this: Ptr<Self>) -> Ptr<i32> {
        this.reinterpret_cast()
    }
}
impl Default for anon_0 {
    fn default() -> Self {
        anon_0 {
            __bytes: Rc::new(RefCell::new(Box::from([0u8; 8]))),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(24)]
pub struct Event {
    #[offset(0)]
    pub kind: Kind_enum,
    #[offset(8)]
    #[byte_size(8)]
    pub handle: AnyPtr,
    #[offset(16)]
    #[byte_size(8)]
    pub payload: anon_0,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let dummy: Value<i32> = Rc::new(RefCell::new(0));
    let m1: Value<Event> = <Value<Event>>::default();
    (*m1.borrow_mut()).kind = Kind_enum_KIND_DONE;
    (*m1.borrow_mut()).handle = ((dummy.as_pointer()) as Ptr<i32>).to_any();
    anon_0::code(field_ptr!(m1.as_pointer(), payload)).write(42);
    {
        &(4usize);
        {
            if (((({ (*m1.borrow()).kind } as u32) == ((Kind_enum_KIND_DONE as i32) as u32))
                as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_1(
                        Ptr::<i8>::from_string_literal(b"m1.kind == KIND_DONE"),
                        Ptr::<i8>::from_string_literal(b"union_tagged_simple.c"),
                        25_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    {
        &(4usize);
        {
            if ((((anon_0::code(field_ptr!(m1.as_pointer(), payload)).read()) == 42) as i32) != 0) {
            } else {
                ({
                    __assert_fail_1(
                        Ptr::<i8>::from_string_literal(b"m1.payload.code == 42"),
                        Ptr::<i8>::from_string_literal(b"union_tagged_simple.c"),
                        26_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    let m2: Value<Event> = <Value<Event>>::default();
    (*m2.borrow_mut()).kind = Kind_enum_KIND_NONE;
    (*m2.borrow_mut()).handle = ((dummy.as_pointer()) as Ptr<i32>).to_any();
    anon_0::obj(field_ptr!(m2.as_pointer(), payload))
        .write(((dummy.as_pointer()) as Ptr<i32>).to_any());
    {
        &(4usize);
        {
            if ((({ (anon_0::obj(field_ptr!(m2.as_pointer(), payload)).read()) } == {
                ((dummy.as_pointer()) as Ptr<i32>).to_any()
            }) as i32)
                != 0)
            {
            } else {
                ({
                    __assert_fail_1(
                        Ptr::<i8>::from_string_literal(b"m2.payload.obj == &dummy"),
                        Ptr::<i8>::from_string_literal(b"union_tagged_simple.c"),
                        32_u32,
                        Ptr::<i8>::from_string_literal(b"int main(void)"),
                    )
                });
            }
        }
    };
    return 0;
}
pub fn __assert_fail_1(
    mut __assertion: Ptr<i8>,
    mut __file: Ptr<i8>,
    mut __line: u32,
    mut __function: Ptr<i8>,
) {
    unimplemented!()
}
pub fn __cpp2rust_init_globals() {}
