#[test]

/*
/* Remove all the lifetimes that can be elided */

fn input<'a>(x: &'a i32) {
    println!("`annotated_input`: {}", x);
}

fn pass<'a>(x: &'a i32) -> &'a i32 { x }

fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
    x
}

struct Owner(i32);

impl Owner {
    // Annotate lifetimes as in a standalone function.
    fn add_one<'a>(&'a mut self) { self.0 += 1; }
    fn print<'a>(&'a self) {
        println!("`print`: {}", self.0);
    }
}

struct Person<'a> {
    age: u8,
    name: &'a str,
}

enum Either<'a> {
    Num(i32),
    Ref(&'a i32),
}

fn main() {}
*/

fn main() {}
#[allow(dead_code)]
fn input(x: &i32) {
    println!("`input`: {}", x);
}
#[allow(dead_code)]

fn pass(x: &i32) -> &i32 { x }
#[allow(dead_code)]

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
#[allow(dead_code)]

struct Owner(i32);
#[allow(dead_code)]

impl Owner {
    fn add_one(&mut self) {
        self.0 += 1;
    }

    fn print(&self) {
        println!("`print`: {}", self.0);
    }
}
#[allow(dead_code)]

struct Person<'a> {
    age: u8,
    name: &'a str,
}
#[allow(dead_code)]

enum Either<'a> {
    Num(i32),
    Ref(&'a i32),
}







/*
Параметри функцій та повернення:
Видалені явні анотації тривалості з параметрів функцій та типів повернення (input, pass, longest).

Методи структур:
Видалені анотації тривалості в методах структури Owner (add_one, print).

Оголошення структур і енумів:
Видалені анотації тривалості з структур Person та Either.
*/