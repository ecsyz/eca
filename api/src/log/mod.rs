use regex;
use regex::Regex;
use std::borrow::Cow;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{self, prelude::*, BufReader, SeekFrom};
use std::path::{Path, PathBuf};
use std::result;
use std::thread::sleep;
use std::time::{self, Duration, SystemTime};
use std::{env, fs};

include!("language.rs");

#[derive(Debug)]
pub struct Log {
    fpath:     PathBuf,
    size:      u64,
    offset:    u64,
    modified:  u64,
    // is_multiline: bool,
    user_id:   u32,
    user_name: String,
    ignored:   bool,
    language:  String,
    error:     String,
}

impl Log {
    pub fn new(fpath: PathBuf) -> Self {
        let meta = fpath.metadata().unwrap();

        let mut this = Self {
            fpath:     fpath,
            size:      meta.len(),
            offset:    0,
            modified:  0,
            user_id:   0,
            user_name: "".into(),
            ignored:   false,
            language:  "".into(),
            error:     "".into(),
        };

        this.check_modtime();
        this.check_filename();
        this.check_header();

        this
    }

    fn check_modtime(&mut self) {
        if self.ignored {
            return;
        }

        let meta = self.fpath.metadata().unwrap();
        let last_modified = meta
            .modified()
            .unwrap()
            .duration_since(time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // let time_to_old: u64 = 60 * 360; // 120 minuts
        // let cur_time = SystemTime::now()
        //     .duration_since(time::UNIX_EPOCH)
        //     .unwrap()
        //     .as_secs();

        self.modified = last_modified;
    }

    fn check_filename(&mut self) {
        if self.ignored {
            return;
        }

        // выдираем из названия файла UserId
        let fname = self
            .fpath
            .file_name()
            .unwrap()
            .to_str()
            .map(|s| s.to_string())
            .unwrap();

        // 20220620_193132_2117846337
        let re_user_id = regex::Regex::new(r"^\d{8}_\d{6}_(\d+).txt$").unwrap();
        if let Some(cap) = re_user_id.captures(&fname) {
            self.user_id = cap.get(1).unwrap().as_str().parse::<u32>().unwrap();
        } else {
            self.error = "bad file name format(timestamp + user_id)".to_string();
            self.ignored = true;
        }
    }

    fn check_header(&mut self) {
        if self.ignored {
            return;
        }

        let lang: Vec<LogLanguage> = generate_language();

        let file = File::open(&self.fpath).unwrap();
        let mut reader = BufReader::new(file);

        let mut err = false;
        //log header example:
        //1   ------------------------------------------------------------
        //2     Gamelog
        //3     Listener: Some UserXY
        //4     Session Started: 2022.01.12 20:00:00
        //5   ------------------------------------------------------------

        let mut line = String::new();

        // println!("{:?}\n", self.fpath.file_name().unwrap());
        let mut i = -1;
        let mut offset: usize = 0;
        while let Ok(num) = reader.read_line(&mut line) {
            i = i + 1;

            println!(
                "i:{:3}|of: {:5} +{:4}->{:5}| s: {}",
                &i,
                &offset,
                &num,
                (offset + num),
                &line.trim_end()
            );

            offset = offset + num;
            if num == 0 {
                // EOF check
                line.clear();
                break;
            }

            match i {
                0 => {
                    if line.contains("------------") == false {
                        err = true;
                    }
                }
                1 => {}
                2 => {
                    // println!("\tline:{:?}", line.trim());
                    let mut m = 0;
                    for re in &lang {
                        if !re.character.is_match(&line) {
                            continue;
                        }

                        if let Some(cap) = re.character.captures(&line) {
                            // println!("\t+ {:?}", re.language);
                            self.language = re.language.clone();
                            self.user_name = cap.get(1).unwrap().as_str().to_owned();
                            break;
                        } else {
                            // println!("\t- {:?}", re.language);
                        }
                    }
                }
                3 => {}
                4 => {
                    if !line.contains("------------") {
                        err = true;
                    }
                    self.offset = offset as u64;
                    // self.offset = reader.stream_position().unwrap();
                }
                _ => {
                    break;
                }
            };

            line.clear();
        }

        if err {
            self.error.push_str("error in header parse");
        }
    }

    fn is_changed(&mut self) -> bool {
        let meta = self.fpath.metadata().unwrap();

        let modified = meta
            .modified()
            .unwrap()
            .duration_since(time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        if meta.len() > self.offset {
            self.size = meta.len();
            return true;
        } else {
            return false;
        }
    }

    // reader.seek(SeekFrom::Start(1653147)).unwrap();
    // let current_pos = reader
    //     .seek(SeekFrom::Current(0))
    //     .expect("Could not get current position!");
    pub fn parse(&mut self) {
        println!("parse: start");
        // fs::metadata(&path).unwrap();
        // let meta = self.fpath.metadata().unwrap();

        if !self.is_changed() {
            println!("parse: end(not changed)");
            return;
        }

        // FIXME: надо будет ещё добавить проверку на уменьшение файла с момента последнего чтения. если такое произошло, то наверно надо вообще пересоздавать объект или помечать этот как error

        let file = File::open(&self.fpath).unwrap();
        let mut reader = BufReader::new(file);

        reader.seek(SeekFrom::Start(self.offset)).unwrap();

        let mut i = 0;
        let mut line = String::new();
        while let Ok(num) = reader.read_line(&mut line) {
            if num == 0 {
                // EOF check
                line.clear();
                break;
            }
            i = i + 1;
            self.offset = self.offset + (num as u64);

            // self.parse_line(line.clone());
            println!("----------------------------------------------------");
            println!("{:?}", self.log_line_normalizer(&line));

            line.clear();
            // важный момент - читаем не пока reader выдаёт нам строки, а только до self.size.
            // если во время нашего чтения из файла в него будут добавлены ещё строки, то reader сможет прочитать и их, без пересоздания BufReader, а это выдаёт некоторые артефакты.
            if self.size == self.offset {
                break;
            } else if self.offset > self.size {
                panic!("read file: self.offset > self.size");
            }
        }
        println!("parse: end");
    }

    fn parse_line<'a>(&mut self, line: String) {
        //-> Vec<&str>
        let nline = self.log_line_normalizer(&line);

        let mut arr: Vec<&str> = nline.split("█").collect();

        dbg!(&arr);
        // arr
    }
    fn log_line_normalizer<'a>(&'a mut self, line: &String) -> String {
        let re1 = regex::Regex::new(r"<.*?>").unwrap();
        let re2 = regex::Regex::new(r"█?[\s-]+█").unwrap();
        let re3 = regex::Regex::new(r"█[\s-]+").unwrap();
        let re4 = regex::Regex::new(r"█+").unwrap();

        let mut nline: Cow<str> = line.into();

        nline = replaceall_cow(nline, &re1, "█");
        nline = replaceall_cow(nline, &re2, "█");
        nline = replaceall_cow(nline, &re3, "█");
        nline = replaceall_cow(nline, &re4, "█");

        nline.into_owned()
    }
}

fn replaceall_cow<'a>(cow: Cow<'a, str>, regex: &Regex, replacement: &str) -> Cow<'a, str> {
    match cow {
        Cow::Borrowed(s) => regex.replace_all(s, replacement),
        Cow::Owned(s) => Cow::Owned(regex.replace_all(&s, replacement).into_owned()),
    }
}
fn dbg<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
