use std::collections::HashMap;
// необходимый импорт для работы с хэшмэпами

// хэшмэпы - в них хранятся данные ключ: значение
fn hashmaps() {
  let mut map = HashMap::new();

  // добавляем данне в хэшмэп
  map.insert("Denis".to_string(), 10);
  map.insert("Mike".to_string(), 13);
  println!("{:?}", map);
  println!("{}", map["Denis"]);


}