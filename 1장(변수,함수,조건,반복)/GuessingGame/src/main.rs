extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("숫자를 추리해 보세요!");

    let secret_number = rand::thread_rng().gen_range(1,101);

    loop {
        println!("추리한 숫자를 입력해 주세요.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("읽기 실패");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("추리한 값: {}", guess);

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
