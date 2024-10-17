#[test]

/*
#[derive(Debug)]
struct NoCopyType {}

#[derive(Debug)]
#[allow(dead_code)]
struct Example<'a, 'b> {
    a: &'a u32,
    b: &'b NoCopyType
}

/* Fix function signature */
fn fix_me(foo: &Example) -> &NoCopyType
{ foo.b }

fn main()
{
    let no_copy = NoCopyType {};
    let example = Example { a: &1, b: &no_copy };
    fix_me(&example);
    println!("Success!")
}
*/
fn main() {
    let no_copy = NoCopyType {};
    let example = Example { a: &1, b: &no_copy };
    let result = fix_me(&example);

    println!("Success! Retrieved: {:?}", result);
}

#[derive(Debug)]
struct NoCopyType {}

#[derive(Debug)]
#[allow(dead_code)]
struct Example<'a, 'b> {
    a: &'a u32,
    b: &'b NoCopyType,
}

/* Fix function signature */
fn fix_me<'a, 'b>(foo: &'a Example<'a, 'b>) -> &'b NoCopyType {
    foo.b
}




/*
Функція fix_me:
Додані терміни життя 'a і 'b до сигнатури функції. 'a прив'язаний до терміна життя самого об'єкта Example, а 'b визначає термін життя поля b, яке повертається.

Додано вивід результату, щоб показати, що функція працює і повертає правильний об'єкт.
Тепер компілятор може правильно зрозуміти, що fix_me повертає посилання на NoCopyType,
термін життя якого прив'язаний до об'єкта Example, який передається в функцію.
Таким чином, ви не будете отримувати помилок про терміни життя.
*/