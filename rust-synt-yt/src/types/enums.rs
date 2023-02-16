enum Lavka {
  Order,
  Payed,
  Delivered,
  Canceled
}

// расширенные возможности
enum Cmd {
  Quit,
  WriteText(String),
  ChangeColor()
}

struct Color {
  red: i32,
  green: i32,
  blue: i32
}

fn enums() {
  let breakfast = Lavka::Order;
  let result = calc(breakfast, 7, 8);

  let cmdName = Cmd::WriteText("Just text".to_string());
  match cmdName {
    Cmd::WriteText(message) => println!("message {}", message),
    Cmd::ChangeColor(color) => println!("{} {} {}", color.red, color.green, color.blue),
    Cmd::Quit => println!("Quit")
  }
}

fn calc(breakfast: Lavka, x: i32, y: i32) -> i32 {
  match breakfast {
    Lavka::Order => x * y,
    Lavka::Payed => x + y,
    _ => 0
  }
}