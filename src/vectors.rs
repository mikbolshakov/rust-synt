// вектор - аналог динамического массива

pub fn vectors() {
  let mut list = vec![1, 2, 3];
  list.push(4);
  list.remove(2); // удаление по индексу
  let _first_elem = &list[0];
  let _lot_of = &list[0..2];
  let _second_elem = list.get(1); // этот вариант чтения элемента лучше 

  println!("{:?}", list);


}