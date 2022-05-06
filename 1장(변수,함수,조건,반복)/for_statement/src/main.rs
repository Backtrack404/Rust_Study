fn main(){
    let a = [10,20,30,40,50];

    for element in a.iter() { 
        println!("the value is: {}", element);
    }

    space_launch_counter();
}


fn space_launch_counter(){
    for number in (1..4).rev(){
        println!("{}!", number);
    }
    println!("LIFTOFF!!");
}