#[test]

/*
// `print_refs` takes two references to `i32` which have different
// lifetimes `'a` and `'b`. These two lifetimes must both be at
// least as long as the function `print_refs`.
fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("x is {} and y is {}", x, y);
}

/* Make it work */
// A function which takes no arguments, but has a lifetime parameter `'a`.
fn failed_borrow<'a>() {
    let _x = 12;

    // ERROR: `_x` does not live long enough
    let y: &'a i32 = &_x;
    // Attempting to use the lifetime `'a` as an explicit type annotation
    // inside the function will fail because the lifetime of `&_x` is shorter
    // than `'a` . A short lifetime cannot be coerced into a longer one.
}

fn main() {
    let (four, nine) = (4, 9);

    // Borrows (`&`) of both variables are passed into the function.
    print_refs(&four, &nine);
    // Any input which is borrowed must outlive the borrower.
    // In other words, the lifetime of `four` and `nine` must
    // be longer than that of `print_refs`.

    failed_borrow();
    // `failed_borrow` contains no references to force `'a` to be
    // longer than the lifetime of the function, but `'a` is longer.
    // Because the lifetime is never constrained, it defaults to `'static`.
}
*/


//1
fn main() {
    let (four, nine) = (4, 9);
    print_refs(&four, &nine);

    let value = failed_borrow(); // отримуємо значення
    println!("Returned value: {}", value);
}

fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("x is {} and y is {}", x, y);
}

// Функція, яка повертає значення замість посилання
fn failed_borrow() -> i32 {
    let x = 12; // x живе в межах функції
    x // повертаємо значення, а не посилання
}


//2

/*fn main() {
    let (four, nine) = (4, 9);
    print_refs(&four, &nine);

    let x = 12; // x живе в основній функції
    failed_borrow(&x); // передаємо посилання на x
}

fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("x is {} and y is {}", x, y);
}

// Функція, яка приймає посилання як параметр
fn failed_borrow<'a>(value: &'a i32) {
    let y: &'a i32 = value; // тепер y може коректно посилатись на value
    println!("y is {}", y);
}*/


//3

/*fn main() {
    let (four, nine) = (4, 9);
    print_refs(&four, &nine);

    let boxed_value = failed_borrow(); // отримуємо Box<i32>
    println!("Returned value: {}", boxed_value);
}

fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("x is {} and y is {}", x, y);
}

// Функція, яка повертає Box<i32>
fn failed_borrow<'a>() -> Box<i32> {
    let x = Box::new(12); // x зберігається на купі
    x // повертаємо Box<i32>, що є валідним
}*/

/*

Повернення значення: У першому варіанті функція failed_borrow повертає значення i32, що виключає потребу в тривалості.
Передача параметра: У другому варіанті функція приймає посилання, яке залишається дійсним протягом всього життя.
Використання Box: У третьому варіанті ви зберігаєте дані в купі, що усуває проблему з тривалістю локальних змінних.

*/


