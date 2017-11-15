extern crate bincode;
extern crate libflate;

use bincode::{serialize, Infinite};

use std::io::{BufRead, BufReader, Write};
use std::io;
use std::fs::File;
use std::collections::HashMap;
use libflate::gzip::Encoder;

fn main() {
    let file = File::open("alastalon_salissa.txt").unwrap();
    let words = count_words(file).unwrap();
    write_output(words).unwrap();
}

fn write_output(map: HashMap<String, i32>) -> io::Result<()> {
    let mut out = File::create("./src/answer.gz")?;
    let bytes = serialize(&map, Infinite).unwrap();

    let mut encoder = Encoder::new(Vec::new())?;
    encoder.write_all(&bytes)?;
    let result = encoder.finish().into_result()?;

    out.write_all(&result)
}

fn count_words(file: File) -> io::Result<HashMap<String, i32>> {
    let reader = BufReader::new(file);
    let mut words: HashMap<String, i32> = std::collections::HashMap::new();

    for l in reader.lines(){
        let line = l?;
        for word in line.split(" ") {
            let str = word.to_owned();
            if str == "" {
                continue;
            }
            let w = *words.get(&str).unwrap_or(&0);
            words.insert(str, w + 1);
        }
    }

    Ok(words)
}