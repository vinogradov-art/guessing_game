use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Угалай число");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Пожалуйста, введите свою догадку.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Не получилось прочитать строку");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Вы загодали {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Слишком малое число!"),
            Ordering::Equal => {
                println!("Вы выиграли!");
                break;
            }
            Ordering::Greater => println!("Слишком большое число!"),
        }
    }
}
