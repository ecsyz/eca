#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(deprecated)]
#![allow(unused_assignments)]
#![allow(non_snake_case)]

use std::{env, fs, path::PathBuf};

use api::log::*;

#[test]
fn test_1() {
    assert_eq!(4, 4);
    _ = env::set_current_dir(r"..\test_data");

    assert!(
        fs::metadata(r"20220630_191551_0000000000.txt").is_ok(),
        "test file with simple lines not found"
    );

    assert!(fs::metadata(r"20220630_191551_2117846337.txt").is_ok());
    assert!(fs::metadata(r"20220630_191551_2117846338.txt").is_ok());
}

#[test]
fn new_ok() {
    _ = env::set_current_dir(r"..\test_data");

    let lr = LogReader::new(PathBuf::from(r"20220630_191551_0000000000.txt"));
    assert!(
        lr.is_ok(),
        "create logreader from '20220630_191551_0000000000.txt' ({lr:?})"
    );

    let lr = LogReader::new(PathBuf::from(r"20220630_191551_2117846337.txt"));
    assert!(
        lr.is_ok(),
        "create logreader from '20220630_191551_2117846337.txt' ({lr:?})"
    );

    let lr = LogReader::new(PathBuf::from(r"20220630_191551_2117846338.txt"));
    assert!(
        lr.is_ok(),
        "create logreader from '20220630_191551_2117846338.txt' ({lr:?})"
    );
}

#[test]
fn new_err() {
    // file not exists
    let lr = LogReader::new(PathBuf::from(r"20220630_191551_2117846338.txt"));
    assert!(lr.is_err(), "{lr:?}");

    // only launcher log
    let lr = LogReader::new(PathBuf::from(r"..\test_data\bad_log\20220101_111111.txt"));
    assert!(lr.is_err(), "{lr:?}");

    // Bad Header #1
    let lr = LogReader::new(PathBuf::from(
        r"..\test_data\bad_log\20220101_111111_0000000001.txt",
    ));
    assert!(lr.is_err(), "{lr:?}");

    // Bad Header #2
    let lr = LogReader::new(PathBuf::from(
        r"..\test_data\bad_log\20220101_111111_0000000002.txt",
    ));
    assert!(lr.is_err(), "{lr:?}");

    // // Bad Header #3 - cant detect language
    // let lr = LogReader::new(PathBuf::from(
    //     r"..\test_data\bad_log\20220101_111111_0000000003.txt",
    // ));
    // assert!(lr.is_err(), "{lr:?}");
}

// #[test]
// #[should_panic]
// fn new_panic_1() {}
