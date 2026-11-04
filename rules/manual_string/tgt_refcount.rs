// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

fn f1<T1: CChar, T2, T3>(a0: Ptr<::std::fs::File>, a1: Ptr<::std::fs::File>, a2: T3) -> Vec<T1> {
    let mut __bytes: Vec<u8> = Vec::new();
    if !a0.is_null() {
        a0.with_mut(|__f: &mut ::std::fs::File| ::std::io::Read::read_to_end(__f, &mut __bytes))
            .expect("couldn't read the file");
    }
    __bytes.push(0);
    T1::from_byte_vec(__bytes)
}
