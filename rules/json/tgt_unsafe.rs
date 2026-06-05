// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

fn t1() -> serde_json::Value {
    serde_json::Value::Null
}

fn t2() -> Vec<serde_json::Value> {
    Vec::new()
}

fn f1<T1: Into<serde_json::Value>>(a0: T1) -> serde_json::Value {
    a0.into()
}

fn f2(a0: Ptr<u8>) -> serde_json::Value {
    serde_json::Value::from(a0.to_rust_string())
}

fn f3() -> Vec<serde_json::Value> {
    Vec::new()
}

fn f4(mut a0: Vec<serde_json::Value>, a1: serde_json::Value) {
    a0.push(a1)
}

fn f5(mut a0: Vec<serde_json::Value>, a1: serde_json::Value) {
    a0.push(a1)
}

fn f6(a0: Vec<serde_json::Value>) -> serde_json::Value {
    serde_json::Value::Array(a0.clone())
}

fn f7(a0: serde_json::Value) -> Vec<u8> {
    let mut str = serde_json::to_string(&a0).unwrap().into_bytes();
    str.push(0);
    str
}
