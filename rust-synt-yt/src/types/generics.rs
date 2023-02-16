use std::fmt::Display;

fn generics() {
  let string = String::from("Hi");
  let num = 777;

  let nums = [10, 1, 2, 3, 4, 3, 5, 6, 7, 8, 7, 7, 9, 10, 10];
  let chars = ['x', 'b', 'a', 'b', 'c', 'x', 'b', 'r', 'q'];

  println!("{:?}", find_dublicate(&nums));
  println!("{:?}", find_dublicate(&chars));
}

// функция принимает данные любого типа за счет <T>
fn print_value<T: Display>(value: T) {
  println!("{}", value);
}

// функция поиска повторяющтхся значений
fn find_dublicate<T>(list: &[T]) -> Vec<T>
where T: PartialEq + Copy
{
  let mut dublicates: Vec<T> = Vec::new();

  for i in 0..(list.len()) {
    for j in (i + 1)..(list.len()) {
      if list[i] == list[j] {
        if !dublicates.contains(&list[i]) {
          dublicates.push(list[i]);
        }
      }
    }
  }
  dublicates
}