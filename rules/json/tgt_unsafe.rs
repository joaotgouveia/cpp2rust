// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

fn types() {
    let t1: serde_json::Value = serde_json::Value::Null;
    let t2: Vec<serde_json::Value> = Vec::new();
}

fn f1(a0: i64) -> serde_json::Value {
    serde_json::Value::from(a0)
}

fn f2(a0: u64) -> serde_json::Value {
    serde_json::Value::from(a0)
}

fn f3(a0: f64) -> serde_json::Value {
    serde_json::Value::from(a0)
}

fn f4(a0: bool) -> serde_json::Value {
    serde_json::Value::from(a0)
}

fn f5(a0: Ptr<u8>) -> serde_json::Value {
    serde_json::Value::from(a0.to_rust_string())
}

fn f6(mut a0: Vec<serde_json::Value>, a1: serde_json::Value) {
    a0.push(a1)
}

fn f7(a0: Vec<serde_json::Value>) -> serde_json::Value {
    serde_json::Value::Array(a0)
}

fn f8(a0: serde_json::Value) -> Vec<u8> {
    serde_json::to_string(&a0).unwrap().into_bytes()
}
