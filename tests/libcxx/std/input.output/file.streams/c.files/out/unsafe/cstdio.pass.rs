extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;

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
    let mut fp: *mut ::libc::FILE = std::ptr::null_mut();
    let mut fpos: ::libc::fpos_t = unsafe { std::mem::zeroed() };
    let mut s: () = 0_();
    let mut cp: *mut () = std::ptr::null_mut();
    let mut va: *const VaArg = ::std::ptr::null();
    let mut vp: () = unimplemented!();
    (&(fp));
    (&(fpos));
    (&(s));
    (&(cp));
    (&(va));
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
pub unsafe fn __cpp2rust_init_globals() {}
