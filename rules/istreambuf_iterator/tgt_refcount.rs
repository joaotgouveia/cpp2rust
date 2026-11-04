// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

fn t1<T1, T2>() -> Ptr<::std::fs::File> {
    Ptr::null()
}

fn f3<T1, T2>(a0: Ptr<::std::fs::File>) -> Ptr<::std::fs::File> {
    a0
}

fn f5<T1, T2>(a0: Ptr<::std::fs::File>) -> Ptr<::std::fs::File> {
    a0
}
