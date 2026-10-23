extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn get_temp_file_name_0() -> () {
    let mut Name: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
        { Ptr::<()>::from_string_literal(b"libcxx.XXXXXX") },
        None,
    );
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
    let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    {
        let f: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let f: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    ({ remove_7(unimplemented!()) });
    {
        let f: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let f: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    ({ remove_7(unimplemented!()) });
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
