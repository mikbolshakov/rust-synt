// Если в трейте объявляем что-то помимо названия функции - это будет дефолтным значением
trait Employ {
  fn do_work(&self) {
    println!("по дефолту")
  }

  // метод изменяющий состояние
  fn take_cash(&mut self, salary: i32);

  // ассоциированная функция
  fn compare_cash(cash1: i32, cash2: i32) -> bool {
    cash1 > cash2;
  }
}

impl Employ for Developer {
  fn do_work(&self) {
    println!("did it")
  }

  fn take_cash(&mut self, salary: i32) {
    self.cash += salary;
  }
}

struct Developer {
  name: String,
  age: i32,
  lang: String,
  cash: i32
}

// вызываем метод у экземпляра
fn traits() {
  let mut topDev = Developer::new(
    26,
    "Mike".to_string(),
    "JS".to_string(),
    1000000
  );

  let result = Developer::compare_cash(topDev.cash, 1);

  topDev.do_work();
  topDev.take_cash(1000);
}
