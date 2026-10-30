extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};

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
    let mut fp: () = unimplemented!();
    let fpos: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut s: () = 0_();
    let mut cp: Ptr<()> = Ptr::<()>::null();
    let va: Value<Ptr<VaArg>> = Rc::new(RefCell::new(Ptr::<VaArg>::default()));
    let mut vp: () = unimplemented!();
    (&(fp));
    (&(*fpos.borrow_mut()));
    (&(s));
    (&(cp));
    (&(*va.borrow_mut()));
    (&(vp));
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::fclose(fp)), int>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::fflush(fp)), int>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::setbuf(fp,cp)), void>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::vfprintf(fp,\" \",va)), int>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::fprintf(fp,\" \")), int>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::fscanf(fp,\" \")), int>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::snprintf(cp,0,\" \")), int>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::sprintf(cp,\" \")), int>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::sscanf(\"\",\" \")), int>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::vfprintf(fp,\" \",va)), int>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::vfscanf(fp,\" \",va)), int>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::vsnprintf(cp,0,\" \",va)), int>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::vsprintf(cp,\" \",va)), int>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::vsscanf(\"\",\" \",va)), int>::value)"
    );;
    const _: () = assert!(true, "(std::is_same<decltype(std::fgetc(fp)), int>::value)");;
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::fgets(cp,0,fp)), char*>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::fputc(0,fp)), int>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::fputs(\"\",fp)), int>::value)"
    );;
    const _: () = assert!(true, "(std::is_same<decltype(std::getc(fp)), int>::value)");;
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::putc(0,fp)), int>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::ungetc(0,fp)), int>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::fread((void*)0,0,0,fp)), std::size_t>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::fwrite(vp,0,0,fp)), std::size_t>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::fgetpos(fp, &fpos)), int>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::fseek(fp, 0,0)), int>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::fsetpos(fp, &fpos)), int>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::ftell(fp)), long>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::rewind(fp)), void>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::clearerr(fp)), void>::value)"
    );;
    const _: () = assert!(true, "(std::is_same<decltype(std::feof(fp)), int>::value)");;
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::ferror(fp)), int>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::perror(\"\")), void>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::fopen(\"\", \"\")), std::FILE*>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::freopen(\"\", \"\", fp)), std::FILE*>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::remove(\"\")), int>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::rename(\"\",\"\")), int>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::tmpfile()), std::FILE*>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::tmpnam(cp)), char*>::value)"
    );;
    const _: () = assert!(true, "(std::is_same<decltype(std::getchar()), int>::value)");;
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::scanf(\" \")), int>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::vscanf(\" \",va)), int>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::printf(\" \")), int>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::putchar(0)), int>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::puts(\"\")), int>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::vprintf(\" \",va)), int>::value)"
    );;
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
