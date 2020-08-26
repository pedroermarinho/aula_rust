
use std::ops::Add;

#[derive(Clone, Copy)]
struct Pessoa{
    anos:i32,
    nome:&'static str,
}

impl Add<i32> for Pessoa {
    type Output =i32;

    fn add(self,b:i32) -> i32{
        self.anos +b
    }
}

fn main() {

    println!("1 + 2: {}",1 + 2);//add
    println!("1.3 + 2.4: {}",1.3 + 2.4);//add

    println!("1 - 2: {}",1 - 2);//sub
    println!("1.3 - 2.4: {}",1.3 - 2.4);//sub
    
    println!("1 * 2: {}",1 * 2);//mul
    println!("1.3 * 2.4: {}",1.3 * 2.4);//mul
    
    println!("1 / 2: {}",1 / 2);//div
    println!("1.3 / 2.4: {}",1.3 / 2.4);//div
    
    println!("1 % 2: {}",1 % 2);//rem
    println!("1.3 % 2.4: {}",1.3 % 2.4);//rem
    
    println!("!false: {}",!false);//not
    println!("!3: {}",!3);//not
    
    println!("true & false: {}",true & false);//BitAnd
    println!("2 & 6: {}",2 & 6);//BitAnd
    
    println!("true | false: {}",true | false);
    println!("12 | 5: {}",12 | 5);
    
    println!("true ^ false: {}",true ^ false);//BitXor
    println!("12 ^ 5: {}",12 ^ 5);//BitXor
    
    println!("12 >> 5: {}",12 >> 5);//Shr
    println!("12 << 5: {}",12 << 5);//Shl


    println!("\n");

    let p1 = Pessoa{
        nome:"Pedro Marinho",
        anos:20
    };

    let x = p1 + 10;

    println!("A nova idade de {} é {}",p1.nome,x);
    
    
}