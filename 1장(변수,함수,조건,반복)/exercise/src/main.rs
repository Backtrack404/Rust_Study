// Celsius to Fahrenheit
// Fahrenheit to Celsius
// fibonacci number

use std::io;

fn main() {
    
    loop {
        let mut temp = String::new();
        io::stdin().read_line(&mut temp).expect("읽기 실패");
        let temp: f32 = match temp.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("화씨: {}",celcius_to_fahrenheit(temp));
        println!("섭씨: {}", fahrenheit_to_celcius(temp));
        break;
    }

    fibonacci_number();
}

fn celcius_to_fahrenheit(cel:f32)->f32{
    let far = (cel * 1.8) + 32.0;
    far
}

fn fahrenheit_to_celcius(far:f32)->f32{
    let cel = (far - 32.0) / 1.8;
    cel
}


// 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89

fn fibonacci_number(){
    let mut n1 = 1;
    let mut n2 = 1;
    
    println!("{}", n1);
    println!("{}", n2);

    loop {
        let n = n1 + n2;
        if n < 100 {
            println!("{}", n);  
            n1 = n2;           
            n2 = n;
        }else{
            break;
        }        
    }

}





