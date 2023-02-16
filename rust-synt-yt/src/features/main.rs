mod structs;
mod arrays;
mod functions;
mod iofromusers;
mod vectors;
mod errors;

mod mainmain;
// неизменяемые, всегда нужно прописывать тип, можно объявлять за пределами главной функции
const _CHILDREN: i32 = 6; 
fn main() {
  // structs::structs_func();
  // arrays::arrays();
  // functions::functions();
  // iofromusers::inputfromuser();
  // vectors::vectors();
  // errors::errors();
  mainmain::main();
  // Переменные
// integer -1,4,-11,7...   i8 i16 i32 i64 i128 isize(= i64 в моем случае)
// float 5.73, 3.9...      f8 f16 f32 f64 f128 fsize
// uint 1,2,3,4...         u8 u16 u32 u64 u128 usize
    let age: i8 = 26;
    let name: &str = "Mike"; // небезопасный способ создания строки
    let mut surname: String = String::from("Bolshakov"); // безопасный 
    surname.push('s'); // добавление символа к строке
    surname.push_str(" Mikhailovich"); // добавление строки к строке
    let fio = format!("{} {}", name, surname); // конкатенация (объединение) строк
    println!("{}", &fio[..3]); // вытаскиваем по индексу, в консоли будет Mik
    for razbivka in name.chars() {
      println!("{}", razbivka) // разбивка по буквам. Есть еще .bytes() - в консоле будут не буквы от байты для каждой буквы
    }
    let male: char = 'M'; // char - только один символ
    let developer: bool = true;
    println!("My name is {} {} ({}), age {}. Occupation dev: {}", name, surname, male, age, developer);

    // Условные операторы    
    let mut number: i32 = if developer {
      10
    } else {
      0
    };

    // Циклы (loop (бесконечный цикл), while, for)
    loop {
      if number == 13 {
        break;
      }
      number += 1;
    }

    // 6 не включается в перебор. Чтобы включить 0..=6
    for i in 0..6 {
      if i % 2 == 0 { // 18 % 4 = 2
        println!("{}", i)
      }
    }

    // c match мы сравниваем значение age с цифрами ниже
    match age {
      10 => println!("Age is 10"),
      26 => {
        println!("Age is 26");
        println!("its me");
      },
      12..=17 => {
        println!("возраст в диапазоне от 12 до 17 включая 12 и 17")
      }
      _ => {
        println!("это дефолтное значение, когда предыдущие не сработали")
      }
    }
    
    // присвоение значения переменной
    let _random_numb: i32 = match age {
        21 => 35,
        26 => 11,
        3..=9 => 44,
        _ => 0
    };

    // присвоение нового значение через new()
    let mut art: String = String::new();
    match developer {
        true => {
          art = String::from("разраб")
        },
        false => {
          art = String::from("менеджер")
        }
    }
    println!("{}", art);

    // без clone() data перейдет в Y и к data уже не будет доступа 
    // автоматически копируются примитивные типы: числа, bool и char (+ кортежи с примит типами)
    let data = "some data".to_string();
    let _y = data.clone(); // можно через clone()
    let _y = &data; // можно через ссылку &
    println!("{}", data)
}
 
