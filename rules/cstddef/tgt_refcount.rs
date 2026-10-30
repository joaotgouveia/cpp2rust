// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

fn f2(a0: Ptr<u8>, a1: u8) -> Ptr<u8> {
    a0.write(a0.read() & a1);
    a0
}

fn f4<T1>(a0: Ptr<u8>, a1: T1) -> Ptr<u8>
where
    u8: ::std::ops::Shl<T1, Output = u8>,
{
    a0.write(a0.read() << a1);
    a0
}

fn f6<T1>(a0: Ptr<u8>, a1: T1) -> Ptr<u8>
where
    u8: ::std::ops::Shr<T1, Output = u8>,
{
    a0.write(a0.read() >> a1);
    a0
}

fn f8(a0: Ptr<u8>, a1: u8) -> Ptr<u8> {
    a0.write(a0.read() ^ a1);
    a0
}

fn f10(a0: Ptr<u8>, a1: u8) -> Ptr<u8> {
    a0.write(a0.read() | a1);
    a0
}
