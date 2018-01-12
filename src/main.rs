extern crate bincode;
extern crate clap;
extern crate libflate;

use clap::{App, Arg};
use bincode::deserialize;

use std::io::{BufRead, BufReader, Read};
use std::io;
use std::fs::File;
use std::collections::HashMap;
use libflate::gzip::{Decoder};


fn main() {
    let app = App::new("Optimized Alastalon Salissa validator")
        .version("1.0")
        .about("Validates Wundernut #8 answer")
        .arg(Arg::with_name("answer_file")
            .required(true)
            .help("Optimized Alastalon Salissa book file")
            .value_name("ANSWER FILE")
            .index(1));
    let arg_matches = app.get_matches();

    let filename = arg_matches.value_of("answer_file").unwrap();

    let failed = match run(filename) {
        Err(err) => {
            println!("ERROR: {}", err);
            true
        },
        Ok(()) => {
            false
        },
    };

    std::process::exit(if failed { 1 } else { 0 });
}

#[cfg(any(unix, macos))]
fn check(ok: bool) -> &'static str {
    if ok { "[✓]" } else { "[✗]" }
}

#[cfg(windows)]
fn check(ok: bool) -> &'static str {
    if ok { "[OK]" } else { "[FAIL]" }
}

fn run(filename: &str) -> io::Result<()> {
    let file = File::open(filename)?;

    match validate(file) {
        Err(err) => {
            println!("ERROR: {}", err);
        }
        Ok(Validation { line_len_ok, words, lines }) => {
            println!("All words found:    {}", check(has_all_words(words)));
            println!("Lines max 80 chars: {}", check(line_len_ok));
            println!("{} lines", lines);
        }
    };
    Ok(())
}

fn has_all_words(words: HashMap<String, i32>) -> bool {
    let answer_bytes: &[u8] = include_bytes!("answer.gz");
    let mut decoder = Decoder::new(answer_bytes).unwrap();
    let mut buf = Vec::new();
    decoder.read_to_end(&mut buf).unwrap();
    let answer: HashMap<String, i32> = deserialize(&buf).expect("Could not deserialize answer");

    answer == words
}

struct Validation {
    words: HashMap<String, i32>,
    lines: i32,
    line_len_ok: bool,
}

fn validate(file: File) -> io::Result<Validation> {
    let reader = BufReader::new(file);
    let mut words: HashMap<String, i32> = std::collections::HashMap::new();

    let mut line_len_ok = true;
    let mut lines = 0;
    for l in reader.lines(){
        let line = l?;
        lines += 1;
        if line.chars().count() > 80 {
            line_len_ok = false;
        }
        for word in line.split(" ") {
            if word == "" {
                continue;
            }
            let str = word.to_owned();
            let w = *words.get(&str).unwrap_or(&0);
            words.insert(str, w + 1);
        }
    }

    Ok(Validation {
        words,
        lines,
        line_len_ok,
    })
}
