
// fn main(){
//     let mut user1 = User{
//         email:String::from("someone@example.com"),
//         username:String::from("someusername123"),
//         active: true,
//         sign_in_count: 1,
//     };

//     user1.email = String::from("anotheremail@example.com");

//     println!("user email: {}", user1.email);
// }

// fn main(){
//     let email = String::from("someone@example.com");
//     let username = String::from("someusername123");
//     let user = build_user(email, username);

//     println!("username: {}", user.username);
//     println!("email: {}", user.email);
//     println!("active: {}", user.active);
//     println!("sign_in_count: {}", user.sign_in_count);
    
// }

// fn build_user(email: String, username: String) -> User {
//     User{
//         email: email,
//         username: username,
//         active: true,
//         sign_in_count: 1,
//     }
// }

struct User{
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main(){
    let email = String::from("someone@example.com");
    let username = String::from("someusername123");
    let user1 = build_user(email, username);

    let user2 = User{
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };

    println!("username: {}", user1.username);
    println!("email: {}", user1.email);
    println!("active: {}", user1.active);
    println!("sign_in_count: {}", user1.sign_in_count);

    println!("username: {}", user2.username);
    println!("email: {}", user2.email);
    println!("active: {}", user2.active);
    println!("sign_in_count: {}", user2.sign_in_count);

}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}