struct Color(i32, i32, i32);
struct Point(i32, i32, f32);

fn main(){
    let black = Color(255,0,255);
    let origin = Point(0,0,0.1);

    println!("{}", black.0);
    println!("{}", origin.2);
}