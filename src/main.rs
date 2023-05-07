use std::io;
use std::fs;

fn main() {
    println!("Введите имя файла: ");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Ошибка в чтении строки ввода");

    let content = fs::read_to_string(input.trim())
        .expect("Должен был открыться файл");

    println!("{content}");


}