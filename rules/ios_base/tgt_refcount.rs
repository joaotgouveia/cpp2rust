// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

fn f2(a0: Ptr<::std::fs::File>) -> Ptr<::std::fs::File> {
    dec_refcount(a0)
}

fn f5(a0: Ptr<::std::fs::File>) -> Ptr<::std::fs::File> {
    hex_refcount(a0)
}

fn f81(a0: Ptr<::std::fs::File>) -> Ptr<::std::fs::File> {
    oct_refcount(a0)
}
