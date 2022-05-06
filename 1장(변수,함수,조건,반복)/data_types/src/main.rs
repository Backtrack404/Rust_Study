fn main(){
    let guess: u32 = "42".parse().expect("Not a number!");

    println!("val: {}", guess);
}


// 실행 안됨
// fn main(){
//     let guess = "42".parse().expect("Not a number!");
//     println!("val: {}", guess)
// }