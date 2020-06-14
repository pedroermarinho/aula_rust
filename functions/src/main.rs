fn soma(a:i32, b:i32) -> i32 {
    a + b
}

fn mult(a:i32, b:i32) -> i32{
    a * b
}

fn sub(a:i32, b:i32) -> i32{
    a - b
}

fn div(a:i32, b:i32) -> i32{
    a / b
}

fn main() {

    let a:i32 = 5;
    let b:i32 = 10;

    //soma
    println!("{} + {} = {soma}",a,b,soma=soma(a,b));
    //mult
    println!("{} * {} = {mult}",a,b,mult=mult(a,b));
    //sub
    println!("{} - {} = {sub}",a,b,sub=sub(a,b));
    //div
    println!("{} / {} = {div}",a,b,div=div(a,b));
}
