use regex;
use regex::Regex;
use std::borrow::Cow;
use std::collections::{HashMap, VecDeque};
use std::error::Error;
use std::fs::{File, Metadata};
use std::io::{self, prelude::*, BufReader, SeekFrom};
use std::path::{Path, PathBuf};
use std::result;
use std::thread::sleep;
use std::time::{self, Duration, SystemTime};
use std::{env, fs};

include!("language.rs");

#[derive(Debug)]
pub struct Log {
    log_reader:    LogReader,
    file_path:     PathBuf,
    pub user_id:   u64,
    pub user_name: String,
    pub ignored:   bool,
    pub language:  &'static str,
}

impl Log {
    pub fn new(file_path: PathBuf) -> Result<Self, &'static str> {
        let meta: Metadata;
        match file_path.metadata() {
            Err(e) => {
                // e.to_string().as_str()
                return Err("cant get file metadata");
            }
            Ok(v) => {
                meta = v;
            }
        }

        let mut lreader: LogReader;
        //FIXME: закидывать наверно можно тупо ссылку
        match LogReader::new(file_path.clone()) {
            Err(e) => return Err(e),
            Ok(v) => {
                lreader = v;
            }
        }

        let details: (String, u64);
        match lreader.get_details() {
            Err(e) => return Err(e),
            Ok(v) => {
                details = v;
            }
        }

        let language: &'static str;
        match lreader.get_language() {
            Err(e) => return Err(e),
            Ok(v) => {
                language = v;
            }
        }

        let mut this = Self {
            log_reader: lreader,
            file_path:  file_path,
            user_name:  details.0,
            user_id:    details.1,
            ignored:    false,
            language:   language,
        };

