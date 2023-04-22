use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn main() {
    let text = String::from("Фамилия: Иванов Имя: Иван Отчество: Иванович");
    let text_split: Vec<&str> = text.split(" ").collect();

    println!("{}", text);

    println!("{:?}", text_split);

    for word in text_split {
        println!("{}", word);
    }

    let file = File::open("file.txt").unwrap();
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        // Show the line and its number.
        println!("{}. {}", index + 1, line);
    }
}

//иванов;иван;иванович;2222;;