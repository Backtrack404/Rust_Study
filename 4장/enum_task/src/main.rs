// #[derive(Debug)]
// enum IpAddrKind{
//     V4(String),
//     V6(String),
// }

// fn main(){
//     let home = IpAddrKind::V4(String::from("127.0.0.1"));
    
//     let loopback = IpAddrKind::V6(String::from("::1")); 

//     println!("V4:{:?}", home);
//     println!("V6:{:?}", loopback);
// }

// #[derive(Debug)]
// enum IpAddr{
//     V4(u8, u8, u8, u8),
//     V6(String),
// }

// fn main(){
//     let home = IpAddr::V4(127,0,0,1);
//     let loopback = IpAddr::V6(String::from("::1"));

//     println!("V4: {:?}", home);
//     println!("V4: {:?}", loopback);
// }


struct QuitMessage;

struct MoveMessage{
    x: i32,
    y: i32,
}

struct WriteMessage(String);

struct ChangeColorMessage(i32, i32, i32);

enum Message{
    Quit,
    Move {x:i32, y:i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message{
    fn call(&self){
        
    }
}
fn main(){
    let m = Message::Write(String::from("hello"));
    m.call();
}