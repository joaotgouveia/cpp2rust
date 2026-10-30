// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

fn f7<T1, T2>(a0: Ptr<::std::fs::File>, a1: bool) -> Ptr<::std::fs::File> {
    let __s = a0;
    let _ = write!(__s, "{}", a1 as i32);
    __s
}

fn f10<T1, T2>(a0: Ptr<::std::fs::File>, a1: i32) -> Ptr<::std::fs::File> {
    let __s = a0;
    let _ = write!(__s, "{}", a1);
    __s
}
