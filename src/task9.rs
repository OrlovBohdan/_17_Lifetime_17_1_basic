#[test]

/*
/* Make it work by adding proper lifetime annotations */
struct ImportantExcerpt {
    part: &str,
}

impl ImportantExcerpt {
    fn level(&'a self) -> i32 {
        3
    }
}

fn main() {}
*/
fn main() {
    let text = String::from("This is an important excerpt.");
    let excerpt = ImportantExcerpt {
        part: &text[..], // Беремо зріз рядка, щоб отримати посилання
    };

    println!("Excerpt level: {}", excerpt.level());
}
#[allow(dead_code)]
struct ImportantExcerpt<'a> {
    part: &'a str, // Вказуємо термін життя для поля part
}

impl<'a> ImportantExcerpt<'a> { // Вказуємо термін життя для імплементації
    fn level(&self) -> i32 { // Метод level тепер використовує термін життя self
        3
    }
}




/*
Термін життя в структурі:
Додано 'a як термін життя для поля part, щоб вказати, що посилання повинно жити стільки ж, скільки і сам об'єкт ImportantExcerpt.

Термін життя в імплементації:
Додано 'a до імплементації impl ImportantExcerpt<'a> для того, щоб вказати, що всі методи цієї імплементації повинні дотримуватись того ж терміна життя.

Метод level:
Метод level тепер не має потреби в терміні життя, оскільки self є посиланням на об'єкт структури, що вже має вказане посилання на part.

Тестування в main:
Додано створення рядка text, щоб протестувати структуру ImportantExcerpt і перевірити, що метод level працює правильно.
*/