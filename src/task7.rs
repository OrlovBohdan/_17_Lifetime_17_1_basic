#[test]

/*

*/
fn main() {
    /* 'a tied to fn-main stackframe */
    let var_a = 35;
    let example: Example;

    {
        /* Lifetime 'b tied to new stackframe/scope */
        let var_b = NoCopyType {};

        /* Тепер передаємо значення, а не посилання */
        example = Example {
            a: &var_a,
            b: var_b, // Передаємо значення
        };
    }

    println!("(Success!) {:?}", example);
}
#[derive(Debug)]
struct NoCopyType;

#[derive(Debug)]
#[allow(dead_code)]
struct Example<'a> {
    a: &'a u32,
    b: NoCopyType, // Зберігаємо значення, а не посилання
}




/*
У структурі Example поле b тепер є значенням типу NoCopyType, а не посиланням.
Таким чином, ми зберігаємо копію значення безпосередньо в структурі, і немає проблем з термінами життя.
*/