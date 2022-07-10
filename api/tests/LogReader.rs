#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(deprecated)]
#![allow(unused_assignments)]
#![allow(non_snake_case)]

use std::{collections::HashMap, env, fs, path::PathBuf};

use api::log::*;

fn lr_pb_new(s: &str) -> Result<LogReader, &'static str> {
    let path = r"..\test_data\";
    LogReader::new(PathBuf::from(format!("{path}{s}")))
}

#[test]
fn new_ok() {
    let list = HashMap::from([
        (
            r"20220630_191551_0000000000.txt",
            "20220630_191551_0000000000.txt",
        ),
        (
            r"20220630_191551_4586487627.txt",
            "20220630_191551_4586487627.txt",
        ),
        (
            r"20220630_191551_8779804650.txt",
            "20220630_191551_8779804650.txt",
        ),
    ]);
    for (k, v) in list {
        let res = lr_pb_new(k);
        assert!(res.is_ok(), "{v}\ndump: {res:?}");
    }
}

#[test]
fn new_err() {
    let list = HashMap::from([
        (r"bad_log\20220630_123456_1234567890.txt", "file not exists"),
        (r"bad_log\20220101_111111.txt", "only launcher log"),
        (r"bad_log\20220101_111111_0000000001.txt", "Bad Header #1"),
        (r"bad_log\20220101_111111_0000000002.txt", "Bad Header #2"),
    ]);
    for (k, v) in list {
        let res = lr_pb_new(k);
        assert!(res.is_err(), "{v}\ndump: {res:?}");
    }
}

// #[test]
// #[should_panic]
// fn new_panic_1() {}
