#[test]

/*
/* Make it work by adding proper lifetime annotation */

// A type `Borrowed` which houses a reference to an
// `i32`. The reference to `i32` must outlive `Borrowed`.
#[derive(Debug)]
struct Borrowed(&i32);

// Similarly, both references here must outlive this structure.
#[derive(Debug)]
struct NamedBorrowed {
    x: &i32,
    y: &i32,
}

// An enum which is either an `i32` or a reference to one.
#[derive(Debug)]
enum Either {
    Num(i32),
    Ref(&i32),
}

fn main() {
    let x = 18;
    let y = 15;

    let single = Borrowed(&x);
    let double = NamedBorrowed { x: &x, y: &y };
    let reference = Either::Ref(&x);
    let number    = Either::Num(y);

    println!("x is borrowed in {:?}", single);
    println!("x and y are borrowed in {:?}", double);
    println!("x is borrowed in {:?}", reference);
    println!("y is *not* borrowed in {:?}", number);
}
*/

// A type `Borrowed` which houses a reference to an
// `i32`. The reference to `i32` must outlive `Borrowed`.

fn main() {
    let x = 18;
    let y = 15;

    let single = Borrowed(&x);
    let double = NamedBorrowed { x: &x, y: &y };
    let reference = Either::Ref(&x);
    let number = Either::Num(y);

    println!("x is borrowed in {:?}", single);
    println!("x and y are borrowed in {:?}", double);
    println!("x is borrowed in {:?}", reference);
    println!("y is *not* borrowed in {:?}", number);

    // Додайте використання полів, щоб усунути попередження
    use_fields(&double);
}
#[derive(Debug)]
#[allow(dead_code)]
struct Borrowed<'a>(&'a i32);

// Similarly, both references here must outlive this structure.
#[derive(Debug)]
struct NamedBorrowed<'a> {
    x: &'a i32,
    y: &'a i32,
}

// An enum which is either an `i32` or a reference to one.
#[derive(Debug)]
#[allow(dead_code)]
enum Either<'a> {
    Num(i32),
    Ref(&'a i32),
}



// Функція для використання полів
fn use_fields<'a>(named: &NamedBorrowed<'a>) {
    println!("Using NamedBorrowed with x: {} and y: {}", named.x, named.y);
}


/*

Анотація тривалості для Borrowed:
У структурі Borrowed анотація 'a вказує, що посилання на i32 повинно жити щонайменше так довго, як і сама структура Borrowed.

Анотація тривалості для NamedBorrowed:
У структурі NamedBorrowed анотація 'a вказує, що обидва посилання на i32 (x і y) повинні мати одну і ту ж тривалість 'a, що гарантує їхнє життя на протязі часу, поки існує сама структура.

Анотація тривалості для Either:
У енумі Either анотація 'a вказує, що посилання Ref також повинно жити принаймні так довго, як і сама структура Either.
*/