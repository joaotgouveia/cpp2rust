// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

fn f7<T1>(a0: Ptr<::std::fs::File>, a1: Ptr<i8>) -> Ptr<::std::fs::File> {
    let __s = a0;
    let _ = a1.with_c_str(|__cs| __s.write_all(&__cs.iter().map(|&__c| __c as u8).collect::<Vec<u8>>()));
    __s
}

fn f10<T1>(a0: Ptr<::std::fs::File>, a1: i8) -> Ptr<::std::fs::File> {
    let __s = a0;
    let _ = __s.write_all(&[a1 as u8]);
    __s
}

fn f11<T1>(a0: Ptr<::std::fs::File>, a1: u8) -> Ptr<::std::fs::File> {
    let __s = a0;
    let _ = __s.write_all(&[a1]);
    __s
}

fn f2<T1, T2>(a0: Ptr<::std::fs::File>) -> Ptr<::std::fs::File> {
    endl_refcount(a0)
}

fn f3<T1, T2>(a0: Ptr<::std::fs::File>) -> Ptr<::std::fs::File> {
    ends_refcount(a0)
}

fn f4<T1, T2>(a0: Ptr<::std::fs::File>) -> Ptr<::std::fs::File> {
    flush_refcount(a0)
}
