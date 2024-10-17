#[test]

/*
// `'a` must live longer than the function.
// Here, `&String::from("foo")` would create a `String`, followed by a
// reference. Then the data is dropped upon exiting the scope, leaving
// a reference to invalid data to be returned.

/* Fix the error in three ways  */
fn invalid_output<'a>() -> &'a String {
    &String::from("foo")
}

fn main() {
}
*/

//1
fn main() {
    let my_string = valid_output();
    println!("{}", my_string);
}
fn valid_output() -> String {
    String::from("foo")
}




//2
/*fn main() {
    let my_string = static_output();
    println!("{}", my_string);
}
fn static_output() -> &'static str {
    "foo" // статичний рядок
}
*/



//3
/*fn main() {
    let my_string = String::from("foo");
    let result = valid_output(&my_string);
    println!("{}", result);
}

fn valid_output<'a>(input: &'a String) -> &'a String {
    input // повертаємо посилання на передане значення
}*/





/*
//1
Повернення String замість посилання: Найпростіший спосіб — повернути значення String замість посилання.
Таким чином, String буде зберігатися в пам'яті до тих пір, поки вона потрібна.
//2
Використання статичних рядків: Якщо вам потрібно, щоб повернене посилання було дійсним,
ви можете використовувати статичні рядки (строки, які живуть протягом усього життя програми).
//3
Передача володіння: Якщо функція повинна повертати посилання, можна модифікувати її так, щоб вона приймала посилання, але не створювала локальні змінні.
Наприклад, якщо ви хочете, щоб функція приймала String, а потім повертала посилання на дані:
*/