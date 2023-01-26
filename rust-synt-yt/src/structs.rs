#[derive(Debug)]
// #[derive(Debug)] - для println структур (без него будет ошибка)
struct Home {
  city : String,
  country : String
}

struct Square(i32, i32); // кортежная структура

pub fn structs_func() {

  let mut family = Home {
    city : "Moscow".to_string(),
    country : "Rus".to_string()
  };
  println!("{:?}", family); // печатает структуру в одну строчку
  println!("{:#?}", family); // печатает структуру полноценно в несколько строчек

  let mut holidays = relocate("Dubai".to_string(), "OAE".to_string());

  // .to_string() тоже самое что и String::from()
  family.city = "Dubai".to_string();
  family.country = "OAE".to_string();
  holidays.country = "Rus".to_string();

  let sq = Square(10, 20);
  area(sq);
}

fn relocate(city : String, country : String) -> Home {
  Home {
    city : city,
    country : country
  }
}

fn area(side : Square) -> i32 {
  side.0 * side.1
}

