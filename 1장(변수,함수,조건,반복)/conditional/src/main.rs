fn main(){
    let number = 3;
     
    if number < 5 {
        println!("condition was true");
    }else{
        println!("condition was false");
    }

    condition(number);

    let number  = 6;
    condition_elseif(number);

    conditon_let();
}

fn condition(x: i32){
    if x != 0 {
        println!("number was something other than zero");
    }
}

fn condition_elseif(number:i32){

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is divisible by 4, 3 or 2");
    }
}

fn conditon_let() {
    let condition = true;
    let number = if condition{
        5
    }else {
        6
    };

    println!("The value of number is : {}", number);
}