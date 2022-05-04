fn main(){

    // addition
    let sum = 5 + 10;

    // subtraction 
    let difference = 95.3 - 3.9;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // remainder 
    let remainder = 43 % 5;


    println!("sum: {}", sum);
    println!("difference: {}", difference);
    println!("product: {}", product);
    println!("quotient: {}", quotient);
    println!("remainder: {}", remainder);


    let t = true;
    let f: bool = false;    // with explicit type annotation

    println!("t: {}\nf: {}", t, f);

    let c = 'Z';
    let z = 'ℤ';    
    let heart_eyed_cat = '😻';

    println!("c: {}",c);
    println!("z: {}",z);
    println!("heart_eyed_cat: {}",heart_eyed_cat);
}