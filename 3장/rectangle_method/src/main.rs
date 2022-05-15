// #[derive(Debug)]

// struct Rectangle{
//     length: u32,
//     width: u32,
// }

// impl Rectangle{
//     fn area(&self) -> u32 {
//         //(&self).length * (&self).width //아래와 같은 표현
//         self.length * self.width
//     }

//     fn can_hold(&self, other:&Rectangle) -> bool {
//         self.length > other.length && self.width > other.width
//     }
// }

// fn main(){
//     let rect1 = Rectangle { length:50, width:30 };
//     let rect2 = Rectangle { length:40, width:10 };
//     let rect3 = Rectangle { length:45, width: 60};

//     println!("Can rec1 hold rec2? {}", rect1.can_hold(&rect2));
//     println!("Can rect1 hold rec3? {}", rect1.can_hold(&rect3));
// }

#[derive(Debug)]
struct Rectangle{
    width: u32,
    length: u32,
}

impl Rectangle{
    fn area(&self) -> u32{
        self.width * self.length
    }

    fn square(size:u32) -> Rectangle{
        Rectangle {length: size, width: size}
    }
}

fn main(){
    let sq = Rectangle::square(3);
    
    println!("square is : {}", sq.area());
}