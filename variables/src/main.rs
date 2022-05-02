// fn main() {
//     let x = 5;
//     println!("The value of x is {}", x);
    
//     // x = 6;   // 불변성 변수에 재할당 에러 
//     println!("The value of x is {}", x);
// }


fn main(){
    let mut x:u32 = 5;  // 가변 변수 선언
    println!("The value of x is : {}", x);

    x = 6;
    println!("The value of x is : {}", x);

    const MAX_POINTS:u32 = 100_000; // 싱수 선언
    println!("MAX Point is : {}", MAX_POINTS);
}