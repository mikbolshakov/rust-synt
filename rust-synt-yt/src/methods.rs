struct Developer {
    name: String,
    age: i32,
    lang: String,
}

// объявляем метод
// self - ссылка на нашу структуру
impl Developer {
    fn get_age(&self) -> i32 {
        self.age;
    }
    fn change_lang(&mut self, new_lang: String) {
        self.lang = new_lang;
    }

    // ассоциированная функция для всей структуры
    fn new(age: i32, name: String, lang: String) -> Developer {
      Developer {
        age, name, lang
      }
    }
}

// вызываем метод у экземпляра
fn methods() {
    let mut dev = Developer {
        name: "Mike".to_string(),
        age: 26,
        lang: "JS".to_string(),
    };

    let age = dev.get_age();
    dev.change_lang("Rust".to_string());

    // вызов ассоциированной функции
    let newDev = Developer::new(29, "mik".to_string(), "rust".to_string());
}
