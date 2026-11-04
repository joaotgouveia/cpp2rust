// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

fn f1(a0: Ptr<::std::fs::File>) -> Ptr<::std::fs::File> {
    boolalpha_refcount(a0)
}

fn f2(a0: Ptr<::std::fs::File>) -> Ptr<::std::fs::File> {
    dec_refcount(a0)
}

fn f3(a0: Ptr<::std::fs::File>) -> Ptr<::std::fs::File> {
    defaultfloat_refcount(a0)
}

fn f4(a0: Ptr<::std::fs::File>) -> Ptr<::std::fs::File> {
    fixed_refcount(a0)
}

fn f5(a0: Ptr<::std::fs::File>) -> Ptr<::std::fs::File> {
    hex_refcount(a0)
}

fn f6(a0: Ptr<::std::fs::File>) -> Ptr<::std::fs::File> {
    hexfloat_refcount(a0)
}

fn f7(a0: Ptr<::std::fs::File>) -> Ptr<::std::fs::File> {
    internal_refcount(a0)
}

fn f71(a0: Ptr<::std::fs::File>) -> Ptr<::std::fs::File> {
    left_refcount(a0)
}

fn f74(a0: Ptr<::std::fs::File>) -> Ptr<::std::fs::File> {
    noboolalpha_refcount(a0)
}

fn f75(a0: Ptr<::std::fs::File>) -> Ptr<::std::fs::File> {
    noshowbase_refcount(a0)
}

fn f76(a0: Ptr<::std::fs::File>) -> Ptr<::std::fs::File> {
    noshowpoint_refcount(a0)
}

fn f77(a0: Ptr<::std::fs::File>) -> Ptr<::std::fs::File> {
    noshowpos_refcount(a0)
}

fn f78(a0: Ptr<::std::fs::File>) -> Ptr<::std::fs::File> {
    noskipws_refcount(a0)
}

fn f79(a0: Ptr<::std::fs::File>) -> Ptr<::std::fs::File> {
    nounitbuf_refcount(a0)
}

fn f80(a0: Ptr<::std::fs::File>) -> Ptr<::std::fs::File> {
    nouppercase_refcount(a0)
}

fn f81(a0: Ptr<::std::fs::File>) -> Ptr<::std::fs::File> {
    oct_refcount(a0)
}

fn f103(a0: Ptr<::std::fs::File>) -> Ptr<::std::fs::File> {
    right_refcount(a0)
}

fn f104(a0: Ptr<::std::fs::File>) -> Ptr<::std::fs::File> {
    scientific_refcount(a0)
}

fn f105(a0: Ptr<::std::fs::File>) -> Ptr<::std::fs::File> {
    showbase_refcount(a0)
}

fn f106(a0: Ptr<::std::fs::File>) -> Ptr<::std::fs::File> {
    showpoint_refcount(a0)
}

fn f107(a0: Ptr<::std::fs::File>) -> Ptr<::std::fs::File> {
    showpos_refcount(a0)
}

fn f108(a0: Ptr<::std::fs::File>) -> Ptr<::std::fs::File> {
    skipws_refcount(a0)
}

fn f109(a0: Ptr<::std::fs::File>) -> Ptr<::std::fs::File> {
    unitbuf_refcount(a0)
}

fn f110(a0: Ptr<::std::fs::File>) -> Ptr<::std::fs::File> {
    uppercase_refcount(a0)
}

fn f47() -> i32 {
    1_i32
}

fn f48() -> i32 {
    2_i32
}

fn f49() -> i32 {
    4_i32
}

fn f50() -> i32 {
    8_i32
}

fn f51() -> i32 {
    16_i32
}

fn f52() -> i32 {
    32_i32
}

fn f53() -> i32 {
    64_i32
}
