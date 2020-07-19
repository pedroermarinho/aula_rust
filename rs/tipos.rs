fn main() {
    println!("\n");

    let a:char='\u{2764}';
    let b:char='9';
    let c:char='0';

    println!("{} is a digit? {}",a,a.is_digit(10));
    println!("{} is a binary? {}",a,a.is_digit(2));

    println!("{} is a digit? {}",b,b.is_digit(10));
    println!("{} is a binary? {}",b,b.is_digit(2));

    println!("{} is a digit? {}",c,c.is_digit(10));
    println!("{} is a binary? {}",c,c.is_digit(2));

    println!("\n");

    let d:char='→';
    let repr:String = d.escape_unicode().collect();
    println!("{} -> {}",d,repr);

    println!("\n");

    println!("{}",'a'.is_alphabetic());
    println!("{}",' '.is_alphabetic());
    println!("{}",'→'.is_alphabetic());

    println!("\n");

    println!("Uppercase ->{}",'a'.is_uppercase());
    println!("Lowercase ->{}",'a'.is_lowercase());
    println!("Whitespace ->{}",'a'.is_whitespace());
    println!("Alphanumeric ->{}",'a'.is_alphanumeric());
    println!("Numeric ->{}",'a'.is_numeric());

    println!("\n");

    let e:bool=true;
    let f:bool=false;

    if e {
        println!("E is true");
    }
    if f {
        println!("F is true");
    }

    println!("\n");

    println!("true AND false is {}",true && false);
    println!("true OR false is {}",true || false);
    println!("Not true is {}",!true);

    println!("\n");

    let g:i32 = 429496729; //inteiro com sinal 
    println!("{}",g);
    
    let h:u32 = 4294967295; //inteiro sem sinal 
    println!("{}",h);

    println!("\n");

    println!("i8 = {} a {}",i8::min_value(),i8::max_value());
    println!("i16 = {} a {}",i16::min_value(),i16::max_value());
    println!("i32 = {} a {}",i32::min_value(),i32::max_value());
    println!("i64 = {} a {}",i64::min_value(),i64::max_value());
    println!("u8 = {} a {}",u8::min_value(),u8::max_value());
    println!("u16 = {} a {}",u16::min_value(),u16::max_value());
    println!("u32 = {} a {}",u32::min_value(),u32::max_value());
    println!("u64 = {} a {}",u64::min_value(),u64::max_value());
    
    println!("\n");

    let i:i8 = 1;
    println!("Uns {}",i.count_ones());
    println!("Zeros {}",i.count_zeros());

    println!("\n");

    println!(">>: {}",i.rotate_left(7));
    println!(">>: {}",i.rotate_right(8));
    println!(">>: {}",i.swap_bytes());

    println!("\n");

    let j:f64=34234.23423;
    println!("{}",j);
    
    println!("\n");
    
    let k:f32=3.549236;

    println!("{}",k);
    println!("Floor: {}",k.floor());
    println!("Ceil: {}",k.ceil());
    println!("Round: {}",k.round());
    println!("Truncate: {}",k.trunc());
    println!("Fractional: {}",k.fract());
    println!("Is Finite: {}",k.is_finite());
    println!("Is Infinite: {}",k.is_infinite());
    println!("Is NaN: {}",k.is_nan());

    println!("\n");

    let l =["Pedro","Rust","Marinho"];
    println!("Programando em {}, por {} {}",l[1],l[0],l[2]);

    println!("\n");

    let n =[0;5]; // => [0,0,0,0,0]
    println!("{}",n[0]);
    println!("{}",n.len());

    println!("\n");

    let mut m =["";5];
    m[0]="Pedro";
    m[1]="Rust";
    m[2]="Marinho";
    println!("Programando em {}, por {} {}",l[1],l[0],l[2]);

    println!("\n");

    let o=[1,2,3,4,5,6,7,8,9,0];
    let p=&o[..];
    let q=&o[3..5];

    println!("o tem {} elementos",o.len());
    println!("p tem {} elementos",p.len());
    println!("q tem {} elementos",q.len());

    println!("\n");

    let r = ['a','b','c','d'];
    for x in r.iter(){
        println!("{}",x);
    }
    
    println!("\n");
    let s =('a',"char");
    let t= (32,"Integer");
    let u=(1,"Teste",'c');

    println!("{} {}",s.0,s.1);
    println!("{} {}",t.0,t.1);
    println!("{} {} {}",u.0,u.1,u.2);

    println!("\n");

    let (u1,u2,u3) = u;
    println!("{} {} {}",u1,u2,u3);

    println!("\n");

    let v=(1,"um");
    let x=(2,"dois");
    let w=(3,"tres");

    let z = [v,x,w];

    for element in z.iter() {
        println!("{} -> {}",element.0,element.1);
    }

    println!("\n");
    
}
