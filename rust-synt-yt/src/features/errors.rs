use std::fs::File; // стандартная библиотека для работы с файлами
use std::io::ErrorKind;

fn errors() {
    // пытаемся открыть файл
    let file = File::open("index.html");

    // отлавливаем ошибку:
    let file = match file {
      Ok(file) => file,
      Err(e) => match e.kind() { // kind - вид ошибки: через перечисление "ErrorKind"
        ErrorKind::NotFound => match File::create("index.html") { // если нет нашего файла (NotFound), то создаем новый файл
          Ok(file) => file,
          Err(e) => panic!("Ошибка создания файла {:?}", e),
        },
        another => panic!("ОШИБКА: {:?}", another)
      },
    };

    // unwrap - аналог Ok и Err выше
    let _second_file = File::open("index.html").unwrap();

    // то же самое что и unwrap, только пишем свой собственный текст паники
    let _third_file = File::open("index.html").expect("My own panic");
    // еще в конце можно ставить ?; - тоже что и unwrap только при ошибке откатывает функцию
    // но его можно использовать только если функция возвращает Result
}
