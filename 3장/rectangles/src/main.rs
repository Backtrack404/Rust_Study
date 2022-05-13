// 단일 변수로 구성된 사각형 넓이 구하는 프로그램
// fn main(){
//     let length1 = 50;
//     let width1 = 30;

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(length1, width1)
//     );
// }

// fn area(length: u32, width: u32) -> u32 {
//     length * width
// }


// 튜플로 구성된 사각형의 넓이 구하는 프로그램
// fn main(){
//     let rect1 = (50, 30);

//     println!(
//         "The area of the rectangle is {} square pixels.",
//          area(rect1)
//     );
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

struct Rectangle {
    length: u32,
    width: u32,
}

fn main(){
    let rect1 = Rectangle {length: 50, width: 30};
    println!(
                "The area of the rectangle is {} square pixels.",
                 area(&rect1)
            );
}

fn area(rectangle: &Rectangle) -> u32{
    rectangle.length * rectangle.width
}