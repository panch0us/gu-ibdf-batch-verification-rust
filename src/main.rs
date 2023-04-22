
fn main() {
    let text = String::from("Фамилия: Иванов Имя: Иван Отчество: Иванович");
    let text_split: Vec<&str> = text.split(" ").collect();

    println!("{}", text);

    println!("{:?}", text_split);

    for word in text_split {
        println!("{}", word);
    }
}
