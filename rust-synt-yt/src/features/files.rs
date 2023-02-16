use std::fs::File;
use std::fs::Read;
// fs - file system

fn files() {
    // создает файл и записывает данные, если его нет. Но если есть файл, стирает все данные в нем
    File::create("text.txt").expect("Ошибка создания файла");

    File::open("text.txt").expect("Ошибка открытия файла"); // открывает и читает данные из файла

    let mut file = File::create("index.html").expect("Ошибка создания файла");
    file.write_all("just text".as_bytes()).expect("Ошибка записи данных"); // запись данных в файл (передаем байты)

    let mut file_data = String::new();
    file.read_to_string(&mut file_data).expect("Ошибка чтения файла"); // чтение данных из файла

    // открытие файла в режиме чтения и записи
    let mut new_file_data = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("data.txt")
        .expect("Err");
}
