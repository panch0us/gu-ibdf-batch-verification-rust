use std::fs;

use std::io;
use std::io::{BufRead, BufReader, Read};

use encoding_rs::WINDOWS_1251;
use encoding_rs_io;
use encoding_rs_io::DecodeReaderBytesBuilder;


fn main() {
    println!("Введите имя файла: ");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Ошибка в чтении строки ввода");

    let file = fs::File::open(input.trim()).unwrap();
    let mut reader = BufReader::new(
        DecodeReaderBytesBuilder::new()
            .encoding(Some(WINDOWS_1251))
            .build(file));

    let mut reader_str = String::new();

    for line in reader.lines() {
        let result = match line {
            Ok(line) => line,
            Err(error) => "Ошибка".to_string(),
        };
        if result.contains("Фамилия: ") {
            println!("{}", result);
        }

    }

    //reader.read_to_string(&mut reader_str).expect("cannot read string");

    //for line in reader_str.split("/n") {
    //    if line.contains("2222") {
    //        println!("{line}");
    //    }
    //}

    let result_file = fs::File::create("result.txt");
}