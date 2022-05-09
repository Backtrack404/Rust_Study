// fn main(){
//     let s = String::from("hello world!");

//     let len = first_word(&s);

//     println!("len: {}",len);
// }   

// fn first_word(s: &String) -> usize{
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate(){
//         if item == b' '{
//             return i;
//         }
//     }
//     s.len()
// }

// fn main(){
//     let s = String::from("hello world");
//     let str = first_word(&s);
    
//     // s.clear();
    
//     println!("{}", str);
// }

// fn first_word(s: &String) -> &str{
//     let bytes = s.as_bytes();
//     let void = b' ';
//     println!("void: {}", void);    

//     for (i, &item) in bytes.iter().enumerate(){
//         println!("i: {}\nitem:{}", i, &item);

//         if item == b' '{
//             return &s[0..i];
//         }
//     }
//     &s[..]
// }

fn main(){
    let my_string = String::from("hello world");

    // first_word가 'String'의 슬라이스로 동작
    let world = first_word(&my_string[..]);
    println!("{}", world);

    let my_string_literal = "hello world";

    // first_word가 스트링 리터럴의 슬라이스로 동작/
    let world = first_word(&my_string_literal[..]);
    println!("{}", world);

    // 스트링 리터럴은 *또한* 스트링 슬라이스이기 떄문에, 
    // 아래 코드도 슬라이스 문법 없이 동작.
    let world = first_word(my_string_literal);
    println!("{}", world);
}

fn first_word(s: &str) -> &str{
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}