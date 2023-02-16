struct Rectangle {
  weight: i32,
  height: i32
}

fn functions() {
    sum(4, 6);
    let square: i8 = square1(4);
    println!("{}", square);

    fn sum(x: i8, y: i8) {
        println!("sum {}", x + y);
    }
    fn square1(n: i8) -> i8 {
        n * n
    }
    fn square2(n: i8) -> i8 {
        let result: i8 = n * n;
        return result;
    }

    // анонимные функции
    fn anon_func() {
      println!("Анон");
    };
    let area = |side: Rectangle| -> i32 {
      side.weight * side.height
    };
    let sum = |a: i32, b: i32| a + b;
    sum(4,5);

    // вызов функций в функциях
    fn handler(anon_func: fn(), area: fn()) {
      let res: bool = true;
      if res == true {
        anon_func();
      } else {
        area();
      }
    }
}
