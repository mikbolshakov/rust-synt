use std::io;

pub fn inputfromuser() {
    // ввод пользовательских данных
    let mut name = String::new();

    println!("Введите свое имя:");
    match io::stdin().read_line(&mut name) {
        Ok(_) => {
            println!("Hi, {}", name);
        }
        Err(e) => {
            println!("ошибка")
        }
    }

    // решаем кв уравнение Ax^2 + Bx + C = 0
    let mut a_str: String = String::new();
    let mut b_str: String = String::new();
    let mut c_str: String = String::new();

    println!("введите а");
    match io::stdin().read_line(&mut a_str) {
        Ok(_) => {},
        Err(e) => println!("ошибка ввода")
    }

    println!("введите b");
    match io::stdin().read_line(&mut b_str) {
        Ok(_) => {},
        Err(e) => println!("ошибка ввода")
    }

    println!("введите c");
    match io::stdin().read_line(&mut c_str) {
        Ok(_) => {},
        Err(e) => println!("ошибка ввода")
    }

    // trim убирает ненужные пробелы, parse - отображает, unwrap - распаковывает
    let a: f64 = a_str.trim().parse().unwrap();
    let b: f64 = b_str.trim().parse().unwrap();
    let c: f64 = c_str.trim().parse().unwrap();

    // считаем дискриминант
    let d: f64 = (b*b) - 4.0 * a * c;

    // решаем уравнение
    if d > 0.0 {
      let x1 = ((-b) + d.sqrt()) / (2.0 * a);
      let x2 = ((-b) - d.sqrt()) / (2.0 * a);

      // \n - перенос текста на след строку
      println!("Решено\nЕсть два корня {} {}", x1, x2)
    }
    if d == 0.0 {
      let x = (-b) / (2.0 * a);
      println!("Решено\nЕсть 1 корeнь {}", x)
    }
    if d < 0.0 {
      println!("Корней не существует\nD < 0\nD = {}", d)
    }
}
