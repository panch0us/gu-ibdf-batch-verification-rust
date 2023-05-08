use std::fs;

use std::io;
use std::io::{BufRead, BufReader, Error, Write};

use encoding_rs::WINDOWS_1251;
use encoding_rs_io;
use encoding_rs_io::DecodeReaderBytesBuilder;


fn main() {
    println!("Введите имя файла: ");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Ошибка в чтении строки ввода.");

    let sourse_file = fs::File::open(input.trim()).unwrap();
    let reader = BufReader::new(
        DecodeReaderBytesBuilder::new()
            .encoding(Some(WINDOWS_1251))
            .build(sourse_file));

    //Строка для заполнения после обработки (Ф;И;О;YYYY;;)
    let mut result_str = String::new();

    for line in reader.lines() {
        let result = match line {
            Ok(ok) => ok,
            Err(error) => format!("Ошибка {error}"),
        };

        if result.contains("Фамилия: ") {
            result_str.push_str(&result[16..].trim());
            result_str.push(';');
        }

        if result.contains("Имя: ") {
            result_str.push_str(&result[8..].trim());
            result_str.push(';');
        }

        if result.contains("Отчество: ") {
            result_str.push_str(&result[18..].trim());
            result_str.push(';');
        }

        if result.contains("Дата рождения: ") {
            result_str.push_str(&result[33..].trim());
            result_str.push_str(";;");
            result_str.push('\n');
        }
    }

    write_in_result_file(result_str);
}

fn write_in_result_file(s: String) -> Result<(), Error>{
    let mut result_file = fs::File::create("result.txt")?;
    write!(result_file, "{}", s)?;
    Ok(())
}