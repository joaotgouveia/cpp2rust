// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

fn f7<T1, T2>(a0: Ptr<::std::fs::File>, a1: bool) -> Ptr<::std::fs::File> {
    let __s = a0;
    let _ = write!(__s, "{}", a1 as i32);
    __s
}

fn f2<T1, T2>(a0: Ptr<::std::fs::File>, a1: FnPtr<fn(Ptr<::std::fs::File>) -> Ptr<::std::fs::File>>) -> Ptr<::std::fs::File> {
    let __s = a0;
    a1.call(__s)
}

fn f4<T1, T2>(a0: Ptr<::std::fs::File>, a1: FnPtr<fn(Ptr<::std::fs::File>) -> Ptr<::std::fs::File>>) -> Ptr<::std::fs::File> {
    let __s = a0;
    a1.call(__s)
}

fn f5<T1, T2>(a0: Ptr<::std::fs::File>, a1: i64) -> Ptr<::std::fs::File> {
    let __s = a0;
    write_int_refcount(&__s, a1);
    __s
}

fn f6<T1, T2>(a0: Ptr<::std::fs::File>, a1: u64) -> Ptr<::std::fs::File> {
    let __s = a0;
    write_int_refcount(&__s, a1);
    __s
}

fn f8<T1, T2>(a0: Ptr<::std::fs::File>, a1: i16) -> Ptr<::std::fs::File> {
    let __s = a0;
    write_int_refcount(&__s, a1);
    __s
}

fn f9<T1, T2>(a0: Ptr<::std::fs::File>, a1: u16) -> Ptr<::std::fs::File> {
    let __s = a0;
    write_int_refcount(&__s, a1);
    __s
}

fn f10<T1, T2>(a0: Ptr<::std::fs::File>, a1: i32) -> Ptr<::std::fs::File> {
    let __s = a0;
    write_int_refcount(&__s, a1);
    __s
}

fn f11<T1, T2>(a0: Ptr<::std::fs::File>, a1: u32) -> Ptr<::std::fs::File> {
    let __s = a0;
    write_int_refcount(&__s, a1);
    __s
}

fn f12<T1, T2>(a0: Ptr<::std::fs::File>, a1: i64) -> Ptr<::std::fs::File> {
    let __s = a0;
    write_int_refcount(&__s, a1);
    __s
}

fn f13<T1, T2>(a0: Ptr<::std::fs::File>, a1: u64) -> Ptr<::std::fs::File> {
    let __s = a0;
    write_int_refcount(&__s, a1);
    __s
}
