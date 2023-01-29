use std::collections::HashMap;
use std::collections::HashSet;
// для работы с хэшмэпами нужно импортировать их из стандартной библиотеки
use std::fs::File; // для работы с файлами
use std::ops::Add; // импорт операции сложения
use std::ops::Sub;

pub fn main() {
    // Владение
    // У каждого значения есть владелец
    // У значения может быть только один владелец в один момент времени
    // Когда владелец покидает область видимости, значение удаляется и ресурсы памяти освобождаются

    // примитивные типы данных хранятся в стеке, остальные в куче
    // без clone() data перейдет в _y и к data уже не будет доступа
    // автоматически копируются примитивные типы: числа, bool и char (+ кортежи с примит типами)
    let data: String = "some data".to_string();
    let _y: String = data.clone(); // можно через clone()
    let _y: &String = &data; // можно через ссылку &

    // array (массивы) (элементы только одного типа)
    let _some_array: [u8; 4] = [1, 3, 5, 7];

    // slice (срезы) (от массивов)
    println!("{:?}", &_some_array[0..2]); // [1, 3]
    println!("{:?}", &_some_array[2..4]); // [5, 7]
    println!("{:?}", &_some_array[..]); // [1, 3, 5, 7]

    // vec (векторы) (динамический массив)
    let mut dinamic_array: Vec<i32> = Vec::new();
    dinamic_array.push(33);
    dinamic_array.push(54);

    // HashMap (структура с ключ:значение, где для ключа используется алгоритм хэширования)
    let mut some_map: HashMap<&str, i32> = HashMap::new();
    some_map.insert("interesting", 27);
    some_map.insert("task", 72);
    println!("{:?}", some_map); // {"interesting": 27, "task": 72}

    // HashSet - хранилище ключей (как хэшмэп, только без значений)
    let mut some_set: HashSet<&str> = HashSet::new();
    some_set.insert("only");
    some_set.insert("keys");
    println!("{:?}", some_set); // {"keys", "only"}

    // tuple (кортежи) (как массив, но могут быть разные элементы в кортеже)
    let _some_data: (f64, &str, bool) = (3.14, "Mike", true);
    let _pi: f64 = _some_data.0;

    // struct (структуры)
    struct User {
        name: String,
        age: usize,
        active: bool,
    }

    let _student: User = User {
        name: "Mike".to_string(),
        age: 26,
        active: true,
    };

    let _student2: User = User {
        age: 25,
        .._student
    };

    // tuple struct
    struct Color(i32, i32, bool);

    let _first_color: Color = Color(31, 53, true);
    let _second_color: i32 = _first_color.1;

    // Методы структур
    // impl - реализация структуры
    // self - ссылка на себя же (аналог this)
    impl User {
        fn _how_old_user(&self) -> usize {
            self.age
        }
        fn _get_older(&mut self) {
            self.age += 1;
        }
    }

    // enum
    enum Calc {
        Add(i32, i32),
        Minus,
        Multiply,
    }
    let _math: Calc = Calc::Add(10, 5);

    // тип Option
    let one: Option<i32> = Some(22);
    match one {
        Some(two) => println!("{}", two),
        None => println!("No result"),
    }
    // тип Result
    // let my_file = File::open("filename.txt");
    // let _my_file = match my_file {
    //     Ok(our_file) => our_file,
    //     Err(error) => panic!("Error: {}", error),
    // };
    // // unwrap - если файл можно открыть без ошибок, то переменная _smf будет содержать результат
    // let _smf = File::open("secfile.txt").unwrap();
    // // expect - то же что и unwrap(), только при ошибке будет panic с сообщением
    // let _smf = File::open("secfile.txt").expect("Не смогли открыть");

    // generics (дженерики) обобщенные типы данных
    pub struct _Shape<T, U> {
        pub x: T,
        pub y: T,
        pub z: U,
    }

    // применяем несколько трейтов к дженерикам через +
    // T: - значит тип Т реализует:
    // Copy - встроенный в язык раст трейт, реализующий заимствование вместо &
    // Add - встроенный трейт - сложение
    // <Output = T> - возвращаем тип Т
    impl<T: Copy + Add<Output = T>, U: Copy> _Shape<T, U> {
        pub fn _get_shape(&self) -> T {
            self.x + self.y
        }
        pub fn _get_z(&self) -> U {
            self.z
        }
    }
    /* аналогичная запись с where
    impl<T, U> _Shape<T, U>
    where
        T: Copy + Add<Output = T>,
        U: Copy,
    {
        pub fn _get_shape(&self) -> T { self.x + self.y }
        pub fn _get_z(&self) -> U { self.z }
    }*/

    pub struct Shape<T> {
        pub x: T,
        pub y: T,
    }

    // применение трейтов в функциях
    fn summ<T: Copy + Add<Output = T>>(figure: Shape<T>) -> T {
        figure.x + figure.y
    }
    // // пример вызова
    let fig: Shape<i32> = Shape { x: 10, y: -20 };
    let res: i32 = summ::<i32>(fig); // -10

    // traits (трейты) - аналог интерфейсов
    // это сам трейт
    pub trait ShapeCalculation<T> {
        fn sum(&self) -> T;
    }

    // это реализация трейта
    impl<T: Copy + Add<Output = T>> ShapeCalculation<T> for Shape<T> {
        fn sum(&self) -> T {
            self.x + self.y
        }
    }

    // это ограничения трейта и взаимодействие с ним (обращение к реализации)
    impl<T: ShapeCalculation<T>> Shape<T>
    where
        T: Copy + Add<Output = T> + Sub<Output = T>,
    {
        pub fn _calcul(&self, z: T) -> T {
            z.sum() - self.x
        }
    }

    // анонимные функции (closures/лямбда функции) (особенность - доступ к переменным окружения)
    let _closure1 = || 10; // функция без аргумента, возвращающая только число 10
    let clo2 = |x: i32| 10 * x; // аргумент x, возвращаем х * 10
    let _clo3 = |x: i32| -> f32 { (10 * x) as f32 / 2.0 }; // в {} тело функции

    // функции высшего порядка (аргумент -  другая функция)
    // функциональные типы, описанные через трейты:
    // FnOnce - передача владения (по умолчанию)
    // Fn - заимствование без возможности изменения
    // FnMut - заимствование с возможностью изменения

    // функция как аргумент другой функции:
    fn interest_func<F>(clo2: F) -> i32
    where
        F: FnOnce(i32) -> i32,
    {
        5 * clo2(30)
    }
    println!("Result of function: {:?}", interest_func(clo2)); // 1500

    let x: Vec<i32> = vec![10, 20, 30];
    let z1 = |y: i32| x.iter(); // потом можем обращаться к х
    let z2 = move |y: i32| { x.iter(); }; // потом НЕ можем обращаться к х

    // итераторы(перебирают) и генераторы(производят операции)
    let yld: i32 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9]
        .iter() // проходимся по элементам
        .filter(|x| *x % 2 != 0) // фильтруем элементы, оставляем нечетные
        .map(|i| i * i) // возводим в 2 степень
        .filter(|x| *x < 30) // отбираем результаты меньше 30
        .sum(); // складываем результаты
    println!("Result is {:?}", yld); // Result is 35
}
