// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

fn f2<T1, T2>(a0: Ptr<i8>) -> ::std::fs::File {
    ::std::fs::File::open(a0.to_string()).expect("Failed to open file")
}
