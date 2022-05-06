fn main(){
    println!("Hello, world!");

    another_function();
    another_function2(5, 3.3);
    expression();

    let five = five();

    println!("five is : {}", five);

    let x = pluse_one(5);
    println!("The value of x is : {}", x);
}

fn another_function(){
    println!("Another function.");
}

// 인자를 받는 함수
fn another_function2(x: i32, y: f32){
    println!("The value of x is : {}", x);
    println!("The value of y is : {}", y);
}

// 표현식
fn expression(){
    let x = 5;

    let y = 
    //표현식 부
    {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
    println!("The value of x is: {}", x);
}

//반환값을 가지는 함수
fn five() -> i32 {
    5
}

// 반환값을 가지는 함수2 
fn pluse_one(x: i32) -> i32{
    x + 1
}