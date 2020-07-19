extern crate time;

fn print_today(){
    const THE_1900:i32=1900;

    let d = time::now();
    let day:i32= d.tm_mday;
    let mon:i32=d.tm_mon;
    let year:i32=d.tm_year+THE_1900;
    println!("hoje é {}/{}/{}",day,mon,year);
}

fn to_the_things(function:fn()){
    function()
}

fn main() {
    let pointer_to_function:fn() = print_today;
    to_the_things(pointer_to_function);
}
