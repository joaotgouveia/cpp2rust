extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn get_temp_file_name_0() -> () {
    let mut Name: () = unimplemented!();
    let mut FD: () = ({ mkstemp_1((unimplemented!())) });
    if (FD == -1_()) {
        ({ perror_2(Ptr::<()>::from_string_literal(b"mkstemp")) });
        unimplemented!();
    }
    unimplemented!();
    return unimplemented!();
}
pub fn glibc_version_less_than_3(mut version: Ptr<()>) -> () {
    let mut test_version: () = unimplemented!();
    let mut n: () = ({ confstr_4((anon_5__CS_GNU_LIBC_VERSION as ()), Ptr::<()>::null(), 0_()) });
    let mut current_version: Ptr<()> =
        Ptr::alloc_array((0..n).map(|_| unimplemented!()).collect::<Box<[()]>>());
    ({
        confstr_4(
            (anon_5__CS_GNU_LIBC_VERSION as ()),
            (current_version).clone(),
            n,
        )
    });
    let result: Value<()> = Rc::new(RefCell::new(
        (({ strverscmp_6((current_version).clone(), unimplemented!()) }) < 0_()),
    ));
    current_version.delete();
    return (*result.borrow());
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, Destructor, Default)]
#[byte_size(56)]
pub struct UserManagedBuffer {
    #[offset(0)]
    size_: (),
    #[offset(8)]
    #[byte_size(24)]
    buffers_: (),
    #[offset(32)]
    #[byte_size(24)]
    wbuffers_: (),
}
impl UserManagedBuffer {
    pub fn new(mut size: ()) -> Self {
        Self {
            size_: size,
            buffers_: unimplemented!(),
            wbuffers_: unimplemented!(),
        }
    }
    pub fn move_from(_a0: Ptr<UserManagedBuffer>) -> Self {
        Self {
            size_: { (*_a0.upgrade().deref()).size_ },
            buffers_: unimplemented!(),
            wbuffers_: unimplemented!(),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct LibraryManagedBuffer {
    #[offset(0)]
    size_: (),
}
impl LibraryManagedBuffer {
    pub fn new(mut size: ()) -> Self {
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    const _: () = assert!(
        true,
        "(std::is_base_of<std::basic_ostream<char>, std::basic_ofstream<char> >::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<std::basic_ofstream<char>::char_type, char>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<std::basic_ofstream<char>::traits_type, std::char_traits<char> >::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<std::basic_ofstream<char>::int_type, std::char_traits<char>::int_type>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<std::basic_ofstream<char>::pos_type, std::char_traits<char>::pos_type>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<std::basic_ofstream<char>::off_type, std::char_traits<char>::off_type>::value)"
    );;
    return 0_();
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
        unimplemented!();
        unimplemented!();
    }
    fn operator_call_5(&self, stream: Ptr<()>) {
        unimplemented!();
        unimplemented!();
    }
    fn operator_call_6(&self, stream: Ptr<()>) {
        unimplemented!();
        unimplemented!();
    }
    fn operator_call_7(&self, stream: Ptr<()>) {
        unimplemented!();
        unimplemented!();
    }
    fn destructor(&self) {
        'loop_: for p in IterRange::new(unimplemented!(), unimplemented!()) {
            let mut p: Ptr<()> = (unimplemented!().read());
            p.delete();
        }
        'loop_: for p in IterRange::new(unimplemented!(), unimplemented!()) {
            let mut p: Ptr<()> = (unimplemented!().read());
            p.delete();
        }
    }
}
pub fn __cpp2rust_init_globals() {}
