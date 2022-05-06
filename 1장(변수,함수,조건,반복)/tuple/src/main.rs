fn main(){
    let tup:(i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("tup: {}, {}, {}", x, y, z);

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    println!("five_hundred: {}", five_hundred);
    println!("six_point_four: {}", six_point_four);
    println!("one: {}", one);

    let months = ["1월", "2월", "3월", "4월", "5월", "6월", 
                    "7월", "8월", "9월", "10월", "11월", 
                    "12월"];

    let january = months[0];

    let february = months[1];
    println!("January: {}", january);
    println!("February: {}", february);


    
    // let a = [1,2,3,4,5];
    // let index = 10;

    // let element = a[index];
    // println!("The value of elemets is : {}", element);
}