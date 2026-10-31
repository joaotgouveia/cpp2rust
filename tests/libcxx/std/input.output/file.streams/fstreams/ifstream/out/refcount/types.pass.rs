extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn get_temp_file_name_0() -> Vec<i8> {
    let Name: Value<Vec<i8>> = Rc::new(RefCell::new({
        let mut __bytes = Ptr::<i8>::from_string_literal(b"libcxx.XXXXXX").to_c_bytes();
        __bytes.push(0_i8);
        __bytes
    }));
    let mut FD: i32 = unimplemented!();
    if (FD == -1_i32) {
        unimplemented!();
        std::process::abort();
    }
    FdRegistry::close(FD);
    return std::mem::take(&mut (*Name.borrow_mut()));
}
pub fn glibc_version_less_than_1(mut version: Ptr<i8>) -> bool {
    let test_version: Value<Vec<i8>> = Rc::new(RefCell::new({
        let mut __tmp2 = {
            let mut __bytes = Ptr::<i8>::from_string_literal(b"glibc ").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        };
        __tmp2.pop();
        version.with_c_str(|__s| __tmp2.extend_from_slice(__s));
        __tmp2.push(0_i8);
        __tmp2
    }));
    let mut n: usize = unimplemented!();
    let mut current_version: Ptr<i8> =
        Ptr::alloc_array((0..n).map(|_| 0_i8).collect::<Box<[i8]>>());
    unimplemented!();
    let result: Value<bool> = Rc::new(RefCell::new((unimplemented!() < 0)));
    current_version.delete();
    return (*result.borrow());
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, CppFrom, Destructor, Default)]
#[cpp_from(usize, new, value)]
#[byte_size(56)]
pub struct UserManagedBuffer {
    #[offset(0)]
    size_: usize,
    #[offset(8)]
    #[byte_size(24)]
    buffers_: Vec<Ptr<i8>>,
    #[offset(32)]
    #[byte_size(24)]
    wbuffers_: Vec<Ptr<i32>>,
}
impl UserManagedBuffer {
    pub fn new(mut size: usize) -> Self {
        Self {
            size_: size,
            buffers_: Vec::new(),
            wbuffers_: Vec::new(),
        }
    }
    pub fn move_from(_a0: Ptr<UserManagedBuffer>) -> Self {
        Self {
            size_: { (*_a0.upgrade().deref()).size_ },
            buffers_: field!(_a0, buffers_).with_mut(std::mem::take),
            wbuffers_: field!(_a0, wbuffers_).with_mut(std::mem::take),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(usize, new, value)]
#[byte_size(8)]
pub struct LibraryManagedBuffer {
    #[offset(0)]
    size_: usize,
}
impl LibraryManagedBuffer {
    pub fn new(mut size: usize) -> Self {
        Self { size_: size }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct LibraryDefaultBuffer {}
impl LibraryDefaultBuffer {
    pub fn new() -> Self {
        Self {}
    }
}
impl Default for LibraryDefaultBuffer {
    fn default() -> Self {
        { LibraryDefaultBuffer::new() }
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
    const _: () = assert!(
        true,
        "(std::is_base_of<std::basic_istream<char>, std::basic_ifstream<char> >::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<std::basic_ifstream<char>::char_type, char>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<std::basic_ifstream<char>::traits_type, std::char_traits<char> >::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<std::basic_ifstream<char>::int_type, std::char_traits<char>::int_type>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<std::basic_ifstream<char>::pos_type, std::char_traits<char>::pos_type>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<std::basic_ifstream<char>::off_type, std::char_traits<char>::off_type>::value)"
    );;
    return 0;
}
pub trait UserManagedBufferImpl {
    fn operator_call_4(&self, stream: Ptr<()>);
    fn operator_call_5(&self, stream: Ptr<()>);
    fn operator_call_6(&self, stream: Ptr<()>);
    fn operator_call_7(&self, stream: Ptr<()>);
    fn destructor(&self);
}
impl UserManagedBufferImpl for Ptr<UserManagedBuffer> {
    fn operator_call_4(&self, stream: Ptr<()>) {
        {
            let __init = Ptr::alloc_array(
                (0..(*self).with(|__s| __s.size_))
                    .map(|_| 0_i8)
                    .collect::<Box<[i8]>>(),
            );
            field!((*self), buffers_).with_mut(|__v: &mut Vec<Ptr<i8>>| __v.push(__init))
        };
        unimplemented!();
    }
    fn operator_call_5(&self, stream: Ptr<()>) {
        {
            let __init = Ptr::alloc_array(
                (0..(*self).with(|__s| __s.size_))
                    .map(|_| 0_i8)
                    .collect::<Box<[i8]>>(),
            );
            field!((*self), buffers_).with_mut(|__v: &mut Vec<Ptr<i8>>| __v.push(__init))
        };
        unimplemented!();
    }
    fn operator_call_6(&self, stream: Ptr<()>) {
        {
            let __init = Ptr::alloc_array(
                (0..(*self).with(|__s| __s.size_))
                    .map(|_| 0_i32)
                    .collect::<Box<[i32]>>(),
            );
            field!((*self), wbuffers_).with_mut(|__v: &mut Vec<Ptr<i32>>| __v.push(__init))
        };
        unimplemented!();
    }
    fn operator_call_7(&self, stream: Ptr<()>) {
        {
            let __init = Ptr::alloc_array(
                (0..(*self).with(|__s| __s.size_))
                    .map(|_| 0_i32)
                    .collect::<Box<[i32]>>(),
            );
            field!((*self), wbuffers_).with_mut(|__v: &mut Vec<Ptr<i32>>| __v.push(__init))
        };
        unimplemented!();
    }
    fn destructor(&self) {
        'loop_: for p in IterRange::new(
            field_ref!((*self), buffers_).decay().begin(),
            field_ref!((*self), buffers_).decay().end(),
        ) {
            let mut p: Ptr<i8> = (p.iter_deref().read());
            p.delete();
        }
        'loop_: for p in IterRange::new(
            field_ref!((*self), wbuffers_).decay().begin(),
            field_ref!((*self), wbuffers_).decay().end(),
        ) {
            let mut p: Ptr<i32> = (p.iter_deref().read());
            p.delete();
        }
    }
}
pub fn __cpp2rust_init_globals() {}
