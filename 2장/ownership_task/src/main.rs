// fn main() {
//     let s1 = String::from("hello");
//     let s2 = s1;

//     println!("{}, world!", s1);
// }


// fn main(){
//     let s1 = String::from("hello");
//     let s2 = s1.clone();

//     println!("s1={}, s2={}", s1, s2);
// }

// fn main(){
//     // s가 스코프 안으로 들어옴
//     let s = String::from("hello");
//     // s의 값이 함수 안으로 이동(Move)
//     takes_ownership(s);
//     // 더이상 s는 유효하지 않음

//     // x가 스코프 안으로 들어옴
//     let x = 5;
//     // x가 함수 안으로 이동했으나 i32는 Copy가 되므로 
//     // x를 이후에 계속 사용할 수 있음
//     make_copy(x);

// }   // x는 스코프 밖으로 나가고, s도 나가지만 s는 이동되었으므로,
//     // 별다른 일이 발생하지 않음


// fn takes_ownership(some_string: String){
// // some_string이 스코프 안으로 들어옴
//     println!("{}", some_string);
// } // some_string이 스코프 밖으로 벗어났고 drop이 호출
//   // 메모리 해제

// fn make_copy(some_integer:i32){
// // some_integer이 스코프 안으로 들어옴
//     println!("{}", some_integer)
// } // some_integer가 스코프 밖으로 벗어나지만
//   // 별다른 일은 발생하지 않음.AsMut



// fn main(){
//     // gives_ownership은 반환값을 s1에게 이동시킴
//     let s1 = gives_ownership();

//     // s2가 스코프 안에 들어옴
//     let s2 = String::from("hello");

//     // s2는 takes_and_gives_back 안으로 이동되었고,
//     // 이 함수가 반환값을 s3로도 이동시킴
//     let s3 = takes_and_gives_back(s2);


//     println!("s1:{}", s1);
//     // println!("s2:{}", s2);  // 에러
//     println!("s3:{}", s3);

// }   // 여기서 s3는 스코프 밖으로 벗어나 drop 호출 
//     // s2는 스코프 밖으로 벗어났지만 이동되었으므로 아무 일도 없음
//     // s1은 스코프 밖으로 벗어나 drop 호출


// fn gives_ownership() -> String{ // gives_ownership 함수가 반환값을 
//                                 // 호출한 쪽으로 이동시킴
    
//     // some_string이 스코프 안에 들어옴
//     let some_string = String::from("hello");

//     some_string // some_string이 반환되고, 호출한 쪽의 함수로 이동
// }


// // takes_and_gives_back 함수는 String을 하나 받아 다른 하나를 반환
// fn takes_and_gives_back(a_string: String)->String{  // a_string이 스코프 안으로 들어옴
//     a_string    // a_string은 반환되고, 호출한 쪽의 함수로 이동
// }


fn main(){
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize){
    let length = s.len();

    (s, length)
}