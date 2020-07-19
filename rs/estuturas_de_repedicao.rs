fn main() {
    let mut a=0;

    while a< 10 {
        a += 1;
        println!("{}",a);
    }

    println!("\n");

    a =0;

    loop {
        a+=1;
        println!("{}",a);
        if a >10{
            break;
        }
    }

    println!("\n");

    for x in 1..11 {
        println!("{}",x);
    }

}