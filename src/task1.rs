#[test]

/*
/* Annotate the lifetime of `i` and `borrow2` */

// Lifetimes are annotated below with lines denoting the creation
// and destruction of each variable.
// `i` has the longest lifetime because its scope entirely encloses
// both `borrow1` and `borrow2`. The duration of `borrow1` compared
// to `borrow2` is irrelevant since they are disjoint.
fn main() {
    let i = 3;
    {
        let borrow1 = &i; // `borrow1` lifetime starts. ──┐
        //                                                │
        println!("borrow1: {}", borrow1); //              │
    } // `borrow1 ends. ──────────────────────────────────┘
    {
        let borrow2 = &i;

        println!("borrow2: {}", borrow2);
    }
}
*/

fn main() {
    let i = 3; // `i` має тип `i32` за замовчуванням
    {
        let borrow1: &i32 = &i; // borrow1 отримує посилання на i
        println!("borrow1: {}", borrow1); // виводимо borrow1
    } // `borrow1` завершує своє життя

    {
        let borrow2: &i32 = &i; // borrow2 отримує посилання на i
        println!("borrow2: {}", borrow2); // виводимо borrow2
    } // `borrow2` завершує своє життя
}



/*
Для borrow1 та borrow2 потрібно вказати тип посилання на i, наприклад, &i32, оскільки i є цілим числом типу i32.
i: Тривалість змінної i триває протягом усього блоку main, тому вона має найдовшу тривалість.
borrow1 і borrow2: Обидва ці змінні є посиланнями на i, і їх тривалість обмежена відповідними блоками, в яких вони визначені.
Після виходу з блоку, в якому визначені borrow1 і borrow2, їхні посилання більше не дійсні.
*/