        Ok(this)
    }

    pub fn is_changed(&mut self) -> bool {
        self.log_reader.is_changed()
    }

    pub fn read_allines(&mut self) -> Vec<String> {
        self.log_reader.read_allines()
    }

    pub fn parse(&mut self) {
        println!("parse: start");

        if !self.is_changed() {
            println!("parse: end(not changed)");
            return;
        }

        // FIXME: надо будет ещё добавить проверку на уменьшение файла с момента последнего чтения. если такое произошло, то наверно надо вообще пересоздавать объект или помечать этот как error

        let mut line = String::new();
        loop {
            line.clear();
            match self.log_reader.read_multiline() {
                Some(v) => {
                    line = v;
                }
                None => break,
            }

            self.parse_line(&line);
        }
        println!("parse: end");
    }

    fn log_line_normalizer<'a>(&'a mut self, line: &String) -> String {
        let re0 = regex::Regex::new(r"[\r\n]+").unwrap();
        let re1 = regex::Regex::new(r"<.*?>").unwrap();
        let re2 = regex::Regex::new(r"█?[\s-]+█").unwrap();
        let re3 = regex::Regex::new(r"█[\s-]+").unwrap();
        let re4 = regex::Regex::new(r"█+").unwrap();
        let re5 = regex::Regex::new(r"[█\.,*]+$").unwrap();
        let re6 = regex::Regex::new(r"\s-\s").unwrap(); //<Weapon_name> - <Hit_type>
        let re7 = regex::Regex::new(r"[\.,*]*█[\.,\+\-*]*").unwrap();

        let mut nline: Cow<str> = line.into();

        nline = replaceall_cow(nline, &re0, "");
        nline = replaceall_cow(nline, &re1, "█");
        nline = replaceall_cow(nline, &re2, "█");
        nline = replaceall_cow(nline, &re3, "█");
        nline = replaceall_cow(nline, &re4, "█");
        nline = replaceall_cow(nline, &re5, "");
        nline = replaceall_cow(nline, &re7, "█");

        nline.into_owned()
    }

    fn parse_line(&mut self, line: &String) {
        //-> Vec<&str>
        // println!(":4:{}", &line);
        let nline = self.log_line_normalizer(&line);

        let mut arr: Vec<&str> = nline.split("█").collect();
        // println!(
        //     ":{}:{:?}\n\t{:?} | {:?}",
        //     arr.len(),
        //     &nline,
        //     &arr[0][2..21],
        //     &arr[0][25..31]
        // );

        let time = &arr[0][2..21];
        let message_type = &arr[0][25..31];

        let lang = LOGLANGUAGE.get(self.language).unwrap();

        match message_type {
            "combat" => {
                // println!("\n:{}:{:?}", arr.len(), &nline);

                match arr.len() {
                    5 => {
                        // :5:"[ 2022.06.30 19:47:05 ] (combat)█39█from█Anchoring Damavik█Glances Off"
                        // <0:time+other> | <1:damage> | <2:from> | <3:enemy_name> | <4:hit_type>
                        //
                        // :5:"[ 2022.06.30 19:33:15 ] (combat)█20 GJ█energy neutralized█Faded Hypnosian Warden█Faded Hypnosian Warden"
                        // <0:time+other> | <1:energy_amounte> | <2:action> | <3:enemy_name> | <4:enemy_name>
                        if arr[2] == lang.damage_in {
                            println!("\ndamage_in_pve\n:{}:{nline:?}\n{arr:?}", arr.len());
                        } else if arr[2] == lang.cap_neutralized_in {
                            println!("\ncap_neut_in\n:{}:{nline:?}\n{arr:?}", arr.len());
                        } else {
                            println!("\n5:UNSUPORTED\n:{}:{nline:?}\n{arr:?}", arr.len());
                        }
                    }
                    6 => {
                        // <0:time+other> | <1:damage> | <2:to|from|action> | <3:enemy_name> | <4:weapon_type> | <5:hit_type>
                        if arr[2] == lang.damage_in && arr[4] == lang.damage_out {
                            // :6:"[ 2022.01.19 18:31:06 ] (combat)█Warp scramble attempt█from█Raznaborg Anchoring Damavik█to█State Navy Rook"
                            println!("\nmodule use\n:{}:{nline:?}\n{arr:?}", arr.len());
                        } else if arr[2] == lang.damage_out {
                            println!("\ndamage_out\n:{}:{nline:?}\n{arr:?}", arr.len());
                        } else if arr[2] == lang.damage_in {
                            println!("\ndamage_in_pvp\n:{}:{nline:?}\n{arr:?}", arr.len());
                        } else {
                            // UNSUPORTED
                            println!("\n6:UNSUPORTED\n:{}:{nline:?}\n{arr:?}", arr.len());
                        }
                    }
                    8 => {
                        if arr[2] == lang.armor_repaired_out {
                            println!("\narmor_repaired_out\n:{}:{nline:?}\n{arr:?}", arr.len());
                        } else if arr[2] == lang.hull_repaired_out {
                            println!("\nhull_repaired_out\n:{}:{nline:?}\n{arr:?}", arr.len());
                        } else if arr[2] == lang.shield_boosted_out {
                            println!("\nshield_boosted_out\n:{}:{nline:?}\n{arr:?}", arr.len());
                        } else if arr[2] == lang.armor_repaired_in {
                            println!("\narmor_repaired_in\n:{}:{nline:?}\n{arr:?}", arr.len());
                        } else if arr[2] == lang.hull_repaired_in {
                            println!("\nhull_repaired_in\n:{}:{nline:?}\n{arr:?}", arr.len());
                        } else if arr[2] == lang.shield_boosted_in {
                            println!("\nshield_boosted_in\n:{}:{nline:?}\n{arr:?}", arr.len());
                        } else if arr[2] == lang.cap_transfered_out {
                            println!("\ncap_transfered_out\n:{}:{nline:?}\n{arr:?}", arr.len());
                        } else if arr[2] == lang.cap_neutralized_out {
                            println!("\ncap_neutralized_out\n:{}:{nline:?}\n{arr:?}", arr.len());
                        } else if arr[2] == lang.nos_recieved {
                            println!("\nnos_recieved\n:{}:{nline:?}\n{arr:?}", arr.len());
                        } else if arr[2] == lang.cap_transfered_in {
                            println!("\ncap_transfered_in\n:{}:{nline:?}\n{arr:?}", arr.len());
                        } else if arr[2] == lang.cap_neutralized_in {
                            println!("\ncap_neutralized_in\n:{}:{nline:?}\n{arr:?}", arr.len());
                        } else if arr[2] == lang.nos_taken {
                            println!("\nnos_taken\n:{}:{nline:?}\n{arr:?}", arr.len());
                        } else {
                            // UNSUPORTED
                            println!("\n8:UNSUPORTED\n:{}:{nline:?}\n{arr:?}", arr.len());
                        }
                    }
                    _ => {}
                }
                // damage_out,          :r#"to"#,
                // damage_in,           :r#"from"#,
                // armor_repaired_out,  :r#"remote armor repaired to"#,
                // hull_repaired_out,   :r#"remote hull repaired to"#,
                // shield_boosted_out,  :r#"remote shield boosted to"#,
                // armor_repaired_in,   :r#"remote armor repaired by"#,
                // hull_repaired_in,    :r#"remote hull repaired by"#,
                // shield_boosted_in,   :r#"remote shield boosted by"#,
                // cap_transfered_out,  :r#"remote capacitor transmitted to"#,
                // cap_neutralized_out, :r#"energy neutralized"#,
                // nos_recieved,        :r#"energy drained from"#,
                // cap_transfered_in,   :r#"remote capacitor transmitted by"#,
                // cap_neutralized_in,  :r#"energy neutralized"#,
                // nos_taken,           :r#"energy drained to"#,
                // IN damage
                // :5:"[ 2022.06.30 19:47:05 ] (combat)█39█from█Anchoring Damavik█Glances Off"
                // IN energy neutralized
                // :5:"[ 2022.06.30 19:33:15 ] (combat)█20 GJ█energy neutralized█Faded Hypnosian Warden█Faded Hypnosian Warden"

                // OUT damage
                // :6:"[ 2022.06.30 19:46:39 ] (combat)█1814█to█Raznaborg Blinding Leshak█Veles Supratidal Entropic Disintegrator█Smashes"
                // IN damage(pvp)
                // :6:"[ 2022.06.30 21:03:29 ] (combat)█82█from█Username[CTAG](Vargur)█Imperial Navy Large EMP Smartbomb█Hits"
                // IN module(pve - pvp ???)
                // :6:"[ 2022.01.19 18:31:06 ] (combat)█Warp scramble attempt█from█Raznaborg Anchoring Damavik█to█State Navy Rook"

                // IN module use
                // :8:"[ 2022.06.30 19:47:13 ] (combat)█351█remote capacitor transmitted by█Leshak█[REKTD]█[PSST.]█[Dalliloule Nardieu]█Large Remote Capacitor Transmitter II"
                // :8:"[ 2022.06.30 19:47:21 ] (combat)█640█remote armor repaired by█Leshak█[REKTD]█[PSST.]█[Dalliloule Nardieu]█Large Remote Armor Repairer II"

                // OUT module use
                // :8:"[ 2022.06.30 19:47:21 ] (combat)█351█remote capacitor transmitted to█Leshak█[REKTD]█[SPSFC]█[FWC]█Large Remote Capacitor Transmitter II"
                // :8:"[ 2022.06.30 19:47:14 ] (combat)█257█remote armor repaired to█Leshak█[REKTD]█[PSST.]█[Dalliloule Nardieu]█Large Remote Armor Repairer II"

                // ????
                // :8:"[ 2022.06.30 20:34:35 ] (combat)█Warp scramble attempt█from█Hospodar Anchoring Damavik█to█Paladin█[WTFNA]█[VLAD86]"
            }
            "None) " => {}
            "questi" => {}
            "notify" => {}
            _ => {}
        }

        // println!("####################################################");
        // dbg!(&arr);
        // arr
    }
}

