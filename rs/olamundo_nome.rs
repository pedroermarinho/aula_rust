use std::io;

fn main(){
    let ola = "Ola,";
    let mut name = String::new();
    io::stdin().read_line(&mut name);

    println!("{} {}",ola,name);
}