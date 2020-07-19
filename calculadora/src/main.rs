mod calc{

    fn is_zero(number:i32) ->bool{
        number == 0
    }
    
    pub fn add(a:i32 ,b:i32) -> i32{
        a + b
    }

    pub fn divide(a:i32,b:i32) -> i32{
        if is_zero(b){
            0
        }else{
            a / b
        }
    }

    pub fn sub(a:i32,b:i32)-> i32{
        a - b
    }

    pub fn mult(a:i32, b:i32) -> i32{
        a * b
    }

}

use calc::add as soma;

fn main() {
    let x:i32=5;
    let y:i32=3;

    println!("{} + {} = {}",x,y,soma(x, y));
    println!("{} / {} = {}",x,y,calc::divide(x, y));
    println!("{} - {} = {}",x,y,calc::sub(x, y));
    println!("{} * {} = {}",x,y,calc::mult(x, y));
}
