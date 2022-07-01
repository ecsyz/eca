#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(deprecated)]
#![allow(unused_assignments)]

use api::log::Log;
use std::{collections::HashMap, fs, path::PathBuf};

fn main() {
    println!("Run");

    // eve::log::scan_folder();
    let mut l = Log::new(PathBuf::from(
        r"C:\Users\MyUser\Documents\EVE\logs\Gamelogs\20220628_180637_2117958770.txt",
    ));

    dbg!(&l);
    l.parse();
    // seek_test();

    // use std::thread::sleep;
    // sleep(Duration::from_secs(2));
    println!("End");
}

pub struct DirWatcher {
    path:        PathBuf,
    first_run:   bool,
    ignor_files: Vec<String>,
    act_files:   Vec<String>,
    // users:       HashMap<u32, String>,
    // ignor_users: Vec<String>,
}

impl DirWatcher {
    pub fn new(path: String) -> Self {
        Self {
            path:        PathBuf::from(path),
            first_run:   true,
            ignor_files: Vec::with_capacity(30),
            act_files:   Vec::with_capacity(30),
        }
    }
}

pub fn scan_folder() {
    let mut log_list: HashMap<String, Log> = HashMap::new();

    // let log_dir = PathBuf::from(format!(
    //     "{}{}{}",
    //     "C:\\Users\\",
    //     env!("username"),
    //     "\\Documents\\EVE\\logs\\Gamelogs"
    // ));
    let log_dir = PathBuf::from(r#"E:\D_Backup\workspace\Rust\eve_recount\client\data"#);

    for entry in fs::read_dir(log_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let metadata = fs::metadata(&path).unwrap();

        // println!("---{:?}", path);
        // директории игнорим
        if metadata.is_dir() {
            continue;
        }
        // всё кроме "*.txt" игнорим
        match path.extension() {
            Some(ext) => {
                if ext != "txt" {
                    continue;
                }
            }
            None => continue,
        };

        let fname = path
            .file_name()
            .unwrap()
            .to_str()
            .map(|s| s.to_string())
            .unwrap();

        // правильный файл имеет формат "20220619_201441_2117846337.txt"
        // где 2117846337 - ИД персоонажа, для которого пишется лог.
        if fname.matches('_').count() != 2 {
            continue;
        }

        match log_list.get(&fname) {
            Some(l) => {
                // println!("{:?} - in hash", fname);
            }
            None => {
                // println!("{:?} - not in hash", fname);
                let mut elog = Log::new(path.clone());
                log_list.insert(fname, elog);
            }
        }
    }
    dbg!(&log_list);
}
