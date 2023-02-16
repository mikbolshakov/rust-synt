fn arrays() {
  let id = [1, 2, 3, 4, 5];
  let lotsofone = [1; 5]; // элемент 1 повторяется 5 раз, то есть массивы выглядит так: [1, 1, 1, 1, 1]
  let mut lang: [&str; 3]; // в массиве lang будет 3 элемента типа str
  lang = ["sol", "rust", "js"];
  println!("frist lang was {}", lang[2]);
  println!("{:?}", lotsofone); // для вывода всего массива {:?}

  lang[2] = "html";

  // перебор массива: выдаст все три языка друг за другом с новой строки
  for languages in lang.iter() {
    println!("all langs: {}", languages)
  }

  // цикл от длины массива
  // for i in 0..=id.len() {
  //   println!("{}", id[i])
  // }
}