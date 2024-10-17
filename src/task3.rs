#[test]

/*
/* Make it work by adding proper lifetime annotation */
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {}
*/
fn main() {
    let string1 = String::from("Hello");
    let string2 = String::from("World!");

    let result = longest(&string1, &string2);
    println!("The longest string is: {}", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}



/*
Анотація тривалості:

<'a>: Це оголошення тривалості. 'a є параметром тривалості, який ви можете використовувати для вказівки, що всі три посилання (x, y, і повернуте посилання) мають одну і ту ж тривалість.
x: &'a str і y: &'a str: Вхідні параметри функції longest є посиланнями на строки з тривалістю 'a.
-> &'a str: Повернене значення також є посиланням на строку з тривалістю 'a, що означає, що воно дійсне, поки хоча б одне з вхідних посилань дійсне.

Основна функція:

У main, ми створюємо дві строки (string1 і string2), передаємо їх у функцію longest, а потім виводимо найдовшу строку.
*/