#[derive(Debug)]
pub struct LogReader {
    file_path:     PathBuf,
    bufreader:     Option<std::io::BufReader<File>>,
    line_buffer:   VecDeque<String>,
    size:          u64,
    offset:        u64,
    header:        Vec<String>,
    header_offset: u64,
}

impl LogReader {
    pub fn new(file_path: PathBuf) -> Result<Self, &'static str> {
        let meta: Metadata;
        match file_path.metadata() {
            Err(e) => return Err("cant get file metadata"),
            Ok(v) => {
                meta = v;
            }
        }

        let mut this = Self {
            bufreader:     None,
            line_buffer:   VecDeque::with_capacity(3),
            file_path:     file_path,
            size:          meta.len(),
            offset:        0,
            header:        Vec::with_capacity(6),
            header_offset: 0,
        };

        // this.check_filename();
        if let Err(e) = this.check_header() {
            return Err(e);
        }

        Ok(this)
    }

    fn get_modtime(&mut self) -> u64 {
        let meta = self.file_path.metadata().unwrap();

        meta.modified()
            .unwrap()
            .duration_since(time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }

    pub fn is_changed(&mut self) -> bool {
        let meta = self.file_path.metadata().unwrap();

        return meta.len() > self.offset;
    }

    pub fn get_details(&mut self) -> Result<(String, u64), &'static str> {
        // выдираем из названия файла UserId
        let fname = self
            .file_path
            .file_name()
            .unwrap_or_else(|| {
                // panic!("metadata - filename cant get({:?})", error);
                panic!("metadata - filename cant get");
            })
            .to_str()
            .map(|s| s.to_string())
            .unwrap_or_else(|| {
                // panic!("metadata - filename cant convert to String({:?})", error);
                panic!("metadata - filename cant convert to String");
            });

        // USER_NAME - получаем из header
        let user_name: String;
        let re = regex::Regex::new(r"^\s\s.*:\s+(.*?)\r?\n").unwrap();
        if let Some(cap) = re.captures(&self.header[2]) {
            user_name = cap.get(1).unwrap().as_str().to_owned(); // &str -> String
        } else {
            return Err("Header parse error: line #2");
        }

        // USER_ID - получаем из имени файла
        // 20220620_193132_2117846337
        let re_user_id = regex::Regex::new(r"^\d{8}_\d{6}_(\d+).txt$").unwrap();
        if let Some(cap) = re_user_id.captures(&fname) {
            // self.user_id = cap.get(1).unwrap().as_str().parse::<u32>().unwrap();
            // let name = "BLANK Username".to_string();
            let id = cap.get(1).unwrap().as_str().parse::<u64>().unwrap();
            return Ok((user_name, id));
        } else {
            // self.error = "bad file name format(timestamp + user_id)".to_string();
            // self.ignored = true;
            return Err("bad file name format(timestamp + user_id)");
        }
    }

    /*
       проверяем на конец файла
           это конец файла + мультилайн
               prev_line = prev_line + &line;
           это конец файла и это самостоятельная строка, а значит
               let to_ret = tmp_line
               tmp_line = &line;
               return
               потом, в tmp_line записать line, а при след. обращении сразу вернуть line
           return prev_line;

    if self.bufreader == None()
        обновитьб всё и запихнуть рейдера
    if offset==size
        line_buffer.len()==1 иначе паника. обнуляем рейдер.
        return self.line_buffer.pop()

    loop читаем строку
        конец файла ?
            прочитано 0 байт ?
                да:
                    line_buffer.len()==1 иначе паника. обнуляем рейдер.
                    return self.line_buffer.pop()
                нет:
                    прочитанная строка самостоятельная ?
                        да:
                            self.line_buffer.push(line)
                            return self.line_buffer.pop()
                        нет:
                            line_buffer.len()==1 иначе паника. обнуляем рейдер.
                            return self.line_buffer.pop() + &line
        прочитанная строка самостоятельная ?
            да:
                self.line_buffer.push(line)
                return self.line_buffer.pop()
            нет:
                self.line_buffer[0] += &line
    */

    pub fn read_multiline(&mut self) -> Option<String> {
        // если self.bufreader отсутствует, то создаём рейдера, устанавливаем длинну файла, до которой мы будем читать
        match self.bufreader {
            None => {
                let meta = self.file_path.metadata().unwrap();

                if self.offset == meta.len() {
                    return None;
                }

                self.size = meta.len();

                let file = File::open(&self.file_path).unwrap();
                let mut bufreader = BufReader::new(file);

                bufreader.seek(SeekFrom::Start(self.offset)).unwrap();

                self.bufreader = Some(bufreader);
            }
            _ => (),
        }
        // if offset==size
        //      line_buffer.len()==1 иначе паника. обнуляем рейдер.
        //      return self.line_buffer.pop()
        if self.offset == self.size {
            if self.line_buffer.len() > 1 {
                panic!("LogReader: end of file and line_buffer contain more that 1 line");
            }
            if self.line_buffer.len() == 0 {
                return None;
            }
            self.bufreader = None;
            return self.line_buffer.pop_front();
        }
        // ---------------------------------------------------
        let mut line = String::new();
        let mut reader = self.bufreader.as_mut().unwrap();
        while let Ok(num) = reader.read_line(&mut line) {
            self.offset = self.offset + (num as u64);

            // ----------------------------------------------------
            if self.offset == self.size {
                if num == 0 {
                    if self.line_buffer.len() > 1 {
                        panic!("LogReader: end of file and line_buffer contain more that 1 line");
                    }
                    self.bufreader = None;
                    return self.line_buffer.pop_front();
                } else {
                    if let Some(pos) = line.find("[ ") {
                        self.line_buffer.push_back(line);
                        return self.line_buffer.pop_front();
                    } else {
                        if self.line_buffer.len() > 1 {
                            panic!(
                                "LogReader: end of file and line_buffer contain more that 1 line"
                            );
                        }
                        return Some(self.line_buffer.pop_front().unwrap() + &line);
                    }
                }
            }

            // ----------------------------------------------------
            if let Some(pos) = line.find("[ ") {
                if pos == 0 {
                    if self.line_buffer.len() == 0 {
                        self.line_buffer.push_back(line.clone());
                    } else {
                        self.line_buffer.push_back(line);
                        return self.line_buffer.pop_front();
                    }
                } else {
                    // тут скорее не мультилайн, а некорректная строка. вообще эта часть условия не должна срабатывать никогда.
                    panic!("LogReader: not correct line in log body\n\tline: {line:?}");
                }
            } else {
                self.line_buffer[0] = self.line_buffer[0].clone() + &line;
            }
            line.clear();
        }
        return None;
    }

    pub fn read_allines(&mut self) -> Vec<String> {
        let mut arr: Vec<String> = Vec::new();

        loop {
            match self.read_multiline() {
                Some(v) => {
                    arr.push(v);
                }
                None => break,
            }
        }

        arr
    }

    pub fn get_language(&mut self) -> Result<&'static str, &'static str> {
        let mut language: &str;
        for (k, v) in &*LOGLANGUAGE {
            if self.header[2].contains(v.character) {
                return Ok(k);
            }
        }
        return Err("problem with detecting language");
    }

    pub fn check_header(&mut self) -> Result<(), &'static str> {
        // let lang: Vec<LogLanguage> = generate_language();

        let file = File::open(&self.file_path).unwrap();
        let mut reader = BufReader::new(file);

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
            // println!(
            //     "i:{:3}|of: {:5} +{:4}->{:5}| s: {}",
            //     &i,
            //     &offset,
            //     &num,
            //     (offset + num),
            //     &line.trim_end()
            // );

            offset = offset + num;
            if num == 0 {
                // EOF check
                line.clear();
                break;
            }

            match i {
                0 => {
                    if line.contains("------------") == false {
                        return Err("Header parse error: line #0");
                    }
                }
                1 => {
                    let re = regex::Regex::new(r"^\s\s.*\r?\n").unwrap();
                    if !re.is_match(&line) {
                        return Err("Header parse error: line #1");
                    }
                }
                2 => {
                    // =~/\s\s.*\:.*\n/
                    let re = regex::Regex::new(r"^\s\s.*:.*\r?\n").unwrap();
                    if !re.is_match(&line) {
                        return Err("Header parse error: line #2");
                    }
                }
                3 => {
                    // =~/\s\s.*\:\d{4}\.\d{2}\.\d{4}\s\d{2}\:\d{2}\:\d{2}\n/
                    let re =
                        regex::Regex::new(r"\s\s.*:\s+\d{4}.\d{2}.\d{2}\s\d{2}:\d{2}:\d{2}\r?\n")
                            .unwrap();
                    if !re.is_match(&line) {
                        // eprintln!("Header parse:\n\tline: {line:?}");
                        return Err("Header parse error: line #3");
                    }
                }
                4 => {
                    if !line.contains("------------") {
                        return Err("Header parse error: line #4");
                    }
                    self.offset = offset as u64;
                    self.header_offset = self.offset;
                    // self.offset = reader.stream_position().unwrap();
                    // break;
                }
                _ => {
                    break;
                }
            };
            self.header.push(line.clone());

            line.clear();
        }

        if self.header.len() != 5 {
            return Err("Header parse error: header contains less that 5 lines");
        }

        Ok(())
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

fn cur_time_ms() -> u128 {
    SystemTime::now()
        .duration_since(time::UNIX_EPOCH)
        .unwrap()
        .as_millis()
}
