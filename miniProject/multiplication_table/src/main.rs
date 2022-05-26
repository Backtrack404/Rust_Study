use std::io;

fn main(){
    loop {
        let mut start_number = String::new();
        
        println!("구구단을 시작할 숫자를 입력해 주세요");
        io::stdin().read_line(&mut start_number).expect("읽기 실패");

        let start_number: u32 = match start_number.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("\n{}단\n", start_number);

        let vec = mutiple(start_number, 10);
        for number in 0..9 {
            println!("{} x {} = {}",start_number, number+1, vec[number]);
        }
        println!("\n\n");
    }
}

fn mutiple(x:u32, len:usize) -> Vec<u32> {
    let mut vec = Vec::with_capacity(len);

    for number in 1..10{
        vec.push(x * number);
    }
    
    vec
}