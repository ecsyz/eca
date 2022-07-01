use regex;
use regex::Regex;
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
    lastpos:   u64,
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
            lastpos:   0,
            // is_multiline: false,
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

        let time_to_old: u64 = 60 * 360; // 120 minuts
        let cur_time = SystemTime::now()
            .duration_since(time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // помечаем фал для игнора, если он давно не обновлялся
        // if (cur_time - last_modified) > time_to_old {
        //     self.ignored = true;
        //     self.error = "to old".to_string();
        // }
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
                    self.lastpos = offset as u64;
                    // self.lastpos = reader.stream_position().unwrap();
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

        println!("\tfile len: {:?}\nlast offset: {:?}", meta.len(), self.size);
        if meta.len() > self.lastpos {
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

        let file = File::open(&self.fpath).unwrap();
        let mut reader = BufReader::new(file);

        reader.seek(SeekFrom::Start(self.lastpos)).unwrap();

        let mut i = 0;
        let mut line = String::new();
        while let Ok(num) = reader.read_line(&mut line) {
            i = i + 1;
            if num == 0 {
                // EOF check
                break;
            }

            self.parse_line(line.clone());

            if i == 2 {
                break;
            }

            line.clear();
        }
        println!("parse: end");
        println!("end: {:?}", &reader.stream_position().unwrap());
        println!("fsize: {:?}", fs::metadata(&self.fpath).unwrap().len());
    }

    fn parse_line<'a>(&mut self, line: String) {
        //-> Vec<&str>
        let nline = self.log_line_normalizer(line.as_str());

        let mut arr: Vec<&str> = nline.split("█").collect();

        dbg!(&arr);
        // arr
    }

    fn log_line_normalizer<'a>(&'a mut self, line: &'a str) -> &'a str {
        line
        // let re1 = regex::Regex::new(r"<.*?>").unwrap();
        // let re2 = regex::Regex::new(r"█?[\s-]+█").unwrap();
        // let re3 = regex::Regex::new(r"█[\s-]+").unwrap();
        // let re4 = regex::Regex::new(r"█+").unwrap();

        // let mut nline = &re1.replace_all(&line, "█").into_owned();
        // nline = &re2.replace_all(&nline, "█").into_owned();
        // nline = &re3.replace_all(&nline, "█").into_owned();
        // nline = &re4.replace_all(&nline, "█").into_owned();

        // nline
    }
}
